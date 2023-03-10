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
"""

import json
import re

from arista.studio.v1 import models
from arista.studio.v1.services import AssignedTagsServiceStub, InputsServiceStub
from arista.studio.v1.services.gen_pb2 import AssignedTagsRequest, InputsRequest
from fmp.wrappers_pb2 import RepeatedString
from google.protobuf.wrappers_pb2 import StringValue
from tagsearch_python.tagsearch_pb2 import TagMatchRequestV2, TagValueSearchRequest
from tagsearch_python.tagsearch_pb2_grpc import TagSearchStub

DATA_MAPS = []
WORKSPACE_ID = ctx.action.args["WorkspaceID"]
STUDIO_ID = ctx.action.args["StudioID"]


def __get_studio_devices():
    # First get the query string from the studio
    get_req = AssignedTagsRequest()
    get_req.key.workspace_id.value = WORKSPACE_ID
    get_req.key.studio_id.value = STUDIO_ID
    tsclient = ctx.getApiClient(AssignedTagsServiceStub)
    try:
        tag = tsclient.GetOne(get_req)
    except Exception:
        get_req.key.workspace_id.value = ""
        tag = tsclient.GetOne(get_req)
    query = tag.value.query.value
    # Now try to search with this query to get the matching devices
    tsclient = ctx.getApiClient(TagSearchStub)
    search_req = TagMatchRequestV2(query=query, workspace_id=WORKSPACE_ID, topology_studio_request=True)
    search_res = tsclient.GetTagMatchesV2(search_req)
    return [match.device.device_id for match in search_res.matches] or query


def __get_device_tags(device_id, labels):
    tsclient = ctx.getApiClient(TagSearchStub)
    matching_tags = []
    for label in labels:
        label_search_req = TagValueSearchRequest(label=label, workspace_id=WORKSPACE_ID, topology_studio_request=True)
        for tag in tsclient.GetTagValueSuggestions(label_search_req).tags:
            query = '{}:"{}" AND device:{}'.format(tag.label, tag.value, device_id)
            value_search_req = TagMatchRequestV2(query=query, workspace_id=WORKSPACE_ID, topology_studio_request=True)
            value_search_res = tsclient.GetTagMatchesV2(value_search_req)
            for match in value_search_res.matches:
                if match.device.device_id == device_id:
                    matching_tags.append(tag)
    return {tag.label: tag.value for tag in matching_tags}


class InputParser:
    """
    Transform raw studio input into a list of data sets to be applied for device queries
    Example with first common vars and then a resolver with two different tag queries:
    [
        {
            "device_queries": [],
            "path": [],
            "inputs": {"p2p_uplinks_mtu": 9000},
        },
        {
            "inputs": {"dc": {}},
            "path": ["dc_vars"],
            "device_queries": ["DC_Name:DC1"],
        },
        {
            "device_queries": ["DC_Name:DC2"],
            "path": ["dc_vars"],
            "inputs": {"dc": {"mgmt_gateway": "10.10.10.1"}},
        }
    ]
    """
    queue: list

    def __init__(self, interface_resolver_paths: list[list[str]] = []):
        self.interface_resolver_paths = interface_resolver_paths
        self.queue = []

    def parse_inputs(self, inputs):
        self.queue.append({"inputs": inputs, "path": [], "device_queries": []})
        datasets = []

        while self.queue:
            queue_item = self.queue.pop(0)
            dataset = queue_item.copy()
            dataset["inputs"] = self._parse_inputs(**queue_item)
            if not dataset["inputs"]:
                continue
            datasets.append(dataset)

        return datasets

    def _parse_inputs(self, inputs, path=[], device_queries=[]):
        if isinstance(inputs, dict):
            parsed = {}
            for key, val in inputs.items():
                if (
                    parsed_val := self._parse_inputs(val, path + [key], device_queries)
                ) is not None:
                    parsed[key] = parsed_val
            return parsed

        if isinstance(inputs, list) and inputs:
            if (
                isinstance(inputs[0], dict)
                and "tags" in inputs[0]
                and "inputs" in inputs[0]
            ):
                # this is a type of resolver
                # check which one - default to device resolver
                if path in self.interface_resolver_paths:
                    # TODO: Implement something for interface resolvers
                    pass
                else:
                    for resolver_item in inputs:
                        self.queue.append(
                            {
                                "inputs": resolver_item["inputs"],
                                "path": path,
                                "device_queries": device_queries
                                + [resolver_item["tags"]["query"]],
                            }
                        )
                    return None

            # collection type
            return [
                self._parse_inputs(element, path + [index], device_queries)
                for index, element in enumerate(inputs)
            ]
        # base case for other python basic classes
        return inputs


def build_key(studio_id, workspace_id, path=[]):
    # Get the inputs for the workspace/studioId
    wid = StringValue(value=workspace_id)
    sid = StringValue(value=studio_id)
    path = RepeatedString(values=path)
    key = models.InputsKey(studio_id=sid, workspace_id=wid, path=path)
    return key


def get_raw_studio_inputs(studio_id, workspace_id):
    get_req = InputsRequest(key=build_key(studio_id, workspace_id))
    tsclient = ctx.getApiClient(InputsServiceStub)
    get_res = tsclient.GetOne(get_req)
    return json.loads(get_res.value.inputs.value)


def transform_studio_inputs_to_AVD(raw_studio_inputs: dict):
    """
    First parse raw studio inputs and expand into a list of data sets
    Next for each data set perform variable mapping into AVD variables
    """
    parser = InputParser()
    parser.parse_inputs(raw_studio_inputs)


def __strip_and_resolve(_input, _device):
    # Resolve any nested resolvers and strip empty strings
    if isinstance(_input, DeviceResolver):
        _resolved_input = _input.resolve(device=_device)
        return __strip_and_resolve(_resolved_input, _device)
    if isinstance(_input, dict):
        _data = {}
        for _key in _input:
            _tmp_value = __strip_and_resolve(_input[_key], _device)
            if _tmp_value is None:
                continue
            _data[_key] = _tmp_value
        return _data
    if isinstance(_input, list):
        _data = []
        for _item in _input:
            _tmp_value = __strip_and_resolve(_item, _device)
            if _tmp_value is None:
                continue
            _data.append(_tmp_value)
        return _data
    if isinstance(_input, str):
        if _input == "":
            return None
    return _input


def __node_groups(_data_maps, _input_data, _tag_data):
    _node_groups = {}
    for _device in sorted(_input_data):
        for _data_map in _data_maps:
            if "tag" in _data_map and _data_map.get("type") == "node_group":
                value = _tag_data.get(_device, {}).get(_data_map["tag"])
                if value is None:
                    break
                _node_groups.setdefault(value, {"nodes": {}})
                _node_groups[value]["nodes"][_device] = {}
                break
    return _node_groups


def map_data(data_maps, studio_inputs, start_path):
    output = {}
    pointer = output
    for element in start_path:
        if isinstance(element, str):


    value = None
    for _data_map in _data_maps:
        if "input" in _data_map:
            # Get value from studio_inputs
            data_pointer = studio_inputs[device]
            keys = _data_map["input"].split("-")
            # First key in path would be "root" so we skip that
            for key in keys[1:-1]:
                if data_pointer.get(key) is None:
                    data_pointer = {}
                    break
                data_pointer = data_pointer[key]
            value = data_pointer.get(keys[-1])
            if value is None:
                continue

        if "tag" in _data_map:
            # Get value from _tag_data
            value = _tag_data.get(_device, {}).get(_data_map["tag"])
            if value is None:
                continue
            if _data_map.get("type") == "int":
                value = int(value)
            elif _data_map.get("type") == "list":
                value = value.split(",")

        if (value is not None) and "avd_var" in _data_map:
            # Set avd_var with value
            data_pointer = studio_inputs[device]
            keys = list(map(lambda p: p.replace("\\\\.", "."), re.split(r"(?<!\\\\)\.", _data_map["avd_var"])))

            for key in keys[:-1]:
                data_pointer.setdefault(key, {})
                data_pointer = data_pointer[key]
            data_pointer[keys[-1]] = value

ctx.alog(f"{get_studio_inputs()}")
