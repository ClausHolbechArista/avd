"""
action-studio-prerender.py

This is a prebuild action running after workspace prebuild in a studio build pipeline.
This action is running once per studio, so it handles all devices covered in the studio.

The purpose of this action is:
 - Run AVD Structured Config for the scope of the studio (eos_designs / yaml_templates_to_facts)
 - Store result in cache per device
"""
