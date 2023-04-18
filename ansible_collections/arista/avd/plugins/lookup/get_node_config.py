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
        - If the value is not set, None/null will be returned.
    options:
        _terms:
            description: List of keys to be read and returned. Supports dot-notation for deeper lookups.
            required: True
    notes:
        - By default the lookup will read the variables for the current node.
        - It is possible to read variables for another by setting variables=hostvars[<other hostname>]
"""
from copy import deepcopy

from ansible.plugins.lookup import LookupBase, display

from ansible_collections.arista.avd.plugins.plugin_utils.eos_designs_shared_utils import SharedUtils
from ansible_collections.arista.avd.plugins.plugin_utils.schema.avdschematools import AvdSchemaTools
from ansible_collections.arista.avd.plugins.plugin_utils.utils import get


class LookupModule(LookupBase):
    def run(self, terms, variables=None, **kwargs):
        # First of all populate options,
        # this will already take into account env vars and ini config
        self.set_options(var_options=variables, direct=kwargs)

        ret = []
        avd_schema_tools = AvdSchemaTools(
            variables["inventory_hostname"],
            display,
            schema_id="eos_designs",
            conversion_mode="debug",
            validation_mode="error",
            plugin_name="get_node_config",
        )
        hostvars = deepcopy(variables)
        avd_schema_tools.convert_and_validate_data(hostvars)
        avd_shared_utils = SharedUtils(hostvars, self._templar)
        node_config = avd_shared_utils.switch_data_combined

        for term in terms:
            display.debug(f"get_node_config lookup term: {term}")
            ret.append(get(node_config, term))

        return ret
