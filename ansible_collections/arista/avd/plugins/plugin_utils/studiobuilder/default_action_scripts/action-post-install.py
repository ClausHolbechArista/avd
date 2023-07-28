# Copyright (c) 2022 Arista Networks, Inc.  All rights reserved.
# Arista Networks, Inc. Confidential and Proprietary.
# Subject to Arista Networks, Inc.'s EULA.
# FOR INTERNAL USE ONLY. NOT FOR DISTRIBUTION.
# pylint: skip-file
import requests

hooks = [
    # (stage, scope, actionId, dependsOn?)
    ("BUILD_STAGE_INPUT_VALIDATION", "SCOPE_STUDIO", ctx.action.args["StudioPreBuildActionIDs"].split(",")[0]),
    ("BUILD_STAGE_INPUT_VALIDATION", "SCOPE_WORKSPACE", ctx.action.args["WorkspacePreBuildActionIDs"].split(",")[0]),
    ("BUILD_STAGE_INPUT_VALIDATION", "SCOPE_STUDIO", ctx.action.args["StudioPreRenderActionIDs"].split(",")[0]),
]

cookies = {"access_token": ctx.user.token}

ctx.alog("Postinstall script trying to associate build actions")
# ctx.alog(f"args: {ctx.action.args}")

url = f"https://{ctx.connections.serviceAddr}/api/resources/studio/v1/BuildHookConfig"

# TODO: Remove any previous hooks associated with this studio

prev_hook_id = None
for stage, scope, action in hooks:
    if prev_hook_id:
        dependsOn = {"values": [prev_hook_id]}
    else:
        dependsOn = {}

    data = {
        "key": {
            "workspaceId": ctx.action.args["WorkspaceID"],
            "studioId": ctx.action.args["StudioID"],
            "hookId": action,
        },
        "scope": scope,
        "stage": stage,
        "actionId": action,
        "dependsOn": dependsOn,
    }

    response = requests.post(url, json=data, verify=False, cookies=cookies)

    if response.status_code != 200:
        ctx.alog("Postinstall script failure suring association of build actions")
        raise RuntimeError(response.text)

ctx.alog("Postinstall script done.")
