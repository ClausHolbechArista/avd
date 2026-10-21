// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(key), python(name = "AVDDesign.CustomNodeTypeKeys", schema = List(756), item_name = "AVDDesign.CustomNodeTypeKeysItem"))]
pub struct CustomNodeTypeKeys<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.CustomNodeTypeKeysItemBase", schema = Dict(1710)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(3))]
    pub key: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1575), rename = "type")]
    pub field_type: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(600))]
    pub connected_endpoints: ::data_views::Field<bool>,
    #[data_view(schema = Str(1576))]
    pub default_evpn_role: ::data_views::Field<&'a str>,
    #[data_view(schema = Int(618))]
    pub default_ptp_priority1: ::data_views::Field<i64>,
    #[data_view(schema = Str(1577))]
    pub default_underlay_routing_protocol: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1578))]
    pub default_overlay_routing_protocol: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1579))]
    pub default_mpls_overlay_role: ::data_views::Field<&'a str>,
    #[data_view(schema = List(755))]
    pub default_overlay_address_families: ::data_views::Field<item::DefaultOverlayAddressFamilies<'a, Mode>>,
    #[data_view(schema = Str(1581))]
    pub default_evpn_encapsulation: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1582))]
    pub default_wan_role: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1583))]
    pub default_flow_tracker_type: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(601))]
    pub mlag_support: ::data_views::Field<bool>,
    #[data_view(schema = Dict(1707))]
    pub network_services: ::data_views::Field<item::NetworkServices<'a, Mode>>,
    #[data_view(schema = Bool(605))]
    pub underlay_router: ::data_views::Field<bool>,
    #[data_view(schema = Str(1584))]
    pub uplink_type: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(606))]
    pub vtep: ::data_views::Field<bool>,
    #[data_view(schema = Bool(607))]
    pub mpls_lsr: ::data_views::Field<bool>,
    #[data_view(schema = Dict(1708))]
    pub ip_addressing: ::data_views::Field<item::IpAddressing<'a, Mode>>,
    #[data_view(schema = Dict(1709))]
    pub interface_descriptions: ::data_views::Field<item::InterfaceDescriptions<'a, Mode>>,
    #[data_view(schema = Str(1591))]
    pub cv_tags_topology_type: ::data_views::Field<&'a str>,
}

pub mod item {

    #[::data_views::data_view(list, python(name = "AVDDesign.CustomNodeTypeKeysItemBase.DefaultOverlayAddressFamilies", schema = List(755)))]
    pub struct DefaultOverlayAddressFamilies<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(python(name = "AVDDesign.CustomNodeTypeKeysItemBase.NetworkServices", schema = Dict(1707)))]
    pub struct NetworkServices<'a, Mode> {
        #[data_view(schema = Bool(602))]
        pub l1: ::data_views::Field<bool>,
        #[data_view(schema = Bool(603))]
        pub l2: ::data_views::Field<bool>,
        #[data_view(schema = Bool(604))]
        pub l3: ::data_views::Field<bool>,
    }

    #[::data_views::data_view(python(name = "AVDDesign.CustomNodeTypeKeysItemBase.IpAddressing", schema = Dict(1708)))]
    pub struct IpAddressing<'a, Mode> {
        #[data_view(schema = Str(1585))]
        pub python_module: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1586))]
        pub python_class_name: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub router_id: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub router_id_ipv6: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub mlag_ip_primary: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub mlag_ip_secondary: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub mlag_l3_ip_primary: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub mlag_l3_ip_secondary: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub mlag_ibgp_peering_ip_primary: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub mlag_ibgp_peering_ip_secondary: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub p2p_uplinks_ip: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub p2p_uplinks_peer_ip: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub vtep_ip_mlag: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub vtep_ip: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view(python(name = "AVDDesign.CustomNodeTypeKeysItemBase.InterfaceDescriptions", schema = Dict(1709)))]
    pub struct InterfaceDescriptions<'a, Mode> {
        #[data_view(schema = Str(1588))]
        pub python_module: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1589))]
        pub python_class_name: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub underlay_ethernet_interfaces: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub underlay_port_channel_interfaces: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub mlag_ethernet_interfaces: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub mlag_port_channel_interfaces: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub connected_endpoints_ethernet_interfaces: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub connected_endpoints_port_channel_interfaces: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub router_id_loopback_interface: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub vtep_loopback_interface: ::data_views::Field<&'a str>,
    }
}
