// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem"))]
pub struct Item<'a, Mode> {
    pub platforms: ::validated_data::Field<item::Platforms<'a, Mode>>,
    pub trident_forwarding_table_partition: ::validated_data::Field<&'a str>,
    pub reload_delay: ::validated_data::Field<item::ReloadDelay<'a, Mode>>,
    pub tcam_profile: ::validated_data::Field<&'a str>,
    pub additional_tcam_profiles: ::validated_data::Field<item::AdditionalTcamProfiles<'a, Mode>>,
    pub lag_hardware_only: ::validated_data::Field<bool>,
    pub default_interface_mtu: ::validated_data::Field<i64>,
    pub p2p_uplinks_mtu: ::validated_data::Field<i64>,
    pub feature_support: ::validated_data::Field<item::FeatureSupport<'a, Mode>>,
    pub management_interface: ::validated_data::Field<&'a str>,
    pub security_entropy_sources: ::validated_data::Field<item::SecurityEntropySources<'a, Mode>>,
    pub digital_twin: ::validated_data::Field<item::DigitalTwin<'a, Mode>>,
    #[data_view(relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_config::EosConfig<'a, ::validated_data::RelaxedValidated>>,
    pub raw_eos_cli: ::validated_data::Field<&'a str>,
}

pub mod item {

    #[::validated_data::data_view(list, python(name = "AVDDesign.PlatformSettingsItem.Platforms"))]
    pub struct Platforms<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.ReloadDelay"))]
    pub struct ReloadDelay<'a, Mode> {
        pub mlag: ::validated_data::Field<i64>,
        pub non_mlag: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(list, python(name = "AVDDesign.PlatformSettingsItem.AdditionalTcamProfiles"))]
    pub struct AdditionalTcamProfiles<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport"))]
    pub struct FeatureSupport<'a, Mode> {
        pub address_locking: ::validated_data::Field<feature_support::AddressLocking<'a, Mode>>,
        pub queue_monitor: ::validated_data::Field<bool>,
        pub queue_monitor_length_notify: ::validated_data::Field<bool>,
        pub interface_storm_control: ::validated_data::Field<bool>,
        pub poe: ::validated_data::Field<bool>,
        pub subinterface_mtu: ::validated_data::Field<bool>,
        pub subinterface_monitor_session: ::validated_data::Field<bool>,
        pub per_interface_mtu: ::validated_data::Field<bool>,
        pub per_interface_l2_mtu: ::validated_data::Field<bool>,
        pub per_interface_l2_mru: ::validated_data::Field<bool>,
        pub bgp_update_wait_install: ::validated_data::Field<bool>,
        pub bgp_update_wait_for_convergence: ::validated_data::Field<bool>,
        pub platform_sfe_interface_profile: ::validated_data::Field<feature_support::PlatformSfeInterfaceProfile<'a, Mode>>,
        pub evpn_gateway_all_active_multihoming: ::validated_data::Field<bool>,
        pub evpn_gateway_rd_rt_rewrite: ::validated_data::Field<bool>,
        pub hardware_counters: ::validated_data::Field<bool>,
        pub hardware_counter_features: ::validated_data::Field<feature_support::HardwareCounterFeatures<'a, Mode>>,
        pub hardware_speed_group: ::validated_data::Field<bool>,
        pub private_vlan: ::validated_data::Field<bool>,
        pub sflow: ::validated_data::Field<bool>,
        pub sflow_subinterfaces: ::validated_data::Field<bool>,
        pub wan: ::validated_data::Field<bool>,
        pub ptp: ::validated_data::Field<bool>,
        pub hardware_validation: ::validated_data::Field<bool>,
        pub errdisable_causes: ::validated_data::Field<feature_support::ErrdisableCauses<'a, Mode>>,
    }

    pub mod feature_support {

        #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.AddressLocking"))]
        pub struct AddressLocking<'a, Mode> {
            pub supported: ::validated_data::Field<bool>,
            pub ipv4_enforcement_disabled: ::validated_data::Field<bool>,
            pub ipv6_enforcement_disabled: ::validated_data::Field<bool>,
            pub ipv6_ethernet_interface: ::validated_data::Field<bool>,
            pub ipv6_vlan: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.PlatformSfeInterfaceProfile"))]
        pub struct PlatformSfeInterfaceProfile<'a, Mode> {
            pub supported: ::validated_data::Field<bool>,
            pub max_rx_queues: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.HardwareCounterFeatures"))]
        pub struct HardwareCounterFeatures<'a, Mode> {
            pub acl: ::validated_data::Field<bool>,
            pub decap_group: ::validated_data::Field<bool>,
            pub directflow: ::validated_data::Field<bool>,
            pub ecn: ::validated_data::Field<bool>,
            pub flow_spec: ::validated_data::Field<bool>,
            pub gre_tunnel_interface: ::validated_data::Field<bool>,
            pub ip: ::validated_data::Field<bool>,
            pub mpls_interface: ::validated_data::Field<bool>,
            pub mpls_lfib: ::validated_data::Field<bool>,
            pub mpls_tunnel: ::validated_data::Field<bool>,
            pub multicast: ::validated_data::Field<bool>,
            pub nexthop: ::validated_data::Field<bool>,
            pub pbr: ::validated_data::Field<bool>,
            pub pdp: ::validated_data::Field<bool>,
            pub policing_interface: ::validated_data::Field<bool>,
            pub qos: ::validated_data::Field<bool>,
            pub qos_dual_rate_policer: ::validated_data::Field<bool>,
            pub route: ::validated_data::Field<bool>,
            pub routed_port: ::validated_data::Field<bool>,
            pub segment_security: ::validated_data::Field<bool>,
            pub subinterface: ::validated_data::Field<bool>,
            pub tapagg: ::validated_data::Field<bool>,
            pub traffic_class: ::validated_data::Field<bool>,
            pub traffic_policy: ::validated_data::Field<bool>,
            pub vlan: ::validated_data::Field<bool>,
            pub vlan_interface: ::validated_data::Field<bool>,
            pub vni_decap: ::validated_data::Field<bool>,
            pub vni_encap: ::validated_data::Field<bool>,
            pub vtep_decap: ::validated_data::Field<bool>,
            pub vtep_encap: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses"))]
        pub struct ErrdisableCauses<'a, Mode> {
            pub acl: ::validated_data::Field<errdisable_causes::Acl<'a, Mode>>,
            pub arp_inspection: ::validated_data::Field<errdisable_causes::ArpInspection<'a, Mode>>,
            pub bpduguard: ::validated_data::Field<errdisable_causes::Bpduguard<'a, Mode>>,
            pub dot1x: ::validated_data::Field<errdisable_causes::Dot1x<'a, Mode>>,
            pub dot1x_coa: ::validated_data::Field<errdisable_causes::Dot1xCoa<'a, Mode>>,
            pub dot1x_phone_classification: ::validated_data::Field<errdisable_causes::Dot1xPhoneClassification<'a, Mode>>,
            pub dot1x_session_replace: ::validated_data::Field<errdisable_causes::Dot1xSessionReplace<'a, Mode>>,
            pub error_correction_encoding: ::validated_data::Field<errdisable_causes::ErrorCorrectionEncoding<'a, Mode>>,
            pub fabric_capacity_low: ::validated_data::Field<errdisable_causes::FabricCapacityLow<'a, Mode>>,
            pub hardware_speed_group: ::validated_data::Field<errdisable_causes::HardwareSpeedGroup<'a, Mode>>,
            pub hitless_reload_down: ::validated_data::Field<errdisable_causes::HitlessReloadDown<'a, Mode>>,
            pub interface_speed: ::validated_data::Field<errdisable_causes::InterfaceSpeed<'a, Mode>>,
            pub internal_error: ::validated_data::Field<errdisable_causes::InternalError<'a, Mode>>,
            pub lacp_rate_limit: ::validated_data::Field<errdisable_causes::LacpRateLimit<'a, Mode>>,
            pub link_change: ::validated_data::Field<errdisable_causes::LinkChange<'a, Mode>>,
            pub link_flap: ::validated_data::Field<errdisable_causes::LinkFlap<'a, Mode>>,
            pub no_internal_vlan: ::validated_data::Field<errdisable_causes::NoInternalVlan<'a, Mode>>,
            pub port_breakout: ::validated_data::Field<errdisable_causes::PortBreakout<'a, Mode>>,
            pub portchannelguard: ::validated_data::Field<errdisable_causes::Portchannelguard<'a, Mode>>,
            pub portsec: ::validated_data::Field<errdisable_causes::Portsec<'a, Mode>>,
            pub speed_misconfigured: ::validated_data::Field<errdisable_causes::SpeedMisconfigured<'a, Mode>>,
            pub storm_control: ::validated_data::Field<errdisable_causes::StormControl<'a, Mode>>,
            pub stuck_queue: ::validated_data::Field<errdisable_causes::StuckQueue<'a, Mode>>,
            pub switchcard_unreachable: ::validated_data::Field<errdisable_causes::SwitchcardUnreachable<'a, Mode>>,
            pub tap_port_init: ::validated_data::Field<errdisable_causes::TapPortInit<'a, Mode>>,
            pub tapagg: ::validated_data::Field<errdisable_causes::Tapagg<'a, Mode>>,
            pub tpid: ::validated_data::Field<errdisable_causes::Tpid<'a, Mode>>,
            pub transceiver_adapter: ::validated_data::Field<errdisable_causes::TransceiverAdapter<'a, Mode>>,
            pub uplink_failure_detection: ::validated_data::Field<errdisable_causes::UplinkFailureDetection<'a, Mode>>,
            pub xcvr_misconfigured: ::validated_data::Field<errdisable_causes::XcvrMisconfigured<'a, Mode>>,
            pub xcvr_overheat: ::validated_data::Field<errdisable_causes::XcvrOverheat<'a, Mode>>,
            pub xcvr_power_unsupported: ::validated_data::Field<errdisable_causes::XcvrPowerUnsupported<'a, Mode>>,
            pub xcvr_unsupported: ::validated_data::Field<errdisable_causes::XcvrUnsupported<'a, Mode>>,
        }

        pub mod errdisable_causes {

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.Acl"))]
            pub struct Acl<'a, Mode> {
                pub detection: ::validated_data::Field<bool>,
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.ArpInspection"))]
            pub struct ArpInspection<'a, Mode> {
                pub detection: ::validated_data::Field<bool>,
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.Bpduguard"))]
            pub struct Bpduguard<'a, Mode> {
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.Dot1x"))]
            pub struct Dot1x<'a, Mode> {
                pub detection: ::validated_data::Field<bool>,
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.Dot1xCoa"))]
            pub struct Dot1xCoa<'a, Mode> {
                pub detection: ::validated_data::Field<bool>,
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.Dot1xPhoneClassification"))]
            pub struct Dot1xPhoneClassification<'a, Mode> {
                pub detection: ::validated_data::Field<bool>,
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.Dot1xSessionReplace"))]
            pub struct Dot1xSessionReplace<'a, Mode> {
                pub detection: ::validated_data::Field<bool>,
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.ErrorCorrectionEncoding"))]
            pub struct ErrorCorrectionEncoding<'a, Mode> {
                pub detection: ::validated_data::Field<bool>,
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.FabricCapacityLow"))]
            pub struct FabricCapacityLow<'a, Mode> {
                pub detection: ::validated_data::Field<bool>,
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.HardwareSpeedGroup"))]
            pub struct HardwareSpeedGroup<'a, Mode> {
                pub detection: ::validated_data::Field<bool>,
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.HitlessReloadDown"))]
            pub struct HitlessReloadDown<'a, Mode> {
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.InterfaceSpeed"))]
            pub struct InterfaceSpeed<'a, Mode> {
                pub detection: ::validated_data::Field<bool>,
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.InternalError"))]
            pub struct InternalError<'a, Mode> {
                pub detection: ::validated_data::Field<bool>,
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.LacpRateLimit"))]
            pub struct LacpRateLimit<'a, Mode> {
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.LinkChange"))]
            pub struct LinkChange<'a, Mode> {
                pub detection: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.LinkFlap"))]
            pub struct LinkFlap<'a, Mode> {
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.NoInternalVlan"))]
            pub struct NoInternalVlan<'a, Mode> {
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.PortBreakout"))]
            pub struct PortBreakout<'a, Mode> {
                pub detection: ::validated_data::Field<bool>,
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.Portchannelguard"))]
            pub struct Portchannelguard<'a, Mode> {
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.Portsec"))]
            pub struct Portsec<'a, Mode> {
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.SpeedMisconfigured"))]
            pub struct SpeedMisconfigured<'a, Mode> {
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.StormControl"))]
            pub struct StormControl<'a, Mode> {
                pub detection: ::validated_data::Field<bool>,
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.StuckQueue"))]
            pub struct StuckQueue<'a, Mode> {
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.SwitchcardUnreachable"))]
            pub struct SwitchcardUnreachable<'a, Mode> {
                pub detection: ::validated_data::Field<bool>,
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.TapPortInit"))]
            pub struct TapPortInit<'a, Mode> {
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.Tapagg"))]
            pub struct Tapagg<'a, Mode> {
                pub detection: ::validated_data::Field<bool>,
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.Tpid"))]
            pub struct Tpid<'a, Mode> {
                pub detection: ::validated_data::Field<bool>,
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.TransceiverAdapter"))]
            pub struct TransceiverAdapter<'a, Mode> {
                pub detection: ::validated_data::Field<bool>,
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.UplinkFailureDetection"))]
            pub struct UplinkFailureDetection<'a, Mode> {
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.XcvrMisconfigured"))]
            pub struct XcvrMisconfigured<'a, Mode> {
                pub detection: ::validated_data::Field<bool>,
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.XcvrOverheat"))]
            pub struct XcvrOverheat<'a, Mode> {
                pub detection: ::validated_data::Field<bool>,
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.XcvrPowerUnsupported"))]
            pub struct XcvrPowerUnsupported<'a, Mode> {
                pub detection: ::validated_data::Field<bool>,
                pub recovery: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.FeatureSupport.ErrdisableCauses.XcvrUnsupported"))]
            pub struct XcvrUnsupported<'a, Mode> {
                pub recovery: ::validated_data::Field<bool>,
            }
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.SecurityEntropySources"))]
    pub struct SecurityEntropySources<'a, Mode> {
        pub hardware: ::validated_data::Field<bool>,
        pub haveged: ::validated_data::Field<bool>,
        pub cpu_jitter: ::validated_data::Field<bool>,
        pub hardware_exclusive: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.PlatformSettingsItem.DigitalTwin"))]
    pub struct DigitalTwin<'a, Mode> {
        pub platform: ::validated_data::Field<&'a str>,
        pub act_node_type: ::validated_data::Field<&'a str>,
    }
}
