from collections import ChainMap

from .avd_schema_tools import AvdSchemaTools
from .constants import EOS_CLI_CONFIG_GEN_SCHEMA_ID
from .vendor.eos_designs.get_structured_config import get_structured_config
from .vendor.errors import AristaAvdError


def get_device_structured_config(hostname: str, hostvars: dict, avd_facts: dict) -> dict:
    """
    Build and return the AVD structured configuration for one device.

    Args:
        hostname: Hostname of device.
        hostvars: Dictionary of variables passed to AVD `eos_designs` modules.
            Variables should be converted and validated according to AVD `eos_designs` schema first using `pyavd.validate_inputs`.
        avd_facts: Dictionary of avd_facts as returned from `pyavd.get_avd_facts`.

    Returns:
        Device Structured Configuration as a dictionary
    """

    # Set 'inventory_hostname' on the input hostvars, to keep compatability with Ansible focused code.
    # Also map in avd_facts without touching the hostvars
    mapped_hostvars = ChainMap(
        {
            "inventory_hostname": hostname,
            "switch": avd_facts["avd_switch_facts"][hostname]["switch"],
        },
        avd_facts,
        hostvars,
    )

    output_schema_tools = AvdSchemaTools(schema_id=EOS_CLI_CONFIG_GEN_SCHEMA_ID)
    result = {}
    structured_config = get_structured_config(
        hostvars=mapped_hostvars,
        input_schema_tools=None,
        output_schema_tools=output_schema_tools,
        result=result,
        templar=None,
    )
    if result.get("failed"):
        raise AristaAvdError(f"{[str(error) for error in result['errors']]}")

    return structured_config
