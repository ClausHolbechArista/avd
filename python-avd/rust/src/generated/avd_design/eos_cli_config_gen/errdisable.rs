// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Detect<'a, Mode> {
    pub causes: ::validated_data::Field<detect::Causes<'a, Mode>>,
}

pub mod detect {

    #[::validated_data::data_view(list)]
    pub struct Causes<'a, Mode> (::validated_data::Field<&'a str>);
}

#[::validated_data::data_view]
pub struct DetectCause<'a, Mode> {
    pub acl: ::validated_data::Field<bool>,
    pub arp_inspection: ::validated_data::Field<bool>,
    pub dot1x: ::validated_data::Field<bool>,
    pub dot1x_coa: ::validated_data::Field<bool>,
    pub dot1x_phone_classification: ::validated_data::Field<bool>,
    pub dot1x_session_replace: ::validated_data::Field<bool>,
    pub error_correction_encoding: ::validated_data::Field<bool>,
    pub fabric_capacity_low: ::validated_data::Field<bool>,
    pub hardware_speed_group: ::validated_data::Field<bool>,
    pub interface_speed: ::validated_data::Field<bool>,
    pub internal_error: ::validated_data::Field<bool>,
    pub link_change: ::validated_data::Field<bool>,
    pub port_breakout: ::validated_data::Field<bool>,
    pub storm_control: ::validated_data::Field<bool>,
    pub switchcard_unreachable: ::validated_data::Field<bool>,
    pub tapagg: ::validated_data::Field<bool>,
    pub tpid: ::validated_data::Field<bool>,
    pub transceiver_adapter: ::validated_data::Field<bool>,
    pub xcvr_misconfigured: ::validated_data::Field<bool>,
    pub xcvr_overheat: ::validated_data::Field<bool>,
    pub xcvr_power_unsupported: ::validated_data::Field<bool>,
}

#[::validated_data::data_view]
pub struct Recovery<'a, Mode> {
    pub causes: ::validated_data::Field<recovery::Causes<'a, Mode>>,
    pub interval: ::validated_data::Field<i64>,
}

pub mod recovery {

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct Causes<'a, Mode> (::validated_data::Field<causes::Item<'a, Mode>>);

    pub mod causes {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub interval: ::validated_data::Field<i64>,
        }
    }
}

#[::validated_data::data_view]
pub struct RecoveryCause<'a, Mode> {
    pub acl: ::validated_data::Field<recovery_cause::Acl<'a, Mode>>,
    pub arp_inspection: ::validated_data::Field<recovery_cause::ArpInspection<'a, Mode>>,
    pub bpduguard: ::validated_data::Field<recovery_cause::Bpduguard<'a, Mode>>,
    pub dot1x: ::validated_data::Field<recovery_cause::Dot1x<'a, Mode>>,
    pub dot1x_coa: ::validated_data::Field<recovery_cause::Dot1xCoa<'a, Mode>>,
    pub dot1x_phone_classification: ::validated_data::Field<recovery_cause::Dot1xPhoneClassification<'a, Mode>>,
    pub dot1x_session_replace: ::validated_data::Field<recovery_cause::Dot1xSessionReplace<'a, Mode>>,
    pub error_correction_encoding: ::validated_data::Field<recovery_cause::ErrorCorrectionEncoding<'a, Mode>>,
    pub fabric_capacity_low: ::validated_data::Field<recovery_cause::FabricCapacityLow<'a, Mode>>,
    pub hardware_speed_group: ::validated_data::Field<recovery_cause::HardwareSpeedGroup<'a, Mode>>,
    pub hitless_reload_down: ::validated_data::Field<recovery_cause::HitlessReloadDown<'a, Mode>>,
    pub interface_speed: ::validated_data::Field<recovery_cause::InterfaceSpeed<'a, Mode>>,
    pub internal_error: ::validated_data::Field<recovery_cause::InternalError<'a, Mode>>,
    pub lacp_rate_limit: ::validated_data::Field<recovery_cause::LacpRateLimit<'a, Mode>>,
    pub link_flap: ::validated_data::Field<recovery_cause::LinkFlap<'a, Mode>>,
    pub no_internal_vlan: ::validated_data::Field<recovery_cause::NoInternalVlan<'a, Mode>>,
    pub port_breakout: ::validated_data::Field<recovery_cause::PortBreakout<'a, Mode>>,
    pub portchannelguard: ::validated_data::Field<recovery_cause::Portchannelguard<'a, Mode>>,
    pub portsec: ::validated_data::Field<recovery_cause::Portsec<'a, Mode>>,
    pub speed_misconfigured: ::validated_data::Field<recovery_cause::SpeedMisconfigured<'a, Mode>>,
    pub storm_control: ::validated_data::Field<recovery_cause::StormControl<'a, Mode>>,
    pub stuck_queue: ::validated_data::Field<recovery_cause::StuckQueue<'a, Mode>>,
    pub switchcard_unreachable: ::validated_data::Field<recovery_cause::SwitchcardUnreachable<'a, Mode>>,
    pub tap_port_init: ::validated_data::Field<recovery_cause::TapPortInit<'a, Mode>>,
    pub tapagg: ::validated_data::Field<recovery_cause::Tapagg<'a, Mode>>,
    pub tpid: ::validated_data::Field<recovery_cause::Tpid<'a, Mode>>,
    pub transceiver_adapter: ::validated_data::Field<recovery_cause::TransceiverAdapter<'a, Mode>>,
    pub uplink_failure_detection: ::validated_data::Field<recovery_cause::UplinkFailureDetection<'a, Mode>>,
    pub xcvr_misconfigured: ::validated_data::Field<recovery_cause::XcvrMisconfigured<'a, Mode>>,
    pub xcvr_overheat: ::validated_data::Field<recovery_cause::XcvrOverheat<'a, Mode>>,
    pub xcvr_power_unsupported: ::validated_data::Field<recovery_cause::XcvrPowerUnsupported<'a, Mode>>,
    pub xcvr_unsupported: ::validated_data::Field<recovery_cause::XcvrUnsupported<'a, Mode>>,
}

pub mod recovery_cause {

    #[::validated_data::data_view]
    pub struct Acl<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct ArpInspection<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Bpduguard<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Dot1x<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Dot1xCoa<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Dot1xPhoneClassification<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Dot1xSessionReplace<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct ErrorCorrectionEncoding<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct FabricCapacityLow<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct HardwareSpeedGroup<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct HitlessReloadDown<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct InterfaceSpeed<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct InternalError<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct LacpRateLimit<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct LinkFlap<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct NoInternalVlan<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct PortBreakout<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Portchannelguard<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Portsec<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct SpeedMisconfigured<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct StormControl<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct StuckQueue<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct SwitchcardUnreachable<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct TapPortInit<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Tapagg<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Tpid<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct TransceiverAdapter<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct UplinkFailureDetection<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct XcvrMisconfigured<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct XcvrOverheat<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct XcvrPowerUnsupported<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct XcvrUnsupported<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
    }
}
