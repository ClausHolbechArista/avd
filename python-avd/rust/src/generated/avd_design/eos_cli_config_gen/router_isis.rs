// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Timers {
        model local_convergence("local_convergence", 0) -> timers::LocalConvergence<'a>;
        model lsp("lsp", 1) -> timers::Lsp<'a>;
        model csnp("csnp", 2) -> timers::Csnp<'a>;
    }
}

pub mod timers {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct LocalConvergence {
            scalar protected_prefixes("protected_prefixes", 0) -> bool;
            scalar delay("delay", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Lsp {
            model generation("generation", 0) -> lsp::Generation<'a>;
            scalar out_delay("out_delay", 1) -> i64;
            scalar refresh_interval("refresh_interval", 2) -> i64;
            scalar min_remaining_lifetime("min_remaining_lifetime", 3) -> i64;
        }
    }

    pub mod lsp {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Generation {
                scalar interval("interval", 0) -> i64;
                scalar initial_wait_time("initial_wait_time", 1) -> i64;
                scalar wait_time("wait_time", 2) -> i64;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Csnp {
            model generation("generation", 0) -> csnp::Generation<'a>;
        }
    }

    pub mod csnp {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Generation {
                scalar interval("interval", 0) -> i64;
                scalar p2p_disabled("p2p_disabled", 1) -> bool;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SetOverloadBit {
        scalar enabled("enabled", 0) -> bool;
        model on_startup("on_startup", 1) -> set_overload_bit::OnStartup<'a>;
    }
}

pub mod set_overload_bit {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct OnStartup {
            scalar delay("delay", 0) -> i64;
            model wait_for_bgp("wait_for_bgp", 1) -> on_startup::WaitForBgp<'a>;
        }
    }

    pub mod on_startup {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct WaitForBgp {
                scalar enabled("enabled", 0) -> bool;
                scalar timeout("timeout", 1) -> i64;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Authentication {
        model both("both", 0) -> authentication::Both<'a>;
        model level_1("level_1", 1) -> authentication::Level1<'a>;
        model level_2("level_2", 2) -> authentication::Level2<'a>;
    }
}

pub mod authentication {

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
    pub struct Advertise {
        scalar passive_only("passive_only", 0) -> bool;
    }
}

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RedistributeRoutes {
        model item (0) -> redistribute_routes::Item<'a>;
    }
}

pub mod redistribute_routes {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar source_protocol("source_protocol", 0) -> &'a str;
            scalar route_map("route_map", 1) -> &'a str;
            scalar include_leaked("include_leaked", 2) -> bool;
            scalar ospf_route_type("ospf_route_type", 3) -> &'a str;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AddressFamilyIpv4 {
        scalar enabled("enabled", 0) -> bool;
        scalar maximum_paths("maximum_paths", 1) -> i64;
        scalar bfd_all_interfaces("bfd_all_interfaces", 2) -> bool;
        model fast_reroute_ti_lfa("fast_reroute_ti_lfa", 3) -> address_family_ipv4::FastRerouteTiLfa<'a>;
        model tunnel_source_labeled_unicast("tunnel_source_labeled_unicast", 4) -> address_family_ipv4::TunnelSourceLabeledUnicast<'a>;
    }
}

pub mod address_family_ipv4 {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct FastRerouteTiLfa {
            scalar mode("mode", 0) -> &'a str;
            scalar level("level", 1) -> &'a str;
            model srlg("srlg", 2) -> fast_reroute_ti_lfa::Srlg<'a>;
        }
    }

    pub mod fast_reroute_ti_lfa {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Srlg {
                scalar enable("enable", 0) -> bool;
                scalar strict("strict", 1) -> bool;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct TunnelSourceLabeledUnicast {
            scalar enabled("enabled", 0) -> bool;
            scalar rcf("rcf", 1) -> &'a str;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AddressFamilyIpv6 {
        scalar enabled("enabled", 0) -> bool;
        scalar maximum_paths("maximum_paths", 1) -> i64;
        scalar bfd_all_interfaces("bfd_all_interfaces", 2) -> bool;
        scalar multi_topology("multi_topology", 3) -> bool;
        model fast_reroute_ti_lfa("fast_reroute_ti_lfa", 4) -> address_family_ipv6::FastRerouteTiLfa<'a>;
    }
}

pub mod address_family_ipv6 {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct FastRerouteTiLfa {
            scalar mode("mode", 0) -> &'a str;
            scalar level("level", 1) -> &'a str;
            model srlg("srlg", 2) -> fast_reroute_ti_lfa::Srlg<'a>;
        }
    }

    pub mod fast_reroute_ti_lfa {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Srlg {
                scalar enable("enable", 0) -> bool;
                scalar strict("strict", 1) -> bool;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SegmentRoutingMpls {
        scalar enabled("enabled", 0) -> bool;
        scalar router_id("router_id", 1) -> &'a str;
        model prefix_segments("prefix_segments", 2) -> segment_routing_mpls::PrefixSegments<'a>;
    }
}

pub mod segment_routing_mpls {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PrefixSegments {
            model item (0) -> prefix_segments::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod prefix_segments {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar prefix("prefix", 0) -> &'a str;
                scalar index("index", 1) -> i64;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SpfInterval {
        scalar interval("interval", 0) -> i64;
        scalar interval_unit("interval_unit", 1) -> &'a str;
        scalar wait_interval("wait_interval", 2) -> i64;
        scalar hold_interval("hold_interval", 3) -> i64;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct GracefulRestart {
        scalar enabled("enabled", 0) -> bool;
        scalar restart_hold_time("restart_hold_time", 1) -> i64;
        model t2("t2", 2) -> graceful_restart::T2<'a>;
    }
}

pub mod graceful_restart {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct T2 {
            scalar level_1_wait_time("level_1_wait_time", 0) -> i64;
            scalar level_2_wait_time("level_2_wait_time", 1) -> i64;
        }
    }
}
