# Copyright (c) 2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.

from __future__ import annotations

import json
from pathlib import Path

from pyavd._rust import archive_avd_design


def _schema_archive() -> Path:
    return Path(__file__).parents[2] / "pyavd/_schema/schemas.rkyv"


def test_archive_avd_design_publishes_valid_input(tmp_path: Path) -> None:
    """Publish the validated representation and return its caller-selected path."""
    destination = tmp_path / "host.rkyv"
    result = archive_avd_design(
        '{"fabric_name": "TEST", "devices": []}',
        destination,
        _schema_archive(),
    )

    assert result.destination == destination
    assert destination.is_file()
    assert json.loads(result.input_diagnostics_json) == []
    assert json.loads(result.errors_json) == []


def test_archive_avd_design_does_not_publish_invalid_schema_data(tmp_path: Path) -> None:
    """Return validation errors without publishing rejected input."""
    destination = tmp_path / "host.rkyv"
    result = archive_avd_design(
        '{"devices": 42}',
        destination,
        _schema_archive(),
    )

    assert result.destination is None
    assert not destination.exists()
    errors = json.loads(result.errors_json)
    assert errors


def test_archive_avd_design_does_not_publish_invalid_json(tmp_path: Path) -> None:
    """Return parse diagnostics without publishing an incomplete archive."""
    destination = tmp_path / "host.rkyv"
    result = archive_avd_design(
        "{",
        destination,
        _schema_archive(),
    )

    assert result.destination is None
    assert not destination.exists()
    input_diagnostics = json.loads(result.input_diagnostics_json)
    assert input_diagnostics
