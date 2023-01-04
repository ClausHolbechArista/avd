from pydantic import BaseModel


class RootSwitchIpvpn_gateway(BaseModel):
    evpn_domain_id: str = "0:1"
    ipvpn_domain_id: str = "0:2"
    max_routes: int = 0
    local_as: str = None
    remote_peers: list[str] = []


class RootSwitchOverlay(BaseModel):
    peering_address: str = None
    ler: bool = None
    vtep: bool = None
    evpn: bool = None
    evpn_vxlan: bool = None
    evpn_mpls: bool = None
    vpn_ipv4: bool = None
    vpn_ipv6: bool = None
    ipvpn_gateway: bool = None
    dpath: bool = None


class RootSwitchUnderlay(BaseModel):
    bgp: bool = None
    ldp: bool = None
    mpls: bool = None
    sr: bool = None
    ospf: bool = None
    isis: bool = None


class RootSwitchMlag_switch_ids(BaseModel):
    primary: int = None
    secondary: int = None


class RootSwitchTrunk_groupsUplink(BaseModel):
    name: str = "UPLINK"


class RootSwitchTrunk_groupsMlag_l3(BaseModel):
    name: str = "LEAF_PEER_L3"


class RootSwitchTrunk_groupsMlag(BaseModel):
    name: str = "MLAG"


class RootSwitchTrunk_groups(BaseModel):
    mlag: RootSwitchTrunk_groupsMlag = None
    mlag_l3: RootSwitchTrunk_groupsMlag_l3 = None
    uplink: RootSwitchTrunk_groupsUplink = None


class RootSwitchLacp_port_id(BaseModel):
    begin: int = None
    end: int = None


class RootSwitchUplink_ptp(BaseModel):
    enable: bool = None


class RootSwitch(BaseModel):
    type: str = None
    hostname: str = None
    node_type_key: str = None
    connected_endpoints: bool = False
    default_downlink_interfaces: list[str] = []
    default_evpn_role: str = "none"
    default_interfaces: dict = {}
    default_underlay_routing_protocol: str = "ebgp"
    default_overlay_routing_protocol: str = "ebgp"
    default_overlay_address_families: list[str] = ["evpn"]
    default_mpls_overlay_role: str = "none"
    mpls_lsr: bool = False
    mlag_support: bool = False
    network_services_l1: bool = False
    network_services_l2: bool = False
    network_services_l3: bool = False
    underlay_router: bool = True
    uplink_type: str = "p2p"
    vtep: bool = False
    ip_addressing: dict = {}
    interface_descriptions: dict = {}
    group: str = None
    id: int = None
    mgmt_ip: str = None
    platform: str = None
    max_parallel_uplinks: int = 1
    uplink_switches: list[str] = None
    uplink_interfaces: list[str] = []
    uplink_switch_interfaces: list[str] = None
    uplink_interface_speed: str = None
    uplink_bfd: bool = None
    uplink_ptp: RootSwitchUplink_ptp = None
    default_ptp_priority1: int = 127
    ptp: dict = None
    uplink_macsec: bool = None
    uplink_structured_config: dict = None
    short_esi: str = None
    rack: str = None
    raw_eos_cli: str = None
    struct_cfg: dict = None
    max_uplink_switches: int = None
    is_deployed: bool = True
    platform_settings: dict = {}
    mgmt_interface: str = None
    system_mac_address: str = None
    underlay_routing_protocol: str = None
    overlay_routing_protocol: str = None
    overlay_address_families: list[str] = []
    link_tracking_groups: list[dict] = None
    lacp_port_id: RootSwitchLacp_port_id = None
    filter_tenants: list[str] = ["all"]
    always_include_vrfs_in_tenants: list[str] = None
    filter_tags: list[str] = ["all"]
    filter_only_vlans_in_use: bool = False
    virtual_router_mac_address: str = None
    trunk_groups: RootSwitchTrunk_groups = None
    enable_trunk_groups: bool = None
    only_local_vlan_trunk_groups: bool = None
    endpoint_trunk_groups: list = []
    vlans: str = ""
    spanning_tree_mode: str = "none"
    spanning_tree_priority: int = None
    spanning_tree_root_super: bool = None
    underlay_multicast: bool = None
    overlay_rd_type_admin_subfield: str = None
    evpn_multicast: bool = None
    multi_vtep: bool = None
    igmp_snooping_enabled: bool = None
    loopback_ipv4_pool: str = None
    loopback_ipv4_offset: int = None
    uplink_ipv4_pool: str = None
    router_id: str = None
    evpn_gateway_vxlan_l2: bool = None
    evpn_gateway_vxlan_l3: bool = None
    evpn_gateway_vxlan_l3_inter_domain: bool = None
    evpn_gateway_remote_peers: list[dict] = None
    bgp_defaults: list[str] = None
    bgp_cluster_id: str = None
    bgp_peer_groups: dict = None
    evpn_role: str = None
    mpls_overlay_role: str = None
    bgp_as: str = None
    evpn_route_servers: list[str] = []
    mpls_route_reflectors: list[str] = None
    isis_net: str = None
    is_type: str = None
    isis_instance_name: str = None
    node_sid: str = None
    underlay_ipv6: bool = None
    loopback_ipv6_pool: str = None
    loopback_ipv6_offset: int = None
    ipv6_router_id: str = None
    mlag: bool = None
    mlag_ibgp_origin_incomplete: bool = None
    mlag_peer_vlan: int = None
    mlag_peer_link_allowed_vlans: str = None
    mlag_dual_primary_detection: bool = None
    mlag_interfaces: list[str] = None
    mlag_interfaces_speed: str = None
    mlag_peer_ipv4_pool: str = None
    mlag_peer_l3_ipv4_pool: str = None
    mlag_port_channel_structured_config: dict = None
    mlag_peer_vlan_structured_config: dict = None
    mlag_peer_l3_vlan_structured_config: dict = None
    mlag_role: str = None
    mlag_peer: str = None
    mlag_l3: bool = None
    mlag_peer_l3_vlan: int = None
    mlag_port_channel_id: int = None
    vtep_loopback_ipv4_pool: str = None
    vtep_loopback: str = None
    inband_management_subnet: str = None
    inband_management_role: str = None
    inband_management_parents: list[str] = None
    inband_management_vlan: int = None
    inband_management_ip: str = None
    inband_management_gateway: str = None
    inband_management_interface: str = None
    uplinks: list[str] = []
    uplink_peers: list[str] = []
    mlag_switch_ids: RootSwitchMlag_switch_ids = None
    vtep_ip: str = None
    mlag_ip: str = None
    mlag_peer_ip: str = None
    mlag_l3_ip: str = None
    mlag_peer_l3_ip: str = None
    mlag_peer_mgmt_ip: str = None
    overlay_routing_protocol_address_family: str = "ipv4"
    bgp: bool = None
    evpn_encapsulation: str = "vxlan"
    underlay: RootSwitchUnderlay = None
    overlay: RootSwitchOverlay = None
    ipvpn_gateway: RootSwitchIpvpn_gateway = None


class Root(BaseModel):
    switch: RootSwitch = None
