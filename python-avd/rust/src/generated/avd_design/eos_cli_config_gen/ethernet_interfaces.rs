// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar comment("comment", 1) -> &'a str;
        scalar description("description", 2) -> &'a str;
        scalar shutdown("shutdown", 3) -> bool;
        scalar load_interval("load_interval", 4) -> i64;
        scalar speed("speed", 5) -> &'a str;
        scalar mtu("mtu", 6) -> i64;
        scalar l2_mtu("l2_mtu", 7) -> i64;
        scalar l2_mru("l2_mru", 8) -> i64;
        scalar loop_protection("loop_protection", 9) -> bool;
        scalar arp_gratuitous_accept("arp_gratuitous_accept", 10) -> bool;
        model l2_protocol("l2_protocol", 11) -> item::L2Protocol<'a>;
        scalar mac_timestamp("mac_timestamp", 12) -> &'a str;
        scalar snmp_trap_link_change("snmp_trap_link_change", 13) -> bool;
        model address_locking("address_locking", 14) -> item::AddressLocking<'a>;
        model flowcontrol("flowcontrol", 15) -> item::Flowcontrol<'a>;
        scalar vrf("vrf", 16) -> &'a str;
        model flow_tracker("flow_tracker", 17) -> item::FlowTracker<'a>;
        model error_correction_encoding("error_correction_encoding", 18) -> item::ErrorCorrectionEncoding<'a>;
        model link_tracking_groups("link_tracking_groups", 19) -> item::LinkTrackingGroups<'a>;
        model link_tracking("link_tracking", 20) -> item::LinkTracking<'a>;
        model evpn_ethernet_segment("evpn_ethernet_segment", 21) -> item::EvpnEthernetSegment<'a>;
        model encapsulation_dot1q("encapsulation_dot1q", 22) -> item::EncapsulationDot1q<'a>;
        model encapsulation_vlan("encapsulation_vlan", 23) -> item::EncapsulationVlan<'a>;
        scalar vlan_id("vlan_id", 24) -> i64;
        scalar ip_address("ip_address", 25) -> &'a str;
        model ip_address_secondaries("ip_address_secondaries", 26) -> item::IpAddressSecondaries<'a>;
        scalar ip_verify_unicast_source_reachable_via("ip_verify_unicast_source_reachable_via", 27) -> &'a str;
        scalar dhcp_client_accept_default_route("dhcp_client_accept_default_route", 28) -> bool;
        scalar dhcp_server_ipv4("dhcp_server_ipv4", 29) -> bool;
        scalar dhcp_server_ipv6("dhcp_server_ipv6", 30) -> bool;
        model ip_helpers("ip_helpers", 31) -> item::IpHelpers<'a>;
        model ip_nat("ip_nat", 32) -> item::IpNat<'a>;
        scalar ipv6_enable("ipv6_enable", 33) -> bool;
        scalar ipv6_address("ipv6_address", 34) -> &'a str;
        model ipv6_addresses("ipv6_addresses", 35) -> item::Ipv6Addresses<'a>;
        scalar ipv6_address_auto_config("ipv6_address_auto_config", 36) -> bool;
        scalar ipv6_address_link_local("ipv6_address_link_local", 37) -> &'a str;
        scalar ipv6_nd_ra_disabled("ipv6_nd_ra_disabled", 38) -> bool;
        scalar ipv6_nd_managed_config_flag("ipv6_nd_managed_config_flag", 39) -> bool;
        model ipv6_nd_prefixes("ipv6_nd_prefixes", 40) -> item::Ipv6NdPrefixes<'a>;
        model ipv6_nd("ipv6_nd", 41) -> item::Ipv6Nd<'a>;
        model ipv6_dhcp_relay_destinations("ipv6_dhcp_relay_destinations", 42) -> item::Ipv6DhcpRelayDestinations<'a>;
        scalar access_group_in("access_group_in", 43) -> &'a str;
        scalar access_group_out("access_group_out", 44) -> &'a str;
        scalar ipv6_access_group_in("ipv6_access_group_in", 45) -> &'a str;
        scalar ipv6_access_group_out("ipv6_access_group_out", 46) -> &'a str;
        scalar mac_access_group_in("mac_access_group_in", 47) -> &'a str;
        scalar mac_access_group_out("mac_access_group_out", 48) -> &'a str;
        model multicast("multicast", 49) -> item::Multicast<'a>;
        scalar ospf_network_point_to_point("ospf_network_point_to_point", 50) -> bool;
        scalar ospf_area("ospf_area", 51) -> &'a str;
        scalar ospf_cost("ospf_cost", 52) -> i64;
        scalar ospf_authentication("ospf_authentication", 53) -> &'a str;
        scalar ospf_authentication_key("ospf_authentication_key", 54) -> &'a str;
        scalar ospf_authentication_key_type("ospf_authentication_key_type", 55) -> &'a str;
        model ospf_message_digest_keys("ospf_message_digest_keys", 56) -> item::OspfMessageDigestKeys<'a>;
        model pim("pim", 57) -> item::Pim<'a>;
        model mac_security("mac_security", 58) -> item::MacSecurity<'a>;
        scalar ntp_serve("ntp_serve", 59) -> bool;
        model tcp_mss_ceiling("tcp_mss_ceiling", 60) -> item::TcpMssCeiling<'a>;
        model channel_group("channel_group", 61) -> item::ChannelGroup<'a>;
        scalar isis_enable("isis_enable", 62) -> &'a str;
        scalar isis_bfd("isis_bfd", 63) -> bool;
        scalar isis_passive("isis_passive", 64) -> bool;
        scalar isis_metric("isis_metric", 65) -> i64;
        scalar isis_network_point_to_point("isis_network_point_to_point", 66) -> bool;
        scalar isis_circuit_type("isis_circuit_type", 67) -> &'a str;
        scalar isis_hello_padding("isis_hello_padding", 68) -> bool;
        model isis_authentication("isis_authentication", 69) -> item::IsisAuthentication<'a>;
        model poe("poe", 70) -> item::Poe<'a>;
        model ptp("ptp", 71) -> item::Ptp<'a>;
        scalar profile("profile", 72) -> &'a str;
        model storm_control("storm_control", 73) -> item::StormControl<'a>;
        model logging("logging", 74) -> item::Logging<'a>;
        model lldp("lldp", 75) -> item::Lldp<'a>;
        model dot1x("dot1x", 76) -> item::Dot1x<'a>;
        scalar service_profile("service_profile", 77) -> &'a str;
        model shape("shape", 78) -> item::Shape<'a>;
        model qos("qos", 79) -> item::Qos<'a>;
        scalar spanning_tree_bpdufilter("spanning_tree_bpdufilter", 80) -> &'a str;
        scalar spanning_tree_bpduguard("spanning_tree_bpduguard", 81) -> &'a str;
        model spanning_tree_bpduguard_rate_limit("spanning_tree_bpduguard_rate_limit", 82) -> item::SpanningTreeBpduguardRateLimit<'a>;
        scalar spanning_tree_guard("spanning_tree_guard", 83) -> &'a str;
        scalar spanning_tree_portfast("spanning_tree_portfast", 84) -> &'a str;
        scalar spanning_tree_link_type("spanning_tree_link_type", 85) -> &'a str;
        scalar vmtracer("vmtracer", 86) -> bool;
        model priority_flow_control("priority_flow_control", 87) -> item::PriorityFlowControl<'a>;
        model bfd("bfd", 88) -> item::Bfd<'a>;
        model service_policy("service_policy", 89) -> item::ServicePolicy<'a>;
        scalar cpu_traffic_policy_fallback_vrf("cpu_traffic_policy_fallback_vrf", 90) -> &'a str;
        model mpls("mpls", 91) -> item::Mpls<'a>;
        model lacp_timer("lacp_timer", 92) -> item::LacpTimer<'a>;
        scalar lacp_port_priority("lacp_port_priority", 93) -> i64;
        model transceiver("transceiver", 94) -> item::Transceiver<'a>;
        scalar ip_proxy_arp("ip_proxy_arp", 95) -> bool;
        model traffic_policy("traffic_policy", 96) -> item::TrafficPolicy<'a>;
        model bgp("bgp", 97) -> item::Bgp<'a>;
        model ip_igmp_host_proxy("ip_igmp_host_proxy", 98) -> item::IpIgmpHostProxy<'a>;
        model metadata("metadata", 99) -> item::Metadata<'a>;
        model sflow("sflow", 100) -> item::Sflow<'a>;
        model sync_e("sync_e", 101) -> item::SyncE<'a>;
        model uc_tx_queues("uc_tx_queues", 102) -> item::UcTxQueues<'a>;
        model tx_queues("tx_queues", 103) -> item::TxQueues<'a>;
        model vrrp_ids("vrrp_ids", 104) -> item::VrrpIds<'a>;
        model switchport("switchport", 105) -> item::Switchport<'a>;
        model traffic_engineering("traffic_engineering", 106) -> item::TrafficEngineering<'a>;
        model monitor_link_flap_profiles("monitor_link_flap_profiles", 107) -> item::MonitorLinkFlapProfiles<'a>;
        scalar eos_cli("eos_cli", 108) -> &'a str;
    }
}

pub mod item {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct L2Protocol {
            scalar encapsulation_dot1q_vlan("encapsulation_dot1q_vlan", 0) -> i64;
            scalar forwarding_profile("forwarding_profile", 1) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct AddressLocking {
            scalar ipv4("ipv4", 0) -> bool;
            scalar ipv6("ipv6", 1) -> bool;
            model address_family("address_family", 2) -> address_locking::AddressFamily<'a>;
            scalar ipv4_enforcement_disabled("ipv4_enforcement_disabled", 3) -> bool;
        }
    }

    pub mod address_locking {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AddressFamily {
                scalar ipv4("ipv4", 0) -> bool;
                scalar ipv6("ipv6", 1) -> bool;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Flowcontrol {
            scalar received("received", 0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct FlowTracker {
            scalar sampled("sampled", 0) -> &'a str;
            scalar hardware("hardware", 1) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct ErrorCorrectionEncoding {
            scalar enabled("enabled", 0) -> bool;
            scalar fire_code("fire_code", 1) -> bool;
            scalar reed_solomon("reed_solomon", 2) -> bool;
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct LinkTrackingGroups {
            model item (0) -> link_tracking_groups::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod link_tracking_groups {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar direction("direction", 1) -> &'a str;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct LinkTracking {
            scalar direction("direction", 0) -> &'a str;
            model groups("groups", 1) -> link_tracking::Groups<'a>;
        }
    }

    pub mod link_tracking {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Groups {
                scalar item (0) -> &'a str;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct EvpnEthernetSegment {
            scalar identifier("identifier", 0) -> &'a str;
            scalar redundancy("redundancy", 1) -> &'a str;
            model designated_forwarder_election("designated_forwarder_election", 2) -> evpn_ethernet_segment::DesignatedForwarderElection<'a>;
            model mpls("mpls", 3) -> evpn_ethernet_segment::Mpls<'a>;
            scalar route_target("route_target", 4) -> &'a str;
        }
    }

    pub mod evpn_ethernet_segment {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct DesignatedForwarderElection {
                scalar algorithm("algorithm", 0) -> &'a str;
                scalar preference_value("preference_value", 1) -> i64;
                scalar dont_preempt("dont_preempt", 2) -> bool;
                scalar hold_time("hold_time", 3) -> i64;
                scalar subsequent_hold_time("subsequent_hold_time", 4) -> i64;
                scalar candidate_reachability_required("candidate_reachability_required", 5) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Mpls {
                scalar shared_index("shared_index", 0) -> i64;
                scalar tunnel_flood_filter_time("tunnel_flood_filter_time", 1) -> i64;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct EncapsulationDot1q {
            scalar vlan("vlan", 0) -> i64;
            scalar inner_vlan("inner_vlan", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct EncapsulationVlan {
            model client("client", 0) -> encapsulation_vlan::Client<'a>;
            model network("network", 1) -> encapsulation_vlan::Network<'a>;
        }
    }

    pub mod encapsulation_vlan {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Client {
                scalar encapsulation("encapsulation", 0) -> &'a str;
                scalar vlan("vlan", 1) -> i64;
                scalar outer_vlan("outer_vlan", 2) -> i64;
                scalar inner_vlan("inner_vlan", 3) -> i64;
                scalar inner_encapsulation("inner_encapsulation", 4) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Network {
                scalar encapsulation("encapsulation", 0) -> &'a str;
                scalar vlan("vlan", 1) -> i64;
                scalar outer_vlan("outer_vlan", 2) -> i64;
                scalar inner_vlan("inner_vlan", 3) -> i64;
                scalar inner_encapsulation("inner_encapsulation", 4) -> &'a str;
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct IpAddressSecondaries {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct IpHelpers {
            model item (0) -> ip_helpers::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod ip_helpers {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar ip_helper("ip_helper", 0) -> &'a str;
                scalar source_interface("source_interface", 1) -> &'a str;
                scalar vrf("vrf", 2) -> &'a str;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct IpNat {
            scalar service_profile("service_profile", 0) -> &'a str;
            model destination("destination", 1) -> ip_nat::Destination<'a>;
            model source("source", 2) -> ip_nat::Source<'a>;
        }
    }

    pub mod ip_nat {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Destination {
                model dynamic("dynamic", 0) -> destination::Dynamic<'a>;
                model field_static("static", 1) -> destination::FieldStatic<'a>;
            }
        }

        pub mod destination {

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Dynamic {
                    model item (0) -> dynamic::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod dynamic {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar access_list("access_list", 0) -> &'a str;
                        scalar comment("comment", 1) -> &'a str;
                        scalar pool_name("pool_name", 2) -> &'a str;
                        scalar priority("priority", 3) -> i64;
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct FieldStatic {
                    model item (0) -> field_static::Item<'a>;
                }
            }

            pub mod field_static {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar access_list("access_list", 0) -> &'a str;
                        scalar comment("comment", 1) -> &'a str;
                        scalar direction("direction", 2) -> &'a str;
                        scalar group("group", 3) -> i64;
                        scalar original_ip("original_ip", 4) -> &'a str;
                        scalar original_port("original_port", 5) -> i64;
                        scalar priority("priority", 6) -> i64;
                        scalar protocol("protocol", 7) -> &'a str;
                        scalar translated_ip("translated_ip", 8) -> &'a str;
                        scalar translated_port("translated_port", 9) -> i64;
                    }
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Source {
                model dynamic("dynamic", 0) -> source::Dynamic<'a>;
                model field_static("static", 1) -> source::FieldStatic<'a>;
            }
        }

        pub mod source {

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Dynamic {
                    model item (0) -> dynamic::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod dynamic {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar access_list("access_list", 0) -> &'a str;
                        scalar comment("comment", 1) -> &'a str;
                        scalar nat_type("nat_type", 2) -> &'a str;
                        scalar pool_name("pool_name", 3) -> &'a str;
                        scalar priority("priority", 4) -> i64;
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct FieldStatic {
                    model item (0) -> field_static::Item<'a>;
                }
            }

            pub mod field_static {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar access_list("access_list", 0) -> &'a str;
                        scalar comment("comment", 1) -> &'a str;
                        scalar direction("direction", 2) -> &'a str;
                        scalar group("group", 3) -> i64;
                        scalar original_ip("original_ip", 4) -> &'a str;
                        scalar original_port("original_port", 5) -> i64;
                        scalar priority("priority", 6) -> i64;
                        scalar protocol("protocol", 7) -> &'a str;
                        scalar translated_ip("translated_ip", 8) -> &'a str;
                        scalar translated_port("translated_port", 9) -> i64;
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

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipv6NdPrefixes {
            model item (0) -> ipv6_nd_prefixes::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod ipv6_nd_prefixes {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar ipv6_prefix("ipv6_prefix", 0) -> &'a str;
                scalar valid_lifetime("valid_lifetime", 1) -> &'a str;
                scalar preferred_lifetime("preferred_lifetime", 2) -> &'a str;
                scalar no_autoconfig_flag("no_autoconfig_flag", 3) -> bool;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipv6Nd {
            model cache("cache", 0) -> ipv6_nd::Cache<'a>;
            model ra("ra", 1) -> ipv6_nd::Ra<'a>;
            scalar managed_config_flag("managed_config_flag", 2) -> bool;
            model prefixes("prefixes", 3) -> ipv6_nd::Prefixes<'a>;
            scalar other_config_flag("other_config_flag", 4) -> bool;
        }
    }

    pub mod ipv6_nd {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Cache {
                scalar dynamic_capacity("dynamic_capacity", 0) -> i64;
                scalar expire("expire", 1) -> i64;
                scalar refresh_always("refresh_always", 2) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ra {
                scalar disabled("disabled", 0) -> bool;
                model rx_accept("rx_accept", 1) -> ra::RxAccept<'a>;
                model dns_servers("dns_servers", 2) -> ra::DnsServers<'a>;
                scalar dns_servers_lifetime("dns_servers_lifetime", 3) -> i64;
            }
        }

        pub mod ra {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct RxAccept {
                    scalar default_route("default_route", 0) -> bool;
                    scalar route_preference("route_preference", 1) -> bool;
                }
            }

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct DnsServers {
                    model item (0) -> dns_servers::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod dns_servers {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar address("address", 0) -> &'a str;
                        scalar lifetime("lifetime", 1) -> i64;
                    }
                }
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Prefixes {
                model item (0) -> prefixes::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod prefixes {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar ipv6_prefix("ipv6_prefix", 0) -> &'a str;
                    scalar valid_lifetime("valid_lifetime", 1) -> &'a str;
                    scalar preferred_lifetime("preferred_lifetime", 2) -> &'a str;
                    scalar no_autoconfig_flag("no_autoconfig_flag", 3) -> bool;
                }
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipv6DhcpRelayDestinations {
            model item (0) -> ipv6_dhcp_relay_destinations::Item<'a>;
        }
    }

    pub mod ipv6_dhcp_relay_destinations {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar address("address", 0) -> &'a str;
                scalar vrf("vrf", 1) -> &'a str;
                scalar local_interface("local_interface", 2) -> &'a str;
                scalar source_address("source_address", 3) -> &'a str;
                scalar link_address("link_address", 4) -> &'a str;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Multicast {
            model ipv4("ipv4", 0) -> multicast::Ipv4<'a>;
            model ipv6("ipv6", 1) -> multicast::Ipv6<'a>;
        }
    }

    pub mod multicast {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ipv4 {
                model boundaries("boundaries", 0) -> ipv4::Boundaries<'a>;
                scalar field_static("static", 1) -> bool;
            }
        }

        pub mod ipv4 {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Boundaries {
                    model item (0) -> boundaries::Item<'a>;
                }
            }

            pub mod boundaries {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar boundary("boundary", 0) -> &'a str;
                        scalar out("out", 1) -> bool;
                    }
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ipv6 {
                model boundaries("boundaries", 0) -> ipv6::Boundaries<'a>;
                scalar field_static("static", 1) -> bool;
            }
        }

        pub mod ipv6 {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Boundaries {
                    model item (0) -> boundaries::Item<'a>;
                }
            }

            pub mod boundaries {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar boundary("boundary", 0) -> &'a str;
                    }
                }
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct OspfMessageDigestKeys {
            model item (0) -> ospf_message_digest_keys::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod ospf_message_digest_keys {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar id("id", 0) -> i64;
                scalar hash_algorithm("hash_algorithm", 1) -> &'a str;
                scalar key("key", 2) -> &'a str;
                scalar key_type("key_type", 3) -> &'a str;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Pim {
            model ipv4("ipv4", 0) -> pim::Ipv4<'a>;
        }
    }

    pub mod pim {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ipv4 {
                scalar border_router("border_router", 0) -> bool;
                scalar dr_priority("dr_priority", 1) -> i64;
                scalar sparse_mode("sparse_mode", 2) -> bool;
                scalar bfd("bfd", 3) -> bool;
                scalar bidirectional("bidirectional", 4) -> bool;
                scalar neighbor_filter("neighbor_filter", 5) -> &'a str;
                model hello("hello", 6) -> ipv4::Hello<'a>;
            }
        }

        pub mod ipv4 {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Hello {
                    scalar count("count", 0) -> &'a str;
                    scalar interval("interval", 1) -> i64;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct MacSecurity {
            scalar profile("profile", 0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct TcpMssCeiling {
            scalar ipv4("ipv4", 0) -> i64;
            scalar ipv6("ipv6", 1) -> i64;
            scalar direction("direction", 2) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct ChannelGroup {
            scalar id("id", 0) -> i64;
            scalar mode("mode", 1) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct IsisAuthentication {
            model both("both", 0) -> isis_authentication::Both<'a>;
            model level_1("level_1", 1) -> isis_authentication::Level1<'a>;
            model level_2("level_2", 2) -> isis_authentication::Level2<'a>;
        }
    }

    pub mod isis_authentication {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Both {
                scalar key_type("key_type", 0) -> &'a str;
                scalar key("key", 1) -> &'a str;
                model key_ids("key_ids", 2) -> both::KeyIds<'a>;
                scalar mode("mode", 3) -> &'a str;
                model sha("sha", 4) -> both::Sha<'a>;
                model shared_secret("shared_secret", 5) -> both::SharedSecret<'a>;
                scalar rx_disabled("rx_disabled", 6) -> bool;
            }
        }

        pub mod both {

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct KeyIds {
                    model item (0) -> key_ids::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod key_ids {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar id("id", 0) -> i64;
                        scalar algorithm("algorithm", 1) -> &'a str;
                        scalar key_type("key_type", 2) -> &'a str;
                        scalar key("key", 3) -> &'a str;
                        scalar rfc_5310("rfc_5310", 4) -> bool;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Sha {
                    scalar key_id("key_id", 0) -> i64;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct SharedSecret {
                    scalar profile("profile", 0) -> &'a str;
                    scalar algorithm("algorithm", 1) -> &'a str;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Level1 {
                scalar key_type("key_type", 0) -> &'a str;
                scalar key("key", 1) -> &'a str;
                model key_ids("key_ids", 2) -> level_1::KeyIds<'a>;
                scalar mode("mode", 3) -> &'a str;
                model sha("sha", 4) -> level_1::Sha<'a>;
                model shared_secret("shared_secret", 5) -> level_1::SharedSecret<'a>;
                scalar rx_disabled("rx_disabled", 6) -> bool;
            }
        }

        pub mod level_1 {

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct KeyIds {
                    model item (0) -> key_ids::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod key_ids {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar id("id", 0) -> i64;
                        scalar algorithm("algorithm", 1) -> &'a str;
                        scalar key_type("key_type", 2) -> &'a str;
                        scalar key("key", 3) -> &'a str;
                        scalar rfc_5310("rfc_5310", 4) -> bool;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Sha {
                    scalar key_id("key_id", 0) -> i64;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct SharedSecret {
                    scalar profile("profile", 0) -> &'a str;
                    scalar algorithm("algorithm", 1) -> &'a str;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Level2 {
                scalar key_type("key_type", 0) -> &'a str;
                scalar key("key", 1) -> &'a str;
                model key_ids("key_ids", 2) -> level_2::KeyIds<'a>;
                scalar mode("mode", 3) -> &'a str;
                model sha("sha", 4) -> level_2::Sha<'a>;
                model shared_secret("shared_secret", 5) -> level_2::SharedSecret<'a>;
                scalar rx_disabled("rx_disabled", 6) -> bool;
            }
        }

        pub mod level_2 {

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct KeyIds {
                    model item (0) -> key_ids::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod key_ids {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar id("id", 0) -> i64;
                        scalar algorithm("algorithm", 1) -> &'a str;
                        scalar key_type("key_type", 2) -> &'a str;
                        scalar key("key", 3) -> &'a str;
                        scalar rfc_5310("rfc_5310", 4) -> bool;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Sha {
                    scalar key_id("key_id", 0) -> i64;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct SharedSecret {
                    scalar profile("profile", 0) -> &'a str;
                    scalar algorithm("algorithm", 1) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Poe {
            scalar disabled("disabled", 0) -> bool;
            scalar priority("priority", 1) -> &'a str;
            model reboot("reboot", 2) -> poe::Reboot<'a>;
            model link_down("link_down", 3) -> poe::LinkDown<'a>;
            model shutdown("shutdown", 4) -> poe::Shutdown<'a>;
            model limit("limit", 5) -> poe::Limit<'a>;
            scalar negotiation_lldp("negotiation_lldp", 6) -> bool;
            scalar legacy_detect("legacy_detect", 7) -> bool;
        }
    }

    pub mod poe {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Reboot {
                scalar action("action", 0) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct LinkDown {
                scalar action("action", 0) -> &'a str;
                scalar power_off_delay("power_off_delay", 1) -> i64;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Shutdown {
                scalar action("action", 0) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Limit {
                scalar field_class("class", 0) -> i64;
                scalar watts("watts", 1) -> &'a str;
                scalar fixed("fixed", 2) -> bool;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ptp {
            scalar enable("enable", 0) -> bool;
            model announce("announce", 1) -> ptp::Announce<'a>;
            scalar delay_req("delay_req", 2) -> i64;
            scalar delay_mechanism("delay_mechanism", 3) -> &'a str;
            model profile("profile", 4) -> ptp::Profile<'a>;
            model region("region", 5) -> ptp::Region<'a>;
            model sync_message("sync_message", 6) -> ptp::SyncMessage<'a>;
            scalar role("role", 7) -> &'a str;
            scalar vlan("vlan", 8) -> &'a str;
            scalar transport("transport", 9) -> &'a str;
            model management("management", 10) -> ptp::Management<'a>;
        }
    }

    pub mod ptp {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Announce {
                scalar interval("interval", 0) -> i64;
                scalar timeout("timeout", 1) -> i64;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Profile {
                model g8275_1("g8275_1", 0) -> profile::G82751<'a>;
            }
        }

        pub mod profile {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct G82751 {
                    scalar destination_mac_address("destination_mac_address", 0) -> &'a str;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Region {
                scalar domain_number("domain_number", 0) -> i64;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct SyncMessage {
                scalar interval("interval", 0) -> i64;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Management {
                scalar drop("drop", 0) -> bool;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct StormControl {
            model all("all", 0) -> storm_control::All<'a>;
            model broadcast("broadcast", 1) -> storm_control::Broadcast<'a>;
            model multicast("multicast", 2) -> storm_control::Multicast<'a>;
            model unknown_unicast("unknown_unicast", 3) -> storm_control::UnknownUnicast<'a>;
        }
    }

    pub mod storm_control {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct All {
                scalar level("level", 0) -> &'a str;
                scalar unit("unit", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Broadcast {
                scalar level("level", 0) -> &'a str;
                scalar unit("unit", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Multicast {
                scalar level("level", 0) -> &'a str;
                scalar unit("unit", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct UnknownUnicast {
                scalar level("level", 0) -> &'a str;
                scalar unit("unit", 1) -> &'a str;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Logging {
            model event("event", 0) -> logging::Event<'a>;
        }
    }

    pub mod logging {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Event {
                scalar link_status("link_status", 0) -> bool;
                scalar congestion_drops("congestion_drops", 1) -> bool;
                scalar spanning_tree("spanning_tree", 2) -> bool;
                scalar storm_control_discards("storm_control_discards", 3) -> bool;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Lldp {
            scalar transmit("transmit", 0) -> bool;
            scalar receive("receive", 1) -> bool;
            scalar ztp_vlan("ztp_vlan", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Dot1x {
            scalar port_control("port_control", 0) -> &'a str;
            scalar port_control_force_authorized_phone("port_control_force_authorized_phone", 1) -> bool;
            scalar reauthentication("reauthentication", 2) -> bool;
            model pae("pae", 3) -> dot1x::Pae<'a>;
            model authentication_failure("authentication_failure", 4) -> dot1x::AuthenticationFailure<'a>;
            model host_mode("host_mode", 5) -> dot1x::HostMode<'a>;
            model mac_based_authentication("mac_based_authentication", 6) -> dot1x::MacBasedAuthentication<'a>;
            scalar mac_based_access_list("mac_based_access_list", 7) -> bool;
            model timeout("timeout", 8) -> dot1x::Timeout<'a>;
            scalar reauthorization_request_limit("reauthorization_request_limit", 9) -> i64;
            model unauthorized("unauthorized", 10) -> dot1x::Unauthorized<'a>;
            model eapol("eapol", 11) -> dot1x::Eapol<'a>;
            model aaa("aaa", 12) -> dot1x::Aaa<'a>;
        }
    }

    pub mod dot1x {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Pae {
                scalar mode("mode", 0) -> &'a str;
                scalar supplicant_profile("supplicant_profile", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AuthenticationFailure {
                scalar action("action", 0) -> &'a str;
                scalar allow_vlan("allow_vlan", 1) -> i64;
                scalar allow_access_list("allow_access_list", 2) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct HostMode {
                scalar mode("mode", 0) -> &'a str;
                scalar multi_host_authenticated("multi_host_authenticated", 1) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct MacBasedAuthentication {
                scalar enabled("enabled", 0) -> bool;
                scalar always("always", 1) -> bool;
                scalar host_mode_common("host_mode_common", 2) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Timeout {
                scalar idle_host("idle_host", 0) -> i64;
                scalar quiet_period("quiet_period", 1) -> i64;
                scalar reauth_period("reauth_period", 2) -> &'a str;
                scalar reauth_timeout_ignore("reauth_timeout_ignore", 3) -> bool;
                scalar tx_period("tx_period", 4) -> i64;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Unauthorized {
                scalar access_vlan_membership_egress("access_vlan_membership_egress", 0) -> bool;
                scalar native_vlan_membership_egress("native_vlan_membership_egress", 1) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Eapol {
                scalar disabled("disabled", 0) -> bool;
                model authentication_failure_fallback_mba("authentication_failure_fallback_mba", 1) -> eapol::AuthenticationFailureFallbackMba<'a>;
            }
        }

        pub mod eapol {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AuthenticationFailureFallbackMba {
                    scalar enabled("enabled", 0) -> bool;
                    scalar timeout("timeout", 1) -> i64;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Aaa {
                model unresponsive("unresponsive", 0) -> aaa::Unresponsive<'a>;
            }
        }

        pub mod aaa {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Unresponsive {
                    scalar eap_response("eap_response", 0) -> &'a str;
                    model action("action", 1) -> unresponsive::Action<'a>;
                    model phone_action("phone_action", 2) -> unresponsive::PhoneAction<'a>;
                }
            }

            pub mod unresponsive {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Action {
                        scalar traffic_allow_access_list("traffic_allow_access_list", 0) -> &'a str;
                        scalar apply_alternate("apply_alternate", 1) -> bool;
                        scalar traffic_allow_vlan("traffic_allow_vlan", 2) -> i64;
                        scalar apply_cached_results("apply_cached_results", 3) -> bool;
                        model cached_results_timeout("cached_results_timeout", 4) -> action::CachedResultsTimeout<'a>;
                        scalar traffic_allow("traffic_allow", 5) -> bool;
                    }
                }

                pub mod action {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct CachedResultsTimeout {
                            scalar time_duration("time_duration", 0) -> i64;
                            scalar time_duration_unit("time_duration_unit", 1) -> &'a str;
                        }
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct PhoneAction {
                        scalar apply_cached_results("apply_cached_results", 0) -> bool;
                        model cached_results_timeout("cached_results_timeout", 1) -> phone_action::CachedResultsTimeout<'a>;
                        scalar apply_alternate("apply_alternate", 2) -> bool;
                        scalar traffic_allow("traffic_allow", 3) -> bool;
                    }
                }

                pub mod phone_action {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct CachedResultsTimeout {
                            scalar time_duration("time_duration", 0) -> i64;
                            scalar time_duration_unit("time_duration_unit", 1) -> &'a str;
                        }
                    }
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Shape {
            scalar rate("rate", 0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Qos {
            scalar trust("trust", 0) -> &'a str;
            scalar dscp("dscp", 1) -> i64;
            scalar cos("cos", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct SpanningTreeBpduguardRateLimit {
            scalar enabled("enabled", 0) -> bool;
            scalar count("count", 1) -> i64;
            scalar interval("interval", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PriorityFlowControl {
            scalar enabled("enabled", 0) -> bool;
            model priorities("priorities", 1) -> priority_flow_control::Priorities<'a>;
        }
    }

    pub mod priority_flow_control {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Priorities {
                model item (0) -> priorities::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod priorities {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar priority("priority", 0) -> i64;
                    scalar no_drop("no_drop", 1) -> bool;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Bfd {
            scalar echo("echo", 0) -> bool;
            scalar interval("interval", 1) -> i64;
            scalar min_rx("min_rx", 2) -> i64;
            scalar multiplier("multiplier", 3) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct ServicePolicy {
            model pbr("pbr", 0) -> service_policy::Pbr<'a>;
            model qos("qos", 1) -> service_policy::Qos<'a>;
        }
    }

    pub mod service_policy {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Pbr {
                scalar input("input", 0) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Qos {
                scalar input("input", 0) -> &'a str;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Mpls {
            scalar ip("ip", 0) -> bool;
            model ldp("ldp", 1) -> mpls::Ldp<'a>;
        }
    }

    pub mod mpls {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ldp {
                scalar interface("interface", 0) -> bool;
                scalar igp_sync("igp_sync", 1) -> bool;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct LacpTimer {
            scalar mode("mode", 0) -> &'a str;
            scalar multiplier("multiplier", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Transceiver {
            scalar frequency("frequency", 0) -> &'a str;
            scalar frequency_unit("frequency_unit", 1) -> &'a str;
            model media("media", 2) -> transceiver::Media<'a>;
            scalar application_override("application_override", 3) -> &'a str;
            model application_override_lanes("application_override_lanes", 4) -> transceiver::ApplicationOverrideLanes<'a>;
            model power("power", 5) -> transceiver::Power<'a>;
            model transmitter("transmitter", 6) -> transceiver::Transmitter<'a>;
        }
    }

    pub mod transceiver {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Media {
                scalar field_override("override", 0) -> &'a str;
            }
        }

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct ApplicationOverrideLanes {
                model item (0) -> application_override_lanes::Item<'a>;
            }
        }

        pub mod application_override_lanes {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar field_override("override", 0) -> i64;
                    scalar first_lane("first_lane", 1) -> i64;
                    scalar last_lane("last_lane", 2) -> i64;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Power {
                scalar ignore("ignore", 0) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Transmitter {
                scalar signal_power("signal_power", 0) -> &'a str;
                scalar disabled("disabled", 1) -> bool;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct TrafficPolicy {
            scalar input("input", 0) -> &'a str;
            scalar output("output", 1) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Bgp {
            scalar session_tracker("session_tracker", 0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct IpIgmpHostProxy {
            scalar enabled("enabled", 0) -> bool;
            model groups("groups", 1) -> ip_igmp_host_proxy::Groups<'a>;
            scalar report_interval("report_interval", 2) -> i64;
            model access_lists("access_lists", 3) -> ip_igmp_host_proxy::AccessLists<'a>;
            scalar version("version", 4) -> i64;
        }
    }

    pub mod ip_igmp_host_proxy {

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
                    scalar group("group", 0) -> &'a str;
                    model exclude("exclude", 1) -> item::Exclude<'a>;
                    model include("include", 2) -> item::Include<'a>;
                }
            }

            pub mod item {

                ::validation::define_archive_indexed_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Exclude {
                        model item (0) -> exclude::Item<'a>;
                        primary_key_fields: [0];
                    }
                }

                pub mod exclude {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            scalar source("source", 0) -> &'a str;
                        }
                    }
                }

                ::validation::define_archive_indexed_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Include {
                        model item (0) -> include::Item<'a>;
                        primary_key_fields: [0];
                    }
                }

                pub mod include {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            scalar source("source", 0) -> &'a str;
                        }
                    }
                }
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AccessLists {
                model item (0) -> access_lists::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod access_lists {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Metadata {
            scalar peer("peer", 0) -> &'a str;
            scalar peer_interface("peer_interface", 1) -> &'a str;
            scalar peer_type("peer_type", 2) -> &'a str;
            scalar peer_key("peer_key", 3) -> &'a str;
            scalar port_profile("port_profile", 4) -> &'a str;
            scalar validate_state("validate_state", 5) -> bool;
            scalar validate_lldp("validate_lldp", 6) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Sflow {
            scalar enable("enable", 0) -> bool;
            model egress("egress", 1) -> sflow::Egress<'a>;
        }
    }

    pub mod sflow {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Egress {
                scalar enable("enable", 0) -> bool;
                scalar unmodified_enable("unmodified_enable", 1) -> bool;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct SyncE {
            scalar enable("enable", 0) -> bool;
            scalar priority("priority", 1) -> &'a str;
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct UcTxQueues {
            model item (0) -> uc_tx_queues::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod uc_tx_queues {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar id("id", 0) -> i64;
                model random_detect("random_detect", 1) -> item::RandomDetect<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct RandomDetect {
                    model ecn("ecn", 0) -> random_detect::Ecn<'a>;
                }
            }

            pub mod random_detect {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Ecn {
                        scalar count("count", 0) -> bool;
                        model threshold("threshold", 1) -> ecn::Threshold<'a>;
                    }
                }

                pub mod ecn {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Threshold {
                            scalar units("units", 0) -> &'a str;
                            scalar min("min", 1) -> i64;
                            scalar max("max", 2) -> i64;
                            scalar max_probability("max_probability", 3) -> i64;
                            scalar weight("weight", 4) -> i64;
                        }
                    }
                }
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct TxQueues {
            model item (0) -> tx_queues::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod tx_queues {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar id("id", 0) -> i64;
                scalar scheduler_profile_responsive("scheduler_profile_responsive", 1) -> bool;
                model random_detect("random_detect", 2) -> item::RandomDetect<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct RandomDetect {
                    model ecn("ecn", 0) -> random_detect::Ecn<'a>;
                }
            }

            pub mod random_detect {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Ecn {
                        scalar count("count", 0) -> bool;
                        model threshold("threshold", 1) -> ecn::Threshold<'a>;
                    }
                }

                pub mod ecn {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Threshold {
                            scalar units("units", 0) -> &'a str;
                            scalar min("min", 1) -> i64;
                            scalar max("max", 2) -> i64;
                            scalar max_probability("max_probability", 3) -> i64;
                            scalar weight("weight", 4) -> i64;
                        }
                    }
                }
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct VrrpIds {
            model item (0) -> vrrp_ids::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod vrrp_ids {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar id("id", 0) -> i64;
                scalar priority_level("priority_level", 1) -> i64;
                model advertisement("advertisement", 2) -> item::Advertisement<'a>;
                model preempt("preempt", 3) -> item::Preempt<'a>;
                model timers("timers", 4) -> item::Timers<'a>;
                model tracked_object("tracked_object", 5) -> item::TrackedObject<'a>;
                model ipv4("ipv4", 6) -> item::Ipv4<'a>;
                model ipv6("ipv6", 7) -> item::Ipv6<'a>;
                model peer_authentication("peer_authentication", 8) -> item::PeerAuthentication<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Advertisement {
                    scalar interval("interval", 0) -> i64;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Preempt {
                    scalar enabled("enabled", 0) -> bool;
                    model delay("delay", 1) -> preempt::Delay<'a>;
                }
            }

            pub mod preempt {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Delay {
                        scalar minimum("minimum", 0) -> i64;
                        scalar reload("reload", 1) -> i64;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Timers {
                    model delay("delay", 0) -> timers::Delay<'a>;
                }
            }

            pub mod timers {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Delay {
                        scalar reload("reload", 0) -> i64;
                    }
                }
            }

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct TrackedObject {
                    model item (0) -> tracked_object::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod tracked_object {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar name("name", 0) -> &'a str;
                        scalar decrement("decrement", 1) -> i64;
                        scalar shutdown("shutdown", 2) -> bool;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Ipv4 {
                    scalar address("address", 0) -> &'a str;
                    model secondary_addresses("secondary_addresses", 1) -> ipv4::SecondaryAddresses<'a>;
                    scalar version("version", 2) -> i64;
                }
            }

            pub mod ipv4 {

                ::validation::define_archive_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct SecondaryAddresses {
                        scalar item (0) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Ipv6 {
                    model addresses("addresses", 0) -> ipv6::Addresses<'a>;
                }
            }

            pub mod ipv6 {

                ::validation::define_archive_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Addresses {
                        scalar item (0) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct PeerAuthentication {
                    scalar mode("mode", 0) -> &'a str;
                    scalar key("key", 1) -> &'a str;
                    scalar key_type("key_type", 2) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Switchport {
            scalar enabled("enabled", 0) -> bool;
            scalar mode("mode", 1) -> &'a str;
            scalar access_vlan("access_vlan", 2) -> i64;
            model trunk("trunk", 3) -> switchport::Trunk<'a>;
            model phone("phone", 4) -> switchport::Phone<'a>;
            scalar pvlan_mapping("pvlan_mapping", 5) -> &'a str;
            model dot1q("dot1q", 6) -> switchport::Dot1q<'a>;
            scalar source_interface("source_interface", 7) -> &'a str;
            model vlan_translations("vlan_translations", 8) -> switchport::VlanTranslations<'a>;
            scalar vlan_forwarding_accept_all("vlan_forwarding_accept_all", 9) -> bool;
            model backup_link("backup_link", 10) -> switchport::BackupLink<'a>;
            model backup("backup", 11) -> switchport::Backup<'a>;
            model port_security("port_security", 12) -> switchport::PortSecurity<'a>;
            model tap("tap", 13) -> switchport::Tap<'a>;
            model tool("tool", 14) -> switchport::Tool<'a>;
        }
    }

    pub mod switchport {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Trunk {
                scalar allowed_vlan("allowed_vlan", 0) -> &'a str;
                scalar native_vlan("native_vlan", 1) -> i64;
                scalar native_vlan_tag("native_vlan_tag", 2) -> bool;
                scalar private_vlan_secondary("private_vlan_secondary", 3) -> bool;
                model groups("groups", 4) -> trunk::Groups<'a>;
            }
        }

        pub mod trunk {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Groups {
                    scalar item (0) -> &'a str;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Phone {
                scalar vlan("vlan", 0) -> i64;
                scalar trunk("trunk", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Dot1q {
                scalar ethertype("ethertype", 0) -> i64;
                scalar vlan_tag("vlan_tag", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct VlanTranslations {
                scalar in_required("in_required", 0) -> bool;
                scalar out_required("out_required", 1) -> bool;
                model direction_in("direction_in", 2) -> vlan_translations::DirectionIn<'a>;
                model direction_out("direction_out", 3) -> vlan_translations::DirectionOut<'a>;
                model direction_both("direction_both", 4) -> vlan_translations::DirectionBoth<'a>;
            }
        }

        pub mod vlan_translations {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct DirectionIn {
                    model item (0) -> direction_in::Item<'a>;
                }
            }

            pub mod direction_in {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar field_from("from", 0) -> &'a str;
                        scalar to("to", 1) -> i64;
                        scalar dot1q_tunnel("dot1q_tunnel", 2) -> bool;
                        scalar inner_vlan_from("inner_vlan_from", 3) -> i64;
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct DirectionOut {
                    model item (0) -> direction_out::Item<'a>;
                }
            }

            pub mod direction_out {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar field_from("from", 0) -> &'a str;
                        scalar to("to", 1) -> i64;
                        scalar dot1q_tunnel_to("dot1q_tunnel_to", 2) -> &'a str;
                        scalar inner_vlan_to("inner_vlan_to", 3) -> i64;
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct DirectionBoth {
                    model item (0) -> direction_both::Item<'a>;
                }
            }

            pub mod direction_both {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar field_from("from", 0) -> &'a str;
                        scalar to("to", 1) -> i64;
                        scalar dot1q_tunnel("dot1q_tunnel", 2) -> bool;
                        scalar inner_vlan_from("inner_vlan_from", 3) -> i64;
                        scalar network("network", 4) -> bool;
                    }
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct BackupLink {
                scalar interface("interface", 0) -> &'a str;
                scalar prefer_vlan("prefer_vlan", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Backup {
                scalar dest_macaddr("dest_macaddr", 0) -> &'a str;
                scalar initial_mac_move_delay("initial_mac_move_delay", 1) -> i64;
                scalar mac_move_burst("mac_move_burst", 2) -> i64;
                scalar mac_move_burst_interval("mac_move_burst_interval", 3) -> i64;
                scalar preemption_delay("preemption_delay", 4) -> i64;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct PortSecurity {
                scalar enabled("enabled", 0) -> bool;
                model mac_address_maximum("mac_address_maximum", 1) -> port_security::MacAddressMaximum<'a>;
                model violation("violation", 2) -> port_security::Violation<'a>;
                scalar vlan_default_mac_address_maximum("vlan_default_mac_address_maximum", 3) -> i64;
                model vlans("vlans", 4) -> port_security::Vlans<'a>;
            }
        }

        pub mod port_security {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MacAddressMaximum {
                    scalar disabled("disabled", 0) -> bool;
                    scalar limit("limit", 1) -> i64;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Violation {
                    scalar mode("mode", 0) -> &'a str;
                    scalar protect_log("protect_log", 1) -> bool;
                }
            }

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Vlans {
                    model item (0) -> vlans::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod vlans {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar range("range", 0) -> &'a str;
                        scalar mac_address_maximum("mac_address_maximum", 1) -> i64;
                    }
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Tap {
                scalar allowed_vlan("allowed_vlan", 0) -> &'a str;
                model default("default", 1) -> tap::Default<'a>;
                model identity("identity", 2) -> tap::Identity<'a>;
                scalar mpls_pop_all("mpls_pop_all", 3) -> bool;
                scalar native_vlan("native_vlan", 4) -> i64;
                model truncation("truncation", 5) -> tap::Truncation<'a>;
                model mac_address("mac_address", 6) -> tap::MacAddress<'a>;
                model encapsulation("encapsulation", 7) -> tap::Encapsulation<'a>;
            }
        }

        pub mod tap {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Default {
                    model groups("groups", 0) -> default::Groups<'a>;
                    model interfaces("interfaces", 1) -> default::Interfaces<'a>;
                    model nexthop_groups("nexthop_groups", 2) -> default::NexthopGroups<'a>;
                }
            }

            pub mod default {

                ::validation::define_archive_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Groups {
                        scalar item (0) -> &'a str;
                    }
                }

                ::validation::define_archive_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Interfaces {
                        scalar item (0) -> &'a str;
                    }
                }

                ::validation::define_archive_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct NexthopGroups {
                        scalar item (0) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Identity {
                    scalar id("id", 0) -> i64;
                    scalar inner_vlan("inner_vlan", 1) -> i64;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Truncation {
                    scalar enabled("enabled", 0) -> bool;
                    scalar size("size", 1) -> i64;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MacAddress {
                    scalar source("source", 0) -> &'a str;
                    scalar destination("destination", 1) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Encapsulation {
                    scalar vxlan_strip("vxlan_strip", 0) -> bool;
                    model gre("gre", 1) -> encapsulation::Gre<'a>;
                }
            }

            pub mod encapsulation {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Gre {
                        scalar strip("strip", 0) -> bool;
                        model protocols("protocols", 1) -> gre::Protocols<'a>;
                        model destinations("destinations", 2) -> gre::Destinations<'a>;
                    }
                }

                pub mod gre {

                    ::validation::define_archive_indexed_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Protocols {
                            model item (0) -> protocols::Item<'a>;
                            primary_key_fields: [0];
                        }
                    }

                    pub mod protocols {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Item {
                                scalar protocol("protocol", 0) -> &'a str;
                                scalar strip("strip", 1) -> bool;
                                scalar feature_header_length("feature_header_length", 2) -> i64;
                                scalar re_encapsulation_ethernet_header("re_encapsulation_ethernet_header", 3) -> bool;
                            }
                        }
                    }

                    ::validation::define_archive_indexed_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Destinations {
                            model item (0) -> destinations::Item<'a>;
                            primary_key_fields: [0];
                        }
                    }

                    pub mod destinations {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Item {
                                scalar destination("destination", 0) -> &'a str;
                                scalar source("source", 1) -> &'a str;
                                scalar strip("strip", 2) -> bool;
                                model protocols("protocols", 3) -> item::Protocols<'a>;
                            }
                        }

                        pub mod item {

                            ::validation::define_archive_indexed_list_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct Protocols {
                                    model item (0) -> protocols::Item<'a>;
                                    primary_key_fields: [0];
                                }
                            }

                            pub mod protocols {

                                ::validation::define_archive_dict_view! {
                                    #[derive(Clone, Copy, Debug)]
                                    pub struct Item {
                                        scalar protocol("protocol", 0) -> &'a str;
                                        scalar strip("strip", 1) -> bool;
                                        scalar feature_header_length("feature_header_length", 2) -> i64;
                                        scalar re_encapsulation_ethernet_header("re_encapsulation_ethernet_header", 3) -> bool;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Tool {
                scalar mpls_pop_all("mpls_pop_all", 0) -> bool;
                model encapsulation("encapsulation", 1) -> tool::Encapsulation<'a>;
                scalar allowed_vlan("allowed_vlan", 2) -> &'a str;
                model identity("identity", 3) -> tool::Identity<'a>;
                model groups("groups", 4) -> tool::Groups<'a>;
                scalar dot1q_remove_outer_vlan_tag("dot1q_remove_outer_vlan_tag", 5) -> &'a str;
            }
        }

        pub mod tool {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Encapsulation {
                    scalar dot1br_strip("dot1br_strip", 0) -> bool;
                    scalar vn_tag_strip("vn_tag_strip", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Identity {
                    scalar tag("tag", 0) -> &'a str;
                    scalar dot1q_dzgre_source("dot1q_dzgre_source", 1) -> &'a str;
                    scalar qinq_dzgre_source("qinq_dzgre_source", 2) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Groups {
                    scalar item (0) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct TrafficEngineering {
            scalar enabled("enabled", 0) -> bool;
            model administrative_groups("administrative_groups", 1) -> traffic_engineering::AdministrativeGroups<'a>;
            model srlgs("srlgs", 2) -> traffic_engineering::Srlgs<'a>;
            scalar metric("metric", 3) -> i64;
            model bandwidth("bandwidth", 4) -> traffic_engineering::Bandwidth<'a>;
            model min_delay_static("min_delay_static", 5) -> traffic_engineering::MinDelayStatic<'a>;
            model min_delay_dynamic("min_delay_dynamic", 6) -> traffic_engineering::MinDelayDynamic<'a>;
        }
    }

    pub mod traffic_engineering {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AdministrativeGroups {
                scalar item (0) -> &'a str;
            }
        }

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Srlgs {
                scalar item (0) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Bandwidth {
                scalar number("number", 0) -> i64;
                scalar unit("unit", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct MinDelayStatic {
                scalar number("number", 0) -> i64;
                scalar unit("unit", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct MinDelayDynamic {
                model twamp_light_fallback("twamp_light_fallback", 0) -> min_delay_dynamic::TwampLightFallback<'a>;
            }
        }

        pub mod min_delay_dynamic {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct TwampLightFallback {
                    scalar number("number", 0) -> i64;
                    scalar unit("unit", 1) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct MonitorLinkFlapProfiles {
            scalar item (0) -> &'a str;
        }
    }
}
