# Copyright (c) 2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.

"""Typing-contract and behavioral checks for native validated-data models."""

import ast
import json
import subprocess
import sys
from pathlib import Path

import pytest

from pyavd import _rust, _validated_data
from pyavd._utils.undefined import Undefined


def test_private_native_module_is_lazy_and_has_no_python_wrapper() -> None:
    """Keep the extension lazy while exposing the native module through the package API."""
    program = """
import sys
import pyavd
assert 'pyavd._rust' not in sys.modules
assert '_validated_data' not in pyavd.__all__
from pyavd import _validated_data
from pyavd import _rust
assert _validated_data is _rust._validated_data
assert _validated_data.__name__ == 'pyavd._validated_data'
assert not getattr(_validated_data, '__file__', None)
assert sys.modules['pyavd._validated_data'] is _validated_data
assert not hasattr(_rust, '_DictView')
assert not hasattr(_rust, '_ListView')
assert not hasattr(_rust, '_ValueHandle')
assert not hasattr(_rust, '_open_avd_design_handle')
"""
    subprocess.run([sys.executable, "-c", program], check=True)  # noqa: S603 - runs a fixed inline import contract check.


def test_native_module_reexport_is_cached(monkeypatch: pytest.MonkeyPatch) -> None:
    """A cached native module must not be mistaken for a shadowing implementation module."""
    import pyavd
    from pyavd import _lazy_import

    def unexpected_import(_name: str) -> None:
        pytest.fail("cached native module export was resolved again")

    monkeypatch.setattr(_lazy_import, "import_module", unexpected_import)
    assert pyavd._validated_data is _validated_data


def _public_properties(model: type) -> set[str]:
    """Include inherited contextual key properties, ignoring generic backing implementation."""
    return {name for base in model.__mro__ for name, value in vars(base).items() if not name.startswith("_") and hasattr(value, "__get__")}


def test_native_catalog_matches_every_generated_stub_model() -> None:
    """Compare all public types and fields against the typing contract, including inherited keys."""
    native = _validated_data
    assert native is _rust._validated_data
    assert native.__name__ == "pyavd._validated_data"
    stub_path = Path(__file__).parents[2] / "pyavd/_validated_data.pyi"
    declarations = {node.name: node for node in ast.parse(stub_path.read_text()).body if isinstance(node, ast.ClassDef)}
    assert not stub_path.with_suffix(".py").exists()
    native_models = {name: value for name, value in vars(native).items() if isinstance(value, type) and value.__module__ == native.__name__}
    assert native_models.keys() == declarations.keys()
    assert len(native_models) > 1000

    def declared_properties(name: str) -> set[str]:
        declaration = declarations[name]
        properties = {node.name for node in declaration.body if isinstance(node, ast.FunctionDef) and not node.name.startswith("_")}
        for base in declaration.bases:
            if isinstance(base, ast.Name) and base.id in declarations:
                properties.update(declared_properties(base.id))
        return properties

    for name in declarations:
        assert _public_properties(native_models[name]) == declared_properties(name), name


def test_binding_collection_edge_cases_and_ownership(tmp_path: Path) -> None:
    """Preserve defaults, negative indexing/slices, hybrid duplicates, errors, and retained iterators."""
    schema = Path(__file__).parents[2] / "pyavd/_schema/schemas.rkyv"
    archive = tmp_path / "host.rkyv"
    result = _rust.archive_avd_design(
        json.dumps({"fabric_name": "TEST", "devices": [{"name": "leaf1", "id": 1, "uplink_interfaces": ["Ethernet1", "Ethernet2", "Ethernet3"]}]}),
        archive,
        schema,
    )
    assert result.destination == archive
    models = _validated_data
    root = models.open_avd_design(archive, schema)
    devices = root.devices
    item = devices["leaf1"]
    assert isinstance(item, models.AVDDesignDevicesItems)
    assert devices.get("absent") is Undefined
    assert devices.get("absent", None) is None
    assert devices.get("absent", default=None) is None
    marker = object()
    assert devices.get("absent", marker) is marker
    assert devices.get("absent", default=marker) is marker
    assert devices.get(key="leaf1").name == "leaf1"
    with pytest.raises(KeyError):
        devices["absent"]
    with pytest.raises(TypeError):
        devices.get("leaf1", marker, default=marker)
    interfaces = item.uplink_interfaces
    assert interfaces[::-1] == ["Ethernet3", "Ethernet2", "Ethernet1"]
    assert interfaces[::2] == ["Ethernet1", "Ethernet3"]
    assert interfaces[-1] == "Ethernet3"
    with pytest.raises(IndexError):
        interfaces[3]
    with pytest.raises(IndexError):
        interfaces[-4]
    with pytest.raises(ValueError):  # noqa: PT011 - CPython raises this for a zero slice step
        interfaces[::0]
    with pytest.raises(AttributeError):
        item.id = 2
    iterator = iter(interfaces)
    items = devices.items()
    keys = devices.keys()
    del root, devices, item, interfaces
    assert list(iterator) == ["Ethernet1", "Ethernet2", "Ethernet3"]
    assert [(key, value.name) for key, value in items] == [("leaf1", "leaf1")]
    assert list(keys) == ["leaf1"]
