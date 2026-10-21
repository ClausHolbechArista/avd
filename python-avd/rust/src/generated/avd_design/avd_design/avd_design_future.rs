// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.AvdDesignFuture", schema = Dict(1388)))]
pub struct AvdDesignFuture<'a, Mode> {
    #[data_view(schema = Bool(402))]
    pub accept_dhcp_default_route_for_mgmt_ip_dhcp: ::data_views::Field<bool>,
    #[data_view(schema = Bool(403))]
    pub accept_ra_default_route_for_ipv6_mgmt_ip_auto_config: ::data_views::Field<bool>,
    #[data_view(schema = Bool(404))]
    pub accept_dhcp_default_route_for_inband_mgmt_ip_dhcp: ::data_views::Field<bool>,
    #[data_view(schema = Bool(405))]
    pub allow_recursive_profile_inheritance: ::data_views::Field<bool>,
    #[data_view(schema = Bool(406))]
    pub configure_inband_mgmt_ipv6_vrf: ::data_views::Field<bool>,
    #[data_view(schema = Bool(407))]
    pub consistent_uplink_vlans: ::data_views::Field<bool>,
    #[data_view(schema = Bool(408))]
    pub fix_address_locking_dhcp_server_interfaces: ::data_views::Field<bool>,
    #[data_view(schema = Bool(409))]
    pub fix_match_ipv6_prefix_list_on_mlag_route_map: ::data_views::Field<bool>,
    #[data_view(schema = Bool(410))]
    pub fix_mlag_ibgp_peering_ipv6_pool: ::data_views::Field<bool>,
    #[data_view(schema = Bool(411))]
    pub fix_radius_server_group_tls: ::data_views::Field<bool>,
    #[data_view(schema = Bool(412))]
    pub only_configure_ipv6_inband_mgmt_prefix_list_when_used: ::data_views::Field<bool>,
    #[data_view(schema = Bool(413))]
    pub only_configure_mlag_vrfs_peer_group_when_used: ::data_views::Field<bool>,
    #[data_view(schema = Bool(414))]
    pub only_configure_pvst_border_when_mode_is_mstp: ::data_views::Field<bool>,
    #[data_view(schema = Bool(415))]
    pub only_configure_route_map_connected_to_bgp_vrfs_when_used: ::data_views::Field<bool>,
    #[data_view(schema = Bool(416))]
    pub raise_for_port_channels_without_members: ::data_views::Field<bool>,
    #[data_view(schema = Bool(417))]
    pub raise_for_underlay_router_with_uplink_type_port_channel: ::data_views::Field<bool>,
    #[data_view(schema = Bool(418))]
    pub remove_redundant_ipv4_unicast_for_peer_groups: ::data_views::Field<bool>,
}
