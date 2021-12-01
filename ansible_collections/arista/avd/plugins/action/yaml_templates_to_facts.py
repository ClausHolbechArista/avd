from __future__ import (absolute_import, division, print_function)
__metaclass__ = type

import yaml
from ansible.plugins.action import ActionBase
from ansible.errors import AnsibleActionFail
from ansible.utils.vars import isidentifier
from ansible.plugins.filter.core import combine
from ansible.plugins.loader import lookup_loader
from ansible_collections.arista.avd.plugins.module_utils.strip_empties import strip_null_from_data
from datetime import datetime;

class ActionModule(ActionBase):
    def run(self, tmp=None, task_vars=None):
        if task_vars is None:
            task_vars = {}

        result = super().run(tmp, task_vars)
        del tmp  # tmp no longer has any effect

        root_key = ""

        if self._task.args:
            if "root_key" in self._task.args:
                n = self._task.args.get("root_key")
                n = self._templar.template(n)
                if not isidentifier(n):
                    raise AnsibleActionFail(f"The argument 'root_key' value of '{n}' is not valid. Keys must start with a letter or underscore character, "
                                            "and contain only letters, numbers and underscores.")
                root_key = n

            if "templates" in self._task.args:
                t = self._task.args.get("templates")
                if isinstance(t, list):
                    template_list = t
                else:
                    raise AnsibleActionFail("The argument 'templates' is not a list")
            else:
                raise AnsibleActionFail("The argument 'templates' must be set")

            debug = self._task.args.get("debug",False)

        else:
            raise AnsibleActionFail("The argument 'templates' must be set")

        output = {}

        template_lookup_module = lookup_loader.get('ansible.builtin.template', loader=self._loader, templar=self._templar)

        template_vars = task_vars

        if debug:
            avd_yaml_templates_to_facts_debug = template_vars.get('avd_yaml_templates_to_facts_debug',[])

        for template_item in template_list:
            if debug:
                debug_item = template_item
                debug_item['timestamps'] = { "starting": datetime.now() }
                
            template = template_item.get('template')
            if not template:
                raise AnsibleActionFail("Invalid template data")

            template_options = template_item.get('options', {})
            list_merge = template_options.get('list_merge', 'append')

            strip_empty_keys = template_options.get('strip_empty_keys', True)

            if root_key:
                template_vars[root_key] = output
            else:
                template_vars = combine(task_vars, output, recursive=True)

            if debug:
                debug_item['timestamps']['run_template'] = datetime.now()

            template_output = template_lookup_module.run([template], template_vars)
            if debug:
                debug_item['timestamps']['load_yaml'] = datetime.now()

            template_output_data = yaml.safe_load(template_output[0])

            if strip_empty_keys:
                if debug:
                    debug_item['timestamps']['strip_empty_keys'] = datetime.now()

                template_output_data = strip_null_from_data(template_output_data)

            if template_output_data:
                if debug:
                    debug_item['timestamps']['combine_data'] = datetime.now()

                output = combine(output, template_output_data, recursive=True, list_merge=list_merge)

            if debug:
                debug_item['timestamps']['done'] = datetime.now()
                avd_yaml_templates_to_facts_debug.append(debug_item)

        if debug:
            output['avd_yaml_templates_to_facts_debug'] = avd_yaml_templates_to_facts_debug

        if root_key:
            result['ansible_facts'] = {root_key: output}
        else:
            result['ansible_facts'] = output
        return result
