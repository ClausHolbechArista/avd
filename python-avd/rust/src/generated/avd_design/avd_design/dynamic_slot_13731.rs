// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase", schema = Dict(1806)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1125))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1701), rename = "type")]
    pub field_type: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1127))]
    pub rack: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(575))]
    pub adapters: ::validated_data::Field<item::Adapters<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.Adapters", schema = List(575)))]
    pub struct Adapters<'a, Mode> (::validated_data::Field<adapters::Item<'a, Mode>>);

    pub mod adapters {

        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem", schema = Dict(1433)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = List(565))]
            pub switch_ports: ::validated_data::RequiredValue<item::SwitchPorts<'a, Mode>, Mode>,
            #[data_view(schema = List(566))]
            pub switches: ::validated_data::RequiredValue<item::Switches<'a, Mode>, Mode>,
            #[data_view(schema = List(567))]
            pub endpoint_ports: ::validated_data::Field<item::EndpointPorts<'a, Mode>>,
            #[data_view(schema = List(568))]
            pub descriptions: ::validated_data::Field<item::Descriptions<'a, Mode>>,
            #[data_view(schema = List(569))]
            pub subinterfaces: ::validated_data::Field<item::Subinterfaces<'a, Mode>>,
            #[data_view(schema = Dict(1416))]
            pub port_channel: ::validated_data::Field<item::PortChannel<'a, Mode>>,
            #[data_view(schema = Str(1145))]
            pub speed: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1146))]
            pub description: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1140))]
            pub profile: ::validated_data::Field<&'a str>,
            #[data_view(schema = Bool(431))]
            pub enabled: ::validated_data::Field<bool>,
            #[data_view(schema = Str(1147))]
            pub mode: ::validated_data::Field<&'a str>,
            #[data_view(schema = Int(72))]
            pub mtu: ::validated_data::Field<i64>,
            #[data_view(schema = Int(73))]
            pub l2_mtu: ::validated_data::Field<i64>,
            #[data_view(schema = Int(74))]
            pub l2_mru: ::validated_data::Field<i64>,
            #[data_view(schema = Int(515))]
            pub native_vlan: ::validated_data::Field<i64>,
            #[data_view(schema = Bool(432))]
            pub native_vlan_tag: ::validated_data::Field<bool>,
            #[data_view(schema = Int(516))]
            pub phone_vlan: ::validated_data::Field<i64>,
            #[data_view(schema = Str(1148))]
            pub phone_trunk_mode: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(571))]
            pub trunk_groups: ::validated_data::Field<item::TrunkGroups<'a, Mode>>,
            #[data_view(schema = Str(1149))]
            pub vlans: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1150))]
            pub mac_acl_in: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1151))]
            pub mac_acl_out: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(263))]
            pub spanning_tree_portfast: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(261))]
            pub spanning_tree_bpdufilter: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(261))]
            pub spanning_tree_bpduguard: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(264))]
            pub spanning_tree_link_type: ::validated_data::Field<&'a str>,
            #[data_view(schema = Dict(213))]
            pub flowcontrol: ::validated_data::Field<super::super::super::super::eos_config::ethernet_interfaces::item::Flowcontrol<'a, Mode>>,
            #[data_view(schema = Str(1152))]
            pub qos_profile: ::validated_data::Field<&'a str>,
            #[data_view(schema = Dict(1417))]
            pub ptp: ::validated_data::Field<item::Ptp<'a, Mode>>,
            #[data_view(schema = Bool(433))]
            pub sflow: ::validated_data::Field<bool>,
            #[data_view(schema = Dict(1418))]
            pub flow_tracking: ::validated_data::Field<item::FlowTracking<'a, Mode>>,
            #[data_view(schema = Dict(1419))]
            pub link_tracking: ::validated_data::Field<item::LinkTracking<'a, Mode>>,
            #[data_view(schema = Dict(1421))]
            pub dot1x: ::validated_data::Field<item::Dot1x<'a, Mode>>,
            #[data_view(schema = Dict(1422))]
            pub address_locking: ::validated_data::Field<item::AddressLocking<'a, Mode>>,
            #[data_view(schema = Dict(1423))]
            pub poe: ::validated_data::Field<super::super::super::super::eos_config::ethernet_interfaces::item::Poe<'a, Mode>>,
            #[data_view(schema = Dict(1425))]
            pub storm_control: ::validated_data::Field<item::StormControl<'a, Mode>>,
            #[data_view(schema = List(572))]
            pub monitor_sessions: ::validated_data::Field<item::MonitorSessions<'a, Mode>>,
            #[data_view(schema = Dict(1431))]
            pub ethernet_segment: ::validated_data::Field<item::EthernetSegment<'a, Mode>>,
            #[data_view(schema = Bool(161))]
            pub validate_state: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(162))]
            pub validate_lldp: ::validated_data::Field<bool>,
            #[data_view(schema = List(574))]
            pub campus_link_type: ::validated_data::Field<item::CampusLinkType<'a, Mode>>,
            #[data_view(schema = Str(1166))]
            pub raw_eos_cli: ::validated_data::Field<&'a str>,
            #[data_view(schema = Dict(1432), relaxed)]
            pub structured_config: ::validated_data::Field<super::super::super::super::eos_config::ethernet_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.SwitchPorts", schema = List(565)))]
            pub struct SwitchPorts<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.Switches", schema = List(566)))]
            pub struct Switches<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.EndpointPorts", schema = List(567)))]
            pub struct EndpointPorts<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.Descriptions", schema = List(568)))]
            pub struct Descriptions<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(indexed_list, primary_key(number), python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.Subinterfaces", schema = List(569), item_name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.SubinterfacesItem"))]
            pub struct Subinterfaces<'a, Mode> (::validated_data::Field<subinterfaces::Item<'a, Mode>>);

            pub mod subinterfaces {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.SubinterfacesItemBase", schema = Dict(1408)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Int(506))]
                    pub number: ::validated_data::Field<i64>,
                    #[data_view(schema = Str(1131))]
                    pub description: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1132))]
                    pub short_esi: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Int(507))]
                    pub vlan_id: ::validated_data::Field<i64>,
                    #[data_view(schema = Dict(1406))]
                    pub encapsulation_vlan: ::validated_data::Field<item::EncapsulationVlan<'a, Mode>>,
                    #[data_view(schema = Str(1133))]
                    pub raw_eos_cli: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Dict(1407), relaxed)]
                    pub structured_config: ::validated_data::Field<super::super::super::super::super::super::eos_config::ethernet_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
                }

                pub mod item {

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.SubinterfacesItemBase.EncapsulationVlan", schema = Dict(1406)))]
                    pub struct EncapsulationVlan<'a, Mode> {
                        #[data_view(schema = Int(508))]
                        pub client_dot1q: ::validated_data::Field<i64>,
                    }
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.PortChannel", schema = Dict(1416)))]
            pub struct PortChannel<'a, Mode> {
                #[data_view(schema = List(570))]
                pub subinterfaces: ::validated_data::Field<port_channel::Subinterfaces<'a, Mode>>,
                #[data_view(schema = Str(1136))]
                pub mode: ::validated_data::Field<&'a str>,
                #[data_view(schema = Int(511))]
                pub channel_id: ::validated_data::Field<i64>,
                #[data_view(schema = Str(1137))]
                pub description: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(1138))]
                pub endpoint_port_channel: ::validated_data::Field<&'a str>,
                #[data_view(schema = Bool(429))]
                pub enabled: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(430))]
                pub ptp_mpass: ::validated_data::Field<bool>,
                #[data_view(schema = Dict(1413))]
                pub lacp_fallback: ::validated_data::Field<port_channel::LacpFallback<'a, Mode>>,
                #[data_view(schema = Dict(1414))]
                pub lacp_timer: ::validated_data::Field<port_channel::LacpTimer<'a, Mode>>,
                #[data_view(schema = Str(1144))]
                pub raw_eos_cli: ::validated_data::Field<&'a str>,
                #[data_view(schema = Dict(1415), relaxed)]
                pub structured_config: ::validated_data::Field<super::super::super::super::super::eos_config::port_channel_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
            }

            pub mod port_channel {

                #[::validated_data::data_view(indexed_list, primary_key(number), python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.PortChannel.Subinterfaces", schema = List(570), item_name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.PortChannel.SubinterfacesItem"))]
                pub struct Subinterfaces<'a, Mode> (::validated_data::Field<subinterfaces::Item<'a, Mode>>);

                pub mod subinterfaces {

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.PortChannel.SubinterfacesItemBase", schema = Dict(1411)))]
                    pub struct Item<'a, Mode> {
                        #[data_view(schema = Int(506))]
                        pub number: ::validated_data::Field<i64>,
                        #[data_view(schema = Str(1131))]
                        pub description: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Str(1134))]
                        pub short_esi: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Int(509))]
                        pub vlan_id: ::validated_data::Field<i64>,
                        #[data_view(schema = Dict(1409))]
                        pub encapsulation_vlan: ::validated_data::Field<item::EncapsulationVlan<'a, Mode>>,
                        #[data_view(schema = Str(1135))]
                        pub raw_eos_cli: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Dict(1410), relaxed)]
                        pub structured_config: ::validated_data::Field<super::super::super::super::super::super::super::eos_config::port_channel_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
                    }

                    pub mod item {

                        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.PortChannel.SubinterfacesItemBase.EncapsulationVlan", schema = Dict(1409)))]
                        pub struct EncapsulationVlan<'a, Mode> {
                            #[data_view(schema = Int(510))]
                            pub client_dot1q: ::validated_data::Field<i64>,
                        }
                    }
                }

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.PortChannel.LacpFallback", schema = Dict(1413)))]
                pub struct LacpFallback<'a, Mode> {
                    #[data_view(schema = Str(1139))]
                    pub mode: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Dict(1412))]
                    pub individual: ::validated_data::Field<lacp_fallback::Individual<'a, Mode>>,
                    #[data_view(schema = Int(513))]
                    pub timeout: ::validated_data::Field<i64>,
                }

                pub mod lacp_fallback {

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.PortChannel.LacpFallback.Individual", schema = Dict(1412)))]
                    pub struct Individual<'a, Mode> {
                        #[data_view(schema = Str(1140))]
                        pub profile: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Str(1141))]
                        pub vlans: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Int(512))]
                        pub native_vlan: ::validated_data::Field<i64>,
                        #[data_view(schema = Str(1142))]
                        pub mode: ::validated_data::Field<&'a str>,
                    }
                }

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.PortChannel.LacpTimer", schema = Dict(1414)))]
                pub struct LacpTimer<'a, Mode> {
                    #[data_view(schema = Str(1143))]
                    pub mode: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Int(514))]
                    pub multiplier: ::validated_data::Field<i64>,
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.TrunkGroups", schema = List(571)))]
            pub struct TrunkGroups<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.Ptp", schema = Dict(1417)))]
            pub struct Ptp<'a, Mode> {
                #[data_view(schema = Bool(340))]
                pub enabled: ::validated_data::Field<bool>,
                #[data_view(schema = Str(1153))]
                pub endpoint_role: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(1154))]
                pub profile: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.FlowTracking", schema = Dict(1418)))]
            pub struct FlowTracking<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::validated_data::Field<bool>,
                #[data_view(schema = Str(1155))]
                pub name: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.LinkTracking", schema = Dict(1419)))]
            pub struct LinkTracking<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::validated_data::Field<bool>,
                #[data_view(schema = Str(1156))]
                pub name: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.Dot1x", schema = Dict(1421)))]
            pub struct Dot1x<'a, Mode> {
                #[data_view(schema = Dict(1420))]
                pub authentication_failure: ::validated_data::Field<dot1x::AuthenticationFailure<'a, Mode>>,
                #[data_view(schema = Str(249))]
                pub port_control: ::validated_data::Field<&'a str>,
                #[data_view(schema = Bool(7))]
                pub port_control_force_authorized_phone: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(7))]
                pub reauthentication: ::validated_data::Field<bool>,
                #[data_view(schema = Dict(277))]
                pub pae: ::validated_data::Field<dot1x::Pae<'a, Mode>>,
                #[data_view(schema = Dict(279))]
                pub host_mode: ::validated_data::Field<dot1x::HostMode<'a, Mode>>,
                #[data_view(schema = Dict(280))]
                pub mac_based_authentication: ::validated_data::Field<dot1x::MacBasedAuthentication<'a, Mode>>,
                #[data_view(schema = Bool(157))]
                pub mac_based_access_list: ::validated_data::Field<bool>,
                #[data_view(schema = Dict(281))]
                pub timeout: ::validated_data::Field<dot1x::Timeout<'a, Mode>>,
                #[data_view(schema = Int(109))]
                pub reauthorization_request_limit: ::validated_data::Field<i64>,
                #[data_view(schema = Dict(282))]
                pub unauthorized: ::validated_data::Field<dot1x::Unauthorized<'a, Mode>>,
                #[data_view(schema = Dict(284))]
                pub eapol: ::validated_data::Field<dot1x::Eapol<'a, Mode>>,
                #[data_view(schema = Dict(287))]
                pub aaa: ::validated_data::Field<dot1x::Aaa<'a, Mode>>,
            }

            pub mod dot1x {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.Dot1x.AuthenticationFailure", schema = Dict(1420)))]
                pub struct AuthenticationFailure<'a, Mode> {
                    #[data_view(schema = Str(1157))]
                    pub allow_access_list: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(252))]
                    pub action: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Int(26))]
                    pub allow_vlan: ::validated_data::Field<i64>,
                }

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.Dot1x.Pae", schema = Dict(277)))]
                pub struct Pae<'a, Mode> {
                    #[data_view(schema = Str(250))]
                    pub mode: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(251))]
                    pub supplicant_profile: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.Dot1x.HostMode", schema = Dict(279)))]
                pub struct HostMode<'a, Mode> {
                    #[data_view(schema = Str(254))]
                    pub mode: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Bool(7))]
                    pub multi_host_authenticated: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.Dot1x.MacBasedAuthentication", schema = Dict(280)))]
                pub struct MacBasedAuthentication<'a, Mode> {
                    #[data_view(schema = Bool(7))]
                    pub enabled: ::validated_data::Field<bool>,
                    #[data_view(schema = Bool(7))]
                    pub always: ::validated_data::Field<bool>,
                    #[data_view(schema = Bool(7))]
                    pub host_mode_common: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.Dot1x.Timeout", schema = Dict(281)))]
                pub struct Timeout<'a, Mode> {
                    #[data_view(schema = Int(107))]
                    pub idle_host: ::validated_data::Field<i64>,
                    #[data_view(schema = Int(108))]
                    pub quiet_period: ::validated_data::Field<i64>,
                    #[data_view(schema = Str(255))]
                    pub reauth_period: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Bool(7))]
                    pub reauth_timeout_ignore: ::validated_data::Field<bool>,
                    #[data_view(schema = Int(108))]
                    pub tx_period: ::validated_data::Field<i64>,
                }

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.Dot1x.Unauthorized", schema = Dict(282)))]
                pub struct Unauthorized<'a, Mode> {
                    #[data_view(schema = Bool(7))]
                    pub access_vlan_membership_egress: ::validated_data::Field<bool>,
                    #[data_view(schema = Bool(7))]
                    pub native_vlan_membership_egress: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.Dot1x.Eapol", schema = Dict(284)))]
                pub struct Eapol<'a, Mode> {
                    #[data_view(schema = Bool(7))]
                    pub disabled: ::validated_data::Field<bool>,
                    #[data_view(schema = Dict(283))]
                    pub authentication_failure_fallback_mba: ::validated_data::Field<eapol::AuthenticationFailureFallbackMba<'a, Mode>>,
                }

                pub mod eapol {

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.Dot1x.Eapol.AuthenticationFailureFallbackMba", schema = Dict(283)))]
                    pub struct AuthenticationFailureFallbackMba<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub enabled: ::validated_data::Field<bool>,
                        #[data_view(schema = Int(110))]
                        pub timeout: ::validated_data::Field<i64>,
                    }
                }

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.Dot1x.Aaa", schema = Dict(287)))]
                pub struct Aaa<'a, Mode> {
                    #[data_view(schema = Dict(286))]
                    pub unresponsive: ::validated_data::Field<aaa::Unresponsive<'a, Mode>>,
                }

                pub mod aaa {

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.Dot1x.Aaa.Unresponsive", schema = Dict(286)))]
                    pub struct Unresponsive<'a, Mode> {
                        #[data_view(schema = Str(256))]
                        pub eap_response: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Dict(285))]
                        pub action: ::validated_data::Field<unresponsive::Action<'a, Mode>>,
                        #[data_view(schema = Dict(153))]
                        pub phone_action: ::validated_data::Field<super::super::super::super::super::super::super::eos_config::dot1x::aaa::unresponsive::PhoneAction<'a, Mode>>,
                    }

                    pub mod unresponsive {

                        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.Dot1x.Aaa.Unresponsive.Action", schema = Dict(285)))]
                        pub struct Action<'a, Mode> {
                            #[data_view(schema = Str(257))]
                            pub traffic_allow_access_list: ::validated_data::Field<&'a str>,
                            #[data_view(schema = Bool(47))]
                            pub apply_alternate: ::validated_data::Field<bool>,
                            #[data_view(schema = Int(26))]
                            pub traffic_allow_vlan: ::validated_data::Field<i64>,
                            #[data_view(schema = Bool(48))]
                            pub apply_cached_results: ::validated_data::Field<bool>,
                            #[data_view(schema = Dict(151))]
                            pub cached_results_timeout: ::validated_data::Field<action::CachedResultsTimeout<'a, Mode>>,
                            #[data_view(schema = Bool(49))]
                            pub traffic_allow: ::validated_data::Field<bool>,
                        }

                        pub mod action {

                            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.Dot1x.Aaa.Unresponsive.Action.CachedResultsTimeout", schema = Dict(151)))]
                            pub struct CachedResultsTimeout<'a, Mode> {
                                #[data_view(schema = Int(27))]
                                pub time_duration: ::validated_data::RequiredValue<i64, Mode>,
                                #[data_view(schema = Str(155))]
                                pub time_duration_unit: ::validated_data::RequiredValue<&'a str, Mode>,
                            }
                        }
                    }
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.AddressLocking", schema = Dict(1422)))]
            pub struct AddressLocking<'a, Mode> {
                #[data_view(schema = Bool(434))]
                pub ipv4: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(435))]
                pub ipv6: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.StormControl", schema = Dict(1425)))]
            pub struct StormControl<'a, Mode> {
                #[data_view(schema = Dict(1424))]
                pub all: ::validated_data::Field<storm_control::All<'a, Mode>>,
                #[data_view(schema = Dict(1424))]
                pub broadcast: ::validated_data::Field<storm_control::Broadcast<'a, Mode>>,
                #[data_view(schema = Dict(1424))]
                pub multicast: ::validated_data::Field<storm_control::Multicast<'a, Mode>>,
                #[data_view(schema = Dict(1424))]
                pub unknown_unicast: ::validated_data::Field<storm_control::UnknownUnicast<'a, Mode>>,
            }

            pub mod storm_control {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.StormControl.All", schema = Dict(1424)))]
                pub struct All<'a, Mode> {
                    #[data_view(schema = Str(247))]
                    pub level: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1158))]
                    pub unit: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.StormControl.Broadcast", schema = Dict(1424)))]
                pub struct Broadcast<'a, Mode> {
                    #[data_view(schema = Str(247))]
                    pub level: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1158))]
                    pub unit: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.StormControl.Multicast", schema = Dict(1424)))]
                pub struct Multicast<'a, Mode> {
                    #[data_view(schema = Str(247))]
                    pub level: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1158))]
                    pub unit: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.StormControl.UnknownUnicast", schema = Dict(1424)))]
                pub struct UnknownUnicast<'a, Mode> {
                    #[data_view(schema = Str(247))]
                    pub level: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1158))]
                    pub unit: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.MonitorSessions", schema = List(572)))]
            pub struct MonitorSessions<'a, Mode> (::validated_data::Field<monitor_sessions::Item<'a, Mode>>);

            pub mod monitor_sessions {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.MonitorSessionsItem", schema = Dict(1430)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(1159))]
                    pub name: ::validated_data::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Str(1160))]
                    pub role: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Dict(1427))]
                    pub source_settings: ::validated_data::Field<item::SourceSettings<'a, Mode>>,
                    #[data_view(schema = Dict(1429))]
                    pub session_settings: ::validated_data::Field<item::SessionSettings<'a, Mode>>,
                }

                pub mod item {

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.MonitorSessionsItem.SourceSettings", schema = Dict(1427)))]
                    pub struct SourceSettings<'a, Mode> {
                        #[data_view(schema = Str(593))]
                        pub direction: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Dict(1426))]
                        pub access_group: ::validated_data::Field<source_settings::AccessGroup<'a, Mode>>,
                    }

                    pub mod source_settings {

                        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.MonitorSessionsItem.SourceSettings.AccessGroup", schema = Dict(1426)))]
                        pub struct AccessGroup<'a, Mode> {
                            #[data_view(schema = Str(594), rename = "type")]
                            pub field_type: ::validated_data::Field<&'a str>,
                            #[data_view(schema = Str(1161))]
                            pub name: ::validated_data::Field<&'a str>,
                            #[data_view(schema = Int(77))]
                            pub priority: ::validated_data::Field<i64>,
                        }
                    }

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.MonitorSessionsItem.SessionSettings", schema = Dict(1429)))]
                    pub struct SessionSettings<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub encapsulation_gre_metadata_tx: ::validated_data::Field<bool>,
                        #[data_view(schema = Int(273))]
                        pub header_remove_size: ::validated_data::Field<i64>,
                        #[data_view(schema = Dict(1428))]
                        pub access_group: ::validated_data::Field<session_settings::AccessGroup<'a, Mode>>,
                        #[data_view(schema = Str(597))]
                        pub rate_limit_per_ingress_chip: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Str(597))]
                        pub rate_limit_per_egress_chip: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Int(77))]
                        pub sample: ::validated_data::Field<i64>,
                        #[data_view(schema = Dict(666))]
                        pub truncate: ::validated_data::Field<session_settings::Truncate<'a, Mode>>,
                    }

                    pub mod session_settings {

                        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.MonitorSessionsItem.SessionSettings.AccessGroup", schema = Dict(1428)))]
                        pub struct AccessGroup<'a, Mode> {
                            #[data_view(schema = Str(594), rename = "type")]
                            pub field_type: ::validated_data::Field<&'a str>,
                            #[data_view(schema = Str(1161))]
                            pub name: ::validated_data::Field<&'a str>,
                        }

                        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.MonitorSessionsItem.SessionSettings.Truncate", schema = Dict(666)))]
                        pub struct Truncate<'a, Mode> {
                            #[data_view(schema = Bool(7))]
                            pub enabled: ::validated_data::Field<bool>,
                            #[data_view(schema = Int(274))]
                            pub size: ::validated_data::Field<i64>,
                        }
                    }
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.EthernetSegment", schema = Dict(1431)))]
            pub struct EthernetSegment<'a, Mode> {
                #[data_view(schema = Str(1162))]
                pub short_esi: ::validated_data::RequiredValue<&'a str, Mode>,
                #[data_view(schema = Str(1163))]
                pub redundancy: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(1164))]
                pub designated_forwarder_algorithm: ::validated_data::Field<&'a str>,
                #[data_view(schema = List(573))]
                pub designated_forwarder_preferences: ::validated_data::Field<ethernet_segment::DesignatedForwarderPreferences<'a, Mode>>,
                #[data_view(schema = Bool(436))]
                pub dont_preempt: ::validated_data::Field<bool>,
            }

            pub mod ethernet_segment {

                #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.EthernetSegment.DesignatedForwarderPreferences", schema = List(573)))]
                pub struct DesignatedForwarderPreferences<'a, Mode> (::validated_data::Field<i64>);
            }

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.ConnectedEndpointsItemBase.AdaptersItem.CampusLinkType", schema = List(574)))]
            pub struct CampusLinkType<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }
}
