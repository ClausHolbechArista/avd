# Copyright (c) 2023 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
from __future__ import annotations

from ..api.arista.workspace.v1 import WorkspaceState
from ..client import CVClient
from ..client.exceptions import CVResourceInvalidState, CVResourceNotFound
from ..models import CVWorkspace


async def create_workspace_on_cv(workspace: CVWorkspace, cv_client: CVClient) -> None:
    """
    Create or update a Workspace from the given workspace object.
    In-place update the workspace state.
    """
    try:
        existing_workspace = await cv_client.get_workspace(workspace_id=workspace.id)
        if existing_workspace.state == WorkspaceState.WORKSPACE_STATE_PENDING:
            workspace.final_state = "pending"
        else:
            raise CVResourceInvalidState("The requested workspace is not in state 'pending'")
    except CVResourceNotFound:
        await cv_client.create_workspace(workspace_id=workspace.id, display_name=workspace.name, description=workspace.description)
        workspace.final_state = "pending"
