# python 3 headers, required if submitting to Ansible
from __future__ import absolute_import, division, print_function

__metaclass__ = type

DOCUMENTATION = r"""
    name: get_node_config
    author: Arista Ansible Team (@aristanetworks)
    version_added: "4.0.0"
    short_description: Get AVD node config from Fabric Topology inputs, using same logic as eos_designs.
    description:
        - This lookup returns the contents of one or more AVD node config keys.
        - Node config is the internal data set for a given node, after reading "Fabric Topology" inputs and implementing inheritance etc.
        - If the requested key is not set, None/null will be returned.
        - If no keys are requested (no argument) the entire node_config dict will be returned.
    options:
        _terms:
            description: List of keys to be read and returned. Supports dot-notation for deeper lookups.
            required: True
        hostname:
            type: str
            description:
                - By default the lookup will read the variables for the current node.
                - If hostname is set, the lookup will access hostvars of the given node.
            required: False
    notes:
        -
        - It is possible to read variables for another by setting hostname=<other hostname>
"""
from collections import ChainMap

from ansible.errors import AnsibleLookupError
from ansible.plugins.lookup import LookupBase, display

from ansible_collections.arista.avd.plugins.plugin_utils.eos_designs_shared_utils import SharedUtils
from ansible_collections.arista.avd.plugins.plugin_utils.schema.avdschematools import AvdSchemaTools
from ansible_collections.arista.avd.plugins.plugin_utils.utils import get


class LookupModule(LookupBase):
    def run(self, terms, variables=None, **kwargs):
        # First of all populate options,
        # this will already take into account env vars and ini config
        self.set_options(var_options=variables, direct=kwargs)

        default_hostname = variables["inventory_hostname"]
        hostname = self.get_option("hostname")
        if hostname is not None and hostname != default_hostname:
            hostvars = ChainMap(variables["hostvars"].get(hostname))
        else:
            hostname = default_hostname
            hostvars = ChainMap(variables["hostvars"].get(hostname))

        ret = []
        avd_schema_tools = AvdSchemaTools(
            hostname=hostname,
            ansible_display=display,
            schema_id="eos_designs",
            conversion_mode="debug",
            validation_mode="error",
            plugin_name="get_node_config",
        )
        result = avd_schema_tools.convert_and_validate_data(hostvars)
        if result.get("failed"):
            raise AnsibleLookupError(result.get("msg"))

        avd_shared_utils = SharedUtils(hostvars, self._templar)
        node_config = avd_shared_utils.switch_data_combined

        if not terms:
            return [node_config]
        for term in terms:
            display.debug(f"get_node_config lookup term: {term}")
            ret.append(get(node_config, term))

        return ret
