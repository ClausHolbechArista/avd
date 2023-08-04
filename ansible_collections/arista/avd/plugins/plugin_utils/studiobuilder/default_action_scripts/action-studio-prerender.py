# Copyright (c) 2022 Arista Networks, Inc.  All rights reserved.
# Arista Networks, Inc. Confidential and Proprietary.
# Subject to Arista Networks, Inc.'s EULA.
# FOR INTERNAL USE ONLY. NOT FOR DISTRIBUTION.
# pylint: skip-file
"""
action-studio-prerender.py

This is a prebuild action running after workspace prebuild in a studio build pipeline.
This action is running once per studio, so it handles all devices covered in the studio.

For now we will not be using this action. Everything has been moved to the template
"""

pyavd_timer = time()
structured_config = get_device_structured_config(hostname, device_vars, avd_switch_facts)
runtimes["pyavd_struct_cfg"] = str(time() - pyavd_timer)
pyavd_timer = time()
eos_config = get_device_config(structured_config)
runtimes["pyavd_eos_cfg"] = str(time() - pyavd_timer)
