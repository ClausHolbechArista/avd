# Copyright (c) 2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
# ruff: noqa: PYI021

from pathlib import Path

from . import _validated_data as _validated_data

class OpaqueData:
    """Immutable relaxed-validation payload without Python field accessors."""

    def __bool__(self) -> bool:
        """Return whether the payload contains entries without materializing it."""
    def to_json(self) -> str:
        """Explicitly materialize JSON for facts serialization or a legacy merger."""

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
