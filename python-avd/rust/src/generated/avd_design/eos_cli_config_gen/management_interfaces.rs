// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar description("description", 1) -> &'a str;
        scalar shutdown("shutdown", 2) -> bool;
        scalar speed("speed", 3) -> &'a str;
        scalar mtu("mtu", 4) -> i64;
        scalar vrf("vrf", 5) -> &'a str;
        scalar ip_address("ip_address", 6) -> &'a str;
        scalar ipv6_enable("ipv6_enable", 7) -> bool;
        scalar ipv6_address("ipv6_address", 8) -> &'a str;
        model ipv6_addresses("ipv6_addresses", 9) -> item::Ipv6Addresses<'a>;
        scalar ipv6_address_auto_config("ipv6_address_auto_config", 10) -> bool;
        model ipv6_nd("ipv6_nd", 11) -> item::Ipv6Nd<'a>;
        scalar field_type("type", 12) -> &'a str;
        scalar gateway("gateway", 13) -> &'a str;
        scalar ipv6_gateway("ipv6_gateway", 14) -> &'a str;
        scalar mac_address("mac_address", 15) -> &'a str;
        scalar dhcp_client_accept_default_route("dhcp_client_accept_default_route", 16) -> bool;
        model lldp("lldp", 17) -> item::Lldp<'a>;
        model redundancy("redundancy", 18) -> item::Redundancy<'a>;
        scalar eos_cli("eos_cli", 19) -> &'a str;
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
        pub struct Lldp {
            scalar transmit("transmit", 0) -> bool;
            scalar receive("receive", 1) -> bool;
            scalar ztp_vlan("ztp_vlan", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Redundancy {
            scalar fallback_delay("fallback_delay", 0) -> &'a str;
            model monitor("monitor", 1) -> redundancy::Monitor<'a>;
            model supervisor_1("supervisor_1", 2) -> redundancy::Supervisor1<'a>;
            model supervisor_2("supervisor_2", 3) -> redundancy::Supervisor2<'a>;
        }
    }

    pub mod redundancy {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Monitor {
                scalar link_state("link_state", 0) -> bool;
                model neighbor("neighbor", 1) -> monitor::Neighbor<'a>;
            }
        }

        pub mod monitor {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Neighbor {
                    scalar ipv6_address("ipv6_address", 0) -> &'a str;
                    scalar interval("interval", 1) -> i64;
                    scalar multiplier("multiplier", 2) -> i64;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Supervisor1 {
                scalar primary_management_interface("primary_management_interface", 0) -> &'a str;
                model backup_management_interfaces("backup_management_interfaces", 1) -> supervisor_1::BackupManagementInterfaces<'a>;
            }
        }

        pub mod supervisor_1 {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct BackupManagementInterfaces {
                    scalar item (0) -> &'a str;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Supervisor2 {
                scalar primary_management_interface("primary_management_interface", 0) -> &'a str;
                model backup_management_interfaces("backup_management_interfaces", 1) -> supervisor_2::BackupManagementInterfaces<'a>;
            }
        }

        pub mod supervisor_2 {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct BackupManagementInterfaces {
                    scalar item (0) -> &'a str;
                }
            }
        }
    }
}
