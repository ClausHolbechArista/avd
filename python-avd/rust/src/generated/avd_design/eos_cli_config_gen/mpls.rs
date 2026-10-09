// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Ldp<'a, Mode> {
    pub interface_disabled_default: ::validated_data::Field<bool>,
    pub router_id: ::validated_data::Field<&'a str>,
    pub shutdown: ::validated_data::Field<bool>,
    pub transport_address_interface: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct Icmp<'a, Mode> {
    pub fragmentation_needed_tunneling: ::validated_data::Field<bool>,
    pub ttl_exceeded_tunneling: ::validated_data::Field<bool>,
}

#[::validated_data::data_view]
pub struct Rsvp<'a, Mode> {
    pub refresh: ::validated_data::Field<rsvp::Refresh<'a, Mode>>,
    pub authentication: ::validated_data::Field<rsvp::Authentication<'a, Mode>>,
    pub neighbors: ::validated_data::Field<rsvp::Neighbors<'a, Mode>>,
    pub ip_access_group: ::validated_data::Field<&'a str>,
    pub ipv6_access_group: ::validated_data::Field<&'a str>,
    pub fast_reroute: ::validated_data::Field<rsvp::FastReroute<'a, Mode>>,
    pub srlg: ::validated_data::Field<rsvp::Srlg<'a, Mode>>,
    pub label_local_termination: ::validated_data::Field<&'a str>,
    pub preemption_method: ::validated_data::Field<rsvp::PreemptionMethod<'a, Mode>>,
    pub mtu_signaling: ::validated_data::Field<bool>,
    pub graceful_restart: ::validated_data::Field<rsvp::GracefulRestart<'a, Mode>>,
    pub hello: ::validated_data::Field<rsvp::Hello<'a, Mode>>,
    pub hitless_restart: ::validated_data::Field<rsvp::HitlessRestart<'a, Mode>>,
    pub p2mp: ::validated_data::Field<rsvp::P2mp<'a, Mode>>,
    pub shutdown: ::validated_data::Field<bool>,
}

pub mod rsvp {

    #[::validated_data::data_view]
    pub struct Refresh<'a, Mode> {
        pub interval: ::validated_data::Field<i64>,
        pub method: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct Authentication<'a, Mode> {
        pub password_indexes: ::validated_data::Field<authentication::PasswordIndexes<'a, Mode>>,
        pub active_index: ::validated_data::Field<i64>,
        pub sequence_number_window: ::validated_data::Field<i64>,
        #[data_view(rename = "type")]
        pub field_type: ::validated_data::Field<&'a str>,
    }

    pub mod authentication {

        #[::validated_data::data_view(indexed_list, primary_key(index))]
        pub struct PasswordIndexes<'a, Mode> (::validated_data::Field<password_indexes::Item<'a, Mode>>);

        pub mod password_indexes {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub index: ::validated_data::Field<i64>,
                pub password_type: ::validated_data::Field<&'a str>,
                pub password: ::validated_data::Field<&'a str>,
            }
        }
    }

    #[::validated_data::data_view(list)]
    pub struct Neighbors<'a, Mode> (::validated_data::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::validated_data::Field<&'a str>,
            pub ipv6_address: ::validated_data::Field<&'a str>,
            pub authentication: ::validated_data::Field<item::Authentication<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct Authentication<'a, Mode> {
                pub index: ::validated_data::Field<i64>,
                #[data_view(rename = "type")]
                pub field_type: ::validated_data::Field<&'a str>,
            }
        }
    }

    #[::validated_data::data_view]
    pub struct FastReroute<'a, Mode> {
        pub mode: ::validated_data::Field<&'a str>,
        pub reversion: ::validated_data::Field<&'a str>,
        pub bypass_tunnel_optimization_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Srlg<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub strict: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view]
    pub struct PreemptionMethod<'a, Mode> {
        pub preemption: ::validated_data::Field<&'a str>,
        pub timer: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct GracefulRestart<'a, Mode> {
        pub role_helper: ::validated_data::Field<graceful_restart::RoleHelper<'a, Mode>>,
        pub role_speaker: ::validated_data::Field<graceful_restart::RoleSpeaker<'a, Mode>>,
    }

    pub mod graceful_restart {

        #[::validated_data::data_view]
        pub struct RoleHelper<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub timer_recovery: ::validated_data::Field<i64>,
            pub timer_restart: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view]
        pub struct RoleSpeaker<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub timer_recovery: ::validated_data::Field<i64>,
            pub timer_restart: ::validated_data::Field<i64>,
        }
    }

    #[::validated_data::data_view]
    pub struct Hello<'a, Mode> {
        pub interval: ::validated_data::Field<i64>,
        pub multiplier: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct HitlessRestart<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub timer_recovery: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct P2mp<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
    }
}

#[::validated_data::data_view]
pub struct LabelRanges<'a, Mode> {
    pub bgp_sr: ::validated_data::Field<label_ranges::BgpSr<'a, Mode>>,
    pub dynamic: ::validated_data::Field<label_ranges::Dynamic<'a, Mode>>,
    pub isis_sr: ::validated_data::Field<label_ranges::IsisSr<'a, Mode>>,
    pub l2evpn: ::validated_data::Field<label_ranges::L2evpn<'a, Mode>>,
    pub l2evpn_ethernet_segment: ::validated_data::Field<label_ranges::L2evpnEthernetSegment<'a, Mode>>,
    pub ospf_sr: ::validated_data::Field<label_ranges::OspfSr<'a, Mode>>,
    pub srlb: ::validated_data::Field<label_ranges::Srlb<'a, Mode>>,
    #[data_view(rename = "static")]
    pub field_static: ::validated_data::Field<label_ranges::FieldStatic<'a, Mode>>,
}

pub mod label_ranges {

    #[::validated_data::data_view]
    pub struct BgpSr<'a, Mode> {
        pub base: ::validated_data::RequiredValue<i64, Mode>,
        pub size: ::validated_data::RequiredValue<i64, Mode>,
    }

    #[::validated_data::data_view]
    pub struct Dynamic<'a, Mode> {
        pub base: ::validated_data::RequiredValue<i64, Mode>,
        pub size: ::validated_data::RequiredValue<i64, Mode>,
    }

    #[::validated_data::data_view]
    pub struct IsisSr<'a, Mode> {
        pub base: ::validated_data::RequiredValue<i64, Mode>,
        pub size: ::validated_data::RequiredValue<i64, Mode>,
    }

    #[::validated_data::data_view]
    pub struct L2evpn<'a, Mode> {
        pub base: ::validated_data::RequiredValue<i64, Mode>,
        pub size: ::validated_data::RequiredValue<i64, Mode>,
    }

    #[::validated_data::data_view]
    pub struct L2evpnEthernetSegment<'a, Mode> {
        pub base: ::validated_data::RequiredValue<i64, Mode>,
        pub size: ::validated_data::RequiredValue<i64, Mode>,
    }

    #[::validated_data::data_view]
    pub struct OspfSr<'a, Mode> {
        pub base: ::validated_data::RequiredValue<i64, Mode>,
        pub size: ::validated_data::RequiredValue<i64, Mode>,
    }

    #[::validated_data::data_view]
    pub struct Srlb<'a, Mode> {
        pub base: ::validated_data::RequiredValue<i64, Mode>,
        pub size: ::validated_data::RequiredValue<i64, Mode>,
    }

    #[::validated_data::data_view]
    pub struct FieldStatic<'a, Mode> {
        pub base: ::validated_data::RequiredValue<i64, Mode>,
        pub size: ::validated_data::RequiredValue<i64, Mode>,
    }
}

#[::validated_data::data_view]
pub struct Tunnel<'a, Mode> {
    pub termination: ::validated_data::Field<tunnel::Termination<'a, Mode>>,
}

pub mod tunnel {

    #[::validated_data::data_view]
    pub struct Termination<'a, Mode> {
        pub model: ::validated_data::Field<termination::Model<'a, Mode>>,
        pub php_model: ::validated_data::Field<termination::PhpModel<'a, Mode>>,
    }

    pub mod termination {

        #[::validated_data::data_view]
        pub struct Model<'a, Mode> {
            pub ttl: ::validated_data::RequiredValue<&'a str, Mode>,
            pub dscp: ::validated_data::RequiredValue<&'a str, Mode>,
        }

        #[::validated_data::data_view]
        pub struct PhpModel<'a, Mode> {
            pub ttl: ::validated_data::RequiredValue<&'a str, Mode>,
            pub dscp: ::validated_data::RequiredValue<&'a str, Mode>,
        }
    }
}
