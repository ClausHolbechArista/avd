from __future__ import annotations

from ansible_collections.arista.avd.plugins.plugin_utils.utils import setattr_if_not_none, update_if_not_none


class StudioField:
    """
    Base class for all Studio Fields

    TODO: Add remaining schema properties
    """

    id = ""
    name = ""
    label = ""
    description: str | None = None
    required: bool | None = None
    parent: "StudioGroupField" | "StudioCollectionField" | "StudioResolverField" | None
    type: str

    def __init__(
        self,
        name: str,
        label: str | None = None,
        description: str | None = None,
        parent: "StudioGroupField" | "StudioCollectionField" | "StudioResolverField" | None = None,
        required: bool | None = None,
        id: str = None,
    ) -> None:
        if parent is not None:
            self.parent = parent
            self.id = id or f"{parent.id}-{name}"
            if isinstance(parent, StudioGroupField):
                parent.members.append(self)
            elif isinstance(parent, (StudioCollectionField, StudioResolverField)):
                parent.base_field = self
        else:
            self.id = id or name

        self.name = name
        self.required = required
        self.description = description
        setattr_if_not_none(self, "label", label)

    def render_input(self) -> dict[str, dict]:
        """
        Render Studios input schema for this field
        """
        schema = {
            self.id: {
                "id": self.id,
                "type": self.type,
                "name": self.name,
                "label": self.label,
            }
        }
        update_if_not_none(schema[self.id], "description", self.description)
        return schema

    def render_layout(self) -> dict[str, dict]:
        """
        Render Studios layout schema for this field

        # TODO add other layout schema options
        """
        return {}


class StudioGroupField(StudioField):
    """
    Group Studio Input
    A group is similar to a dict

    TODO: Add remaining schema properties
    """

    members: list[StudioField]
    layout: str | None = None
    type = "INPUT_FIELD_TYPE_GROUP"

    def member_ids(self) -> list[str]:
        """
        Return list of member field ids
        """
        return [member.id for member in self.members]

    def __init__(
        self,
        name: str,
        label: str | None = None,
        description: str | None = None,
        parent: "StudioGroupField" | "StudioCollectionField" | "StudioResolverField" | None = None,
        required: bool | None = None,
        id: str = None,
        layout: str | None = None,
    ) -> None:
        super().__init__(name=name, label=label, description=description, parent=parent, required=required, id=id)

        self.layout = layout
        self.members = []

    def render_input(self) -> dict[str, dict]:
        """
        Render Studios input schema for this field
        """
        schema = super().render_input()
        schema[self.id].update(
            {
                "group_props": {
                    "members": {
                        "values": self.member_ids(),
                    }
                }
            }
        )
        return schema

    def render_layout(self) -> dict[str, dict]:
        """
        Render Studios layout schema for this field

        # TODO add other layout schema options
        """
        schema = super().render_layout()
        if self.layout == "hierarchical":
            schema.setdefault(self.id, {}).update(
                {
                    "key": self.id,
                    "type": "INPUT",
                    "isPageLayout": True,
                }
            )

        return schema


class StudioCollectionField(StudioField):
    """
    Collection Studio Input
    A collection is similar to a list

    TODO: Add remaining schema properties
    """

    base_field: StudioField | None = None
    key: str | None = None
    """
    Key is a string with the name of the child group member that is the "primary_key" of this collection.
    The id of the key field is derived during rendering, since we do not know during creation.
    """
    layout: str | None = None
    type = "INPUT_FIELD_TYPE_COLLECTION"

    def __init__(
        self,
        name: str,
        label: str | None = None,
        description: str | None = None,
        parent: "StudioGroupField" | "StudioCollectionField" | "StudioResolverField" | None = None,
        required: bool | None = None,
        id: str = None,
        layout: str | None = None,
        key: str | None = None,
    ) -> None:
        super().__init__(name=name, label=label, description=description, parent=parent, required=required, id=id)

        self.layout = layout
        self.key = key

    def render_input(self) -> dict[str, dict]:
        """
        Render Studios input schema for this field
        """
        schema = super().render_input()

        props = {"base_field_id": self.base_field.id}
        if self.key is not None and isinstance(self.base_field, StudioGroupField):
            for member in self.base_field.members:
                if member.name == self.key:
                    props["key"] = member.id

        schema[self.id].update({"collection_props": props})

        return schema

    def render_layout(self) -> dict[str, dict]:
        """
        Render Studios layout schema for this field

        # TODO add other layout schema options
        """
        schema = super().render_layout()
        if self.layout == "hierarchical":
            schema.setdefault(self.id, {}).update(
                {
                    "key": self.id,
                    "type": "INPUT",
                    "isPageLayout": True,
                }
            )

        return schema


class StudioStringField(StudioField):
    """
    String Studio Input

    TODO: Add remaining schema properties
    """

    type = "INPUT_FIELD_TYPE_STRING"
    default_value: str | None = None
    min_length: int | None = None
    max_length: int | None = None
    pattern: str | None = None
    static_options: list[str] | None = None
    string_format: str | None = None

    def __init__(
        self,
        name: str,
        label: str | None = None,
        description: str | None = None,
        parent: "StudioGroupField" | "StudioCollectionField" | "StudioResolverField" | None = None,
        required: bool | None = None,
        id: str = None,
        default_value: str | None = None,
        min_length: int | None = None,
        max_length: int | None = None,
        pattern: int | None = None,
        static_options: list[str] | None = None,
        string_format: str | None = None,
    ) -> None:
        super().__init__(name=name, label=label, description=description, parent=parent, required=required, id=id)

        self.default_value = default_value
        self.min_length = min_length
        self.max_length = max_length
        self.pattern = pattern
        self.static_options = static_options
        self.string_format = string_format

    def length(self) -> str | None:
        """
        Create string with lengths depending on which values are set:
        <min_length>..<max_length>
        min..<max_length>
        <min_length>..max
        Returns None if neither value is set.
        """
        if self.min_length is None and self.max_length is None:
            return None

        minimum = self.min_length or "min"
        maximum = self.max_length or "max"
        return f"{minimum}..{maximum}"

    def render_static_options(self) -> dict[str, list] | None:
        """
        Render static options or or None if no static options are set
        """
        if self.static_options:
            return {"values": self.static_options}
        return None

    def render_input(self) -> dict[str, dict]:
        """
        Render Studios input schema for this field
        """
        schema = super().render_input()

        props = {}
        update_if_not_none(props, "length", self.length())
        update_if_not_none(props, "pattern", self.pattern)
        update_if_not_none(props, "default_value", self.default_value)
        update_if_not_none(props, "static_options", self.render_static_options())
        update_if_not_none(props, "format", self.string_format)

        schema[self.id].update({"string_props": props})
        return schema

    def render_layout(self) -> dict[str, dict]:
        """
        Render Studios layout schema for this field

        # TODO add other layout schema options
        """
        schema = super().render_layout()
        return schema


class StudioIntegerField(StudioField):
    """
    String Studio Input

    TODO: Add remaining schema properties
    """

    type = "INPUT_FIELD_TYPE_INTEGER"
    default_value: int | None = None
    min: int | None = None
    max: int | None = None
    static_options: list[int] | None = None

    def __init__(
        self,
        name: str,
        label: str | None = None,
        description: str | None = None,
        parent: "StudioGroupField" | "StudioCollectionField" | "StudioResolverField" | None = None,
        required: bool | None = None,
        id: str = None,
        default_value: int | None = None,
        min: int | None = None,
        max: int | None = None,
        static_options: list[int] | None = None,
    ) -> None:
        super().__init__(name=name, label=label, description=description, parent=parent, required=required, id=id)

        self.default_value = default_value
        self.min = min
        self.max = max
        self.static_options = static_options

    def range(self) -> str | None:
        """
        Create string with range depending on which values are set:
        <min>..<max>
        min..<max>
        <min>..max
        Returns None if neither value is set.
        """
        if self.min is None and self.max is None:
            return None

        minimum = self.min or "min"
        maximum = self.max or "max"
        return f"{minimum}..{maximum}"

    def render_static_options(self) -> dict[str, list] | None:
        """
        Render static options or or None if no static options are set
        """
        if self.static_options:
            return {"values": self.static_options}
        return None

    def render_input(self) -> dict[str, dict]:
        """
        Render Studios input schema for this field
        """
        schema = super().render_input()

        props = {}
        update_if_not_none(props, "range", self.range())
        update_if_not_none(props, "default_value", self.default_value)
        update_if_not_none(props, "static_options", self.render_static_options())

        schema[self.id].update({"integer_props": props})
        return schema

    def render_layout(self) -> dict[str, dict]:
        """
        Render Studios layout schema for this field

        # TODO add other layout schema options
        """
        schema = super().render_layout()
        return schema


class StudioBooleanField(StudioField):
    """
    Boolean Studio Input

    TODO: Add remaining schema properties
    """

    type = "INPUT_FIELD_TYPE_BOOLEAN"
    default_value: bool | None = None

    def __init__(
        self,
        name: str,
        label: str | None = None,
        description: str | None = None,
        parent: "StudioGroupField" | "StudioCollectionField" | "StudioResolverField" | None = None,
        required: bool | None = None,
        id: str = None,
        default_value: bool | None = None,
    ) -> None:
        super().__init__(name=name, label=label, description=description, parent=parent, required=required, id=id)

        self.default_value = default_value

    def render_input(self) -> dict[str, dict]:
        """
        Render Studios input schema for this field
        """
        schema = super().render_input()

        props = {}
        update_if_not_none(props, "default_value", self.default_value)

        schema[self.id].update({"boolean_props": props})
        return schema

    def render_layout(self) -> dict[str, dict]:
        """
        Render Studios layout schema for this field

        # TODO add other layout schema options
        """
        schema = super().render_layout()
        return schema


class StudioResolverField(StudioField):
    """
    Resolver Studio Input

    TODO: Add remaining schema properties
    """

    base_field: StudioField | None = None
    layout: str | None = None
    prepopulated = False
    resolver_type = "multi"
    tag_type = "device"
    tag_label = ""
    tag_filter_query = ""
    type = "INPUT_FIELD_TYPE_RESOLVER"

    def __init__(
        self,
        name: str,
        label: str | None = None,
        description: str | None = None,
        parent: "StudioGroupField" | "StudioCollectionField" | "StudioResolverField" | None = None,
        required: bool | None = None,
        id: str = None,
        layout: str | None = None,
        prepopulated: bool | None = None,
        resolver_type: str | None = None,
        tag_type: str | None = None,
        tag_label: str | None = None,
        tag_filter_query: str | None = None,
    ) -> None:
        super().__init__(name=name, label=label, description=description, parent=parent, required=required, id=id)

        self.layout = layout
        setattr_if_not_none(self, "prepopulated", prepopulated)
        setattr_if_not_none(self, "resolver_type", resolver_type)
        setattr_if_not_none(self, "tag_type", tag_type)
        setattr_if_not_none(self, "tag_label", tag_label)
        setattr_if_not_none(self, "tag_filter_query", tag_filter_query)

    def display_mode(self) -> str:
        if self.prepopulated and self.resolver_type == "single":
            return "RESOLVER_FIELD_DISPLAY_MODE_ALL"
        else:
            return "RESOLVER_FIELD_DISPLAY_MODE_SPARSE"

    def input_mode(self) -> str:
        if self.tag_type == "interface" and self.resolver_type == "single":
            return "RESOLVER_FIELD_INPUT_MODE_SINGLE_INTERFACE_TAG"
        elif self.tag_type == "interface":
            return "RESOLVER_FIELD_INPUT_MODE_MULTI_INTERFACE_TAG"
        elif self.resolver_type == "single":
            return "RESOLVER_FIELD_INPUT_MODE_SINGLE_DEVICE_TAG"
        else:
            return "RESOLVER_FIELD_INPUT_MODE_MULTI_DEVICE_TAG"

    def render_input(self) -> dict[str, dict]:
        """
        Render Studios input schema for this field
        """
        schema = super().render_input()

        props = {
            "base_field_id": self.base_field.id,
            "display_mode": self.display_mode(),
            "input_mode": self.input_mode(),
            "input_tag_label": self.tag_label,
            "tag_filter_query": self.tag_filter_query,
        }
        schema[self.id].update({"resolver_props": props})

        return schema

    def render_layout(self) -> dict[str, dict]:
        """
        Render Studios layout schema for this field

        # TODO add other layout schema options
        """
        schema = super().render_layout()
        if self.layout == "hierarchical":
            schema.setdefault(self.id, {}).update(
                {
                    "key": self.id,
                    "type": "INPUT",
                    "isPageLayout": True,
                }
            )

        return schema


class StudioTaggerField(StudioField):
    """
    Tagger Studio Input

    TODO: Add remaining schema properties
    """

    parent: StudioField | None = None
    tag_type = "device"
    assignment_type = "single"
    columns = []

    def __init__(
        self,
        name: str,
        columns: list[dict],
        description: str | None = None,
        parent: "StudioGroupField" | "StudioCollectionField" | "StudioResolverField" | None = None,
        required: bool | None = None,
        id: str = None,
        tag_type: str | None = None,
        assignment_type: str | None = None,
    ) -> None:
        # Notice that we remove parent from the options, since taggers are not regular members of parent groups.
        super().__init__(name=name, description=description, required=required, id=id or f"{parent.id}-{name}")

        self.parent = parent
        self.columns = columns
        setattr_if_not_none(self, "tag_type", tag_type)
        setattr_if_not_none(self, "assignment_type", assignment_type)

    def render_columns(self) -> list[dict]:
        return [
            {
                "tagLabel": column["tag_label"],
                "suggestedValues": column.get("suggested_values", []),
            }
            for column in self.columns
        ]

    def render_input(self) -> dict[str, dict]:
        """
        Taggers are not part of the input schema.
        They are only in layout schema
        """
        return {}

    def render_layout(self) -> dict[str, dict]:
        """
        Render Studios layout schema for this field

        # TODO add other layout schema options
        """
        schema = super().render_layout()
        schema.setdefault(self.id, {}).update(
            {
                "type": "TAGGER",
                "parentKey": self.parent.id,
                "key": self.id,
                "name": self.name,
                "description": self.description or "",
                "tagType": self.tag_type.upper(),
                "assignmentType": self.assignment_type.upper(),
                "columns": self.render_columns(),
            }
        )
        return schema
