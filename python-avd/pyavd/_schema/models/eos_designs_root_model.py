# Copyright (c) 2023-2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
from __future__ import annotations

from collections import ChainMap
from collections.abc import Iterator, Mapping
from typing import TYPE_CHECKING, TypeVar, TypedDict, Generator, cast

from pyavd._eos_cli_config_gen.schema import EosCliConfigGen
from pyavd._schema.store import create_store
from pyavd._schema.utils import get_instance_with_defaults
from pyavd._utils import get_all

from .avd_list import AvdList
from .avd_base import AvdBase

from .avd_indexed_list import AvdIndexedList
from .avd_model import AvdModel
from .avd_profile import AvdProfileResolver
from .avd_profile_ref import AvdProfileRef


if TYPE_CHECKING:
    from typing_extensions import Self

    from pyavd._eos_designs.schema import EosDesigns

    T = TypeVar("T", bound="EosDesignsRootModel")


SKIP_KEYS = ["custom_structured_configuration_list_merge", "custom_structured_configuration_prefix"]


class EosDesignsRootModel(AvdModel):
    """Root model responsible for loading dynamic keys, custom structured config, and profile references."""

    profile_resolver = AvdProfileResolver()

    @classmethod
    def _from_dict(cls, data: Mapping, load_custom_structured_config: bool = True) -> Self:
        """
        Returns a new instance loaded with the data from the given dict.

        The EosDesignsRootModel is special because it will also load "dynamic keys" like `node_type_keys` and `network_services_keys` and
        `connected_endpoints_keys`. Those models will be parsed and all mentioned keys will be searched for in the input and loaded into the
        corresponding model under `_dynamic_keys`.

        Furthermore the EosDesignsRootModel will also load `custom_structured_configuration_prefix` and search for any keys prefixed with those. Found keys
        will be loaded into the `_custom_structured_configurations` model.

        Finally, the EosDesignsRootModel resolves reusable profile references declared on child models before returning the loaded model.

        Args:
            data: A mapping containing the EosDesigns input data to be loaded.
            load_custom_structured_config: Some custom structured config contains inline Jinja templates relying on variables produced by EosDesignsFacts.
                To avoid such templates breaking the type checks, we can skip loading custom_structured_configuration during the facts phase by setting this
                to False.

        """
        if not isinstance(data, Mapping):
            msg = f"Expecting 'data' as a 'Mapping' when loading data into '{cls.__name__}'. Got '{type(data)}"
            raise TypeError(msg)

        #cls.profile_resolver._storage.clear()
        #cls.profile_resolver._detect_profile_references(cls, data)
        profiles = cls._detect_profile_refs(data, load_custom_structured_config)
        result = cls._from_dict_internal(data, load_custom_structured_config=load_custom_structured_config)
        #cls.profile_resolver._resolve_profiles(result)
        result._apply_profiles(profiles)
        return result

    @classmethod
    def _from_dict_internal(cls, data: Mapping, load_custom_structured_config: bool = True):
        root_data = {"_dynamic_keys": cls._get_dynamic_keys(data)}
        if load_custom_structured_config:
            root_data["_custom_structured_configurations"] = cls._CustomStructuredConfigurations(cls._get_csc_items(data))
        return  super()._from_dict(ChainMap(root_data, data))

    @classmethod
    def _detect_profile_refs(cls, data: Mapping, load_custom_structured_config: bool = True) -> dict[tuple[str, str, str, str], EosDesignsRootModel]:
        profiles = {}

        def _to_model(profile: dict, target: list[str], model: type[AvdModel]):
            root_dict = target_dict = {}
            for p in target:
                target_dict = target_dict.setdefault(p, {})
            target_dict.update(profile)
            return cls._from_dict_internal(root_dict, load_custom_structured_config)

        def _resolve_profile_tree(
                graph: dict[str|None,list[str]], 
                parents: dict[str, str],
                raw_profiles: dict[str, EosDesignsRootModel]):
            visited = set()
            q = graph.pop(None)
            while q:
                current = q.pop()
                if current in visited:
                    # as this is the tree-like structure where each node can only have 1 parent, 
                    # we can assume that it is not possible to visit single node twice unless we have
                    # a cycle
                    cycle_path: list[str] = [current]
                    parent = current
                    while (parent := parents[parent]) != current:
                        cycle_path.append(parent) # this never going to be none
                    cycle_path.append(current)
                    raise Exception("Cycle detected: " + " -> ".join(cycle_path))
                if current in parents:
                    raw_profiles[current]._deepmerge(raw_profiles[parents[current]])
                q.extend(graph.get(current, []))

        def _resolve_profiles(selector: ProfileSelector, data) -> dict[str, EosDesignsRootModel]:
            target = selector["target"].split("/")
            catalog = selector["catalog"].split("/")
            profile_mapping = {} 

            catalog_list = data
            for p in catalog:
                if (catalog_list := catalog_list.get(p)) is None:
                    raise KeyError(f"catalog not found under the path `{selector['catalog']}`")

            profile_graph = {}
            profile_graph_parents = {}
            for profile_spec in catalog_list:
                profile_id = profile_spec.pop("profile")
                parent_profile = profile_spec.pop("parent_profile", None)
                profile_graph.setdefault(parent_profile, [profile_id])

                profile_mapping[profile_id] = _to_model(profile_spec, target, cls)
                if parent_profile:
                    profile_graph_parents[profile_id] = parent_profile

            _resolve_profile_tree(profile_graph, profile_graph_parents, profile_mapping)

            return profile_mapping

        for field_name, profile_selector in _internal_recursion(cls):
            p = _resolve_profiles(profile_selector, data)
            for name, profile in p.items():
                profiles[(profile_selector["catalog"], profile_selector["target"], field_name, name)] = profile
        
        return profiles

    def _apply_profiles(self, profiles: dict[tuple[str, str, str, str], EosDesignsRootModel]) -> EosDesignsRootModel:
        def _match_profiles(instance: AvdModel) -> Generator[EosDesignsRootModel]:
            for field_name, field_spec in instance._fields.items():
                t = field_spec['type']

                field_value = getattr(instance, field_name)
                if t is AvdProfileRef:
                    k = (field_spec["catalog"], field_spec["target"], field_name, field_value)
                    if k not in profiles:
                        raise KeyError(f"profile '{field_value}' is missing")
                    yield profiles[k]
                elif isinstance(field_value, AvdList):
                    for next_instance in field_value:
                        yield from _match_profiles(next_instance)
                elif isinstance(field_value, AvdModel):
                    yield from _match_profiles(field_value)
            
        for profile in _match_profiles(self):
            self._deepmerge(profile)
        return self

    @classmethod
    def _get_csc_items(cls, data: Mapping) -> Iterator[EosDesigns._CustomStructuredConfigurationsItem]:
        """
        Returns a list of _CustomStructuredConfigurationsItem objects containing each custom structured configuration extracted from the inputs.

        Find any keys starting with any prefix defined under "custom_structured_configuration_prefix".
        """
        prefixes = data.get("custom_structured_configuration_prefix", cls._get_field_default_value("custom_structured_configuration_prefix"))
        if not isinstance(prefixes, (list, AvdList)):
            # Invalid prefix format.
            return

        for prefix in prefixes:
            if not isinstance(prefix, str):
                # Invalid prefix format.
                continue

            if not (matching_keys := [key for key in data if str(key).startswith(prefix) and key not in SKIP_KEYS]):
                continue

            prefix_length = len(prefix)
            for key in matching_keys:
                yield cls._CustomStructuredConfigurationsItem(key=key, value=EosCliConfigGen._from_dict({key[prefix_length:]: data[key]}))

    @classmethod
    def _get_dynamic_keys(cls, data: Mapping) -> EosDesigns._DynamicKeys:
        """
        Returns the DynamicKeys object which holds a list for each dynamic key.

        The lists contain an entry for each dynamic key found in the inputs and the content of that key conforming to the schema.

        The corresponding data models are auto created by the conversion from schemas, which also sets "_dynamic_key_maps" on the class:
        ```python
        _dynamic_key_maps: list[dict] = [{"dynamic_keys_path": "connected_endpoints_keys.key", "model_key": "connected_endpoints_keys"}, ...]
        ```

        Here we parse "_dynamic_key_maps" and for entry  find all values for the dynamic_keys_path (ex "node_type_keys.key") in the input data
        to identify all dynamic keys (ex "l3leaf", "spine" ...)
        """
        schema = create_store(load_from_yaml=False)["eos_designs"]

        dynamic_keys_dict = {}

        for dynamic_key_map in cls._DynamicKeys._dynamic_key_maps:
            dynamic_keys_path: str = dynamic_key_map["dynamic_keys_path"]
            model_key_list: list = dynamic_keys_dict.setdefault(dynamic_key_map["model_key"], [])

            # TODO: Improve the fetch of default. We need to store the default value somewhere, since this is executed before __init__ of EosDesigns.
            data_with_default = get_instance_with_defaults(data, dynamic_keys_path, schema)
            dynamic_keys = get_all(data_with_default, dynamic_keys_path)
            for dynamic_key in dynamic_keys:
                # dynamic_key is one key like "l3leaf".
                if (value := data.get(dynamic_key)) is None:
                    # Do not add missing key or None.
                    continue

                model_key_list.append({"key": dynamic_key, "value": value})

        # TODO: Just create to proper data models instead of using coerce type.
        return cls._DynamicKeys._from_dict(dynamic_keys_dict)


class ProfileSelector(TypedDict):
    catalog: str
    target: str


class ProfileSpec(TypedDict):
    profile: str
    parent_profile: str

def _internal_recursion(model: type[AvdModel]) -> Generator[tuple[str, ProfileSelector]]:
    for field_name, field_spec in model._fields.items():
        t = field_spec.get("type")
        if t is AvdProfileRef:
            yield field_name, cast(ProfileSelector, field_spec)
        elif isinstance(t, type) and issubclass(t, AvdModel):
            yield from _internal_recursion(t)
        elif isinstance(t, type) and issubclass(t, (AvdList, AvdIndexedList)) \
            and issubclass(t._item_type, AvdModel):
            yield from _internal_recursion(t._item_type)
