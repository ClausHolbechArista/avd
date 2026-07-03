# Copyright (c) 2023-2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
from __future__ import annotations

from collections.abc import Mapping, Sequence
from typing import TYPE_CHECKING, Any, TypeAlias, cast

if TYPE_CHECKING:
    from .avd_model import AvdModel

ProfileData: TypeAlias = Mapping[str, Any]


class AvdProfile:
    """
    Descriptor field used to apply reusable configuration snippets called profiles.

    A profile is loaded from a list of mappings, where each mapping must have a
    unique ``name`` key. The descriptor stores the profile name assigned during
    model loading and applies the matching profile after the full input mapping
    has been loaded.

    Example:

    .. code-block:: yaml

        interface_profiles:
          - name: uplink
            description: Uplink interface
            shutdown: false

        interface_profile: uplink
        interface:
          description: Host-facing override

    With a generated model field defined as:

    .. code-block:: python

        interface_profile = AvdProfile("interface_profiles", "interface")

    Loading the model applies the ``uplink`` profile to the ``interface`` model.
    Values already set on the instance take precedence over values from the
    profile when the profile model is combined into the instance.
    """

    def __init__(self, source: str, target: str) -> None:
        """
        Initialize an AvdProfile descriptor.

        Args:
            source: Dot-separated path in the input mapping where the profiles are defined.
            target: Dot-separated path to the model where the selected profile should be applied.
        """
        self._storage: dict[str, ProfileData] = {}
        self._instances = []
        self._source = source.split(".")
        self._target = target.split(".")

    def _resolve_profile(self, instance: AvdModel, profile_data: ProfileData, target: list[str]) -> None:
        """Apply profile data to the model found by walking the target path."""
        if not target:
            model_cls = type(instance)
            new_data = dict(profile_data)
            new_data.pop("name", None)

            profile_model = model_cls._from_dict(new_data)

            instance._combine(profile_model)
            return

        next_target = target[0]
        instance = getattr(instance, next_target)
        self._resolve_profile(instance=instance, profile_data=profile_data, target=target[1:])

    def _get_profile_data(self, instance: AvdModel, target: list[str]) -> Any:
        """Return the model or value found by walking the target path."""
        if not target:
            return instance

        next_target = target[0]
        instance = getattr(instance, next_target)
        return self._get_profile_data(instance, target=target[1:])

    def __set__(self, instance: AvdModel, profile_name: str) -> None:
        """Store a pending profile reference for the given model instance."""
        self._instances.append((instance, profile_name))

    def __get__(self, instance: AvdModel | None, owner: type[AvdModel]) -> Any:
        """Return the descriptor on the class or resolved profile target on an instance."""
        if instance is None:
            return self

        return self._get_profile_data(instance, self._target)

    def _populate_profiles(self, data: Mapping[str, Any]) -> None:
        """Load profile definitions from input data and resolve all pending references."""
        for path_part in self._source:
            data = data[path_part]
        profiles = cast("Sequence[ProfileData]", data)
        for profile in profiles:
            self._storage[cast("str", profile["name"])] = profile
        for instance, profile_name in self._instances:
            if profile_name not in self._storage:
                msg = f"profile '{profile_name}' is missing"
                raise KeyError(msg)

            self._resolve_profile(instance, self._storage[profile_name], target=self._target[:])
