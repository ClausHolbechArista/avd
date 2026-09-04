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
    from collections.abc import Generator, Mapping, Sequence


class ProfileData(TypedDict):
    """Profile catalog item with a required profile name."""

    profile: str


class ProfileSelector(TypedDict):
    """Profile selector metadata from an ``AvdProfileRef`` field."""

    catalog: str
    target: str


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

    def _detect_profile_refs(self, cls: type[AvdModel], data: Mapping, load_custom_structured_config: bool = True) -> dict[tuple[str, str, str, str], AvdModel]:
        """Load and resolve profile catalogs for all ``AvdProfileRef`` fields declared below ``cls``."""
        profiles: dict[tuple[str, str, str, str], AvdModel] = {}

        def _to_model(profile: dict, target: list[str], owner_model: type[AvdModel]) -> AvdModel:
            # TODO: do we even need to support list targets?
            if _target_is_list_of_model(cls, target, owner_model):
                return owner_model._from_dict(profile)
            if _target_is_path_on_model(owner_model, target):
                return owner_model._from_dict(_dict_from_path(profile, target))

            root_dict = _dict_from_path(profile, target)
            if hasattr(cls, "_from_dict_internal"):
                return cast("Any", cls)._from_dict_internal(root_dict, load_custom_structured_config)
            return cls._from_dict(root_dict)

        def _resolve_profile_tree(graph: dict[str | None, list[str]], parents: dict[str, str], raw_profiles: dict[str, AvdModel]) -> None:
            visited = set()
            q = graph.pop(None)
            while q:
                current = q.pop()
                if current in visited:
                    # This is a tree-like structure where each node can only have one parent, so visiting a node
                    # twice means there is a cycle.
                    cycle_path: list[str] = [current]
                    parent = current
                    while (parent := parents[parent]) != current:
                        cycle_path.append(parent)
                    cycle_path.append(current)
                    msg = "Cycle detected: " + " -> ".join(cycle_path)
                    raise ValueError(msg)
                if current in parents:
                    raw_profiles[current]._deepmerge(raw_profiles[parents[current]])
                q.extend(graph.get(current, []))

        def _resolve_profiles(selector: ProfileSelector, owner_model: type[AvdModel], data: Mapping[str, Any]) -> dict[str, AvdModel]:
            target = selector["target"].split("/")
            catalog = selector["catalog"].split("/")
            profile_mapping = {}

            catalog_list = data
            for p in catalog:
                if (catalog_list := catalog_list.get(p)) is None:
                    return profile_mapping

            profile_graph = {}
            profile_graph_parents = {}
            for profile_spec_data in catalog_list:
                profile_spec = dict(profile_spec_data)
                profile_id = profile_spec.pop("profile")
                parent_profile = profile_spec.pop("parent_profile", None)
                profile_graph.setdefault(parent_profile, []).append(profile_id)

                profile_mapping[profile_id] = _to_model(profile_spec, target, owner_model)
                if parent_profile:
                    profile_graph_parents[profile_id] = parent_profile

            _resolve_profile_tree(profile_graph, profile_graph_parents, profile_mapping)

            return profile_mapping

        for field_name, profile_selector, owner_model in _internal_recursion(cls):
            for name, profile in _resolve_profiles(profile_selector, owner_model, data).items():
                profiles[(profile_selector["catalog"], profile_selector["target"], field_name, name)] = profile

        return profiles

    def _apply_profiles(self, instance: AvdModel, profiles: dict[tuple[str, str, str, str], AvdModel]) -> AvdModel:
        """Apply selected profile models for all ``AvdProfileRef`` values below ``instance``."""
        root_instance = instance

        def _apply_matching_profiles(instance: AvdModel) -> None:
            for field_name, field_spec in instance._fields.items():
                t = field_spec["type"]

                field_value = instance._get(field_name)
                if t is AvdProfileRef:
                    if field_value is None:
                        continue
                    k = (field_spec["catalog"], field_spec["target"], field_name, field_value)
                    if k not in profiles:
                        msg = f"profile '{field_value}' is missing"
                        raise KeyError(msg)
                    profile = profiles[k]
                    if isinstance(profile, type(instance)):
                        instance._deepmerge(profile)
                    else:
                        root_instance._deepmerge(profile)
                elif isinstance(field_value, (AvdList, AvdIndexedList)):
                    for next_instance in field_value:
                        if isinstance(next_instance, AvdModel):
                            _apply_matching_profiles(next_instance)
                elif isinstance(field_value, AvdModel):
                    _apply_matching_profiles(field_value)

        _apply_matching_profiles(instance)
        return instance

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


def _internal_recursion(model: type[AvdModel]) -> Generator[tuple[str, ProfileSelector, type[AvdModel]]]:
    for field_name, field_spec in model._fields.items():
        t = field_spec.get("type")
        if t is AvdProfileRef:
            yield field_name, cast("ProfileSelector", field_spec), model
        elif isinstance(t, type) and issubclass(t, AvdModel):
            yield from _internal_recursion(t)
        elif isinstance(t, type) and issubclass(t, (AvdList, AvdIndexedList)) and issubclass(t._item_type, AvdModel):
            yield from _internal_recursion(t._item_type)


def _target_is_list_of_model(root_model: type[AvdModel], target: list[str], model: type[AvdModel]) -> bool:
    """Return True when the root-relative profile target points at a list containing ``model`` items."""
    target_type: type | None = root_model
    for field_name in target:
        if not isinstance(target_type, type) or not issubclass(target_type, AvdModel):
            return False
        field_spec = target_type._fields.get(field_name)
        if field_spec is None:
            return False
        target_type = field_spec["type"]

    return isinstance(target_type, type) and issubclass(target_type, (AvdList, AvdIndexedList)) and target_type._item_type is model


def _target_is_path_on_model(model: type[AvdModel], target: list[str]) -> bool:
    """Return True when the profile target is a path below ``model``."""
    target_type: type | None = model
    for field_name in target:
        if not isinstance(target_type, type) or not issubclass(target_type, AvdModel):
            return False
        field_spec = target_type._fields.get(field_name)
        if field_spec is None:
            return False
        target_type = field_spec["type"]

    return True


def _dict_from_path(data: dict, path: list[str]) -> dict:
    """Return ``data`` nested below ``path``."""
    root_dict = target_dict = {}
    for p in path:
        target_dict = target_dict.setdefault(p, {})
    target_dict.update(data)
    return root_dict
