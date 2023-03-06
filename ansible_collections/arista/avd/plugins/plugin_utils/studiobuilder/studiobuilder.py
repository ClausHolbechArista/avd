from pathlib import Path
from uuid import uuid4

from ansible_collections.arista.avd.plugins.plugin_utils.schema.avdschema import AvdSchema
from ansible_collections.arista.avd.plugins.plugin_utils.schema.key_to_display_name import key_to_display_name

from .avdtostudioschemaconverter import AvdToStudioSchemaConverter
from .studio import Studio
from .studiofields import (
    StudioBooleanField,
    StudioCollectionField,
    StudioField,
    StudioGroupField,
    StudioIntegerField,
    StudioResolverField,
    StudioStringField,
    StudioTaggerField,
)


class AvdStudioBuilder:
    def __init__(
        self,
        avdschema: AvdSchema,
    ):
        self._converter = AvdToStudioSchemaConverter(avdschema)

        self.builders = {
            "avd_schema": self.buildfromavdschema,
            "string": self.buildstringfield,
            "integer": self.buildintegerfield,
            "boolean": self.buildbooleanfield,
            "collection": self.buildcollectionfield,
            "group": self.buildgroupfield,
            "resolver": self.buildresolverfield,
            "tagger": self.buildtaggerfield,
        }

    def build(self, studio_design: dict):
        """
        Build CloudVision Studio based on studio_design.

        Returns
        -------
        dict
          Complete studio object adhering to CloudVision studio.v1 resource API schema
        """

        if template_file := studio_design.get("template_file"):
            template = Path(template_file).read_text(encoding="UTF-8")
        else:
            template = ""

        self.studio = Studio(
            display_name=studio_design["display_name"],
            description=studio_design["description"],
            template=template,
            id=studio_design.get("studio_id", uuid4()),
        )

        # Builder all studio input fields at root level giving the root group as parent.
        # Everything else is built recursively in build_fields.
        self.build_fields(fields=studio_design.get("studio_inputs", []), parent=self.studio.root)

        return self.studio.render()

    def build_fields(self, fields: list[dict], parent: StudioField):
        for field in fields:
            if (field_type := field.get("type")) not in self.builders:
                raise ValueError(f"Unknown field type {field_type}. Must be one of {list(self.builders)}")

            self.builders[field_type](field=field, parent=parent)

    def buildfromavdschema(self, field: dict, parent: StudioField):
        self.studio.input_fields.extend(self._converter.convert_schema(field.get("schema_path"), field.get("name"), parent=parent))

    def buildstringfield(self, field: dict, parent: StudioField):
        self.studio.input_fields.append(
            StudioStringField(
                name=field.get("name"),
                parent=parent,
                label=field.get("display_name", key_to_display_name(field.get("name"))),
                default_value=field.get("default"),
                required=field.get("required"),
                min_length=field.get("min_length"),
                max_length=field.get("max_length"),
                static_options=field.get("static_options"),
                string_format=field.get("format"),
                pattern=field.get("pattern"),
            )
        )

    def buildintegerfield(self, field: dict, parent: StudioField):
        self.studio.input_fields.append(
            StudioIntegerField(
                name=field.get("name"),
                parent=parent,
                label=field.get("display_name", key_to_display_name(field.get("name"))),
                default_value=field.get("default"),
                required=field.get("required"),
                min=field.get("min"),
                max=field.get("max"),
                static_options=field.get("static_options"),
            )
        )

    def buildbooleanfield(self, field: dict, parent: StudioField):
        self.studio.input_fields.append(
            StudioBooleanField(
                name=field.get("name"),
                parent=parent,
                label=field.get("display_name", key_to_display_name(field.get("name"))),
                default_value=field.get("default"),
                required=field.get("required"),
            )
        )

    def buildcollectionfield(self, field: dict, parent: StudioField):
        collection = StudioCollectionField(
            name=field.get("name"),
            parent=parent,
            label=field.get("display_name", key_to_display_name(field.get("name"))),
            required=field.get("required"),
            key=field.get("key"),
            layout=field.get("layout"),
        )
        self.studio.input_fields.append(collection)
        if not (item_input := field.get("items")):
            return

        self.build_fields([item_input], collection)

    def buildgroupfield(self, field: dict, parent: StudioField):
        group = StudioGroupField(
            name=field.get("name"),
            parent=parent,
            label=field.get(
                "display_name",
                key_to_display_name(
                    field.get("name"),
                ),
            ),
            required=field.get("required"),
        )
        self.studio.input_fields.append(group)

        if not (members := field.get("members")):
            return

        self.build_fields(members, group)

    def buildresolverfield(self, field: dict, parent: StudioField):
        resolver = StudioResolverField(
            name=field.get("name"),
            parent=parent,
            label=field.get("display_name", key_to_display_name(field.get("name"))),
            description=field.get("description"),
            required=field.get("required"),
            layout=field.get("layout"),
            prepopulated=field.get("prepopulated"),
            resolver_type=field.get("resolver_type"),
            tag_type=field.get("tag_type"),
            tag_label=field.get("tag_label"),
            tag_filter_query=field.get("tag_filter_query"),
        )
        self.studio.input_fields.append(resolver)
        if not (item_input := field.get("items")):
            return

        self.build_fields([item_input], resolver)

    def buildtaggerfield(self, field: dict, parent: StudioField):
        self.studio.input_fields.append(
            StudioTaggerField(
                name=field.get("name"),
                columns=field.get("columns", []),
                description=field.get("description"),
                parent=parent,
                required=field.get("required"),
                tag_type=field.get("tag_type"),
                assignment_type=field.get("assignment_type"),
            )
        )
