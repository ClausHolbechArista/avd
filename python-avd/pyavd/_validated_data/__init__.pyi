# Copyright (c) 2026 Arista Networks, Inc.
# Generated from the AVD schema. Do not edit by hand.
# These stubs are the Python API documentation; there is no companion Python source.
from pathlib import Path

from .avd_design import AVDDesign as AVDDesign
from .eos_config import EOSConfig as EOSConfig

def open_avd_design(archive: Path, schema_archive: Path) -> AVDDesign: ...
