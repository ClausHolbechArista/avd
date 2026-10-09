# Copyright (c) 2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
# ruff: noqa: PYI021

from pathlib import Path
from typing import Any

class _ValueHandle: ...

class OpaqueData:
    """Immutable relaxed-validation payload without Python field accessors."""

    def __init__(self, handle: _ValueHandle) -> None: ...
    def __bool__(self) -> bool:
        """Return whether the payload contains entries without materializing it."""
    def to_json(self) -> str:
        """Explicitly materialize JSON for facts serialization or a legacy merger."""

class _DictView:
    def __init__(self, handle: _ValueHandle) -> None: ...
    def _get_field(self, slot: int) -> Any: ...

class _ListView:
    def __init__(self, handle: _ValueHandle) -> None: ...
    def __len__(self) -> int: ...
    def _get_item(self, index: int) -> Any: ...
    def _get_by_primary_key(self, components: tuple[bool | int | str, ...]) -> Any: ...
    def _contains_primary_key(self, components: tuple[bool | int | str, ...]) -> bool: ...

class PublicationResult:
    """Diagnostics and optional path from validated-data archive publication."""

    @property
    def destination(self) -> Path | None:
        """Published path, or None when validation rejected the input."""
    @property
    def input_diagnostics_json(self) -> str:
        """JSON-encoded input parse diagnostics."""
    @property
    def errors_json(self) -> str:
        """JSON-encoded schema validation errors."""
    @property
    def warnings_json(self) -> str:
        """JSON-encoded schema validation warnings."""
    @property
    def infos_json(self) -> str:
        """JSON-encoded coercion and informational diagnostics."""

def archive_avd_design(input_json: str, destination: Path, schema_archive: Path) -> PublicationResult:
    """Validate one host's AVD Design inputs and publish an archive when valid."""

def _open_avd_design_handle(archive: Path, schema_archive: Path) -> _ValueHandle: ...
