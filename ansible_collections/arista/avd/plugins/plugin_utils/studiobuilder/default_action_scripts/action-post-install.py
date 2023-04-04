import requests

cookies = {"access_token": ctx.user.token}

ctx.alog("Postinstall script trying to associate build actions")
# ctx.alog(f"args: {ctx.action.args}")

url = f"https://{ctx.connections.serviceAddr}/api/resources/studio/v1/StudioConfig"
data = {
    "key": {
        "workspace_id": ctx.action.args["WorkspaceID"],
        "studio_id": ctx.action.args["StudioID"],
    },
    "studioPreBuildActions": {
        "values": ctx.action.args["StudioPreBuildActionIDs"].split(","),
    },
    "studioPreRenderActions": {
        "values": ctx.action.args["StudioPreRenderActionIDs"].split(","),
    },
    "workspacePreBuildActions": {
        "values": ctx.action.args["WorkspacePreBuildActionIDs"].split(","),
    },
}

response = requests.post(url, json=data, verify=False, cookies=cookies)

if response.status_code != 200:
    ctx.alog("Postinstall script failure suring association of build actions")
    raise RuntimeError(response.text)

ctx.alog("Postinstall script done.")
