import glob
from os import path
from typing import Iterable

from lib.eos_designs.eos_designs_facts import EosDesignsFacts
from read_vars import read_vars
from write_result import write_result
from yaml import safe_dump as yaml_dump

JINJA2_TEMPLATE_PATHS = [path.join(path.realpath(path.dirname(__file__)), "templates")]


def eos_designs_facts(
    all_hostvars: dict[str, dict],
    verbosity: int = 0,
) -> dict[str, dict]:
    """
    Render eos_designs_facts from hostvars.

    Note! No support for inline templating or jinja templates for descriptions or ip addressing

    Parameters
    ----------
    all_hostvars : dict
        hostname1 : dict
        hostname2 : dict
    verbosity : int, optional, default=0

    Returns
    -------
    dict
        avd_switch_facts : dict
        avd_overlay_peers : dict
        avd_topology_peers : dict
    """

    avd_switch_facts_instances = create_avd_switch_facts_instances(all_hostvars.keys(), all_hostvars)

    avd_switch_facts = render_avd_switch_facts(avd_switch_facts_instances)
    avd_overlay_peers, avd_topology_peers = render_peers(avd_switch_facts)

    return {
        "avd_switch_facts": avd_switch_facts,
        "avd_overlay_peers": avd_overlay_peers,
        "avd_topology_peers": avd_topology_peers,
    }


def create_avd_switch_facts_instances(fabric_hosts: Iterable, all_hostvars: dict):
    """
    Create "avd_switch_facts_instances" dictionary

    Parameters
    ----------
    fabric_hosts : Iterable
        Iterable of hostnames
    all_hostvars : dict
        hostname1 : dict
        hostname2 : dict

    Returns
    -------
    dict
        hostname1 : dict
            switch : <EosDesignsFacts object>,
        hostname2 : dict
            switch : <EosDesignsFacts object>,
        ...
    """
    avd_switch_facts = {}
    for host in fabric_hosts:
        host_hostvars = all_hostvars[host]
        avd_switch_facts[host] = {}

        # Add reference to dict "avd_switch_facts".
        # This is used to access EosDesignsFacts objects of other switches during rendering of one switch.
        host_hostvars["avd_switch_facts"] = avd_switch_facts

        # Notice templar is set as None, so any calls to jinja templates will fail with Nonetype has no "_loader" attribute
        avd_switch_facts[host] = {"switch": EosDesignsFacts(hostvars=host_hostvars, templar=None)}

        # Add reference to EosDesignsFacts object inside hostvars.
        # This is used to allow templates to access the facts object directly with "switch.*"
        host_hostvars["switch"] = avd_switch_facts[host]["switch"]

    return avd_switch_facts


def render_avd_switch_facts(avd_switch_facts_instances: dict):
    """
    Run the render method on each EosDesignsFacts object

    Parameters
    ----------
    avd_switch_facts_instances : dict of EosDesignsFacts

    Returns
    -------
    dict
        hostname1 : dict
            switch : < switch.* facts >
        hostname2 : dict
            switch : < switch.* facts >
    """
    return {host: {"switch": avd_switch_facts_instances[host]["switch"].render()} for host in avd_switch_facts_instances}


def render_peers(avd_switch_facts: dict) -> tuple[dict, dict]:
    """
    Build dicts of underlay and overlay peerings based on avd_switch_facts

    Parameters
    ----------
    avd_switch_facts : dict
        hostname1 : dict
            switch : < switch.* facts >
        hostname2 : dict
            switch : < switch.* facts >

    Returns
    -------
    avd_overlay_peers: dict
        hostname1 : list[str]
            List of switches pointing to hostname1 as route server / route reflector
        hostname2 : list[str]
            List of switches pointing to hostname2 as route server / route reflector
    avd_topology_peers: dict
        hostname1 : list[str]
            List of switches having hostname1 as uplink_switch
        hostname2 : list[str]
            List of switches having hostname2 as uplink_switch

    """

    avd_overlay_peers = {}
    avd_topology_peers = {}
    for host in avd_switch_facts:
        host_evpn_route_servers = avd_switch_facts[host]["switch"].get("evpn_route_servers", [])
        for peer in host_evpn_route_servers:
            avd_overlay_peers.setdefault(peer, []).append(host)

        host_mpls_route_reflectors = avd_switch_facts[host]["switch"].get("mpls_route_reflectors", [])
        for peer in host_mpls_route_reflectors:
            avd_overlay_peers.setdefault(peer, []).append(host)

        host_topology_peers = avd_switch_facts[host]["switch"].get("uplink_peers", [])

        for peer in host_topology_peers:
            avd_topology_peers.setdefault(peer, []).append(host)

    return avd_overlay_peers, avd_topology_peers


def run_eos_designs_facts(common_varfiles: list[str], device_varfiles: str, facts_file: str, verbosity: int) -> None:
    """
    Read variables from files and run eos_designs_facts.

    Intended for CLI use via runner.py

    Parameters
    ----------
    common_varfiles : list[str]
        List of common var files to import and merge.
    device_varfiles : str
        Glob for device specific var files to import and merge on top of common vars.
        Filenames will be used as hostnames.
    facts_file: str
        Path to output facts file
    verbosity: int
        Vebosity level for output. Passed along to other functions
    """

    # Read common vars
    common_vars = {}

    for file in common_varfiles:
        common_vars.update(read_vars(file))

    all_hostvars = {}
    for device_var_file in glob.iglob(device_varfiles):
        device_vars = common_vars.copy()
        device_vars.update(read_vars(device_var_file))
        hostname = str(path.basename(device_var_file)).removesuffix(".yaml").removesuffix(".yml").removesuffix(".json")

        all_hostvars[hostname] = device_vars

    hostnames = list(all_hostvars.keys())
    for hostname, device_vars in all_hostvars.items():
        # Insert ansible vars our code relies on today
        device_vars["inventory_hostname"] = hostname
        fabric_name = device_vars.get("fabric_name", "all")
        device_vars["groups"] = {fabric_name: hostnames}

    facts = eos_designs_facts(all_hostvars, verbosity)

    # Insert ansible vars our code relies on today
    facts["groups"] = {fabric_name: hostnames}

    if facts_file:
        write_result(facts_file, yaml_dump(facts, sort_keys=False))

    print("OK eos_designs_facts")
