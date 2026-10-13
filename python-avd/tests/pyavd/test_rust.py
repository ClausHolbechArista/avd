# Copyright (c) 2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.

from __future__ import annotations

import json
from pathlib import Path

from pyavd import _validated_data
from pyavd._rust import OpaqueData, archive_avd_design
from pyavd._utils.undefined import Undefined


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


def test_open_avd_design_exposes_typed_immutable_views(tmp_path: Path) -> None:
    """Keep lazy typed child views usable independently of their parent views."""
    destination = tmp_path / "host.rkyv"
    result = archive_avd_design(
        json.dumps(
            {
                "fabric_name": "TEST",
                "devices": [
                    {
                        "name": "leaf1",
                        "id": 42,
                        "vtep": True,
                        "serial_number": None,
                        "uplink_interfaces": ["Ethernet1", "Ethernet2"],
                    }
                ],
            }
        ),
        destination,
        _schema_archive(),
    )
    assert result.destination == destination

    root = _validated_data.open_avd_design(destination, _schema_archive())
    assert isinstance(root, _validated_data.AVDDesign)
    assert root.fabric_name == "TEST"
    devices = root.devices
    assert devices is not Undefined
    assert devices is not None
    assert len(devices) == 1
    assert "leaf1" in devices
    assert list(devices.keys()) == ["leaf1"]
    assert [item.name for item in devices.values()] == ["leaf1"]
    assert [(key, item.name) for key, item in devices.items()] == [("leaf1", "leaf1")]
    assert devices.get("missing") is Undefined

    device = devices["leaf1"]
    assert device.name == "leaf1"
    assert device.id == 42
    assert device.vtep is True
    assert device.serial_number is None
    assert device.platform is Undefined

    uplink_interfaces = device.uplink_interfaces
    assert uplink_interfaces is not Undefined
    assert uplink_interfaces is not None
    assert uplink_interfaces[-1] == "Ethernet2"
    del root, devices, device
    assert list(uplink_interfaces) == ["Ethernet1", "Ethernet2"]


def test_relaxed_payload_is_opaque_and_retains_its_archive(tmp_path: Path) -> None:
    """Allow partial custom config without exposing strict descendant guarantees to Python."""
    destination = tmp_path / "host.rkyv"
    payload = {"ethernet_interfaces": [{"name": "Ethernet1", "description": None}]}
    result = archive_avd_design(
        json.dumps({"fabric_name": "TEST", "devices": [{"name": "leaf1", "structured_config": payload}]}),
        destination,
        _schema_archive(),
    )
    assert result.destination == destination
    root = _validated_data.open_avd_design(destination, _schema_archive())
    config = root.devices["leaf1"].structured_config
    assert isinstance(config, OpaqueData)
    assert bool(config)
    assert not hasattr(config, "ethernet_interfaces")
    del root
    assert json.loads(config.to_json()) == payload


def test_archive_rejects_null_required_fields(tmp_path: Path) -> None:
    """Reject explicit null before publishing data promising non-null required fields."""
    destination = tmp_path / "host.rkyv"
    result = archive_avd_design(
        json.dumps({"fabric_name": None, "devices": []}),
        destination,
        _schema_archive(),
    )
    assert result.destination is None
    assert not destination.exists()
    assert json.loads(result.errors_json)


def test_duplicate_primary_keys_keep_positional_items_and_guaranteed_keys(tmp_path: Path) -> None:
    """Keep both hybrid-list entries while exposing their structurally required key fields."""
    destination = tmp_path / "host.rkyv"
    result = archive_avd_design(
        json.dumps(
            {
                "fabric_name": "TEST",
                "devices": [],
                "network_services": [
                    {
                        "name": "TENANT",
                        "l2vlans": [{"id": 10, "name": "first"}, {"id": 10, "name": "second"}],
                    }
                ],
            }
        ),
        destination,
        _schema_archive(),
    )
    assert result.destination == destination
    root = _validated_data.open_avd_design(destination, _schema_archive())
    vlans = root.network_services["TENANT"].l2vlans
    assert len(vlans) == 2
    assert [(vlan.id, vlan.name) for vlan in vlans] == [(10, "first"), (10, "second")]
    assert vlans[0].id == 10
    assert vlans[-1].name == "second"
    assert not hasattr(vlans, "keys")
    assert not hasattr(vlans, "get")
    del root
    assert [vlan.id for vlan in vlans[:]] == [10, 10]


def test_hybrid_primary_key_presence_is_enforced_inside_relaxed_payload(tmp_path: Path) -> None:
    """A duplicate-key list still requires its structural key under relaxed custom config."""
    for item in [{"vrf": "default"}, {"ip_address": None, "vrf": "default"}]:
        destination = tmp_path / "host.rkyv"
        result = archive_avd_design(
            json.dumps(
                {
                    "fabric_name": "TEST",
                    "devices": [
                        {
                            "name": "leaf1",
                            "structured_config": {"ip_name_server_groups": [{"name": "DNS", "name_servers": [item]}]},
                        }
                    ],
                }
            ),
            destination,
            _schema_archive(),
        )
        assert result.destination is None
        assert not destination.exists()
        assert json.loads(result.errors_json)
