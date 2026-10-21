// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(indexed_list, primary_key(key), python(name = "AVDDesign.NodeTypeKeys", schema = List(757), item_name = "AVDDesign.NodeTypeKeysItem"))]
pub struct NodeTypeKeys<'a, Mode> (::validated_data::Field<Item<'a, Mode>>);

#[::validated_data::data_view(python(name = "AVDDesign.NodeTypeKeysItemBase", schema = Dict(1710)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(3))]
    pub key: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1575), rename = "type")]
    pub field_type: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(600))]
    pub connected_endpoints: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1576))]
    pub default_evpn_role: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(618))]
    pub default_ptp_priority1: ::validated_data::Field<i64>,
    #[data_view(schema = Str(1577))]
    pub default_underlay_routing_protocol: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1578))]
    pub default_overlay_routing_protocol: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1579))]
    pub default_mpls_overlay_role: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(755))]
    pub default_overlay_address_families: ::validated_data::Field<item::DefaultOverlayAddressFamilies<'a, Mode>>,
    #[data_view(schema = Str(1581))]
    pub default_evpn_encapsulation: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1582))]
    pub default_wan_role: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1583))]
    pub default_flow_tracker_type: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(601))]
    pub mlag_support: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1707))]
    pub network_services: ::validated_data::Field<item::NetworkServices<'a, Mode>>,
    #[data_view(schema = Bool(605))]
    pub underlay_router: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1584))]
    pub uplink_type: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(606))]
    pub vtep: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(607))]
    pub mpls_lsr: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1708))]
    pub ip_addressing: ::validated_data::Field<item::IpAddressing<'a, Mode>>,
    #[data_view(schema = Dict(1709))]
    pub interface_descriptions: ::validated_data::Field<item::InterfaceDescriptions<'a, Mode>>,
    #[data_view(schema = Str(1591))]
    pub cv_tags_topology_type: ::validated_data::Field<&'a str>,
}

pub mod item {

    #[::validated_data::data_view(list, python(name = "AVDDesign.NodeTypeKeysItemBase.DefaultOverlayAddressFamilies", schema = List(755)))]
    pub struct DefaultOverlayAddressFamilies<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(python(name = "AVDDesign.NodeTypeKeysItemBase.NetworkServices", schema = Dict(1707)))]
    pub struct NetworkServices<'a, Mode> {
        #[data_view(schema = Bool(602))]
        pub l1: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(603))]
        pub l2: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(604))]
        pub l3: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.NodeTypeKeysItemBase.IpAddressing", schema = Dict(1708)))]
    pub struct IpAddressing<'a, Mode> {
        #[data_view(schema = Str(1585))]
        pub python_module: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1586))]
        pub python_class_name: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub router_id: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub router_id_ipv6: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub mlag_ip_primary: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub mlag_ip_secondary: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub mlag_l3_ip_primary: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub mlag_l3_ip_secondary: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub mlag_ibgp_peering_ip_primary: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub mlag_ibgp_peering_ip_secondary: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub p2p_uplinks_ip: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub p2p_uplinks_peer_ip: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub vtep_ip_mlag: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub vtep_ip: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.NodeTypeKeysItemBase.InterfaceDescriptions", schema = Dict(1709)))]
    pub struct InterfaceDescriptions<'a, Mode> {
        #[data_view(schema = Str(1588))]
        pub python_module: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1589))]
        pub python_class_name: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub underlay_ethernet_interfaces: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub underlay_port_channel_interfaces: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub mlag_ethernet_interfaces: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub mlag_port_channel_interfaces: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub connected_endpoints_ethernet_interfaces: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub connected_endpoints_port_channel_interfaces: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub router_id_loopback_interface: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1587))]
        pub vtep_loopback_interface: ::validated_data::Field<&'a str>,
    }
}
