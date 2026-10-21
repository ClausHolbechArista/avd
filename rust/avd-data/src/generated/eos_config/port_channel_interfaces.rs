// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct PortChannelInterfaces<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view]
pub struct Item<'a, Mode> {
    pub name: ::data_views::Field<&'a str>,
    pub comment: ::data_views::Field<&'a str>,
    pub description: ::data_views::Field<&'a str>,
    pub profile: ::data_views::Field<&'a str>,
    pub logging: ::data_views::Field<item::Logging<'a, Mode>>,
    pub shutdown: ::data_views::Field<bool>,
    pub l2_mtu: ::data_views::Field<i64>,
    pub l2_mru: ::data_views::Field<i64>,
    pub loop_protection: ::data_views::Field<bool>,
    pub arp_gratuitous_accept: ::data_views::Field<bool>,
    pub snmp_trap_link_change: ::data_views::Field<bool>,
    pub address_locking: ::data_views::Field<item::AddressLocking<'a, Mode>>,
    pub encapsulation_dot1q: ::data_views::Field<item::EncapsulationDot1q<'a, Mode>>,
    pub vrf: ::data_views::Field<&'a str>,
    pub encapsulation_vlan: ::data_views::Field<item::EncapsulationVlan<'a, Mode>>,
    pub vlan_id: ::data_views::Field<i64>,
    pub link_tracking_groups: ::data_views::Field<item::LinkTrackingGroups<'a, Mode>>,
    pub link_tracking: ::data_views::Field<item::LinkTracking<'a, Mode>>,
    pub l2_protocol: ::data_views::Field<item::L2Protocol<'a, Mode>>,
    pub mtu: ::data_views::Field<i64>,
    pub mlag: ::data_views::Field<i64>,
    pub lacp_fallback_timeout: ::data_views::Field<i64>,
    pub min_links: ::data_views::Field<i64>,
    pub lacp_fallback_mode: ::data_views::Field<&'a str>,
    pub qos: ::data_views::Field<item::Qos<'a, Mode>>,
    pub bfd: ::data_views::Field<item::Bfd<'a, Mode>>,
    pub service_policy: ::data_views::Field<item::ServicePolicy<'a, Mode>>,
    pub cpu_traffic_policy_fallback_vrf: ::data_views::Field<&'a str>,
    pub mpls: ::data_views::Field<item::Mpls<'a, Mode>>,
    pub ntp_serve: ::data_views::Field<bool>,
    pub shape: ::data_views::Field<item::Shape<'a, Mode>>,
    pub storm_control: ::data_views::Field<item::StormControl<'a, Mode>>,
    pub ip_proxy_arp: ::data_views::Field<bool>,
    pub isis_enable: ::data_views::Field<&'a str>,
    pub isis_bfd: ::data_views::Field<bool>,
    pub isis_passive: ::data_views::Field<bool>,
    pub isis_metric: ::data_views::Field<i64>,
    pub isis_network_point_to_point: ::data_views::Field<bool>,
    pub isis_circuit_type: ::data_views::Field<&'a str>,
    pub isis_hello_padding: ::data_views::Field<bool>,
    pub isis_authentication: ::data_views::Field<item::IsisAuthentication<'a, Mode>>,
    pub traffic_policy: ::data_views::Field<item::TrafficPolicy<'a, Mode>>,
    pub evpn_ethernet_segment: ::data_views::Field<item::EvpnEthernetSegment<'a, Mode>>,
    pub lacp_id: ::data_views::Field<&'a str>,
    pub spanning_tree_bpdufilter: ::data_views::Field<&'a str>,
    pub spanning_tree_bpduguard: ::data_views::Field<&'a str>,
    pub spanning_tree_guard: ::data_views::Field<&'a str>,
    pub spanning_tree_portfast: ::data_views::Field<&'a str>,
    pub spanning_tree_link_type: ::data_views::Field<&'a str>,
    pub vmtracer: ::data_views::Field<bool>,
    pub ptp: ::data_views::Field<item::Ptp<'a, Mode>>,
    pub ip_address: ::data_views::Field<&'a str>,
    pub ip_address_secondaries: ::data_views::Field<item::IpAddressSecondaries<'a, Mode>>,
    pub dhcp_client_accept_default_route: ::data_views::Field<bool>,
    pub dhcp_server_ipv4: ::data_views::Field<bool>,
    pub dhcp_server_ipv6: ::data_views::Field<bool>,
    pub ip_verify_unicast_source_reachable_via: ::data_views::Field<&'a str>,
    pub ip_nat: ::data_views::Field<item::IpNat<'a, Mode>>,
    pub ipv6_enable: ::data_views::Field<bool>,
    pub ipv6_address: ::data_views::Field<&'a str>,
    pub ipv6_addresses: ::data_views::Field<item::Ipv6Addresses<'a, Mode>>,
    pub ipv6_address_auto_config: ::data_views::Field<bool>,
    pub ipv6_address_link_local: ::data_views::Field<&'a str>,
    pub ipv6_nd_ra_disabled: ::data_views::Field<bool>,
    pub ipv6_nd_managed_config_flag: ::data_views::Field<bool>,
    pub ipv6_nd_prefixes: ::data_views::Field<item::Ipv6NdPrefixes<'a, Mode>>,
    pub ipv6_nd: ::data_views::Field<item::Ipv6Nd<'a, Mode>>,
    pub access_group_in: ::data_views::Field<&'a str>,
    pub access_group_out: ::data_views::Field<&'a str>,
    pub ipv6_access_group_in: ::data_views::Field<&'a str>,
    pub ipv6_access_group_out: ::data_views::Field<&'a str>,
    pub mac_access_group_in: ::data_views::Field<&'a str>,
    pub mac_access_group_out: ::data_views::Field<&'a str>,
    pub pim: ::data_views::Field<item::Pim<'a, Mode>>,
    pub multicast: ::data_views::Field<item::Multicast<'a, Mode>>,
    pub service_profile: ::data_views::Field<&'a str>,
    pub ospf_network_point_to_point: ::data_views::Field<bool>,
    pub ospf_area: ::data_views::Field<&'a str>,
    pub ospf_cost: ::data_views::Field<i64>,
    pub ospf_authentication: ::data_views::Field<&'a str>,
    pub ospf_authentication_key: ::data_views::Field<&'a str>,
    pub ospf_authentication_key_type: ::data_views::Field<&'a str>,
    pub ospf_message_digest_keys: ::data_views::Field<item::OspfMessageDigestKeys<'a, Mode>>,
    pub flow_tracker: ::data_views::Field<item::FlowTracker<'a, Mode>>,
    pub bgp: ::data_views::Field<item::Bgp<'a, Mode>>,
    pub ip_igmp_host_proxy: ::data_views::Field<item::IpIgmpHostProxy<'a, Mode>>,
    pub metadata: ::data_views::Field<item::Metadata<'a, Mode>>,
    pub sflow: ::data_views::Field<item::Sflow<'a, Mode>>,
    pub vrrp_ids: ::data_views::Field<item::VrrpIds<'a, Mode>>,
    pub switchport: ::data_views::Field<item::Switchport<'a, Mode>>,
    pub traffic_engineering: ::data_views::Field<item::TrafficEngineering<'a, Mode>>,
    pub eos_cli: ::data_views::Field<&'a str>,
}

pub mod item {

    #[::data_views::data_view]
    pub struct Logging<'a, Mode> {
        pub event: ::data_views::Field<logging::Event<'a, Mode>>,
    }

    pub mod logging {

        #[::data_views::data_view]
        pub struct Event<'a, Mode> {
            pub link_status: ::data_views::Field<bool>,
            pub storm_control_discards: ::data_views::Field<bool>,
        }
    }

    #[::data_views::data_view]
    pub struct AddressLocking<'a, Mode> {
        pub address_family: ::data_views::Field<address_locking::AddressFamily<'a, Mode>>,
    }

    pub mod address_locking {

        #[::data_views::data_view]
        pub struct AddressFamily<'a, Mode> {
            pub ipv4: ::data_views::Field<bool>,
            pub ipv6: ::data_views::Field<bool>,
        }
    }

    #[::data_views::data_view]
    pub struct EncapsulationDot1q<'a, Mode> {
        pub vlan: ::data_views::RequiredValue<i64, Mode>,
        pub inner_vlan: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct EncapsulationVlan<'a, Mode> {
        pub client: ::data_views::Field<encapsulation_vlan::Client<'a, Mode>>,
        pub network: ::data_views::Field<encapsulation_vlan::Network<'a, Mode>>,
    }

    pub mod encapsulation_vlan {

        #[::data_views::data_view]
        pub struct Client<'a, Mode> {
            pub encapsulation: ::data_views::RequiredValue<&'a str, Mode>,
            pub vlan: ::data_views::Field<i64>,
            pub outer_vlan: ::data_views::Field<i64>,
            pub inner_vlan: ::data_views::Field<i64>,
            pub inner_encapsulation: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct Network<'a, Mode> {
            pub encapsulation: ::data_views::RequiredValue<&'a str, Mode>,
            pub vlan: ::data_views::Field<i64>,
            pub outer_vlan: ::data_views::Field<i64>,
            pub inner_vlan: ::data_views::Field<i64>,
            pub inner_encapsulation: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct LinkTrackingGroups<'a, Mode> (::data_views::Field<link_tracking_groups::Item<'a, Mode>>);

    pub mod link_tracking_groups {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub direction: ::data_views::RequiredValue<&'a str, Mode>,
        }
    }

    #[::data_views::data_view]
    pub struct LinkTracking<'a, Mode> {
        pub direction: ::data_views::RequiredValue<&'a str, Mode>,
        pub groups: ::data_views::RequiredValue<link_tracking::Groups<'a, Mode>, Mode>,
    }

    pub mod link_tracking {

        #[::data_views::data_view(list)]
        pub struct Groups<'a, Mode> (::data_views::Field<&'a str>);
    }

    #[::data_views::data_view]
    pub struct L2Protocol<'a, Mode> {
        pub encapsulation_dot1q_vlan: ::data_views::Field<i64>,
        pub forwarding_profile: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view]
    pub struct Qos<'a, Mode> {
        pub trust: ::data_views::Field<&'a str>,
        pub dscp: ::data_views::Field<i64>,
        pub cos: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct Bfd<'a, Mode> {
        pub echo: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
        pub min_rx: ::data_views::Field<i64>,
        pub multiplier: ::data_views::Field<i64>,
        pub neighbor: ::data_views::Field<&'a str>,
        pub per_link: ::data_views::Field<bfd::PerLink<'a, Mode>>,
    }

    pub mod bfd {

        #[::data_views::data_view]
        pub struct PerLink<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub rfc_7130: ::data_views::Field<bool>,
        }
    }

    #[::data_views::data_view]
    pub struct ServicePolicy<'a, Mode> {
        pub pbr: ::data_views::Field<service_policy::Pbr<'a, Mode>>,
        pub qos: ::data_views::Field<service_policy::Qos<'a, Mode>>,
    }

    pub mod service_policy {

        #[::data_views::data_view]
        pub struct Pbr<'a, Mode> {
            pub input: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct Qos<'a, Mode> {
            pub input: ::data_views::RequiredValue<&'a str, Mode>,
        }
    }

    #[::data_views::data_view]
    pub struct Mpls<'a, Mode> {
        pub ip: ::data_views::Field<bool>,
        pub ldp: ::data_views::Field<mpls::Ldp<'a, Mode>>,
    }

    pub mod mpls {

        #[::data_views::data_view]
        pub struct Ldp<'a, Mode> {
            pub interface: ::data_views::Field<bool>,
            pub igp_sync: ::data_views::Field<bool>,
        }
    }

    #[::data_views::data_view]
    pub struct Shape<'a, Mode> {
        pub rate: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view]
    pub struct StormControl<'a, Mode> {
        pub all: ::data_views::Field<storm_control::All<'a, Mode>>,
        pub broadcast: ::data_views::Field<storm_control::Broadcast<'a, Mode>>,
        pub multicast: ::data_views::Field<storm_control::Multicast<'a, Mode>>,
        pub unknown_unicast: ::data_views::Field<storm_control::UnknownUnicast<'a, Mode>>,
    }

    pub mod storm_control {

        #[::data_views::data_view]
        pub struct All<'a, Mode> {
            pub level: ::data_views::Field<&'a str>,
            pub unit: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct Broadcast<'a, Mode> {
            pub level: ::data_views::Field<&'a str>,
            pub unit: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct Multicast<'a, Mode> {
            pub level: ::data_views::Field<&'a str>,
            pub unit: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct UnknownUnicast<'a, Mode> {
            pub level: ::data_views::Field<&'a str>,
            pub unit: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view]
    pub struct IsisAuthentication<'a, Mode> {
        pub both: ::data_views::Field<isis_authentication::Both<'a, Mode>>,
        pub level_1: ::data_views::Field<isis_authentication::Level1<'a, Mode>>,
        pub level_2: ::data_views::Field<isis_authentication::Level2<'a, Mode>>,
    }

    pub mod isis_authentication {

        #[::data_views::data_view]
        pub struct Both<'a, Mode> {
            pub key_type: ::data_views::Field<&'a str>,
            pub key: ::data_views::Field<&'a str>,
            pub key_ids: ::data_views::Field<both::KeyIds<'a, Mode>>,
            pub mode: ::data_views::Field<&'a str>,
            pub sha: ::data_views::Field<both::Sha<'a, Mode>>,
            pub shared_secret: ::data_views::Field<both::SharedSecret<'a, Mode>>,
            pub rx_disabled: ::data_views::Field<bool>,
        }

        pub mod both {

            #[::data_views::data_view(indexed_list, primary_key(id))]
            pub struct KeyIds<'a, Mode> (::data_views::Field<key_ids::Item<'a, Mode>>);

            pub mod key_ids {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub id: ::data_views::RequiredValue<i64, Mode>,
                    pub algorithm: ::data_views::RequiredValue<&'a str, Mode>,
                    pub key_type: ::data_views::RequiredValue<&'a str, Mode>,
                    pub key: ::data_views::RequiredValue<&'a str, Mode>,
                    pub rfc_5310: ::data_views::Field<bool>,
                }
            }

            #[::data_views::data_view]
            pub struct Sha<'a, Mode> {
                pub key_id: ::data_views::RequiredValue<i64, Mode>,
            }

            #[::data_views::data_view]
            pub struct SharedSecret<'a, Mode> {
                pub profile: ::data_views::RequiredValue<&'a str, Mode>,
                pub algorithm: ::data_views::RequiredValue<&'a str, Mode>,
            }
        }

        #[::data_views::data_view]
        pub struct Level1<'a, Mode> {
            pub key_type: ::data_views::Field<&'a str>,
            pub key: ::data_views::Field<&'a str>,
            pub key_ids: ::data_views::Field<level_1::KeyIds<'a, Mode>>,
            pub mode: ::data_views::Field<&'a str>,
            pub sha: ::data_views::Field<level_1::Sha<'a, Mode>>,
            pub shared_secret: ::data_views::Field<level_1::SharedSecret<'a, Mode>>,
            pub rx_disabled: ::data_views::Field<bool>,
        }

        pub mod level_1 {

            #[::data_views::data_view(indexed_list, primary_key(id))]
            pub struct KeyIds<'a, Mode> (::data_views::Field<key_ids::Item<'a, Mode>>);

            pub mod key_ids {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub id: ::data_views::RequiredValue<i64, Mode>,
                    pub algorithm: ::data_views::RequiredValue<&'a str, Mode>,
                    pub key_type: ::data_views::RequiredValue<&'a str, Mode>,
                    pub key: ::data_views::RequiredValue<&'a str, Mode>,
                    pub rfc_5310: ::data_views::Field<bool>,
                }
            }

            #[::data_views::data_view]
            pub struct Sha<'a, Mode> {
                pub key_id: ::data_views::RequiredValue<i64, Mode>,
            }

            #[::data_views::data_view]
            pub struct SharedSecret<'a, Mode> {
                pub profile: ::data_views::RequiredValue<&'a str, Mode>,
                pub algorithm: ::data_views::RequiredValue<&'a str, Mode>,
            }
        }

        #[::data_views::data_view]
        pub struct Level2<'a, Mode> {
            pub key_type: ::data_views::Field<&'a str>,
            pub key: ::data_views::Field<&'a str>,
            pub key_ids: ::data_views::Field<level_2::KeyIds<'a, Mode>>,
            pub mode: ::data_views::Field<&'a str>,
            pub sha: ::data_views::Field<level_2::Sha<'a, Mode>>,
            pub shared_secret: ::data_views::Field<level_2::SharedSecret<'a, Mode>>,
            pub rx_disabled: ::data_views::Field<bool>,
        }

        pub mod level_2 {

            #[::data_views::data_view(indexed_list, primary_key(id))]
            pub struct KeyIds<'a, Mode> (::data_views::Field<key_ids::Item<'a, Mode>>);

            pub mod key_ids {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub id: ::data_views::RequiredValue<i64, Mode>,
                    pub algorithm: ::data_views::RequiredValue<&'a str, Mode>,
                    pub key_type: ::data_views::RequiredValue<&'a str, Mode>,
                    pub key: ::data_views::RequiredValue<&'a str, Mode>,
                    pub rfc_5310: ::data_views::Field<bool>,
                }
            }

            #[::data_views::data_view]
            pub struct Sha<'a, Mode> {
                pub key_id: ::data_views::RequiredValue<i64, Mode>,
            }

            #[::data_views::data_view]
            pub struct SharedSecret<'a, Mode> {
                pub profile: ::data_views::RequiredValue<&'a str, Mode>,
                pub algorithm: ::data_views::RequiredValue<&'a str, Mode>,
            }
        }
    }

    #[::data_views::data_view]
    pub struct TrafficPolicy<'a, Mode> {
        pub input: ::data_views::Field<&'a str>,
        pub output: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view]
    pub struct EvpnEthernetSegment<'a, Mode> {
        pub identifier: ::data_views::Field<&'a str>,
        pub redundancy: ::data_views::Field<&'a str>,
        pub designated_forwarder_election: ::data_views::Field<evpn_ethernet_segment::DesignatedForwarderElection<'a, Mode>>,
        pub mpls: ::data_views::Field<evpn_ethernet_segment::Mpls<'a, Mode>>,
        pub route_target: ::data_views::Field<&'a str>,
    }

    pub mod evpn_ethernet_segment {

        #[::data_views::data_view]
        pub struct DesignatedForwarderElection<'a, Mode> {
            pub algorithm: ::data_views::Field<&'a str>,
            pub preference_value: ::data_views::Field<i64>,
            pub dont_preempt: ::data_views::Field<bool>,
            pub hold_time: ::data_views::Field<i64>,
            pub subsequent_hold_time: ::data_views::Field<i64>,
            pub candidate_reachability_required: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct Mpls<'a, Mode> {
            pub shared_index: ::data_views::Field<i64>,
            pub tunnel_flood_filter_time: ::data_views::Field<i64>,
        }
    }

    #[::data_views::data_view]
    pub struct Ptp<'a, Mode> {
        pub enable: ::data_views::Field<bool>,
        pub announce: ::data_views::Field<ptp::Announce<'a, Mode>>,
        pub delay_req: ::data_views::Field<i64>,
        pub delay_mechanism: ::data_views::Field<&'a str>,
        pub profile: ::data_views::Field<ptp::Profile<'a, Mode>>,
        pub region: ::data_views::Field<ptp::Region<'a, Mode>>,
        pub sync_message: ::data_views::Field<ptp::SyncMessage<'a, Mode>>,
        pub role: ::data_views::Field<&'a str>,
        pub vlan: ::data_views::Field<&'a str>,
        pub transport: ::data_views::Field<&'a str>,
        pub mpass: ::data_views::Field<bool>,
        pub management: ::data_views::Field<ptp::Management<'a, Mode>>,
    }

    pub mod ptp {

        #[::data_views::data_view]
        pub struct Announce<'a, Mode> {
            pub interval: ::data_views::Field<i64>,
            pub timeout: ::data_views::Field<i64>,
        }

        #[::data_views::data_view]
        pub struct Profile<'a, Mode> {
            pub g8275_1: ::data_views::Field<profile::G82751<'a, Mode>>,
        }

        pub mod profile {

            #[::data_views::data_view]
            pub struct G82751<'a, Mode> {
                pub destination_mac_address: ::data_views::Field<&'a str>,
            }
        }

        #[::data_views::data_view]
        pub struct Region<'a, Mode> {
            pub domain_number: ::data_views::Field<i64>,
        }

        #[::data_views::data_view]
        pub struct SyncMessage<'a, Mode> {
            pub interval: ::data_views::Field<i64>,
        }

        #[::data_views::data_view]
        pub struct Management<'a, Mode> {
            pub drop: ::data_views::Field<bool>,
        }
    }

    #[::data_views::data_view(list)]
    pub struct IpAddressSecondaries<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view]
    pub struct IpNat<'a, Mode> {
        pub service_profile: ::data_views::Field<&'a str>,
        pub destination: ::data_views::Field<ip_nat::Destination<'a, Mode>>,
        pub source: ::data_views::Field<ip_nat::Source<'a, Mode>>,
    }

    pub mod ip_nat {

        #[::data_views::data_view]
        pub struct Destination<'a, Mode> {
            pub dynamic: ::data_views::Field<destination::Dynamic<'a, Mode>>,
            #[data_view(rename = "static")]
            pub field_static: ::data_views::Field<destination::FieldStatic<'a, Mode>>,
        }

        pub mod destination {

            #[::data_views::data_view(indexed_list, primary_key(access_list))]
            pub struct Dynamic<'a, Mode> (::data_views::Field<dynamic::Item<'a, Mode>>);

            pub mod dynamic {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub access_list: ::data_views::Field<&'a str>,
                    pub comment: ::data_views::Field<&'a str>,
                    pub pool_name: ::data_views::RequiredValue<&'a str, Mode>,
                    pub priority: ::data_views::Field<i64>,
                }
            }

            #[::data_views::data_view(list)]
            pub struct FieldStatic<'a, Mode> (::data_views::Field<field_static::Item<'a, Mode>>);

            pub mod field_static {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub access_list: ::data_views::Field<&'a str>,
                    pub comment: ::data_views::Field<&'a str>,
                    pub direction: ::data_views::Field<&'a str>,
                    pub group: ::data_views::Field<i64>,
                    pub original_ip: ::data_views::Field<&'a str>,
                    pub original_port: ::data_views::Field<i64>,
                    pub priority: ::data_views::Field<i64>,
                    pub protocol: ::data_views::Field<&'a str>,
                    pub translated_ip: ::data_views::RequiredValue<&'a str, Mode>,
                    pub translated_port: ::data_views::Field<i64>,
                }
            }
        }

        #[::data_views::data_view]
        pub struct Source<'a, Mode> {
            pub dynamic: ::data_views::Field<source::Dynamic<'a, Mode>>,
            #[data_view(rename = "static")]
            pub field_static: ::data_views::Field<source::FieldStatic<'a, Mode>>,
        }

        pub mod source {

            #[::data_views::data_view(indexed_list, primary_key(access_list))]
            pub struct Dynamic<'a, Mode> (::data_views::Field<dynamic::Item<'a, Mode>>);

            pub mod dynamic {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub access_list: ::data_views::Field<&'a str>,
                    pub comment: ::data_views::Field<&'a str>,
                    pub nat_type: ::data_views::RequiredValue<&'a str, Mode>,
                    pub pool_name: ::data_views::Field<&'a str>,
                    pub priority: ::data_views::Field<i64>,
                }
            }

            #[::data_views::data_view(list)]
            pub struct FieldStatic<'a, Mode> (::data_views::Field<field_static::Item<'a, Mode>>);

            pub mod field_static {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub access_list: ::data_views::Field<&'a str>,
                    pub comment: ::data_views::Field<&'a str>,
                    pub direction: ::data_views::Field<&'a str>,
                    pub group: ::data_views::Field<i64>,
                    pub original_ip: ::data_views::Field<&'a str>,
                    pub original_port: ::data_views::Field<i64>,
                    pub priority: ::data_views::Field<i64>,
                    pub protocol: ::data_views::Field<&'a str>,
                    pub translated_ip: ::data_views::RequiredValue<&'a str, Mode>,
                    pub translated_port: ::data_views::Field<i64>,
                }
            }
        }
    }

    #[::data_views::data_view(list)]
    pub struct Ipv6Addresses<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(indexed_list, primary_key(ipv6_prefix))]
    pub struct Ipv6NdPrefixes<'a, Mode> (::data_views::Field<ipv6_nd_prefixes::Item<'a, Mode>>);

    pub mod ipv6_nd_prefixes {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub ipv6_prefix: ::data_views::Field<&'a str>,
            pub valid_lifetime: ::data_views::Field<&'a str>,
            pub preferred_lifetime: ::data_views::Field<&'a str>,
            pub no_autoconfig_flag: ::data_views::Field<bool>,
        }
    }

    #[::data_views::data_view]
    pub struct Ipv6Nd<'a, Mode> {
        pub cache: ::data_views::Field<ipv6_nd::Cache<'a, Mode>>,
        pub ra: ::data_views::Field<ipv6_nd::Ra<'a, Mode>>,
        pub managed_config_flag: ::data_views::Field<bool>,
        pub prefixes: ::data_views::Field<ipv6_nd::Prefixes<'a, Mode>>,
        pub other_config_flag: ::data_views::Field<bool>,
    }

    pub mod ipv6_nd {

        #[::data_views::data_view]
        pub struct Cache<'a, Mode> {
            pub dynamic_capacity: ::data_views::Field<i64>,
            pub expire: ::data_views::Field<i64>,
            pub refresh_always: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct Ra<'a, Mode> {
            pub disabled: ::data_views::Field<bool>,
            pub rx_accept: ::data_views::Field<ra::RxAccept<'a, Mode>>,
            pub dns_servers: ::data_views::Field<ra::DnsServers<'a, Mode>>,
            pub dns_servers_lifetime: ::data_views::Field<i64>,
        }

        pub mod ra {

            #[::data_views::data_view]
            pub struct RxAccept<'a, Mode> {
                pub default_route: ::data_views::Field<bool>,
                pub route_preference: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(indexed_list, primary_key(address))]
            pub struct DnsServers<'a, Mode> (::data_views::Field<dns_servers::Item<'a, Mode>>);

            pub mod dns_servers {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub address: ::data_views::Field<&'a str>,
                    pub lifetime: ::data_views::Field<i64>,
                }
            }
        }

        #[::data_views::data_view(indexed_list, primary_key(ipv6_prefix))]
        pub struct Prefixes<'a, Mode> (::data_views::Field<prefixes::Item<'a, Mode>>);

        pub mod prefixes {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub ipv6_prefix: ::data_views::Field<&'a str>,
                pub valid_lifetime: ::data_views::Field<&'a str>,
                pub preferred_lifetime: ::data_views::Field<&'a str>,
                pub no_autoconfig_flag: ::data_views::Field<bool>,
            }
        }
    }

    #[::data_views::data_view]
    pub struct Pim<'a, Mode> {
        pub ipv4: ::data_views::Field<pim::Ipv4<'a, Mode>>,
    }

    pub mod pim {

        #[::data_views::data_view]
        pub struct Ipv4<'a, Mode> {
            pub border_router: ::data_views::Field<bool>,
            pub dr_priority: ::data_views::Field<i64>,
            pub sparse_mode: ::data_views::Field<bool>,
            pub bfd: ::data_views::Field<bool>,
            pub bidirectional: ::data_views::Field<bool>,
            pub neighbor_filter: ::data_views::Field<&'a str>,
            pub hello: ::data_views::Field<ipv4::Hello<'a, Mode>>,
        }

        pub mod ipv4 {

            #[::data_views::data_view]
            pub struct Hello<'a, Mode> {
                pub count: ::data_views::Field<&'a str>,
                pub interval: ::data_views::Field<i64>,
            }
        }
    }

    #[::data_views::data_view]
    pub struct Multicast<'a, Mode> {
        pub ipv4: ::data_views::Field<multicast::Ipv4<'a, Mode>>,
        pub ipv6: ::data_views::Field<multicast::Ipv6<'a, Mode>>,
    }

    pub mod multicast {

        #[::data_views::data_view]
        pub struct Ipv4<'a, Mode> {
            pub boundaries: ::data_views::Field<ipv4::Boundaries<'a, Mode>>,
            #[data_view(rename = "static")]
            pub field_static: ::data_views::Field<bool>,
        }

        pub mod ipv4 {

            #[::data_views::data_view(indexed_list, primary_key(boundary))]
            pub struct Boundaries<'a, Mode> (::data_views::Field<boundaries::Item<'a, Mode>>);

            pub mod boundaries {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub boundary: ::data_views::Field<&'a str>,
                    pub out: ::data_views::Field<bool>,
                }
            }
        }

        #[::data_views::data_view]
        pub struct Ipv6<'a, Mode> {
            pub boundaries: ::data_views::Field<ipv6::Boundaries<'a, Mode>>,
            #[data_view(rename = "static")]
            pub field_static: ::data_views::Field<bool>,
        }

        pub mod ipv6 {

            #[::data_views::data_view(indexed_list, primary_key(boundary))]
            pub struct Boundaries<'a, Mode> (::data_views::Field<boundaries::Item<'a, Mode>>);

            pub mod boundaries {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub boundary: ::data_views::Field<&'a str>,
                }
            }
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(id))]
    pub struct OspfMessageDigestKeys<'a, Mode> (::data_views::Field<ospf_message_digest_keys::Item<'a, Mode>>);

    pub mod ospf_message_digest_keys {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub id: ::data_views::Field<i64>,
            pub hash_algorithm: ::data_views::Field<&'a str>,
            pub key: ::data_views::Field<&'a str>,
            pub key_type: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view]
    pub struct FlowTracker<'a, Mode> {
        pub sampled: ::data_views::Field<&'a str>,
        pub hardware: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view]
    pub struct Bgp<'a, Mode> {
        pub session_tracker: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view]
    pub struct IpIgmpHostProxy<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub groups: ::data_views::Field<ip_igmp_host_proxy::Groups<'a, Mode>>,
        pub report_interval: ::data_views::Field<i64>,
        pub access_lists: ::data_views::Field<ip_igmp_host_proxy::AccessLists<'a, Mode>>,
        pub version: ::data_views::Field<i64>,
    }

    pub mod ip_igmp_host_proxy {

        #[::data_views::data_view(indexed_list, primary_key(group))]
        pub struct Groups<'a, Mode> (::data_views::Field<groups::Item<'a, Mode>>);

        pub mod groups {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub group: ::data_views::Field<&'a str>,
                pub exclude: ::data_views::Field<item::Exclude<'a, Mode>>,
                pub include: ::data_views::Field<item::Include<'a, Mode>>,
            }

            pub mod item {

                #[::data_views::data_view(indexed_list, primary_key(source))]
                pub struct Exclude<'a, Mode> (::data_views::Field<exclude::Item<'a, Mode>>);

                pub mod exclude {

                    #[::data_views::data_view]
                    pub struct Item<'a, Mode> {
                        pub source: ::data_views::Field<&'a str>,
                    }
                }

                #[::data_views::data_view(indexed_list, primary_key(source))]
                pub struct Include<'a, Mode> (::data_views::Field<include::Item<'a, Mode>>);

                pub mod include {

                    #[::data_views::data_view]
                    pub struct Item<'a, Mode> {
                        pub source: ::data_views::Field<&'a str>,
                    }
                }
            }
        }

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct AccessLists<'a, Mode> (::data_views::Field<access_lists::Item<'a, Mode>>);

        pub mod access_lists {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
            }
        }
    }

    #[::data_views::data_view]
    pub struct Metadata<'a, Mode> {
        pub peer: ::data_views::Field<&'a str>,
        pub peer_interface: ::data_views::Field<&'a str>,
        pub peer_type: ::data_views::Field<&'a str>,
        pub peer_key: ::data_views::Field<&'a str>,
        pub validate_state: ::data_views::Field<bool>,
        pub validate_lldp: ::data_views::Field<bool>,
    }

    #[::data_views::data_view]
    pub struct Sflow<'a, Mode> {
        pub enable: ::data_views::Field<bool>,
        pub egress: ::data_views::Field<sflow::Egress<'a, Mode>>,
    }

    pub mod sflow {

        #[::data_views::data_view]
        pub struct Egress<'a, Mode> {
            pub enable: ::data_views::Field<bool>,
            pub unmodified_enable: ::data_views::Field<bool>,
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(id))]
    pub struct VrrpIds<'a, Mode> (::data_views::Field<vrrp_ids::Item<'a, Mode>>);

    pub mod vrrp_ids {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub id: ::data_views::Field<i64>,
            pub priority_level: ::data_views::Field<i64>,
            pub advertisement: ::data_views::Field<item::Advertisement<'a, Mode>>,
            pub preempt: ::data_views::Field<item::Preempt<'a, Mode>>,
            pub timers: ::data_views::Field<item::Timers<'a, Mode>>,
            pub tracked_object: ::data_views::Field<item::TrackedObject<'a, Mode>>,
            pub ipv4: ::data_views::Field<item::Ipv4<'a, Mode>>,
            pub ipv6: ::data_views::Field<item::Ipv6<'a, Mode>>,
            pub peer_authentication: ::data_views::Field<item::PeerAuthentication<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct Advertisement<'a, Mode> {
                pub interval: ::data_views::Field<i64>,
            }

            #[::data_views::data_view]
            pub struct Preempt<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub delay: ::data_views::Field<preempt::Delay<'a, Mode>>,
            }

            pub mod preempt {

                #[::data_views::data_view]
                pub struct Delay<'a, Mode> {
                    pub minimum: ::data_views::Field<i64>,
                    pub reload: ::data_views::Field<i64>,
                }
            }

            #[::data_views::data_view]
            pub struct Timers<'a, Mode> {
                pub delay: ::data_views::Field<timers::Delay<'a, Mode>>,
            }

            pub mod timers {

                #[::data_views::data_view]
                pub struct Delay<'a, Mode> {
                    pub reload: ::data_views::Field<i64>,
                }
            }

            #[::data_views::data_view(indexed_list, primary_key(name))]
            pub struct TrackedObject<'a, Mode> (::data_views::Field<tracked_object::Item<'a, Mode>>);

            pub mod tracked_object {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub name: ::data_views::Field<&'a str>,
                    pub decrement: ::data_views::Field<i64>,
                    pub shutdown: ::data_views::Field<bool>,
                }
            }

            #[::data_views::data_view]
            pub struct Ipv4<'a, Mode> {
                pub address: ::data_views::RequiredValue<&'a str, Mode>,
                pub secondary_addresses: ::data_views::Field<ipv4::SecondaryAddresses<'a, Mode>>,
                pub version: ::data_views::Field<i64>,
            }

            pub mod ipv4 {

                #[::data_views::data_view(list)]
                pub struct SecondaryAddresses<'a, Mode> (::data_views::Field<&'a str>);
            }

            #[::data_views::data_view]
            pub struct Ipv6<'a, Mode> {
                pub addresses: ::data_views::RequiredValue<ipv6::Addresses<'a, Mode>, Mode>,
            }

            pub mod ipv6 {

                #[::data_views::data_view(list)]
                pub struct Addresses<'a, Mode> (::data_views::Field<&'a str>);
            }

            #[::data_views::data_view]
            pub struct PeerAuthentication<'a, Mode> {
                pub mode: ::data_views::RequiredValue<&'a str, Mode>,
                pub key: ::data_views::RequiredValue<&'a str, Mode>,
                pub key_type: ::data_views::Field<&'a str>,
            }
        }
    }

    #[::data_views::data_view]
    pub struct Switchport<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub mode: ::data_views::Field<&'a str>,
        pub access_vlan: ::data_views::Field<i64>,
        pub trunk: ::data_views::Field<switchport::Trunk<'a, Mode>>,
        pub phone: ::data_views::Field<switchport::Phone<'a, Mode>>,
        pub pvlan_mapping: ::data_views::Field<&'a str>,
        pub dot1q: ::data_views::Field<switchport::Dot1q<'a, Mode>>,
        pub source_interface: ::data_views::Field<&'a str>,
        pub vlan_translations: ::data_views::Field<switchport::VlanTranslations<'a, Mode>>,
        pub vlan_forwarding_accept_all: ::data_views::Field<bool>,
        pub backup_link: ::data_views::Field<switchport::BackupLink<'a, Mode>>,
        pub backup: ::data_views::Field<switchport::Backup<'a, Mode>>,
        pub port_security: ::data_views::Field<switchport::PortSecurity<'a, Mode>>,
        pub tap: ::data_views::Field<switchport::Tap<'a, Mode>>,
        pub tool: ::data_views::Field<switchport::Tool<'a, Mode>>,
    }

    pub mod switchport {

        #[::data_views::data_view]
        pub struct Trunk<'a, Mode> {
            pub allowed_vlan: ::data_views::Field<&'a str>,
            pub native_vlan: ::data_views::Field<i64>,
            pub native_vlan_tag: ::data_views::Field<bool>,
            pub private_vlan_secondary: ::data_views::Field<bool>,
            pub groups: ::data_views::Field<trunk::Groups<'a, Mode>>,
        }

        pub mod trunk {

            #[::data_views::data_view(list)]
            pub struct Groups<'a, Mode> (::data_views::Field<&'a str>);
        }

        #[::data_views::data_view]
        pub struct Phone<'a, Mode> {
            pub vlan: ::data_views::Field<i64>,
            pub trunk: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct Dot1q<'a, Mode> {
            pub ethertype: ::data_views::Field<i64>,
            pub vlan_tag: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct VlanTranslations<'a, Mode> {
            pub in_required: ::data_views::Field<bool>,
            pub out_required: ::data_views::Field<bool>,
            pub direction_in: ::data_views::Field<vlan_translations::DirectionIn<'a, Mode>>,
            pub direction_out: ::data_views::Field<vlan_translations::DirectionOut<'a, Mode>>,
            pub direction_both: ::data_views::Field<vlan_translations::DirectionBoth<'a, Mode>>,
        }

        pub mod vlan_translations {

            #[::data_views::data_view(list)]
            pub struct DirectionIn<'a, Mode> (::data_views::Field<direction_in::Item<'a, Mode>>);

            pub mod direction_in {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    #[data_view(rename = "from")]
                    pub field_from: ::data_views::Field<&'a str>,
                    pub to: ::data_views::Field<i64>,
                    pub dot1q_tunnel: ::data_views::Field<bool>,
                    pub inner_vlan_from: ::data_views::Field<i64>,
                }
            }

            #[::data_views::data_view(list)]
            pub struct DirectionOut<'a, Mode> (::data_views::Field<direction_out::Item<'a, Mode>>);

            pub mod direction_out {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    #[data_view(rename = "from")]
                    pub field_from: ::data_views::RequiredValue<&'a str, Mode>,
                    pub to: ::data_views::Field<i64>,
                    pub dot1q_tunnel_to: ::data_views::Field<&'a str>,
                    pub inner_vlan_to: ::data_views::Field<i64>,
                }
            }

            #[::data_views::data_view(list)]
            pub struct DirectionBoth<'a, Mode> (::data_views::Field<direction_both::Item<'a, Mode>>);

            pub mod direction_both {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    #[data_view(rename = "from")]
                    pub field_from: ::data_views::RequiredValue<&'a str, Mode>,
                    pub to: ::data_views::RequiredValue<i64, Mode>,
                    pub dot1q_tunnel: ::data_views::Field<bool>,
                    pub inner_vlan_from: ::data_views::Field<i64>,
                    pub network: ::data_views::Field<bool>,
                }
            }
        }

        #[::data_views::data_view]
        pub struct BackupLink<'a, Mode> {
            pub interface: ::data_views::RequiredValue<&'a str, Mode>,
            pub prefer_vlan: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct Backup<'a, Mode> {
            pub dest_macaddr: ::data_views::Field<&'a str>,
            pub initial_mac_move_delay: ::data_views::Field<i64>,
            pub mac_move_burst: ::data_views::Field<i64>,
            pub mac_move_burst_interval: ::data_views::Field<i64>,
            pub preemption_delay: ::data_views::Field<i64>,
        }

        #[::data_views::data_view]
        pub struct PortSecurity<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub mac_address_maximum: ::data_views::Field<port_security::MacAddressMaximum<'a, Mode>>,
            pub violation: ::data_views::Field<port_security::Violation<'a, Mode>>,
            pub vlan_default_mac_address_maximum: ::data_views::Field<i64>,
            pub vlans: ::data_views::Field<port_security::Vlans<'a, Mode>>,
        }

        pub mod port_security {

            #[::data_views::data_view]
            pub struct MacAddressMaximum<'a, Mode> {
                pub disabled: ::data_views::Field<bool>,
                pub limit: ::data_views::Field<i64>,
            }

            #[::data_views::data_view]
            pub struct Violation<'a, Mode> {
                pub mode: ::data_views::Field<&'a str>,
                pub protect_log: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(indexed_list, primary_key(range))]
            pub struct Vlans<'a, Mode> (::data_views::Field<vlans::Item<'a, Mode>>);

            pub mod vlans {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub range: ::data_views::Field<&'a str>,
                    pub mac_address_maximum: ::data_views::RequiredValue<i64, Mode>,
                }
            }
        }

        #[::data_views::data_view]
        pub struct Tap<'a, Mode> {
            pub allowed_vlan: ::data_views::Field<&'a str>,
            pub default: ::data_views::Field<tap::Default<'a, Mode>>,
            pub identity: ::data_views::Field<tap::Identity<'a, Mode>>,
            pub mpls_pop_all: ::data_views::Field<bool>,
            pub native_vlan: ::data_views::Field<i64>,
            pub truncation: ::data_views::Field<tap::Truncation<'a, Mode>>,
            pub mac_address: ::data_views::Field<tap::MacAddress<'a, Mode>>,
            pub encapsulation: ::data_views::Field<tap::Encapsulation<'a, Mode>>,
        }

        pub mod tap {

            #[::data_views::data_view]
            pub struct Default<'a, Mode> {
                pub groups: ::data_views::Field<default::Groups<'a, Mode>>,
                pub interfaces: ::data_views::Field<default::Interfaces<'a, Mode>>,
                pub nexthop_groups: ::data_views::Field<default::NexthopGroups<'a, Mode>>,
            }

            pub mod default {

                #[::data_views::data_view(list)]
                pub struct Groups<'a, Mode> (::data_views::Field<&'a str>);

                #[::data_views::data_view(list)]
                pub struct Interfaces<'a, Mode> (::data_views::Field<&'a str>);

                #[::data_views::data_view(list)]
                pub struct NexthopGroups<'a, Mode> (::data_views::Field<&'a str>);
            }

            #[::data_views::data_view]
            pub struct Identity<'a, Mode> {
                pub id: ::data_views::Field<i64>,
                pub inner_vlan: ::data_views::Field<i64>,
            }

            #[::data_views::data_view]
            pub struct Truncation<'a, Mode> {
                pub enabled: ::data_views::Field<bool>,
                pub size: ::data_views::Field<i64>,
            }

            #[::data_views::data_view]
            pub struct MacAddress<'a, Mode> {
                pub source: ::data_views::Field<&'a str>,
                pub destination: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct Encapsulation<'a, Mode> {
                pub vxlan_strip: ::data_views::Field<bool>,
                pub gre: ::data_views::Field<encapsulation::Gre<'a, Mode>>,
            }

            pub mod encapsulation {

                #[::data_views::data_view]
                pub struct Gre<'a, Mode> {
                    pub strip: ::data_views::Field<bool>,
                    pub protocols: ::data_views::Field<gre::Protocols<'a, Mode>>,
                    pub destinations: ::data_views::Field<gre::Destinations<'a, Mode>>,
                }

                pub mod gre {

                    #[::data_views::data_view(indexed_list, primary_key(protocol))]
                    pub struct Protocols<'a, Mode> (::data_views::Field<protocols::Item<'a, Mode>>);

                    pub mod protocols {

                        #[::data_views::data_view]
                        pub struct Item<'a, Mode> {
                            pub protocol: ::data_views::Field<&'a str>,
                            pub strip: ::data_views::Field<bool>,
                            pub feature_header_length: ::data_views::Field<i64>,
                            pub re_encapsulation_ethernet_header: ::data_views::Field<bool>,
                        }
                    }

                    #[::data_views::data_view(indexed_list, primary_key(destination))]
                    pub struct Destinations<'a, Mode> (::data_views::Field<destinations::Item<'a, Mode>>);

                    pub mod destinations {

                        #[::data_views::data_view]
                        pub struct Item<'a, Mode> {
                            pub destination: ::data_views::Field<&'a str>,
                            pub source: ::data_views::Field<&'a str>,
                            pub strip: ::data_views::Field<bool>,
                            pub protocols: ::data_views::Field<item::Protocols<'a, Mode>>,
                        }

                        pub mod item {

                            #[::data_views::data_view(indexed_list, primary_key(protocol))]
                            pub struct Protocols<'a, Mode> (::data_views::Field<protocols::Item<'a, Mode>>);

                            pub mod protocols {

                                #[::data_views::data_view]
                                pub struct Item<'a, Mode> {
                                    pub protocol: ::data_views::Field<&'a str>,
                                    pub strip: ::data_views::Field<bool>,
                                    pub feature_header_length: ::data_views::Field<i64>,
                                    pub re_encapsulation_ethernet_header: ::data_views::Field<bool>,
                                }
                            }
                        }
                    }
                }
            }
        }

        #[::data_views::data_view]
        pub struct Tool<'a, Mode> {
            pub mpls_pop_all: ::data_views::Field<bool>,
            pub encapsulation: ::data_views::Field<tool::Encapsulation<'a, Mode>>,
            pub allowed_vlan: ::data_views::Field<&'a str>,
            pub identity: ::data_views::Field<tool::Identity<'a, Mode>>,
            pub groups: ::data_views::Field<tool::Groups<'a, Mode>>,
            pub dot1q_remove_outer_vlan_tag: ::data_views::Field<&'a str>,
        }

        pub mod tool {

            #[::data_views::data_view]
            pub struct Encapsulation<'a, Mode> {
                pub dot1br_strip: ::data_views::Field<bool>,
                pub vn_tag_strip: ::data_views::Field<bool>,
            }

            #[::data_views::data_view]
            pub struct Identity<'a, Mode> {
                pub tag: ::data_views::Field<&'a str>,
                pub dot1q_dzgre_source: ::data_views::Field<&'a str>,
                pub qinq_dzgre_source: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view(list)]
            pub struct Groups<'a, Mode> (::data_views::Field<&'a str>);
        }
    }

    #[::data_views::data_view]
    pub struct TrafficEngineering<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub administrative_groups: ::data_views::Field<traffic_engineering::AdministrativeGroups<'a, Mode>>,
        pub srlgs: ::data_views::Field<traffic_engineering::Srlgs<'a, Mode>>,
        pub metric: ::data_views::Field<i64>,
        pub bandwidth: ::data_views::Field<traffic_engineering::Bandwidth<'a, Mode>>,
        pub min_delay_static: ::data_views::Field<traffic_engineering::MinDelayStatic<'a, Mode>>,
        pub min_delay_dynamic: ::data_views::Field<traffic_engineering::MinDelayDynamic<'a, Mode>>,
    }

    pub mod traffic_engineering {

        #[::data_views::data_view(list)]
        pub struct AdministrativeGroups<'a, Mode> (::data_views::Field<&'a str>);

        #[::data_views::data_view(list)]
        pub struct Srlgs<'a, Mode> (::data_views::Field<&'a str>);

        #[::data_views::data_view]
        pub struct Bandwidth<'a, Mode> {
            pub number: ::data_views::RequiredValue<i64, Mode>,
            pub unit: ::data_views::RequiredValue<&'a str, Mode>,
        }

        #[::data_views::data_view]
        pub struct MinDelayStatic<'a, Mode> {
            pub number: ::data_views::RequiredValue<i64, Mode>,
            pub unit: ::data_views::RequiredValue<&'a str, Mode>,
        }

        #[::data_views::data_view]
        pub struct MinDelayDynamic<'a, Mode> {
            pub twamp_light_fallback: ::data_views::Field<min_delay_dynamic::TwampLightFallback<'a, Mode>>,
        }

        pub mod min_delay_dynamic {

            #[::data_views::data_view]
            pub struct TwampLightFallback<'a, Mode> {
                pub number: ::data_views::RequiredValue<i64, Mode>,
                pub unit: ::data_views::RequiredValue<&'a str, Mode>,
            }
        }
    }
}
