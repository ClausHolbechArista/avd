// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Causes<'a, Mode> {
    pub acl: ::validated_data::Field<causes::Acl<'a, Mode>>,
    pub arp_inspection: ::validated_data::Field<causes::ArpInspection<'a, Mode>>,
    pub bpduguard: ::validated_data::Field<causes::Bpduguard<'a, Mode>>,
    pub dot1x: ::validated_data::Field<causes::Dot1x<'a, Mode>>,
    pub dot1x_coa: ::validated_data::Field<causes::Dot1xCoa<'a, Mode>>,
    pub dot1x_phone_classification: ::validated_data::Field<causes::Dot1xPhoneClassification<'a, Mode>>,
    pub dot1x_session_replace: ::validated_data::Field<causes::Dot1xSessionReplace<'a, Mode>>,
    pub error_correction_encoding: ::validated_data::Field<causes::ErrorCorrectionEncoding<'a, Mode>>,
    pub fabric_capacity_low: ::validated_data::Field<causes::FabricCapacityLow<'a, Mode>>,
    pub hardware_speed_group: ::validated_data::Field<causes::HardwareSpeedGroup<'a, Mode>>,
    pub hitless_reload_down: ::validated_data::Field<causes::HitlessReloadDown<'a, Mode>>,
    pub interface_speed: ::validated_data::Field<causes::InterfaceSpeed<'a, Mode>>,
    pub internal_error: ::validated_data::Field<causes::InternalError<'a, Mode>>,
    pub lacp_rate_limit: ::validated_data::Field<causes::LacpRateLimit<'a, Mode>>,
    pub link_change: ::validated_data::Field<causes::LinkChange<'a, Mode>>,
    pub link_flap: ::validated_data::Field<causes::LinkFlap<'a, Mode>>,
    pub no_internal_vlan: ::validated_data::Field<causes::NoInternalVlan<'a, Mode>>,
    pub port_breakout: ::validated_data::Field<causes::PortBreakout<'a, Mode>>,
    pub portchannelguard: ::validated_data::Field<causes::Portchannelguard<'a, Mode>>,
    pub portsec: ::validated_data::Field<causes::Portsec<'a, Mode>>,
    pub speed_misconfigured: ::validated_data::Field<causes::SpeedMisconfigured<'a, Mode>>,
    pub storm_control: ::validated_data::Field<causes::StormControl<'a, Mode>>,
    pub stuck_queue: ::validated_data::Field<causes::StuckQueue<'a, Mode>>,
    pub switchcard_unreachable: ::validated_data::Field<causes::SwitchcardUnreachable<'a, Mode>>,
    pub tap_port_init: ::validated_data::Field<causes::TapPortInit<'a, Mode>>,
    pub tapagg: ::validated_data::Field<causes::Tapagg<'a, Mode>>,
    pub tpid: ::validated_data::Field<causes::Tpid<'a, Mode>>,
    pub transceiver_adapter: ::validated_data::Field<causes::TransceiverAdapter<'a, Mode>>,
    pub uplink_failure_detection: ::validated_data::Field<causes::UplinkFailureDetection<'a, Mode>>,
    pub xcvr_misconfigured: ::validated_data::Field<causes::XcvrMisconfigured<'a, Mode>>,
    pub xcvr_overheat: ::validated_data::Field<causes::XcvrOverheat<'a, Mode>>,
    pub xcvr_power_unsupported: ::validated_data::Field<causes::XcvrPowerUnsupported<'a, Mode>>,
    pub xcvr_unsupported: ::validated_data::Field<causes::XcvrUnsupported<'a, Mode>>,
}

pub mod causes {

    #[::validated_data::data_view]
    pub struct Acl<'a, Mode> {
        pub detection: ::validated_data::Field<bool>,
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct ArpInspection<'a, Mode> {
        pub detection: ::validated_data::Field<bool>,
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Bpduguard<'a, Mode> {
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Dot1x<'a, Mode> {
        pub detection: ::validated_data::Field<bool>,
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Dot1xCoa<'a, Mode> {
        pub detection: ::validated_data::Field<bool>,
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Dot1xPhoneClassification<'a, Mode> {
        pub detection: ::validated_data::Field<bool>,
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Dot1xSessionReplace<'a, Mode> {
        pub detection: ::validated_data::Field<bool>,
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct ErrorCorrectionEncoding<'a, Mode> {
        pub detection: ::validated_data::Field<bool>,
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct FabricCapacityLow<'a, Mode> {
        pub detection: ::validated_data::Field<bool>,
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct HardwareSpeedGroup<'a, Mode> {
        pub detection: ::validated_data::Field<bool>,
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct HitlessReloadDown<'a, Mode> {
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct InterfaceSpeed<'a, Mode> {
        pub detection: ::validated_data::Field<bool>,
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct InternalError<'a, Mode> {
        pub detection: ::validated_data::Field<bool>,
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct LacpRateLimit<'a, Mode> {
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct LinkChange<'a, Mode> {
        pub detection: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view]
    pub struct LinkFlap<'a, Mode> {
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct NoInternalVlan<'a, Mode> {
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct PortBreakout<'a, Mode> {
        pub detection: ::validated_data::Field<bool>,
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Portchannelguard<'a, Mode> {
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Portsec<'a, Mode> {
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct SpeedMisconfigured<'a, Mode> {
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct StormControl<'a, Mode> {
        pub detection: ::validated_data::Field<bool>,
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct StuckQueue<'a, Mode> {
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct SwitchcardUnreachable<'a, Mode> {
        pub detection: ::validated_data::Field<bool>,
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct TapPortInit<'a, Mode> {
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Tapagg<'a, Mode> {
        pub detection: ::validated_data::Field<bool>,
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Tpid<'a, Mode> {
        pub detection: ::validated_data::Field<bool>,
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct TransceiverAdapter<'a, Mode> {
        pub detection: ::validated_data::Field<bool>,
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct UplinkFailureDetection<'a, Mode> {
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct XcvrMisconfigured<'a, Mode> {
        pub detection: ::validated_data::Field<bool>,
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct XcvrOverheat<'a, Mode> {
        pub detection: ::validated_data::Field<bool>,
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct XcvrPowerUnsupported<'a, Mode> {
        pub detection: ::validated_data::Field<bool>,
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct XcvrUnsupported<'a, Mode> {
        pub recovery: ::validated_data::Field<bool>,
        pub recovery_interval: ::validated_data::Field<i64>,
    }
}
