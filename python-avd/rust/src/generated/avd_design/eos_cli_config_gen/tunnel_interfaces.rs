// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar description("description", 1) -> &'a str;
        scalar shutdown("shutdown", 2) -> bool;
        scalar mtu("mtu", 3) -> i64;
        scalar vrf("vrf", 4) -> &'a str;
        scalar underlay_vrf("underlay_vrf", 5) -> &'a str;
        scalar ip_address("ip_address", 6) -> &'a str;
        scalar ipv6_enable("ipv6_enable", 7) -> bool;
        scalar ipv6_address("ipv6_address", 8) -> &'a str;
        model ipv6_addresses("ipv6_addresses", 9) -> item::Ipv6Addresses<'a>;
        scalar ipv6_address_auto_config("ipv6_address_auto_config", 10) -> bool;
        model ipv6_nd("ipv6_nd", 11) -> item::Ipv6Nd<'a>;
        scalar access_group_in("access_group_in", 12) -> &'a str;
        scalar access_group_out("access_group_out", 13) -> &'a str;
        scalar ipv6_access_group_in("ipv6_access_group_in", 14) -> &'a str;
        scalar ipv6_access_group_out("ipv6_access_group_out", 15) -> &'a str;
        model tcp_mss_ceiling("tcp_mss_ceiling", 16) -> item::TcpMssCeiling<'a>;
        scalar tunnel_mode("tunnel_mode", 17) -> &'a str;
        scalar source_interface("source_interface", 18) -> &'a str;
        scalar source("source", 19) -> &'a str;
        scalar destination("destination", 20) -> &'a str;
        scalar path_mtu_discovery("path_mtu_discovery", 21) -> bool;
        scalar ipsec_profile("ipsec_profile", 22) -> &'a str;
        scalar nat_profile("nat_profile", 23) -> &'a str;
        scalar eos_cli("eos_cli", 24) -> &'a str;
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
        pub struct TcpMssCeiling {
            scalar ipv4("ipv4", 0) -> i64;
            scalar ipv6("ipv6", 1) -> i64;
            scalar direction("direction", 2) -> &'a str;
        }
    }
}
