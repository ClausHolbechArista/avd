from concurrent.futures import ProcessPoolExecutor
from glob import iglob
from itertools import repeat
from os import path

from read_vars import read_vars
from templater import Templar
from write_result import write_result


def run_template_var_files_process(device_var_file: str, common_vars: dict, verbosity: int):
    """
    Function run as process by ProcessPoolExecutor
    """

    template_vars = common_vars.copy()
    template_vars.update(read_vars(device_var_file))

    dir = path.dirname(path.abspath(device_var_file))
    filename = path.basename(device_var_file)
    templar = Templar([dir], cache=False)

    write_result(device_var_file, templar.render_template_from_file(filename, template_vars))

    print(f"OK: {device_var_file}")


def run_template_var_files(common_varfiles: list[str], device_varfiles: str, verbosity: int):
    # Read common vars
    common_vars = {}

    for file in common_varfiles:
        common_vars.update(read_vars(file))

    # First template the common var files using their own vars
    with ProcessPoolExecutor(max_workers=20) as executor:
        return_values = executor.map(
            run_template_var_files_process,
            common_varfiles,
            repeat(common_vars),
            repeat(verbosity),
        )

    # Next template the device var files
    with ProcessPoolExecutor(max_workers=20) as executor:
        return_values = executor.map(
            run_template_var_files_process,
            iglob(device_varfiles),
            repeat(common_vars),
            repeat(verbosity),
        )

    for return_value in return_values:
        if return_value is not None:
            print(return_value)
