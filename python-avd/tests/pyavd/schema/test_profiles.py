# Copyright (c) 2024-2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
from __future__ import annotations

from typing import ClassVar

import pytest

from pyavd._schema.models.avd_indexed_list import AvdIndexedList
from pyavd._schema.models.avd_model import AvdModel
from pyavd._schema.models.avd_profile import AvdProfileResolver
from pyavd._schema.models.avd_profile_ref import AvdProfileRef
from pyavd._schema.models.eos_designs_root_model import EosDesignsRootModel
from pyavd.api.schemas import ConsolidatedAVDDesign


class ProfileTestRootModel(EosDesignsRootModel):
    """Minimal EosDesignsRootModel for profile resolver tests."""

    class _DynamicKeys(AvdModel):
        _dynamic_key_maps: ClassVar[list[dict]] = []
        _fields: ClassVar[dict] = {}

    _allow_other_keys = True


def load_with_profiles(schema_cls: type[ProfileTestRootModel], data: dict, load_custom_structured_config: bool = False) -> ProfileTestRootModel:
    """Load test schema data and apply profiles with an explicit resolver."""
    profile_resolver = AvdProfileResolver()
    profiles = profile_resolver._detect_profile_refs(schema_cls, data, load_custom_structured_config)
    result = schema_cls._from_dict(data, load_custom_structured_config=load_custom_structured_config)
    return profile_resolver._apply_profiles(result, profiles)


def test_avd_model_stuff() -> None:
    class SomeModel(AvdModel):
        _fields: ClassVar[dict] = {
            "number": {"type": int},
            "string": {"type": str},
        }
        number: int
        string: str

    class ProfiledModel(AvdModel):
        _fields: ClassVar[dict] = {
            "example_profile": {"type": AvdProfileRef, "catalog": "source", "target": "profiled_model/some_model"},
            "some_model": {"type": SomeModel},
        }

        some_model: SomeModel
        example_profile: AvdProfileRef | None

    class DemoSchema(ProfileTestRootModel):
        _fields: ClassVar[dict] = {
            "profiled_model": {"type": ProfiledModel},
        }

        profiled_model: ProfiledModel

    data = load_with_profiles(
        DemoSchema,
        {
            "profiled_model": {
                "example_profile": "test",
                "some_model": {
                    "number": 10,
                },
            },
            "source": [
                {
                    "string": "some-string",
                    "profile": "test",
                },
            ],
        },
    )
    assert data.profiled_model.some_model.number == 10
    assert data.profiled_model.some_model.string == "some-string"


def test_avd_profile_resolver_applies_device_profiles_to_consolidated_avd_designs() -> None:
    class ProfiledConsolidatedAVDDesign(ConsolidatedAVDDesign):
        _fields: ClassVar[dict] = {
            **ConsolidatedAVDDesign._fields,
            "consolidated_profile": {"type": AvdProfileRef, "catalog": "consolidated_profiles", "target": "consolidated"},
        }

        consolidated_profile: AvdProfileRef | None

    inputs = {
        "fabric_name": "FABRIC",
        "devices": [
            {
                "name": "leaf1",
                "type": "l2leaf",
            },
            {
                "name": "leaf2",
                "type": "l2leaf",
            },
        ],
        "dns_settings_profile": None,
    }
    profile_catalog = [
        {
            "profile": "leaf1_profile",
            "type": "leaf1_type_from_profile",
            "group": "LEAF1_GROUP_FROM_PROFILE",
            "mlag": True,
            "node_group_length": 11,
        },
        {
            "profile": "leaf2_profile",
            "type": "leaf2_type_from_profile",
            "group": "LEAF2_GROUP_FROM_PROFILE",
            "mlag": False,
            "node_group_length": 22,
        },
    ]
    dns_settings_profile_catalog = [
        {
            "profile": "leaf1_dns_profile",
            "domain": "leaf1.example.com",
            "domain_list": ["leaf1.example.com"],
        },
        {
            "profile": "leaf2_dns_profile",
            "domain": "leaf2.example.com",
            "domain_list": ["leaf2.example.com"],
        },
    ]

    results = {}
    for device_name, profile_name, dns_settings_profile_name in (
        ("leaf1", "leaf1_profile", "leaf1_dns_profile"),
        ("leaf2", "leaf2_profile", "leaf2_dns_profile"),
    ):
        inputs["dns_settings_profile"] = dns_settings_profile_name
        consolidated_avd_design = ConsolidatedAVDDesign._from_avd_design(device_name, inputs)
        profiled_data = {
            **consolidated_avd_design._dump(),
            "consolidated_profile": profile_name,
            "consolidated_profiles": profile_catalog,
            "dns_settings_profiles": dns_settings_profile_catalog,
        }

        profile_resolver = AvdProfileResolver()
        profiles = profile_resolver._detect_profile_refs(ProfiledConsolidatedAVDDesign, profiled_data)
        result = ProfiledConsolidatedAVDDesign._from_dict(profiled_data)
        results[device_name] = profile_resolver._apply_profiles(result, profiles)

    assert results["leaf1"].consolidated.type == "leaf1_type_from_profile"
    assert results["leaf1"].consolidated.group == "LEAF1_GROUP_FROM_PROFILE"
    assert results["leaf1"].consolidated.mlag is True
    assert results["leaf1"].consolidated.node_group_length == 11
    assert results["leaf2"].consolidated.type == "leaf2_type_from_profile"
    assert results["leaf2"].consolidated.group == "LEAF2_GROUP_FROM_PROFILE"
    assert results["leaf2"].consolidated.mlag is False
    assert results["leaf2"].consolidated.node_group_length == 22
    assert results["leaf1"].inputs.dns_settings.domain == "leaf1.example.com"
    assert list(results["leaf1"].inputs.dns_settings.domain_list) == ["leaf1.example.com"]
    assert results["leaf2"].inputs.dns_settings.domain == "leaf2.example.com"
    assert list(results["leaf2"].inputs.dns_settings.domain_list) == ["leaf2.example.com"]


def test_avd_profile_with_deep_source_and_target() -> None:
    class SomeModel(AvdModel):
        _fields: ClassVar[dict] = {
            "number": {"type": int},
            "string": {"type": str},
        }
        number: int
        string: str

    class TargetContainer(AvdModel):
        _fields: ClassVar[dict] = {
            "some_model": {"type": SomeModel},
        }
        some_model: SomeModel

    class Settings(AvdModel):
        _fields: ClassVar[dict] = {
            "target_container": {"type": TargetContainer},
        }
        target_container: TargetContainer

    class ProfiledModel(AvdModel):
        _fields: ClassVar[dict] = {
            "example_profile": {
                "type": AvdProfileRef,
                "catalog": "profile_catalog/nested/profiles",
                "target": "profiled_model/settings/target_container/some_model",
            },
            "settings": {"type": Settings},
        }

        settings: Settings
        example_profile: AvdProfileRef | None

    class DemoSchema(ProfileTestRootModel):
        _fields: ClassVar[dict] = {
            "profiled_model": {"type": ProfiledModel},
        }

        profiled_model: ProfiledModel

    data = load_with_profiles(
        DemoSchema,
        {
            "profiled_model": {
                "example_profile": "test",
                "settings": {
                    "target_container": {
                        "some_model": {
                            "number": 10,
                        },
                    },
                },
            },
            "profile_catalog": {
                "nested": {
                    "profiles": [
                        {
                            "profile": "test",
                            "string": "some-string",
                        },
                    ],
                },
            },
        },
    )

    assert data.profiled_model.settings.target_container.some_model.number == 10
    assert data.profiled_model.settings.target_container.some_model.string == "some-string"


def test_avd_profile_resolves_profiles_on_nested_model() -> None:
    class SomeModel(AvdModel):
        _fields: ClassVar[dict] = {
            "number": {"type": int},
            "string": {"type": str},
        }
        number: int
        string: str

    class NestedSchema(AvdModel):
        _fields: ClassVar[dict] = {
            "nested_profile": {"type": AvdProfileRef, "catalog": "source", "target": "nested/some_model"},
            "some_model": {"type": SomeModel},
        }

        some_model: SomeModel
        nested_profile: AvdProfileRef | None

    class DemoSchema(ProfileTestRootModel):
        _fields: ClassVar[dict] = {
            "nested": {"type": NestedSchema},
        }

        nested: NestedSchema

    data = load_with_profiles(
        DemoSchema,
        {
            "nested": {
                "nested_profile": "test",
                "some_model": {
                    "number": 10,
                },
            },
            "source": [
                {
                    "string": "some-string",
                    "profile": "test",
                },
            ],
        },
    )

    assert data.nested.some_model.number == 10
    assert data.nested.some_model.string == "some-string"


def test_avd_profile_resolves_underlay_profile_on_list_item() -> None:
    class Device(AvdModel):
        _fields: ClassVar[dict] = {
            "name": {"type": str},
            "underlay_profile": {"type": AvdProfileRef, "catalog": "underlay_profiles", "target": "devices"},
            "underlay_routing_protocol": {"type": str},
        }

        name: str
        underlay_profile: AvdProfileRef | None
        underlay_routing_protocol: str

    class Devices(AvdIndexedList[str, Device]):
        _item_type = Device
        _primary_key = "name"

    class EosDesigns(ProfileTestRootModel):
        _fields: ClassVar[dict] = {
            "underlay_profiles": {"type": list},
            "devices": {"type": Devices},
        }

        underlay_profiles: list
        devices: Devices

    data = load_with_profiles(
        EosDesigns,
        {
            "underlay_profiles": [
                {
                    "profile": "foo",
                    "underlay_routing_protocol": "ospf",
                },
            ],
            "devices": [
                {
                    "name": "mydevice",
                    "underlay_profile": "foo",
                },
            ],
        },
    )

    device_input = data.devices["mydevice"]
    assert device_input.underlay_routing_protocol == "ospf"


def test_avd_profile_raises_when_profile_does_not_exist() -> None:
    class SomeModel(AvdModel):
        _fields: ClassVar[dict] = {
            "number": {"type": int},
            "string": {"type": str},
        }
        number: int
        string: str

    class ProfiledModel(AvdModel):
        _fields: ClassVar[dict] = {
            "example_profile": {"type": AvdProfileRef, "catalog": "source", "target": "some_model"},
            "some_model": {"type": SomeModel},
        }

        some_model: SomeModel
        example_profile: AvdProfileRef | None

    class DemoSchema(ProfileTestRootModel):
        _fields: ClassVar[dict] = {
            "profiled_model": {"type": ProfiledModel},
        }

        profiled_model: ProfiledModel

    with pytest.raises(KeyError, match="profile 'missing' is missing"):
        load_with_profiles(
            DemoSchema,
            {
                "profiled_model": {
                    "example_profile": "missing",
                    "some_model": {
                        "number": 10,
                    },
                },
                "source": [
                    {
                        "string": "some-string",
                        "profile": "test",
                    },
                ],
            },
        )


def test_avd_profile_with_deep_source_and_target_raises_when_profile_does_not_exist() -> None:
    class SomeModel(AvdModel):
        _fields: ClassVar[dict] = {
            "number": {"type": int},
            "string": {"type": str},
        }
        number: int
        string: str

    class TargetContainer(AvdModel):
        _fields: ClassVar[dict] = {
            "some_model": {"type": SomeModel},
        }
        some_model: SomeModel

    class Settings(AvdModel):
        _fields: ClassVar[dict] = {
            "target_container": {"type": TargetContainer},
        }
        target_container: TargetContainer

    class ProfiledModel(AvdModel):
        _fields: ClassVar[dict] = {
            "example_profile": {"type": AvdProfileRef, "catalog": "profile_catalog/nested/profiles", "target": "settings/target_container/some_model"},
            "settings": {"type": Settings},
        }

        settings: Settings
        example_profile: AvdProfileRef | None

    class DemoSchema(ProfileTestRootModel):
        _fields: ClassVar[dict] = {
            "profiled_model": {"type": ProfiledModel},
        }

        profiled_model: ProfiledModel

    with pytest.raises(KeyError, match="profile 'missing' is missing"):
        load_with_profiles(
            DemoSchema,
            {
                "profiled_model": {
                    "example_profile": "missing",
                    "settings": {
                        "target_container": {
                            "some_model": {
                                "number": 10,
                            },
                        },
                    },
                },
                "profile_catalog": {
                    "nested": {
                        "profiles": [
                            {
                                "profile": "test",
                                "string": "some-string",
                            },
                        ],
                    },
                },
            },
        )
