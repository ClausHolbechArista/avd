// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Detect {
        model causes("causes", 0) -> detect::Causes<'a>;
    }
}

pub mod detect {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Causes {
            scalar item (0) -> &'a str;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DetectCause {
        scalar acl("acl", 0) -> bool;
        scalar arp_inspection("arp_inspection", 1) -> bool;
        scalar dot1x("dot1x", 2) -> bool;
        scalar dot1x_coa("dot1x_coa", 3) -> bool;
        scalar dot1x_phone_classification("dot1x_phone_classification", 4) -> bool;
        scalar dot1x_session_replace("dot1x_session_replace", 5) -> bool;
        scalar error_correction_encoding("error_correction_encoding", 6) -> bool;
        scalar fabric_capacity_low("fabric_capacity_low", 7) -> bool;
        scalar hardware_speed_group("hardware_speed_group", 8) -> bool;
        scalar interface_speed("interface_speed", 9) -> bool;
        scalar internal_error("internal_error", 10) -> bool;
        scalar link_change("link_change", 11) -> bool;
        scalar port_breakout("port_breakout", 12) -> bool;
        scalar storm_control("storm_control", 13) -> bool;
        scalar switchcard_unreachable("switchcard_unreachable", 14) -> bool;
        scalar tapagg("tapagg", 15) -> bool;
        scalar tpid("tpid", 16) -> bool;
        scalar transceiver_adapter("transceiver_adapter", 17) -> bool;
        scalar xcvr_misconfigured("xcvr_misconfigured", 18) -> bool;
        scalar xcvr_overheat("xcvr_overheat", 19) -> bool;
        scalar xcvr_power_unsupported("xcvr_power_unsupported", 20) -> bool;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Recovery {
        model causes("causes", 0) -> recovery::Causes<'a>;
        scalar interval("interval", 1) -> i64;
    }
}

pub mod recovery {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Causes {
            model item (0) -> causes::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod causes {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar interval("interval", 1) -> i64;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RecoveryCause {
        model acl("acl", 0) -> recovery_cause::Acl<'a>;
        model arp_inspection("arp_inspection", 1) -> recovery_cause::ArpInspection<'a>;
        model bpduguard("bpduguard", 2) -> recovery_cause::Bpduguard<'a>;
        model dot1x("dot1x", 3) -> recovery_cause::Dot1x<'a>;
        model dot1x_coa("dot1x_coa", 4) -> recovery_cause::Dot1xCoa<'a>;
        model dot1x_phone_classification("dot1x_phone_classification", 5) -> recovery_cause::Dot1xPhoneClassification<'a>;
        model dot1x_session_replace("dot1x_session_replace", 6) -> recovery_cause::Dot1xSessionReplace<'a>;
        model error_correction_encoding("error_correction_encoding", 7) -> recovery_cause::ErrorCorrectionEncoding<'a>;
        model fabric_capacity_low("fabric_capacity_low", 8) -> recovery_cause::FabricCapacityLow<'a>;
        model hardware_speed_group("hardware_speed_group", 9) -> recovery_cause::HardwareSpeedGroup<'a>;
        model hitless_reload_down("hitless_reload_down", 10) -> recovery_cause::HitlessReloadDown<'a>;
        model interface_speed("interface_speed", 11) -> recovery_cause::InterfaceSpeed<'a>;
        model internal_error("internal_error", 12) -> recovery_cause::InternalError<'a>;
        model lacp_rate_limit("lacp_rate_limit", 13) -> recovery_cause::LacpRateLimit<'a>;
        model link_flap("link_flap", 14) -> recovery_cause::LinkFlap<'a>;
        model no_internal_vlan("no_internal_vlan", 15) -> recovery_cause::NoInternalVlan<'a>;
        model port_breakout("port_breakout", 16) -> recovery_cause::PortBreakout<'a>;
        model portchannelguard("portchannelguard", 17) -> recovery_cause::Portchannelguard<'a>;
        model portsec("portsec", 18) -> recovery_cause::Portsec<'a>;
        model speed_misconfigured("speed_misconfigured", 19) -> recovery_cause::SpeedMisconfigured<'a>;
        model storm_control("storm_control", 20) -> recovery_cause::StormControl<'a>;
        model stuck_queue("stuck_queue", 21) -> recovery_cause::StuckQueue<'a>;
        model switchcard_unreachable("switchcard_unreachable", 22) -> recovery_cause::SwitchcardUnreachable<'a>;
        model tap_port_init("tap_port_init", 23) -> recovery_cause::TapPortInit<'a>;
        model tapagg("tapagg", 24) -> recovery_cause::Tapagg<'a>;
        model tpid("tpid", 25) -> recovery_cause::Tpid<'a>;
        model transceiver_adapter("transceiver_adapter", 26) -> recovery_cause::TransceiverAdapter<'a>;
        model uplink_failure_detection("uplink_failure_detection", 27) -> recovery_cause::UplinkFailureDetection<'a>;
        model xcvr_misconfigured("xcvr_misconfigured", 28) -> recovery_cause::XcvrMisconfigured<'a>;
        model xcvr_overheat("xcvr_overheat", 29) -> recovery_cause::XcvrOverheat<'a>;
        model xcvr_power_unsupported("xcvr_power_unsupported", 30) -> recovery_cause::XcvrPowerUnsupported<'a>;
        model xcvr_unsupported("xcvr_unsupported", 31) -> recovery_cause::XcvrUnsupported<'a>;
    }
}

pub mod recovery_cause {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Acl {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct ArpInspection {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Bpduguard {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Dot1x {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Dot1xCoa {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Dot1xPhoneClassification {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Dot1xSessionReplace {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct ErrorCorrectionEncoding {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct FabricCapacityLow {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct HardwareSpeedGroup {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct HitlessReloadDown {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct InterfaceSpeed {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct InternalError {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct LacpRateLimit {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct LinkFlap {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct NoInternalVlan {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PortBreakout {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Portchannelguard {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Portsec {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct SpeedMisconfigured {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct StormControl {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct StuckQueue {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct SwitchcardUnreachable {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct TapPortInit {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Tapagg {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Tpid {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct TransceiverAdapter {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct UplinkFailureDetection {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct XcvrMisconfigured {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct XcvrOverheat {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct XcvrPowerUnsupported {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct XcvrUnsupported {
            scalar enabled("enabled", 0) -> bool;
            scalar interval("interval", 1) -> i64;
        }
    }
}
