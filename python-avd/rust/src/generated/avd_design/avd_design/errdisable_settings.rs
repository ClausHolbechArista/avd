// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes", schema = Dict(1549)))]
pub struct Causes<'a, Mode> {
    #[data_view(schema = Dict(1546))]
    pub acl: ::validated_data::Field<causes::Acl<'a, Mode>>,
    #[data_view(schema = Dict(1546))]
    pub arp_inspection: ::validated_data::Field<causes::ArpInspection<'a, Mode>>,
    #[data_view(schema = Dict(1547))]
    pub bpduguard: ::validated_data::Field<causes::Bpduguard<'a, Mode>>,
    #[data_view(schema = Dict(1546))]
    pub dot1x: ::validated_data::Field<causes::Dot1x<'a, Mode>>,
    #[data_view(schema = Dict(1546))]
    pub dot1x_coa: ::validated_data::Field<causes::Dot1xCoa<'a, Mode>>,
    #[data_view(schema = Dict(1546))]
    pub dot1x_phone_classification: ::validated_data::Field<causes::Dot1xPhoneClassification<'a, Mode>>,
    #[data_view(schema = Dict(1546))]
    pub dot1x_session_replace: ::validated_data::Field<causes::Dot1xSessionReplace<'a, Mode>>,
    #[data_view(schema = Dict(1546))]
    pub error_correction_encoding: ::validated_data::Field<causes::ErrorCorrectionEncoding<'a, Mode>>,
    #[data_view(schema = Dict(1546))]
    pub fabric_capacity_low: ::validated_data::Field<causes::FabricCapacityLow<'a, Mode>>,
    #[data_view(schema = Dict(1546))]
    pub hardware_speed_group: ::validated_data::Field<causes::HardwareSpeedGroup<'a, Mode>>,
    #[data_view(schema = Dict(1547))]
    pub hitless_reload_down: ::validated_data::Field<causes::HitlessReloadDown<'a, Mode>>,
    #[data_view(schema = Dict(1546))]
    pub interface_speed: ::validated_data::Field<causes::InterfaceSpeed<'a, Mode>>,
    #[data_view(schema = Dict(1546))]
    pub internal_error: ::validated_data::Field<causes::InternalError<'a, Mode>>,
    #[data_view(schema = Dict(1547))]
    pub lacp_rate_limit: ::validated_data::Field<causes::LacpRateLimit<'a, Mode>>,
    #[data_view(schema = Dict(1548))]
    pub link_change: ::validated_data::Field<causes::LinkChange<'a, Mode>>,
    #[data_view(schema = Dict(1547))]
    pub link_flap: ::validated_data::Field<causes::LinkFlap<'a, Mode>>,
    #[data_view(schema = Dict(1547))]
    pub no_internal_vlan: ::validated_data::Field<causes::NoInternalVlan<'a, Mode>>,
    #[data_view(schema = Dict(1546))]
    pub port_breakout: ::validated_data::Field<causes::PortBreakout<'a, Mode>>,
    #[data_view(schema = Dict(1547))]
    pub portchannelguard: ::validated_data::Field<causes::Portchannelguard<'a, Mode>>,
    #[data_view(schema = Dict(1547))]
    pub portsec: ::validated_data::Field<causes::Portsec<'a, Mode>>,
    #[data_view(schema = Dict(1547))]
    pub speed_misconfigured: ::validated_data::Field<causes::SpeedMisconfigured<'a, Mode>>,
    #[data_view(schema = Dict(1546))]
    pub storm_control: ::validated_data::Field<causes::StormControl<'a, Mode>>,
    #[data_view(schema = Dict(1547))]
    pub stuck_queue: ::validated_data::Field<causes::StuckQueue<'a, Mode>>,
    #[data_view(schema = Dict(1546))]
    pub switchcard_unreachable: ::validated_data::Field<causes::SwitchcardUnreachable<'a, Mode>>,
    #[data_view(schema = Dict(1547))]
    pub tap_port_init: ::validated_data::Field<causes::TapPortInit<'a, Mode>>,
    #[data_view(schema = Dict(1546))]
    pub tapagg: ::validated_data::Field<causes::Tapagg<'a, Mode>>,
    #[data_view(schema = Dict(1546))]
    pub tpid: ::validated_data::Field<causes::Tpid<'a, Mode>>,
    #[data_view(schema = Dict(1546))]
    pub transceiver_adapter: ::validated_data::Field<causes::TransceiverAdapter<'a, Mode>>,
    #[data_view(schema = Dict(1547))]
    pub uplink_failure_detection: ::validated_data::Field<causes::UplinkFailureDetection<'a, Mode>>,
    #[data_view(schema = Dict(1546))]
    pub xcvr_misconfigured: ::validated_data::Field<causes::XcvrMisconfigured<'a, Mode>>,
    #[data_view(schema = Dict(1546))]
    pub xcvr_overheat: ::validated_data::Field<causes::XcvrOverheat<'a, Mode>>,
    #[data_view(schema = Dict(1546))]
    pub xcvr_power_unsupported: ::validated_data::Field<causes::XcvrPowerUnsupported<'a, Mode>>,
    #[data_view(schema = Dict(1547))]
    pub xcvr_unsupported: ::validated_data::Field<causes::XcvrUnsupported<'a, Mode>>,
}

pub mod causes {

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.Acl", schema = Dict(1546)))]
    pub struct Acl<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub detection: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.ArpInspection", schema = Dict(1546)))]
    pub struct ArpInspection<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub detection: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.Bpduguard", schema = Dict(1547)))]
    pub struct Bpduguard<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.Dot1x", schema = Dict(1546)))]
    pub struct Dot1x<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub detection: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.Dot1xCoa", schema = Dict(1546)))]
    pub struct Dot1xCoa<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub detection: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.Dot1xPhoneClassification", schema = Dict(1546)))]
    pub struct Dot1xPhoneClassification<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub detection: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.Dot1xSessionReplace", schema = Dict(1546)))]
    pub struct Dot1xSessionReplace<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub detection: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.ErrorCorrectionEncoding", schema = Dict(1546)))]
    pub struct ErrorCorrectionEncoding<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub detection: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.FabricCapacityLow", schema = Dict(1546)))]
    pub struct FabricCapacityLow<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub detection: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.HardwareSpeedGroup", schema = Dict(1546)))]
    pub struct HardwareSpeedGroup<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub detection: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.HitlessReloadDown", schema = Dict(1547)))]
    pub struct HitlessReloadDown<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.InterfaceSpeed", schema = Dict(1546)))]
    pub struct InterfaceSpeed<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub detection: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.InternalError", schema = Dict(1546)))]
    pub struct InternalError<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub detection: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.LacpRateLimit", schema = Dict(1547)))]
    pub struct LacpRateLimit<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.LinkChange", schema = Dict(1548)))]
    pub struct LinkChange<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub detection: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.LinkFlap", schema = Dict(1547)))]
    pub struct LinkFlap<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.NoInternalVlan", schema = Dict(1547)))]
    pub struct NoInternalVlan<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.PortBreakout", schema = Dict(1546)))]
    pub struct PortBreakout<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub detection: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.Portchannelguard", schema = Dict(1547)))]
    pub struct Portchannelguard<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.Portsec", schema = Dict(1547)))]
    pub struct Portsec<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.SpeedMisconfigured", schema = Dict(1547)))]
    pub struct SpeedMisconfigured<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.StormControl", schema = Dict(1546)))]
    pub struct StormControl<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub detection: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.StuckQueue", schema = Dict(1547)))]
    pub struct StuckQueue<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.SwitchcardUnreachable", schema = Dict(1546)))]
    pub struct SwitchcardUnreachable<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub detection: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.TapPortInit", schema = Dict(1547)))]
    pub struct TapPortInit<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.Tapagg", schema = Dict(1546)))]
    pub struct Tapagg<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub detection: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.Tpid", schema = Dict(1546)))]
    pub struct Tpid<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub detection: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.TransceiverAdapter", schema = Dict(1546)))]
    pub struct TransceiverAdapter<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub detection: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.UplinkFailureDetection", schema = Dict(1547)))]
    pub struct UplinkFailureDetection<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.XcvrMisconfigured", schema = Dict(1546)))]
    pub struct XcvrMisconfigured<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub detection: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.XcvrOverheat", schema = Dict(1546)))]
    pub struct XcvrOverheat<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub detection: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.XcvrPowerUnsupported", schema = Dict(1546)))]
    pub struct XcvrPowerUnsupported<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub detection: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings.Causes.XcvrUnsupported", schema = Dict(1547)))]
    pub struct XcvrUnsupported<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub recovery: ::validated_data::Field<bool>,
        #[data_view(schema = Int(36))]
        pub recovery_interval: ::validated_data::Field<i64>,
    }
}
