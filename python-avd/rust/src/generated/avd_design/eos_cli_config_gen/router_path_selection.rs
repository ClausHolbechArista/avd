// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MtuDiscoveryHosts {
        scalar enabled("enabled", 0) -> bool;
        scalar fragmentation_needed_rate_limit("fragmentation_needed_rate_limit", 1) -> i64;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct PathGroups {
        model item (0) -> path_groups::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod path_groups {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar id("id", 1) -> i64;
            scalar ipsec_profile("ipsec_profile", 2) -> &'a str;
            scalar flow_assignment("flow_assignment", 3) -> &'a str;
            model local_interfaces("local_interfaces", 4) -> item::LocalInterfaces<'a>;
            model local_ips("local_ips", 5) -> item::LocalIps<'a>;
            model dynamic_peers("dynamic_peers", 6) -> item::DynamicPeers<'a>;
            model static_peers("static_peers", 7) -> item::StaticPeers<'a>;
            model keepalive("keepalive", 8) -> item::Keepalive<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct LocalInterfaces {
                model item (0) -> local_interfaces::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod local_interfaces {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    scalar public_address("public_address", 1) -> &'a str;
                    model stun("stun", 2) -> item::Stun<'a>;
                }
            }

            pub mod item {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Stun {
                        model server_profiles("server_profiles", 0) -> stun::ServerProfiles<'a>;
                    }
                }

                pub mod stun {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct ServerProfiles {
                            scalar item (0) -> &'a str;
                        }
                    }
                }
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct LocalIps {
                model item (0) -> local_ips::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod local_ips {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar ip_address("ip_address", 0) -> &'a str;
                    scalar public_address("public_address", 1) -> &'a str;
                    model stun("stun", 2) -> item::Stun<'a>;
                }
            }

            pub mod item {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Stun {
                        model server_profiles("server_profiles", 0) -> stun::ServerProfiles<'a>;
                    }
                }

                pub mod stun {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct ServerProfiles {
                            scalar item (0) -> &'a str;
                        }
                    }
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct DynamicPeers {
                scalar enabled("enabled", 0) -> bool;
                scalar ip_local("ip_local", 1) -> bool;
                scalar ipsec("ipsec", 2) -> bool;
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct StaticPeers {
                model item (0) -> static_peers::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod static_peers {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar router_ip("router_ip", 0) -> &'a str;
                    scalar name("name", 1) -> &'a str;
                    model ipv4_addresses("ipv4_addresses", 2) -> item::Ipv4Addresses<'a>;
                }
            }

            pub mod item {

                ::validation::define_archive_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Ipv4Addresses {
                        scalar item (0) -> &'a str;
                    }
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Keepalive {
                scalar auto("auto", 0) -> bool;
                scalar interval("interval", 1) -> i64;
                scalar failure_threshold("failure_threshold", 2) -> i64;
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct LoadBalancePolicies {
        model item (0) -> load_balance_policies::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod load_balance_policies {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar lowest_hop_count("lowest_hop_count", 1) -> bool;
            scalar jitter("jitter", 2) -> i64;
            scalar latency("latency", 3) -> i64;
            scalar loss_rate("loss_rate", 4) -> &'a str;
            model path_groups("path_groups", 5) -> item::PathGroups<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct PathGroups {
                model item (0) -> path_groups::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod path_groups {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    scalar priority("priority", 1) -> i64;
                }
            }
        }
    }
}

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
            model default_match("default_match", 1) -> item::DefaultMatch<'a>;
            model rules("rules", 2) -> item::Rules<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct DefaultMatch {
                scalar load_balance("load_balance", 0) -> &'a str;
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Rules {
                model item (0) -> rules::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod rules {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar id("id", 0) -> i64;
                    scalar application_profile("application_profile", 1) -> &'a str;
                    scalar load_balance("load_balance", 2) -> &'a str;
                }
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Vrfs {
        model item (0) -> vrfs::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod vrfs {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar path_selection_policy("path_selection_policy", 1) -> &'a str;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct TcpMssCeiling {
        scalar ipv4("ipv4", 0) -> &'a str;
        scalar direction("direction", 1) -> &'a str;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Interfaces {
        model item (0) -> interfaces::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod interfaces {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model metric_bandwidth("metric_bandwidth", 1) -> item::MetricBandwidth<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct MetricBandwidth {
                scalar receive("receive", 0) -> i64;
                scalar transmit("transmit", 1) -> i64;
            }
        }
    }
}
