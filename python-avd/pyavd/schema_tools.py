from lib.schema.avdschema import AvdSchema
from lib.schema.avdschematools import AvdSchemaTools as AnsibleAvdSchemaTools
from yaml import safe_load as yaml_safe_load


def convert_and_validate(data: dict, schema_file: str, verbosity: int = 0) -> None:
    with open(schema_file, "r", encoding="UTF-8") as file:
        schema = yaml_safe_load(file.read())

    avdschema = AvdSchema(schema)
    for conversion_error in avdschema.convert(data):
        if verbosity > 0:
            print(conversion_error)

    validation_failed = False
    for validation_error in avdschema.validate(data):
        print(validation_error)
        validation_failed = True

    if validation_failed:
        raise ValueError("The supplied vars are not valid according to the schema")


class Display:
    """
    Emulating Ansible's display function where needed to be compatible with schema tools written for Ansible

    TODO: Evaludate if we yield here instead of printing.
    TODO: colors?, handling verbosity?, nicer deprecation messages
    """

    def __init__(self, verbosity: int = 0):
        self.verbosity = verbosity

    def deprecated(self, msg, version, date, collection_name, removed):
        print(f"DEPRECATED: {msg} {version} {date} {collection_name} {removed}")

    def error(self, msg, *args):
        print(f"ERROR: {msg}")

    def display(self, msg, *args):
        print(f"INFO: {msg}")

    def v(self, msg, *args):
        if self.verbosity > 2:
            print(f"DEBUG: {msg}")

    def warning(self, msg, *args):
        print(f"WARNING: {msg}")


class AvdSchemaTools(AnsibleAvdSchemaTools):
    """
    Wrapper of AvdSchemaTools built for Ansible. Here we overload init to avoid Ansible objects.

    Tools that wrap the various schema components for easy reuse
    """

    def __init__(self, schema: dict, hostname: str, conversion_mode: str = None, validation_mode: str = None, plugin: str = None, verbosity: int = 0) -> None:
        self.avdschema = AvdSchema(schema)

        self.hostname = hostname
        self.ansible_display = Display(verbosity)
        self.plugin_name = plugin
        self._set_conversion_mode(conversion_mode)
        self._set_validation_mode(validation_mode)
