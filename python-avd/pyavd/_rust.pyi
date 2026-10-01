# Copyright (c) 2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
# ruff: noqa: PYI021

from pathlib import Path

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

def archive_eos_designs(input_json: str, destination: Path, schema_archive: Path) -> PublicationResult:
    """Validate one host's AVD design inputs and publish an archive when valid."""
