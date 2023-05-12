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

from deepmerge import always_merger
from pyavd import eos_designs_structured_configs, eos_cli_config_gen
from tagsearch_python.tagsearch_pb2 import TagMatchRequestV2, TagValueSearchRequest
from tagsearch_python.tagsearch_pb2_grpc import TagSearchStub

# Get studio info from ctx
DEVICE = ctx.getDevice()
DEVICE_ID = DEVICE.id
WORKSPACE_ID = ctx.studio.workspaceId

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
        hostname1 : dict
        hostname2 : dict
    """
    one_device_vars = {}
    for dataset in datasets:
        devices = __resolve_device_tag_query(dataset["device_query"])
        if device_id in devices:
            always_merger.merge(one_device_vars, deepcopy(dataset["avd_vars"]))
    return one_device_vars


avd_inputs =       json.loads(ctx.retrieve(path=["avd"], customKey="avd_inputs", delete=False))
avd_switch_facts = json.loads(ctx.retrieve(path=["avd"], customKey="avd_switch_facts", delete=False))

device_vars = __build_device_vars(avd_inputs, DEVICE_ID)
device_vars.update(avd_switch_facts)
device_vars["switch"] = avd_switch_facts["avd_switch_facts"][DEVICE_ID]["switch"]

### TMP/HACKS
device_vars.setdefault("default_igmp_snooping_enabled", True)

ctx.store(json.dumps(device_vars), customKey="device_vars", path=["avd"])

structured_config = eos_designs_structured_configs(DEVICE_ID, device_vars)
ctx.store(json.dumps(structured_config), customKey=f"{DEVICE_ID}_structured_config", path=["avd"])

eos_config, _ = eos_cli_config_gen(DEVICE_ID, structured_config)
ctx.store(json.dumps(eos_config), customKey=f"{DEVICE_ID}_config", path=["avd"])

%>
${eos_config}