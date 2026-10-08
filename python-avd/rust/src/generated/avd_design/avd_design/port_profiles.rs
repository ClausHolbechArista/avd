// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar profile("profile", 0) -> &'a str;
        scalar parent_profile("parent_profile", 1) -> &'a str;
        model port_channel("port_channel", 2) -> item::PortChannel<'a>;
        scalar speed("speed", 3) -> &'a str;
        scalar description("description", 4) -> &'a str;
        scalar enabled("enabled", 5) -> bool;
        scalar mode("mode", 6) -> &'a str;
        scalar mtu("mtu", 7) -> i64;
        scalar l2_mtu("l2_mtu", 8) -> i64;
        scalar l2_mru("l2_mru", 9) -> i64;
        scalar native_vlan("native_vlan", 10) -> i64;
        scalar native_vlan_tag("native_vlan_tag", 11) -> bool;
        scalar phone_vlan("phone_vlan", 12) -> i64;
        scalar phone_trunk_mode("phone_trunk_mode", 13) -> &'a str;
        model trunk_groups("trunk_groups", 14) -> item::TrunkGroups<'a>;
        scalar vlans("vlans", 15) -> &'a str;
        scalar mac_acl_in("mac_acl_in", 16) -> &'a str;
        scalar mac_acl_out("mac_acl_out", 17) -> &'a str;
        scalar spanning_tree_portfast("spanning_tree_portfast", 18) -> &'a str;
        scalar spanning_tree_bpdufilter("spanning_tree_bpdufilter", 19) -> &'a str;
        scalar spanning_tree_bpduguard("spanning_tree_bpduguard", 20) -> &'a str;
        scalar spanning_tree_link_type("spanning_tree_link_type", 21) -> &'a str;
        model flowcontrol("flowcontrol", 22) -> super::super::eos_cli_config_gen::ethernet_interfaces::item::Flowcontrol<'a>;
        scalar qos_profile("qos_profile", 23) -> &'a str;
        model ptp("ptp", 24) -> item::Ptp<'a>;
        scalar sflow("sflow", 25) -> bool;
        model flow_tracking("flow_tracking", 26) -> item::FlowTracking<'a>;
        model link_tracking("link_tracking", 27) -> item::LinkTracking<'a>;
        model dot1x("dot1x", 28) -> item::Dot1x<'a>;
        model address_locking("address_locking", 29) -> item::AddressLocking<'a>;
        model poe("poe", 30) -> super::super::eos_cli_config_gen::ethernet_interfaces::item::Poe<'a>;
        model storm_control("storm_control", 31) -> item::StormControl<'a>;
        model monitor_sessions("monitor_sessions", 32) -> item::MonitorSessions<'a>;
        model ethernet_segment("ethernet_segment", 33) -> item::EthernetSegment<'a>;
        scalar validate_state("validate_state", 34) -> bool;
        scalar validate_lldp("validate_lldp", 35) -> bool;
        model campus_link_type("campus_link_type", 36) -> item::CampusLinkType<'a>;
        scalar raw_eos_cli("raw_eos_cli", 37) -> &'a str;
        model structured_config("structured_config", 38) -> super::super::eos_cli_config_gen::ethernet_interfaces::Item<'a>;
    }
}

pub mod item {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PortChannel {
            model subinterfaces("subinterfaces", 0) -> port_channel::Subinterfaces<'a>;
            scalar mode("mode", 1) -> &'a str;
            scalar channel_id("channel_id", 2) -> i64;
            scalar description("description", 3) -> &'a str;
            scalar endpoint_port_channel("endpoint_port_channel", 4) -> &'a str;
            scalar enabled("enabled", 5) -> bool;
            scalar ptp_mpass("ptp_mpass", 6) -> bool;
            model lacp_fallback("lacp_fallback", 7) -> port_channel::LacpFallback<'a>;
            model lacp_timer("lacp_timer", 8) -> port_channel::LacpTimer<'a>;
            scalar raw_eos_cli("raw_eos_cli", 9) -> &'a str;
            model structured_config("structured_config", 10) -> super::super::super::eos_cli_config_gen::port_channel_interfaces::Item<'a>;
        }
    }

    pub mod port_channel {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Subinterfaces {
                model item (0) -> subinterfaces::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod subinterfaces {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar number("number", 0) -> i64;
                    scalar description("description", 1) -> &'a str;
                    scalar short_esi("short_esi", 2) -> &'a str;
                    scalar vlan_id("vlan_id", 3) -> i64;
                    model encapsulation_vlan("encapsulation_vlan", 4) -> item::EncapsulationVlan<'a>;
                    scalar raw_eos_cli("raw_eos_cli", 5) -> &'a str;
                    model structured_config("structured_config", 6) -> super::super::super::super::super::eos_cli_config_gen::port_channel_interfaces::Item<'a>;
                }
            }

            pub mod item {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct EncapsulationVlan {
                        scalar client_dot1q("client_dot1q", 0) -> i64;
                    }
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct LacpFallback {
                scalar mode("mode", 0) -> &'a str;
                model individual("individual", 1) -> lacp_fallback::Individual<'a>;
                scalar timeout("timeout", 2) -> i64;
            }
        }

        pub mod lacp_fallback {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Individual {
                    scalar profile("profile", 0) -> &'a str;
                    scalar vlans("vlans", 1) -> &'a str;
                    scalar native_vlan("native_vlan", 2) -> i64;
                    scalar mode("mode", 3) -> &'a str;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct LacpTimer {
                scalar mode("mode", 0) -> &'a str;
                scalar multiplier("multiplier", 1) -> i64;
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct TrunkGroups {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ptp {
            scalar enabled("enabled", 0) -> bool;
            scalar endpoint_role("endpoint_role", 1) -> &'a str;
            scalar profile("profile", 2) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct FlowTracking {
            scalar enabled("enabled", 0) -> bool;
            scalar name("name", 1) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct LinkTracking {
            scalar enabled("enabled", 0) -> bool;
            scalar name("name", 1) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Dot1x {
            model authentication_failure("authentication_failure", 0) -> dot1x::AuthenticationFailure<'a>;
            scalar port_control("port_control", 1) -> &'a str;
            scalar port_control_force_authorized_phone("port_control_force_authorized_phone", 2) -> bool;
            scalar reauthentication("reauthentication", 3) -> bool;
            model pae("pae", 4) -> dot1x::Pae<'a>;
            model host_mode("host_mode", 5) -> dot1x::HostMode<'a>;
            model mac_based_authentication("mac_based_authentication", 6) -> dot1x::MacBasedAuthentication<'a>;
            scalar mac_based_access_list("mac_based_access_list", 7) -> bool;
            model timeout("timeout", 8) -> dot1x::Timeout<'a>;
            scalar reauthorization_request_limit("reauthorization_request_limit", 9) -> i64;
            model unauthorized("unauthorized", 10) -> dot1x::Unauthorized<'a>;
            model eapol("eapol", 11) -> dot1x::Eapol<'a>;
            model aaa("aaa", 12) -> dot1x::Aaa<'a>;
        }
    }

    pub mod dot1x {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AuthenticationFailure {
                scalar allow_access_list("allow_access_list", 0) -> &'a str;
                scalar action("action", 1) -> &'a str;
                scalar allow_vlan("allow_vlan", 2) -> i64;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Pae {
                scalar mode("mode", 0) -> &'a str;
                scalar supplicant_profile("supplicant_profile", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct HostMode {
                scalar mode("mode", 0) -> &'a str;
                scalar multi_host_authenticated("multi_host_authenticated", 1) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct MacBasedAuthentication {
                scalar enabled("enabled", 0) -> bool;
                scalar always("always", 1) -> bool;
                scalar host_mode_common("host_mode_common", 2) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Timeout {
                scalar idle_host("idle_host", 0) -> i64;
                scalar quiet_period("quiet_period", 1) -> i64;
                scalar reauth_period("reauth_period", 2) -> &'a str;
                scalar reauth_timeout_ignore("reauth_timeout_ignore", 3) -> bool;
                scalar tx_period("tx_period", 4) -> i64;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Unauthorized {
                scalar access_vlan_membership_egress("access_vlan_membership_egress", 0) -> bool;
                scalar native_vlan_membership_egress("native_vlan_membership_egress", 1) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Eapol {
                scalar disabled("disabled", 0) -> bool;
                model authentication_failure_fallback_mba("authentication_failure_fallback_mba", 1) -> eapol::AuthenticationFailureFallbackMba<'a>;
            }
        }

        pub mod eapol {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AuthenticationFailureFallbackMba {
                    scalar enabled("enabled", 0) -> bool;
                    scalar timeout("timeout", 1) -> i64;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Aaa {
                model unresponsive("unresponsive", 0) -> aaa::Unresponsive<'a>;
            }
        }

        pub mod aaa {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Unresponsive {
                    scalar eap_response("eap_response", 0) -> &'a str;
                    model action("action", 1) -> unresponsive::Action<'a>;
                    model phone_action("phone_action", 2) -> super::super::super::super::super::eos_cli_config_gen::dot1x::aaa::unresponsive::PhoneAction<'a>;
                }
            }

            pub mod unresponsive {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Action {
                        scalar traffic_allow_access_list("traffic_allow_access_list", 0) -> &'a str;
                        scalar apply_alternate("apply_alternate", 1) -> bool;
                        scalar traffic_allow_vlan("traffic_allow_vlan", 2) -> i64;
                        scalar apply_cached_results("apply_cached_results", 3) -> bool;
                        model cached_results_timeout("cached_results_timeout", 4) -> action::CachedResultsTimeout<'a>;
                        scalar traffic_allow("traffic_allow", 5) -> bool;
                    }
                }

                pub mod action {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct CachedResultsTimeout {
                            scalar time_duration("time_duration", 0) -> i64;
                            scalar time_duration_unit("time_duration_unit", 1) -> &'a str;
                        }
                    }
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct AddressLocking {
            scalar ipv4("ipv4", 0) -> bool;
            scalar ipv6("ipv6", 1) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct StormControl {
            model all("all", 0) -> storm_control::All<'a>;
            model broadcast("broadcast", 1) -> storm_control::Broadcast<'a>;
            model multicast("multicast", 2) -> storm_control::Multicast<'a>;
            model unknown_unicast("unknown_unicast", 3) -> storm_control::UnknownUnicast<'a>;
        }
    }

    pub mod storm_control {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct All {
                scalar level("level", 0) -> &'a str;
                scalar unit("unit", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Broadcast {
                scalar level("level", 0) -> &'a str;
                scalar unit("unit", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Multicast {
                scalar level("level", 0) -> &'a str;
                scalar unit("unit", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct UnknownUnicast {
                scalar level("level", 0) -> &'a str;
                scalar unit("unit", 1) -> &'a str;
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct MonitorSessions {
            model item (0) -> monitor_sessions::Item<'a>;
        }
    }

    pub mod monitor_sessions {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar role("role", 1) -> &'a str;
                model source_settings("source_settings", 2) -> item::SourceSettings<'a>;
                model session_settings("session_settings", 3) -> item::SessionSettings<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct SourceSettings {
                    scalar direction("direction", 0) -> &'a str;
                    model access_group("access_group", 1) -> source_settings::AccessGroup<'a>;
                }
            }

            pub mod source_settings {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct AccessGroup {
                        scalar field_type("type", 0) -> &'a str;
                        scalar name("name", 1) -> &'a str;
                        scalar priority("priority", 2) -> i64;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct SessionSettings {
                    scalar encapsulation_gre_metadata_tx("encapsulation_gre_metadata_tx", 0) -> bool;
                    scalar header_remove_size("header_remove_size", 1) -> i64;
                    model access_group("access_group", 2) -> session_settings::AccessGroup<'a>;
                    scalar rate_limit_per_ingress_chip("rate_limit_per_ingress_chip", 3) -> &'a str;
                    scalar rate_limit_per_egress_chip("rate_limit_per_egress_chip", 4) -> &'a str;
                    scalar sample("sample", 5) -> i64;
                    model truncate("truncate", 6) -> session_settings::Truncate<'a>;
                }
            }

            pub mod session_settings {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct AccessGroup {
                        scalar field_type("type", 0) -> &'a str;
                        scalar name("name", 1) -> &'a str;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Truncate {
                        scalar enabled("enabled", 0) -> bool;
                        scalar size("size", 1) -> i64;
                    }
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct EthernetSegment {
            scalar short_esi("short_esi", 0) -> &'a str;
            scalar redundancy("redundancy", 1) -> &'a str;
            scalar designated_forwarder_algorithm("designated_forwarder_algorithm", 2) -> &'a str;
            model designated_forwarder_preferences("designated_forwarder_preferences", 3) -> ethernet_segment::DesignatedForwarderPreferences<'a>;
            scalar dont_preempt("dont_preempt", 4) -> bool;
        }
    }

    pub mod ethernet_segment {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct DesignatedForwarderPreferences {
                scalar item (0) -> i64;
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct CampusLinkType {
            scalar item (0) -> &'a str;
        }
    }
}
