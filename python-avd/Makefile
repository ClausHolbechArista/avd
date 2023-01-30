CURRENT_DIR = $(shell pwd)
ANSIBLE_AVD_DIR ?= ..
LIB = lib
LIB_DIR = pyavd/$(LIB)
J2_DIR = $(LIB_DIR)/j2
TESTS_DIR = tests
EOS_CLI_CONFIG_GEN_TEMPLATE_DIR = pyavd/templates
EOS_CLI_CONFIG_GEN_SCHEMAS_DIR = pyavd/schemas

.PHONY: help
help: ## Display help message
	@grep -E '^[0-9a-zA-Z_-]+\.*[0-9a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONE: dep
dep: copy-libs fix-libs copy-test-data ## Copy and Fix Ansible AVD python files

.PHONY: copy-libs
copy-libs: ## Copy files from Ansible AVD collection
	rm -rf $(LIB_DIR)
	mkdir -p $(LIB_DIR)
	cp -r $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/plugins/plugin_utils/* $(LIB_DIR)
	cp -r $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/plugins/module_utils/* $(LIB_DIR)
	mkdir -p $(J2_DIR)
	cp -r $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/plugins/filter $(J2_DIR)
	cp -r $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/plugins/test $(J2_DIR)
	rm -rf $(EOS_CLI_CONFIG_GEN_TEMPLATE_DIR)
	mkdir -p $(EOS_CLI_CONFIG_GEN_TEMPLATE_DIR)
	mkdir -p $(EOS_CLI_CONFIG_GEN_TEMPLATE_DIR)/j2cache
	cp -r $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/roles/eos_cli_config_gen/templates/* $(EOS_CLI_CONFIG_GEN_TEMPLATE_DIR)
	rm -rf $(EOS_CLI_CONFIG_GEN_SCHEMAS_DIR)
	mkdir -p $(EOS_CLI_CONFIG_GEN_SCHEMAS_DIR)
	cp -r $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/roles/eos_cli_config_gen/schemas/eos_cli_config_gen.schema.yml $(EOS_CLI_CONFIG_GEN_SCHEMAS_DIR)/

.PHONY: fix-libs
fix-libs: ## Fix/remove various Ansible specifics things from python files
	find -name '*.py' -exec sed -z -i 's/from __future__ import (absolute_import, division, print_function)\n__metaclass__ = type\n\n//g' {} +
	find -name '*.py' -exec sed -i 's/ansible_collections\.arista\.avd\.plugins\.plugin_utils/$(LIB)/g' {} +
	find -name '*.py' -exec sed -i 's/ansible_collections\.arista\.avd\.plugins\.module_utils/$(LIB)/g' {} +
	find -name '*.py' -exec sed -i 's/ansible_collections\.arista\.avd\.plugins\.filter/$(LIB)\.j2\.filter/g' {} +

.PHONY: copy-test-data
copy-test-data: ## Copy inventory files from AVD collection Molecule scenario. Also copy custom templates to template dir
	rm -rf $(TESTS_DIR)/eos_cli_config_gen/vars
	mkdir -p $(TESTS_DIR)/eos_cli_config_gen/vars
	cp $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/molecule/eos_cli_config_gen/inventory/group_vars/all.yml $(TESTS_DIR)/eos_cli_config_gen/
	cp $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/molecule/eos_cli_config_gen/inventory/host_vars/* $(TESTS_DIR)/eos_cli_config_gen/vars/
	rm -rf $(TESTS_DIR)/eos_cli_config_gen/expected
	mkdir -p $(TESTS_DIR)/eos_cli_config_gen/expected
	cp $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/molecule/eos_cli_config_gen/intended/configs/* $(TESTS_DIR)/eos_cli_config_gen/expected/
	mkdir -p $(EOS_CLI_CONFIG_GEN_TEMPLATE_DIR)/custom_templates
	cp $(ANSIBLE_AVD_DIR)/ansible_collections/arista/avd/molecule/eos_cli_config_gen/custom_templates/* $(EOS_CLI_CONFIG_GEN_TEMPLATE_DIR)/custom_templates/
	find -name '*.yml' -exec sed -i 's/!unsafe //g' {} +

.PHONY: test
test: ## Run tests by parsing test inputs and comparing with expected outputs
	pyavd/runner.py \
	    -e $(TESTS_DIR)/eos_cli_config_gen/all.yml \
	    -g '$(TESTS_DIR)/eos_cli_config_gen/vars/*' \
	    -c $(TESTS_DIR)/eos_cli_config_gen/output/
	diff $(TESTS_DIR)/eos_cli_config_gen/expected/ $(TESTS_DIR)/eos_cli_config_gen/output/
