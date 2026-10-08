// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar profile("profile", 0) -> &'a str;
        scalar parent_profile("parent_profile", 1) -> &'a str;
        model nodes("nodes", 2) -> item::Nodes<'a>;
        scalar name("name", 3) -> &'a str;
        scalar enabled("enabled", 4) -> bool;
        scalar description("description", 5) -> &'a str;
        scalar arp_gratuitous_accept("arp_gratuitous_accept", 6) -> bool;
        scalar ip_address("ip_address", 7) -> &'a str;
        model ip_address_secondaries("ip_address_secondaries", 8) -> item::IpAddressSecondaries<'a>;
        scalar ipv6_address("ipv6_address", 9) -> &'a str;
        scalar ipv6_enable("ipv6_enable", 10) -> bool;
        scalar ip_address_virtual("ip_address_virtual", 11) -> &'a str;
        model ipv6_address_virtuals("ipv6_address_virtuals", 12) -> item::Ipv6AddressVirtuals<'a>;
        model ipv6_nd("ipv6_nd", 13) -> item::Ipv6Nd<'a>;
        model ipv6_dhcp_relay("ipv6_dhcp_relay", 14) -> item::Ipv6DhcpRelay<'a>;
        model ip_address_virtual_secondaries("ip_address_virtual_secondaries", 15) -> item::IpAddressVirtualSecondaries<'a>;
        model ip_virtual_router_addresses("ip_virtual_router_addresses", 16) -> item::IpVirtualRouterAddresses<'a>;
        model ipv6_virtual_router_addresses("ipv6_virtual_router_addresses", 17) -> item::Ipv6VirtualRouterAddresses<'a>;
        scalar ipv4_acl_in("ipv4_acl_in", 18) -> &'a str;
        scalar ipv4_acl_out("ipv4_acl_out", 19) -> &'a str;
        scalar ipv6_acl_in("ipv6_acl_in", 20) -> &'a str;
        scalar ipv6_acl_out("ipv6_acl_out", 21) -> &'a str;
        model ip_helpers("ip_helpers", 22) -> item::IpHelpers<'a>;
        model static_routes("static_routes", 23) -> item::StaticRoutes<'a>;
        model ipv6_static_routes("ipv6_static_routes", 24) -> item::Ipv6StaticRoutes<'a>;
        scalar vni_override("vni_override", 25) -> i64;
        scalar rt_override("rt_override", 26) -> &'a str;
        scalar rd_override("rd_override", 27) -> &'a str;
        model trunk_groups("trunk_groups", 28) -> item::TrunkGroups<'a>;
        model evpn_l2_multicast("evpn_l2_multicast", 29) -> item::EvpnL2Multicast<'a>;
        scalar evpn_redistribute_router_mac_system("evpn_redistribute_router_mac_system", 30) -> bool;
        model vxlan_flood_multicast("vxlan_flood_multicast", 31) -> item::VxlanFloodMulticast<'a>;
        model evpn_l3_multicast("evpn_l3_multicast", 32) -> item::EvpnL3Multicast<'a>;
        model igmp_snooping("igmp_snooping", 33) -> item::IgmpSnooping<'a>;
        scalar igmp_snooping_enabled("igmp_snooping_enabled", 34) -> bool;
        model igmp_snooping_querier("igmp_snooping_querier", 35) -> item::IgmpSnoopingQuerier<'a>;
        scalar vxlan("vxlan", 36) -> bool;
        scalar spanning_tree_priority("spanning_tree_priority", 37) -> i64;
        scalar mtu("mtu", 38) -> i64;
        model ospf("ospf", 39) -> item::Ospf<'a>;
        model bgp("bgp", 40) -> item::Bgp<'a>;
        scalar raw_eos_cli("raw_eos_cli", 41) -> &'a str;
        model structured_config("structured_config", 42) -> super::super::eos_cli_config_gen::vlan_interfaces::Item<'a>;
        scalar evpn_l2_multi_domain("evpn_l2_multi_domain", 43) -> bool;
    }
}

pub mod item {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Nodes {
            model item (0) -> nodes::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod nodes {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar node("node", 0) -> &'a str;
                scalar name("name", 1) -> &'a str;
                scalar enabled("enabled", 2) -> bool;
                scalar description("description", 3) -> &'a str;
                scalar arp_gratuitous_accept("arp_gratuitous_accept", 4) -> bool;
                scalar ip_address("ip_address", 5) -> &'a str;
                model ip_address_secondaries("ip_address_secondaries", 6) -> item::IpAddressSecondaries<'a>;
                scalar ipv6_address("ipv6_address", 7) -> &'a str;
                scalar ipv6_enable("ipv6_enable", 8) -> bool;
                scalar ip_address_virtual("ip_address_virtual", 9) -> &'a str;
                model ipv6_address_virtuals("ipv6_address_virtuals", 10) -> item::Ipv6AddressVirtuals<'a>;
                model ipv6_nd("ipv6_nd", 11) -> item::Ipv6Nd<'a>;
                model ipv6_dhcp_relay("ipv6_dhcp_relay", 12) -> item::Ipv6DhcpRelay<'a>;
                model ip_address_virtual_secondaries("ip_address_virtual_secondaries", 13) -> item::IpAddressVirtualSecondaries<'a>;
                model ip_virtual_router_addresses("ip_virtual_router_addresses", 14) -> item::IpVirtualRouterAddresses<'a>;
                model ipv6_virtual_router_addresses("ipv6_virtual_router_addresses", 15) -> item::Ipv6VirtualRouterAddresses<'a>;
                scalar ipv4_acl_in("ipv4_acl_in", 16) -> &'a str;
                scalar ipv4_acl_out("ipv4_acl_out", 17) -> &'a str;
                scalar ipv6_acl_in("ipv6_acl_in", 18) -> &'a str;
                scalar ipv6_acl_out("ipv6_acl_out", 19) -> &'a str;
                model ip_helpers("ip_helpers", 20) -> item::IpHelpers<'a>;
                model static_routes("static_routes", 21) -> item::StaticRoutes<'a>;
                model ipv6_static_routes("ipv6_static_routes", 22) -> item::Ipv6StaticRoutes<'a>;
                scalar vni_override("vni_override", 23) -> i64;
                scalar rt_override("rt_override", 24) -> &'a str;
                scalar rd_override("rd_override", 25) -> &'a str;
                model trunk_groups("trunk_groups", 26) -> item::TrunkGroups<'a>;
                model evpn_l2_multicast("evpn_l2_multicast", 27) -> item::EvpnL2Multicast<'a>;
                scalar evpn_redistribute_router_mac_system("evpn_redistribute_router_mac_system", 28) -> bool;
                model vxlan_flood_multicast("vxlan_flood_multicast", 29) -> item::VxlanFloodMulticast<'a>;
                model evpn_l3_multicast("evpn_l3_multicast", 30) -> item::EvpnL3Multicast<'a>;
                model igmp_snooping("igmp_snooping", 31) -> item::IgmpSnooping<'a>;
                scalar igmp_snooping_enabled("igmp_snooping_enabled", 32) -> bool;
                model igmp_snooping_querier("igmp_snooping_querier", 33) -> item::IgmpSnoopingQuerier<'a>;
                scalar vxlan("vxlan", 34) -> bool;
                scalar spanning_tree_priority("spanning_tree_priority", 35) -> i64;
                scalar mtu("mtu", 36) -> i64;
                model ospf("ospf", 37) -> item::Ospf<'a>;
                model bgp("bgp", 38) -> item::Bgp<'a>;
                scalar raw_eos_cli("raw_eos_cli", 39) -> &'a str;
                model structured_config("structured_config", 40) -> super::super::super::super::eos_cli_config_gen::vlan_interfaces::Item<'a>;
                scalar evpn_l2_multi_domain("evpn_l2_multi_domain", 41) -> bool;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct IpAddressSecondaries {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Ipv6AddressVirtuals {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Ipv6Nd {
                    scalar advertise_ipv6_address_virtuals("advertise_ipv6_address_virtuals", 0) -> bool;
                    scalar valid_lifetime("valid_lifetime", 1) -> &'a str;
                    scalar preferred_lifetime("preferred_lifetime", 2) -> &'a str;
                    model ra_dns_servers("ra_dns_servers", 3) -> ipv6_nd::RaDnsServers<'a>;
                }
            }

            pub mod ipv6_nd {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct RaDnsServers {
                        model servers("servers", 0) -> ra_dns_servers::Servers<'a>;
                        scalar dns_servers_lifetime("dns_servers_lifetime", 1) -> i64;
                    }
                }

                pub mod ra_dns_servers {

                    ::validation::define_archive_indexed_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Servers {
                            model item (0) -> servers::Item<'a>;
                            primary_key_fields: [0];
                        }
                    }

                    pub mod servers {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Item {
                                scalar address("address", 0) -> &'a str;
                                scalar lifetime("lifetime", 1) -> i64;
                            }
                        }
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Ipv6DhcpRelay {
                    model destinations("destinations", 0) -> ipv6_dhcp_relay::Destinations<'a>;
                }
            }

            pub mod ipv6_dhcp_relay {

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
                            scalar address("address", 0) -> &'a str;
                            scalar vrf("vrf", 1) -> &'a str;
                            scalar local_interface("local_interface", 2) -> &'a str;
                            scalar source_address("source_address", 3) -> &'a str;
                            scalar link_address("link_address", 4) -> &'a str;
                        }
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct IpAddressVirtualSecondaries {
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
                pub struct Ipv6VirtualRouterAddresses {
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
                        scalar source_vrf("source_vrf", 2) -> &'a str;
                    }
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
                        scalar next_hop("next_hop", 1) -> &'a str;
                        scalar track_bfd("track_bfd", 2) -> bool;
                        scalar distance("distance", 3) -> i64;
                        scalar tag("tag", 4) -> i64;
                        scalar name("name", 5) -> &'a str;
                        scalar metric("metric", 6) -> i64;
                        scalar interface("interface", 7) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Ipv6StaticRoutes {
                    model item (0) -> ipv6_static_routes::Item<'a>;
                }
            }

            pub mod ipv6_static_routes {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar prefix("prefix", 0) -> &'a str;
                        scalar next_hop("next_hop", 1) -> &'a str;
                        scalar track_bfd("track_bfd", 2) -> bool;
                        scalar distance("distance", 3) -> i64;
                        scalar tag("tag", 4) -> i64;
                        scalar name("name", 5) -> &'a str;
                        scalar metric("metric", 6) -> i64;
                        scalar interface("interface", 7) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct TrunkGroups {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct EvpnL2Multicast {
                    scalar enabled("enabled", 0) -> bool;
                    scalar always_redistribute_igmp("always_redistribute_igmp", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct VxlanFloodMulticast {
                    scalar enabled("enabled", 0) -> bool;
                    scalar underlay_multicast_group("underlay_multicast_group", 1) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct EvpnL3Multicast {
                    scalar enabled("enabled", 0) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct IgmpSnooping {
                    scalar enabled("enabled", 0) -> bool;
                    model querier("querier", 1) -> igmp_snooping::Querier<'a>;
                    scalar fast_leave("fast_leave", 2) -> bool;
                }
            }

            pub mod igmp_snooping {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Querier {
                        scalar enabled("enabled", 0) -> bool;
                        scalar source_address("source_address", 1) -> &'a str;
                        scalar version("version", 2) -> i64;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct IgmpSnoopingQuerier {
                    scalar enabled("enabled", 0) -> bool;
                    scalar source_address("source_address", 1) -> &'a str;
                    scalar version("version", 2) -> i64;
                    scalar fast_leave("fast_leave", 3) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Ospf {
                    scalar enabled("enabled", 0) -> bool;
                    scalar point_to_point("point_to_point", 1) -> bool;
                    scalar area("area", 2) -> &'a str;
                    scalar cost("cost", 3) -> i64;
                    scalar authentication("authentication", 4) -> &'a str;
                    scalar simple_auth_key("simple_auth_key", 5) -> &'a str;
                    scalar cleartext_simple_auth_key("cleartext_simple_auth_key", 6) -> &'a str;
                    model message_digest_keys("message_digest_keys", 7) -> ospf::MessageDigestKeys<'a>;
                }
            }

            pub mod ospf {

                ::validation::define_archive_indexed_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct MessageDigestKeys {
                        model item (0) -> message_digest_keys::Item<'a>;
                        primary_key_fields: [0];
                    }
                }

                pub mod message_digest_keys {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            scalar id("id", 0) -> i64;
                            scalar hash_algorithm("hash_algorithm", 1) -> &'a str;
                            scalar key("key", 2) -> &'a str;
                            scalar cleartext_key("cleartext_key", 3) -> &'a str;
                        }
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Bgp {
                    model structured_config("structured_config", 0) -> super::super::super::super::super::eos_cli_config_gen::router_bgp::vlans::Item<'a>;
                    scalar raw_eos_cli("raw_eos_cli", 1) -> &'a str;
                }
            }

            pub mod bgp {
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
        pub struct Ipv6AddressVirtuals {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipv6Nd {
            scalar advertise_ipv6_address_virtuals("advertise_ipv6_address_virtuals", 0) -> bool;
            scalar valid_lifetime("valid_lifetime", 1) -> &'a str;
            scalar preferred_lifetime("preferred_lifetime", 2) -> &'a str;
            model ra_dns_servers("ra_dns_servers", 3) -> ipv6_nd::RaDnsServers<'a>;
        }
    }

    pub mod ipv6_nd {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct RaDnsServers {
                model servers("servers", 0) -> ra_dns_servers::Servers<'a>;
                scalar dns_servers_lifetime("dns_servers_lifetime", 1) -> i64;
            }
        }

        pub mod ra_dns_servers {

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Servers {
                    model item (0) -> servers::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod servers {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar address("address", 0) -> &'a str;
                        scalar lifetime("lifetime", 1) -> i64;
                    }
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipv6DhcpRelay {
            model destinations("destinations", 0) -> ipv6_dhcp_relay::Destinations<'a>;
        }
    }

    pub mod ipv6_dhcp_relay {

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
                    scalar address("address", 0) -> &'a str;
                    scalar vrf("vrf", 1) -> &'a str;
                    scalar local_interface("local_interface", 2) -> &'a str;
                    scalar source_address("source_address", 3) -> &'a str;
                    scalar link_address("link_address", 4) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct IpAddressVirtualSecondaries {
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
        pub struct Ipv6VirtualRouterAddresses {
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
                scalar source_vrf("source_vrf", 2) -> &'a str;
            }
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
                scalar next_hop("next_hop", 1) -> &'a str;
                scalar track_bfd("track_bfd", 2) -> bool;
                scalar distance("distance", 3) -> i64;
                scalar tag("tag", 4) -> i64;
                scalar name("name", 5) -> &'a str;
                scalar metric("metric", 6) -> i64;
                scalar interface("interface", 7) -> &'a str;
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipv6StaticRoutes {
            model item (0) -> ipv6_static_routes::Item<'a>;
        }
    }

    pub mod ipv6_static_routes {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar prefix("prefix", 0) -> &'a str;
                scalar next_hop("next_hop", 1) -> &'a str;
                scalar track_bfd("track_bfd", 2) -> bool;
                scalar distance("distance", 3) -> i64;
                scalar tag("tag", 4) -> i64;
                scalar name("name", 5) -> &'a str;
                scalar metric("metric", 6) -> i64;
                scalar interface("interface", 7) -> &'a str;
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct TrunkGroups {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct EvpnL2Multicast {
            scalar enabled("enabled", 0) -> bool;
            scalar always_redistribute_igmp("always_redistribute_igmp", 1) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct VxlanFloodMulticast {
            scalar enabled("enabled", 0) -> bool;
            scalar underlay_multicast_group("underlay_multicast_group", 1) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct EvpnL3Multicast {
            scalar enabled("enabled", 0) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct IgmpSnooping {
            scalar enabled("enabled", 0) -> bool;
            model querier("querier", 1) -> igmp_snooping::Querier<'a>;
            scalar fast_leave("fast_leave", 2) -> bool;
        }
    }

    pub mod igmp_snooping {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Querier {
                scalar enabled("enabled", 0) -> bool;
                scalar source_address("source_address", 1) -> &'a str;
                scalar version("version", 2) -> i64;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct IgmpSnoopingQuerier {
            scalar enabled("enabled", 0) -> bool;
            scalar source_address("source_address", 1) -> &'a str;
            scalar version("version", 2) -> i64;
            scalar fast_leave("fast_leave", 3) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ospf {
            scalar enabled("enabled", 0) -> bool;
            scalar point_to_point("point_to_point", 1) -> bool;
            scalar area("area", 2) -> &'a str;
            scalar cost("cost", 3) -> i64;
            scalar authentication("authentication", 4) -> &'a str;
            scalar simple_auth_key("simple_auth_key", 5) -> &'a str;
            scalar cleartext_simple_auth_key("cleartext_simple_auth_key", 6) -> &'a str;
            model message_digest_keys("message_digest_keys", 7) -> ospf::MessageDigestKeys<'a>;
        }
    }

    pub mod ospf {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct MessageDigestKeys {
                model item (0) -> message_digest_keys::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod message_digest_keys {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar id("id", 0) -> i64;
                    scalar hash_algorithm("hash_algorithm", 1) -> &'a str;
                    scalar key("key", 2) -> &'a str;
                    scalar cleartext_key("cleartext_key", 3) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Bgp {
            model structured_config("structured_config", 0) -> super::super::super::eos_cli_config_gen::router_bgp::vlans::Item<'a>;
            scalar raw_eos_cli("raw_eos_cli", 1) -> &'a str;
        }
    }

    pub mod bgp {
    }
}
