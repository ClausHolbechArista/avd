// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct CpuTrafficPolicy {
        model vrf_all("vrf_all", 0) -> cpu_traffic_policy::VrfAll<'a>;
        scalar enforcement_ip_ttl_expired("enforcement_ip_ttl_expired", 1) -> bool;
        scalar fragment_implicit_permit_disabled("fragment_implicit_permit_disabled", 2) -> bool;
    }
}

pub mod cpu_traffic_policy {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct VrfAll {
            scalar name("name", 0) -> &'a str;
            scalar enforcement_management("enforcement_management", 1) -> bool;
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
            model cpu("cpu", 1) -> item::Cpu<'a>;
            scalar traffic_policy_input_physical("traffic_policy_input_physical", 2) -> &'a str;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Cpu {
                model traffic_policy("traffic_policy", 0) -> cpu::TrafficPolicy<'a>;
            }
        }

        pub mod cpu {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct TrafficPolicy {
                    scalar name("name", 0) -> &'a str;
                    scalar enforcement_management("enforcement_management", 1) -> bool;
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Options {
        scalar counter_per_interface("counter_per_interface", 0) -> bool;
        scalar counter_interface_poll_interval("counter_interface_poll_interval", 1) -> i64;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct FieldSets {
        model ipv4("ipv4", 0) -> field_sets::Ipv4<'a>;
        model ipv6("ipv6", 1) -> field_sets::Ipv6<'a>;
        model ports("ports", 2) -> field_sets::Ports<'a>;
    }
}

pub mod field_sets {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipv4 {
            model item (0) -> ipv4::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod ipv4 {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                model prefixes("prefixes", 1) -> item::Prefixes<'a>;
                model field_except("except", 2) -> item::Except<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Prefixes {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Except {
                    scalar item (0) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipv6 {
            model item (0) -> ipv6::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod ipv6 {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                model prefixes("prefixes", 1) -> item::Prefixes<'a>;
                model field_except("except", 2) -> item::Except<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Prefixes {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Except {
                    scalar item (0) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ports {
            model item (0) -> ports::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod ports {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar port_range("port_range", 1) -> &'a str;
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
            model counters("counters", 1) -> item::Counters<'a>;
            model matches("matches", 2) -> item::Matches<'a>;
            model default_actions("default_actions", 3) -> item::DefaultActions<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Counters {
                scalar item (0) -> &'a str;
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Matches {
                model item (0) -> matches::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod matches {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    scalar field_type("type", 1) -> &'a str;
                    model source("source", 2) -> item::Source<'a>;
                    model destination("destination", 3) -> item::Destination<'a>;
                    scalar ttl("ttl", 4) -> &'a str;
                    model fragment("fragment", 5) -> item::Fragment<'a>;
                    model protocols("protocols", 6) -> item::Protocols<'a>;
                    model packet_type("packet_type", 7) -> item::PacketType<'a>;
                    model actions("actions", 8) -> item::Actions<'a>;
                }
            }

            pub mod item {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Source {
                        model prefixes("prefixes", 0) -> source::Prefixes<'a>;
                        model prefix_lists("prefix_lists", 1) -> source::PrefixLists<'a>;
                    }
                }

                pub mod source {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Prefixes {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct PrefixLists {
                            scalar item (0) -> &'a str;
                        }
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Destination {
                        model prefixes("prefixes", 0) -> destination::Prefixes<'a>;
                        model prefix_lists("prefix_lists", 1) -> destination::PrefixLists<'a>;
                    }
                }

                pub mod destination {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Prefixes {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct PrefixLists {
                            scalar item (0) -> &'a str;
                        }
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Fragment {
                        scalar offset("offset", 0) -> &'a str;
                    }
                }

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
                            scalar src_port("src_port", 1) -> &'a str;
                            scalar dst_port("dst_port", 2) -> &'a str;
                            scalar src_field("src_field", 3) -> &'a str;
                            scalar dst_field("dst_field", 4) -> &'a str;
                            model flags("flags", 5) -> item::Flags<'a>;
                            model icmp_type("icmp_type", 6) -> item::IcmpType<'a>;
                            scalar enforce_gtsm("enforce_gtsm", 7) -> bool;
                        }
                    }

                    pub mod item {

                        ::validation::define_archive_list_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Flags {
                                scalar item (0) -> &'a str;
                            }
                        }

                        ::validation::define_archive_list_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct IcmpType {
                                scalar item (0) -> &'a str;
                            }
                        }
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct PacketType {
                        scalar vxlan("vxlan", 0) -> &'a str;
                        scalar multicast("multicast", 1) -> bool;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Actions {
                        scalar dscp("dscp", 0) -> i64;
                        scalar traffic_class("traffic_class", 1) -> i64;
                        scalar count("count", 2) -> &'a str;
                        scalar drop("drop", 3) -> bool;
                        scalar log("log", 4) -> bool;
                        model redirect("redirect", 5) -> actions::Redirect<'a>;
                    }
                }

                pub mod actions {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Redirect {
                            model aggregation_groups("aggregation_groups", 0) -> redirect::AggregationGroups<'a>;
                            scalar interface("interface", 1) -> &'a str;
                            model next_hop("next_hop", 2) -> redirect::NextHop<'a>;
                        }
                    }

                    pub mod redirect {

                        ::validation::define_archive_list_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct AggregationGroups {
                                scalar item (0) -> &'a str;
                            }
                        }

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct NextHop {
                                model ipv4_addresses("ipv4_addresses", 0) -> next_hop::Ipv4Addresses<'a>;
                                model ipv6_addresses("ipv6_addresses", 1) -> next_hop::Ipv6Addresses<'a>;
                                scalar vrf("vrf", 2) -> &'a str;
                                model groups("groups", 3) -> next_hop::Groups<'a>;
                                model recursive_ipv4_addresses("recursive_ipv4_addresses", 4) -> next_hop::RecursiveIpv4Addresses<'a>;
                                model recursive_ipv6_addresses("recursive_ipv6_addresses", 5) -> next_hop::RecursiveIpv6Addresses<'a>;
                                scalar ttl("ttl", 6) -> i64;
                            }
                        }

                        pub mod next_hop {

                            ::validation::define_archive_list_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct Ipv4Addresses {
                                    scalar item (0) -> &'a str;
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
                                pub struct Groups {
                                    scalar item (0) -> &'a str;
                                }
                            }

                            ::validation::define_archive_list_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct RecursiveIpv4Addresses {
                                    scalar item (0) -> &'a str;
                                }
                            }

                            ::validation::define_archive_list_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct RecursiveIpv6Addresses {
                                    scalar item (0) -> &'a str;
                                }
                            }
                        }
                    }
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct DefaultActions {
                model ipv4("ipv4", 0) -> default_actions::Ipv4<'a>;
                model ipv6("ipv6", 1) -> default_actions::Ipv6<'a>;
            }
        }

        pub mod default_actions {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Ipv4 {
                    scalar dscp("dscp", 0) -> i64;
                    scalar traffic_class("traffic_class", 1) -> i64;
                    scalar count("count", 2) -> &'a str;
                    scalar drop("drop", 3) -> bool;
                    scalar log("log", 4) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Ipv6 {
                    scalar dscp("dscp", 0) -> i64;
                    scalar traffic_class("traffic_class", 1) -> i64;
                    scalar count("count", 2) -> &'a str;
                    scalar drop("drop", 3) -> bool;
                    scalar log("log", 4) -> bool;
                }
            }
        }
    }
}
