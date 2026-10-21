// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(list, python(name = "AVDDesign.NetworkPorts", schema = List(686)))]
pub struct NetworkPorts<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem", schema = Dict(1633)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = List(683))]
    pub switches: ::data_views::Field<item::Switches<'a, Mode>>,
    #[data_view(schema = List(684))]
    pub platforms: ::data_views::Field<item::Platforms<'a, Mode>>,
    #[data_view(schema = List(685))]
    pub switch_ports: ::data_views::Field<item::SwitchPorts<'a, Mode>>,
    #[data_view(schema = Str(1471))]
    pub description: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1472))]
    pub endpoint: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1145))]
    pub speed: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1140))]
    pub profile: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(431))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = Str(1147))]
    pub mode: ::data_views::Field<&'a str>,
    #[data_view(schema = Int(72))]
    pub mtu: ::data_views::Field<i64>,
    #[data_view(schema = Int(73))]
    pub l2_mtu: ::data_views::Field<i64>,
    #[data_view(schema = Int(74))]
    pub l2_mru: ::data_views::Field<i64>,
    #[data_view(schema = Int(515))]
    pub native_vlan: ::data_views::Field<i64>,
    #[data_view(schema = Bool(432))]
    pub native_vlan_tag: ::data_views::Field<bool>,
    #[data_view(schema = Int(516))]
    pub phone_vlan: ::data_views::Field<i64>,
    #[data_view(schema = Str(1148))]
    pub phone_trunk_mode: ::data_views::Field<&'a str>,
    #[data_view(schema = List(571))]
    pub trunk_groups: ::data_views::Field<item::TrunkGroups<'a, Mode>>,
    #[data_view(schema = Str(1149))]
    pub vlans: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1150))]
    pub mac_acl_in: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1151))]
    pub mac_acl_out: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(263))]
    pub spanning_tree_portfast: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(261))]
    pub spanning_tree_bpdufilter: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(261))]
    pub spanning_tree_bpduguard: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(264))]
    pub spanning_tree_link_type: ::data_views::Field<&'a str>,
    #[data_view(schema = Dict(213))]
    pub flowcontrol: ::data_views::Field<super::super::eos_config::ethernet_interfaces::item::Flowcontrol<'a, Mode>>,
    #[data_view(schema = Str(1152))]
    pub qos_profile: ::data_views::Field<&'a str>,
    #[data_view(schema = Dict(1417))]
    pub ptp: ::data_views::Field<item::Ptp<'a, Mode>>,
    #[data_view(schema = Bool(433))]
    pub sflow: ::data_views::Field<bool>,
    #[data_view(schema = Dict(1418))]
    pub flow_tracking: ::data_views::Field<item::FlowTracking<'a, Mode>>,
    #[data_view(schema = Dict(1419))]
    pub link_tracking: ::data_views::Field<item::LinkTracking<'a, Mode>>,
    #[data_view(schema = Dict(1421))]
    pub dot1x: ::data_views::Field<item::Dot1x<'a, Mode>>,
    #[data_view(schema = Dict(1422))]
    pub address_locking: ::data_views::Field<item::AddressLocking<'a, Mode>>,
    #[data_view(schema = Dict(1423))]
    pub poe: ::data_views::Field<super::super::eos_config::ethernet_interfaces::item::Poe<'a, Mode>>,
    #[data_view(schema = Dict(1425))]
    pub storm_control: ::data_views::Field<item::StormControl<'a, Mode>>,
    #[data_view(schema = List(572))]
    pub monitor_sessions: ::data_views::Field<item::MonitorSessions<'a, Mode>>,
    #[data_view(schema = Dict(1431))]
    pub ethernet_segment: ::data_views::Field<item::EthernetSegment<'a, Mode>>,
    #[data_view(schema = Dict(1632))]
    pub port_channel: ::data_views::Field<item::PortChannel<'a, Mode>>,
    #[data_view(schema = Bool(161))]
    pub validate_state: ::data_views::Field<bool>,
    #[data_view(schema = Bool(162))]
    pub validate_lldp: ::data_views::Field<bool>,
    #[data_view(schema = List(574))]
    pub campus_link_type: ::data_views::Field<item::CampusLinkType<'a, Mode>>,
    #[data_view(schema = Str(1166))]
    pub raw_eos_cli: ::data_views::Field<&'a str>,
    #[data_view(schema = Dict(1432), relaxed)]
    pub structured_config: ::data_views::Field<super::super::eos_config::ethernet_interfaces::Item<'a, ::data_views::RelaxedValidated>>,
}

pub mod item {

    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkPortsItem.Switches", schema = List(683)))]
    pub struct Switches<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkPortsItem.Platforms", schema = List(684)))]
    pub struct Platforms<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkPortsItem.SwitchPorts", schema = List(685)))]
    pub struct SwitchPorts<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkPortsItem.TrunkGroups", schema = List(571)))]
    pub struct TrunkGroups<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.Ptp", schema = Dict(1417)))]
    pub struct Ptp<'a, Mode> {
        #[data_view(schema = Bool(340))]
        pub enabled: ::data_views::Field<bool>,
        #[data_view(schema = Str(1153))]
        pub endpoint_role: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1154))]
        pub profile: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.FlowTracking", schema = Dict(1418)))]
    pub struct FlowTracking<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub enabled: ::data_views::Field<bool>,
        #[data_view(schema = Str(1155))]
        pub name: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.LinkTracking", schema = Dict(1419)))]
    pub struct LinkTracking<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub enabled: ::data_views::Field<bool>,
        #[data_view(schema = Str(1156))]
        pub name: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.Dot1x", schema = Dict(1421)))]
    pub struct Dot1x<'a, Mode> {
        #[data_view(schema = Dict(1420))]
        pub authentication_failure: ::data_views::Field<dot1x::AuthenticationFailure<'a, Mode>>,
        #[data_view(schema = Str(249))]
        pub port_control: ::data_views::Field<&'a str>,
        #[data_view(schema = Bool(7))]
        pub port_control_force_authorized_phone: ::data_views::Field<bool>,
        #[data_view(schema = Bool(7))]
        pub reauthentication: ::data_views::Field<bool>,
        #[data_view(schema = Dict(277))]
        pub pae: ::data_views::Field<dot1x::Pae<'a, Mode>>,
        #[data_view(schema = Dict(279))]
        pub host_mode: ::data_views::Field<dot1x::HostMode<'a, Mode>>,
        #[data_view(schema = Dict(280))]
        pub mac_based_authentication: ::data_views::Field<dot1x::MacBasedAuthentication<'a, Mode>>,
        #[data_view(schema = Bool(157))]
        pub mac_based_access_list: ::data_views::Field<bool>,
        #[data_view(schema = Dict(281))]
        pub timeout: ::data_views::Field<dot1x::Timeout<'a, Mode>>,
        #[data_view(schema = Int(109))]
        pub reauthorization_request_limit: ::data_views::Field<i64>,
        #[data_view(schema = Dict(282))]
        pub unauthorized: ::data_views::Field<dot1x::Unauthorized<'a, Mode>>,
        #[data_view(schema = Dict(284))]
        pub eapol: ::data_views::Field<dot1x::Eapol<'a, Mode>>,
        #[data_view(schema = Dict(287))]
        pub aaa: ::data_views::Field<dot1x::Aaa<'a, Mode>>,
    }

    pub mod dot1x {

        #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.Dot1x.AuthenticationFailure", schema = Dict(1420)))]
        pub struct AuthenticationFailure<'a, Mode> {
            #[data_view(schema = Str(1157))]
            pub allow_access_list: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(252))]
            pub action: ::data_views::Field<&'a str>,
            #[data_view(schema = Int(26))]
            pub allow_vlan: ::data_views::Field<i64>,
        }

        #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.Dot1x.Pae", schema = Dict(277)))]
        pub struct Pae<'a, Mode> {
            #[data_view(schema = Str(250))]
            pub mode: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(251))]
            pub supplicant_profile: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.Dot1x.HostMode", schema = Dict(279)))]
        pub struct HostMode<'a, Mode> {
            #[data_view(schema = Str(254))]
            pub mode: ::data_views::Field<&'a str>,
            #[data_view(schema = Bool(7))]
            pub multi_host_authenticated: ::data_views::Field<bool>,
        }

        #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.Dot1x.MacBasedAuthentication", schema = Dict(280)))]
        pub struct MacBasedAuthentication<'a, Mode> {
            #[data_view(schema = Bool(7))]
            pub enabled: ::data_views::Field<bool>,
            #[data_view(schema = Bool(7))]
            pub always: ::data_views::Field<bool>,
            #[data_view(schema = Bool(7))]
            pub host_mode_common: ::data_views::Field<bool>,
        }

        #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.Dot1x.Timeout", schema = Dict(281)))]
        pub struct Timeout<'a, Mode> {
            #[data_view(schema = Int(107))]
            pub idle_host: ::data_views::Field<i64>,
            #[data_view(schema = Int(108))]
            pub quiet_period: ::data_views::Field<i64>,
            #[data_view(schema = Str(255))]
            pub reauth_period: ::data_views::Field<&'a str>,
            #[data_view(schema = Bool(7))]
            pub reauth_timeout_ignore: ::data_views::Field<bool>,
            #[data_view(schema = Int(108))]
            pub tx_period: ::data_views::Field<i64>,
        }

        #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.Dot1x.Unauthorized", schema = Dict(282)))]
        pub struct Unauthorized<'a, Mode> {
            #[data_view(schema = Bool(7))]
            pub access_vlan_membership_egress: ::data_views::Field<bool>,
            #[data_view(schema = Bool(7))]
            pub native_vlan_membership_egress: ::data_views::Field<bool>,
        }

        #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.Dot1x.Eapol", schema = Dict(284)))]
        pub struct Eapol<'a, Mode> {
            #[data_view(schema = Bool(7))]
            pub disabled: ::data_views::Field<bool>,
            #[data_view(schema = Dict(283))]
            pub authentication_failure_fallback_mba: ::data_views::Field<eapol::AuthenticationFailureFallbackMba<'a, Mode>>,
        }

        pub mod eapol {

            #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.Dot1x.Eapol.AuthenticationFailureFallbackMba", schema = Dict(283)))]
            pub struct AuthenticationFailureFallbackMba<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::data_views::Field<bool>,
                #[data_view(schema = Int(110))]
                pub timeout: ::data_views::Field<i64>,
            }
        }

        #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.Dot1x.Aaa", schema = Dict(287)))]
        pub struct Aaa<'a, Mode> {
            #[data_view(schema = Dict(286))]
            pub unresponsive: ::data_views::Field<aaa::Unresponsive<'a, Mode>>,
        }

        pub mod aaa {

            #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.Dot1x.Aaa.Unresponsive", schema = Dict(286)))]
            pub struct Unresponsive<'a, Mode> {
                #[data_view(schema = Str(256))]
                pub eap_response: ::data_views::Field<&'a str>,
                #[data_view(schema = Dict(285))]
                pub action: ::data_views::Field<unresponsive::Action<'a, Mode>>,
                #[data_view(schema = Dict(153))]
                pub phone_action: ::data_views::Field<super::super::super::super::super::eos_config::dot1x::aaa::unresponsive::PhoneAction<'a, Mode>>,
            }

            pub mod unresponsive {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.Dot1x.Aaa.Unresponsive.Action", schema = Dict(285)))]
                pub struct Action<'a, Mode> {
                    #[data_view(schema = Str(257))]
                    pub traffic_allow_access_list: ::data_views::Field<&'a str>,
                    #[data_view(schema = Bool(47))]
                    pub apply_alternate: ::data_views::Field<bool>,
                    #[data_view(schema = Int(26))]
                    pub traffic_allow_vlan: ::data_views::Field<i64>,
                    #[data_view(schema = Bool(48))]
                    pub apply_cached_results: ::data_views::Field<bool>,
                    #[data_view(schema = Dict(151))]
                    pub cached_results_timeout: ::data_views::Field<action::CachedResultsTimeout<'a, Mode>>,
                    #[data_view(schema = Bool(49))]
                    pub traffic_allow: ::data_views::Field<bool>,
                }

                pub mod action {

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.Dot1x.Aaa.Unresponsive.Action.CachedResultsTimeout", schema = Dict(151)))]
                    pub struct CachedResultsTimeout<'a, Mode> {
                        #[data_view(schema = Int(27))]
                        pub time_duration: ::data_views::RequiredValue<i64, Mode>,
                        #[data_view(schema = Str(155))]
                        pub time_duration_unit: ::data_views::RequiredValue<&'a str, Mode>,
                    }
                }
            }
        }
    }

    #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.AddressLocking", schema = Dict(1422)))]
    pub struct AddressLocking<'a, Mode> {
        #[data_view(schema = Bool(434))]
        pub ipv4: ::data_views::Field<bool>,
        #[data_view(schema = Bool(435))]
        pub ipv6: ::data_views::Field<bool>,
    }

    #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.StormControl", schema = Dict(1425)))]
    pub struct StormControl<'a, Mode> {
        #[data_view(schema = Dict(1424))]
        pub all: ::data_views::Field<storm_control::All<'a, Mode>>,
        #[data_view(schema = Dict(1424))]
        pub broadcast: ::data_views::Field<storm_control::Broadcast<'a, Mode>>,
        #[data_view(schema = Dict(1424))]
        pub multicast: ::data_views::Field<storm_control::Multicast<'a, Mode>>,
        #[data_view(schema = Dict(1424))]
        pub unknown_unicast: ::data_views::Field<storm_control::UnknownUnicast<'a, Mode>>,
    }

    pub mod storm_control {

        #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.StormControl.All", schema = Dict(1424)))]
        pub struct All<'a, Mode> {
            #[data_view(schema = Str(247))]
            pub level: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1158))]
            pub unit: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.StormControl.Broadcast", schema = Dict(1424)))]
        pub struct Broadcast<'a, Mode> {
            #[data_view(schema = Str(247))]
            pub level: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1158))]
            pub unit: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.StormControl.Multicast", schema = Dict(1424)))]
        pub struct Multicast<'a, Mode> {
            #[data_view(schema = Str(247))]
            pub level: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1158))]
            pub unit: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.StormControl.UnknownUnicast", schema = Dict(1424)))]
        pub struct UnknownUnicast<'a, Mode> {
            #[data_view(schema = Str(247))]
            pub level: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1158))]
            pub unit: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkPortsItem.MonitorSessions", schema = List(572)))]
    pub struct MonitorSessions<'a, Mode> (::data_views::Field<monitor_sessions::Item<'a, Mode>>);

    pub mod monitor_sessions {

        #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.MonitorSessionsItem", schema = Dict(1430)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(1159))]
            pub name: ::data_views::RequiredValue<&'a str, Mode>,
            #[data_view(schema = Str(1160))]
            pub role: ::data_views::Field<&'a str>,
            #[data_view(schema = Dict(1427))]
            pub source_settings: ::data_views::Field<item::SourceSettings<'a, Mode>>,
            #[data_view(schema = Dict(1429))]
            pub session_settings: ::data_views::Field<item::SessionSettings<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.MonitorSessionsItem.SourceSettings", schema = Dict(1427)))]
            pub struct SourceSettings<'a, Mode> {
                #[data_view(schema = Str(593))]
                pub direction: ::data_views::Field<&'a str>,
                #[data_view(schema = Dict(1426))]
                pub access_group: ::data_views::Field<source_settings::AccessGroup<'a, Mode>>,
            }

            pub mod source_settings {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.MonitorSessionsItem.SourceSettings.AccessGroup", schema = Dict(1426)))]
                pub struct AccessGroup<'a, Mode> {
                    #[data_view(schema = Str(594), rename = "type")]
                    pub field_type: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(1161))]
                    pub name: ::data_views::Field<&'a str>,
                    #[data_view(schema = Int(77))]
                    pub priority: ::data_views::Field<i64>,
                }
            }

            #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.MonitorSessionsItem.SessionSettings", schema = Dict(1429)))]
            pub struct SessionSettings<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub encapsulation_gre_metadata_tx: ::data_views::Field<bool>,
                #[data_view(schema = Int(273))]
                pub header_remove_size: ::data_views::Field<i64>,
                #[data_view(schema = Dict(1428))]
                pub access_group: ::data_views::Field<session_settings::AccessGroup<'a, Mode>>,
                #[data_view(schema = Str(597))]
                pub rate_limit_per_ingress_chip: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(597))]
                pub rate_limit_per_egress_chip: ::data_views::Field<&'a str>,
                #[data_view(schema = Int(77))]
                pub sample: ::data_views::Field<i64>,
                #[data_view(schema = Dict(666))]
                pub truncate: ::data_views::Field<session_settings::Truncate<'a, Mode>>,
            }

            pub mod session_settings {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.MonitorSessionsItem.SessionSettings.AccessGroup", schema = Dict(1428)))]
                pub struct AccessGroup<'a, Mode> {
                    #[data_view(schema = Str(594), rename = "type")]
                    pub field_type: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(1161))]
                    pub name: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.MonitorSessionsItem.SessionSettings.Truncate", schema = Dict(666)))]
                pub struct Truncate<'a, Mode> {
                    #[data_view(schema = Bool(7))]
                    pub enabled: ::data_views::Field<bool>,
                    #[data_view(schema = Int(274))]
                    pub size: ::data_views::Field<i64>,
                }
            }
        }
    }

    #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.EthernetSegment", schema = Dict(1431)))]
    pub struct EthernetSegment<'a, Mode> {
        #[data_view(schema = Str(1162))]
        pub short_esi: ::data_views::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Str(1163))]
        pub redundancy: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1164))]
        pub designated_forwarder_algorithm: ::data_views::Field<&'a str>,
        #[data_view(schema = List(573))]
        pub designated_forwarder_preferences: ::data_views::Field<ethernet_segment::DesignatedForwarderPreferences<'a, Mode>>,
        #[data_view(schema = Bool(436))]
        pub dont_preempt: ::data_views::Field<bool>,
    }

    pub mod ethernet_segment {

        #[::data_views::data_view(list, python(name = "AVDDesign.NetworkPortsItem.EthernetSegment.DesignatedForwarderPreferences", schema = List(573)))]
        pub struct DesignatedForwarderPreferences<'a, Mode> (::data_views::Field<i64>);
    }

    #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.PortChannel", schema = Dict(1632)))]
    pub struct PortChannel<'a, Mode> {
        #[data_view(schema = Str(1136))]
        pub mode: ::data_views::Field<&'a str>,
        #[data_view(schema = Int(511))]
        pub channel_id: ::data_views::Field<i64>,
        #[data_view(schema = Str(1137))]
        pub description: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1138))]
        pub endpoint_port_channel: ::data_views::Field<&'a str>,
        #[data_view(schema = Bool(429))]
        pub enabled: ::data_views::Field<bool>,
        #[data_view(schema = Bool(430))]
        pub ptp_mpass: ::data_views::Field<bool>,
        #[data_view(schema = Dict(1413))]
        pub lacp_fallback: ::data_views::Field<port_channel::LacpFallback<'a, Mode>>,
        #[data_view(schema = Dict(1414))]
        pub lacp_timer: ::data_views::Field<port_channel::LacpTimer<'a, Mode>>,
        #[data_view(schema = Str(1144))]
        pub raw_eos_cli: ::data_views::Field<&'a str>,
        #[data_view(schema = Dict(1415), relaxed)]
        pub structured_config: ::data_views::Field<super::super::super::eos_config::port_channel_interfaces::Item<'a, ::data_views::RelaxedValidated>>,
    }

    pub mod port_channel {

        #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.PortChannel.LacpFallback", schema = Dict(1413)))]
        pub struct LacpFallback<'a, Mode> {
            #[data_view(schema = Str(1139))]
            pub mode: ::data_views::Field<&'a str>,
            #[data_view(schema = Dict(1412))]
            pub individual: ::data_views::Field<lacp_fallback::Individual<'a, Mode>>,
            #[data_view(schema = Int(513))]
            pub timeout: ::data_views::Field<i64>,
        }

        pub mod lacp_fallback {

            #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.PortChannel.LacpFallback.Individual", schema = Dict(1412)))]
            pub struct Individual<'a, Mode> {
                #[data_view(schema = Str(1140))]
                pub profile: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(1141))]
                pub vlans: ::data_views::Field<&'a str>,
                #[data_view(schema = Int(512))]
                pub native_vlan: ::data_views::Field<i64>,
                #[data_view(schema = Str(1142))]
                pub mode: ::data_views::Field<&'a str>,
            }
        }

        #[::data_views::data_view(python(name = "AVDDesign.NetworkPortsItem.PortChannel.LacpTimer", schema = Dict(1414)))]
        pub struct LacpTimer<'a, Mode> {
            #[data_view(schema = Str(1143))]
            pub mode: ::data_views::Field<&'a str>,
            #[data_view(schema = Int(514))]
            pub multiplier: ::data_views::Field<i64>,
        }
    }

    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkPortsItem.CampusLinkType", schema = List(574)))]
    pub struct CampusLinkType<'a, Mode> (::data_views::Field<&'a str>);
}
