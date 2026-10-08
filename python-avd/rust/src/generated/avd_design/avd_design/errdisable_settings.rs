// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Causes {
        model acl("acl", 0) -> causes::Acl<'a>;
        model arp_inspection("arp_inspection", 1) -> causes::ArpInspection<'a>;
        model bpduguard("bpduguard", 2) -> causes::Bpduguard<'a>;
        model dot1x("dot1x", 3) -> causes::Dot1x<'a>;
        model dot1x_coa("dot1x_coa", 4) -> causes::Dot1xCoa<'a>;
        model dot1x_phone_classification("dot1x_phone_classification", 5) -> causes::Dot1xPhoneClassification<'a>;
        model dot1x_session_replace("dot1x_session_replace", 6) -> causes::Dot1xSessionReplace<'a>;
        model error_correction_encoding("error_correction_encoding", 7) -> causes::ErrorCorrectionEncoding<'a>;
        model fabric_capacity_low("fabric_capacity_low", 8) -> causes::FabricCapacityLow<'a>;
        model hardware_speed_group("hardware_speed_group", 9) -> causes::HardwareSpeedGroup<'a>;
        model hitless_reload_down("hitless_reload_down", 10) -> causes::HitlessReloadDown<'a>;
        model interface_speed("interface_speed", 11) -> causes::InterfaceSpeed<'a>;
        model internal_error("internal_error", 12) -> causes::InternalError<'a>;
        model lacp_rate_limit("lacp_rate_limit", 13) -> causes::LacpRateLimit<'a>;
        model link_change("link_change", 14) -> causes::LinkChange<'a>;
        model link_flap("link_flap", 15) -> causes::LinkFlap<'a>;
        model no_internal_vlan("no_internal_vlan", 16) -> causes::NoInternalVlan<'a>;
        model port_breakout("port_breakout", 17) -> causes::PortBreakout<'a>;
        model portchannelguard("portchannelguard", 18) -> causes::Portchannelguard<'a>;
        model portsec("portsec", 19) -> causes::Portsec<'a>;
        model speed_misconfigured("speed_misconfigured", 20) -> causes::SpeedMisconfigured<'a>;
        model storm_control("storm_control", 21) -> causes::StormControl<'a>;
        model stuck_queue("stuck_queue", 22) -> causes::StuckQueue<'a>;
        model switchcard_unreachable("switchcard_unreachable", 23) -> causes::SwitchcardUnreachable<'a>;
        model tap_port_init("tap_port_init", 24) -> causes::TapPortInit<'a>;
        model tapagg("tapagg", 25) -> causes::Tapagg<'a>;
        model tpid("tpid", 26) -> causes::Tpid<'a>;
        model transceiver_adapter("transceiver_adapter", 27) -> causes::TransceiverAdapter<'a>;
        model uplink_failure_detection("uplink_failure_detection", 28) -> causes::UplinkFailureDetection<'a>;
        model xcvr_misconfigured("xcvr_misconfigured", 29) -> causes::XcvrMisconfigured<'a>;
        model xcvr_overheat("xcvr_overheat", 30) -> causes::XcvrOverheat<'a>;
        model xcvr_power_unsupported("xcvr_power_unsupported", 31) -> causes::XcvrPowerUnsupported<'a>;
        model xcvr_unsupported("xcvr_unsupported", 32) -> causes::XcvrUnsupported<'a>;
    }
}

pub mod causes {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Acl {
            scalar detection("detection", 0) -> bool;
            scalar recovery("recovery", 1) -> bool;
            scalar recovery_interval("recovery_interval", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct ArpInspection {
            scalar detection("detection", 0) -> bool;
            scalar recovery("recovery", 1) -> bool;
            scalar recovery_interval("recovery_interval", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Bpduguard {
            scalar recovery("recovery", 0) -> bool;
            scalar recovery_interval("recovery_interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Dot1x {
            scalar detection("detection", 0) -> bool;
            scalar recovery("recovery", 1) -> bool;
            scalar recovery_interval("recovery_interval", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Dot1xCoa {
            scalar detection("detection", 0) -> bool;
            scalar recovery("recovery", 1) -> bool;
            scalar recovery_interval("recovery_interval", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Dot1xPhoneClassification {
            scalar detection("detection", 0) -> bool;
            scalar recovery("recovery", 1) -> bool;
            scalar recovery_interval("recovery_interval", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Dot1xSessionReplace {
            scalar detection("detection", 0) -> bool;
            scalar recovery("recovery", 1) -> bool;
            scalar recovery_interval("recovery_interval", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct ErrorCorrectionEncoding {
            scalar detection("detection", 0) -> bool;
            scalar recovery("recovery", 1) -> bool;
            scalar recovery_interval("recovery_interval", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct FabricCapacityLow {
            scalar detection("detection", 0) -> bool;
            scalar recovery("recovery", 1) -> bool;
            scalar recovery_interval("recovery_interval", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct HardwareSpeedGroup {
            scalar detection("detection", 0) -> bool;
            scalar recovery("recovery", 1) -> bool;
            scalar recovery_interval("recovery_interval", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct HitlessReloadDown {
            scalar recovery("recovery", 0) -> bool;
            scalar recovery_interval("recovery_interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct InterfaceSpeed {
            scalar detection("detection", 0) -> bool;
            scalar recovery("recovery", 1) -> bool;
            scalar recovery_interval("recovery_interval", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct InternalError {
            scalar detection("detection", 0) -> bool;
            scalar recovery("recovery", 1) -> bool;
            scalar recovery_interval("recovery_interval", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct LacpRateLimit {
            scalar recovery("recovery", 0) -> bool;
            scalar recovery_interval("recovery_interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct LinkChange {
            scalar detection("detection", 0) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct LinkFlap {
            scalar recovery("recovery", 0) -> bool;
            scalar recovery_interval("recovery_interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct NoInternalVlan {
            scalar recovery("recovery", 0) -> bool;
            scalar recovery_interval("recovery_interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PortBreakout {
            scalar detection("detection", 0) -> bool;
            scalar recovery("recovery", 1) -> bool;
            scalar recovery_interval("recovery_interval", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Portchannelguard {
            scalar recovery("recovery", 0) -> bool;
            scalar recovery_interval("recovery_interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Portsec {
            scalar recovery("recovery", 0) -> bool;
            scalar recovery_interval("recovery_interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct SpeedMisconfigured {
            scalar recovery("recovery", 0) -> bool;
            scalar recovery_interval("recovery_interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct StormControl {
            scalar detection("detection", 0) -> bool;
            scalar recovery("recovery", 1) -> bool;
            scalar recovery_interval("recovery_interval", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct StuckQueue {
            scalar recovery("recovery", 0) -> bool;
            scalar recovery_interval("recovery_interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct SwitchcardUnreachable {
            scalar detection("detection", 0) -> bool;
            scalar recovery("recovery", 1) -> bool;
            scalar recovery_interval("recovery_interval", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct TapPortInit {
            scalar recovery("recovery", 0) -> bool;
            scalar recovery_interval("recovery_interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Tapagg {
            scalar detection("detection", 0) -> bool;
            scalar recovery("recovery", 1) -> bool;
            scalar recovery_interval("recovery_interval", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Tpid {
            scalar detection("detection", 0) -> bool;
            scalar recovery("recovery", 1) -> bool;
            scalar recovery_interval("recovery_interval", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct TransceiverAdapter {
            scalar detection("detection", 0) -> bool;
            scalar recovery("recovery", 1) -> bool;
            scalar recovery_interval("recovery_interval", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct UplinkFailureDetection {
            scalar recovery("recovery", 0) -> bool;
            scalar recovery_interval("recovery_interval", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct XcvrMisconfigured {
            scalar detection("detection", 0) -> bool;
            scalar recovery("recovery", 1) -> bool;
            scalar recovery_interval("recovery_interval", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct XcvrOverheat {
            scalar detection("detection", 0) -> bool;
            scalar recovery("recovery", 1) -> bool;
            scalar recovery_interval("recovery_interval", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct XcvrPowerUnsupported {
            scalar detection("detection", 0) -> bool;
            scalar recovery("recovery", 1) -> bool;
            scalar recovery_interval("recovery_interval", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct XcvrUnsupported {
            scalar recovery("recovery", 0) -> bool;
            scalar recovery_interval("recovery_interval", 1) -> i64;
        }
    }
}
