<%
# Copyright (c) 2022 Arista Networks, Inc.  All rights reserved.
# Arista Networks, Inc. Confidential and Proprietary.
# Subject to Arista Networks, Inc.'s EULA.
# FOR INTERNAL USE ONLY. NOT FOR DISTRIBUTION.
# pylint: skip-file
"""
studio-template.mako

This is a MAKO Studio template being rendered per device per studio

The purpose of this template is:
 - Run AVD Structured Config for the scope of the studio (eos_designs / yaml_templates_to_facts)
 - Run AVD eos_cli_config_gen
 - Print configuration
"""
import json

from copy import deepcopy
from time import time

from deepmerge import always_merger
from pyavd import get_device_structured_config, get_device_config
from tagsearch_python.tagsearch_pb2 import TagMatchRequestV2
from tagsearch_python.tagsearch_pb2_grpc import TagSearchStub

TAGMAPPINGS = []

# Get studio info from ctx
DEVICE = ctx.getDevice()
DEVICE_ID = DEVICE.id
WORKSPACE_ID = ctx.studio.workspaceId


runtimes = {"start_time": time()}


def __resolve_device_tag_query(query):
    if query == "":
        return []
    if query is None:
        query = "device:*"
    tsclient = ctx.getApiClient(TagSearchStub)
    search_req = TagMatchRequestV2(query=query, workspace_id=WORKSPACE_ID)
    search_res = tsclient.GetTagMatchesV2(search_req)
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


def __build_device_vars(datasets: list[dict], device_id: str):
    """
    Parse avd_inputs data structure and build a nested dict with all vars for each host

    Parameters
    ----------
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
    """
    one_device_vars = {}
    for dataset in datasets:
        devices = __resolve_device_tag_query(dataset["device_query"])
        if device_id in devices:
            always_merger.merge(one_device_vars, deepcopy(dataset["avd_vars"]))
    return one_device_vars


def __update_device_vars_with_tag_values(device_id: str, one_device_vars: dict):
    """
    inplace merge data from tag mappings

    Parameters
    ----------
    device_id : str
    device_vars : dict

    Returns
    -------
    dict
    """
    tag_mapper = TagMapper(TAGMAPPINGS)
    device_tag_values = __get_device_tags(device_id, tag_mapper.labels)
    mapped_tag_data = tag_mapper.map_device_tags(device_tag_values)
    always_merger.merge(one_device_vars, deepcopy(mapped_tag_data))


def __build_device_vars(device_id: str, datasets: list[dict]):
    """
    Parse avd_inputs data structure and build a nested dict with all vars for the given device_id
    Then merge data from tag mappings

    Parameters
    ----------
    device_id : str
        Device IDs
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
    """
    tag_mapper = TagMapper(TAGMAPPINGS)

    one_device_vars = {}
    for dataset in datasets:
        devices = __resolve_device_tag_query(dataset["device_query"])
        if device_id in devices:
            always_merger.merge(one_device_vars, deepcopy(dataset["avd_vars"]))

    device_tag_values = __get_device_tags(device_id, tag_mapper.labels)
    mapped_tag_data = tag_mapper.map_device_tags(device_tag_values)
    always_merger.merge(one_device_vars, deepcopy(mapped_tag_data))

    if not one_device_vars.get("hostname"):
        raise KeyError(
            f"Key 'hostname' not found in vars for device ID '{device_id}'"
        )

    return one_device_vars


retrieval_timer = time()
avd_inputs = json.loads(ctx.retrieve(path=["avd"], customKey="avd_inputs", delete=False))
avd_switch_facts = json.loads(ctx.retrieve(path=["avd"], customKey="avd_switch_facts", delete=False))
runtimes["retrieve"] = str(time() - retrieval_timer)

device_vars = __build_device_vars(DEVICE_ID, avd_inputs)
hostname = device_vars["hostname"]

pyavd_timer = time()
structured_config = get_device_structured_config(hostname, device_vars, avd_switch_facts)
runtimes["pyavd_struct_cfg"] = str(time() - pyavd_timer)
pyavd_timer = time()
eos_config = get_device_config(structured_config)
runtimes["pyavd_eos_cfg"] = str(time() - pyavd_timer)

# storage_timer = time()
# ctx.store(json.dumps(device_vars), customKey=f"{hostname}_device_vars", path=["avd"])
# ctx.store(json.dumps(structured_config), customKey=f"{hostname}_structured_config", path=["avd"])
# ctx.store(json.dumps(eos_config), customKey=f"{hostname}_config", path=["avd"])
# runtimes["store"] = str(time() - storage_timer)

runtimes["total"] = str(time() - runtimes["start_time"])

ctx.alog(f"Completed studio template for '{ctx.studio.studioId}' with runtimes {runtimes}.")

%>
${eos_config}
