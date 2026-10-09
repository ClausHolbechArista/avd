// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub key: ::validated_data::Field<&'a str>,
    #[data_view(rename = "type")]
    pub field_type: ::validated_data::Field<&'a str>,
    pub connected_endpoints: ::validated_data::Field<bool>,
    pub default_evpn_role: ::validated_data::Field<&'a str>,
    pub default_ptp_priority1: ::validated_data::Field<i64>,
    pub default_underlay_routing_protocol: ::validated_data::Field<&'a str>,
    pub default_overlay_routing_protocol: ::validated_data::Field<&'a str>,
    pub default_mpls_overlay_role: ::validated_data::Field<&'a str>,
    pub default_overlay_address_families: ::validated_data::Field<item::DefaultOverlayAddressFamilies<'a, Mode>>,
    pub default_evpn_encapsulation: ::validated_data::Field<&'a str>,
    pub default_wan_role: ::validated_data::Field<&'a str>,
    pub default_flow_tracker_type: ::validated_data::Field<&'a str>,
    pub mlag_support: ::validated_data::Field<bool>,
    pub network_services: ::validated_data::Field<item::NetworkServices<'a, Mode>>,
    pub underlay_router: ::validated_data::Field<bool>,
    pub uplink_type: ::validated_data::Field<&'a str>,
    pub vtep: ::validated_data::Field<bool>,
    pub mpls_lsr: ::validated_data::Field<bool>,
    pub ip_addressing: ::validated_data::Field<item::IpAddressing<'a, Mode>>,
    pub interface_descriptions: ::validated_data::Field<item::InterfaceDescriptions<'a, Mode>>,
    pub cv_tags_topology_type: ::validated_data::Field<&'a str>,
}

pub mod item {

    #[::validated_data::data_view(list)]
    pub struct DefaultOverlayAddressFamilies<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view]
    pub struct NetworkServices<'a, Mode> {
        pub l1: ::validated_data::Field<bool>,
        pub l2: ::validated_data::Field<bool>,
        pub l3: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view]
    pub struct IpAddressing<'a, Mode> {
        pub python_module: ::validated_data::Field<&'a str>,
        pub python_class_name: ::validated_data::Field<&'a str>,
        pub router_id: ::validated_data::Field<&'a str>,
        pub router_id_ipv6: ::validated_data::Field<&'a str>,
        pub mlag_ip_primary: ::validated_data::Field<&'a str>,
        pub mlag_ip_secondary: ::validated_data::Field<&'a str>,
        pub mlag_l3_ip_primary: ::validated_data::Field<&'a str>,
        pub mlag_l3_ip_secondary: ::validated_data::Field<&'a str>,
        pub mlag_ibgp_peering_ip_primary: ::validated_data::Field<&'a str>,
        pub mlag_ibgp_peering_ip_secondary: ::validated_data::Field<&'a str>,
        pub p2p_uplinks_ip: ::validated_data::Field<&'a str>,
        pub p2p_uplinks_peer_ip: ::validated_data::Field<&'a str>,
        pub vtep_ip_mlag: ::validated_data::Field<&'a str>,
        pub vtep_ip: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct InterfaceDescriptions<'a, Mode> {
        pub python_module: ::validated_data::Field<&'a str>,
        pub python_class_name: ::validated_data::Field<&'a str>,
        pub underlay_ethernet_interfaces: ::validated_data::Field<&'a str>,
        pub underlay_port_channel_interfaces: ::validated_data::Field<&'a str>,
        pub mlag_ethernet_interfaces: ::validated_data::Field<&'a str>,
        pub mlag_port_channel_interfaces: ::validated_data::Field<&'a str>,
        pub connected_endpoints_ethernet_interfaces: ::validated_data::Field<&'a str>,
        pub connected_endpoints_port_channel_interfaces: ::validated_data::Field<&'a str>,
        pub router_id_loopback_interface: ::validated_data::Field<&'a str>,
        pub vtep_loopback_interface: ::validated_data::Field<&'a str>,
    }
}
