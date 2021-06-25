# Fabric Variables for mpls-vpn Design

- The fabric underlay and overlay topology variables, define the elements related to build the L3 Leaf and Spine fabric.
- The following underlay routing protocols are supported:
  - ISIS-SR.
  - ISIS + LDP.
  - ISIS-SR + LDP.
  - OSPF + LDP.
- The following overlay routing protocols are supported:
  - IBGP (default)
- Only summary network addresses need to be defined. IP addresses are then assigned to each node, based on its unique device id.
  - To view IP address allocation and consumption, a summary is provided in the auto-generated fabric documentation in Markdown and CSV format.
- The variables should be applied to all devices in the fabric.

**Variables and Options:**

```yaml
# Underlay routing protocol | Required.
underlay_routing_protocol: < isis-sr, isis-ldp, isis-sr-ldp, ospf-ldp | Default -> isis-sr >
overlay_routing_protocol: < ibgp >

# Underlay OSFP | Required when < underlay_routing_protocol > == OSPF
underlay_ospf_process_id: < process_id | Default -> 100 >
underlay_ospf_area: < ospf_area | Default -> 0.0.0.0 >
underlay_ospf_max_lsa: < lsa | Default -> 12000 >
underlay_ospf_bfd_enable: < true | false | Default -> false >

# Underlay ISIS | Required when < underlay_routing_protocol > == ISIS
isis_area_id: < isis area | Default -> "49.0001" >

# Underlay ISIS parameters
isis_default_is_type: < level-1 | level-2 | level-1-2 | Default -> level-1-2 >
isis_default_circuit_type: < level-1 | level-2 | level-1-2 | Default -> level-1-2 >
isis_default_metric: < int >
isis_advertise_passive_only: < true | false | Default -> false >

#  Underlay ISIS TI-LFA parameters
isis_ti_lfa:
  enabled: < true | false | Default -> false >
  protection: < link | node | Default -> link >
  # Microloop protection delay in ms
  local_convergence_delay: < int | Default -> 10000 >

# Underlay IPv6 turns on ipv6 for the underlay, which requires
underlay_ipv6: < true | false | Default -> false >

# AS number to use to configure overlay when < overlay_routing_protocol > == IBGP
bgp_as: < AS number >

# BGP multi-path | Optional
bgp_maximum_paths: < number_of_max_paths | Default -> 4 >
bgp_ecmp: < number_of_ecmp_paths | Default -> 4 >

# Whether to configure an iBGP full mesh between PEs, either because there is no RR used or other reasons.
bgp_mesh_pes: < true | false | Default -> false >

# BGP peer groups encrypted password
# MPLS_OVERLAY_PEERS | Required
# RR_OVERLAY_PEERS | Optional (Used to peer route reflectors in the same node-group (rr cluster) to each other)
# Leverage an Arista EOS switch to generate the encrypted password using the correct peer group name.
# Note that the name of the peer groups use '-' instead of '_' in EOS configuration.
bgp_peer_groups:
  MPLS_OVERLAY_PEERS:
    name: < name of peer group | default -> MPLS-OVERLAY-PEERS >
    password: "< encrypted password >"
  RR_OVERLAY_PEERS:
    name: < name of peer group | default -> RR-OVERLAY-PEERS >
    password: "< encrypted password >"

# Disable IGMP snooping at fabric level.
# If set, it overrides per vlan settings
default_igmp_snooping_enabled: < boolean | default -> true >

# BFD Multihop tuning | Required.
bfd_multihop:
  interval: < | default -> 300 >
  min_rx: < | default -> 300 >
  multiplier: < | default -> 3 >

## EVPN Host Flapping Settings
evpn_hostflap_detection:

  # If set to false it will disable EVPN host-flap detection
  enabled: < true | false | default -> true >

  # Minimum number of MAC moves that indicate a MAC duplication issue
  threshold: < number | default 5 >

  # Time (in seconds) to detect a MAC duplication issue
  window: < seconds | default 180 >

# Enable Route Target Membership Constraint Address Family on EVPN overlay BGP peerings (Min. EOS 4.25.1F)
# Requires use eBGP as overlay protocol.
evpn_overlay_bgp_rtc: < true | false , default -> false >

# Enable VPN import pruning (Min. EOS 4.24.2F)
# The Route Target extended communities carried by incoming VPN paths will
# be examined. If none of those Route Targets have been configured for import,
# the path will be immediately discarded
evpn_import_pruning: true

# Configure route-map on eBGP sessions towards route-servers, where prefixes with the peer's ASN in the AS Path are filtered away.
# This is very useful in very large scale networks, where convergence will be quicker by not having to return all updates received
# from Route-server-1 to Router-server-2 just for Route-server-2 to throw them away because of AS Path loop detection.
evpn_prevent_readvertise_to_server : < true | false , default -> false >

# Configure prefix for "short_esi" values | Optional
evpn_short_esi_prefix: < string, default -> "0000:0000:" >

```
