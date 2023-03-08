<%  # noqa: W605
import ansible_runner
import json
import re
from arista.studio.v1.services import AssignedTagsServiceStub
from arista.studio.v1.services.gen_pb2 import AssignedTagsRequest
from tagsearch_python.tagsearch_pb2_grpc import TagSearchStub
from tagsearch_python.tagsearch_pb2 import TagMatchRequestV2, TagValueSearchRequest
from webapp.v2.types import DeviceResolver

DATA_MAPS = []

def __get_studio_devices():
    # First get the query string from the studio
    get_req = AssignedTagsRequest()
    get_req.key.workspace_id.value = ctx.studio.workspaceId
    get_req.key.studio_id.value = ctx.studio.studioId
    tsclient = ctx.getApiClient(AssignedTagsServiceStub)
    try:
        tag = tsclient.GetOne(get_req)
    except Exception:
        get_req.key.workspace_id.value = ""
        tag = tsclient.GetOne(get_req)
    query = tag.value.query.value
    # Now try to search with this query to get the matching devices
    tsclient = ctx.getApiClient(TagSearchStub)
    search_req = TagMatchRequestV2(query=query, workspace_id=ctx.studio.workspaceId, topology_studio_request=True)
    search_res = tsclient.GetTagMatchesV2(search_req)
    return [match.device.device_id for match in search_res.matches] or query

def __get_device_tags(device_id, labels):
    tsclient = ctx.getApiClient(TagSearchStub)
    matching_tags = []
    for label in labels:
        label_search_req = TagValueSearchRequest(label=label, workspace_id=ctx.studio.workspaceId, topology_studio_request=True)
        for tag in tsclient.GetTagValueSuggestions(label_search_req).tags:
            query= '{}:\"{}\" AND device:{}'.format(tag.label, tag.value, device_id)
            value_search_req = TagMatchRequestV2(query=query, workspace_id=ctx.studio.workspaceId, topology_studio_request=True)
            value_search_res = tsclient.GetTagMatchesV2(value_search_req)
            for match in value_search_res.matches:
                if match.device.device_id == device_id:
                    matching_tags.append(tag)
    return {tag.label: tag.value for tag in matching_tags}

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

def __map_data(_data_maps, _input_data, _tag_data, _node_groups):
    _output = {}
    value = None
    for _device in _input_data:
        for _data_map in _data_maps:
            if "input" in _data_map:
                # Get value from _input_data
                data_pointer = _input_data[_device]
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
                    value = value.split(',')
                elif _data_map.get("type") == "node_group":
                    value = {str(value): _node_groups.get(value, {})}

            if (not value is None) and "avd_var" in _data_map:
                # Set avd_var with value
                data_pointer = _input_data[_device]
                keys = list(map(lambda p: p.replace('\\\\.', '.'), re.split(r'(?<!\\\\)\.', _data_map["avd_var"])))

                # Hack to insert ex. "l3leaf" instead of "node_type_key"
                if keys[0] == "node_type_keys.key":
                    keys[0] = _tag_data.get(_device, {}).get("Role", "node_type_keys.key")

                # Hack to insert ex. "servers" instead of "connected_endpoints_keys.key"
                if keys[0] == "connected_endpoints_keys.key":
                    keys[0] = "servers"

                for key in keys[:-1]:
                    data_pointer.setdefault(key, {})
                    data_pointer = data_pointer[key]
                data_pointer[keys[-1]] = value

_studio_devices = __get_studio_devices()
_all_devices = [device for device in ctx.topology.getDevices() if device.id in _studio_devices]
_this_hostname = ctx.getDevice().hostName
_tag_data = {}
_input_data = {}
for _device in _all_devices:
    _mapped_device_tags = [data_map["tag"] for data_map in DATA_MAPS if "tag" in data_map]
    _tag_data[_device.hostName] = __get_device_tags(_device.id, _mapped_device_tags)
    _input_data[_device.hostName] = {}
    for _input_name, _input in ctx.studio.inputs.items():
        _input_data[_device.hostName][_input_name] = __strip_and_resolve(_input, _device.id)

_node_groups = __node_groups(DATA_MAPS, _input_data, _tag_data)
__map_data(DATA_MAPS, _input_data, _tag_data, _node_groups)
_input_data = __strip_and_resolve(_input_data, None)

_fabric_name = ctx.studio.inputs.get('fabric_name', "no_fabric_name")

# Run Ansible AVD passing inventory, variables and playbook as arguments.

_inventory={
    "all": {
        "children": {
            _fabric_name: {
                "hosts": _input_data
            }
        }
    }
}

_runner_result = ansible_runner.interface.run(
    envvars={
        "ANSIBLE_JINJA2_EXTENSIONS": "jinja2.ext.loopcontrols,jinja2.ext.do,jinja2.ext.i18n"
    },
    inventory=_inventory,
    skip_tags="documentation",
    verbosity=0,
    limit=_this_hostname,
    playbook=[
        {
            "name": "Run AVD",
            "hosts": _fabric_name,
            "gather_facts": "false",
            "connection": "local",
            "tasks": [
                {
                    "import_role": {
                        "name": "arista.avd.eos_designs",
                        "tasks_from": "studios"
                    },
                },
                {
                    "name": "generate device intended config and documentation",
                    "import_role": {
                        "name": "arista.avd.eos_cli_config_gen",
                        "tasks_from": "studios"
                    }
                },
            ]
        }
    ],
    json_mode=False,
    quiet=True
)
_result = ""
for _event in _runner_result.host_events(_this_hostname):
    _event_data = _event.get('event_data',{})
    if _event_data.get('role') == 'eos_cli_config_gen':
        _result = _event['event_data'].get('res',{}).get('ansible_facts',{}).get('eos_config')
        if _result:
            break
if not _result:
    with _runner_result.stdout as _output:
        _result = _output.read()
else:
    ctx.store(json.dumps(_inventory), key="avd_last_build_inventory", path="avd")
%>
! ${f"{_input_data.get(_this_hostname)}"}
${_result}
