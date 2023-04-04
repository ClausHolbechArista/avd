from ansible_collections.arista.avd.plugins.plugin_utils.schema.avdschema import AvdSchema
from ansible_collections.arista.avd.plugins.plugin_utils.schema.key_to_display_name import key_to_display_name

from .studiofields import StudioBooleanField, StudioCollectionField, StudioGroupField, StudioIntegerField, StudioResolverField, StudioStringField


class AvdToStudioSchemaConverter:
    def __init__(self, avdschema: AvdSchema):
        self.avdschema = avdschema
        self.converters = {
            # "case_sensitive": studioschemaconverters.case_sensitive,
            "int": self.intconverter,
            "str": self.strconverter,
            "bool": self.boolconverter,
            "list": self.listconverter,
            "dict": self.dictconverter,
        }

    def convert_schema(
        self, avd_schema_path: list[str], name: str, parent: StudioGroupField | StudioCollectionField | StudioResolverField, set_path: list | None
    ):
        schema = self.avdschema.subschema(avd_schema_path)

        return self.convert_key(schema=schema, name=name, parent=parent, set_path=set_path)

    def convert_key(self, schema: dict, name: str, parent: StudioGroupField | StudioCollectionField | StudioResolverField, set_path: list | None = None):
        if (schema_type := schema.get("type")) not in self.converters:
            raise ValueError(f"Unable to convert schema type {schema_type} on name {name}. Must be one of {list(self.converters)}")

        return self.converters[schema_type](schema, name, parent, set_path)

    def strconverter(self, schema, name, parent, set_path: list | None):
        format_converters = {
            "ipv4": "ip",
            "ipv4_cidr": "cidr",
            "ipv6": "ipv6",
            "ipv6_cidr": "cidr",
            "ip": "ip",
            "cidr": "cidr",
            "mac": "mac",
        }
        return [
            StudioStringField(
                name=name,
                parent=parent,
                label=schema.get("display_name", key_to_display_name(name)),
                default_value=schema.get("default"),
                required=schema.get("required"),
                set_path=set_path,
                min_length=schema.get("min_length"),
                max_length=schema.get("max_length"),
                static_options=schema.get("valid_values"),
                string_format=format_converters.get(schema.get("format", ""), None),
                pattern=schema.get("pattern"),
            )
        ]

    def intconverter(self, schema, name, parent, set_path: list | None):
        return [
            StudioIntegerField(
                name=name,
                parent=parent,
                label=schema.get("display_name", key_to_display_name(name)),
                default_value=schema.get("default"),
                required=schema.get("required"),
                set_path=set_path,
                min=schema.get("min"),
                max=schema.get("max"),
                static_options=schema.get("valid_values"),
            )
        ]

    def boolconverter(self, schema, name, parent, set_path: list | None):
        return [
            StudioBooleanField(
                name=name,
                parent=parent,
                label=schema.get("display_name", key_to_display_name(name)),
                default_value=schema.get("default"),
                required=schema.get("required"),
                set_path=set_path,
            )
        ]

    def listconverter(self, schema, name, parent, set_path: list | None):
        if not (item_schema := schema.get("items")):
            return []

        collection_field = StudioCollectionField(
            name=name,
            parent=parent,
            label=schema.get("display_name", key_to_display_name(name)),
            required=schema.get("required"),
            set_path=set_path,
            key=schema.get("primary_key"),
        )
        fields = [collection_field]
        fields.extend(self.convert_key(item_schema, "item", collection_field, set_path=None))
        return fields

    def dictconverter(self, schema, name, parent, set_path: list | None):
        if not (keys := schema.get("keys")):
            return []

        group_field = StudioGroupField(
            name=name,
            parent=parent,
            label=schema.get("display_name", key_to_display_name(name)),
            required=schema.get("required"),
            set_path=set_path,
        )
        fields = [group_field]
        for key, subschema in keys.items():
            fields.extend(self.convert_key(subschema, key, group_field, set_path=None))

        return fields
