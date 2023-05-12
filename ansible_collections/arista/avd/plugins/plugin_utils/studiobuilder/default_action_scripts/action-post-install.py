# Copyright (c) 2022 Arista Networks, Inc.  All rights reserved.
# Arista Networks, Inc. Confidential and Proprietary.
# Subject to Arista Networks, Inc.'s EULA.
# FOR INTERNAL USE ONLY. NOT FOR DISTRIBUTION.
# pylint: skip-file
import requests

hooks = [
    ("BUILD_STAGE_STUDIO_PRE_BUILD", ctx.action.args["StudioPreBuildActionIDs"].split(",")[0]),
    ("BUILD_STAGE_WORKSPACE_PRE_BUILD", ctx.action.args["WorkspacePreBuildActionIDs"].split(",")[0]),
    ("BUILD_STAGE_STUDIO_PRE_RENDER", ctx.action.args["StudioPreRenderActionIDs"].split(",")[0]),
]

cookies = {"access_token": ctx.user.token}

ctx.alog("Postinstall script trying to associate build actions")
# ctx.alog(f"args: {ctx.action.args}")

url = f"https://{ctx.connections.serviceAddr}/api/resources/studio/v1/BuildHookConfig"

for stage, action in hooks:
    data = {
        "key": {
            "workspaceId": ctx.action.args["WorkspaceID"],
            "studioId": ctx.action.args["StudioID"],
            "stage": stage,
            "hookId": "something",
        },
        "actionId": action,
    }

    response = requests.post(url, json=data, verify=False, cookies=cookies)

    if response.status_code != 200:
        ctx.alog("Postinstall script failure suring association of build actions")
        raise RuntimeError(response.text)

ctx.alog("Postinstall script done.")
