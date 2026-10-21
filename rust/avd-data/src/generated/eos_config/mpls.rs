// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Mpls<'a, Mode> {
    pub ip: ::data_views::Field<bool>,
    pub ldp: ::data_views::Field<Ldp<'a, Mode>>,
    pub icmp: ::data_views::Field<Icmp<'a, Mode>>,
    pub rsvp: ::data_views::Field<Rsvp<'a, Mode>>,
    pub label_ranges: ::data_views::Field<LabelRanges<'a, Mode>>,
    pub tunnel: ::data_views::Field<Tunnel<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Ldp<'a, Mode> {
    pub interface_disabled_default: ::data_views::Field<bool>,
    pub router_id: ::data_views::Field<&'a str>,
    pub shutdown: ::data_views::Field<bool>,
    pub transport_address_interface: ::data_views::Field<&'a str>,
}

#[::data_views::data_view]
pub struct Icmp<'a, Mode> {
    pub fragmentation_needed_tunneling: ::data_views::Field<bool>,
    pub ttl_exceeded_tunneling: ::data_views::Field<bool>,
}

#[::data_views::data_view]
pub struct Rsvp<'a, Mode> {
    pub refresh: ::data_views::Field<rsvp::Refresh<'a, Mode>>,
    pub authentication: ::data_views::Field<rsvp::Authentication<'a, Mode>>,
    pub neighbors: ::data_views::Field<rsvp::Neighbors<'a, Mode>>,
    pub ip_access_group: ::data_views::Field<&'a str>,
    pub ipv6_access_group: ::data_views::Field<&'a str>,
    pub fast_reroute: ::data_views::Field<rsvp::FastReroute<'a, Mode>>,
    pub srlg: ::data_views::Field<rsvp::Srlg<'a, Mode>>,
    pub label_local_termination: ::data_views::Field<&'a str>,
    pub preemption_method: ::data_views::Field<rsvp::PreemptionMethod<'a, Mode>>,
    pub mtu_signaling: ::data_views::Field<bool>,
    pub graceful_restart: ::data_views::Field<rsvp::GracefulRestart<'a, Mode>>,
    pub hello: ::data_views::Field<rsvp::Hello<'a, Mode>>,
    pub hitless_restart: ::data_views::Field<rsvp::HitlessRestart<'a, Mode>>,
    pub p2mp: ::data_views::Field<rsvp::P2mp<'a, Mode>>,
    pub shutdown: ::data_views::Field<bool>,
}

pub mod rsvp {

    #[::data_views::data_view]
    pub struct Refresh<'a, Mode> {
        pub interval: ::data_views::Field<i64>,
        pub method: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view]
    pub struct Authentication<'a, Mode> {
        pub password_indexes: ::data_views::Field<authentication::PasswordIndexes<'a, Mode>>,
        pub active_index: ::data_views::Field<i64>,
        pub sequence_number_window: ::data_views::Field<i64>,
        #[data_view(rename = "type")]
        pub field_type: ::data_views::Field<&'a str>,
    }

    pub mod authentication {

        #[::data_views::data_view(indexed_list, primary_key(index))]
        pub struct PasswordIndexes<'a, Mode> (::data_views::Field<password_indexes::Item<'a, Mode>>);

        pub mod password_indexes {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub index: ::data_views::Field<i64>,
                pub password_type: ::data_views::Field<&'a str>,
                pub password: ::data_views::Field<&'a str>,
            }
        }
    }

    #[::data_views::data_view(list)]
    pub struct Neighbors<'a, Mode> (::data_views::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::data_views::Field<&'a str>,
            pub ipv6_address: ::data_views::Field<&'a str>,
            pub authentication: ::data_views::Field<item::Authentication<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct Authentication<'a, Mode> {
                pub index: ::data_views::Field<i64>,
                #[data_view(rename = "type")]
                pub field_type: ::data_views::Field<&'a str>,
            }
        }
    }

    #[::data_views::data_view]
    pub struct FastReroute<'a, Mode> {
        pub mode: ::data_views::Field<&'a str>,
        pub reversion: ::data_views::Field<&'a str>,
        pub bypass_tunnel_optimization_interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct Srlg<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub strict: ::data_views::Field<bool>,
    }

    #[::data_views::data_view]
    pub struct PreemptionMethod<'a, Mode> {
        pub preemption: ::data_views::Field<&'a str>,
        pub timer: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct GracefulRestart<'a, Mode> {
        pub role_helper: ::data_views::Field<graceful_restart::RoleHelper<'a, Mode>>,
        pub role_speaker: ::data_views::Field<graceful_restart::RoleSpeaker<'a, Mode>>,
    }

    pub mod graceful_restart {

        #[::data_views::data_view]
        pub struct RoleHelper<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub timer_recovery: ::data_views::Field<i64>,
            pub timer_restart: ::data_views::Field<i64>,
        }

        #[::data_views::data_view]
        pub struct RoleSpeaker<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub timer_recovery: ::data_views::Field<i64>,
            pub timer_restart: ::data_views::Field<i64>,
        }
    }

    #[::data_views::data_view]
    pub struct Hello<'a, Mode> {
        pub interval: ::data_views::Field<i64>,
        pub multiplier: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct HitlessRestart<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub timer_recovery: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct P2mp<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
    }
}

#[::data_views::data_view]
pub struct LabelRanges<'a, Mode> {
    pub bgp_sr: ::data_views::Field<label_ranges::BgpSr<'a, Mode>>,
    pub dynamic: ::data_views::Field<label_ranges::Dynamic<'a, Mode>>,
    pub isis_sr: ::data_views::Field<label_ranges::IsisSr<'a, Mode>>,
    pub l2evpn: ::data_views::Field<label_ranges::L2evpn<'a, Mode>>,
    pub l2evpn_ethernet_segment: ::data_views::Field<label_ranges::L2evpnEthernetSegment<'a, Mode>>,
    pub ospf_sr: ::data_views::Field<label_ranges::OspfSr<'a, Mode>>,
    pub srlb: ::data_views::Field<label_ranges::Srlb<'a, Mode>>,
    #[data_view(rename = "static")]
    pub field_static: ::data_views::Field<label_ranges::FieldStatic<'a, Mode>>,
}

pub mod label_ranges {

    #[::data_views::data_view]
    pub struct BgpSr<'a, Mode> {
        pub base: ::data_views::RequiredValue<i64, Mode>,
        pub size: ::data_views::RequiredValue<i64, Mode>,
    }

    #[::data_views::data_view]
    pub struct Dynamic<'a, Mode> {
        pub base: ::data_views::RequiredValue<i64, Mode>,
        pub size: ::data_views::RequiredValue<i64, Mode>,
    }

    #[::data_views::data_view]
    pub struct IsisSr<'a, Mode> {
        pub base: ::data_views::RequiredValue<i64, Mode>,
        pub size: ::data_views::RequiredValue<i64, Mode>,
    }

    #[::data_views::data_view]
    pub struct L2evpn<'a, Mode> {
        pub base: ::data_views::RequiredValue<i64, Mode>,
        pub size: ::data_views::RequiredValue<i64, Mode>,
    }

    #[::data_views::data_view]
    pub struct L2evpnEthernetSegment<'a, Mode> {
        pub base: ::data_views::RequiredValue<i64, Mode>,
        pub size: ::data_views::RequiredValue<i64, Mode>,
    }

    #[::data_views::data_view]
    pub struct OspfSr<'a, Mode> {
        pub base: ::data_views::RequiredValue<i64, Mode>,
        pub size: ::data_views::RequiredValue<i64, Mode>,
    }

    #[::data_views::data_view]
    pub struct Srlb<'a, Mode> {
        pub base: ::data_views::RequiredValue<i64, Mode>,
        pub size: ::data_views::RequiredValue<i64, Mode>,
    }

    #[::data_views::data_view]
    pub struct FieldStatic<'a, Mode> {
        pub base: ::data_views::RequiredValue<i64, Mode>,
        pub size: ::data_views::RequiredValue<i64, Mode>,
    }
}

#[::data_views::data_view]
pub struct Tunnel<'a, Mode> {
    pub termination: ::data_views::Field<tunnel::Termination<'a, Mode>>,
}

pub mod tunnel {

    #[::data_views::data_view]
    pub struct Termination<'a, Mode> {
        pub model: ::data_views::Field<termination::Model<'a, Mode>>,
        pub php_model: ::data_views::Field<termination::PhpModel<'a, Mode>>,
    }

    pub mod termination {

        #[::data_views::data_view]
        pub struct Model<'a, Mode> {
            pub ttl: ::data_views::RequiredValue<&'a str, Mode>,
            pub dscp: ::data_views::RequiredValue<&'a str, Mode>,
        }

        #[::data_views::data_view]
        pub struct PhpModel<'a, Mode> {
            pub ttl: ::data_views::RequiredValue<&'a str, Mode>,
            pub dscp: ::data_views::RequiredValue<&'a str, Mode>,
        }
    }
}
