// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct RouterOspf<'a, Mode> {
    pub process_ids: ::data_views::Field<ProcessIds<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(id))]
pub struct ProcessIds<'a, Mode> (::data_views::Field<process_ids::Item<'a, Mode>>);

pub mod process_ids {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub id: ::data_views::Field<i64>,
        pub vrf: ::data_views::Field<&'a str>,
        pub passive_interface_default: ::data_views::Field<bool>,
        pub router_id: ::data_views::Field<&'a str>,
        pub distance: ::data_views::Field<item::Distance<'a, Mode>>,
        pub log_adjacency_changes_detail: ::data_views::Field<bool>,
        pub network_prefixes: ::data_views::Field<item::NetworkPrefixes<'a, Mode>>,
        pub bfd_enable: ::data_views::Field<bool>,
        pub bfd_adjacency_state_any: ::data_views::Field<bool>,
        pub no_passive_interfaces: ::data_views::Field<item::NoPassiveInterfaces<'a, Mode>>,
        pub distribute_list_in: ::data_views::Field<item::DistributeListIn<'a, Mode>>,
        pub max_lsa: ::data_views::Field<i64>,
        pub timers: ::data_views::Field<item::Timers<'a, Mode>>,
        pub default_information_originate: ::data_views::Field<item::DefaultInformationOriginate<'a, Mode>>,
        pub summary_addresses: ::data_views::Field<item::SummaryAddresses<'a, Mode>>,
        pub redistribute: ::data_views::Field<item::Redistribute<'a, Mode>>,
        pub auto_cost_reference_bandwidth: ::data_views::Field<i64>,
        pub areas: ::data_views::Field<item::Areas<'a, Mode>>,
        pub maximum_paths: ::data_views::Field<i64>,
        pub max_metric: ::data_views::Field<item::MaxMetric<'a, Mode>>,
        pub graceful_restart: ::data_views::Field<item::GracefulRestart<'a, Mode>>,
        pub graceful_restart_helper: ::data_views::Field<bool>,
        pub mpls_ldp_sync_default: ::data_views::Field<bool>,
        pub segment_routing_mpls: ::data_views::Field<item::SegmentRoutingMpls<'a, Mode>>,
        pub eos_cli: ::data_views::Field<&'a str>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct Distance<'a, Mode> {
            pub external: ::data_views::Field<i64>,
            pub inter_area: ::data_views::Field<i64>,
            pub intra_area: ::data_views::Field<i64>,
        }

        #[::data_views::data_view(indexed_list, primary_key(ipv4_prefix))]
        pub struct NetworkPrefixes<'a, Mode> (::data_views::Field<network_prefixes::Item<'a, Mode>>);

        pub mod network_prefixes {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub ipv4_prefix: ::data_views::Field<&'a str>,
                pub area: ::data_views::Field<&'a str>,
            }
        }

        #[::data_views::data_view(list)]
        pub struct NoPassiveInterfaces<'a, Mode> (::data_views::Field<&'a str>);

        #[::data_views::data_view]
        pub struct DistributeListIn<'a, Mode> {
            pub route_map: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct Timers<'a, Mode> {
            pub lsa: ::data_views::Field<timers::Lsa<'a, Mode>>,
            pub spf_delay: ::data_views::Field<timers::SpfDelay<'a, Mode>>,
        }

        pub mod timers {

            #[::data_views::data_view]
            pub struct Lsa<'a, Mode> {
                pub rx_min_interval: ::data_views::Field<i64>,
                pub tx_delay: ::data_views::Field<lsa::TxDelay<'a, Mode>>,
            }

            pub mod lsa {

                #[::data_views::data_view]
                pub struct TxDelay<'a, Mode> {
                    pub initial: ::data_views::Field<i64>,
                    pub min: ::data_views::Field<i64>,
                    pub max: ::data_views::Field<i64>,
                }
            }

            #[::data_views::data_view]
            pub struct SpfDelay<'a, Mode> {
                pub initial: ::data_views::Field<i64>,
                pub min: ::data_views::Field<i64>,
                pub max: ::data_views::Field<i64>,
            }
        }

        #[::data_views::data_view]
        pub struct DefaultInformationOriginate<'a, Mode> {
            pub always: ::data_views::Field<bool>,
            pub metric: ::data_views::Field<i64>,
            pub metric_type: ::data_views::Field<i64>,
        }

        #[::data_views::data_view(indexed_list, primary_key(prefix))]
        pub struct SummaryAddresses<'a, Mode> (::data_views::Field<summary_addresses::Item<'a, Mode>>);

        pub mod summary_addresses {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub prefix: ::data_views::Field<&'a str>,
                pub tag: ::data_views::Field<i64>,
                pub attribute_map: ::data_views::Field<&'a str>,
                pub not_advertise: ::data_views::Field<bool>,
            }
        }

        #[::data_views::data_view]
        pub struct Redistribute<'a, Mode> {
            #[data_view(rename = "static")]
            pub field_static: ::data_views::Field<redistribute::FieldStatic<'a, Mode>>,
            pub connected: ::data_views::Field<redistribute::Connected<'a, Mode>>,
            pub bgp: ::data_views::Field<redistribute::Bgp<'a, Mode>>,
        }

        pub mod redistribute {

            #[::data_views::data_view]
            pub struct FieldStatic<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
                pub include_leaked: ::data_views::Field<bool>,
            }

            #[::data_views::data_view]
            pub struct Connected<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
                pub include_leaked: ::data_views::Field<bool>,
            }

            #[::data_views::data_view]
            pub struct Bgp<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
                pub include_leaked: ::data_views::Field<bool>,
            }
        }

        #[::data_views::data_view(indexed_list, primary_key(id))]
        pub struct Areas<'a, Mode> (::data_views::Field<areas::Item<'a, Mode>>);

        pub mod areas {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub id: ::data_views::Field<&'a str>,
                pub filter: ::data_views::Field<item::Filter<'a, Mode>>,
                #[data_view(rename = "type")]
                pub field_type: ::data_views::Field<&'a str>,
                pub no_summary: ::data_views::Field<bool>,
                pub nssa_only: ::data_views::Field<bool>,
                pub default_information_originate: ::data_views::Field<item::DefaultInformationOriginate<'a, Mode>>,
            }

            pub mod item {

                #[::data_views::data_view]
                pub struct Filter<'a, Mode> {
                    pub networks: ::data_views::Field<filter::Networks<'a, Mode>>,
                    pub prefix_list: ::data_views::Field<&'a str>,
                }

                pub mod filter {

                    #[::data_views::data_view(list)]
                    pub struct Networks<'a, Mode> (::data_views::Field<&'a str>);
                }

                #[::data_views::data_view]
                pub struct DefaultInformationOriginate<'a, Mode> {
                    pub metric: ::data_views::Field<i64>,
                    pub metric_type: ::data_views::Field<i64>,
                }
            }
        }

        #[::data_views::data_view]
        pub struct MaxMetric<'a, Mode> {
            pub router_lsa: ::data_views::Field<max_metric::RouterLsa<'a, Mode>>,
        }

        pub mod max_metric {

            #[::data_views::data_view]
            pub struct RouterLsa<'a, Mode> {
                pub external_lsa: ::data_views::Field<router_lsa::ExternalLsa<'a, Mode>>,
                pub include_stub: ::data_views::Field<bool>,
                pub on_startup: ::data_views::Field<&'a str>,
                pub summary_lsa: ::data_views::Field<router_lsa::SummaryLsa<'a, Mode>>,
            }

            pub mod router_lsa {

                #[::data_views::data_view]
                pub struct ExternalLsa<'a, Mode> {
                    pub override_metric: ::data_views::Field<i64>,
                }

                #[::data_views::data_view]
                pub struct SummaryLsa<'a, Mode> {
                    pub override_metric: ::data_views::Field<i64>,
                }
            }
        }

        #[::data_views::data_view]
        pub struct GracefulRestart<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub grace_period: ::data_views::Field<i64>,
        }

        #[::data_views::data_view]
        pub struct SegmentRoutingMpls<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub shutdown: ::data_views::Field<bool>,
            pub prefix_segments: ::data_views::Field<segment_routing_mpls::PrefixSegments<'a, Mode>>,
            pub adjacency_segment_allocation: ::data_views::Field<&'a str>,
        }

        pub mod segment_routing_mpls {

            #[::data_views::data_view(indexed_list, primary_key(prefix))]
            pub struct PrefixSegments<'a, Mode> (::data_views::Field<prefix_segments::Item<'a, Mode>>);

            pub mod prefix_segments {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub prefix: ::data_views::Field<&'a str>,
                    pub index: ::data_views::RequiredValue<i64, Mode>,
                }
            }
        }
    }
}
