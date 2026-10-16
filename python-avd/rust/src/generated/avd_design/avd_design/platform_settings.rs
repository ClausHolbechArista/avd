// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem", schema = Dict(1727)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = List(761))]
    pub platforms: ::validated_data::Field<item::Platforms<'a, Mode>>,
    #[data_view(schema = Str(1610))]
    pub trident_forwarding_table_partition: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1716))]
    pub reload_delay: ::validated_data::Field<item::ReloadDelay<'a, Mode>>,
    #[data_view(schema = Str(1611))]
    pub tcam_profile: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(762))]
    pub additional_tcam_profiles: ::validated_data::Field<item::AdditionalTcamProfiles<'a, Mode>>,
    #[data_view(schema = Bool(7))]
    pub lag_hardware_only: ::validated_data::Field<bool>,
    #[data_view(schema = Int(623))]
    pub default_interface_mtu: ::validated_data::Field<i64>,
    #[data_view(schema = Int(624))]
    pub p2p_uplinks_mtu: ::validated_data::Field<i64>,
    #[data_view(schema = Dict(1724))]
    pub feature_support: ::validated_data::Field<item::FeatureSupport<'a, Mode>>,
    #[data_view(schema = Str(1612))]
    pub management_interface: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1725))]
    pub security_entropy_sources: ::validated_data::Field<item::SecurityEntropySources<'a, Mode>>,
    #[data_view(schema = Dict(1726))]
    pub digital_twin: ::validated_data::Field<item::DigitalTwin<'a, Mode>>,
    #[data_view(schema = Dict(1695), relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_config::EosConfig<'a, ::validated_data::RelaxedValidated>>,
    #[data_view(schema = Str(1568))]
    pub raw_eos_cli: ::validated_data::Field<&'a str>,
}

pub mod item {

    #[::validated_data::data_view(list, python(name = "AVDDesign.PlatformSettingsItem.Platforms", schema = List(761)))]
    pub struct Platforms<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.ReloadDelay", schema = Dict(1716)))]
    pub struct ReloadDelay<'a, Mode> {
        #[data_view(schema = Int(622))]
        pub mlag: ::validated_data::Field<i64>,
        #[data_view(schema = Int(622))]
        pub non_mlag: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(list, python(name = "AVDDesign.PlatformSettingsItem.AdditionalTcamProfiles", schema = List(762)))]
    pub struct AdditionalTcamProfiles<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport", schema = Dict(1724)))]
    pub struct FeatureSupport<'a, Mode> {
        #[data_view(schema = Dict(1717))]
        pub address_locking: ::validated_data::Field<feature_support::AddressLocking<'a, Mode>>,
        #[data_view(schema = Bool(617))]
        pub queue_monitor: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(618))]
        pub queue_monitor_length_notify: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(619))]
        pub interface_storm_control: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(620))]
        pub poe: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(621))]
        pub subinterface_mtu: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(622))]
        pub subinterface_monitor_session: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(623))]
        pub per_interface_mtu: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(624))]
        pub per_interface_l2_mtu: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(625))]
        pub per_interface_l2_mru: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(626))]
        pub bgp_update_wait_install: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(627))]
        pub bgp_update_wait_for_convergence: ::validated_data::Field<bool>,
        #[data_view(schema = Dict(1718))]
        pub platform_sfe_interface_profile: ::validated_data::Field<feature_support::PlatformSfeInterfaceProfile<'a, Mode>>,
        #[data_view(schema = Bool(629))]
        pub evpn_gateway_all_active_multihoming: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(630))]
        pub evpn_gateway_rd_rt_rewrite: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(631))]
        pub hardware_counters: ::validated_data::Field<bool>,
        #[data_view(schema = Dict(1719))]
        pub hardware_counter_features: ::validated_data::Field<feature_support::HardwareCounterFeatures<'a, Mode>>,
        #[data_view(schema = Bool(632))]
        pub hardware_speed_group: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(633))]
        pub private_vlan: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(634))]
        pub sflow: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(635))]
        pub sflow_subinterfaces: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(636))]
        pub wan: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(637))]
        pub ptp: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(638))]
        pub hardware_validation: ::validated_data::Field<bool>,
        #[data_view(schema = Dict(1723))]
        pub errdisable_causes: ::validated_data::Field<feature_support::ErrdisableCauses<'a, Mode>>,
    }

    pub mod feature_support {

        #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.AddressLocking", schema = Dict(1717)))]
        pub struct AddressLocking<'a, Mode> {
            #[data_view(schema = Bool(612))]
            pub supported: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(613))]
            pub ipv4_enforcement_disabled: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(614))]
            pub ipv6_enforcement_disabled: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(615))]
            pub ipv6_ethernet_interface: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(616))]
            pub ipv6_vlan: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.PlatformSfeInterfaceProfile", schema = Dict(1718)))]
        pub struct PlatformSfeInterfaceProfile<'a, Mode> {
            #[data_view(schema = Bool(628))]
            pub supported: ::validated_data::Field<bool>,
            #[data_view(schema = Int(625))]
            pub max_rx_queues: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.HardwareCounterFeatures", schema = Dict(1719)))]
        pub struct HardwareCounterFeatures<'a, Mode> {
            #[data_view(schema = Bool(37))]
            pub acl: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub decap_group: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub directflow: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub ecn: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub flow_spec: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub gre_tunnel_interface: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub ip: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub mpls_interface: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub mpls_lfib: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub mpls_tunnel: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub multicast: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub nexthop: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub pbr: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub pdp: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub policing_interface: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub qos: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub qos_dual_rate_policer: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub route: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub routed_port: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub segment_security: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub subinterface: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub tapagg: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub traffic_class: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub traffic_policy: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub vlan: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub vlan_interface: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub vni_decap: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub vni_encap: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub vtep_decap: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub vtep_encap: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses", schema = Dict(1723)))]
        pub struct ErrdisableCauses<'a, Mode> {
            #[data_view(schema = Dict(1720))]
            pub acl: ::validated_data::Field<errdisable_causes::Acl<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub arp_inspection: ::validated_data::Field<errdisable_causes::ArpInspection<'a, Mode>>,
            #[data_view(schema = Dict(1721))]
            pub bpduguard: ::validated_data::Field<errdisable_causes::Bpduguard<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub dot1x: ::validated_data::Field<errdisable_causes::Dot1x<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub dot1x_coa: ::validated_data::Field<errdisable_causes::Dot1xCoa<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub dot1x_phone_classification: ::validated_data::Field<errdisable_causes::Dot1xPhoneClassification<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub dot1x_session_replace: ::validated_data::Field<errdisable_causes::Dot1xSessionReplace<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub error_correction_encoding: ::validated_data::Field<errdisable_causes::ErrorCorrectionEncoding<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub fabric_capacity_low: ::validated_data::Field<errdisable_causes::FabricCapacityLow<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub hardware_speed_group: ::validated_data::Field<errdisable_causes::HardwareSpeedGroup<'a, Mode>>,
            #[data_view(schema = Dict(1721))]
            pub hitless_reload_down: ::validated_data::Field<errdisable_causes::HitlessReloadDown<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub interface_speed: ::validated_data::Field<errdisable_causes::InterfaceSpeed<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub internal_error: ::validated_data::Field<errdisable_causes::InternalError<'a, Mode>>,
            #[data_view(schema = Dict(1721))]
            pub lacp_rate_limit: ::validated_data::Field<errdisable_causes::LacpRateLimit<'a, Mode>>,
            #[data_view(schema = Dict(1722))]
            pub link_change: ::validated_data::Field<errdisable_causes::LinkChange<'a, Mode>>,
            #[data_view(schema = Dict(1721))]
            pub link_flap: ::validated_data::Field<errdisable_causes::LinkFlap<'a, Mode>>,
            #[data_view(schema = Dict(1721))]
            pub no_internal_vlan: ::validated_data::Field<errdisable_causes::NoInternalVlan<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub port_breakout: ::validated_data::Field<errdisable_causes::PortBreakout<'a, Mode>>,
            #[data_view(schema = Dict(1721))]
            pub portchannelguard: ::validated_data::Field<errdisable_causes::Portchannelguard<'a, Mode>>,
            #[data_view(schema = Dict(1721))]
            pub portsec: ::validated_data::Field<errdisable_causes::Portsec<'a, Mode>>,
            #[data_view(schema = Dict(1721))]
            pub speed_misconfigured: ::validated_data::Field<errdisable_causes::SpeedMisconfigured<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub storm_control: ::validated_data::Field<errdisable_causes::StormControl<'a, Mode>>,
            #[data_view(schema = Dict(1721))]
            pub stuck_queue: ::validated_data::Field<errdisable_causes::StuckQueue<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub switchcard_unreachable: ::validated_data::Field<errdisable_causes::SwitchcardUnreachable<'a, Mode>>,
            #[data_view(schema = Dict(1721))]
            pub tap_port_init: ::validated_data::Field<errdisable_causes::TapPortInit<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub tapagg: ::validated_data::Field<errdisable_causes::Tapagg<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub tpid: ::validated_data::Field<errdisable_causes::Tpid<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub transceiver_adapter: ::validated_data::Field<errdisable_causes::TransceiverAdapter<'a, Mode>>,
            #[data_view(schema = Dict(1721))]
            pub uplink_failure_detection: ::validated_data::Field<errdisable_causes::UplinkFailureDetection<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub xcvr_misconfigured: ::validated_data::Field<errdisable_causes::XcvrMisconfigured<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub xcvr_overheat: ::validated_data::Field<errdisable_causes::XcvrOverheat<'a, Mode>>,
            #[data_view(schema = Dict(1720))]
            pub xcvr_power_unsupported: ::validated_data::Field<errdisable_causes::XcvrPowerUnsupported<'a, Mode>>,
            #[data_view(schema = Dict(1721))]
            pub xcvr_unsupported: ::validated_data::Field<errdisable_causes::XcvrUnsupported<'a, Mode>>,
        }

        pub mod errdisable_causes {

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.Acl", schema = Dict(1720)))]
            pub struct Acl<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.ArpInspection", schema = Dict(1720)))]
            pub struct ArpInspection<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.Bpduguard", schema = Dict(1721)))]
            pub struct Bpduguard<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.Dot1x", schema = Dict(1720)))]
            pub struct Dot1x<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.Dot1xCoa", schema = Dict(1720)))]
            pub struct Dot1xCoa<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.Dot1xPhoneClassification", schema = Dict(1720)))]
            pub struct Dot1xPhoneClassification<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.Dot1xSessionReplace", schema = Dict(1720)))]
            pub struct Dot1xSessionReplace<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.ErrorCorrectionEncoding", schema = Dict(1720)))]
            pub struct ErrorCorrectionEncoding<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.FabricCapacityLow", schema = Dict(1720)))]
            pub struct FabricCapacityLow<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.HardwareSpeedGroup", schema = Dict(1720)))]
            pub struct HardwareSpeedGroup<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.HitlessReloadDown", schema = Dict(1721)))]
            pub struct HitlessReloadDown<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.InterfaceSpeed", schema = Dict(1720)))]
            pub struct InterfaceSpeed<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.InternalError", schema = Dict(1720)))]
            pub struct InternalError<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.LacpRateLimit", schema = Dict(1721)))]
            pub struct LacpRateLimit<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.LinkChange", schema = Dict(1722)))]
            pub struct LinkChange<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.LinkFlap", schema = Dict(1721)))]
            pub struct LinkFlap<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.NoInternalVlan", schema = Dict(1721)))]
            pub struct NoInternalVlan<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.PortBreakout", schema = Dict(1720)))]
            pub struct PortBreakout<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.Portchannelguard", schema = Dict(1721)))]
            pub struct Portchannelguard<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.Portsec", schema = Dict(1721)))]
            pub struct Portsec<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.SpeedMisconfigured", schema = Dict(1721)))]
            pub struct SpeedMisconfigured<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.StormControl", schema = Dict(1720)))]
            pub struct StormControl<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.StuckQueue", schema = Dict(1721)))]
            pub struct StuckQueue<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.SwitchcardUnreachable", schema = Dict(1720)))]
            pub struct SwitchcardUnreachable<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.TapPortInit", schema = Dict(1721)))]
            pub struct TapPortInit<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.Tapagg", schema = Dict(1720)))]
            pub struct Tapagg<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.Tpid", schema = Dict(1720)))]
            pub struct Tpid<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.TransceiverAdapter", schema = Dict(1720)))]
            pub struct TransceiverAdapter<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.UplinkFailureDetection", schema = Dict(1721)))]
            pub struct UplinkFailureDetection<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.XcvrMisconfigured", schema = Dict(1720)))]
            pub struct XcvrMisconfigured<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.XcvrOverheat", schema = Dict(1720)))]
            pub struct XcvrOverheat<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.XcvrPowerUnsupported", schema = Dict(1720)))]
            pub struct XcvrPowerUnsupported<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub detection: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.XcvrUnsupported", schema = Dict(1721)))]
            pub struct XcvrUnsupported<'a, Mode> {
                #[data_view(schema = Bool(37))]
                pub recovery: ::validated_data::Field<bool>,
            }
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.SecurityEntropySources", schema = Dict(1725)))]
    pub struct SecurityEntropySources<'a, Mode> {
        #[data_view(schema = Bool(249))]
        pub hardware: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(250))]
        pub haveged: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(251))]
        pub cpu_jitter: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(252))]
        pub hardware_exclusive: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.DigitalTwin", schema = Dict(1726)))]
    pub struct DigitalTwin<'a, Mode> {
        #[data_view(schema = Str(1613))]
        pub platform: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1614))]
        pub act_node_type: ::validated_data::Field<&'a str>,
    }
}
