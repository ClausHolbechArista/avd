import json

from ansible_collections.arista.avd.plugins.plugin_utils.utils import setattr_if_not_none

from .studiofields import StudioField, StudioGroupField


class Studio:
    display_name: str = ""
    description: str = ""
    input_fields: list[StudioField] = []
    template: str = ""
    root: StudioField
    id: str | None = None

    def __init__(
        self,
        display_name: str,
        description: str,
        template: str,
        id: str | None = None,
    ):
        self.display_name = display_name
        self.description = description
        self.template = template
        self.id = id

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
            "template": {"type": "TEMPLATE_TYPE_MAKO", "body": self.template},
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

        return json.dumps(layouts)
