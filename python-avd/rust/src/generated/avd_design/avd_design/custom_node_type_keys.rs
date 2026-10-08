// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar key("key", 0) -> &'a str;
        scalar field_type("type", 1) -> &'a str;
        scalar connected_endpoints("connected_endpoints", 2) -> bool;
        scalar default_evpn_role("default_evpn_role", 3) -> &'a str;
        scalar default_ptp_priority1("default_ptp_priority1", 4) -> i64;
        scalar default_underlay_routing_protocol("default_underlay_routing_protocol", 5) -> &'a str;
        scalar default_overlay_routing_protocol("default_overlay_routing_protocol", 6) -> &'a str;
        scalar default_mpls_overlay_role("default_mpls_overlay_role", 7) -> &'a str;
        model default_overlay_address_families("default_overlay_address_families", 8) -> item::DefaultOverlayAddressFamilies<'a>;
        scalar default_evpn_encapsulation("default_evpn_encapsulation", 9) -> &'a str;
        scalar default_wan_role("default_wan_role", 10) -> &'a str;
        scalar default_flow_tracker_type("default_flow_tracker_type", 11) -> &'a str;
        scalar mlag_support("mlag_support", 12) -> bool;
        model network_services("network_services", 13) -> item::NetworkServices<'a>;
        scalar underlay_router("underlay_router", 14) -> bool;
        scalar uplink_type("uplink_type", 15) -> &'a str;
        scalar vtep("vtep", 16) -> bool;
        scalar mpls_lsr("mpls_lsr", 17) -> bool;
        model ip_addressing("ip_addressing", 18) -> item::IpAddressing<'a>;
        model interface_descriptions("interface_descriptions", 19) -> item::InterfaceDescriptions<'a>;
        scalar cv_tags_topology_type("cv_tags_topology_type", 20) -> &'a str;
    }
}

pub mod item {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct DefaultOverlayAddressFamilies {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct NetworkServices {
            scalar l1("l1", 0) -> bool;
            scalar l2("l2", 1) -> bool;
            scalar l3("l3", 2) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct IpAddressing {
            scalar python_module("python_module", 0) -> &'a str;
            scalar python_class_name("python_class_name", 1) -> &'a str;
            scalar router_id("router_id", 2) -> &'a str;
            scalar router_id_ipv6("router_id_ipv6", 3) -> &'a str;
            scalar mlag_ip_primary("mlag_ip_primary", 4) -> &'a str;
            scalar mlag_ip_secondary("mlag_ip_secondary", 5) -> &'a str;
            scalar mlag_l3_ip_primary("mlag_l3_ip_primary", 6) -> &'a str;
            scalar mlag_l3_ip_secondary("mlag_l3_ip_secondary", 7) -> &'a str;
            scalar mlag_ibgp_peering_ip_primary("mlag_ibgp_peering_ip_primary", 8) -> &'a str;
            scalar mlag_ibgp_peering_ip_secondary("mlag_ibgp_peering_ip_secondary", 9) -> &'a str;
            scalar p2p_uplinks_ip("p2p_uplinks_ip", 10) -> &'a str;
            scalar p2p_uplinks_peer_ip("p2p_uplinks_peer_ip", 11) -> &'a str;
            scalar vtep_ip_mlag("vtep_ip_mlag", 12) -> &'a str;
            scalar vtep_ip("vtep_ip", 13) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct InterfaceDescriptions {
            scalar python_module("python_module", 0) -> &'a str;
            scalar python_class_name("python_class_name", 1) -> &'a str;
            scalar underlay_ethernet_interfaces("underlay_ethernet_interfaces", 2) -> &'a str;
            scalar underlay_port_channel_interfaces("underlay_port_channel_interfaces", 3) -> &'a str;
            scalar mlag_ethernet_interfaces("mlag_ethernet_interfaces", 4) -> &'a str;
            scalar mlag_port_channel_interfaces("mlag_port_channel_interfaces", 5) -> &'a str;
            scalar connected_endpoints_ethernet_interfaces("connected_endpoints_ethernet_interfaces", 6) -> &'a str;
            scalar connected_endpoints_port_channel_interfaces("connected_endpoints_port_channel_interfaces", 7) -> &'a str;
            scalar router_id_loopback_interface("router_id_loopback_interface", 8) -> &'a str;
            scalar vtep_loopback_interface("vtep_loopback_interface", 9) -> &'a str;
        }
    }
}
