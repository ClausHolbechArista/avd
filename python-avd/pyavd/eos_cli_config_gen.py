#!/usr/bin/env python3
import argparse
import json
from collections import ChainMap
from os import path
from sys import stdin

import yaml
from jinja2 import Environment, FileSystemBytecodeCache, FileSystemLoader, StrictUndefined
from lib.j2.filter.convert_dicts import convert_dicts
from lib.j2.filter.default import default
from lib.j2.filter.list_compress import list_compress
from lib.j2.filter.natural_sort import natural_sort
from lib.j2.filter.range_expand import range_expand
from lib.j2.test.contains import contains
from lib.j2.test.defined import defined
from lib.merge.merge import merge
from lib.schema.avdschema import AvdSchema

JINJA2_EXTENSIONS = ["jinja2.ext.loopcontrols", "jinja2.ext.do", "jinja2.ext.i18n"]
JINJA2_CUSTOM_FILTERS = {
    "arista.avd.default": default,
    "arista.avd.convert_dicts": convert_dicts,
    "arista.avd.list_compress": list_compress,
    "arista.avd.natural_sort": natural_sort,
    "arista.avd.range_expand": range_expand,
}
JINJA2_CUSTOM_TESTS = {
    "arista.avd.defined": defined,
    "arista.avd.contains": contains,
}
JINJA2_TEMPLATE_PATHS = [path.join(path.realpath(path.dirname(__file__)), "templates")]
JINJA2_CONFIG_TEMPLATE = "eos-intended-config.j2"
JINJA2_DOCUMENTAITON_TEMPLATE = "eos-device-documentation.j2"
AVD_SCHEMA_FILE = path.join(path.realpath(path.dirname(__file__)), "schemas", "eos_cli_config_gen.schema.yml")


class Undefined(StrictUndefined):
    """
    Allow nested checks for undefined instead of having to check on every level.
    Example "{% if var.key.subkey is arista.avd.undefined %}" is ok.

    Without this it we would have to test every level, like
    "{% if var is arista.avd.undefined or var.key is arista.avd.undefined or var.key.subkey is arista.avd.undefined %}"

    Inspired from Ansible's AnsibleUndefined class.
    """

    def __getattr__(self, name):
        # Return original Undefined object to preserve the first failure context
        return self

    def __getitem__(self, key):
        # Return original Undefined object to preserve the first failure context
        return self

    def __repr__(self):
        return f"Undefined(hint={self._undefined_hint}, obj={self._undefined_obj}, name={self._undefined_name})"

    def __contains__(self, item):
        # Return original Undefined object to preserve the first failure context
        return self


def convert_and_validate(data: dict, verbosity: int = 0) -> None:
    with open(AVD_SCHEMA_FILE, "r", encoding="UTF-8") as file:
        schema = yaml.safe_load(file.read())

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


def eos_cli_config_gen(
    hostname: str,
    template_vars: dict,
    render_configuration: bool = True,
    render_documentation: bool = False,
    verbosity: int = 0,
) -> tuple[str | None, str | None]:
    configuration = None
    documentation = None
    if not isinstance(template_vars, (dict, ChainMap)):
        raise TypeError(f"vars argument must be a dictionary. Got {type(template_vars)}")

    convert_and_validate(template_vars, verbosity)
    template_vars["inventory_hostname"] = hostname

    loader = FileSystemLoader(JINJA2_TEMPLATE_PATHS)
    bytecode_cache = FileSystemBytecodeCache(path.join(JINJA2_TEMPLATE_PATHS[0], "j2cache"))
    environment = Environment(
        extensions=JINJA2_EXTENSIONS,
        loader=loader,
        undefined=Undefined,
        trim_blocks=True,
        bytecode_cache=bytecode_cache,
    )
    environment.filters.update(JINJA2_CUSTOM_FILTERS)
    environment.tests.update(JINJA2_CUSTOM_TESTS)

    if render_configuration:
        configuration = environment.get_template(JINJA2_CONFIG_TEMPLATE).render(template_vars)

    if render_documentation:
        documentation = environment.get_template(JINJA2_DOCUMENTAITON_TEMPLATE).render(template_vars)

    return configuration, documentation


def read_vars(filename):
    if filename == "/dev/stdin" and stdin.isatty():
        print("Write variables in YAML or JSON format and end with ctrl+d to exit")
    with open(filename, "r", encoding="UTF-8") as file:
        data = file.read()

    try:
        return json.loads(data)
    except json.JSONDecodeError:
        pass

    return yaml.safe_load(data) or {}


def write_result(filename, result):
    mode = "w+"
    if filename == "/dev/stdout":
        mode = "w"

    with open(filename, mode, encoding="UTF-8") as file:
        file.write(result)


def main():
    parser = argparse.ArgumentParser(
        prog="eos_cli_config_gen",
        description="Render Arista EOS Configurations from Structured Configuration variables",
        epilog="See https://avd.sh/en/stable/roles/eos_cli_config_gen for details on supported variables",
    )
    parser.add_argument("hostname", help="Device hostname")
    parser.add_argument(
        "--varfile",
        "-v",
        help=(
            "YAML or JSON File where variables are read from. Default is stdin."
            " Multiple files can be added by repeating the argument."
            " Data will be deepmerged in the order of the varfile arguments."
        ),
        action="append",
    )
    parser.add_argument("--cfgfile", "-c", help="Destination file for device configuration")
    parser.add_argument("--docfile", "-d", help="Destination file for device documentation")
    args = parser.parse_args()

    vars = {}
    files = args.varfile or ["/dev/stdin"]
    for file in files:
        merge(vars, read_vars(file), recursive=False)

    render_configuration = args.cfgfile is not None
    render_documentation = args.docfile is not None
    configuration, documentation = eos_cli_config_gen(args.hostname, vars, render_configuration, render_documentation)

    if render_configuration:
        write_result(args.cfgfile, configuration)

    if render_documentation:
        write_result(args.docfile, documentation)


if __name__ == "__main__":
    main()
