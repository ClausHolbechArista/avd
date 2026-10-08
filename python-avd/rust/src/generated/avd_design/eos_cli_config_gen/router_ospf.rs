// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ProcessIds {
        model item (0) -> process_ids::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod process_ids {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar id("id", 0) -> i64;
            scalar vrf("vrf", 1) -> &'a str;
            scalar passive_interface_default("passive_interface_default", 2) -> bool;
            scalar router_id("router_id", 3) -> &'a str;
            model distance("distance", 4) -> item::Distance<'a>;
            scalar log_adjacency_changes_detail("log_adjacency_changes_detail", 5) -> bool;
            model network_prefixes("network_prefixes", 6) -> item::NetworkPrefixes<'a>;
            scalar bfd_enable("bfd_enable", 7) -> bool;
            scalar bfd_adjacency_state_any("bfd_adjacency_state_any", 8) -> bool;
            model no_passive_interfaces("no_passive_interfaces", 9) -> item::NoPassiveInterfaces<'a>;
            model distribute_list_in("distribute_list_in", 10) -> item::DistributeListIn<'a>;
            scalar max_lsa("max_lsa", 11) -> i64;
            model timers("timers", 12) -> item::Timers<'a>;
            model default_information_originate("default_information_originate", 13) -> item::DefaultInformationOriginate<'a>;
            model summary_addresses("summary_addresses", 14) -> item::SummaryAddresses<'a>;
            model redistribute("redistribute", 15) -> item::Redistribute<'a>;
            scalar auto_cost_reference_bandwidth("auto_cost_reference_bandwidth", 16) -> i64;
            model areas("areas", 17) -> item::Areas<'a>;
            scalar maximum_paths("maximum_paths", 18) -> i64;
            model max_metric("max_metric", 19) -> item::MaxMetric<'a>;
            model graceful_restart("graceful_restart", 20) -> item::GracefulRestart<'a>;
            scalar graceful_restart_helper("graceful_restart_helper", 21) -> bool;
            scalar mpls_ldp_sync_default("mpls_ldp_sync_default", 22) -> bool;
            model segment_routing_mpls("segment_routing_mpls", 23) -> item::SegmentRoutingMpls<'a>;
            scalar eos_cli("eos_cli", 24) -> &'a str;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Distance {
                scalar external("external", 0) -> i64;
                scalar inter_area("inter_area", 1) -> i64;
                scalar intra_area("intra_area", 2) -> i64;
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct NetworkPrefixes {
                model item (0) -> network_prefixes::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod network_prefixes {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar ipv4_prefix("ipv4_prefix", 0) -> &'a str;
                    scalar area("area", 1) -> &'a str;
                }
            }
        }

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct NoPassiveInterfaces {
                scalar item (0) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct DistributeListIn {
                scalar route_map("route_map", 0) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Timers {
                model lsa("lsa", 0) -> timers::Lsa<'a>;
                model spf_delay("spf_delay", 1) -> timers::SpfDelay<'a>;
            }
        }

        pub mod timers {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Lsa {
                    scalar rx_min_interval("rx_min_interval", 0) -> i64;
                    model tx_delay("tx_delay", 1) -> lsa::TxDelay<'a>;
                }
            }

            pub mod lsa {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct TxDelay {
                        scalar initial("initial", 0) -> i64;
                        scalar min("min", 1) -> i64;
                        scalar max("max", 2) -> i64;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct SpfDelay {
                    scalar initial("initial", 0) -> i64;
                    scalar min("min", 1) -> i64;
                    scalar max("max", 2) -> i64;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct DefaultInformationOriginate {
                scalar always("always", 0) -> bool;
                scalar metric("metric", 1) -> i64;
                scalar metric_type("metric_type", 2) -> i64;
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct SummaryAddresses {
                model item (0) -> summary_addresses::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod summary_addresses {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar prefix("prefix", 0) -> &'a str;
                    scalar tag("tag", 1) -> i64;
                    scalar attribute_map("attribute_map", 2) -> &'a str;
                    scalar not_advertise("not_advertise", 3) -> bool;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Redistribute {
                model field_static("static", 0) -> redistribute::FieldStatic<'a>;
                model connected("connected", 1) -> redistribute::Connected<'a>;
                model bgp("bgp", 2) -> redistribute::Bgp<'a>;
            }
        }

        pub mod redistribute {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct FieldStatic {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                    scalar include_leaked("include_leaked", 2) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Connected {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                    scalar include_leaked("include_leaked", 2) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Bgp {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                    scalar include_leaked("include_leaked", 2) -> bool;
                }
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Areas {
                model item (0) -> areas::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod areas {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar id("id", 0) -> &'a str;
                    model filter("filter", 1) -> item::Filter<'a>;
                    scalar field_type("type", 2) -> &'a str;
                    scalar no_summary("no_summary", 3) -> bool;
                    scalar nssa_only("nssa_only", 4) -> bool;
                    model default_information_originate("default_information_originate", 5) -> item::DefaultInformationOriginate<'a>;
                }
            }

            pub mod item {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Filter {
                        model networks("networks", 0) -> filter::Networks<'a>;
                        scalar prefix_list("prefix_list", 1) -> &'a str;
                    }
                }

                pub mod filter {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Networks {
                            scalar item (0) -> &'a str;
                        }
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct DefaultInformationOriginate {
                        scalar metric("metric", 0) -> i64;
                        scalar metric_type("metric_type", 1) -> i64;
                    }
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct MaxMetric {
                model router_lsa("router_lsa", 0) -> max_metric::RouterLsa<'a>;
            }
        }

        pub mod max_metric {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct RouterLsa {
                    model external_lsa("external_lsa", 0) -> router_lsa::ExternalLsa<'a>;
                    scalar include_stub("include_stub", 1) -> bool;
                    scalar on_startup("on_startup", 2) -> &'a str;
                    model summary_lsa("summary_lsa", 3) -> router_lsa::SummaryLsa<'a>;
                }
            }

            pub mod router_lsa {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct ExternalLsa {
                        scalar override_metric("override_metric", 0) -> i64;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct SummaryLsa {
                        scalar override_metric("override_metric", 0) -> i64;
                    }
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct GracefulRestart {
                scalar enabled("enabled", 0) -> bool;
                scalar grace_period("grace_period", 1) -> i64;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct SegmentRoutingMpls {
                scalar enabled("enabled", 0) -> bool;
                scalar shutdown("shutdown", 1) -> bool;
                model prefix_segments("prefix_segments", 2) -> segment_routing_mpls::PrefixSegments<'a>;
                scalar adjacency_segment_allocation("adjacency_segment_allocation", 3) -> &'a str;
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
    }
}
