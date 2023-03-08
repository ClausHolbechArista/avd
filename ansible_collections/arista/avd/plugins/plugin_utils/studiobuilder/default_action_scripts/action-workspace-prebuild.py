"""
action-workspace-prebuild.py

This is a prebuild action running after studio prebuild in a studio build pipeline.
This action is running once per workspace, so it handles all studios and devices.

The purpose of this action is:
 - Run AVD Topology (eos_designs_facts)
 - Store result in cache
"""
