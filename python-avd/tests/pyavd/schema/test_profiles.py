# Copyright (c) 2024-2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
from __future__ import annotations

from typing import ClassVar

import pytest

from pyavd._schema.models.avd_indexed_list import AvdIndexedList
from pyavd._schema.models.avd_model import AvdModel
from pyavd._schema.models.avd_profile import AvdProfile


def test_avd_model_stuff() -> None:
    class SomeModel(AvdModel):
        _fields: ClassVar[dict] = {
            "number": {"type": int},
            "string": {"type": str},
        }
        number: int
        string: str

    class DemoSchema(AvdModel):
        _allow_other_keys = True
        _fields: ClassVar[dict] = {
            "example_profile": {"type": AvdProfile},
            "some_model": {"type": SomeModel},
        }

        some_model: SomeModel
        example_profile = AvdProfile("source", "some_model")

    data = DemoSchema._from_dict({
        "example_profile": "test",
        "some_model": {
            "number": 10,
        },
        "source": [
            {
                "string": "some-string",
                "name": "test",
            },
        ],
    })
    assert data.some_model.number == 10
    assert data.some_model.string == "some-string"


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

    class DemoSchema(AvdModel):
        _allow_other_keys = True
        _fields: ClassVar[dict] = {
            "example_profile": {"type": AvdProfile},
            "settings": {"type": Settings},
        }

        settings: Settings
        example_profile = AvdProfile("profile_catalog.nested.profiles", "settings.target_container.some_model")

    schema = DemoSchema._from_dict({
        "example_profile": "test",
        "settings": {
            "target_container": {
                "some_model": {
                    "number": 10,
                },
            },
        },
        "profile_catalog": {
            "nested": {
                "profiles": [
                    {
                        "name": "test",
                        "string": "some-string",
                    },
                ],
            },
        },
    })

    assert schema.settings.target_container.some_model.number == 10
    assert schema.settings.target_container.some_model.string == "some-string"


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
            "nested_profile": {"type": AvdProfile},
            "some_model": {"type": SomeModel},
        }

        some_model: SomeModel
        nested_profile = AvdProfile("source", "some_model")

    class DemoSchema(AvdModel):
        _allow_other_keys = True
        _fields: ClassVar[dict] = {
            "nested": {"type": NestedSchema},
        }

        nested: NestedSchema

    data = DemoSchema._from_dict({
        "nested": {
            "nested_profile": "test",
            "some_model": {
                "number": 10,
            },
        },
        "source": [
            {
                "string": "some-string",
                "name": "test",
            },
        ],
    })

    assert data.nested.some_model.number == 10
    assert data.nested.some_model.string == "some-string"


def test_avd_profile_resolves_underlay_profile_on_list_item() -> None:
    class Device(AvdModel):
        _fields: ClassVar[dict] = {
            "name": {"type": str},
            "underlay_profile": {"type": AvdProfile},
            "underlay_routing_protocol": {"type": str},
        }

        name: str
        underlay_profile = AvdProfile("underlay_profiles", ".")
        underlay_routing_protocol: str

    class Devices(AvdIndexedList[str, Device]):
        _item_type = Device
        _primary_key = "name"

    class DemoSchema(AvdModel):
        _allow_other_keys = True
        _fields: ClassVar[dict] = {
            "underlay_profiles": {"type": list},
            "devices": {"type": Devices},
        }

        underlay_profiles: list
        devices: Devices

    data = DemoSchema._from_dict({
        "underlay_profiles": [
            {
                "name": "foo",
                "underlay_routing_protocol": "ospf",
            },
        ],
        "devices": [
            {
                "name": "mydevice",
                "underlay_profile": "foo",
            },
        ],
    })

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

    class DemoSchema(AvdModel):
        _allow_other_keys = True
        _fields: ClassVar[dict] = {
            "example_profile": {"type": AvdProfile},
            "some_model": {"type": SomeModel},
        }

        some_model: SomeModel
        example_profile = AvdProfile("source", "some_model")

    with pytest.raises(KeyError, match="profile 'missing' is missing"):
        DemoSchema._from_dict({
            "example_profile": "missing",
            "some_model": {
                "number": 10,
            },
            "source": [
                {
                    "string": "some-string",
                    "name": "test",
                },
            ],
        })


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

    class DemoSchema(AvdModel):
        _allow_other_keys = True
        _fields: ClassVar[dict] = {
            "example_profile": {"type": AvdProfile},
            "settings": {"type": Settings},
        }

        settings: Settings
        example_profile = AvdProfile("profile_catalog.nested.profiles", "settings.target_container.some_model")

    with pytest.raises(KeyError, match="profile 'missing' is missing"):
        DemoSchema._from_dict({
            "example_profile": "missing",
            "settings": {
                "target_container": {
                    "some_model": {
                        "number": 10,
                    },
                },
            },
            "profile_catalog": {
                "nested": {
                    "profiles": [
                        {
                            "name": "test",
                            "string": "some-string",
                        },
                    ],
                },
            },
        })
