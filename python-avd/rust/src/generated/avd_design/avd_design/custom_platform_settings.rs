// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        model platforms("platforms", 0) -> item::Platforms<'a>;
        scalar trident_forwarding_table_partition("trident_forwarding_table_partition", 1) -> &'a str;
        model reload_delay("reload_delay", 2) -> item::ReloadDelay<'a>;
        scalar tcam_profile("tcam_profile", 3) -> &'a str;
        model additional_tcam_profiles("additional_tcam_profiles", 4) -> item::AdditionalTcamProfiles<'a>;
        scalar lag_hardware_only("lag_hardware_only", 5) -> bool;
        scalar default_interface_mtu("default_interface_mtu", 6) -> i64;
        scalar p2p_uplinks_mtu("p2p_uplinks_mtu", 7) -> i64;
        model feature_support("feature_support", 8) -> item::FeatureSupport<'a>;
        scalar management_interface("management_interface", 9) -> &'a str;
        model security_entropy_sources("security_entropy_sources", 10) -> item::SecurityEntropySources<'a>;
        model digital_twin("digital_twin", 11) -> item::DigitalTwin<'a>;
        model structured_config("structured_config", 12) -> super::super::eos_cli_config_gen::EosCliConfigGen<'a>;
        scalar raw_eos_cli("raw_eos_cli", 13) -> &'a str;
    }
}

pub mod item {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Platforms {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct ReloadDelay {
            scalar mlag("mlag", 0) -> i64;
            scalar non_mlag("non_mlag", 1) -> i64;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct AdditionalTcamProfiles {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct FeatureSupport {
            model address_locking("address_locking", 0) -> feature_support::AddressLocking<'a>;
            scalar queue_monitor("queue_monitor", 1) -> bool;
            scalar queue_monitor_length_notify("queue_monitor_length_notify", 2) -> bool;
            scalar interface_storm_control("interface_storm_control", 3) -> bool;
            scalar poe("poe", 4) -> bool;
            scalar subinterface_mtu("subinterface_mtu", 5) -> bool;
            scalar subinterface_monitor_session("subinterface_monitor_session", 6) -> bool;
            scalar per_interface_mtu("per_interface_mtu", 7) -> bool;
            scalar per_interface_l2_mtu("per_interface_l2_mtu", 8) -> bool;
            scalar per_interface_l2_mru("per_interface_l2_mru", 9) -> bool;
            scalar bgp_update_wait_install("bgp_update_wait_install", 10) -> bool;
            scalar bgp_update_wait_for_convergence("bgp_update_wait_for_convergence", 11) -> bool;
            model platform_sfe_interface_profile("platform_sfe_interface_profile", 12) -> feature_support::PlatformSfeInterfaceProfile<'a>;
            scalar evpn_gateway_all_active_multihoming("evpn_gateway_all_active_multihoming", 13) -> bool;
            scalar evpn_gateway_rd_rt_rewrite("evpn_gateway_rd_rt_rewrite", 14) -> bool;
            scalar hardware_counters("hardware_counters", 15) -> bool;
            model hardware_counter_features("hardware_counter_features", 16) -> feature_support::HardwareCounterFeatures<'a>;
            scalar hardware_speed_group("hardware_speed_group", 17) -> bool;
            scalar private_vlan("private_vlan", 18) -> bool;
            scalar sflow("sflow", 19) -> bool;
            scalar sflow_subinterfaces("sflow_subinterfaces", 20) -> bool;
            scalar wan("wan", 21) -> bool;
            scalar ptp("ptp", 22) -> bool;
            scalar hardware_validation("hardware_validation", 23) -> bool;
            model errdisable_causes("errdisable_causes", 24) -> feature_support::ErrdisableCauses<'a>;
        }
    }

    pub mod feature_support {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AddressLocking {
                scalar supported("supported", 0) -> bool;
                scalar ipv4_enforcement_disabled("ipv4_enforcement_disabled", 1) -> bool;
                scalar ipv6_enforcement_disabled("ipv6_enforcement_disabled", 2) -> bool;
                scalar ipv6_ethernet_interface("ipv6_ethernet_interface", 3) -> bool;
                scalar ipv6_vlan("ipv6_vlan", 4) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct PlatformSfeInterfaceProfile {
                scalar supported("supported", 0) -> bool;
                scalar max_rx_queues("max_rx_queues", 1) -> i64;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct HardwareCounterFeatures {
                scalar acl("acl", 0) -> bool;
                scalar decap_group("decap_group", 1) -> bool;
                scalar directflow("directflow", 2) -> bool;
                scalar ecn("ecn", 3) -> bool;
                scalar flow_spec("flow_spec", 4) -> bool;
                scalar gre_tunnel_interface("gre_tunnel_interface", 5) -> bool;
                scalar ip("ip", 6) -> bool;
                scalar mpls_interface("mpls_interface", 7) -> bool;
                scalar mpls_lfib("mpls_lfib", 8) -> bool;
                scalar mpls_tunnel("mpls_tunnel", 9) -> bool;
                scalar multicast("multicast", 10) -> bool;
                scalar nexthop("nexthop", 11) -> bool;
                scalar pbr("pbr", 12) -> bool;
                scalar pdp("pdp", 13) -> bool;
                scalar policing_interface("policing_interface", 14) -> bool;
                scalar qos("qos", 15) -> bool;
                scalar qos_dual_rate_policer("qos_dual_rate_policer", 16) -> bool;
                scalar route("route", 17) -> bool;
                scalar routed_port("routed_port", 18) -> bool;
                scalar segment_security("segment_security", 19) -> bool;
                scalar subinterface("subinterface", 20) -> bool;
                scalar tapagg("tapagg", 21) -> bool;
                scalar traffic_class("traffic_class", 22) -> bool;
                scalar traffic_policy("traffic_policy", 23) -> bool;
                scalar vlan("vlan", 24) -> bool;
                scalar vlan_interface("vlan_interface", 25) -> bool;
                scalar vni_decap("vni_decap", 26) -> bool;
                scalar vni_encap("vni_encap", 27) -> bool;
                scalar vtep_decap("vtep_decap", 28) -> bool;
                scalar vtep_encap("vtep_encap", 29) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct ErrdisableCauses {
                model acl("acl", 0) -> errdisable_causes::Acl<'a>;
                model arp_inspection("arp_inspection", 1) -> errdisable_causes::ArpInspection<'a>;
                model bpduguard("bpduguard", 2) -> errdisable_causes::Bpduguard<'a>;
                model dot1x("dot1x", 3) -> errdisable_causes::Dot1x<'a>;
                model dot1x_coa("dot1x_coa", 4) -> errdisable_causes::Dot1xCoa<'a>;
                model dot1x_phone_classification("dot1x_phone_classification", 5) -> errdisable_causes::Dot1xPhoneClassification<'a>;
                model dot1x_session_replace("dot1x_session_replace", 6) -> errdisable_causes::Dot1xSessionReplace<'a>;
                model error_correction_encoding("error_correction_encoding", 7) -> errdisable_causes::ErrorCorrectionEncoding<'a>;
                model fabric_capacity_low("fabric_capacity_low", 8) -> errdisable_causes::FabricCapacityLow<'a>;
                model hardware_speed_group("hardware_speed_group", 9) -> errdisable_causes::HardwareSpeedGroup<'a>;
                model hitless_reload_down("hitless_reload_down", 10) -> errdisable_causes::HitlessReloadDown<'a>;
                model interface_speed("interface_speed", 11) -> errdisable_causes::InterfaceSpeed<'a>;
                model internal_error("internal_error", 12) -> errdisable_causes::InternalError<'a>;
                model lacp_rate_limit("lacp_rate_limit", 13) -> errdisable_causes::LacpRateLimit<'a>;
                model link_change("link_change", 14) -> errdisable_causes::LinkChange<'a>;
                model link_flap("link_flap", 15) -> errdisable_causes::LinkFlap<'a>;
                model no_internal_vlan("no_internal_vlan", 16) -> errdisable_causes::NoInternalVlan<'a>;
                model port_breakout("port_breakout", 17) -> errdisable_causes::PortBreakout<'a>;
                model portchannelguard("portchannelguard", 18) -> errdisable_causes::Portchannelguard<'a>;
                model portsec("portsec", 19) -> errdisable_causes::Portsec<'a>;
                model speed_misconfigured("speed_misconfigured", 20) -> errdisable_causes::SpeedMisconfigured<'a>;
                model storm_control("storm_control", 21) -> errdisable_causes::StormControl<'a>;
                model stuck_queue("stuck_queue", 22) -> errdisable_causes::StuckQueue<'a>;
                model switchcard_unreachable("switchcard_unreachable", 23) -> errdisable_causes::SwitchcardUnreachable<'a>;
                model tap_port_init("tap_port_init", 24) -> errdisable_causes::TapPortInit<'a>;
                model tapagg("tapagg", 25) -> errdisable_causes::Tapagg<'a>;
                model tpid("tpid", 26) -> errdisable_causes::Tpid<'a>;
                model transceiver_adapter("transceiver_adapter", 27) -> errdisable_causes::TransceiverAdapter<'a>;
                model uplink_failure_detection("uplink_failure_detection", 28) -> errdisable_causes::UplinkFailureDetection<'a>;
                model xcvr_misconfigured("xcvr_misconfigured", 29) -> errdisable_causes::XcvrMisconfigured<'a>;
                model xcvr_overheat("xcvr_overheat", 30) -> errdisable_causes::XcvrOverheat<'a>;
                model xcvr_power_unsupported("xcvr_power_unsupported", 31) -> errdisable_causes::XcvrPowerUnsupported<'a>;
                model xcvr_unsupported("xcvr_unsupported", 32) -> errdisable_causes::XcvrUnsupported<'a>;
            }
        }

        pub mod errdisable_causes {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Acl {
                    scalar detection("detection", 0) -> bool;
                    scalar recovery("recovery", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct ArpInspection {
                    scalar detection("detection", 0) -> bool;
                    scalar recovery("recovery", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Bpduguard {
                    scalar recovery("recovery", 0) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Dot1x {
                    scalar detection("detection", 0) -> bool;
                    scalar recovery("recovery", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Dot1xCoa {
                    scalar detection("detection", 0) -> bool;
                    scalar recovery("recovery", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Dot1xPhoneClassification {
                    scalar detection("detection", 0) -> bool;
                    scalar recovery("recovery", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Dot1xSessionReplace {
                    scalar detection("detection", 0) -> bool;
                    scalar recovery("recovery", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct ErrorCorrectionEncoding {
                    scalar detection("detection", 0) -> bool;
                    scalar recovery("recovery", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct FabricCapacityLow {
                    scalar detection("detection", 0) -> bool;
                    scalar recovery("recovery", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct HardwareSpeedGroup {
                    scalar detection("detection", 0) -> bool;
                    scalar recovery("recovery", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct HitlessReloadDown {
                    scalar recovery("recovery", 0) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct InterfaceSpeed {
                    scalar detection("detection", 0) -> bool;
                    scalar recovery("recovery", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct InternalError {
                    scalar detection("detection", 0) -> bool;
                    scalar recovery("recovery", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct LacpRateLimit {
                    scalar recovery("recovery", 0) -> bool;
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
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct NoInternalVlan {
                    scalar recovery("recovery", 0) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct PortBreakout {
                    scalar detection("detection", 0) -> bool;
                    scalar recovery("recovery", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Portchannelguard {
                    scalar recovery("recovery", 0) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Portsec {
                    scalar recovery("recovery", 0) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct SpeedMisconfigured {
                    scalar recovery("recovery", 0) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct StormControl {
                    scalar detection("detection", 0) -> bool;
                    scalar recovery("recovery", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct StuckQueue {
                    scalar recovery("recovery", 0) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct SwitchcardUnreachable {
                    scalar detection("detection", 0) -> bool;
                    scalar recovery("recovery", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct TapPortInit {
                    scalar recovery("recovery", 0) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Tapagg {
                    scalar detection("detection", 0) -> bool;
                    scalar recovery("recovery", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Tpid {
                    scalar detection("detection", 0) -> bool;
                    scalar recovery("recovery", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct TransceiverAdapter {
                    scalar detection("detection", 0) -> bool;
                    scalar recovery("recovery", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct UplinkFailureDetection {
                    scalar recovery("recovery", 0) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct XcvrMisconfigured {
                    scalar detection("detection", 0) -> bool;
                    scalar recovery("recovery", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct XcvrOverheat {
                    scalar detection("detection", 0) -> bool;
                    scalar recovery("recovery", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct XcvrPowerUnsupported {
                    scalar detection("detection", 0) -> bool;
                    scalar recovery("recovery", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct XcvrUnsupported {
                    scalar recovery("recovery", 0) -> bool;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct SecurityEntropySources {
            scalar hardware("hardware", 0) -> bool;
            scalar haveged("haveged", 1) -> bool;
            scalar cpu_jitter("cpu_jitter", 2) -> bool;
            scalar hardware_exclusive("hardware_exclusive", 3) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct DigitalTwin {
            scalar platform("platform", 0) -> &'a str;
            scalar act_node_type("act_node_type", 1) -> &'a str;
        }
    }
}
