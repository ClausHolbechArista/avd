# Copyright (c) 2022 Arista Networks, Inc.  All rights reserved.
# Arista Networks, Inc. Confidential and Proprietary.
# Subject to Arista Networks, Inc.'s EULA.
# FOR INTERNAL USE ONLY. NOT FOR DISTRIBUTION.
# pylint: skip-file
# flake8: noqa
"""
action-workspace-prebuild.py

This is a prebuild action running after studio prebuild in a studio build pipeline.
This action is running once per workspace, so it handles all studios and devices.

The purpose of this action is:
 - Run AVD Topology (eos_designs_facts)
 - Store result in cache
"""
import json
from copy import deepcopy
from time import time

from cloudvision.cvlib import Studio
from deepmerge import always_merger
from pyavd import get_avd_facts
from tagsearch_python.tagsearch_pb2 import TagMatchRequestV2
from tagsearch_python.tagsearch_pb2_grpc import TagSearchStub

WORKSPACE_ID = ctx.action.args["WorkspaceID"]
STUDIO_IDS = str(ctx.action.args["StudioIDs"]).split(",")
TAGMAPPINGS = []

runtimes = {"start_time": time()}

# Hack to get ctx.tags working outside of a studio template:
ctx.studio = Studio(workspaceId=WORKSPACE_ID, studioId="")


def __resolve_device_tag_query(query):
    timer = time()
    if query == "":
        return []
    if query is None:
        query = "device:*"
    tsclient = ctx.getApiClient(TagSearchStub)
    search_req = TagMatchRequestV2(query=query, workspace_id=WORKSPACE_ID)
    search_res = tsclient.GetTagMatchesV2(search_req)
    runtimes.setdefault("resolve_device_tag_query", []).append(str(time() - timer))
    return [match.device.device_id for match in search_res.matches]


def __get_device_tags(device_id, labels):
    timer = time()
    device_tags = {}
    all_device_tags = ctx.tags._getDeviceTags(device_id)
    for label in labels:
        if value_list := all_device_tags.get(label):
            if len(value_list) == 1:
                device_tags[label] = value_list[0]
            else:
                device_tags[label] = value_list
    runtimes.setdefault("get_device_tags", []).append(str(time() - timer))
    return device_tags


class TagMapper:
    def __init__(self, mappings):
        """
        Maps data from studio inputs to AVD data model.

        If "convert_value" is set (Currently only supporting "int"), the data will be converted.

        Parameters
        ----------
        mappings : list[dict]
            List of variable mappings like:
            [
                {
                    "from_tag": "Fabric_Name",
                    "to_path": ["fabric_name"],
                    "convert_value": "int"
                }
            ]
        """
        self.mappings = mappings
        self.labels = [mapping["from_tag"] for mapping in mappings]

    def __convert_value(self, convert_value: str, value):
        """Convert value if supported. Raise if not."""
        # ctx.alog(f"Converting value {value} using {convert_value}")
        if convert_value == "int" and isinstance(value, str):
            return int(value)

        raise ValueError(f"Unsupported convert_value: {convert_value}")

    def __set_value_from_path(self, path: list, data: list | dict, value):
        """Recursive function to walk through data to set value of path, creating any level needed."""
        if not path:
            raise ValueError("Path is empty. Something bad happened.")

        if len(path) == 1:
            if isinstance(data, dict):
                data[path[0]] = value
            elif isinstance(data, list) and isinstance(path[0], int):
                # We ignore the actual integer value and just append the item to the list.
                data.append(value)
            else:
                raise ValueError(f"Path '{path}' cannot be set on data of type '{type(data)}'")
            return

        # Two or more elements in path.
        if isinstance(data, dict):
            # For dict, create the child key with correct type and call recursively.
            if isinstance(path[1], int):
                data.setdefault(path[0], [])
                self.__set_value_from_path(path[1:], data[path[0]], value)
            else:
                data.setdefault(path[0], {})
                self.__set_value_from_path(path[1:], data[path[0]], value)
        elif isinstance(data, list) and isinstance(path[0], int):
            # For list, append item of correct type and call recursively.
            # Notice that the actual index in path[0] is ignored.
            # TODO: Consider accepting index or lookup based on key, to ensure consistency when updating multiple fields.
            index = len(data)
            if isinstance(path[1], int):
                data.append([])
                self.__set_value_from_path(path[1:], data[index], value)
            else:
                data.append({})
                self.__set_value_from_path(path[1:], data[index], value)

        else:
            raise ValueError(f"Path '{path}' cannot be set on data of type '{type(data)}'")

        return None

    def map_device_tags(self, device_tag_values: dict) -> dict:
        """
        Map tags for the given device_id according to mappings given at initilization.

        Returns a dict with values from tags.

        Parameters
        ----------
        device_tag_values : dict
            Dictionary with all device tags and values

        Returns
        -------
        dict
            mapped data from tags for one device
        """
        output = {}
        for mapping in self.mappings:
            if (value := device_tag_values.get(mapping["from_tag"])) is not None:
                if (convert_value := mapping.get("convert_value")) is not None:
                    value = self.__convert_value(convert_value, value)

                self.__set_value_from_path(mapping["to_path"], output, value)

        return output


def __build_device_vars(device_list: list, datasets: list[dict]):
    """
    Parse avd_inputs data structure and build a nested dict with all vars for each host
    Then merge data from tag mappings

    Parameters
    ----------
    device_list : list
        List of device IDs
    list
        dict
            Dataset only containing device_query and mapped_vars
            {
                "device_query": "DC_Name:DC2 AND Role:L3leaf",
                "avd_vars": {"network_type": {"defaults": {"bgp_as": "5678"}}}
            }
    Returns
    -------
    dict
        hostname1 : dict
        hostname2 : dict
    """
    tag_mapper = TagMapper(TAGMAPPINGS)

    # device_id_vars is using device_id as key, since we may not have the hostname yet. Hostname could come from tags.
    device_id_vars = {}

    for dataset in datasets:
        matching_device_ids = __resolve_device_tag_query(dataset["device_query"])
        for device_id in matching_device_ids:
            if device_id not in device_list:
                continue

            one_device_vars = device_id_vars.setdefault(device_id, {})
            always_merger.merge(one_device_vars, deepcopy(dataset["avd_vars"]))

    device_vars = {}
    for device_id in device_list:
        device_tag_values = __get_device_tags(device_id, tag_mapper.labels)
        mapped_tag_data = tag_mapper.map_device_tags(device_tag_values)
        one_device_vars = device_id_vars.setdefault(device_id, {})
        always_merger.merge(one_device_vars, deepcopy(mapped_tag_data))
        if not one_device_vars.get("hostname"):
            raise KeyError(f"Key 'hostname' not found in vars for device ID '{device_id}'")
        hostname = one_device_vars["hostname"]
        device_vars[hostname] = one_device_vars

    return device_vars


retrieval_timer = time()
avd_inputs = json.loads(ctx.retrieve(path=["avd"], customKey="avd_inputs", delete=False))
device_list = json.loads(ctx.retrieve(path=["avd"], customKey="device_list", delete=False))
runtimes["retrieve"] = str(time() - retrieval_timer)

device_vars = __build_device_vars(device_list, avd_inputs)
# ctx.store(json.dumps(device_vars), customKey="devices_vars_with_tags", path=["avd"])

pyavd_timer = time()
avd_switch_facts = get_avd_facts(device_vars)
runtimes["pyavd_facts"] = str(time() - pyavd_timer)

storage_timer = time()
ctx.store(json.dumps(avd_switch_facts), customKey="avd_switch_facts", path=["avd"])
runtimes["store"] = str(time() - storage_timer)

runtimes["total"] = str(time() - runtimes["start_time"])

ctx.alog(f"Completed build-hook 'action-workspace-prebuild' with runtimes {runtimes}.")
