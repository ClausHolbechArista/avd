// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct RouterIsis<'a, Mode> {
    pub instance: ::data_views::RequiredValue<&'a str, Mode>,
    pub net: ::data_views::Field<&'a str>,
    pub router_id: ::data_views::Field<&'a str>,
    pub is_hostname: ::data_views::Field<&'a str>,
    pub is_type: ::data_views::Field<&'a str>,
    pub log_adjacency_changes: ::data_views::Field<bool>,
    pub mpls_ldp_sync_default: ::data_views::Field<bool>,
    pub timers: ::data_views::Field<Timers<'a, Mode>>,
    pub set_overload_bit: ::data_views::Field<SetOverloadBit<'a, Mode>>,
    pub authentication: ::data_views::Field<Authentication<'a, Mode>>,
    pub advertise: ::data_views::Field<Advertise<'a, Mode>>,
    pub redistribute_routes: ::data_views::Field<RedistributeRoutes<'a, Mode>>,
    pub address_family_ipv4: ::data_views::Field<AddressFamilyIpv4<'a, Mode>>,
    pub address_family_ipv6: ::data_views::Field<AddressFamilyIpv6<'a, Mode>>,
    pub segment_routing_mpls: ::data_views::Field<SegmentRoutingMpls<'a, Mode>>,
    pub spf_interval: ::data_views::Field<SpfInterval<'a, Mode>>,
    pub graceful_restart: ::data_views::Field<GracefulRestart<'a, Mode>>,
    pub eos_cli: ::data_views::Field<&'a str>,
}

#[::data_views::data_view]
pub struct Timers<'a, Mode> {
    pub local_convergence: ::data_views::Field<timers::LocalConvergence<'a, Mode>>,
    pub lsp: ::data_views::Field<timers::Lsp<'a, Mode>>,
    pub csnp: ::data_views::Field<timers::Csnp<'a, Mode>>,
}

pub mod timers {

    #[::data_views::data_view]
    pub struct LocalConvergence<'a, Mode> {
        pub protected_prefixes: ::data_views::Field<bool>,
        pub delay: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct Lsp<'a, Mode> {
        pub generation: ::data_views::Field<lsp::Generation<'a, Mode>>,
        pub out_delay: ::data_views::Field<i64>,
        pub refresh_interval: ::data_views::Field<i64>,
        pub min_remaining_lifetime: ::data_views::Field<i64>,
    }

    pub mod lsp {

        #[::data_views::data_view]
        pub struct Generation<'a, Mode> {
            pub interval: ::data_views::RequiredValue<i64, Mode>,
            pub initial_wait_time: ::data_views::Field<i64>,
            pub wait_time: ::data_views::Field<i64>,
        }
    }

    #[::data_views::data_view]
    pub struct Csnp<'a, Mode> {
        pub generation: ::data_views::Field<csnp::Generation<'a, Mode>>,
    }

    pub mod csnp {

        #[::data_views::data_view]
        pub struct Generation<'a, Mode> {
            pub interval: ::data_views::Field<i64>,
            pub p2p_disabled: ::data_views::Field<bool>,
        }
    }
}

#[::data_views::data_view]
pub struct SetOverloadBit<'a, Mode> {
    pub enabled: ::data_views::RequiredValue<bool, Mode>,
    pub on_startup: ::data_views::Field<set_overload_bit::OnStartup<'a, Mode>>,
}

pub mod set_overload_bit {

    #[::data_views::data_view]
    pub struct OnStartup<'a, Mode> {
        pub delay: ::data_views::Field<i64>,
        pub wait_for_bgp: ::data_views::Field<on_startup::WaitForBgp<'a, Mode>>,
    }

    pub mod on_startup {

        #[::data_views::data_view]
        pub struct WaitForBgp<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub timeout: ::data_views::Field<i64>,
        }
    }
}

#[::data_views::data_view]
pub struct Authentication<'a, Mode> {
    pub both: ::data_views::Field<authentication::Both<'a, Mode>>,
    pub level_1: ::data_views::Field<authentication::Level1<'a, Mode>>,
    pub level_2: ::data_views::Field<authentication::Level2<'a, Mode>>,
}

pub mod authentication {

    #[::data_views::data_view]
    pub struct Both<'a, Mode> {
        pub key_type: ::data_views::Field<&'a str>,
        pub key: ::data_views::Field<&'a str>,
        pub key_ids: ::data_views::Field<both::KeyIds<'a, Mode>>,
        pub mode: ::data_views::Field<&'a str>,
        pub sha: ::data_views::Field<both::Sha<'a, Mode>>,
        pub shared_secret: ::data_views::Field<both::SharedSecret<'a, Mode>>,
        pub rx_disabled: ::data_views::Field<bool>,
    }

    pub mod both {

        #[::data_views::data_view(indexed_list, primary_key(id))]
        pub struct KeyIds<'a, Mode> (::data_views::Field<key_ids::Item<'a, Mode>>);

        pub mod key_ids {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub id: ::data_views::RequiredValue<i64, Mode>,
                pub algorithm: ::data_views::RequiredValue<&'a str, Mode>,
                pub key_type: ::data_views::RequiredValue<&'a str, Mode>,
                pub key: ::data_views::RequiredValue<&'a str, Mode>,
                pub rfc_5310: ::data_views::Field<bool>,
            }
        }

        #[::data_views::data_view]
        pub struct Sha<'a, Mode> {
            pub key_id: ::data_views::RequiredValue<i64, Mode>,
        }

        #[::data_views::data_view]
        pub struct SharedSecret<'a, Mode> {
            pub profile: ::data_views::RequiredValue<&'a str, Mode>,
            pub algorithm: ::data_views::RequiredValue<&'a str, Mode>,
        }
    }

    #[::data_views::data_view]
    pub struct Level1<'a, Mode> {
        pub key_type: ::data_views::Field<&'a str>,
        pub key: ::data_views::Field<&'a str>,
        pub key_ids: ::data_views::Field<level_1::KeyIds<'a, Mode>>,
        pub mode: ::data_views::Field<&'a str>,
        pub sha: ::data_views::Field<level_1::Sha<'a, Mode>>,
        pub shared_secret: ::data_views::Field<level_1::SharedSecret<'a, Mode>>,
        pub rx_disabled: ::data_views::Field<bool>,
    }

    pub mod level_1 {

        #[::data_views::data_view(indexed_list, primary_key(id))]
        pub struct KeyIds<'a, Mode> (::data_views::Field<key_ids::Item<'a, Mode>>);

        pub mod key_ids {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub id: ::data_views::RequiredValue<i64, Mode>,
                pub algorithm: ::data_views::RequiredValue<&'a str, Mode>,
                pub key_type: ::data_views::RequiredValue<&'a str, Mode>,
                pub key: ::data_views::RequiredValue<&'a str, Mode>,
                pub rfc_5310: ::data_views::Field<bool>,
            }
        }

        #[::data_views::data_view]
        pub struct Sha<'a, Mode> {
            pub key_id: ::data_views::RequiredValue<i64, Mode>,
        }

        #[::data_views::data_view]
        pub struct SharedSecret<'a, Mode> {
            pub profile: ::data_views::RequiredValue<&'a str, Mode>,
            pub algorithm: ::data_views::RequiredValue<&'a str, Mode>,
        }
    }

    #[::data_views::data_view]
    pub struct Level2<'a, Mode> {
        pub key_type: ::data_views::Field<&'a str>,
        pub key: ::data_views::Field<&'a str>,
        pub key_ids: ::data_views::Field<level_2::KeyIds<'a, Mode>>,
        pub mode: ::data_views::Field<&'a str>,
        pub sha: ::data_views::Field<level_2::Sha<'a, Mode>>,
        pub shared_secret: ::data_views::Field<level_2::SharedSecret<'a, Mode>>,
        pub rx_disabled: ::data_views::Field<bool>,
    }

    pub mod level_2 {

        #[::data_views::data_view(indexed_list, primary_key(id))]
        pub struct KeyIds<'a, Mode> (::data_views::Field<key_ids::Item<'a, Mode>>);

        pub mod key_ids {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub id: ::data_views::RequiredValue<i64, Mode>,
                pub algorithm: ::data_views::RequiredValue<&'a str, Mode>,
                pub key_type: ::data_views::RequiredValue<&'a str, Mode>,
                pub key: ::data_views::RequiredValue<&'a str, Mode>,
                pub rfc_5310: ::data_views::Field<bool>,
            }
        }

        #[::data_views::data_view]
        pub struct Sha<'a, Mode> {
            pub key_id: ::data_views::RequiredValue<i64, Mode>,
        }

        #[::data_views::data_view]
        pub struct SharedSecret<'a, Mode> {
            pub profile: ::data_views::RequiredValue<&'a str, Mode>,
            pub algorithm: ::data_views::RequiredValue<&'a str, Mode>,
        }
    }
}

#[::data_views::data_view]
pub struct Advertise<'a, Mode> {
    pub passive_only: ::data_views::Field<bool>,
}

#[::data_views::data_view(list)]
pub struct RedistributeRoutes<'a, Mode> (::data_views::Field<redistribute_routes::Item<'a, Mode>>);

pub mod redistribute_routes {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub source_protocol: ::data_views::RequiredValue<&'a str, Mode>,
        pub route_map: ::data_views::Field<&'a str>,
        pub include_leaked: ::data_views::Field<bool>,
        pub ospf_route_type: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view]
pub struct AddressFamilyIpv4<'a, Mode> {
    pub enabled: ::data_views::RequiredValue<bool, Mode>,
    pub maximum_paths: ::data_views::Field<i64>,
    pub bfd_all_interfaces: ::data_views::Field<bool>,
    pub fast_reroute_ti_lfa: ::data_views::Field<address_family_ipv4::FastRerouteTiLfa<'a, Mode>>,
    pub tunnel_source_labeled_unicast: ::data_views::Field<address_family_ipv4::TunnelSourceLabeledUnicast<'a, Mode>>,
}

pub mod address_family_ipv4 {

    #[::data_views::data_view]
    pub struct FastRerouteTiLfa<'a, Mode> {
        pub mode: ::data_views::Field<&'a str>,
        pub level: ::data_views::Field<&'a str>,
        pub srlg: ::data_views::Field<fast_reroute_ti_lfa::Srlg<'a, Mode>>,
    }

    pub mod fast_reroute_ti_lfa {

        #[::data_views::data_view]
        pub struct Srlg<'a, Mode> {
            pub enable: ::data_views::Field<bool>,
            pub strict: ::data_views::Field<bool>,
        }
    }

    #[::data_views::data_view]
    pub struct TunnelSourceLabeledUnicast<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub rcf: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view]
pub struct AddressFamilyIpv6<'a, Mode> {
    pub enabled: ::data_views::RequiredValue<bool, Mode>,
    pub maximum_paths: ::data_views::Field<i64>,
    pub bfd_all_interfaces: ::data_views::Field<bool>,
    pub multi_topology: ::data_views::Field<bool>,
    pub fast_reroute_ti_lfa: ::data_views::Field<address_family_ipv6::FastRerouteTiLfa<'a, Mode>>,
}

pub mod address_family_ipv6 {

    #[::data_views::data_view]
    pub struct FastRerouteTiLfa<'a, Mode> {
        pub mode: ::data_views::Field<&'a str>,
        pub level: ::data_views::Field<&'a str>,
        pub srlg: ::data_views::Field<fast_reroute_ti_lfa::Srlg<'a, Mode>>,
    }

    pub mod fast_reroute_ti_lfa {

        #[::data_views::data_view]
        pub struct Srlg<'a, Mode> {
            pub enable: ::data_views::Field<bool>,
            pub strict: ::data_views::Field<bool>,
        }
    }
}

#[::data_views::data_view]
pub struct SegmentRoutingMpls<'a, Mode> {
    pub enabled: ::data_views::Field<bool>,
    pub router_id: ::data_views::Field<&'a str>,
    pub prefix_segments: ::data_views::Field<segment_routing_mpls::PrefixSegments<'a, Mode>>,
}

pub mod segment_routing_mpls {

    #[::data_views::data_view(indexed_list, primary_key(prefix))]
    pub struct PrefixSegments<'a, Mode> (::data_views::Field<prefix_segments::Item<'a, Mode>>);

    pub mod prefix_segments {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub prefix: ::data_views::Field<&'a str>,
            pub index: ::data_views::Field<i64>,
        }
    }
}

#[::data_views::data_view]
pub struct SpfInterval<'a, Mode> {
    pub interval: ::data_views::Field<i64>,
    pub interval_unit: ::data_views::Field<&'a str>,
    pub wait_interval: ::data_views::Field<i64>,
    pub hold_interval: ::data_views::Field<i64>,
}

#[::data_views::data_view]
pub struct GracefulRestart<'a, Mode> {
    pub enabled: ::data_views::Field<bool>,
    pub restart_hold_time: ::data_views::Field<i64>,
    pub t2: ::data_views::Field<graceful_restart::T2<'a, Mode>>,
}

pub mod graceful_restart {

    #[::data_views::data_view]
    pub struct T2<'a, Mode> {
        pub level_1_wait_time: ::data_views::Field<i64>,
        pub level_2_wait_time: ::data_views::Field<i64>,
    }
}
