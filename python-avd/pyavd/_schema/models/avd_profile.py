# Copyright (c) 2023-2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
from __future__ import annotations

from collections import namedtuple
from typing import TYPE_CHECKING, Any, Callable, ClassVar, TypedDict, cast
import dataclasses
import functools

from pyavd._errors import AristaAvdError, AristaAvdInvalidInputsError, AristaAvdMissingVariableError, AvdSchemaError
from pyavd._utils.get import get_v2

from .avd_indexed_list import AvdIndexedList
from .avd_list import AvdList
from .avd_model import AvdModel
from .avd_profile_ref import AvdProfileRef
from .type_vars import T_AvdModel

if TYPE_CHECKING:
    from collections.abc import Mapping


ProfileSpec = namedtuple("ProfileSpec", ("catalog", "target", "target_model", "field_path"))

class ProfileSelector(TypedDict):
    """Profile selector metadata from an ``AvdProfileRef`` field."""

    catalog: str
    target: str


class ProfileData(AvdModel):
    """Profile catalog item with selector keys separated from data applied to the target model."""
    _fields = {
        "profile": {"type": str},
        "parent_profile": {"type": str},
    }
    profile: str
    parent_profile: str | None = None
    raw_data: dict
    _allow_other_keys = True

    @classmethod
    def _from_dict(cls: type[T_AvdModel], data: Mapping) -> AvdModel:
        raw_data = dict(data) # shallow copy
        raw_data.pop("profile", None)
        raw_data.pop("parent_profile", None)

        model = super()._from_dict(data)
        model.raw_data = raw_data
        return model


class ProfileList(AvdIndexedList[str, ProfileData]):
    _item_type: ClassVar[type[AvdModel]] = ProfileData
    _primary_key: ClassVar[str] = "profile"



@dataclasses.dataclass
class ProfileGraphNode:
    profile_spec: ProfileSpec
    profile: ProfileData | None = None
    parent: ProfileGraphNode | None = None
    children: list[ProfileGraphNode] = dataclasses.field(default_factory=list)

    @property
    def id(self) -> str | None:
        return None if not self.profile else self.profile.profile 

    @functools.cached_property
    def data(self) -> AvdModel:
        # from_dict is quite slow so we only cast it to model when the profile is actually referenced
        if self.profile is None:
            # this should not happen as this would be caught by ProfileGraph._check_all_profiles_resolved
            msg = "Referencing uninitialized profile"
            raise AristaAvdError(msg)
        partial_model = _dict_from_path(self.profile.raw_data, self.profile_spec.target)
        return self.profile_spec.target_model._from_dict(partial_model)


class ProfileGraph:
    """Catalog graph used to resolve selected profiles and their parent profiles once per selector."""

    def __init__(self) -> None:
        self.nodes: dict[str | None, ProfileGraphNode] = {}
        # Cache ensures that the profile is loaded only when it's needed, and it's resolved exactly once
        self._lazy_load_profile: Callable[[str], AvdModel] = \
            functools.cache(lambda profile_id: self._get_profile(profile_id))

    @classmethod
    def _from_profile_list(cls, catalog_list: ProfileList, spec: ProfileSpec):
        graph = ProfileGraph()

        # Initiate the root profile. All profiles that does not have parent_profile specified would
        # inherit this profile
        graph.nodes[None] = ProfileGraphNode(spec)
        for profile_id, profile_data in catalog_list.items():
            # setdefault allows us ensure that the profile is only initialized once
            node = graph.nodes.setdefault(profile_id, ProfileGraphNode(spec))
            parent_node = graph.nodes.setdefault(
                profile_data.parent_profile, ProfileGraphNode(spec)
            )

            node.profile = profile_data
            parent_node.children.append(node)
            node.parent = parent_node

        graph._check_all_profiles_resolved()
        graph._check_cycles()
        return graph

    def _check_all_profiles_resolved(self) -> None:
        # catch the cases when profile is referenced in parent_profile, but not defined in
        # catalog_list
        uninitialized = set()
        for profile_id, profile_node in self.nodes.items():
            if not profile_id:
                # skip root node as this is a syntetic node
                continue
            if not profile_node.profile:
                uninitialized.add(profile_id)
        if uninitialized:
            msg = f"Unresolved `parent_profile` references: {uninitialized}"
            raise AristaAvdInvalidInputsError(msg)

    def _check_cycles(self) -> None:
        """
        Helper function that helps to determine if the parent_profile references does not incur
        a cyclic profile resolution.
        """
        def _check_node(node: ProfileGraphNode, path: list[str]) -> None:
            if node.id in path:
                cycle_path = path[path.index(node.id) :] + [cast(str, node.id)]
                msg = "Cycle detected: " + " -> ".join(cycle_path)
                raise AristaAvdInvalidInputsError(msg)

            if node.id is not None:
                path = [*path, node.id]

            for child in node.children:
                _check_node(child, path)

        for node in self.nodes.values():
            _check_node(node, [])

    def _get_profile(self, profile_id: str) -> AvdModel:
        node = self.nodes.get(profile_id)
        if node is None:
            msg = f"Profile '{profile_id}' is missing"
            raise AristaAvdInvalidInputsError(msg)
        if node.parent and node.parent.id:
            sub_model = self.get_profile(node.parent.id)
            node.data._deepinherit(sub_model)
        return node.data

    def get_profile(self, profile_id) -> AvdModel:
        return self._lazy_load_profile(profile_id)


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

    The resolver is instantiated explicitly by callers that already have a
    loaded target model. For ``eos_designs`` this happens after host-specific
    inputs have been normalized to ``ConsolidatedAVDDesign`` in both pyavd and
    the Ansible action plugin. The ``raw_data`` passed to the resolver is still
    used as the source of profile catalogs, since catalogs can live outside the
    loaded per-device model.

    .. code-block:: python

        consolidated_inputs = ConsolidatedAVDDesign._from_avd_design(hostname, inputs)
        profile_resolver = AvdProfileResolver(inputs, ConsolidatedAVDDesign)
        consolidated_inputs = profile_resolver._apply_profiles(consolidated_inputs)

    ``_apply_profiles`` walks the loaded instance tree. When it finds a set
    ``AvdProfileRef``, it lazily loads the referenced catalog, builds or reuses
    a cached profile graph for that field's selector metadata, resolves any
    ``parent_profile`` chain, and inherits missing fields from the resulting
    partial root model into the root instance. The generated ``target`` path is
    therefore resolved from the root model, not relative to the model containing
    the profile reference. For conflicting values, the loaded instance wins over
    the selected profile, and the selected profile wins over its parent profiles.

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

    If ``interface_profile`` is set to ``uplink``, calling ``_apply_profiles``
    on the loaded root model applies the ``uplink`` profile to the ``interface``
    model.
    """
    def __init__(self, raw_data: Mapping, target_model: type[AvdModel]) -> None:
        self.raw_data = raw_data
        self.target_model = target_model

        self._lazy_load_profile_graph: Callable[[ProfileSpec], ProfileGraph] = \
            functools.cache(lambda profile_spec: self._resolve_profiles(profile_spec))

    def _resolve_profiles(self, profile_spec: ProfileSpec) -> ProfileGraph:
        catalog_list = get_v2(self.raw_data, profile_spec.catalog)
        if catalog_list is None:
            raise AristaAvdMissingVariableError(profile_spec.catalog)
        catalog_list = ProfileList._from_list(catalog_list)
        profile_graph = ProfileGraph._from_profile_list(catalog_list, profile_spec)
        return profile_graph

    def _get_profile(self, profile_spec: ProfileSpec, profile_name: AvdProfileRef) -> AvdModel:

        profiles = self._lazy_load_profile_graph(profile_spec)
        return profiles.get_profile(profile_name)

    def _apply_profiles(self, instance: AvdModel) -> AvdModel:
        """Apply selected profile models for all ``AvdProfileRef`` values below ``instance``."""
        root_instance = instance

        def _apply_matching_profiles(instance: AvdModel | None | Any, prefix: str = "") -> None:
            if isinstance(instance, (AvdList, AvdIndexedList)):
                for next_instance in instance:
                    _apply_matching_profiles(next_instance, prefix)
            elif isinstance(instance, AvdModel):
                for field_name, field_spec in instance._fields.items():
                    new_prefix = prefix + "." + field_name
                    field_type = field_spec["type"]
                    field_value = instance._get(field_name)
                    if field_type is AvdProfileRef and field_value is not None:
                        profile_selector = cast("ProfileSelector", field_spec)
                        self._check_target_is_valid(self.target_model, profile_selector["target"])

                        field_spec = ProfileSpec(
                            profile_selector["catalog"],
                            profile_selector["target"],
                            self.target_model,
                            new_prefix,
                        )
                        profile = self._get_profile(field_spec, field_value)
                        root_instance._deepinherit(profile)
                    else:
                        _apply_matching_profiles(field_value, new_prefix)

        _apply_matching_profiles(instance)
        return instance

    def _check_target_is_valid(self, target_cls: type[AvdModel], target: str, original_target: str | None = None) -> None:
        original_target = original_target or target
        target_path = target.split(".")
        if not target:
            return
        if target_path[0] in target_cls._fields:
            field_type = target_cls._fields[target_path[0]]["type"]
            if issubclass(field_type, AvdModel) and not issubclass(field_type, (AvdIndexedList, AvdList)):
                self._check_target_is_valid(field_type, ".".join(target_path[1:]), original_target)
            else:
                msg = f"`{original_target}` is not a valid profile target: `{field_type}` is not a supported type."
                raise AvdSchemaError(msg)
        else:
            msg = f"`{original_target}` is not a valid profile target: field `{target_path[0]}` is not defined."
            raise AvdSchemaError(msg)

def _dict_from_path(data: dict, path: str) -> dict:
    """Return ``data`` nested below ``path``."""
    if path == ".":
        return data

    root_dict = target_dict = {}
    path_ls = path.split(".")
    for p in path_ls:
        target_dict = target_dict.setdefault(p, {})
    target_dict.update(data)
    return root_dict
