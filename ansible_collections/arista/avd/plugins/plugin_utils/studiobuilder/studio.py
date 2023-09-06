import json

from ansible_collections.arista.avd.plugins.plugin_utils.strip_empties import strip_null_from_data

from .studiofields import StudioField, StudioGroupField


class Studio:
    display_name: str = ""
    description: str = ""
    input_fields: list[StudioField] = []
    template: str = ""
    template_type: str = ""
    root: StudioField
    id: str | None = None

    def __init__(
        self,
        display_name: str,
        description: str,
        template: str,
        template_type: str,
        id: str | None = None,
        layout: str | None = None,
    ):
        self.display_name = display_name
        self.description = description
        self.template = template
        self.template_type = template_type
        self.id = id
        self.layout = layout

        self.root = StudioGroupField(
            name="",
            id="root",
        )
        self.input_fields.append(self.root)

    def render(self) -> dict:
        """
        Render Studio data model
        """
        studio = {
            "key": {
                "studio_id": self.id,
            },
            "display_name": self.display_name,
            "description": self.description,
            "input_schema": {
                "fields": {
                    "values": self.render_inputs(),
                },
                "layout": {
                    "value": self.render_layouts(),
                },
            },
            "template": {"type": self.template_type, "body": self.template},
        }

        return studio

    def render_inputs(self) -> dict:
        """
        Render dict with inputs for all fields merged
        """
        inputs = {}
        for field in self.input_fields:
            inputs.update(field.render_input())

        return inputs

    def render_layouts(self) -> str:
        """
        Render JSON string with dict of layouts for all fields merged
        """
        layouts = {}
        for field in self.input_fields:
            layouts.update(field.render_layout())

        if self.layout:
            layouts.update(
                {
                    "STUDIO": {
                        "type": "STUDIO",
                        "layout": self.layout,
                    }
                }
            )
        return json.dumps(layouts)

    def render_datamappings(self) -> list[dict]:
        """
        Render list of data mappings
        """
        return [datamapping for datamapping in [field.render_datamapping() for field in self.input_fields] if datamapping is not None]

    def render_tagmappings(self) -> list[dict]:
        """
        Render list of tag mappings
        """
        tagmappings = []
        for field in self.input_fields:
            if (field_tagmappings := field.render_tagmappings()) is None:
                continue
            tagmappings.extend(field_tagmappings)

        return strip_null_from_data(tagmappings)
