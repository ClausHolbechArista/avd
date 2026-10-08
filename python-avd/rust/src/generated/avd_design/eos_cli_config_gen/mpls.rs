// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Ldp {
        scalar interface_disabled_default("interface_disabled_default", 0) -> bool;
        scalar router_id("router_id", 1) -> &'a str;
        scalar shutdown("shutdown", 2) -> bool;
        scalar transport_address_interface("transport_address_interface", 3) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Icmp {
        scalar fragmentation_needed_tunneling("fragmentation_needed_tunneling", 0) -> bool;
        scalar ttl_exceeded_tunneling("ttl_exceeded_tunneling", 1) -> bool;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Rsvp {
        model refresh("refresh", 0) -> rsvp::Refresh<'a>;
        model authentication("authentication", 1) -> rsvp::Authentication<'a>;
        model neighbors("neighbors", 2) -> rsvp::Neighbors<'a>;
        scalar ip_access_group("ip_access_group", 3) -> &'a str;
        scalar ipv6_access_group("ipv6_access_group", 4) -> &'a str;
        model fast_reroute("fast_reroute", 5) -> rsvp::FastReroute<'a>;
        model srlg("srlg", 6) -> rsvp::Srlg<'a>;
        scalar label_local_termination("label_local_termination", 7) -> &'a str;
        model preemption_method("preemption_method", 8) -> rsvp::PreemptionMethod<'a>;
        scalar mtu_signaling("mtu_signaling", 9) -> bool;
        model graceful_restart("graceful_restart", 10) -> rsvp::GracefulRestart<'a>;
        model hello("hello", 11) -> rsvp::Hello<'a>;
        model hitless_restart("hitless_restart", 12) -> rsvp::HitlessRestart<'a>;
        model p2mp("p2mp", 13) -> rsvp::P2mp<'a>;
        scalar shutdown("shutdown", 14) -> bool;
    }
}

pub mod rsvp {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Refresh {
            scalar interval("interval", 0) -> i64;
            scalar method("method", 1) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Authentication {
            model password_indexes("password_indexes", 0) -> authentication::PasswordIndexes<'a>;
            scalar active_index("active_index", 1) -> i64;
            scalar sequence_number_window("sequence_number_window", 2) -> i64;
            scalar field_type("type", 3) -> &'a str;
        }
    }

    pub mod authentication {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct PasswordIndexes {
                model item (0) -> password_indexes::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod password_indexes {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar index("index", 0) -> i64;
                    scalar password_type("password_type", 1) -> &'a str;
                    scalar password("password", 2) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Neighbors {
            model item (0) -> neighbors::Item<'a>;
        }
    }

    pub mod neighbors {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar ip_address("ip_address", 0) -> &'a str;
                scalar ipv6_address("ipv6_address", 1) -> &'a str;
                model authentication("authentication", 2) -> item::Authentication<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Authentication {
                    scalar index("index", 0) -> i64;
                    scalar field_type("type", 1) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct FastReroute {
            scalar mode("mode", 0) -> &'a str;
            scalar reversion("reversion", 1) -> &'a str;
            scalar bypass_tunnel_optimization_interval("bypass_tunnel_optimization_interval", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Srlg {
            scalar enabled("enabled", 0) -> bool;
            scalar strict("strict", 1) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PreemptionMethod {
            scalar preemption("preemption", 0) -> &'a str;
            scalar timer("timer", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct GracefulRestart {
            model role_helper("role_helper", 0) -> graceful_restart::RoleHelper<'a>;
            model role_speaker("role_speaker", 1) -> graceful_restart::RoleSpeaker<'a>;
        }
    }

    pub mod graceful_restart {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct RoleHelper {
                scalar enabled("enabled", 0) -> bool;
                scalar timer_recovery("timer_recovery", 1) -> i64;
                scalar timer_restart("timer_restart", 2) -> i64;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct RoleSpeaker {
                scalar enabled("enabled", 0) -> bool;
                scalar timer_recovery("timer_recovery", 1) -> i64;
                scalar timer_restart("timer_restart", 2) -> i64;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Hello {
            scalar interval("interval", 0) -> i64;
            scalar multiplier("multiplier", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct HitlessRestart {
            scalar enabled("enabled", 0) -> bool;
            scalar timer_recovery("timer_recovery", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct P2mp {
            scalar enabled("enabled", 0) -> bool;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct LabelRanges {
        model bgp_sr("bgp_sr", 0) -> label_ranges::BgpSr<'a>;
        model dynamic("dynamic", 1) -> label_ranges::Dynamic<'a>;
        model isis_sr("isis_sr", 2) -> label_ranges::IsisSr<'a>;
        model l2evpn("l2evpn", 3) -> label_ranges::L2evpn<'a>;
        model l2evpn_ethernet_segment("l2evpn_ethernet_segment", 4) -> label_ranges::L2evpnEthernetSegment<'a>;
        model ospf_sr("ospf_sr", 5) -> label_ranges::OspfSr<'a>;
        model srlb("srlb", 6) -> label_ranges::Srlb<'a>;
        model field_static("static", 7) -> label_ranges::FieldStatic<'a>;
    }
}

pub mod label_ranges {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct BgpSr {
            scalar base("base", 0) -> i64;
            scalar size("size", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Dynamic {
            scalar base("base", 0) -> i64;
            scalar size("size", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct IsisSr {
            scalar base("base", 0) -> i64;
            scalar size("size", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct L2evpn {
            scalar base("base", 0) -> i64;
            scalar size("size", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct L2evpnEthernetSegment {
            scalar base("base", 0) -> i64;
            scalar size("size", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct OspfSr {
            scalar base("base", 0) -> i64;
            scalar size("size", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Srlb {
            scalar base("base", 0) -> i64;
            scalar size("size", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct FieldStatic {
            scalar base("base", 0) -> i64;
            scalar size("size", 1) -> i64;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Tunnel {
        model termination("termination", 0) -> tunnel::Termination<'a>;
    }
}

pub mod tunnel {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Termination {
            model model("model", 0) -> termination::Model<'a>;
            model php_model("php_model", 1) -> termination::PhpModel<'a>;
        }
    }

    pub mod termination {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Model {
                scalar ttl("ttl", 0) -> &'a str;
                scalar dscp("dscp", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct PhpModel {
                scalar ttl("ttl", 0) -> &'a str;
                scalar dscp("dscp", 1) -> &'a str;
            }
        }
    }
}
