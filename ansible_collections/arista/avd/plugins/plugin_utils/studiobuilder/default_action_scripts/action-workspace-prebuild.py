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

from deepmerge import always_merger
from pyavd import eos_designs_facts
from tagsearch_python.tagsearch_pb2 import TagMatchRequestV2, TagValueSearchRequest
from tagsearch_python.tagsearch_pb2_grpc import TagSearchStub

WORKSPACE_ID = ctx.action.args["WorkspaceID"]
STUDIO_ID = ctx.action.args["StudioID"]


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
    tsclient = ctx.getApiClient(TagSearchStub)
    matching_tags = []
    for label in labels:
        label_search_req = TagValueSearchRequest(label=label, workspace_id=WORKSPACE_ID)
        for tag in tsclient.GetTagValueSuggestions(label_search_req).tags:
            query = '{}:"{}" AND device:{}'.format(tag.label, tag.value, device_id)
            value_search_req = TagMatchRequestV2(query=query, workspace_id=WORKSPACE_ID)
            value_search_res = tsclient.GetTagMatchesV2(value_search_req)
            for match in value_search_res.matches:
                if match.device.device_id == device_id:
                    matching_tags.append(tag)
    return {tag.label: tag.value for tag in matching_tags}


def __build_device_vars(datasets: list[dict]):
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
        hostname1 : dict
        hostname2 : dict
    """
    device_vars = {}
    for dataset in datasets:
        devices = __resolve_device_tag_query(dataset["device_query"])
        for device in devices:
            one_device_vars = device_vars.setdefault(device, {})
            always_merger.merge(one_device_vars, deepcopy(dataset["avd_vars"]))
    return device_vars


avd_inputs = json.loads(ctx.retrieve(path=["avd"], customKey="avd_inputs", delete=False))
device_list = json.loads(ctx.retrieve(path=["avd"], customKey="device_list", delete=False))

device_vars = __build_device_vars(avd_inputs)
avd_switch_facts = eos_designs_facts(device_vars)
ctx.store(json.dumps(avd_switch_facts), customKey="avd_switch_facts", path=["avd"])
