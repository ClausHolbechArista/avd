import glob
from concurrent.futures import ProcessPoolExecutor
from itertools import repeat
from os import path

from lib.avdfacts import AvdFacts
from lib.eos_designs.base import AvdStructuredConfig as EosDesignsBase
from lib.eos_designs.connected_endpoints import AvdStructuredConfig as EosDesignsConnectedEndpoints
from lib.eos_designs.core_interfaces import AvdStructuredConfig as EosDesignsCoreInterfaces
from lib.eos_designs.custom_structured_configuration import AvdStructuredConfig as EosDesignsCustomStructuredConfiguration
from lib.eos_designs.inband_management import AvdStructuredConfig as EosDesignsInbandManagement
from lib.eos_designs.l3_edge import AvdStructuredConfig as EosDesignsL3Edge
from lib.eos_designs.mlag import AvdStructuredConfig as EosDesignsMlag
from lib.eos_designs.network_services import AvdStructuredConfig as EosDesignsNetworkServices
from lib.eos_designs.overlay import AvdStructuredConfig as EosDesignsOverlay
from lib.eos_designs.underlay import AvdStructuredConfig as EosDesignsUnderlay
from lib.merge import merge
from read_vars import read_vars
from schema_tools import AvdSchemaTools
from write_result import write_result
from yaml import dump as yaml_dump

OUTPUT_SCHEMA_FILE = path.join(path.realpath(path.dirname(__file__)), "schemas", "eos_cli_config_gen.schema.yml")
EOS_DESIGNS_MODULES = {
    "base": EosDesignsBase,
    "mlag": EosDesignsMlag,
    "underlay": EosDesignsUnderlay,
    "overlay": EosDesignsOverlay,
    "core_interfaces": EosDesignsCoreInterfaces,
    "l3_edge": EosDesignsL3Edge,
    "network_services": EosDesignsNetworkServices,
    "connected_endpoints": EosDesignsConnectedEndpoints,
    "inband_management": EosDesignsInbandManagement,
    "custom_structured_configuration": EosDesignsCustomStructuredConfiguration,
}


def eos_designs_structured_configs(
    hostname: str,
    vars: dict,
    modules: list[str] | None = None,
    verbosity: int = 0,
) -> tuple[str | None, str | None]:
    """
    Main function for eos_designs_structured_configs to render structured configs for one device.

    Is used by run_eos_designs_structured_configs_process worker function but can also be called by other frameworks.

    Parameters
    ----------
    hostname : str
        Hostname of device. Set as 'inventory_hostname' on the input vars, to keep compatability with Ansible focused code.
    vars : dict
        Dictionary of variables passed to modules. Variables are converted and validated according to AVD Schema first.
    modules : list[str] | None
        List of eos_designs python modules to run. Must be one of the supported modules set in constant EOS_DESIGNS_MODULES.
        If not set, the full list of modules will be run.
    verbosity: int
        Vebosity level for output. Passed along to other functions

    Returns
    -------
    structured_config : dict
        Device structured configuration rendered by the given modules
    """

    if not modules:
        modules = EOS_DESIGNS_MODULES.keys()

    vars["inventory_hostname"] = hostname

    output_schematools = AvdSchemaTools(read_vars(OUTPUT_SCHEMA_FILE), hostname, plugin="pyavd_eos_designs_structured_configs", verbosity=0)

    structured_config = {}
    for module in modules:
        if module not in EOS_DESIGNS_MODULES:
            raise ValueError(f"Unknown eos_designs module '{module}' during render of eos_designs_structured_config for host '{hostname}'")

        eos_designs_module: AvdFacts = EOS_DESIGNS_MODULES[module](vars, None)
        result = eos_designs_module.render()
        output_schematools.convert_data(result)
        structured_config = merge(structured_config, result, destructive_merge=False)
    return structured_config


def run_eos_designs_structured_configs_process(device_var_file: str, common_vars: dict, struct_cfg_file_dir: str, verbosity: int) -> None:
    """
    Function run as process by ProcessPoolExecutor.

    Read device variables from files and run eos_designs_structured_configs for one device.

    Parameters
    ----------
    device_var_file : str
        Path to device specific var file to import and shallow merge on top of common vars and facts.
        Filename will be used as hostname.
    common_vars : dict
        Common vars to be applied to all devices.
        Per-device facts will be extracted from avd_switch_facts and merged on top of device vars.
    struct_cfg_file_dir: str
        Path to dir for output structured_config files.
    verbosity: int
        Vebosity level for output. Passed along to other functions
    """

    hostname = str(path.basename(device_var_file)).removesuffix(".yaml").removesuffix(".yml").removesuffix(".json")

    device_vars = common_vars.copy()
    device_vars.update(read_vars(device_var_file))
    device_vars.update(common_vars["avd_switch_facts"][hostname])

    structured_configuration = eos_designs_structured_configs(hostname, device_vars, verbosity=verbosity)
    write_result(path.join(struct_cfg_file_dir, f"{hostname}.yml"), yaml_dump(structured_configuration))
    print(f"OK: {hostname}")


def run_eos_designs_structured_configs(common_varfiles: list[str], fact_file: str, device_varfiles: str, struct_cfgfiles: str, verbosity: int) -> None:
    """
    Read common variables from files and run eos_cli_config_gen for each device in process workers.

    Intended for CLI use via runner.py

    Parameters
    ----------
    common_varfiles : list[str]
        List of common var files to import and merge.
    fact_file : str
        Path to fact file produced by eos_designs_facts. Shallow merged on top of common vars
    device_varfiles : str
        Glob for device specific var files to import and shallow merge on top of common vars.
        Filenames will be used as hostname.
    struct_cfgfiles: str
        Path to dir for output structured_config files.
    verbosity: int
        Vebosity level for output. Passed along to other functions
    """

    # Read common vars
    common_vars = {}

    for file in common_varfiles:
        common_vars.update(read_vars(file))

    common_vars.update(read_vars(fact_file))

    with ProcessPoolExecutor(max_workers=20) as executor:
        return_values = executor.map(
            run_eos_designs_structured_configs_process,
            glob.iglob(device_varfiles),
            repeat(common_vars),
            repeat(struct_cfgfiles),
            repeat(verbosity),
        )

    for return_value in return_values:
        if return_value is not None:
            print(return_value)
