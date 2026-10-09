// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(id))]
pub struct ProcessIds<'a, Mode> (::validated_data::Field<process_ids::Item<'a, Mode>>);

pub mod process_ids {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub id: ::validated_data::Field<i64>,
        pub vrf: ::validated_data::Field<&'a str>,
        pub passive_interface_default: ::validated_data::Field<bool>,
        pub router_id: ::validated_data::Field<&'a str>,
        pub distance: ::validated_data::Field<item::Distance<'a, Mode>>,
        pub log_adjacency_changes_detail: ::validated_data::Field<bool>,
        pub network_prefixes: ::validated_data::Field<item::NetworkPrefixes<'a, Mode>>,
        pub bfd_enable: ::validated_data::Field<bool>,
        pub bfd_adjacency_state_any: ::validated_data::Field<bool>,
        pub no_passive_interfaces: ::validated_data::Field<item::NoPassiveInterfaces<'a, Mode>>,
        pub distribute_list_in: ::validated_data::Field<item::DistributeListIn<'a, Mode>>,
        pub max_lsa: ::validated_data::Field<i64>,
        pub timers: ::validated_data::Field<item::Timers<'a, Mode>>,
        pub default_information_originate: ::validated_data::Field<item::DefaultInformationOriginate<'a, Mode>>,
        pub summary_addresses: ::validated_data::Field<item::SummaryAddresses<'a, Mode>>,
        pub redistribute: ::validated_data::Field<item::Redistribute<'a, Mode>>,
        pub auto_cost_reference_bandwidth: ::validated_data::Field<i64>,
        pub areas: ::validated_data::Field<item::Areas<'a, Mode>>,
        pub maximum_paths: ::validated_data::Field<i64>,
        pub max_metric: ::validated_data::Field<item::MaxMetric<'a, Mode>>,
        pub graceful_restart: ::validated_data::Field<item::GracefulRestart<'a, Mode>>,
        pub graceful_restart_helper: ::validated_data::Field<bool>,
        pub mpls_ldp_sync_default: ::validated_data::Field<bool>,
        pub segment_routing_mpls: ::validated_data::Field<item::SegmentRoutingMpls<'a, Mode>>,
        pub eos_cli: ::validated_data::Field<&'a str>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct Distance<'a, Mode> {
            pub external: ::validated_data::Field<i64>,
            pub inter_area: ::validated_data::Field<i64>,
            pub intra_area: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view(indexed_list, primary_key(ipv4_prefix))]
        pub struct NetworkPrefixes<'a, Mode> (::validated_data::Field<network_prefixes::Item<'a, Mode>>);

        pub mod network_prefixes {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub ipv4_prefix: ::validated_data::Field<&'a str>,
                pub area: ::validated_data::Field<&'a str>,
            }
        }

        #[::validated_data::data_view(list)]
        pub struct NoPassiveInterfaces<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view]
        pub struct DistributeListIn<'a, Mode> {
            pub route_map: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct Timers<'a, Mode> {
            pub lsa: ::validated_data::Field<timers::Lsa<'a, Mode>>,
            pub spf_delay: ::validated_data::Field<timers::SpfDelay<'a, Mode>>,
        }

        pub mod timers {

            #[::validated_data::data_view]
            pub struct Lsa<'a, Mode> {
                pub rx_min_interval: ::validated_data::Field<i64>,
                pub tx_delay: ::validated_data::Field<lsa::TxDelay<'a, Mode>>,
            }

            pub mod lsa {

                #[::validated_data::data_view]
                pub struct TxDelay<'a, Mode> {
                    pub initial: ::validated_data::Field<i64>,
                    pub min: ::validated_data::Field<i64>,
                    pub max: ::validated_data::Field<i64>,
                }
            }

            #[::validated_data::data_view]
            pub struct SpfDelay<'a, Mode> {
                pub initial: ::validated_data::Field<i64>,
                pub min: ::validated_data::Field<i64>,
                pub max: ::validated_data::Field<i64>,
            }
        }

        #[::validated_data::data_view]
        pub struct DefaultInformationOriginate<'a, Mode> {
            pub always: ::validated_data::Field<bool>,
            pub metric: ::validated_data::Field<i64>,
            pub metric_type: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view(indexed_list, primary_key(prefix))]
        pub struct SummaryAddresses<'a, Mode> (::validated_data::Field<summary_addresses::Item<'a, Mode>>);

        pub mod summary_addresses {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub prefix: ::validated_data::Field<&'a str>,
                pub tag: ::validated_data::Field<i64>,
                pub attribute_map: ::validated_data::Field<&'a str>,
                pub not_advertise: ::validated_data::Field<bool>,
            }
        }

        #[::validated_data::data_view]
        pub struct Redistribute<'a, Mode> {
            #[data_view(rename = "static")]
            pub field_static: ::validated_data::Field<redistribute::FieldStatic<'a, Mode>>,
            pub connected: ::validated_data::Field<redistribute::Connected<'a, Mode>>,
            pub bgp: ::validated_data::Field<redistribute::Bgp<'a, Mode>>,
        }

        pub mod redistribute {

            #[::validated_data::data_view]
            pub struct FieldStatic<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
                pub include_leaked: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view]
            pub struct Connected<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
                pub include_leaked: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view]
            pub struct Bgp<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
                pub include_leaked: ::validated_data::Field<bool>,
            }
        }

        #[::validated_data::data_view(indexed_list, primary_key(id))]
        pub struct Areas<'a, Mode> (::validated_data::Field<areas::Item<'a, Mode>>);

        pub mod areas {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub id: ::validated_data::Field<&'a str>,
                pub filter: ::validated_data::Field<item::Filter<'a, Mode>>,
                #[data_view(rename = "type")]
                pub field_type: ::validated_data::Field<&'a str>,
                pub no_summary: ::validated_data::Field<bool>,
                pub nssa_only: ::validated_data::Field<bool>,
                pub default_information_originate: ::validated_data::Field<item::DefaultInformationOriginate<'a, Mode>>,
            }

            pub mod item {

                #[::validated_data::data_view]
                pub struct Filter<'a, Mode> {
                    pub networks: ::validated_data::Field<filter::Networks<'a, Mode>>,
                    pub prefix_list: ::validated_data::Field<&'a str>,
                }

                pub mod filter {

                    #[::validated_data::data_view(list)]
                    pub struct Networks<'a, Mode> (::validated_data::Field<&'a str>);
                }

                #[::validated_data::data_view]
                pub struct DefaultInformationOriginate<'a, Mode> {
                    pub metric: ::validated_data::Field<i64>,
                    pub metric_type: ::validated_data::Field<i64>,
                }
            }
        }

        #[::validated_data::data_view]
        pub struct MaxMetric<'a, Mode> {
            pub router_lsa: ::validated_data::Field<max_metric::RouterLsa<'a, Mode>>,
        }

        pub mod max_metric {

            #[::validated_data::data_view]
            pub struct RouterLsa<'a, Mode> {
                pub external_lsa: ::validated_data::Field<router_lsa::ExternalLsa<'a, Mode>>,
                pub include_stub: ::validated_data::Field<bool>,
                pub on_startup: ::validated_data::Field<&'a str>,
                pub summary_lsa: ::validated_data::Field<router_lsa::SummaryLsa<'a, Mode>>,
            }

            pub mod router_lsa {

                #[::validated_data::data_view]
                pub struct ExternalLsa<'a, Mode> {
                    pub override_metric: ::validated_data::Field<i64>,
                }

                #[::validated_data::data_view]
                pub struct SummaryLsa<'a, Mode> {
                    pub override_metric: ::validated_data::Field<i64>,
                }
            }
        }

        #[::validated_data::data_view]
        pub struct GracefulRestart<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub grace_period: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view]
        pub struct SegmentRoutingMpls<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub shutdown: ::validated_data::Field<bool>,
            pub prefix_segments: ::validated_data::Field<segment_routing_mpls::PrefixSegments<'a, Mode>>,
            pub adjacency_segment_allocation: ::validated_data::Field<&'a str>,
        }

        pub mod segment_routing_mpls {

            #[::validated_data::data_view(indexed_list, primary_key(prefix))]
            pub struct PrefixSegments<'a, Mode> (::validated_data::Field<prefix_segments::Item<'a, Mode>>);

            pub mod prefix_segments {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub prefix: ::validated_data::Field<&'a str>,
                    pub index: ::validated_data::RequiredValue<i64, Mode>,
                }
            }
        }
    }
}
