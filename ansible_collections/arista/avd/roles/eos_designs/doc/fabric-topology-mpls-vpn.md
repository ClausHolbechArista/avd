# Fabric Topology Variables for mpls-vpn Design

The fabric topology variables define the connectivity between the various node types, as well as override the default switch properties.

<!-- <div style="text-align:center">
  <img src="../../../../media/5-stage-topology.gif" />
</div>

As per the diagram above, the topology hierarchy is the following:

fabric_name > dc_name > pod_name -->

- Fabric Name, required to match Ansible Group name covering all devices in the Fabric | Required and **must** be an inventory group name.

```yaml
fabric_name: < Fabric_Name >
```

<!-- - DC Name, required to match Ansible Group name covering all devices in the DC | Required for 5-stage CLOS (Super-spines)

```yaml
dc_name: < DC_Name >
```

- POD Name, only used in Fabric Documentation | Optional, fallback to dc_name and then to fabric_name. Recommended to be common between Spines, Leafs within a POD (One l3ls topology)

```yaml
pod_name: < POD_Name >
``` -->

- Connectivity is defined in a free-standing backbone_interfaces construct.
- A static unique identifier (id) is assigned to each device.
  - This is leveraged to derive the IP address assignment from each summary defined in the Fabric Underlay and Overlay Topology Variables.
- Within the pe, p and rr dictionary variables, defaults can be defined.
  - This reduces user input requirements, limiting errors.
  - The default variables can be overridden when defined under the node groups.

## Supported designs

`eos_designs` with the mpls-vpn design type supports any arbitrary physical mesh topology by combining and interconnecting different node types with the backbone_interfaces dictionary. You can also extend `eos_designs` to support your own topology by using [`node_type_keys`](node-types.html) to create your own node type

### Arbitrary Mesh or L3LS Topology

- The **eos_designs** role with the mpls-vpn design type supports any type of topology consisting of any combination of pe-routers, p-routers and rr-routers.
- Any node group of 2 or more rr-routers will form a Route Reflector cluster. The backbone_interfaces construct is used to define underlay interfaces and associated interface profiles.

## Node Type Variables

The following table provide information on the default node types that have been pre-defined in [`eos_designs/defaults/main/defaults-node-type-keys.yml`](https://github.com/aristanetworks/ansible-avd/tree/devel/ansible_collections/arista/avd/roles/eos_designs/defaults). To customize or create new node types, please refer to [node types definition](node-types.md)

| Node Type Key | Underlay Router | Uplink Type | Default Overlay Role | L2 Network Services | L3 Network Services | VTEP | Connected Endpoints |
| --------------| --------------- | ----------- | -------------------- | ------------------- | ------------------- | ---- | ------------------- |
| p             | ✅              | p2p          | none                | ✘                   | ✘                   | ✘     | ✘                  |
| rr            | ✅              | p2p          | server              | ✘                   | ✘                   | ✘     | ✘                  |
| pe            | ✅              | p2p          | client              | ✅                  | ✅                   | ✅    | ✅                  |

The variables should be applied to all devices in the fabric.

- The `type:` variable needs to be defined for each device in the fabric.
- This is leveraged to load the appropriate templates to generate the configuration.

### Variables and Options

As explained above, you can defined your own types of devices. CLI only provides default node types.

```yaml
# define the layer type
type: < p | pe | rr >
```

### Example

```yaml
# Defined in PE.yml file
# Can also be set directly in your inventory file under spine group
type: pe

# Defined in P.yml
# Can also be set directly in your inventory file under l3leaf group
type: p

# Defined in RR.yml
# Can also be set directly in your inventory file under l2leaf group
type: rr
```

All node types have the same structure based on `defaults`, `node_group`, `node` and all variables can be defined in any section and support inheritance like this:

Under `node_type_key:`

```bash
defaults <- node_group <- node_group.node <- node
```

## Node type structure

```yaml
---
<node_type_key>:
  defaults:
    # Define vars for all nodes of this type
  node_groups:
    <node group name>:
    # Vars related to all nodes part of this group
      nodes:
        <node inventory hostname>:
          # Vars defined per node
  nodes:
    <node inventory hostname>:
      # Vars defined per node

      # Unique identifier | Required.
      id: < integer >

      # Node management IP address | Optional.
      mgmt_ip: < IPv4_address/Mask >
```

## Node Variables details

### Generic configuration management

```yaml
< node_type_key >:

  defaults:
    # Arista platform family | Required.
    platform: < Arista Platform Family >

    # Rack that the switch is located in (only used in snmp_settings location) | Optional
    rack: < rack_name >

    # EOS CLI rendered directly on the root level of the final EOS configuration | Optional
    raw_eos_cli: |
      < multiline eos cli >

    # Custom structured config for eos_cli_config_gen | Optional
    structured_config: < dictionary >
```

### Uplink management

Unlike with the l3ls-evpn design type, underlay p2p links are built using the backbone_interfaces dictionary:

```yaml
backbone_interfaces:
  p2p_links_ip_pools:
    < pool name >: < IPv4_address/Mask >
  p2p_links_profiles:
    < backbone profile name >:
      speed: < speed >
      mtu: < mtu >
      isis_hello_padding: < true | false >
      isis_metric: < metric >
      ip_pool: < pool name >
      isis_circuit_type: < isis circuit type >
      ipv6_enable: < true | false >
  p2p_links:
    - id: < Link ID, used for selecting a subnet from the provided pool >
      nodes: ['< node1 inventory_hostname >', '< node2 inventory_hostname >']
      interfaces: ['< node1 interface >', '< node2 interface >']
      profile: < backbone profile name >
    - id: < Link ID >
      ...
```

#### ISIS underlay protocol management

```yaml
< node_type_key >:

  defaults:
    # isis system-id prefix (4.4 hexadecimal)
    isis_system_id_prefix: < hhhh.hhhh >

    # Number of path to configure in ECMP for ISIS
    isis_maximum_paths: < integer >

    # Base value for ISIS-SR Node-SID
    node_sid_base: 100

    # Node is-type as configured under the router isis instance.
    is_type: level-2
```

### Loopback and ISIS-SR Node-SID management

```yaml
< node_type_key >:

  defaults:
    # IPv4 subnet for Loopback0 allocation
    loopback_ipv4_pool: < IPv4_address/Mask >

    # Offset all assigned loopback IP addresses.
    loopback_ipv4_offset: 2

    # IPv6 subnet for Loopback0 allocation
    loopback_ipv6_pool: < IPv6_address/Mask >

    # Offset all assigned loopback IP addresses.
    loopback_ipv6_offset: 2
```

### BGP & Overlay Control plane

```yaml
< node_type_key >:

  defaults:

    # List of EOS command to apply to BGP daemon | Optional
    bgp_defaults: [ < List of EOS commands> ]

    # Acting role in overlay control plane.
    # Override role definition from node_type_keys
    # Can be set per node
    mpls_overlay_role: < client | server | none | Default -> refer to node type variable table >

    # List of inventory hostname acting as MPLS route-reflectors.
    mpls_route_reflectors: [ '< inventory_hostname_of_mpls_route_reflectors >' ]
```

### Overlay services management

```yaml
< node_type_key >:

  defaults:
    # Possibility to prevent configuration of Tenant VRFs and SVIs
    # Override node definition "network_services_l3" from node_type_keys
    # This allows support for centralized routing.
    overlay_services_l2_only: < false | true >

    # Filter L1, L2 and L3 network services based on tenant and tags (and operation filter) | Optional
    # If filter is not defined will default to all
    filter:
      tenants: [ < tenant_1 >, < tenant_2 > | default all ]
      tags: [ < tag_1 >, < tag_2 > | default -> all ]

      # Force VRFs in a tenant to be configured even if VLANs are not included in tags | Optional
      # Useful for "border" leaf.
      always_include_vrfs_in_tenants: [ < tenant_1 >, < tenant_2 >, "all" ]

    # Activate or deactivate IGMP snooping | Optional, default is true
    igmp_snooping_enabled: < true | false >
```

### PE configuration management

```yaml
< node_type_key >:

  defaults:
    # Spanning tree mode | Required.
    spanning_tree_mode: < mstp | rstp | rapid-pvst | none >

    # Spanning tree priority.
    spanning_tree_priority: < spanning-tree priority -> default 32768 >

    # Spanning tree priority.
    spanning_tree_root_super: < true | false  >

    # Virtual router mac address for anycast gateway | Required.
    virtual_router_mac_address: < mac address >
```
