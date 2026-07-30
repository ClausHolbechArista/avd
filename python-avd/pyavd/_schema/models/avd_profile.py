# Copyright (c) 2023-2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
from __future__ import annotations

from typing import TYPE_CHECKING, Any, TypedDict, cast

from .avd_indexed_list import AvdIndexedList
from .avd_list import AvdList
from .avd_model import AvdModel
from .avd_profile_ref import AvdProfileRef

if TYPE_CHECKING:
    from collections.abc import Mapping, Sequence

class ProfileData(TypedDict):
    """Profile catalog item with a required profile name."""

    profile: str


class AvdProfileResolver:
    """
    Resolve ``AvdProfileRef`` fields against reusable profile catalogs.

    Profile reference fields are represented as ``AvdProfileRef`` in generated
    ``_fields`` metadata. The same metadata also carries the profile catalog path
    and target path:

    .. code-block:: python

        _fields = {
            "interface_profile": {
                "type": AvdProfileRef,
                "catalog": "interface_profiles",
                "target": "interface",
            },
        }

    ``EosDesignsRootModel`` owns the resolver. Before normal model loading, the
    resolver walks the schema tree and loads every referenced catalog from the
    root input data. Each catalog item must be a mapping with a unique
    ``profile`` key. After normal model loading, the resolver walks the instance
    tree and combines each selected profile model into the model instance that
    owns the corresponding reference field.

    Example:

    .. code-block:: yaml

        interface_profiles:
          - profile: uplink
            description: Uplink interface
            shutdown: false

        interface_profile: uplink
        interface:
          description: Host-facing override

    With a generated model field defined as:

    .. code-block:: python

        _fields = {
            "interface_profile": {
                "type": AvdProfileRef,
                "catalog": "interface_profiles",
                "target": "interface",
            },
            "interface": {"type": Interface},
        }

    If ``interface_profile`` is set to ``uplink``, loading the root model applies
    the ``uplink`` profile to the ``interface`` model. Values already set on the
    instance take precedence over values from the profile when the profile model
    is combined into the instance.
    """

    def __init__(self) -> None:
        """Initialize an empty profile storage."""
        self._storage: dict[tuple[type[AvdModel], str], AvdModel] = {}


    def _detect_profile_references(self, cls: type[AvdModel], data: Mapping) -> None:
        """Load catalogs for all ``AvdProfileRef`` fields declared below ``cls``."""
        for field_spec in cls._fields.values():
            t = field_spec["type"]

            if t is AvdProfileRef:
                self._populate_profiles(cls, data, field_spec["catalog"], field_spec.get("target", "."))
            elif isinstance(t, type) and issubclass(t, AvdModel):
                self._detect_profile_references(t, data)
            elif isinstance(t, type) and issubclass(t, (AvdList, AvdIndexedList)) and issubclass(t._item_type, AvdModel):
                self._detect_profile_references(t._item_type, data)

    def _resolve_profiles(self, instance: AvdModel) -> None:
        """Apply selected profile models for all ``AvdProfileRef`` values below ``instance``."""
        for field_name, field_spec in instance._fields.items():
            t = field_spec.get("type")
            if t is AvdProfileRef:
                ref = getattr(instance, field_name)
                if ref is None:
                    continue
                try:
                    profile_model = self._storage[(type(instance), ref)]
                except KeyError as error:
                    msg = f"profile '{ref}' is missing"
                    raise KeyError(msg) from error
                instance._combine(profile_model)
            elif isinstance(t, type) and issubclass(t, AvdModel):
                self._resolve_profiles(getattr(instance, field_name))
            elif isinstance(t, type) and issubclass(t, (AvdList, AvdIndexedList)) and issubclass(t._item_type, AvdModel):
                for next_instance in getattr(instance, field_name):
                    self._resolve_profiles(next_instance)

    def _populate_profiles(self, model: type[AvdModel], data: Mapping[str, Any], catalog: str, target: str) -> None:
        """Load one profile catalog and store partial profile models for ``model``."""
        target_path = [] if target == "." else target.split(".")

        def _to_partial_model(profile_data: ProfileData) -> AvdModel:
            d = dict(profile_data)
            d.pop("profile", None)
            for p in reversed(target_path):
                d = {p: d}
            return model._from_dict(d)

        profile_data = data
        prefix = []

        # Traverse the input data to fetch the list of profiles
        for p in catalog.split("."):
            prefix.append(p)
            if p not in profile_data:
                msg = f"missing key in input data: {'.'.join(prefix)}"
                raise KeyError(msg)

            profile_data = profile_data[p]

        # Read the list of profiles and cast them to the model's instances
        profiles = cast("Sequence[ProfileData]", profile_data)
        for profile in profiles:
            profile_key = profile.get("profile")
            if profile_key is None:
                msg = "profile is missing 'profile' key"
                raise KeyError(msg)
            self._storage[(model, profile_key)] = _to_partial_model(profile)
