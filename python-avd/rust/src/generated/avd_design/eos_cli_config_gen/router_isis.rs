// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Timers<'a, Mode> {
    pub local_convergence: ::validated_data::Field<timers::LocalConvergence<'a, Mode>>,
    pub lsp: ::validated_data::Field<timers::Lsp<'a, Mode>>,
    pub csnp: ::validated_data::Field<timers::Csnp<'a, Mode>>,
}

pub mod timers {

    #[::validated_data::data_view]
    pub struct LocalConvergence<'a, Mode> {
        pub protected_prefixes: ::validated_data::Field<bool>,
        pub delay: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Lsp<'a, Mode> {
        pub generation: ::validated_data::Field<lsp::Generation<'a, Mode>>,
        pub out_delay: ::validated_data::Field<i64>,
        pub refresh_interval: ::validated_data::Field<i64>,
        pub min_remaining_lifetime: ::validated_data::Field<i64>,
    }

    pub mod lsp {

        #[::validated_data::data_view]
        pub struct Generation<'a, Mode> {
            pub interval: ::validated_data::RequiredValue<i64, Mode>,
            pub initial_wait_time: ::validated_data::Field<i64>,
            pub wait_time: ::validated_data::Field<i64>,
        }
    }

    #[::validated_data::data_view]
    pub struct Csnp<'a, Mode> {
        pub generation: ::validated_data::Field<csnp::Generation<'a, Mode>>,
    }

    pub mod csnp {

        #[::validated_data::data_view]
        pub struct Generation<'a, Mode> {
            pub interval: ::validated_data::Field<i64>,
            pub p2p_disabled: ::validated_data::Field<bool>,
        }
    }
}

#[::validated_data::data_view]
pub struct SetOverloadBit<'a, Mode> {
    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    pub on_startup: ::validated_data::Field<set_overload_bit::OnStartup<'a, Mode>>,
}

pub mod set_overload_bit {

    #[::validated_data::data_view]
    pub struct OnStartup<'a, Mode> {
        pub delay: ::validated_data::Field<i64>,
        pub wait_for_bgp: ::validated_data::Field<on_startup::WaitForBgp<'a, Mode>>,
    }

    pub mod on_startup {

        #[::validated_data::data_view]
        pub struct WaitForBgp<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub timeout: ::validated_data::Field<i64>,
        }
    }
}

#[::validated_data::data_view]
pub struct Authentication<'a, Mode> {
    pub both: ::validated_data::Field<authentication::Both<'a, Mode>>,
    pub level_1: ::validated_data::Field<authentication::Level1<'a, Mode>>,
    pub level_2: ::validated_data::Field<authentication::Level2<'a, Mode>>,
}

pub mod authentication {

    #[::validated_data::data_view]
    pub struct Both<'a, Mode> {
        pub key_type: ::validated_data::Field<&'a str>,
        pub key: ::validated_data::Field<&'a str>,
        pub key_ids: ::validated_data::Field<both::KeyIds<'a, Mode>>,
        pub mode: ::validated_data::Field<&'a str>,
        pub sha: ::validated_data::Field<both::Sha<'a, Mode>>,
        pub shared_secret: ::validated_data::Field<both::SharedSecret<'a, Mode>>,
        pub rx_disabled: ::validated_data::Field<bool>,
    }

    pub mod both {

        #[::validated_data::data_view(indexed_list, primary_key(id))]
        pub struct KeyIds<'a, Mode> (::validated_data::Field<key_ids::Item<'a, Mode>>);

        pub mod key_ids {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub id: ::validated_data::RequiredValue<i64, Mode>,
                pub algorithm: ::validated_data::RequiredValue<&'a str, Mode>,
                pub key_type: ::validated_data::RequiredValue<&'a str, Mode>,
                pub key: ::validated_data::RequiredValue<&'a str, Mode>,
                pub rfc_5310: ::validated_data::Field<bool>,
            }
        }

        #[::validated_data::data_view]
        pub struct Sha<'a, Mode> {
            pub key_id: ::validated_data::RequiredValue<i64, Mode>,
        }

        #[::validated_data::data_view]
        pub struct SharedSecret<'a, Mode> {
            pub profile: ::validated_data::RequiredValue<&'a str, Mode>,
            pub algorithm: ::validated_data::RequiredValue<&'a str, Mode>,
        }
    }

    #[::validated_data::data_view]
    pub struct Level1<'a, Mode> {
        pub key_type: ::validated_data::Field<&'a str>,
        pub key: ::validated_data::Field<&'a str>,
        pub key_ids: ::validated_data::Field<level_1::KeyIds<'a, Mode>>,
        pub mode: ::validated_data::Field<&'a str>,
        pub sha: ::validated_data::Field<level_1::Sha<'a, Mode>>,
        pub shared_secret: ::validated_data::Field<level_1::SharedSecret<'a, Mode>>,
        pub rx_disabled: ::validated_data::Field<bool>,
    }

    pub mod level_1 {

        #[::validated_data::data_view(indexed_list, primary_key(id))]
        pub struct KeyIds<'a, Mode> (::validated_data::Field<key_ids::Item<'a, Mode>>);

        pub mod key_ids {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub id: ::validated_data::RequiredValue<i64, Mode>,
                pub algorithm: ::validated_data::RequiredValue<&'a str, Mode>,
                pub key_type: ::validated_data::RequiredValue<&'a str, Mode>,
                pub key: ::validated_data::RequiredValue<&'a str, Mode>,
                pub rfc_5310: ::validated_data::Field<bool>,
            }
        }

        #[::validated_data::data_view]
        pub struct Sha<'a, Mode> {
            pub key_id: ::validated_data::RequiredValue<i64, Mode>,
        }

        #[::validated_data::data_view]
        pub struct SharedSecret<'a, Mode> {
            pub profile: ::validated_data::RequiredValue<&'a str, Mode>,
            pub algorithm: ::validated_data::RequiredValue<&'a str, Mode>,
        }
    }

    #[::validated_data::data_view]
    pub struct Level2<'a, Mode> {
        pub key_type: ::validated_data::Field<&'a str>,
        pub key: ::validated_data::Field<&'a str>,
        pub key_ids: ::validated_data::Field<level_2::KeyIds<'a, Mode>>,
        pub mode: ::validated_data::Field<&'a str>,
        pub sha: ::validated_data::Field<level_2::Sha<'a, Mode>>,
        pub shared_secret: ::validated_data::Field<level_2::SharedSecret<'a, Mode>>,
        pub rx_disabled: ::validated_data::Field<bool>,
    }

    pub mod level_2 {

        #[::validated_data::data_view(indexed_list, primary_key(id))]
        pub struct KeyIds<'a, Mode> (::validated_data::Field<key_ids::Item<'a, Mode>>);

        pub mod key_ids {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub id: ::validated_data::RequiredValue<i64, Mode>,
                pub algorithm: ::validated_data::RequiredValue<&'a str, Mode>,
                pub key_type: ::validated_data::RequiredValue<&'a str, Mode>,
                pub key: ::validated_data::RequiredValue<&'a str, Mode>,
                pub rfc_5310: ::validated_data::Field<bool>,
            }
        }

        #[::validated_data::data_view]
        pub struct Sha<'a, Mode> {
            pub key_id: ::validated_data::RequiredValue<i64, Mode>,
        }

        #[::validated_data::data_view]
        pub struct SharedSecret<'a, Mode> {
            pub profile: ::validated_data::RequiredValue<&'a str, Mode>,
            pub algorithm: ::validated_data::RequiredValue<&'a str, Mode>,
        }
    }
}

#[::validated_data::data_view]
pub struct Advertise<'a, Mode> {
    pub passive_only: ::validated_data::Field<bool>,
}

#[::validated_data::data_view(list)]
pub struct RedistributeRoutes<'a, Mode> (::validated_data::Field<redistribute_routes::Item<'a, Mode>>);

pub mod redistribute_routes {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub source_protocol: ::validated_data::RequiredValue<&'a str, Mode>,
        pub route_map: ::validated_data::Field<&'a str>,
        pub include_leaked: ::validated_data::Field<bool>,
        pub ospf_route_type: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view]
pub struct AddressFamilyIpv4<'a, Mode> {
    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    pub maximum_paths: ::validated_data::Field<i64>,
    pub bfd_all_interfaces: ::validated_data::Field<bool>,
    pub fast_reroute_ti_lfa: ::validated_data::Field<address_family_ipv4::FastRerouteTiLfa<'a, Mode>>,
    pub tunnel_source_labeled_unicast: ::validated_data::Field<address_family_ipv4::TunnelSourceLabeledUnicast<'a, Mode>>,
}

pub mod address_family_ipv4 {

    #[::validated_data::data_view]
    pub struct FastRerouteTiLfa<'a, Mode> {
        pub mode: ::validated_data::Field<&'a str>,
        pub level: ::validated_data::Field<&'a str>,
        pub srlg: ::validated_data::Field<fast_reroute_ti_lfa::Srlg<'a, Mode>>,
    }

    pub mod fast_reroute_ti_lfa {

        #[::validated_data::data_view]
        pub struct Srlg<'a, Mode> {
            pub enable: ::validated_data::Field<bool>,
            pub strict: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view]
    pub struct TunnelSourceLabeledUnicast<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub rcf: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view]
pub struct AddressFamilyIpv6<'a, Mode> {
    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    pub maximum_paths: ::validated_data::Field<i64>,
    pub bfd_all_interfaces: ::validated_data::Field<bool>,
    pub multi_topology: ::validated_data::Field<bool>,
    pub fast_reroute_ti_lfa: ::validated_data::Field<address_family_ipv6::FastRerouteTiLfa<'a, Mode>>,
}

pub mod address_family_ipv6 {

    #[::validated_data::data_view]
    pub struct FastRerouteTiLfa<'a, Mode> {
        pub mode: ::validated_data::Field<&'a str>,
        pub level: ::validated_data::Field<&'a str>,
        pub srlg: ::validated_data::Field<fast_reroute_ti_lfa::Srlg<'a, Mode>>,
    }

    pub mod fast_reroute_ti_lfa {

        #[::validated_data::data_view]
        pub struct Srlg<'a, Mode> {
            pub enable: ::validated_data::Field<bool>,
            pub strict: ::validated_data::Field<bool>,
        }
    }
}

#[::validated_data::data_view]
pub struct SegmentRoutingMpls<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub router_id: ::validated_data::Field<&'a str>,
    pub prefix_segments: ::validated_data::Field<segment_routing_mpls::PrefixSegments<'a, Mode>>,
}

pub mod segment_routing_mpls {

    #[::validated_data::data_view(indexed_list, primary_key(prefix))]
    pub struct PrefixSegments<'a, Mode> (::validated_data::Field<prefix_segments::Item<'a, Mode>>);

    pub mod prefix_segments {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub prefix: ::validated_data::Field<&'a str>,
            pub index: ::validated_data::Field<i64>,
        }
    }
}

#[::validated_data::data_view]
pub struct SpfInterval<'a, Mode> {
    pub interval: ::validated_data::Field<i64>,
    pub interval_unit: ::validated_data::Field<&'a str>,
    pub wait_interval: ::validated_data::Field<i64>,
    pub hold_interval: ::validated_data::Field<i64>,
}

#[::validated_data::data_view]
pub struct GracefulRestart<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub restart_hold_time: ::validated_data::Field<i64>,
    pub t2: ::validated_data::Field<graceful_restart::T2<'a, Mode>>,
}

pub mod graceful_restart {

    #[::validated_data::data_view]
    pub struct T2<'a, Mode> {
        pub level_1_wait_time: ::validated_data::Field<i64>,
        pub level_2_wait_time: ::validated_data::Field<i64>,
    }
}
