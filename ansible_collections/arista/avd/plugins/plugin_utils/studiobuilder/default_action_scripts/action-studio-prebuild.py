# Copyright (c) 2022 Arista Networks, Inc.  All rights reserved.
# Arista Networks, Inc. Confidential and Proprietary.
# Subject to Arista Networks, Inc.'s EULA.
# FOR INTERNAL USE ONLY. NOT FOR DISTRIBUTION.
# pylint: skip-file
"""
action-studio-prebuild.py

This is a prebuild action running first in a studio build pipeline.
This action is running once per studio, so it handles inputs for all devices covered in the studio.

The purpose of this action is:
 - extract input values from the studio
 - convert input values to Ansible groupvars and hostvars
 - Ansible groupvars are used to reduce the size of the output dataset
   - Common inputs are added to a common Ansible group which all devices are members of
   - Inputs under device-tag resolvers are added to groups per resolver, which only the matched devices are part of
 - store AVD vars

TODO:
- TAG mappings
- Interface tags in resolvers and other places
- something with lists
"""

import json

from arista.studio.v1 import models
from arista.studio.v1.services import AssignedTagsConfigServiceStub, AssignedTagsServiceStub, InputsServiceStub
from arista.studio.v1.services.gen_pb2 import AssignedTagsConfigRequest, AssignedTagsRequest, InputsRequest
from fmp.wrappers_pb2 import RepeatedString
from google.protobuf.wrappers_pb2 import StringValue
from tagsearch_python.tagsearch_pb2 import TagMatchRequestV2
from tagsearch_python.tagsearch_pb2_grpc import TagSearchStub
from yaml import safe_load as yaml_safe_load

INPUTMAPPINGS = []
TAGMAPPINGS = []
WORKSPACE_ID = ctx.action.args["WorkspaceID"]
STUDIO_ID = ctx.action.args["StudioID"]


def __get_studio_assigned_tagquery():
    """
    Resolve the actual assigned tags.
    Because of a cornercases when changing from "" to None, we have to check the config to get the ws state
    If nothing is found in the workspace, we fallback to mainline.
    """
    get_req = AssignedTagsConfigRequest()
    get_req.key.workspace_id.value = WORKSPACE_ID
    get_req.key.studio_id.value = STUDIO_ID
    client = ctx.getApiClient(AssignedTagsConfigServiceStub)
    try:
        resp = client.GetOne(get_req)
        query = str(resp.value.query)
        if resp.value.remove:
            query = None  # Default is None which translates to all devices
        return query
    except Exception as e:
        # ctx.alog(f"workspace request failed {e}")
        # No changes in the workspace. Fallback to mainline
        pass

    get_req = AssignedTagsRequest()
    get_req.key.workspace_id.value = ""
    get_req.key.studio_id.value = STUDIO_ID
    client = ctx.getApiClient(AssignedTagsServiceStub)
    try:
        resp = client.GetOne(get_req)
        return str(resp.value.query)
    except Exception as e:
        # ctx.alog(f"mainline request failed {e}")
        # The API raises if no tag query is set - which is the default and translates to all devices
        return None


class InputParser:
    """
    Transform raw studio input into a list of data sets to be applied for device queries
    Return data example with first common vars and then a resolver with two different tag queries:
    [
        {
            "device_query": "",
            "path": [],
            "inputs": {"p2p_uplinks_mtu": 9000},
        },
        {
            "inputs": {"dc": {}},
            "path": ["dc_vars"],
            "device_query": "DC_Name:DC1",
        },
        {
            "device_query": "DC_Name:DC2",
            "path": ["dc_vars"],
            "inputs": {"dc": {"mgmt_gateway": "10.10.10.1"}},
        }
    ]
    """

    queue: list
    studio_device_query: str

    def __init__(self, studio_device_query: str | None, interface_resolver_paths: list[list[str]] = []):
        self.studio_device_query = studio_device_query
        self.interface_resolver_paths = interface_resolver_paths
        self.queue = []

    def parse_inputs(self, inputs: dict) -> list[dict]:
        self.queue.append({"inputs": inputs, "path": [], "device_queries": []})
        datasets = []

        while self.queue:
            queue_item = self.queue.pop(0)
            dataset = queue_item.copy()
            dataset["inputs"] = self._parse_inputs(**queue_item)
            if not dataset["inputs"]:
                continue
            datasets.append(dataset)

        for dataset in datasets:
            dataset["device_query"] = self.__combine_device_tag_query_strings([self.studio_device_query] + dataset.pop("device_queries", []))

        return datasets

    def _parse_inputs(self, inputs, path=[], device_queries=[]):
        if isinstance(inputs, dict):
            parsed = {}
            for key, val in inputs.items():
                if (parsed_val := self._parse_inputs(val, path + [key], device_queries)) is not None:
                    parsed[key] = parsed_val
            return parsed

        if isinstance(inputs, list) and inputs:
            if isinstance(inputs[0], dict) and "tags" in inputs[0] and "inputs" in inputs[0]:
                # this is a type of resolver
                # check which one - default to device resolver
                if path in self.interface_resolver_paths:
                    # TODO: Implement something for interface resolvers
                    pass
                else:
                    for resolver_item in inputs:
                        self.queue.append(
                            {"inputs": resolver_item["inputs"], "path": path, "device_queries": device_queries + [resolver_item["tags"]["query"]]}
                        )
                    return None

            # collection type
            return [self._parse_inputs(element, path + [index], device_queries) for index, element in enumerate(inputs)]
        # base case for other python basic classes
        return inputs

    def __combine_device_tag_query_strings(self, queries: list):
        """
        Combine device tag queries with AND
        """
        output = []
        for query in queries:
            if query is None:
                # None means all devices, so no meaning in adding this to the query
                continue
            if query == "":
                # Empty query means no device, so nothing will match.
                return ""
            output.append(query)
        if not output:
            # No specific queries found so we return with query for all devices
            return "device:*"
        return " AND ".join(output)


class DataMapper:
    def __init__(self, mappings: list[dict]):
        """
        Maps data from studio inputs to AVD data model.

        If "convert_value" is set (Currently only supporting "yaml_to_dict"), the data will be converted.

        Parameters
        ----------
        mappings : list[dict]
            List of variable mappings like:
            [
                {
                    "from_path": ["dc_vars", "dc", "role_vars", "role", "bgp_as"],
                    "to_path": ["network_type", "defaults", "bgp_as"],
                    "convert_value": "yaml_to_dict"
                }
            ]
        """
        self.mappings = mappings

    def __mappings_under_path(self, path: list[str | int]) -> list:
        pathlen = len(path)
        return [{**mapping, "from_path": mapping["from_path"][pathlen:]} for mapping in self.mappings if mapping["from_path"][:pathlen] == path]

    def __get_value_from_path(self, path: list, data):
        """Recursive function to walk through data to find value of path. Returns None if not found."""
        if not path:
            return data

        if isinstance(data, dict):
            if path[0] in data:
                return self.__get_value_from_path(path[1:], data[path[0]])

        elif isinstance(data, list) and isinstance(path[0], int):
            if path[0] < len(data):
                return self.__get_value_from_path(path[1:], data[path[0]])

        return None

    def __convert_value(self, convert_value: str, value):
        ctx.alog(f"Converting value {value} using {convert_value}")
        """Convert value if supported. Raise if not."""
        if convert_value == "load_yaml" and isinstance(value, str):
            return yaml_safe_load(value)

        raise ValueError(f"Unsupported convert_value: {convert_value}")

    def __set_value_from_path(self, path: list, data: list | dict, value):
        """Recursive function to walk through data to set value of path, creating any level needed."""
        if not path:
            # Empty path. For dicts we can update value directly.
            if isinstance(data, dict) and isinstance(value, dict):
                ctx.alog(f"Updating value {value} on data {data}")
                data.update(value)
                return

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

    def map_dataset(self, dataset: dict) -> dict:
        """
        Map inputs from given dataset according to mappings given at initilization.
        If mappings do not cover a key, it will not be part of the output (lost!)

        Parameters
        ----------
        dataset : dict
            One dataset like returned by InputParser
            {
                "device_query": "DC_Name:DC2 AND Role:L3leaf",
                "path": ["dc_vars", "dc", "role_vars"],
                "inputs": {
                    "role": {
                        "bgp_as": "5678",
                    },
                },
            }

        Returns
        -------
        dict
            Dataset only containing device_query and mapped_vars
            {
                "device_query": "DC_Name:DC2 AND Role:L3leaf",
                "avd_vars": {"network_type": {"defaults": {"bgp_as": "5678"}}}
            }
        """
        path = dataset["path"]
        inputs = dataset["inputs"]
        output = {
            "device_query": dataset["device_query"],
            "avd_vars": {},
        }
        for mapping in self.__mappings_under_path(path):
            if (value := self.__get_value_from_path(mapping["from_path"], inputs)) is not None:
                if (convert_value := mapping.get("convert_value")) is not None:
                    value = self.__convert_value(convert_value, value)

                self.__set_value_from_path(mapping["to_path"], output["avd_vars"], value)

        return output

    def map_datasets(self, datasets: list[dict]) -> list:
        """
        Run map_dataset for each list item and return list with updated datasets
        """
        return [self.map_dataset(dataset) for dataset in datasets]


def get_raw_studio_inputs():
    wid = StringValue(value=WORKSPACE_ID)
    sid = StringValue(value=STUDIO_ID)
    path = RepeatedString(values=[])
    key = models.InputsKey(studio_id=sid, workspace_id=wid, path=path)

    get_req = InputsRequest(key=key)
    tsclient = ctx.getApiClient(InputsServiceStub)
    try:
        get_res = tsclient.GetOne(get_req)
        return json.loads(get_res.value.inputs.value)
    except Exception:
        # No changes in the workspace. Fallback to mainline
        pass

    wid = StringValue(value="")
    key = models.InputsKey(studio_id=sid, workspace_id=wid, path=path)
    get_req = InputsRequest(key=key)
    tsclient = ctx.getApiClient(InputsServiceStub)
    get_res = tsclient.GetOne(get_req)
    return json.loads(get_res.value.inputs.value)


def transform_studio_inputs_to_avd(raw_studio_inputs: dict, studio_assigned_tagquery: str) -> list[dict]:
    """
    First parse raw studio inputs and expand into a list of data sets
    Next perform variable mapping into AVD variables for each dataset.
    Returns list of datasets with mapped avd_vars.
    """
    parser = InputParser(studio_assigned_tagquery)
    datasets = parser.parse_inputs(raw_studio_inputs)
    mapper = DataMapper(INPUTMAPPINGS)
    return mapper.map_datasets(datasets)


def __resolve_device_tag_query(query):
    if query == "":
        return []
    if query is None:
        query = "device:*"
    tsclient = ctx.getApiClient(TagSearchStub)
    search_req = TagMatchRequestV2(query=query, workspace_id=WORKSPACE_ID, topology_studio_request=True)
    search_res = tsclient.GetTagMatchesV2(search_req)
    return [match.device.device_id for match in search_res.matches] or query


raw_studio_inputs = get_raw_studio_inputs()
ctx.alog(f"{raw_studio_inputs}")

studio_assigned_tagquery = __get_studio_assigned_tagquery()
avd_inputs = transform_studio_inputs_to_avd(raw_studio_inputs, studio_assigned_tagquery)
device_list = __resolve_device_tag_query(studio_assigned_tagquery)

ctx.store(json.dumps(avd_inputs), customKey="avd_inputs", path=["avd"])
ctx.store(json.dumps(device_list), customKey="device_list", path=["avd"])
ctx.alog(f"{avd_inputs}")
ctx.alog(f"{device_list}")
