// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(list, python(name = "AVDDesign.CustomPlatformSettings", schema = List(763)))]
pub struct CustomPlatformSettings<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem", schema = Dict(1727)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = List(761))]
    pub platforms: ::data_views::Field<item::Platforms<'a, Mode>>,
    #[data_view(schema = Str(1610))]
    pub trident_forwarding_table_partition: ::data_views::Field<&'a str>,
    #[data_view(schema = Dict(1716))]
    pub reload_delay: ::data_views::Field<item::ReloadDelay<'a, Mode>>,
    #[data_view(schema = Str(1611))]
    pub tcam_profile: ::data_views::Field<&'a str>,
    #[data_view(schema = List(762))]
    pub additional_tcam_profiles: ::data_views::Field<item::AdditionalTcamProfiles<'a, Mode>>,
    #[data_view(schema = Bool(7))]
    pub lag_hardware_only: ::data_views::Field<bool>,
    #[data_view(schema = Int(623))]
    pub default_interface_mtu: ::data_views::Field<i64>,
    #[data_view(schema = Int(624))]
    pub p2p_uplinks_mtu: ::data_views::Field<i64>,
    #[data_view(schema = Dict(1724))]
    pub feature_support: ::data_views::Field<item::FeatureSupport<'a, Mode>>,
    #[data_view(schema = Str(1612))]
    pub management_interface: ::data_views::Field<&'a str>,
    #[data_view(schema = Dict(1725))]
    pub security_entropy_sources: ::data_views::Field<item::SecurityEntropySources<'a, Mode>>,
    #[data_view(schema = Dict(1726))]
    pub digital_twin: ::data_views::Field<item::DigitalTwin<'a, Mode>>,
    #[data_view(schema = Dict(1695), relaxed)]
    pub structured_config: ::data_views::Field<super::super::eos_config::EosConfig<'a, ::data_views::RelaxedValidated>>,
    #[data_view(schema = Str(1568))]
    pub raw_eos_cli: ::data_views::Field<&'a str>,
}

pub mod item {

    #[::data_views::data_view(list, python(name = "AVDDesign.CustomPlatformSettingsItem.Platforms", schema = List(761)))]
    pub struct Platforms<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.ReloadDelay", schema = Dict(1716)))]
    pub struct ReloadDelay<'a, Mode> {
        #[data_view(schema = Int(622))]
        pub mlag: ::data_views::Field<i64>,
        #[data_view(schema = Int(622))]
        pub non_mlag: ::data_views::Field<i64>,
    }

    #[::data_views::data_view(list, python(name = "AVDDesign.CustomPlatformSettingsItem.AdditionalTcamProfiles", schema = List(762)))]
    pub struct AdditionalTcamProfiles<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport", schema = Dict(1724)))]
    pub struct FeatureSupport<'a, Mode> {
        #[data_view(schema = Dict(1717))]
        pub address_locking: ::data_views::Field<feature_support::AddressLocking<'a, Mode>>,
        #[data_view(schema = Bool(617))]
        pub queue_monitor: ::data_views::Field<bool>,
        #[data_view(schema = Bool(618))]
        pub queue_monitor_length_notify: ::data_views::Field<bool>,
        #[data_view(schema = Bool(619))]
        pub interface_storm_control: ::data_views::Field<bool>,
        #[data_view(schema = Bool(620))]
        pub poe: ::data_views::Field<bool>,
        #[data_view(schema = Bool(621))]
        pub subinterface_mtu: ::data_views::Field<bool>,
        #[data_view(schema = Bool(622))]
        pub subinterface_monitor_session: ::data_views::Field<bool>,
        #[data_view(schema = Bool(623))]
        pub per_interface_mtu: ::data_views::Field<bool>,
        #[data_view(schema = Bool(624))]
        pub per_interface_l2_mtu: ::data_views::Field<bool>,
        #[data_view(schema = Bool(625))]
        pub per_interface_l2_mru: ::data_views::Field<bool>,
        #[data_view(schema = Bool(626))]
        pub bgp_update_wait_install: ::data_views::Field<bool>,
        #[data_view(schema = Bool(627))]
        pub bgp_update_wait_for_convergence: ::data_views::Field<bool>,
        #[data_view(schema = Dict(1718))]
        pub platform_sfe_interface_profile: ::data_views::Field<feature_support::PlatformSfeInterfaceProfile<'a, Mode>>,
        #[data_view(schema = Bool(629))]
        pub evpn_gateway_all_active_multihoming: ::data_views::Field<bool>,
        #[data_view(schema = Bool(630))]
        pub evpn_gateway_rd_rt_rewrite: ::data_views::Field<bool>,
        #[data_view(schema = Bool(631))]
        pub hardware_counters: ::data_views::Field<bool>,
        #[data_view(schema = Dict(1719))]
        pub hardware_counter_features: ::data_views::Field<feature_support::HardwareCounterFeatures<'a, Mode>>,
        #[data_view(schema = Bool(632))]
        pub hardware_speed_group: ::data_views::Field<bool>,
        #[data_view(schema = Bool(633))]
        pub private_vlan: ::data_views::Field<bool>,
        #[data_view(schema = Bool(634))]
        pub sflow: ::data_views::Field<bool>,
        #[data_view(schema = Bool(635))]
        pub sflow_subinterfaces: ::data_views::Field<bool>,
        #[data_view(schema = Bool(636))]
        pub wan: ::data_views::Field<bool>,
        #[data_view(schema = Bool(637))]
        pub ptp: ::data_views::Field<bool>,
        #[data_view(schema = Bool(638))]
        pub hardware_validation: ::data_views::Field<bool>,
        #[data_view(schema = Dict(1723))]
        pub errdisable_causes: ::data_views::Field<feature_support::ErrdisableCauses<'a, Mode>>,
    }

    pub mod feature_support {

        #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.AddressLocking", schema = Dict(1717)))]
        pub struct AddressLocking<'a, Mode> {
            #[data_view(schema = Bool(612))]
            pub supported: ::data_views::Field<bool>,
            #[data_view(schema = Bool(613))]
            pub ipv4_enforcement_disabled: ::data_views::Field<bool>,
            #[data_view(schema = Bool(614))]
            pub ipv6_enforcement_disabled: ::data_views::Field<bool>,
            #[data_view(schema = Bool(615))]
            pub ipv6_ethernet_interface: ::data_views::Field<bool>,
            #[data_view(schema = Bool(616))]
            pub ipv6_vlan: ::data_views::Field<bool>,
        }

        #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.PlatformSfeInterfaceProfile", schema = Dict(1718)))]
        pub struct PlatformSfeInterfaceProfile<'a, Mode> {
            #[data_view(schema = Bool(628))]
            pub supported: ::data_views::Field<bool>,
            #[data_view(schema = Int(625))]
            pub max_rx_queues: ::data_views::Field<i64>,
        }

        #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.HardwareCounterFeatures", schema = Dict(1719)))]
        pub struct HardwareCounterFeatures<'a, Mode> {
            #[data_view(schema = Bool(37))]
            pub acl: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub decap_group: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub directflow: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub ecn: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub flow_spec: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub gre_tunnel_interface: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub ip: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub mpls_interface: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub mpls_lfib: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub mpls_tunnel: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub multicast: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub nexthop: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub pbr: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub pdp: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub policing_interface: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub qos: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub qos_dual_rate_policer: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub route: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub routed_port: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub segment_security: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub subinterface: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub tapagg: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub traffic_class: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub traffic_policy: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub vlan: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub vlan_interface: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub vni_decap: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub vni_encap: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub vtep_decap: ::data_views::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub vtep_encap: ::data_views::Field<bool>,
        }

        #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses", schema = Dict(1723)))]
        pub struct ErrdisableCauses<'a, Mode> {
            #[data_view(schema = Dict(1720))]
            pub acl: ::data_views::Field<errdisable_causes::Acl<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub arp_inspection: ::data_views::Field<errdisable_causes::ArpInspection<'a, Mode>>,
            #[data_view(schema = Dict(1721))]
            pub bpduguard: ::data_views::Field<errdisable_causes::Bpduguard<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub dot1x: ::data_views::Field<errdisable_causes::Dot1x<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub dot1x_coa: ::data_views::Field<errdisable_causes::Dot1xCoa<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub dot1x_phone_classification: ::data_views::Field<errdisable_causes::Dot1xPhoneClassification<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub dot1x_session_replace: ::data_views::Field<errdisable_causes::Dot1xSessionReplace<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub error_correction_encoding: ::data_views::Field<errdisable_causes::ErrorCorrectionEncoding<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub fabric_capacity_low: ::data_views::Field<errdisable_causes::FabricCapacityLow<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub hardware_speed_group: ::data_views::Field<errdisable_causes::HardwareSpeedGroup<'a, Mode>>,
            #[data_view(schema = Dict(1721))]
            pub hitless_reload_down: ::data_views::Field<errdisable_causes::HitlessReloadDown<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub interface_speed: ::data_views::Field<errdisable_causes::InterfaceSpeed<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub internal_error: ::data_views::Field<errdisable_causes::InternalError<'a, Mode>>,
            #[data_view(schema = Dict(1721))]
            pub lacp_rate_limit: ::data_views::Field<errdisable_causes::LacpRateLimit<'a, Mode>>,
            #[data_view(schema = Dict(1722))]
            pub link_change: ::data_views::Field<errdisable_causes::LinkChange<'a, Mode>>,
            #[data_view(schema = Dict(1721))]
            pub link_flap: ::data_views::Field<errdisable_causes::LinkFlap<'a, Mode>>,
            #[data_view(schema = Dict(1721))]
            pub no_internal_vlan: ::data_views::Field<errdisable_causes::NoInternalVlan<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub port_breakout: ::data_views::Field<errdisable_causes::PortBreakout<'a, Mode>>,
            #[data_view(schema = Dict(1721))]
            pub portchannelguard: ::data_views::Field<errdisable_causes::Portchannelguard<'a, Mode>>,
            #[data_view(schema = Dict(1721))]
            pub portsec: ::data_views::Field<errdisable_causes::Portsec<'a, Mode>>,
            #[data_view(schema = Dict(1721))]
            pub speed_misconfigured: ::data_views::Field<errdisable_causes::SpeedMisconfigured<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub storm_control: ::data_views::Field<errdisable_causes::StormControl<'a, Mode>>,
            #[data_view(schema = Dict(1721))]
            pub stuck_queue: ::data_views::Field<errdisable_causes::StuckQueue<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub switchcard_unreachable: ::data_views::Field<errdisable_causes::SwitchcardUnreachable<'a, Mode>>,
            #[data_view(schema = Dict(1721))]
            pub tap_port_init: ::data_views::Field<errdisable_causes::TapPortInit<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub tapagg: ::data_views::Field<errdisable_causes::Tapagg<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub tpid: ::data_views::Field<errdisable_causes::Tpid<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub transceiver_adapter: ::data_views::Field<errdisable_causes::TransceiverAdapter<'a, Mode>>,
            #[data_view(schema = Dict(1721))]
            pub uplink_failure_detection: ::data_views::Field<errdisable_causes::UplinkFailureDetection<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub xcvr_misconfigured: ::data_views::Field<errdisable_causes::XcvrMisconfigured<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub xcvr_overheat: ::data_views::Field<errdisable_causes::XcvrOverheat<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub xcvr_power_unsupported: ::data_views::Field<errdisable_causes::XcvrPowerUnsupported<'a, Mode>>,
            #[data_view(schema = Dict(1721))]
            pub xcvr_unsupported: ::data_views::Field<errdisable_causes::XcvrUnsupported<'a, Mode>>,
        }

        pub mod errdisable_causes {

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.Acl", schema = Dict(1720)))]
            pub struct Acl<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::data_views::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.ArpInspection", schema = Dict(1720)))]
            pub struct ArpInspection<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::data_views::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.Bpduguard", schema = Dict(1721)))]
            pub struct Bpduguard<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.Dot1x", schema = Dict(1720)))]
            pub struct Dot1x<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::data_views::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.Dot1xCoa", schema = Dict(1720)))]
            pub struct Dot1xCoa<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::data_views::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.Dot1xPhoneClassification", schema = Dict(1720)))]
            pub struct Dot1xPhoneClassification<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::data_views::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.Dot1xSessionReplace", schema = Dict(1720)))]
            pub struct Dot1xSessionReplace<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::data_views::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.ErrorCorrectionEncoding", schema = Dict(1720)))]
            pub struct ErrorCorrectionEncoding<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::data_views::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.FabricCapacityLow", schema = Dict(1720)))]
            pub struct FabricCapacityLow<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::data_views::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.HardwareSpeedGroup", schema = Dict(1720)))]
            pub struct HardwareSpeedGroup<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::data_views::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.HitlessReloadDown", schema = Dict(1721)))]
            pub struct HitlessReloadDown<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.InterfaceSpeed", schema = Dict(1720)))]
            pub struct InterfaceSpeed<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::data_views::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.InternalError", schema = Dict(1720)))]
            pub struct InternalError<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::data_views::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.LacpRateLimit", schema = Dict(1721)))]
            pub struct LacpRateLimit<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.LinkChange", schema = Dict(1722)))]
            pub struct LinkChange<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.LinkFlap", schema = Dict(1721)))]
            pub struct LinkFlap<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.NoInternalVlan", schema = Dict(1721)))]
            pub struct NoInternalVlan<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.PortBreakout", schema = Dict(1720)))]
            pub struct PortBreakout<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::data_views::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.Portchannelguard", schema = Dict(1721)))]
            pub struct Portchannelguard<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.Portsec", schema = Dict(1721)))]
            pub struct Portsec<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.SpeedMisconfigured", schema = Dict(1721)))]
            pub struct SpeedMisconfigured<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.StormControl", schema = Dict(1720)))]
            pub struct StormControl<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::data_views::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.StuckQueue", schema = Dict(1721)))]
            pub struct StuckQueue<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.SwitchcardUnreachable", schema = Dict(1720)))]
            pub struct SwitchcardUnreachable<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::data_views::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.TapPortInit", schema = Dict(1721)))]
            pub struct TapPortInit<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.Tapagg", schema = Dict(1720)))]
            pub struct Tapagg<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::data_views::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.Tpid", schema = Dict(1720)))]
            pub struct Tpid<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::data_views::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.TransceiverAdapter", schema = Dict(1720)))]
            pub struct TransceiverAdapter<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::data_views::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.UplinkFailureDetection", schema = Dict(1721)))]
            pub struct UplinkFailureDetection<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.XcvrMisconfigured", schema = Dict(1720)))]
            pub struct XcvrMisconfigured<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::data_views::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.XcvrOverheat", schema = Dict(1720)))]
            pub struct XcvrOverheat<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::data_views::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.XcvrPowerUnsupported", schema = Dict(1720)))]
            pub struct XcvrPowerUnsupported<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::data_views::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.FeatureSupport.ErrdisableCauses.XcvrUnsupported", schema = Dict(1721)))]
            pub struct XcvrUnsupported<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub recovery: ::data_views::Field<bool>,
            }
        }
    }

    #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.SecurityEntropySources", schema = Dict(1725)))]
    pub struct SecurityEntropySources<'a, Mode> {
        #[data_view(schema = Bool(249))]
        pub hardware: ::data_views::Field<bool>,
        #[data_view(schema = Bool(250))]
        pub haveged: ::data_views::Field<bool>,
        #[data_view(schema = Bool(251))]
        pub cpu_jitter: ::data_views::Field<bool>,
        #[data_view(schema = Bool(252))]
        pub hardware_exclusive: ::data_views::Field<bool>,
    }

    #[::data_views::data_view(python(name = "AVDDesign.CustomPlatformSettingsItem.DigitalTwin", schema = Dict(1726)))]
    pub struct DigitalTwin<'a, Mode> {
        #[data_view(schema = Str(1613))]
        pub platform: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1614))]
        pub act_node_type: ::data_views::Field<&'a str>,
    }
}
