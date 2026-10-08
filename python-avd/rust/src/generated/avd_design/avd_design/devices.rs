// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar profile("profile", 0) -> &'a str;
        scalar field_type("type", 1) -> &'a str;
        scalar mlag_group("mlag_group", 2) -> &'a str;
        scalar name("name", 3) -> &'a str;
        model downlink_pools("downlink_pools", 4) -> item::DownlinkPools<'a>;
        scalar id("id", 5) -> i64;
        scalar platform("platform", 6) -> &'a str;
        scalar mac_address("mac_address", 7) -> &'a str;
        scalar system_mac_address("system_mac_address", 8) -> &'a str;
        scalar serial_number("serial_number", 9) -> &'a str;
        scalar rack("rack", 10) -> &'a str;
        scalar mgmt_ip("mgmt_ip", 11) -> &'a str;
        scalar mgmt_gateway("mgmt_gateway", 12) -> &'a str;
        scalar ipv6_mgmt_ip("ipv6_mgmt_ip", 13) -> &'a str;
        scalar ipv6_mgmt_gateway("ipv6_mgmt_gateway", 14) -> &'a str;
        scalar mgmt_interface("mgmt_interface", 15) -> &'a str;
        model link_tracking("link_tracking", 16) -> item::LinkTracking<'a>;
        model lacp_port_id_range("lacp_port_id_range", 17) -> item::LacpPortIdRange<'a>;
        scalar always_configure_ip_routing("always_configure_ip_routing", 18) -> bool;
        scalar raw_eos_cli("raw_eos_cli", 19) -> &'a str;
        model structured_config("structured_config", 20) -> super::super::eos_cli_config_gen::EosCliConfigGen<'a>;
        scalar uplink_type("uplink_type", 21) -> &'a str;
        scalar uplink_ipv4_pool("uplink_ipv4_pool", 22) -> &'a str;
        scalar uplink_ipv6_pool("uplink_ipv6_pool", 23) -> &'a str;
        model uplink_interfaces("uplink_interfaces", 24) -> item::UplinkInterfaces<'a>;
        model uplink_switch_interfaces("uplink_switch_interfaces", 25) -> item::UplinkSwitchInterfaces<'a>;
        model uplink_switches("uplink_switches", 26) -> item::UplinkSwitches<'a>;
        scalar uplink_interface_speed("uplink_interface_speed", 27) -> &'a str;
        scalar uplink_switch_interface_speed("uplink_switch_interface_speed", 28) -> &'a str;
        scalar uplink_mtu("uplink_mtu", 29) -> i64;
        scalar max_uplink_switches("max_uplink_switches", 30) -> i64;
        scalar max_parallel_uplinks("max_parallel_uplinks", 31) -> i64;
        scalar uplink_bfd("uplink_bfd", 32) -> bool;
        scalar uplink_native_vlan("uplink_native_vlan", 33) -> i64;
        model uplink_ptp("uplink_ptp", 34) -> item::UplinkPtp<'a>;
        model uplink_macsec("uplink_macsec", 35) -> item::UplinkMacsec<'a>;
        scalar uplink_port_channel_id("uplink_port_channel_id", 36) -> i64;
        scalar uplink_switch_port_channel_id("uplink_switch_port_channel_id", 37) -> i64;
        model uplink_ethernet_structured_config("uplink_ethernet_structured_config", 38) -> super::super::eos_cli_config_gen::ethernet_interfaces::Item<'a>;
        model uplink_port_channel_structured_config("uplink_port_channel_structured_config", 39) -> super::super::eos_cli_config_gen::port_channel_interfaces::Item<'a>;
        model uplink_switch_ethernet_structured_config("uplink_switch_ethernet_structured_config", 40) -> super::super::eos_cli_config_gen::ethernet_interfaces::Item<'a>;
        model uplink_switch_port_channel_structured_config("uplink_switch_port_channel_structured_config", 41) -> super::super::eos_cli_config_gen::port_channel_interfaces::Item<'a>;
        model mlag_port_channel_structured_config("mlag_port_channel_structured_config", 42) -> super::super::eos_cli_config_gen::port_channel_interfaces::Item<'a>;
        model mlag_peer_vlan_structured_config("mlag_peer_vlan_structured_config", 43) -> super::super::eos_cli_config_gen::vlan_interfaces::Item<'a>;
        model mlag_peer_l3_vlan_structured_config("mlag_peer_l3_vlan_structured_config", 44) -> super::super::eos_cli_config_gen::vlan_interfaces::Item<'a>;
        scalar short_esi("short_esi", 45) -> &'a str;
        scalar isis_system_id_prefix("isis_system_id_prefix", 46) -> &'a str;
        scalar isis_maximum_paths("isis_maximum_paths", 47) -> i64;
        scalar is_type("is_type", 48) -> &'a str;
        scalar node_sid_base("node_sid_base", 49) -> i64;
        model isis_sr("isis_sr", 50) -> item::IsisSr<'a>;
        scalar loopback_ipv4_pool("loopback_ipv4_pool", 51) -> &'a str;
        scalar loopback_ipv4_address("loopback_ipv4_address", 52) -> &'a str;
        scalar vtep_loopback_ipv4_pool("vtep_loopback_ipv4_pool", 53) -> &'a str;
        scalar vtep_loopback_ipv6_pool("vtep_loopback_ipv6_pool", 54) -> &'a str;
        scalar vtep_loopback_ipv4_address("vtep_loopback_ipv4_address", 55) -> &'a str;
        scalar vtep_loopback_ipv6_address("vtep_loopback_ipv6_address", 56) -> &'a str;
        scalar loopback_ipv4_offset("loopback_ipv4_offset", 57) -> i64;
        scalar router_id_pool("router_id_pool", 58) -> &'a str;
        scalar loopback_ipv6_pool("loopback_ipv6_pool", 59) -> &'a str;
        scalar loopback_ipv6_offset("loopback_ipv6_offset", 60) -> i64;
        scalar vtep("vtep", 61) -> bool;
        scalar vtep_loopback("vtep_loopback", 62) -> &'a str;
        scalar bgp_as("bgp_as", 63) -> &'a str;
        model bgp_defaults("bgp_defaults", 64) -> item::BgpDefaults<'a>;
        scalar evpn_role("evpn_role", 65) -> &'a str;
        model evpn_route_servers("evpn_route_servers", 66) -> item::EvpnRouteServers<'a>;
        scalar evpn_services_l2_only("evpn_services_l2_only", 67) -> bool;
        model filter("filter", 68) -> item::Filter<'a>;
        scalar igmp_snooping_enabled("igmp_snooping_enabled", 69) -> bool;
        model evpn_gateway("evpn_gateway", 70) -> item::EvpnGateway<'a>;
        model ipvpn_gateway("ipvpn_gateway", 71) -> item::IpvpnGateway<'a>;
        scalar mlag("mlag", 72) -> bool;
        scalar mlag_dual_primary_detection("mlag_dual_primary_detection", 73) -> bool;
        scalar mlag_ibgp_origin_incomplete("mlag_ibgp_origin_incomplete", 74) -> bool;
        model mlag_interfaces("mlag_interfaces", 75) -> item::MlagInterfaces<'a>;
        scalar mlag_interfaces_speed("mlag_interfaces_speed", 76) -> &'a str;
        scalar mlag_peer_l3_vlan("mlag_peer_l3_vlan", 77) -> i64;
        scalar mlag_peer_l3_ipv4_pool("mlag_peer_l3_ipv4_pool", 78) -> &'a str;
        scalar mlag_peer_l3_ipv6_pool("mlag_peer_l3_ipv6_pool", 79) -> &'a str;
        scalar mlag_peer_vlan("mlag_peer_vlan", 80) -> i64;
        scalar mlag_peer_link_allowed_vlans("mlag_peer_link_allowed_vlans", 81) -> &'a str;
        scalar mlag_peer_address_family("mlag_peer_address_family", 82) -> &'a str;
        scalar mlag_peer_ipv4_pool("mlag_peer_ipv4_pool", 83) -> &'a str;
        scalar mlag_peer_ipv6_pool("mlag_peer_ipv6_pool", 84) -> &'a str;
        scalar mlag_port_channel_id("mlag_port_channel_id", 85) -> i64;
        scalar mlag_domain_id("mlag_domain_id", 86) -> &'a str;
        scalar spanning_tree_mode("spanning_tree_mode", 87) -> &'a str;
        scalar spanning_tree_priority("spanning_tree_priority", 88) -> i64;
        scalar spanning_tree_root_super("spanning_tree_root_super", 89) -> bool;
        scalar spanning_tree_mst_pvst_boundary("spanning_tree_mst_pvst_boundary", 90) -> bool;
        model spanning_tree_port_id_allocation_port_channel_range("spanning_tree_port_id_allocation_port_channel_range", 91) -> super::super::eos_cli_config_gen::spanning_tree::PortIdAllocationPortChannelRange<'a>;
        scalar virtual_router_mac_address("virtual_router_mac_address", 92) -> &'a str;
        scalar inband_mgmt_interface("inband_mgmt_interface", 93) -> &'a str;
        scalar inband_mgmt_vlan("inband_mgmt_vlan", 94) -> i64;
        scalar inband_mgmt_subnet("inband_mgmt_subnet", 95) -> &'a str;
        scalar inband_mgmt_subnet_offset("inband_mgmt_subnet_offset", 96) -> i64;
        scalar inband_mgmt_ip("inband_mgmt_ip", 97) -> &'a str;
        scalar inband_mgmt_gateway("inband_mgmt_gateway", 98) -> &'a str;
        scalar inband_mgmt_ipv6_address("inband_mgmt_ipv6_address", 99) -> &'a str;
        scalar inband_mgmt_ipv6_subnet("inband_mgmt_ipv6_subnet", 100) -> &'a str;
        scalar inband_mgmt_ipv6_gateway("inband_mgmt_ipv6_gateway", 101) -> &'a str;
        scalar inband_mgmt_description("inband_mgmt_description", 102) -> &'a str;
        scalar inband_mgmt_vlan_name("inband_mgmt_vlan_name", 103) -> &'a str;
        scalar inband_mgmt_vrf("inband_mgmt_vrf", 104) -> &'a str;
        scalar inband_mgmt_mtu("inband_mgmt_mtu", 105) -> i64;
        scalar inband_ztp("inband_ztp", 106) -> bool;
        scalar inband_ztp_lacp_fallback_delay("inband_ztp_lacp_fallback_delay", 107) -> i64;
        scalar mpls_overlay_role("mpls_overlay_role", 108) -> &'a str;
        model overlay_address_families("overlay_address_families", 109) -> item::OverlayAddressFamilies<'a>;
        model mpls_route_reflectors("mpls_route_reflectors", 110) -> item::MplsRouteReflectors<'a>;
        scalar bgp_cluster_id("bgp_cluster_id", 111) -> &'a str;
        scalar kernel_ecmp_cli("kernel_ecmp_cli", 112) -> bool;
        model ptp("ptp", 113) -> item::Ptp<'a>;
        scalar wan_role("wan_role", 114) -> &'a str;
        scalar cv_pathfinder_transit_mode("cv_pathfinder_transit_mode", 115) -> &'a str;
        scalar cv_pathfinder_region("cv_pathfinder_region", 116) -> &'a str;
        scalar cv_pathfinder_site("cv_pathfinder_site", 117) -> &'a str;
        model wan_ha("wan_ha", 118) -> item::WanHa<'a>;
        scalar dps_mss_ipv4("dps_mss_ipv4", 119) -> &'a str;
        model l3_interfaces("l3_interfaces", 120) -> item::L3Interfaces<'a>;
        model l3_port_channels("l3_port_channels", 121) -> item::L3PortChannels<'a>;
        scalar data_plane_cpu_allocation_max("data_plane_cpu_allocation_max", 122) -> i64;
        scalar flow_tracker_type("flow_tracker_type", 123) -> &'a str;
        model underlay_multicast("underlay_multicast", 124) -> item::UnderlayMulticast<'a>;
        scalar campus("campus", 125) -> &'a str;
        scalar campus_pod("campus_pod", 126) -> &'a str;
        scalar campus_access_pod("campus_access_pod", 127) -> &'a str;
        scalar cv_tags_topology_type("cv_tags_topology_type", 128) -> &'a str;
        model digital_twin("digital_twin", 129) -> item::DigitalTwin<'a>;
        scalar validation_profile("validation_profile", 130) -> &'a str;
    }
}

pub mod item {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct DownlinkPools {
            model item (0) -> downlink_pools::Item<'a>;
        }
    }

    pub mod downlink_pools {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar ipv4_pool("ipv4_pool", 0) -> &'a str;
                scalar ipv6_pool("ipv6_pool", 1) -> &'a str;
                model downlink_interfaces("downlink_interfaces", 2) -> item::DownlinkInterfaces<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct DownlinkInterfaces {
                    scalar item (0) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct LinkTracking {
            scalar enabled("enabled", 0) -> bool;
            model downlinks("downlinks", 1) -> link_tracking::Downlinks<'a>;
            model groups("groups", 2) -> link_tracking::Groups<'a>;
        }
    }

    pub mod link_tracking {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Downlinks {
                scalar enabled("enabled", 0) -> bool;
                scalar group("group", 1) -> &'a str;
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Groups {
                model item (0) -> groups::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod groups {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    scalar recovery_delay("recovery_delay", 1) -> i64;
                    scalar links_minimum("links_minimum", 2) -> i64;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct LacpPortIdRange {
            scalar enabled("enabled", 0) -> bool;
            scalar size("size", 1) -> i64;
            scalar offset("offset", 2) -> i64;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct UplinkInterfaces {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct UplinkSwitchInterfaces {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct UplinkSwitches {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct UplinkPtp {
            scalar enable("enable", 0) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct UplinkMacsec {
            scalar profile("profile", 0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct IsisSr {
            scalar ipv4_node_sid_index("ipv4_node_sid_index", 0) -> i64;
            scalar ipv4_node_sid_index_base("ipv4_node_sid_index_base", 1) -> i64;
            scalar ipv6_node_sid_index("ipv6_node_sid_index", 2) -> i64;
            scalar ipv6_node_sid_index_base("ipv6_node_sid_index_base", 3) -> i64;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct BgpDefaults {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct EvpnRouteServers {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Filter {
            model tenants("tenants", 0) -> filter::Tenants<'a>;
            model tags("tags", 1) -> filter::Tags<'a>;
            model allow_vrfs("allow_vrfs", 2) -> filter::AllowVrfs<'a>;
            model deny_vrfs("deny_vrfs", 3) -> filter::DenyVrfs<'a>;
            model always_include_vrfs_in_tenants("always_include_vrfs_in_tenants", 4) -> filter::AlwaysIncludeVrfsInTenants<'a>;
            scalar only_vlans_in_use("only_vlans_in_use", 5) -> bool;
        }
    }

    pub mod filter {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Tenants {
                scalar item (0) -> &'a str;
            }
        }

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Tags {
                scalar item (0) -> &'a str;
            }
        }

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AllowVrfs {
                scalar item (0) -> &'a str;
            }
        }

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct DenyVrfs {
                scalar item (0) -> &'a str;
            }
        }

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AlwaysIncludeVrfsInTenants {
                scalar item (0) -> &'a str;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct EvpnGateway {
            model remote_peers("remote_peers", 0) -> evpn_gateway::RemotePeers<'a>;
            model evpn_l2("evpn_l2", 1) -> evpn_gateway::EvpnL2<'a>;
            model evpn_l3("evpn_l3", 2) -> evpn_gateway::EvpnL3<'a>;
            model d_path("d_path", 3) -> evpn_gateway::DPath<'a>;
            model all_active_multihoming("all_active_multihoming", 4) -> evpn_gateway::AllActiveMultihoming<'a>;
        }
    }

    pub mod evpn_gateway {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct RemotePeers {
                model item (0) -> remote_peers::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod remote_peers {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar hostname("hostname", 0) -> &'a str;
                    scalar ip_address("ip_address", 1) -> &'a str;
                    scalar bgp_as("bgp_as", 2) -> &'a str;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct EvpnL2 {
                scalar enabled("enabled", 0) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct EvpnL3 {
                scalar enabled("enabled", 0) -> bool;
                scalar inter_domain("inter_domain", 1) -> bool;
                scalar mode("mode", 2) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct DPath {
                scalar enabled("enabled", 0) -> bool;
                scalar local_domain_id("local_domain_id", 1) -> &'a str;
                scalar remote_domain_id("remote_domain_id", 2) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AllActiveMultihoming {
                scalar enabled("enabled", 0) -> bool;
                scalar enable_d_path("enable_d_path", 1) -> bool;
                scalar evpn_domain_id_local("evpn_domain_id_local", 2) -> &'a str;
                scalar evpn_domain_id_remote("evpn_domain_id_remote", 3) -> &'a str;
                model evpn_ethernet_segment("evpn_ethernet_segment", 4) -> all_active_multihoming::EvpnEthernetSegment<'a>;
            }
        }

        pub mod all_active_multihoming {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct EvpnEthernetSegment {
                    scalar identifier("identifier", 0) -> &'a str;
                    scalar rt_import("rt_import", 1) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct IpvpnGateway {
            scalar enabled("enabled", 0) -> bool;
            scalar evpn_domain_id("evpn_domain_id", 1) -> &'a str;
            scalar ipvpn_domain_id("ipvpn_domain_id", 2) -> &'a str;
            scalar enable_d_path("enable_d_path", 3) -> bool;
            scalar maximum_routes("maximum_routes", 4) -> i64;
            scalar local_as("local_as", 5) -> &'a str;
            model address_families("address_families", 6) -> ipvpn_gateway::AddressFamilies<'a>;
            model remote_peers("remote_peers", 7) -> ipvpn_gateway::RemotePeers<'a>;
        }
    }

    pub mod ipvpn_gateway {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AddressFamilies {
                scalar item (0) -> &'a str;
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct RemotePeers {
                model item (0) -> remote_peers::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod remote_peers {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar hostname("hostname", 0) -> &'a str;
                    scalar ip_address("ip_address", 1) -> &'a str;
                    scalar bgp_as("bgp_as", 2) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct MlagInterfaces {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct OverlayAddressFamilies {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct MplsRouteReflectors {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ptp {
            scalar enabled("enabled", 0) -> bool;
            scalar profile("profile", 1) -> &'a str;
            model uplinks("uplinks", 2) -> ptp::Uplinks<'a>;
            scalar mlag("mlag", 3) -> bool;
            scalar domain("domain", 4) -> i64;
            scalar priority1("priority1", 5) -> i64;
            scalar priority2("priority2", 6) -> i64;
            scalar auto_clock_identity("auto_clock_identity", 7) -> bool;
            scalar clock_identity_prefix("clock_identity_prefix", 8) -> &'a str;
            scalar clock_identity("clock_identity", 9) -> &'a str;
            scalar source_ip("source_ip", 10) -> &'a str;
            scalar mode("mode", 11) -> &'a str;
            scalar mode_one_step("mode_one_step", 12) -> bool;
            scalar ttl("ttl", 13) -> i64;
            scalar forward_unicast("forward_unicast", 14) -> bool;
            scalar forward_v1("forward_v1", 15) -> bool;
            model free_running("free_running", 16) -> super::super::super::eos_cli_config_gen::ptp::FreeRunning<'a>;
            model dscp("dscp", 17) -> ptp::Dscp<'a>;
            model monitor("monitor", 18) -> ptp::Monitor<'a>;
        }
    }

    pub mod ptp {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Uplinks {
                scalar item (0) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Dscp {
                scalar general_messages("general_messages", 0) -> i64;
                scalar event_messages("event_messages", 1) -> i64;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Monitor {
                scalar enabled("enabled", 0) -> bool;
                model threshold("threshold", 1) -> monitor::Threshold<'a>;
                model missing_message("missing_message", 2) -> monitor::MissingMessage<'a>;
            }
        }

        pub mod monitor {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Threshold {
                    scalar offset_from_master("offset_from_master", 0) -> i64;
                    scalar mean_path_delay("mean_path_delay", 1) -> i64;
                    model drop("drop", 2) -> threshold::Drop<'a>;
                }
            }

            pub mod threshold {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Drop {
                        scalar offset_from_master("offset_from_master", 0) -> i64;
                        scalar mean_path_delay("mean_path_delay", 1) -> i64;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MissingMessage {
                    model intervals("intervals", 0) -> missing_message::Intervals<'a>;
                    model sequence_ids("sequence_ids", 1) -> missing_message::SequenceIds<'a>;
                }
            }

            pub mod missing_message {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Intervals {
                        scalar announce("announce", 0) -> i64;
                        scalar follow_up("follow_up", 1) -> i64;
                        scalar sync("sync", 2) -> i64;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct SequenceIds {
                        scalar enabled("enabled", 0) -> bool;
                        scalar announce("announce", 1) -> i64;
                        scalar delay_resp("delay_resp", 2) -> i64;
                        scalar follow_up("follow_up", 3) -> i64;
                        scalar sync("sync", 4) -> i64;
                    }
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct WanHa {
            scalar enabled("enabled", 0) -> bool;
            scalar ipsec("ipsec", 1) -> bool;
            scalar mtu("mtu", 2) -> i64;
            model ha_interfaces("ha_interfaces", 3) -> wan_ha::HaInterfaces<'a>;
            scalar ha_ipv4_pool("ha_ipv4_pool", 4) -> &'a str;
            scalar port_channel_id("port_channel_id", 5) -> i64;
            scalar use_port_channel_for_direct_ha("use_port_channel_for_direct_ha", 6) -> bool;
            model flow_tracking("flow_tracking", 7) -> wan_ha::FlowTracking<'a>;
        }
    }

    pub mod wan_ha {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct HaInterfaces {
                scalar item (0) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct FlowTracking {
                scalar enabled("enabled", 0) -> bool;
                scalar name("name", 1) -> &'a str;
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct L3Interfaces {
            model item (0) -> l3_interfaces::Item<'a>;
            primary_key_fields: [1];
        }
    }

    pub mod l3_interfaces {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar profile("profile", 0) -> &'a str;
                scalar name("name", 1) -> &'a str;
                scalar description("description", 2) -> &'a str;
                scalar ip_address("ip_address", 3) -> &'a str;
                model ipv6_addresses("ipv6_addresses", 4) -> item::Ipv6Addresses<'a>;
                scalar dhcp_ip("dhcp_ip", 5) -> &'a str;
                scalar public_ip("public_ip", 6) -> &'a str;
                scalar encapsulation_dot1q_vlan("encapsulation_dot1q_vlan", 7) -> i64;
                scalar dhcp_accept_default_route("dhcp_accept_default_route", 8) -> bool;
                scalar enabled("enabled", 9) -> bool;
                scalar speed("speed", 10) -> &'a str;
                scalar receive_bandwidth("receive_bandwidth", 11) -> i64;
                scalar transmit_bandwidth("transmit_bandwidth", 12) -> i64;
                scalar peer("peer", 13) -> &'a str;
                scalar peer_interface("peer_interface", 14) -> &'a str;
                scalar peer_ip("peer_ip", 15) -> &'a str;
                scalar peer_ipv6("peer_ipv6", 16) -> &'a str;
                model bgp("bgp", 17) -> item::Bgp<'a>;
                scalar ipv4_acl_in("ipv4_acl_in", 18) -> &'a str;
                scalar ipv4_acl_out("ipv4_acl_out", 19) -> &'a str;
                scalar ipv6_acl_in("ipv6_acl_in", 20) -> &'a str;
                scalar ipv6_acl_out("ipv6_acl_out", 21) -> &'a str;
                model static_routes("static_routes", 22) -> item::StaticRoutes<'a>;
                scalar qos_profile("qos_profile", 23) -> &'a str;
                scalar wan_carrier("wan_carrier", 24) -> &'a str;
                scalar wan_circuit_id("wan_circuit_id", 25) -> &'a str;
                scalar connected_to_pathfinder("connected_to_pathfinder", 26) -> bool;
                model cv_pathfinder_internet_exit("cv_pathfinder_internet_exit", 27) -> item::CvPathfinderInternetExit<'a>;
                model rx_queue("rx_queue", 28) -> item::RxQueue<'a>;
                scalar raw_eos_cli("raw_eos_cli", 29) -> &'a str;
                model flow_tracking("flow_tracking", 30) -> item::FlowTracking<'a>;
                model structured_config("structured_config", 31) -> super::super::super::super::eos_cli_config_gen::ethernet_interfaces::Item<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Ipv6Addresses {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Bgp {
                    scalar peer_as("peer_as", 0) -> &'a str;
                    scalar ipv4_prefix_list_in("ipv4_prefix_list_in", 1) -> &'a str;
                    scalar ipv4_prefix_list_out("ipv4_prefix_list_out", 2) -> &'a str;
                    scalar ipv6_prefix_list_in("ipv6_prefix_list_in", 3) -> &'a str;
                    scalar ipv6_prefix_list_out("ipv6_prefix_list_out", 4) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct StaticRoutes {
                    model item (0) -> static_routes::Item<'a>;
                }
            }

            pub mod static_routes {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar prefix("prefix", 0) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct CvPathfinderInternetExit {
                    model policies("policies", 0) -> cv_pathfinder_internet_exit::Policies<'a>;
                }
            }

            pub mod cv_pathfinder_internet_exit {

                ::validation::define_archive_indexed_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Policies {
                        model item (0) -> policies::Item<'a>;
                        primary_key_fields: [0];
                    }
                }

                pub mod policies {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            scalar name("name", 0) -> &'a str;
                            scalar tunnel_interface_numbers("tunnel_interface_numbers", 1) -> &'a str;
                        }
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct RxQueue {
                    scalar count("count", 0) -> i64;
                    model workers("workers", 1) -> rx_queue::Workers<'a>;
                    scalar mode("mode", 2) -> &'a str;
                }
            }

            pub mod rx_queue {

                ::validation::define_archive_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Workers {
                        scalar item (0) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct FlowTracking {
                    scalar enabled("enabled", 0) -> bool;
                    scalar name("name", 1) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct L3PortChannels {
            model item (0) -> l3_port_channels::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod l3_port_channels {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar description("description", 1) -> &'a str;
                scalar mode("mode", 2) -> &'a str;
                model member_interfaces("member_interfaces", 3) -> item::MemberInterfaces<'a>;
                scalar ip_address("ip_address", 4) -> &'a str;
                model ipv6_addresses("ipv6_addresses", 5) -> item::Ipv6Addresses<'a>;
                scalar dhcp_ip("dhcp_ip", 6) -> &'a str;
                scalar public_ip("public_ip", 7) -> &'a str;
                scalar encapsulation_dot1q_vlan("encapsulation_dot1q_vlan", 8) -> i64;
                scalar dhcp_accept_default_route("dhcp_accept_default_route", 9) -> bool;
                scalar enabled("enabled", 10) -> bool;
                scalar peer("peer", 11) -> &'a str;
                scalar peer_port_channel("peer_port_channel", 12) -> &'a str;
                scalar peer_ip("peer_ip", 13) -> &'a str;
                scalar peer_ipv6("peer_ipv6", 14) -> &'a str;
                model bgp("bgp", 15) -> item::Bgp<'a>;
                scalar ipv4_acl_in("ipv4_acl_in", 16) -> &'a str;
                scalar ipv4_acl_out("ipv4_acl_out", 17) -> &'a str;
                scalar ipv6_acl_in("ipv6_acl_in", 18) -> &'a str;
                scalar ipv6_acl_out("ipv6_acl_out", 19) -> &'a str;
                model static_routes("static_routes", 20) -> item::StaticRoutes<'a>;
                scalar qos_profile("qos_profile", 21) -> &'a str;
                scalar wan_carrier("wan_carrier", 22) -> &'a str;
                scalar wan_circuit_id("wan_circuit_id", 23) -> &'a str;
                scalar connected_to_pathfinder("connected_to_pathfinder", 24) -> bool;
                scalar raw_eos_cli("raw_eos_cli", 25) -> &'a str;
                model flow_tracking("flow_tracking", 26) -> item::FlowTracking<'a>;
                model structured_config("structured_config", 27) -> super::super::super::super::eos_cli_config_gen::port_channel_interfaces::Item<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MemberInterfaces {
                    model item (0) -> member_interfaces::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod member_interfaces {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar name("name", 0) -> &'a str;
                        scalar description("description", 1) -> &'a str;
                        scalar peer("peer", 2) -> &'a str;
                        scalar peer_interface("peer_interface", 3) -> &'a str;
                        scalar speed("speed", 4) -> &'a str;
                        model rx_queue("rx_queue", 5) -> item::RxQueue<'a>;
                        model structured_config("structured_config", 6) -> super::super::super::super::super::super::eos_cli_config_gen::ethernet_interfaces::Item<'a>;
                    }
                }

                pub mod item {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct RxQueue {
                            scalar count("count", 0) -> i64;
                            model workers("workers", 1) -> rx_queue::Workers<'a>;
                            scalar mode("mode", 2) -> &'a str;
                        }
                    }

                    pub mod rx_queue {

                        ::validation::define_archive_list_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Workers {
                                scalar item (0) -> &'a str;
                            }
                        }
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Ipv6Addresses {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Bgp {
                    scalar peer_as("peer_as", 0) -> &'a str;
                    scalar ipv4_prefix_list_in("ipv4_prefix_list_in", 1) -> &'a str;
                    scalar ipv4_prefix_list_out("ipv4_prefix_list_out", 2) -> &'a str;
                    scalar ipv6_prefix_list_in("ipv6_prefix_list_in", 3) -> &'a str;
                    scalar ipv6_prefix_list_out("ipv6_prefix_list_out", 4) -> &'a str;
                }
            }

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct StaticRoutes {
                    model item (0) -> static_routes::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod static_routes {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar prefix("prefix", 0) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct FlowTracking {
                    scalar enabled("enabled", 0) -> bool;
                    scalar name("name", 1) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct UnderlayMulticast {
            model pim_sm("pim_sm", 0) -> underlay_multicast::PimSm<'a>;
            model field_static("static", 1) -> underlay_multicast::FieldStatic<'a>;
        }
    }

    pub mod underlay_multicast {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct PimSm {
                scalar enabled("enabled", 0) -> bool;
                scalar uplinks("uplinks", 1) -> bool;
                model uplink_interfaces("uplink_interfaces", 2) -> pim_sm::UplinkInterfaces<'a>;
                scalar mlag("mlag", 3) -> bool;
            }
        }

        pub mod pim_sm {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct UplinkInterfaces {
                    scalar item (0) -> &'a str;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct FieldStatic {
                scalar enabled("enabled", 0) -> bool;
                scalar uplinks("uplinks", 1) -> bool;
                model uplink_interfaces("uplink_interfaces", 2) -> field_static::UplinkInterfaces<'a>;
                scalar mlag("mlag", 3) -> bool;
            }
        }

        pub mod field_static {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct UplinkInterfaces {
                    scalar item (0) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct DigitalTwin {
            scalar act_os_version("act_os_version", 0) -> &'a str;
            scalar mgmt_ip("mgmt_ip", 1) -> &'a str;
            scalar mgmt_gateway("mgmt_gateway", 2) -> &'a str;
            scalar act_internet_access("act_internet_access", 3) -> bool;
        }
    }
}
