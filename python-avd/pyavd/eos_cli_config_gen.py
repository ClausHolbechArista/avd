import glob
from concurrent.futures import ProcessPoolExecutor
from itertools import repeat
from os import path

from read_vars import read_vars
from schema_tools import AvdSchemaTools
from templater import Templar
from write_result import write_result

JINJA2_TEMPLATE_PATHS = [path.join(path.realpath(path.dirname(__file__)), "templates")]
JINJA2_CONFIG_TEMPLATE = "eos-intended-config.j2"
JINJA2_DOCUMENTAITON_TEMPLATE = "eos-device-documentation.j2"
AVD_SCHEMA_FILE = path.join(path.realpath(path.dirname(__file__)), "schemas", "eos_cli_config_gen.schema.yml")


def eos_cli_config_gen(
    hostname: str,
    template_vars: dict,
    render_configuration: bool = True,
    render_documentation: bool = False,
    verbosity: int = 0,
) -> tuple[str | None, str | None]:
    """
    Main function for eos_cli_config_gen to render configs and/or documentation for one device.

    Is used by run_eos_cli_config_gen_process worker function but can also be called by other frameworks.

    Parameters
    ----------
    hostname : str
        Hostname of device. Set as 'inventory_hostname' on the input vars, to keep compatability with Ansible focused code.
    template_vars : dict
        Dictionary of variables applied to template. Variables are converted and validated according to AVD Schema first.
    device_varfiles : str
        Glob for device specific var files to import and merge on top of common vars.
        Filenames will be used as hostname.
    render_configuration: bool, default=True
        If true, the device configuration will be rendered and returned
    render_documentation: bool, default=False
        If true, the device documentation will be rendered and returned
    verbosity: int
        Vebosity level for output. Passed along to other functions

    Returns
    -------
    configuration : str
        Device configuration in EOS CLI format. None if render_configuration is not true.
    documentation : str
        Device documentation in markdown format. None if render_documentation is not true.
    """

    configuration = None
    documentation = None

    AvdSchemaTools(read_vars(AVD_SCHEMA_FILE), hostname, plugin="pyavd_eos_cli_config_gen", verbosity=0).validate_data(template_vars)

    template_vars["inventory_hostname"] = hostname
    templar = Templar(JINJA2_TEMPLATE_PATHS)

    if render_configuration:
        configuration = templar.render_template_from_file(JINJA2_CONFIG_TEMPLATE, template_vars)

    if render_documentation:
        documentation = templar.render_template_from_file(JINJA2_DOCUMENTAITON_TEMPLATE, template_vars)

    return configuration, documentation


def run_eos_cli_config_gen_process(device_var_file: str, common_vars: dict, cfg_file_dir: str | None, doc_file_dir: str | None, verbosity: int) -> None:
    """
    Function run as process by ProcessPoolExecutor.

    Read device variables from files and run eos_cli_config_gen for one device.

    Parameters
    ----------
    device_var_file : str
        Path to device specific var file to import and merge on top of common vars.
        Filename will be used as hostname.
    common_vars : dict
        Common vars to be applied on all devices.
    cfg_file_dir: str | None
        Path to dir for output config file if set.
    doc_file_dir: str | None
        Path to dir for output documentation file if set.
    verbosity: int
        Vebosity level for output. Passed along to other functions
    """

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


def run_eos_cli_config_gen(common_varfiles: list[str], device_varfiles: str, cfgfiles_dir: str | None, docfiles_dir: str | None, verbosity: int) -> None:
    """
    Read common variables from files and run eos_cli_config_gen for each device in process workers.

    Intended for CLI use via runner.py

    Parameters
    ----------
    common_varfiles : list[str]
        List of common var files to import and merge.
    device_varfiles : str
        Glob for device specific var files to import and merge on top of common vars.
        Filenames will be used as hostname.
    cfgfiles_dir: str | None
        Path to dir for output config files if set.
    docfiles_dir: str | None
        Path to dir for output documentation files if set.
    verbosity: int
        Vebosity level for output. Passed along to other functions
    """

    # Read common vars
    common_vars = {}

    for file in common_varfiles:
        common_vars.update(read_vars(file))

    with ProcessPoolExecutor(max_workers=20) as executor:
        return_values = executor.map(
            run_eos_cli_config_gen_process,
            glob.iglob(device_varfiles),
            repeat(common_vars),
            repeat(cfgfiles_dir),
            repeat(docfiles_dir),
            repeat(verbosity),
        )

    for return_value in return_values:
        if return_value is not None:
            print(return_value)
