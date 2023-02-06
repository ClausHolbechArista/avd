CURRENT_DIR = $(shell pwd)
ANSIBLE_AVD_DIR ?= ..
LIB = lib
LIB_DIR = pyavd/$(LIB)
J2_DIR = $(LIB_DIR)/j2
TESTS_DIR = tests
EOS_CLI_CONFIG_GEN_TEMPLATE_DIR = pyavd/templates
SCHEMAS_DIR = pyavd/schemas
EOS_DESIGNS_MODULES_DIR = $(LIB_DIR)/eos_designs

.PHONY: help
help: ## Display help message
	@grep -E '^[0-9a-zA-Z_-]+\.*[0-9a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: dep
dep: copy-libs fix-libs copy-test-data ## Copy and Fix Ansible AVD python files

.PHONY: clean
	rm -rf $(LIB_DIR)
	rm -rf $(EOS_CLI_CONFIG_GEN_TEMPLATE_DIR)
	rm -rf $(SCHEMAS_DIR)

.PHONY: copy-libs
copy-libs: ## Copy files from Ansible AVD collection
	rm -rf $(LIB_DIR)
	rm -rf $(EOS_CLI_CONFIG_GEN_TEMPLATE_DIR)
	rm -rf $(SCHEMAS_DIR)
	rm -rf $(EOS_DESIGNS_MODULES_DIR)

	mkdir -p $(LIB_DIR)
	mkdir -p $(J2_DIR)
	mkdir -p $(EOS_CLI_CONFIG_GEN_TEMPLATE_DIR)/j2cache
	mkdir -p $(SCHEMAS_DIR)
	mkdir -p $(EOS_DESIGNS_MODULES_DIR)

	cp -r $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/plugins/plugin_utils/* $(LIB_DIR)
	cp -r $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/plugins/module_utils/* $(LIB_DIR)
	cp -r $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/plugins/filter $(J2_DIR)
	cp -r $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/plugins/test $(J2_DIR)
	cp -r $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/roles/eos_cli_config_gen/templates/* $(EOS_CLI_CONFIG_GEN_TEMPLATE_DIR)
	cp $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/roles/eos_cli_config_gen/schemas/eos_cli_config_gen.schema.yml $(SCHEMAS_DIR)/

	cp -r $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/roles/eos_designs/python_modules/* $(EOS_DESIGNS_MODULES_DIR)/
	mv $(LIB_DIR)/eos_designs_facts.py $(EOS_DESIGNS_MODULES_DIR)/
	cp -r $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/roles/eos_designs/schemas/eos_designs.schema.yml $(SCHEMAS_DIR)/

.PHONY: fix-libs
fix-libs: ## Fix/remove various Ansible specifics things from python files
	find -name '*.py' -exec sed -z -i 's/from __future__ import (absolute_import, division, print_function)\n__metaclass__ = type\n\n//g' {} +
	find -name '*.py' -exec sed -i 's/ansible_collections\.arista\.avd\.plugins\.plugin_utils/$(LIB)/g' {} +
	find -name '*.py' -exec sed -i 's/ansible_collections\.arista\.avd\.plugins\.module_utils/$(LIB)/g' {} +
	find -name '*.py' -exec sed -i 's/ansible_collections\.arista\.avd\.plugins\.filter/$(LIB)\.j2\.filter/g' {} +
	find -name '*.py' -exec sed -i 's/ansible_collections\.arista\.avd\.roles\.eos_designs\.python_modules/$(LIB)\.eos_designs/g' {} +

.PHONY: copy-test-data
copy-test-data: ## Copy inventory files from AVD collection Molecule scenario. Also copy custom templates to template dir
	rm -rf $(TESTS_DIR)

	mkdir -p $(TESTS_DIR)/eos_cli_config_gen/vars
	mkdir -p $(TESTS_DIR)/eos_cli_config_gen/expected_configs
	mkdir -p $(TESTS_DIR)/eos_cli_config_gen/configs
	mkdir -p $(EOS_CLI_CONFIG_GEN_TEMPLATE_DIR)/custom_templates
	mkdir -p $(TESTS_DIR)/eos_designs/vars
	mkdir -p $(TESTS_DIR)/eos_designs/facts
	mkdir -p $(TESTS_DIR)/eos_designs/expected_structured_configs
	mkdir -p $(TESTS_DIR)/eos_designs/structured_configs
	mkdir -p $(TESTS_DIR)/eos_designs/expected_configs
	mkdir -p $(TESTS_DIR)/eos_designs/configs

	cp $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/molecule/eos_cli_config_gen/inventory/group_vars/all.yml $(TESTS_DIR)/eos_cli_config_gen/
	cp $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/molecule/eos_cli_config_gen/inventory/host_vars/* $(TESTS_DIR)/eos_cli_config_gen/vars/
	cp $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/molecule/eos_cli_config_gen/intended/configs/* $(TESTS_DIR)/eos_cli_config_gen/expected_configs/
	cp $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/molecule/eos_cli_config_gen/custom_templates/* $(EOS_CLI_CONFIG_GEN_TEMPLATE_DIR)/custom_templates/
	find -name '*.yml' -exec sed -i 's/!unsafe //g' {} +

	cp $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/roles/eos_designs/defaults/main/*.yml $(TESTS_DIR)/eos_designs/
	cp $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/molecule/eos_designs_unit_tests/intended/structured_configs/*.yml $(TESTS_DIR)/eos_designs/expected_structured_configs/
	cp $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/molecule/eos_designs_unit_tests/intended/configs/*.cfg $(TESTS_DIR)/eos_designs/expected_configs/
	for FILE in `ls -1 $(TESTS_DIR)/eos_designs/expected_structured_configs/*.yml` ; \
	do \
	basename $${FILE}; \
	ansible-inventory -i $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/molecule/eos_designs_unit_tests/inventory/hosts.yml \
	-y --host "`basename $${FILE%.yml}`" > $(TESTS_DIR)/eos_designs/vars/`basename $${FILE}`; \
	done

	# Update test vars for default vars.
	sed -i -e 's/{{ inventory_dir }}/$(TESTS_DIR)\/eos_designs/g' $(TESTS_DIR)/eos_designs/main.yml

	pyavd/runner.py --template_vars \
	    --device_varfiles '$(TESTS_DIR)/eos_designs/*.yml' \
	    --common_varfile $(TESTS_DIR)/eos_designs/main.yml \
	    --common_varfile $(TESTS_DIR)/eos_designs/default-connected-endpoints-keys.yml \
	    --common_varfile $(TESTS_DIR)/eos_designs/default-node-type-keys.yml \
	    --common_varfile $(TESTS_DIR)/eos_designs/default-platform-settings.yml \
	    --common_varfile $(TESTS_DIR)/eos_designs/default-templates.yml

	sed -i -e "s/\"\(\[.*\]\)\"/\1/g" $(TESTS_DIR)/eos_designs/main.yml
	sed -i -e "s/\"\({.*}\)\"/\1/g" $(TESTS_DIR)/eos_designs/main.yml
	sed -i 's/ansible_collections\.arista\.avd\.roles\.eos_designs\.python_modules/$(LIB)\.eos_designs/g' $(TESTS_DIR)/eos_designs/main.yml

	# Update test vars for devices
	find $(TESTS_DIR)/eos_designs/vars -name '*.yml' -exec sed -i -e 's/^.*{{ switch\.id == 6 }}.*$$//g' {} +
	find $(TESTS_DIR)/eos_designs/vars -name '*.yml' -exec sed -i -e "s/^.*arista.avd.default(''MGMT'').*$$//g" {} +
	find $(TESTS_DIR)/eos_designs/vars -name '*.yml' -exec sed -i -e "s/^.*arista.avd.default(''Management1'').*$$//g" {} +
	find $(TESTS_DIR)/eos_designs/vars -name '*.yml' -exec sed -i -e 's/^.*{{ playbook_dir }}.*$$//g' {} +
	find $(TESTS_DIR)/eos_designs/vars -name '*.yml' -exec sed -i -e 's/^.*{{ inventory_dir }}.*$$//g' {} +
	find $(TESTS_DIR)/eos_designs/vars -name '*.yml' -exec sed -i -e "s/'\({{.*'.*'.*}}\)'/\"\\\1\"/g" {} +
	find $(TESTS_DIR)/eos_designs/vars -name '*.yml' -exec sed -i -e 's/^.*{{ switch.mgmt_interface }}.*$$//g' {} +
	find $(TESTS_DIR)/eos_designs/vars -name '*.yml' -exec sed -i -e "s/custom_templates\/.*$$/null/g" {} +

	pyavd/runner.py --template_vars \
	    --device_varfiles '$(TESTS_DIR)/eos_designs/vars/*.yml' \
	    --common_varfile $(TESTS_DIR)/eos_designs/main.yml \
	    --common_varfile $(TESTS_DIR)/eos_designs/default-connected-endpoints-keys.yml \
	    --common_varfile $(TESTS_DIR)/eos_designs/default-node-type-keys.yml \
	    --common_varfile $(TESTS_DIR)/eos_designs/default-platform-settings.yml \
	    --common_varfile $(TESTS_DIR)/eos_designs/default-templates.yml

	find $(TESTS_DIR)/eos_designs/vars -name '*.yml' -exec sed -i -e "s/'\(\[.*\]\)'/\1/g" {} +
	find $(TESTS_DIR)/eos_designs/vars -name '*.yml' -exec sed -i -e "s/'\({.*}\)'/\1/g" {} +

.PHONY: test-0
test-0: ## Run tests by parsing test inputs and comparing with expected configs
	pyavd/runner.py --eos_cli_config_gen \
	    -t $(TESTS_DIR)/eos_cli_config_gen/all.yml \
	    -s '$(TESTS_DIR)/eos_cli_config_gen/vars/*' \
	    -c $(TESTS_DIR)/eos_cli_config_gen/configs/
	diff -q $(TESTS_DIR)/eos_cli_config_gen/expected_configs/ $(TESTS_DIR)/eos_cli_config_gen/configs/

.PHONY: test-3
test-3: ## Run tests by parsing test inputs and comparing with expected configs
	pyavd/runner.py --eos_cli_config_gen \
	    --struct_cfgfiles '$(TESTS_DIR)/eos_designs/structured_configs/*' \
	    --cfgfiles $(TESTS_DIR)/eos_designs/configs
	diff -q $(TESTS_DIR)/eos_designs/expected_configs/ $(TESTS_DIR)/eos_designs/configs/

.PHONY: test-2
test-2: ## Run tests by parsing test inputs and comparing with expected structured configs
	pyavd/runner.py --eos_designs_structured_configs \
	    --factsfile '$(TESTS_DIR)/eos_designs/facts/eos_designs_facts.yml' \
	    --device_varfiles '$(TESTS_DIR)/eos_designs/vars/*' \
	    --common_varfile $(TESTS_DIR)/eos_designs/main.yml \
	    --common_varfile $(TESTS_DIR)/eos_designs/default-connected-endpoints-keys.yml \
	    --common_varfile $(TESTS_DIR)/eos_designs/default-node-type-keys.yml \
	    --common_varfile $(TESTS_DIR)/eos_designs/default-platform-settings.yml \
	    --common_varfile $(TESTS_DIR)/eos_designs/default-templates.yml \
		--struct_cfgfiles $(TESTS_DIR)/eos_designs/structured_configs
	diff -q $(TESTS_DIR)/eos_designs/expected_structured_configs/ $(TESTS_DIR)/eos_designs/structured_configs/

.PHONY: test-1
test-1: ## Run tests by parsing test inputs and comparing with expected configs
	pyavd/runner.py --eos_designs_facts \
	    --factsfile '$(TESTS_DIR)/eos_designs/facts/eos_designs_facts.yml' \
	    --device_varfiles '$(TESTS_DIR)/eos_designs/vars/*' \
	    --common_varfile $(TESTS_DIR)/eos_designs/main.yml \
	    --common_varfile $(TESTS_DIR)/eos_designs/default-connected-endpoints-keys.yml \
	    --common_varfile $(TESTS_DIR)/eos_designs/default-node-type-keys.yml \
	    --common_varfile $(TESTS_DIR)/eos_designs/default-platform-settings.yml \
	    --common_varfile $(TESTS_DIR)/eos_designs/default-templates.yml

.PHONE: test
test: test-1 test-3
