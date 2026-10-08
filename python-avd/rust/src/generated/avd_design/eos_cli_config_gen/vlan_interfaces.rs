// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar description("description", 1) -> &'a str;
        model logging("logging", 2) -> item::Logging<'a>;
        scalar shutdown("shutdown", 3) -> bool;
        scalar vrf("vrf", 4) -> &'a str;
        scalar arp_aging_timeout("arp_aging_timeout", 5) -> i64;
        scalar arp_cache_dynamic_capacity("arp_cache_dynamic_capacity", 6) -> i64;
        scalar arp_gratuitous_accept("arp_gratuitous_accept", 7) -> bool;
        scalar arp_monitor_mac_address("arp_monitor_mac_address", 8) -> bool;
        scalar ip_proxy_arp("ip_proxy_arp", 9) -> bool;
        scalar ip_directed_broadcast("ip_directed_broadcast", 10) -> bool;
        scalar ip_address("ip_address", 11) -> &'a str;
        scalar dhcp_client_accept_default_route("dhcp_client_accept_default_route", 12) -> bool;
        model ip_address_secondaries("ip_address_secondaries", 13) -> item::IpAddressSecondaries<'a>;
        model ip_virtual_router_addresses("ip_virtual_router_addresses", 14) -> item::IpVirtualRouterAddresses<'a>;
        scalar ip_address_virtual("ip_address_virtual", 15) -> &'a str;
        model ip_address_virtual_secondaries("ip_address_virtual_secondaries", 16) -> item::IpAddressVirtualSecondaries<'a>;
        scalar ip_verify_unicast_source_reachable_via("ip_verify_unicast_source_reachable_via", 17) -> &'a str;
        scalar ip_igmp("ip_igmp", 18) -> bool;
        scalar ip_igmp_version("ip_igmp_version", 19) -> i64;
        scalar ip_igmp_querier_address_virtual("ip_igmp_querier_address_virtual", 20) -> bool;
        model ip_igmp_host_proxy("ip_igmp_host_proxy", 21) -> item::IpIgmpHostProxy<'a>;
        model ip_helpers("ip_helpers", 22) -> item::IpHelpers<'a>;
        scalar ip_dhcp_relay_all_subnets("ip_dhcp_relay_all_subnets", 23) -> bool;
        model ip_nat("ip_nat", 24) -> item::IpNat<'a>;
        scalar dhcp_server_ipv4("dhcp_server_ipv4", 25) -> bool;
        scalar dhcp_server_ipv6("dhcp_server_ipv6", 26) -> bool;
        scalar ipv6_enable("ipv6_enable", 27) -> bool;
        scalar ipv6_address("ipv6_address", 28) -> &'a str;
        model ipv6_addresses("ipv6_addresses", 29) -> item::Ipv6Addresses<'a>;
        scalar ipv6_address_auto_config("ipv6_address_auto_config", 30) -> bool;
        model ipv6_address_virtuals("ipv6_address_virtuals", 31) -> item::Ipv6AddressVirtuals<'a>;
        scalar ipv6_address_link_local("ipv6_address_link_local", 32) -> &'a str;
        model ipv6_virtual_router_addresses("ipv6_virtual_router_addresses", 33) -> item::Ipv6VirtualRouterAddresses<'a>;
        scalar ipv6_nd_ra_disabled("ipv6_nd_ra_disabled", 34) -> bool;
        scalar ipv6_nd_managed_config_flag("ipv6_nd_managed_config_flag", 35) -> bool;
        scalar ipv6_nd_other_config_flag("ipv6_nd_other_config_flag", 36) -> bool;
        model ipv6_nd_cache("ipv6_nd_cache", 37) -> item::Ipv6NdCache<'a>;
        model ipv6_nd_prefixes("ipv6_nd_prefixes", 38) -> item::Ipv6NdPrefixes<'a>;
        model ipv6_dhcp_relay_destinations("ipv6_dhcp_relay_destinations", 39) -> item::Ipv6DhcpRelayDestinations<'a>;
        scalar ipv6_dhcp_relay_all_subnets("ipv6_dhcp_relay_all_subnets", 40) -> bool;
        model ipv6_nd("ipv6_nd", 41) -> item::Ipv6Nd<'a>;
        scalar access_group_in("access_group_in", 42) -> &'a str;
        scalar access_group_out("access_group_out", 43) -> &'a str;
        scalar ipv6_access_group_in("ipv6_access_group_in", 44) -> &'a str;
        scalar ipv6_access_group_out("ipv6_access_group_out", 45) -> &'a str;
        model multicast("multicast", 46) -> item::Multicast<'a>;
        scalar ospf_network_point_to_point("ospf_network_point_to_point", 47) -> bool;
        scalar ospf_area("ospf_area", 48) -> &'a str;
        model ipv6_ospf("ipv6_ospf", 49) -> item::Ipv6Ospf<'a>;
        model ospfv3("ospfv3", 50) -> item::Ospfv3<'a>;
        scalar ospf_cost("ospf_cost", 51) -> i64;
        scalar ospf_authentication("ospf_authentication", 52) -> &'a str;
        scalar ospf_authentication_key("ospf_authentication_key", 53) -> &'a str;
        scalar ospf_authentication_key_type("ospf_authentication_key_type", 54) -> &'a str;
        model ospf_message_digest_keys("ospf_message_digest_keys", 55) -> item::OspfMessageDigestKeys<'a>;
        model pim("pim", 56) -> item::Pim<'a>;
        scalar isis_enable("isis_enable", 57) -> &'a str;
        scalar isis_bfd("isis_bfd", 58) -> bool;
        scalar isis_passive("isis_passive", 59) -> bool;
        scalar isis_metric("isis_metric", 60) -> i64;
        scalar isis_network_point_to_point("isis_network_point_to_point", 61) -> bool;
        scalar isis_circuit_type("isis_circuit_type", 62) -> &'a str;
        scalar isis_hello_padding("isis_hello_padding", 63) -> bool;
        model isis_authentication("isis_authentication", 64) -> item::IsisAuthentication<'a>;
        scalar mtu("mtu", 65) -> i64;
        scalar no_autostate("no_autostate", 66) -> bool;
        model vrrp_ids("vrrp_ids", 67) -> item::VrrpIds<'a>;
        model ip_attached_host_route_export("ip_attached_host_route_export", 68) -> item::IpAttachedHostRouteExport<'a>;
        model ipv6_attached_host_route_export("ipv6_attached_host_route_export", 69) -> item::Ipv6AttachedHostRouteExport<'a>;
        model bfd("bfd", 70) -> item::Bfd<'a>;
        model service_policy("service_policy", 71) -> item::ServicePolicy<'a>;
        model tcp_mss_ceiling("tcp_mss_ceiling", 72) -> item::TcpMssCeiling<'a>;
        model traffic_policy("traffic_policy", 73) -> item::TrafficPolicy<'a>;
        model mpls("mpls", 74) -> item::Mpls<'a>;
        scalar ntp_serve("ntp_serve", 75) -> bool;
        scalar pvlan_mapping("pvlan_mapping", 76) -> &'a str;
        model metadata("metadata", 77) -> item::Metadata<'a>;
        scalar eos_cli("eos_cli", 78) -> &'a str;
    }
}

pub mod item {

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
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct IpAddressSecondaries {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct IpVirtualRouterAddresses {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct IpAddressVirtualSecondaries {
            scalar item (0) -> &'a str;
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

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipv6AddressVirtuals {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipv6VirtualRouterAddresses {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipv6NdCache {
            scalar dynamic_capacity("dynamic_capacity", 0) -> i64;
            scalar expire("expire", 1) -> i64;
            scalar refresh_always("refresh_always", 2) -> bool;
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

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipv6DhcpRelayDestinations {
            model item (0) -> ipv6_dhcp_relay_destinations::Item<'a>;
            primary_key_fields: [0];
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
                model source_route_export("source_route_export", 1) -> ipv4::SourceRouteExport<'a>;
                scalar field_static("static", 2) -> bool;
            }
        }

        pub mod ipv4 {

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Boundaries {
                    model item (0) -> boundaries::Item<'a>;
                    primary_key_fields: [0];
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

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct SourceRouteExport {
                    scalar enabled("enabled", 0) -> bool;
                    scalar administrative_distance("administrative_distance", 1) -> i64;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ipv6 {
                model boundaries("boundaries", 0) -> ipv6::Boundaries<'a>;
                model source_route_export("source_route_export", 1) -> ipv6::SourceRouteExport<'a>;
                scalar field_static("static", 2) -> bool;
            }
        }

        pub mod ipv6 {

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Boundaries {
                    model item (0) -> boundaries::Item<'a>;
                    primary_key_fields: [0];
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

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct SourceRouteExport {
                    scalar enabled("enabled", 0) -> bool;
                    scalar administrative_distance("administrative_distance", 1) -> i64;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipv6Ospf {
            model process("process", 0) -> ipv6_ospf::Process<'a>;
            scalar network_point_to_point("network_point_to_point", 1) -> bool;
        }
    }

    pub mod ipv6_ospf {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Process {
                scalar id("id", 0) -> i64;
                scalar area("area", 1) -> &'a str;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ospfv3 {
            model ipv4("ipv4", 0) -> ospfv3::Ipv4<'a>;
            model ipv6("ipv6", 1) -> ospfv3::Ipv6<'a>;
            scalar passive_interface("passive_interface", 2) -> bool;
            scalar network_point_to_point("network_point_to_point", 3) -> bool;
        }
    }

    pub mod ospfv3 {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ipv4 {
                scalar area("area", 0) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ipv6 {
                scalar area("area", 0) -> &'a str;
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
                scalar local_interface("local_interface", 3) -> &'a str;
                scalar bfd("bfd", 4) -> bool;
                scalar bidirectional("bidirectional", 5) -> bool;
                scalar neighbor_filter("neighbor_filter", 6) -> &'a str;
                model hello("hello", 7) -> ipv4::Hello<'a>;
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
        pub struct IpAttachedHostRouteExport {
            scalar enabled("enabled", 0) -> bool;
            scalar distance("distance", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipv6AttachedHostRouteExport {
            scalar enabled("enabled", 0) -> bool;
            scalar distance("distance", 1) -> i64;
            scalar prefix_length("prefix_length", 2) -> i64;
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
        }
    }

    pub mod service_policy {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Pbr {
                scalar input("input", 0) -> &'a str;
            }
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
        pub struct TrafficPolicy {
            scalar input("input", 0) -> &'a str;
            scalar output("output", 1) -> &'a str;
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
        pub struct Metadata {
            model tenants("tenants", 0) -> metadata::Tenants<'a>;
            model tags("tags", 1) -> metadata::Tags<'a>;
            scalar field_type("type", 2) -> &'a str;
        }
    }

    pub mod metadata {

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
    }
}
