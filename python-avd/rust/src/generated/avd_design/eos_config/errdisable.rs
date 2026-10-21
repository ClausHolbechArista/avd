// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Errdisable<'a, Mode> {
    pub detect: ::data_views::Field<Detect<'a, Mode>>,
    pub detect_cause: ::data_views::Field<DetectCause<'a, Mode>>,
    pub recovery: ::data_views::Field<Recovery<'a, Mode>>,
    pub recovery_cause: ::data_views::Field<RecoveryCause<'a, Mode>>,
    pub recovery_interval: ::data_views::Field<i64>,
}

#[::data_views::data_view]
pub struct Detect<'a, Mode> {
    pub causes: ::data_views::Field<detect::Causes<'a, Mode>>,
}

pub mod detect {

    #[::data_views::data_view(list)]
    pub struct Causes<'a, Mode> (::data_views::Field<&'a str>);
}

#[::data_views::data_view]
pub struct DetectCause<'a, Mode> {
    pub acl: ::data_views::Field<bool>,
    pub arp_inspection: ::data_views::Field<bool>,
    pub dot1x: ::data_views::Field<bool>,
    pub dot1x_coa: ::data_views::Field<bool>,
    pub dot1x_phone_classification: ::data_views::Field<bool>,
    pub dot1x_session_replace: ::data_views::Field<bool>,
    pub error_correction_encoding: ::data_views::Field<bool>,
    pub fabric_capacity_low: ::data_views::Field<bool>,
    pub hardware_speed_group: ::data_views::Field<bool>,
    pub interface_speed: ::data_views::Field<bool>,
    pub internal_error: ::data_views::Field<bool>,
    pub link_change: ::data_views::Field<bool>,
    pub port_breakout: ::data_views::Field<bool>,
    pub storm_control: ::data_views::Field<bool>,
    pub switchcard_unreachable: ::data_views::Field<bool>,
    pub tapagg: ::data_views::Field<bool>,
    pub tpid: ::data_views::Field<bool>,
    pub transceiver_adapter: ::data_views::Field<bool>,
    pub xcvr_misconfigured: ::data_views::Field<bool>,
    pub xcvr_overheat: ::data_views::Field<bool>,
    pub xcvr_power_unsupported: ::data_views::Field<bool>,
}

#[::data_views::data_view]
pub struct Recovery<'a, Mode> {
    pub causes: ::data_views::Field<recovery::Causes<'a, Mode>>,
    pub interval: ::data_views::Field<i64>,
}

pub mod recovery {

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct Causes<'a, Mode> (::data_views::Field<causes::Item<'a, Mode>>);

    pub mod causes {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub interval: ::data_views::Field<i64>,
        }
    }
}

#[::data_views::data_view]
pub struct RecoveryCause<'a, Mode> {
    pub acl: ::data_views::Field<recovery_cause::Acl<'a, Mode>>,
    pub arp_inspection: ::data_views::Field<recovery_cause::ArpInspection<'a, Mode>>,
    pub bpduguard: ::data_views::Field<recovery_cause::Bpduguard<'a, Mode>>,
    pub dot1x: ::data_views::Field<recovery_cause::Dot1x<'a, Mode>>,
    pub dot1x_coa: ::data_views::Field<recovery_cause::Dot1xCoa<'a, Mode>>,
    pub dot1x_phone_classification: ::data_views::Field<recovery_cause::Dot1xPhoneClassification<'a, Mode>>,
    pub dot1x_session_replace: ::data_views::Field<recovery_cause::Dot1xSessionReplace<'a, Mode>>,
    pub error_correction_encoding: ::data_views::Field<recovery_cause::ErrorCorrectionEncoding<'a, Mode>>,
    pub fabric_capacity_low: ::data_views::Field<recovery_cause::FabricCapacityLow<'a, Mode>>,
    pub hardware_speed_group: ::data_views::Field<recovery_cause::HardwareSpeedGroup<'a, Mode>>,
    pub hitless_reload_down: ::data_views::Field<recovery_cause::HitlessReloadDown<'a, Mode>>,
    pub interface_speed: ::data_views::Field<recovery_cause::InterfaceSpeed<'a, Mode>>,
    pub internal_error: ::data_views::Field<recovery_cause::InternalError<'a, Mode>>,
    pub lacp_rate_limit: ::data_views::Field<recovery_cause::LacpRateLimit<'a, Mode>>,
    pub link_flap: ::data_views::Field<recovery_cause::LinkFlap<'a, Mode>>,
    pub no_internal_vlan: ::data_views::Field<recovery_cause::NoInternalVlan<'a, Mode>>,
    pub port_breakout: ::data_views::Field<recovery_cause::PortBreakout<'a, Mode>>,
    pub portchannelguard: ::data_views::Field<recovery_cause::Portchannelguard<'a, Mode>>,
    pub portsec: ::data_views::Field<recovery_cause::Portsec<'a, Mode>>,
    pub speed_misconfigured: ::data_views::Field<recovery_cause::SpeedMisconfigured<'a, Mode>>,
    pub storm_control: ::data_views::Field<recovery_cause::StormControl<'a, Mode>>,
    pub stuck_queue: ::data_views::Field<recovery_cause::StuckQueue<'a, Mode>>,
    pub switchcard_unreachable: ::data_views::Field<recovery_cause::SwitchcardUnreachable<'a, Mode>>,
    pub tap_port_init: ::data_views::Field<recovery_cause::TapPortInit<'a, Mode>>,
    pub tapagg: ::data_views::Field<recovery_cause::Tapagg<'a, Mode>>,
    pub tpid: ::data_views::Field<recovery_cause::Tpid<'a, Mode>>,
    pub transceiver_adapter: ::data_views::Field<recovery_cause::TransceiverAdapter<'a, Mode>>,
    pub uplink_failure_detection: ::data_views::Field<recovery_cause::UplinkFailureDetection<'a, Mode>>,
    pub xcvr_misconfigured: ::data_views::Field<recovery_cause::XcvrMisconfigured<'a, Mode>>,
    pub xcvr_overheat: ::data_views::Field<recovery_cause::XcvrOverheat<'a, Mode>>,
    pub xcvr_power_unsupported: ::data_views::Field<recovery_cause::XcvrPowerUnsupported<'a, Mode>>,
    pub xcvr_unsupported: ::data_views::Field<recovery_cause::XcvrUnsupported<'a, Mode>>,
}

pub mod recovery_cause {

    #[::data_views::data_view]
    pub struct Acl<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct ArpInspection<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct Bpduguard<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct Dot1x<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct Dot1xCoa<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct Dot1xPhoneClassification<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct Dot1xSessionReplace<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct ErrorCorrectionEncoding<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct FabricCapacityLow<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct HardwareSpeedGroup<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct HitlessReloadDown<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct InterfaceSpeed<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct InternalError<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct LacpRateLimit<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct LinkFlap<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct NoInternalVlan<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct PortBreakout<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct Portchannelguard<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct Portsec<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct SpeedMisconfigured<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct StormControl<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct StuckQueue<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct SwitchcardUnreachable<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct TapPortInit<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct Tapagg<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct Tpid<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct TransceiverAdapter<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct UplinkFailureDetection<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct XcvrMisconfigured<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct XcvrOverheat<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct XcvrPowerUnsupported<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct XcvrUnsupported<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
    }
}
