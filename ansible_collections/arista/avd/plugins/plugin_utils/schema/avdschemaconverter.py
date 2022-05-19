from __future__ import (absolute_import, division, print_function)
__metaclass__ = type

from ansible_collections.arista.avd.plugins.plugin_utils.schema.avdschema import AvdSchema
from deepmerge import always_merger
import ansible_collections.arista.avd.plugins.plugin_utils.schema.studioschemaconverters as studioschemaconverters


class AvdSchemaConverter:
    def __init__(self, avdschema: AvdSchema, studio_converter: dict = None):
        self._avdschema = avdschema
        if studio_converter:
            self.studio_converter = studio_converter
        else:
            self.studio_converters = {
                # "case_sensitive": studioschemaconverters.case_sensitive,
                "default": studioschemaconverters.default,
                "description": studioschemaconverters.description,
                "display_name": studioschemaconverters.display_name,
                "format": studioschemaconverters.format,
                "items": studioschemaconverters.items,
                "keys": studioschemaconverters.keys,
                "max": studioschemaconverters.max,
                "max_length": studioschemaconverters.max_length,
                "min": studioschemaconverters.min,
                "min_length": studioschemaconverters.min_length,
                "pattern": studioschemaconverters.pattern,
                "primary_key": studioschemaconverters.primary_key,
                "required": studioschemaconverters.required,
                "type": studioschemaconverters.convert_type,
                "valid_values": studioschemaconverters.valid_values,
            }

    def to_studios(self, avd_schema_path: str = None, avd_var_name: str = None):
        if avd_schema_path:
            subschema = avd_schema_path.split(".")
            name = f"root-{avd_schema_path.replace('.', '-')}"
        else:
            subschema = []
            name = "root"
        schema = self._avdschema.subschema(subschema)
        return self.convert(schema, self.studio_converters, name, avd_var_name)

    def convert(self, input: dict, converters: dict, name: str = "root", var_name: str = ""):
        output = {name: {"name": var_name, "id": name, "label": var_name}}
        for key, value in input.items():
            if key in converters:
                always_merger.merge(output, converters[key](self, value, name, input, converters))
        return output
