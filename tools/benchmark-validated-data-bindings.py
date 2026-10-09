#!/usr/bin/env python3
# Copyright (c) 2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
"""
Measure validated-data bindings with repeatable inputs, operations, and fresh processes.

Run with the AVD virtualenv and PYTHONPATH=python-avd. JSON output records individual
samples, checksums, interpreter/platform details, and process memory, not just averages.
Archive publication is setup, outside timed accessor loops. Import measurements use fresh
processes; page-cache state is deliberately not described as cold disk I/O.
"""

from __future__ import annotations

import argparse
import gc
import json
import os
import platform
import statistics
import subprocess
import sys
import time
from pathlib import Path
from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from collections.abc import Callable
    from types import ModuleType


def memory() -> dict[str, int]:
    """Return Linux process RSS/PSS/private memory in KiB."""
    values = {}
    for line in Path("/proc/self/smaps_rollup").read_text().splitlines():
        if line.startswith(("Rss:", "Pss:", "Private_Clean:", "Private_Dirty:")):
            key, amount, *_ = line.split()
            values[key.removesuffix(":")] = int(amount)
    return values


def samples(operation: Callable[[], Any], repetitions: int = 7) -> dict[str, Any]:
    """Collect repeated wall-clock samples, checking that the result is deterministic."""
    expected = operation()
    timings = []
    for _ in range(repetitions):
        gc.collect()
        start = time.perf_counter_ns()
        result = operation()
        timings.append(time.perf_counter_ns() - start)
        assert result == expected, (result, expected)  # noqa: S101 - checksum equality is a benchmark correctness guard
    return {"samples_ns": timings, "median_ns": statistics.median(timings), "checksum": expected}


def binding_module() -> ModuleType:
    """Resolve the canonical private data API without eagerly importing it during setup."""
    from pyavd import _validated_data

    return _validated_data


def child(args: argparse.Namespace) -> dict[str, Any]:
    """Measure import and opening in an independent interpreter."""
    before = memory()
    start = time.perf_counter_ns()
    models = binding_module()
    elapsed = time.perf_counter_ns() - start
    imported = memory()
    start = time.perf_counter_ns()
    root = models.open_avd_design(args.archive, args.schema)
    opening = time.perf_counter_ns() - start
    views = list(root.devices.values())
    opened = memory()
    retained = [root.devices["leaf0"] for _ in range(20_000)]
    return {
        "import_ns": elapsed,
        "open_ns": opening,
        "before": before,
        "imported": imported,
        "opened": opened,
        "devices": len(views),
        "retained_views": len(retained),
        "retained": memory(),
    }


def main() -> None:
    """Write benchmark measurements for one installed binding implementation."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--label", default="native")
    parser.add_argument("--iterations", type=int, default=20_000)
    parser.add_argument("--cpu", type=int)
    parser.add_argument("--directory", type=Path, required=True)
    parser.add_argument("--child", action="store_true")
    args = parser.parse_args()
    if args.cpu is not None:
        os.sched_setaffinity(0, {args.cpu})
    args.schema = Path(__file__).parents[1] / "python-avd/pyavd/_schema/schemas.rkyv"
    args.archive = args.directory / "benchmark-host.rkyv"
    if args.child:
        print(json.dumps(child(args)))
        return
    args.directory.mkdir(parents=True, exist_ok=True)
    models = binding_module()
    from pyavd._rust import archive_avd_design
    from pyavd._utils.undefined import Undefined

    payload = {
        "fabric_name": "BENCHMARK",
        "devices": [
            {
                "name": f"leaf{index}",
                "id": index + 1,
                "vtep": True,
                "serial_number": None,
                "uplink_interfaces": ["Ethernet1", "Ethernet2", "Ethernet3", "Ethernet4"],
                "structured_config": {"ethernet_interfaces": [{"name": "Ethernet1", "description": None}]},
            }
            for index in range(64)
        ],
        "network_services": [{"name": f"TENANT{index}", "l2vlans": [{"id": 10, "name": "first"}, {"id": 10, "name": "second"}]} for index in range(4)],
    }
    result = archive_avd_design(json.dumps(payload), args.archive, args.schema)
    assert result.destination == args.archive, result.errors_json  # noqa: S101 - publication correctness guard
    root = models.open_avd_design(args.archive, args.schema)
    devices = root.devices
    device = devices["leaf0"]
    assert device.id == 1 and device.vtep is True  # noqa: S101, PT018 - benchmark correctness guard
    assert device.serial_number is None and device.platform is Undefined  # noqa: S101, PT018 - benchmark correctness guard
    assert json.loads(device.structured_config.to_json()) == payload["devices"][0]["structured_config"]  # noqa: S101 - benchmark correctness guard
    assert [vlan.id for vlan in root.network_services["TENANT0"].l2vlans[:]] == [10, 10]  # noqa: S101 - benchmark correctness guard
    iterations = args.iterations

    def scalar() -> int:
        return sum(device.id for _ in range(iterations))

    def presence() -> int:
        return sum(device.platform is Undefined and device.serial_number is None and device.vtep for _ in range(iterations))

    def nested() -> int:
        return sum(root.devices["leaf0"].id for _ in range(iterations))

    def lists() -> int:
        return sum(len(device.uplink_interfaces[-1]) for _ in range(iterations))

    def indexed() -> int:
        return sum(devices["leaf0"].id for _ in range(iterations))

    def workload() -> int:
        total = 0
        for _ in range(100):
            for item in root.devices.values():
                total += item.id
                total += item.vtep and item.platform is Undefined
                total += sum(len(interface) for interface in item.uplink_interfaces)
                total += bool(item.structured_config)
            for tenant in root.network_services.values():
                total += sum(vlan.id for vlan in tenant.l2vlans)
        return total

    def opening() -> int:
        return len(models.open_avd_design(args.archive, args.schema).devices)

    cases = {
        name: samples(operation)
        for name, operation in {
            "scalar": scalar,
            "presence_null_bool": presence,
            "nested_lookup": nested,
            "list_navigation": lists,
            "indexed_lookup": indexed,
            "read_workload": workload,
            "open": opening,
        }.items()
    }
    fresh = []
    for _ in range(15):
        command = [sys.executable, __file__, "--child", "--directory", str(args.directory)]
        if args.cpu is not None:
            command.extend(["--cpu", str(args.cpu)])
        # Launch only this benchmark script under the active interpreter.
        process = subprocess.run(  # noqa: S603
            command,
            check=True,
            capture_output=True,
            text=True,
        )
        fresh.append(json.loads(process.stdout))
    report = {
        "label": args.label,
        "module": models.__name__,
        "generated_python_loaded": bool(getattr(models, "__file__", None)),
        "python": sys.version,
        "platform": platform.platform(),
        "cpu_count": os.cpu_count(),
        "cpu_affinity": sorted(os.sched_getaffinity(0)),
        "iterations": iterations,
        "archive_bytes": args.archive.stat().st_size,
        "cases": cases,
        "fresh_processes": fresh,
    }
    destination = args.directory / f"{args.label}.json"
    destination.write_text(json.dumps(report, indent=2) + "\n")
    print(destination)


if __name__ == "__main__":
    main()
