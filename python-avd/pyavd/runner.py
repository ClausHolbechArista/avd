#!/usr/bin/env python3
import argparse
import glob
from concurrent.futures import ProcessPoolExecutor
from itertools import repeat
from os import path

from eos_cli_config_gen import eos_cli_config_gen, read_vars, write_result


# Define function to run as process
def run_eos_cli_config_gen(device_var_file: str, common_vars: dict, cfg_file_dir: str, doc_file_dir: str, verbosity: int):
    render_configuration = cfg_file_dir is not None
    render_documentation = doc_file_dir is not None

    device_vars = common_vars.copy()
    device_vars.update(read_vars(device_var_file))
    hostname = str(path.basename(device_var_file)).removesuffix(".yaml").removesuffix(".yml").removesuffix(".json")
    configuration, documentation = eos_cli_config_gen(hostname, device_vars, render_configuration, render_documentation, verbosity)
    if render_configuration:
        write_result(path.join(cfg_file_dir, f"{hostname}.cfg"), configuration)
    if render_documentation:
        write_result(path.join(doc_file_dir, f"{hostname}.md"), documentation)

    print(f"OK: {hostname}")


def main():
    parser = argparse.ArgumentParser(
        prog="Runner",
        description="Run AVD Components like eos_cli_config_gen for multiple devices",
        epilog="See https://avd.sh/en/stable/ for details on supported variables",
    )
    parser.add_argument(
        "--device_varfiles",
        "-g",
        help=(
            "Glob covering paths to YAML or JSON Files where device variables are read from."
            " Files matched by the glob will be iterated over,"
            " and the filename will decide the hostname of each device."
            " Data will be deepmerged on top of common_vars."
            " NOTE: Remember to enclose the glob in single quotes to avoid shell from expanding it."
        ),
    )
    parser.add_argument(
        "--common_varfile",
        "-e",
        help=(
            "YAML or JSON File where common variables are read from."
            " Multiple files can be added by repeating the argument."
            " Data will be deepmerged in the order of the common_varfile arguments."
        ),
        action="append",
    )
    parser.add_argument(
        "--cfgfiles",
        "-c",
        help="Destination directory for device configuration files Filenames will be <hostname>.cfg",
    )
    parser.add_argument(
        "--docfiles",
        "-d",
        help="Destination directory for device documentation files Filenames will be <hostname>.md",
    )
    parser.add_argument(
        "--verbose",
        "-v",
        help="Enable verbose output. Can be repeated for more details.",
        action="count",
        default=0,
    )
    args = parser.parse_args()

    # Read common vars
    common_vars = {}
    for file in args.common_varfile:
        common_vars.update(read_vars(file))

    with ProcessPoolExecutor(max_workers=20) as executor:
        return_values = executor.map(
            run_eos_cli_config_gen,
            glob.iglob(args.device_varfiles),
            repeat(common_vars),
            repeat(args.cfgfiles),
            repeat(args.docfiles),
            repeat(args.verbose),
        )

    for return_value in return_values:
        if return_value is not None:
            print(return_value)


if __name__ == "__main__":
    main()
