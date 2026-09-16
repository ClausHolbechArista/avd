# Copyright (c) 2024-2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
from __future__ import annotations

from typing import ClassVar

import pytest

from pyavd._errors import AristaAvdInvalidInputsError, AristaAvdMissingVariableError, AvdSchemaError
from pyavd._schema.models.avd_model import AvdModel
from pyavd._schema.models.avd_profile import AvdProfileResolver
from pyavd._schema.models.avd_profile_ref import AvdProfileRef


class ProfileTestRootModel(AvdModel):
    """Minimal EosDesignsRootModel for profile resolver tests."""

    _allow_other_keys = True


class SomeModel(AvdModel):
    _fields: ClassVar[dict] = {
        "number": {"type": int},
        "string": {"type": str},
        "child_only": {"type": str},
        "parent_only": {"type": str},
        "grandparent_only": {"type": str},
    }
    number: int
    string: str
    child_only: str
    parent_only: str
    grandparent_only: str


class ProfiledModel(AvdModel):
    _fields: ClassVar[dict] = {
        "example_profile": {"type": AvdProfileRef, "catalog": "source", "target": "profiled_model.some_model"},
        "some_model": {"type": SomeModel},
    }

    some_model: SomeModel
    example_profile: AvdProfileRef | None


class DemoSchema(ProfileTestRootModel):
    _fields: ClassVar[dict] = {
        "profiled_model": {"type": ProfiledModel},
    }

    profiled_model: ProfiledModel


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


class DeepProfiledModel(AvdModel):
    _fields: ClassVar[dict] = {
        "example_profile": {
            "type": AvdProfileRef,
            "catalog": "profile_catalog.nested.profiles",
            "target": "profiled_model.settings.target_container.some_model",
        },
        "settings": {"type": Settings},
    }

    settings: Settings
    example_profile: AvdProfileRef | None


class DeepDemoSchema(ProfileTestRootModel):
    _fields: ClassVar[dict] = {
        "profiled_model": {"type": DeepProfiledModel},
    }

    profiled_model: DeepProfiledModel


def load_with_profiles(schema_cls: type[AvdModel], data: dict) -> AvdModel:
    """Load test schema data and apply profiles with an explicit resolver."""
    profile_resolver = AvdProfileResolver(data, schema_cls)
    result = schema_cls._from_dict(data)
    return profile_resolver._apply_profiles(result)


def test_avd_model_stuff() -> None:
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
    """Use production schemas to verify profile resolution against consolidated eos_designs inputs."""
    from pyavd.api.schemas import AVDDesign, ConsolidatedAVDDesign

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

    inputs = {
        "fabric_name": "FABRIC",
        "devices": [
            {
                "name": "leaf1",
                "type": "l2leaf",
                "dns_settings_profile": "leaf1_dns_profile",
            },
            {
                "name": "leaf2",
                "type": "l2leaf",
                "dns_settings_profile": "leaf2_dns_profile",
            },
        ],
        "dns_settings_profiles": dns_settings_profile_catalog,

    }

    results = {}
    eos_design = AVDDesign._from_dict(inputs)
    profile_resolver = AvdProfileResolver(inputs, ConsolidatedAVDDesign)
    for device_name in ["leaf1", "leaf2"]:
        consolidated_avd_design = ConsolidatedAVDDesign._from_avd_design(device_name, eos_design)
        results[device_name] = profile_resolver._apply_profiles(consolidated_avd_design)

    assert results["leaf1"].inputs.dns_settings.domain == "leaf1.example.com"
    assert list(results["leaf1"].inputs.dns_settings.domain_list) == ["leaf1.example.com"]
    assert results["leaf2"].inputs.dns_settings.domain == "leaf2.example.com"
    assert list(results["leaf2"].inputs.dns_settings.domain_list) == ["leaf2.example.com"]


def test_avd_profile_with_deep_source_and_target() -> None:
    data = load_with_profiles(
        DeepDemoSchema,
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
    class NestedProfiledModel(AvdModel):
        _fields: ClassVar[dict] = {
            "nested_profile": {"type": AvdProfileRef, "catalog": "source", "target": "nested.some_model"},
            "some_model": {"type": SomeModel},
        }

        some_model: SomeModel
        nested_profile: AvdProfileRef | None

    class NestedDemoSchema(ProfileTestRootModel):
        _fields: ClassVar[dict] = {
            "nested": {"type": NestedProfiledModel},
        }

        nested: NestedProfiledModel

    data = load_with_profiles(
        NestedDemoSchema,
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



def test_avd_profile_raises_when_profile_does_not_exist() -> None:
    with pytest.raises(AristaAvdInvalidInputsError, match="Profile 'missing' is missing"):
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


def test_avd_profile_raises_when_profile_catalog_does_not_exist() -> None:
    with pytest.raises(AristaAvdMissingVariableError, match=r"'source' is required but was not found\."):
        load_with_profiles(
            DemoSchema,
            {
                "profiled_model": {
                    "example_profile": "test",
                },
            },
        )


def test_avd_profile_raises_when_profile_target_field_does_not_exist() -> None:
    class ProfiledModelWithMissingTarget(AvdModel):
        _fields: ClassVar[dict] = {
            "example_profile": {"type": AvdProfileRef, "catalog": "source", "target": "profiled_model.missing_model"},
            "some_model": {"type": SomeModel},
        }

        some_model: SomeModel
        example_profile: AvdProfileRef | None

    class MissingTargetDemoSchema(ProfileTestRootModel):
        _fields: ClassVar[dict] = {
            "profiled_model": {"type": ProfiledModelWithMissingTarget},
        }

        profiled_model: ProfiledModelWithMissingTarget

    with pytest.raises(AvdSchemaError, match=r"`profiled_model\.missing_model` is not a valid profile target"):
        load_with_profiles(
            MissingTargetDemoSchema,
            {
                "profiled_model": {
                    "example_profile": "test",
                },
                "source": [
                    {
                        "profile": "test",
                        "string": "some-string",
                    },
                ],
            },
        )


def test_avd_profile_raises_when_profile_target_is_not_model() -> None:
    class ProfiledModelWithScalarTarget(AvdModel):
        _fields: ClassVar[dict] = {
            "example_profile": {"type": AvdProfileRef, "catalog": "source", "target": "profiled_model.some_model.string"},
            "some_model": {"type": SomeModel},
        }

        some_model: SomeModel
        example_profile: AvdProfileRef | None

    class ScalarTargetDemoSchema(ProfileTestRootModel):
        _fields: ClassVar[dict] = {
            "profiled_model": {"type": ProfiledModelWithScalarTarget},
        }

        profiled_model: ProfiledModelWithScalarTarget

    with pytest.raises(AvdSchemaError, match=r"`profiled_model\.some_model\.string` is not a valid profile target"):
        load_with_profiles(
            ScalarTargetDemoSchema,
            {
                "profiled_model": {
                    "example_profile": "test",
                },
                "source": [
                    {
                        "profile": "test",
                        "string": "some-string",
                    },
                ],
            },
        )


def test_avd_profile_raises_when_parent_profile_does_not_exist() -> None:
    with pytest.raises(AristaAvdInvalidInputsError, match=r"Unresolved `parent_profile` references: .*missing_parent"):
        load_with_profiles(
            DemoSchema,
            {
                "profiled_model": {
                    "example_profile": "child",
                },
                "source": [
                    {
                        "profile": "child",
                        "parent_profile": "missing_parent",
                        "string": "child-string",
                    },
                ],
            },
        )


def test_avd_profile_raises_on_indirect_cyclic_parent_profiles() -> None:
    with pytest.raises(AristaAvdInvalidInputsError, match=r"Cycle detected: .*n2.*n4.*n3.*n2") as exc_info:
        load_with_profiles(
            DemoSchema,
            {
                "profiled_model": {
                    "example_profile": "n1",
                },
                "source": [
                    {
                        "profile": "n1",
                        "parent_profile": "n2",
                        "string": "n1-string",
                    },
                    {
                        "profile": "n2",
                        "parent_profile": "n3",
                        "string": "n2-string",
                    },
                    {
                        "profile": "n3",
                        "parent_profile": "n4",
                        "string": "n3-string",
                    },
                    {
                        "profile": "n4",
                        "parent_profile": "n2",
                        "string": "n4-string",
                    },
                ],
            },
        )

    assert "n1" not in str(exc_info.value)


def test_avd_profile_resolver_parent_chain_precedence() -> None:
    data = load_with_profiles(
        DemoSchema,
        {
            "profiled_model": {
                "example_profile": "child",
                "some_model": {
                    "number": 10,
                },
            },
            "source": [
                {
                    "profile": "grandparent",
                    "number": 300,
                    "string": "grandparent-string",
                    "parent_only": "grandparent-parent-only-value",
                    "grandparent_only": "grandparent-only-value",
                },
                {
                    "profile": "parent",
                    "parent_profile": "grandparent",
                    "number": 200,
                    "string": "parent-string",
                    "parent_only": "parent-only-value",
                },
                {
                    "profile": "child",
                    "parent_profile": "parent",
                    "number": 20,
                    "string": "child-string",
                    "child_only": "child-only-value",
                },
            ],
        },
    )

    assert data.profiled_model.some_model.number == 10
    assert data.profiled_model.some_model.string == "child-string"
    assert data.profiled_model.some_model.child_only == "child-only-value"
    assert data.profiled_model.some_model.parent_only == "parent-only-value"
    assert data.profiled_model.some_model.grandparent_only == "grandparent-only-value"


def test_avd_profile_with_deep_source_and_target_raises_when_profile_does_not_exist() -> None:
    with pytest.raises(AristaAvdInvalidInputsError, match="Profile 'missing' is missing"):
        load_with_profiles(
            DeepDemoSchema,
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
