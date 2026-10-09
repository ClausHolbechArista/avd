// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub profile: ::validated_data::Field<&'a str>,
    pub parent_profile: ::validated_data::Field<&'a str>,
    pub port_channel: ::validated_data::Field<item::PortChannel<'a, Mode>>,
    pub speed: ::validated_data::Field<&'a str>,
    pub description: ::validated_data::Field<&'a str>,
    pub enabled: ::validated_data::Field<bool>,
    pub mode: ::validated_data::Field<&'a str>,
    pub mtu: ::validated_data::Field<i64>,
    pub l2_mtu: ::validated_data::Field<i64>,
    pub l2_mru: ::validated_data::Field<i64>,
    pub native_vlan: ::validated_data::Field<i64>,
    pub native_vlan_tag: ::validated_data::Field<bool>,
    pub phone_vlan: ::validated_data::Field<i64>,
    pub phone_trunk_mode: ::validated_data::Field<&'a str>,
    pub trunk_groups: ::validated_data::Field<item::TrunkGroups<'a, Mode>>,
    pub vlans: ::validated_data::Field<&'a str>,
    pub mac_acl_in: ::validated_data::Field<&'a str>,
    pub mac_acl_out: ::validated_data::Field<&'a str>,
    pub spanning_tree_portfast: ::validated_data::Field<&'a str>,
    pub spanning_tree_bpdufilter: ::validated_data::Field<&'a str>,
    pub spanning_tree_bpduguard: ::validated_data::Field<&'a str>,
    pub spanning_tree_link_type: ::validated_data::Field<&'a str>,
    pub flowcontrol: ::validated_data::Field<super::super::eos_cli_config_gen::ethernet_interfaces::item::Flowcontrol<'a, Mode>>,
    pub qos_profile: ::validated_data::Field<&'a str>,
    pub ptp: ::validated_data::Field<item::Ptp<'a, Mode>>,
    pub sflow: ::validated_data::Field<bool>,
    pub flow_tracking: ::validated_data::Field<item::FlowTracking<'a, Mode>>,
    pub link_tracking: ::validated_data::Field<item::LinkTracking<'a, Mode>>,
    pub dot1x: ::validated_data::Field<item::Dot1x<'a, Mode>>,
    pub address_locking: ::validated_data::Field<item::AddressLocking<'a, Mode>>,
    pub poe: ::validated_data::Field<super::super::eos_cli_config_gen::ethernet_interfaces::item::Poe<'a, Mode>>,
    pub storm_control: ::validated_data::Field<item::StormControl<'a, Mode>>,
    pub monitor_sessions: ::validated_data::Field<item::MonitorSessions<'a, Mode>>,
    pub ethernet_segment: ::validated_data::Field<item::EthernetSegment<'a, Mode>>,
    pub validate_state: ::validated_data::Field<bool>,
    pub validate_lldp: ::validated_data::Field<bool>,
    pub campus_link_type: ::validated_data::Field<item::CampusLinkType<'a, Mode>>,
    pub raw_eos_cli: ::validated_data::Field<&'a str>,
    #[data_view(relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_cli_config_gen::ethernet_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
}

pub mod item {

    #[::validated_data::data_view]
    pub struct PortChannel<'a, Mode> {
        pub subinterfaces: ::validated_data::Field<port_channel::Subinterfaces<'a, Mode>>,
        pub mode: ::validated_data::Field<&'a str>,
        pub channel_id: ::validated_data::Field<i64>,
        pub description: ::validated_data::Field<&'a str>,
        pub endpoint_port_channel: ::validated_data::Field<&'a str>,
        pub enabled: ::validated_data::Field<bool>,
        pub ptp_mpass: ::validated_data::Field<bool>,
        pub lacp_fallback: ::validated_data::Field<port_channel::LacpFallback<'a, Mode>>,
        pub lacp_timer: ::validated_data::Field<port_channel::LacpTimer<'a, Mode>>,
        pub raw_eos_cli: ::validated_data::Field<&'a str>,
        #[data_view(relaxed)]
        pub structured_config: ::validated_data::Field<super::super::super::eos_cli_config_gen::port_channel_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
    }

    pub mod port_channel {

        #[::validated_data::data_view(indexed_list, primary_key(number))]
        pub struct Subinterfaces<'a, Mode> (::validated_data::Field<subinterfaces::Item<'a, Mode>>);

        pub mod subinterfaces {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub number: ::validated_data::Field<i64>,
                pub description: ::validated_data::Field<&'a str>,
                pub short_esi: ::validated_data::Field<&'a str>,
                pub vlan_id: ::validated_data::Field<i64>,
                pub encapsulation_vlan: ::validated_data::Field<item::EncapsulationVlan<'a, Mode>>,
                pub raw_eos_cli: ::validated_data::Field<&'a str>,
                #[data_view(relaxed)]
                pub structured_config: ::validated_data::Field<super::super::super::super::super::eos_cli_config_gen::port_channel_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
            }

            pub mod item {

                #[::validated_data::data_view]
                pub struct EncapsulationVlan<'a, Mode> {
                    pub client_dot1q: ::validated_data::Field<i64>,
                }
            }
        }

        #[::validated_data::data_view]
        pub struct LacpFallback<'a, Mode> {
            pub mode: ::validated_data::Field<&'a str>,
            pub individual: ::validated_data::Field<lacp_fallback::Individual<'a, Mode>>,
            pub timeout: ::validated_data::Field<i64>,
        }

        pub mod lacp_fallback {

            #[::validated_data::data_view]
            pub struct Individual<'a, Mode> {
                pub profile: ::validated_data::Field<&'a str>,
                pub vlans: ::validated_data::Field<&'a str>,
                pub native_vlan: ::validated_data::Field<i64>,
                pub mode: ::validated_data::Field<&'a str>,
            }
        }

        #[::validated_data::data_view]
        pub struct LacpTimer<'a, Mode> {
            pub mode: ::validated_data::Field<&'a str>,
            pub multiplier: ::validated_data::Field<i64>,
        }
    }

    #[::validated_data::data_view(list)]
    pub struct TrunkGroups<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view]
    pub struct Ptp<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub endpoint_role: ::validated_data::Field<&'a str>,
        pub profile: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct FlowTracking<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub name: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct LinkTracking<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub name: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct Dot1x<'a, Mode> {
        pub authentication_failure: ::validated_data::Field<dot1x::AuthenticationFailure<'a, Mode>>,
        pub port_control: ::validated_data::Field<&'a str>,
        pub port_control_force_authorized_phone: ::validated_data::Field<bool>,
        pub reauthentication: ::validated_data::Field<bool>,
        pub pae: ::validated_data::Field<dot1x::Pae<'a, Mode>>,
        pub host_mode: ::validated_data::Field<dot1x::HostMode<'a, Mode>>,
        pub mac_based_authentication: ::validated_data::Field<dot1x::MacBasedAuthentication<'a, Mode>>,
        pub mac_based_access_list: ::validated_data::Field<bool>,
        pub timeout: ::validated_data::Field<dot1x::Timeout<'a, Mode>>,
        pub reauthorization_request_limit: ::validated_data::Field<i64>,
        pub unauthorized: ::validated_data::Field<dot1x::Unauthorized<'a, Mode>>,
        pub eapol: ::validated_data::Field<dot1x::Eapol<'a, Mode>>,
        pub aaa: ::validated_data::Field<dot1x::Aaa<'a, Mode>>,
    }

    pub mod dot1x {

        #[::validated_data::data_view]
        pub struct AuthenticationFailure<'a, Mode> {
            pub allow_access_list: ::validated_data::Field<&'a str>,
            pub action: ::validated_data::Field<&'a str>,
            pub allow_vlan: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view]
        pub struct Pae<'a, Mode> {
            pub mode: ::validated_data::Field<&'a str>,
            pub supplicant_profile: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct HostMode<'a, Mode> {
            pub mode: ::validated_data::Field<&'a str>,
            pub multi_host_authenticated: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct MacBasedAuthentication<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub always: ::validated_data::Field<bool>,
            pub host_mode_common: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct Timeout<'a, Mode> {
            pub idle_host: ::validated_data::Field<i64>,
            pub quiet_period: ::validated_data::Field<i64>,
            pub reauth_period: ::validated_data::Field<&'a str>,
            pub reauth_timeout_ignore: ::validated_data::Field<bool>,
            pub tx_period: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view]
        pub struct Unauthorized<'a, Mode> {
            pub access_vlan_membership_egress: ::validated_data::Field<bool>,
            pub native_vlan_membership_egress: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct Eapol<'a, Mode> {
            pub disabled: ::validated_data::Field<bool>,
            pub authentication_failure_fallback_mba: ::validated_data::Field<eapol::AuthenticationFailureFallbackMba<'a, Mode>>,
        }

        pub mod eapol {

            #[::validated_data::data_view]
            pub struct AuthenticationFailureFallbackMba<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub timeout: ::validated_data::Field<i64>,
            }
        }

        #[::validated_data::data_view]
        pub struct Aaa<'a, Mode> {
            pub unresponsive: ::validated_data::Field<aaa::Unresponsive<'a, Mode>>,
        }

        pub mod aaa {

            #[::validated_data::data_view]
            pub struct Unresponsive<'a, Mode> {
                pub eap_response: ::validated_data::Field<&'a str>,
                pub action: ::validated_data::Field<unresponsive::Action<'a, Mode>>,
                pub phone_action: ::validated_data::Field<super::super::super::super::super::eos_cli_config_gen::dot1x::aaa::unresponsive::PhoneAction<'a, Mode>>,
            }

            pub mod unresponsive {

                #[::validated_data::data_view]
                pub struct Action<'a, Mode> {
                    pub traffic_allow_access_list: ::validated_data::Field<&'a str>,
                    pub apply_alternate: ::validated_data::Field<bool>,
                    pub traffic_allow_vlan: ::validated_data::Field<i64>,
                    pub apply_cached_results: ::validated_data::Field<bool>,
                    pub cached_results_timeout: ::validated_data::Field<action::CachedResultsTimeout<'a, Mode>>,
                    pub traffic_allow: ::validated_data::Field<bool>,
                }

                pub mod action {

                    #[::validated_data::data_view]
                    pub struct CachedResultsTimeout<'a, Mode> {
                        pub time_duration: ::validated_data::RequiredValue<i64, Mode>,
                        pub time_duration_unit: ::validated_data::RequiredValue<&'a str, Mode>,
                    }
                }
            }
        }
    }

    #[::validated_data::data_view]
    pub struct AddressLocking<'a, Mode> {
        pub ipv4: ::validated_data::Field<bool>,
        pub ipv6: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view]
    pub struct StormControl<'a, Mode> {
        pub all: ::validated_data::Field<storm_control::All<'a, Mode>>,
        pub broadcast: ::validated_data::Field<storm_control::Broadcast<'a, Mode>>,
        pub multicast: ::validated_data::Field<storm_control::Multicast<'a, Mode>>,
        pub unknown_unicast: ::validated_data::Field<storm_control::UnknownUnicast<'a, Mode>>,
    }

    pub mod storm_control {

        #[::validated_data::data_view]
        pub struct All<'a, Mode> {
            pub level: ::validated_data::Field<&'a str>,
            pub unit: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct Broadcast<'a, Mode> {
            pub level: ::validated_data::Field<&'a str>,
            pub unit: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct Multicast<'a, Mode> {
            pub level: ::validated_data::Field<&'a str>,
            pub unit: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct UnknownUnicast<'a, Mode> {
            pub level: ::validated_data::Field<&'a str>,
            pub unit: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(list)]
    pub struct MonitorSessions<'a, Mode> (::validated_data::Field<monitor_sessions::Item<'a, Mode>>);

    pub mod monitor_sessions {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::RequiredValue<&'a str, Mode>,
            pub role: ::validated_data::Field<&'a str>,
            pub source_settings: ::validated_data::Field<item::SourceSettings<'a, Mode>>,
            pub session_settings: ::validated_data::Field<item::SessionSettings<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct SourceSettings<'a, Mode> {
                pub direction: ::validated_data::Field<&'a str>,
                pub access_group: ::validated_data::Field<source_settings::AccessGroup<'a, Mode>>,
            }

            pub mod source_settings {

                #[::validated_data::data_view]
                pub struct AccessGroup<'a, Mode> {
                    #[data_view(rename = "type")]
                    pub field_type: ::validated_data::Field<&'a str>,
                    pub name: ::validated_data::Field<&'a str>,
                    pub priority: ::validated_data::Field<i64>,
                }
            }

            #[::validated_data::data_view]
            pub struct SessionSettings<'a, Mode> {
                pub encapsulation_gre_metadata_tx: ::validated_data::Field<bool>,
                pub header_remove_size: ::validated_data::Field<i64>,
                pub access_group: ::validated_data::Field<session_settings::AccessGroup<'a, Mode>>,
                pub rate_limit_per_ingress_chip: ::validated_data::Field<&'a str>,
                pub rate_limit_per_egress_chip: ::validated_data::Field<&'a str>,
                pub sample: ::validated_data::Field<i64>,
                pub truncate: ::validated_data::Field<session_settings::Truncate<'a, Mode>>,
            }

            pub mod session_settings {

                #[::validated_data::data_view]
                pub struct AccessGroup<'a, Mode> {
                    #[data_view(rename = "type")]
                    pub field_type: ::validated_data::Field<&'a str>,
                    pub name: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view]
                pub struct Truncate<'a, Mode> {
                    pub enabled: ::validated_data::Field<bool>,
                    pub size: ::validated_data::Field<i64>,
                }
            }
        }
    }

    #[::validated_data::data_view]
    pub struct EthernetSegment<'a, Mode> {
        pub short_esi: ::validated_data::RequiredValue<&'a str, Mode>,
        pub redundancy: ::validated_data::Field<&'a str>,
        pub designated_forwarder_algorithm: ::validated_data::Field<&'a str>,
        pub designated_forwarder_preferences: ::validated_data::Field<ethernet_segment::DesignatedForwarderPreferences<'a, Mode>>,
        pub dont_preempt: ::validated_data::Field<bool>,
    }

    pub mod ethernet_segment {

        #[::validated_data::data_view(list)]
        pub struct DesignatedForwarderPreferences<'a, Mode> (::validated_data::Field<i64>);
    }

    #[::validated_data::data_view(list)]
    pub struct CampusLinkType<'a, Mode> (::validated_data::Field<&'a str>);
}
