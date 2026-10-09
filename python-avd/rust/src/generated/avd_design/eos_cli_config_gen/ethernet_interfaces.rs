// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub comment: ::validated_data::Field<&'a str>,
    pub description: ::validated_data::Field<&'a str>,
    pub shutdown: ::validated_data::Field<bool>,
    pub load_interval: ::validated_data::Field<i64>,
    pub speed: ::validated_data::Field<&'a str>,
    pub mtu: ::validated_data::Field<i64>,
    pub l2_mtu: ::validated_data::Field<i64>,
    pub l2_mru: ::validated_data::Field<i64>,
    pub loop_protection: ::validated_data::Field<bool>,
    pub arp_gratuitous_accept: ::validated_data::Field<bool>,
    pub l2_protocol: ::validated_data::Field<item::L2Protocol<'a, Mode>>,
    pub mac_timestamp: ::validated_data::Field<&'a str>,
    pub snmp_trap_link_change: ::validated_data::Field<bool>,
    pub address_locking: ::validated_data::Field<item::AddressLocking<'a, Mode>>,
    pub flowcontrol: ::validated_data::Field<item::Flowcontrol<'a, Mode>>,
    pub vrf: ::validated_data::Field<&'a str>,
    pub flow_tracker: ::validated_data::Field<item::FlowTracker<'a, Mode>>,
    pub error_correction_encoding: ::validated_data::Field<item::ErrorCorrectionEncoding<'a, Mode>>,
    pub link_tracking_groups: ::validated_data::Field<item::LinkTrackingGroups<'a, Mode>>,
    pub link_tracking: ::validated_data::Field<item::LinkTracking<'a, Mode>>,
    pub evpn_ethernet_segment: ::validated_data::Field<item::EvpnEthernetSegment<'a, Mode>>,
    pub encapsulation_dot1q: ::validated_data::Field<item::EncapsulationDot1q<'a, Mode>>,
    pub encapsulation_vlan: ::validated_data::Field<item::EncapsulationVlan<'a, Mode>>,
    pub vlan_id: ::validated_data::Field<i64>,
    pub ip_address: ::validated_data::Field<&'a str>,
    pub ip_address_secondaries: ::validated_data::Field<item::IpAddressSecondaries<'a, Mode>>,
    pub ip_verify_unicast_source_reachable_via: ::validated_data::Field<&'a str>,
    pub dhcp_client_accept_default_route: ::validated_data::Field<bool>,
    pub dhcp_server_ipv4: ::validated_data::Field<bool>,
    pub dhcp_server_ipv6: ::validated_data::Field<bool>,
    pub ip_helpers: ::validated_data::Field<item::IpHelpers<'a, Mode>>,
    pub ip_nat: ::validated_data::Field<item::IpNat<'a, Mode>>,
    pub ipv6_enable: ::validated_data::Field<bool>,
    pub ipv6_address: ::validated_data::Field<&'a str>,
    pub ipv6_addresses: ::validated_data::Field<item::Ipv6Addresses<'a, Mode>>,
    pub ipv6_address_auto_config: ::validated_data::Field<bool>,
    pub ipv6_address_link_local: ::validated_data::Field<&'a str>,
    pub ipv6_nd_ra_disabled: ::validated_data::Field<bool>,
    pub ipv6_nd_managed_config_flag: ::validated_data::Field<bool>,
    pub ipv6_nd_prefixes: ::validated_data::Field<item::Ipv6NdPrefixes<'a, Mode>>,
    pub ipv6_nd: ::validated_data::Field<item::Ipv6Nd<'a, Mode>>,
    pub ipv6_dhcp_relay_destinations: ::validated_data::Field<item::Ipv6DhcpRelayDestinations<'a, Mode>>,
    pub access_group_in: ::validated_data::Field<&'a str>,
    pub access_group_out: ::validated_data::Field<&'a str>,
    pub ipv6_access_group_in: ::validated_data::Field<&'a str>,
    pub ipv6_access_group_out: ::validated_data::Field<&'a str>,
    pub mac_access_group_in: ::validated_data::Field<&'a str>,
    pub mac_access_group_out: ::validated_data::Field<&'a str>,
    pub multicast: ::validated_data::Field<item::Multicast<'a, Mode>>,
    pub ospf_network_point_to_point: ::validated_data::Field<bool>,
    pub ospf_area: ::validated_data::Field<&'a str>,
    pub ospf_cost: ::validated_data::Field<i64>,
    pub ospf_authentication: ::validated_data::Field<&'a str>,
    pub ospf_authentication_key: ::validated_data::Field<&'a str>,
    pub ospf_authentication_key_type: ::validated_data::Field<&'a str>,
    pub ospf_message_digest_keys: ::validated_data::Field<item::OspfMessageDigestKeys<'a, Mode>>,
    pub pim: ::validated_data::Field<item::Pim<'a, Mode>>,
    pub mac_security: ::validated_data::Field<item::MacSecurity<'a, Mode>>,
    pub ntp_serve: ::validated_data::Field<bool>,
    pub tcp_mss_ceiling: ::validated_data::Field<item::TcpMssCeiling<'a, Mode>>,
    pub channel_group: ::validated_data::Field<item::ChannelGroup<'a, Mode>>,
    pub isis_enable: ::validated_data::Field<&'a str>,
    pub isis_bfd: ::validated_data::Field<bool>,
    pub isis_passive: ::validated_data::Field<bool>,
    pub isis_metric: ::validated_data::Field<i64>,
    pub isis_network_point_to_point: ::validated_data::Field<bool>,
    pub isis_circuit_type: ::validated_data::Field<&'a str>,
    pub isis_hello_padding: ::validated_data::Field<bool>,
    pub isis_authentication: ::validated_data::Field<item::IsisAuthentication<'a, Mode>>,
    pub poe: ::validated_data::Field<item::Poe<'a, Mode>>,
    pub ptp: ::validated_data::Field<item::Ptp<'a, Mode>>,
    pub profile: ::validated_data::Field<&'a str>,
    pub storm_control: ::validated_data::Field<item::StormControl<'a, Mode>>,
    pub logging: ::validated_data::Field<item::Logging<'a, Mode>>,
    pub lldp: ::validated_data::Field<item::Lldp<'a, Mode>>,
    pub dot1x: ::validated_data::Field<item::Dot1x<'a, Mode>>,
    pub service_profile: ::validated_data::Field<&'a str>,
    pub shape: ::validated_data::Field<item::Shape<'a, Mode>>,
    pub qos: ::validated_data::Field<item::Qos<'a, Mode>>,
    pub spanning_tree_bpdufilter: ::validated_data::Field<&'a str>,
    pub spanning_tree_bpduguard: ::validated_data::Field<&'a str>,
    pub spanning_tree_bpduguard_rate_limit: ::validated_data::Field<item::SpanningTreeBpduguardRateLimit<'a, Mode>>,
    pub spanning_tree_guard: ::validated_data::Field<&'a str>,
    pub spanning_tree_portfast: ::validated_data::Field<&'a str>,
    pub spanning_tree_link_type: ::validated_data::Field<&'a str>,
    pub vmtracer: ::validated_data::Field<bool>,
    pub priority_flow_control: ::validated_data::Field<item::PriorityFlowControl<'a, Mode>>,
    pub bfd: ::validated_data::Field<item::Bfd<'a, Mode>>,
    pub service_policy: ::validated_data::Field<item::ServicePolicy<'a, Mode>>,
    pub cpu_traffic_policy_fallback_vrf: ::validated_data::Field<&'a str>,
    pub mpls: ::validated_data::Field<item::Mpls<'a, Mode>>,
    pub lacp_timer: ::validated_data::Field<item::LacpTimer<'a, Mode>>,
    pub lacp_port_priority: ::validated_data::Field<i64>,
    pub transceiver: ::validated_data::Field<item::Transceiver<'a, Mode>>,
    pub ip_proxy_arp: ::validated_data::Field<bool>,
    pub traffic_policy: ::validated_data::Field<item::TrafficPolicy<'a, Mode>>,
    pub bgp: ::validated_data::Field<item::Bgp<'a, Mode>>,
    pub ip_igmp_host_proxy: ::validated_data::Field<item::IpIgmpHostProxy<'a, Mode>>,
    pub metadata: ::validated_data::Field<item::Metadata<'a, Mode>>,
    pub sflow: ::validated_data::Field<item::Sflow<'a, Mode>>,
    pub sync_e: ::validated_data::Field<item::SyncE<'a, Mode>>,
    pub uc_tx_queues: ::validated_data::Field<item::UcTxQueues<'a, Mode>>,
    pub tx_queues: ::validated_data::Field<item::TxQueues<'a, Mode>>,
    pub vrrp_ids: ::validated_data::Field<item::VrrpIds<'a, Mode>>,
    pub switchport: ::validated_data::Field<item::Switchport<'a, Mode>>,
    pub traffic_engineering: ::validated_data::Field<item::TrafficEngineering<'a, Mode>>,
    pub monitor_link_flap_profiles: ::validated_data::Field<item::MonitorLinkFlapProfiles<'a, Mode>>,
    pub eos_cli: ::validated_data::Field<&'a str>,
}

pub mod item {

    #[::validated_data::data_view]
    pub struct L2Protocol<'a, Mode> {
        pub encapsulation_dot1q_vlan: ::validated_data::Field<i64>,
        pub forwarding_profile: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct AddressLocking<'a, Mode> {
        pub ipv4: ::validated_data::Field<bool>,
        pub ipv6: ::validated_data::Field<bool>,
        pub address_family: ::validated_data::Field<address_locking::AddressFamily<'a, Mode>>,
        pub ipv4_enforcement_disabled: ::validated_data::Field<bool>,
    }

    pub mod address_locking {

        #[::validated_data::data_view]
        pub struct AddressFamily<'a, Mode> {
            pub ipv4: ::validated_data::Field<bool>,
            pub ipv6: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view]
    pub struct Flowcontrol<'a, Mode> {
        pub received: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct FlowTracker<'a, Mode> {
        pub sampled: ::validated_data::Field<&'a str>,
        pub hardware: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct ErrorCorrectionEncoding<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub fire_code: ::validated_data::Field<bool>,
        pub reed_solomon: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct LinkTrackingGroups<'a, Mode> (::validated_data::Field<link_tracking_groups::Item<'a, Mode>>);

    pub mod link_tracking_groups {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub direction: ::validated_data::RequiredValue<&'a str, Mode>,
        }
    }

    #[::validated_data::data_view]
    pub struct LinkTracking<'a, Mode> {
        pub direction: ::validated_data::RequiredValue<&'a str, Mode>,
        pub groups: ::validated_data::RequiredValue<link_tracking::Groups<'a, Mode>, Mode>,
    }

    pub mod link_tracking {

        #[::validated_data::data_view(list)]
        pub struct Groups<'a, Mode> (::validated_data::Field<&'a str>);
    }

    #[::validated_data::data_view]
    pub struct EvpnEthernetSegment<'a, Mode> {
        pub identifier: ::validated_data::Field<&'a str>,
        pub redundancy: ::validated_data::Field<&'a str>,
        pub designated_forwarder_election: ::validated_data::Field<evpn_ethernet_segment::DesignatedForwarderElection<'a, Mode>>,
        pub mpls: ::validated_data::Field<evpn_ethernet_segment::Mpls<'a, Mode>>,
        pub route_target: ::validated_data::Field<&'a str>,
    }

    pub mod evpn_ethernet_segment {

        #[::validated_data::data_view]
        pub struct DesignatedForwarderElection<'a, Mode> {
            pub algorithm: ::validated_data::Field<&'a str>,
            pub preference_value: ::validated_data::Field<i64>,
            pub dont_preempt: ::validated_data::Field<bool>,
            pub hold_time: ::validated_data::Field<i64>,
            pub subsequent_hold_time: ::validated_data::Field<i64>,
            pub candidate_reachability_required: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct Mpls<'a, Mode> {
            pub shared_index: ::validated_data::Field<i64>,
            pub tunnel_flood_filter_time: ::validated_data::Field<i64>,
        }
    }

    #[::validated_data::data_view]
    pub struct EncapsulationDot1q<'a, Mode> {
        pub vlan: ::validated_data::RequiredValue<i64, Mode>,
        pub inner_vlan: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct EncapsulationVlan<'a, Mode> {
        pub client: ::validated_data::Field<encapsulation_vlan::Client<'a, Mode>>,
        pub network: ::validated_data::Field<encapsulation_vlan::Network<'a, Mode>>,
    }

    pub mod encapsulation_vlan {

        #[::validated_data::data_view]
        pub struct Client<'a, Mode> {
            pub encapsulation: ::validated_data::RequiredValue<&'a str, Mode>,
            pub vlan: ::validated_data::Field<i64>,
            pub outer_vlan: ::validated_data::Field<i64>,
            pub inner_vlan: ::validated_data::Field<i64>,
            pub inner_encapsulation: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct Network<'a, Mode> {
            pub encapsulation: ::validated_data::RequiredValue<&'a str, Mode>,
            pub vlan: ::validated_data::Field<i64>,
            pub outer_vlan: ::validated_data::Field<i64>,
            pub inner_vlan: ::validated_data::Field<i64>,
            pub inner_encapsulation: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(list)]
    pub struct IpAddressSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(indexed_list, primary_key(ip_helper))]
    pub struct IpHelpers<'a, Mode> (::validated_data::Field<ip_helpers::Item<'a, Mode>>);

    pub mod ip_helpers {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub ip_helper: ::validated_data::Field<&'a str>,
            pub source_interface: ::validated_data::Field<&'a str>,
            pub vrf: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view]
    pub struct IpNat<'a, Mode> {
        pub service_profile: ::validated_data::Field<&'a str>,
        pub destination: ::validated_data::Field<ip_nat::Destination<'a, Mode>>,
        pub source: ::validated_data::Field<ip_nat::Source<'a, Mode>>,
    }

    pub mod ip_nat {

        #[::validated_data::data_view]
        pub struct Destination<'a, Mode> {
            pub dynamic: ::validated_data::Field<destination::Dynamic<'a, Mode>>,
            #[data_view(rename = "static")]
            pub field_static: ::validated_data::Field<destination::FieldStatic<'a, Mode>>,
        }

        pub mod destination {

            #[::validated_data::data_view(indexed_list, primary_key(access_list))]
            pub struct Dynamic<'a, Mode> (::validated_data::Field<dynamic::Item<'a, Mode>>);

            pub mod dynamic {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub access_list: ::validated_data::Field<&'a str>,
                    pub comment: ::validated_data::Field<&'a str>,
                    pub pool_name: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub priority: ::validated_data::Field<i64>,
                }
            }

            #[::validated_data::data_view(list)]
            pub struct FieldStatic<'a, Mode> (::validated_data::Field<field_static::Item<'a, Mode>>);

            pub mod field_static {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub access_list: ::validated_data::Field<&'a str>,
                    pub comment: ::validated_data::Field<&'a str>,
                    pub direction: ::validated_data::Field<&'a str>,
                    pub group: ::validated_data::Field<i64>,
                    pub original_ip: ::validated_data::Field<&'a str>,
                    pub original_port: ::validated_data::Field<i64>,
                    pub priority: ::validated_data::Field<i64>,
                    pub protocol: ::validated_data::Field<&'a str>,
                    pub translated_ip: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub translated_port: ::validated_data::Field<i64>,
                }
            }
        }

        #[::validated_data::data_view]
        pub struct Source<'a, Mode> {
            pub dynamic: ::validated_data::Field<source::Dynamic<'a, Mode>>,
            #[data_view(rename = "static")]
            pub field_static: ::validated_data::Field<source::FieldStatic<'a, Mode>>,
        }

        pub mod source {

            #[::validated_data::data_view(indexed_list, primary_key(access_list))]
            pub struct Dynamic<'a, Mode> (::validated_data::Field<dynamic::Item<'a, Mode>>);

            pub mod dynamic {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub access_list: ::validated_data::Field<&'a str>,
                    pub comment: ::validated_data::Field<&'a str>,
                    pub nat_type: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub pool_name: ::validated_data::Field<&'a str>,
                    pub priority: ::validated_data::Field<i64>,
                }
            }

            #[::validated_data::data_view(list)]
            pub struct FieldStatic<'a, Mode> (::validated_data::Field<field_static::Item<'a, Mode>>);

            pub mod field_static {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub access_list: ::validated_data::Field<&'a str>,
                    pub comment: ::validated_data::Field<&'a str>,
                    pub direction: ::validated_data::Field<&'a str>,
                    pub group: ::validated_data::Field<i64>,
                    pub original_ip: ::validated_data::Field<&'a str>,
                    pub original_port: ::validated_data::Field<i64>,
                    pub priority: ::validated_data::Field<i64>,
                    pub protocol: ::validated_data::Field<&'a str>,
                    pub translated_ip: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub translated_port: ::validated_data::Field<i64>,
                }
            }
        }
    }

    #[::validated_data::data_view(list)]
    pub struct Ipv6Addresses<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(indexed_list, primary_key(ipv6_prefix))]
    pub struct Ipv6NdPrefixes<'a, Mode> (::validated_data::Field<ipv6_nd_prefixes::Item<'a, Mode>>);

    pub mod ipv6_nd_prefixes {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub ipv6_prefix: ::validated_data::Field<&'a str>,
            pub valid_lifetime: ::validated_data::Field<&'a str>,
            pub preferred_lifetime: ::validated_data::Field<&'a str>,
            pub no_autoconfig_flag: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view]
    pub struct Ipv6Nd<'a, Mode> {
        pub cache: ::validated_data::Field<ipv6_nd::Cache<'a, Mode>>,
        pub ra: ::validated_data::Field<ipv6_nd::Ra<'a, Mode>>,
        pub managed_config_flag: ::validated_data::Field<bool>,
        pub prefixes: ::validated_data::Field<ipv6_nd::Prefixes<'a, Mode>>,
        pub other_config_flag: ::validated_data::Field<bool>,
    }

    pub mod ipv6_nd {

        #[::validated_data::data_view]
        pub struct Cache<'a, Mode> {
            pub dynamic_capacity: ::validated_data::Field<i64>,
            pub expire: ::validated_data::Field<i64>,
            pub refresh_always: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct Ra<'a, Mode> {
            pub disabled: ::validated_data::Field<bool>,
            pub rx_accept: ::validated_data::Field<ra::RxAccept<'a, Mode>>,
            pub dns_servers: ::validated_data::Field<ra::DnsServers<'a, Mode>>,
            pub dns_servers_lifetime: ::validated_data::Field<i64>,
        }

        pub mod ra {

            #[::validated_data::data_view]
            pub struct RxAccept<'a, Mode> {
                pub default_route: ::validated_data::Field<bool>,
                pub route_preference: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(indexed_list, primary_key(address))]
            pub struct DnsServers<'a, Mode> (::validated_data::Field<dns_servers::Item<'a, Mode>>);

            pub mod dns_servers {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub address: ::validated_data::Field<&'a str>,
                    pub lifetime: ::validated_data::Field<i64>,
                }
            }
        }

        #[::validated_data::data_view(indexed_list, primary_key(ipv6_prefix))]
        pub struct Prefixes<'a, Mode> (::validated_data::Field<prefixes::Item<'a, Mode>>);

        pub mod prefixes {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub ipv6_prefix: ::validated_data::Field<&'a str>,
                pub valid_lifetime: ::validated_data::Field<&'a str>,
                pub preferred_lifetime: ::validated_data::Field<&'a str>,
                pub no_autoconfig_flag: ::validated_data::Field<bool>,
            }
        }
    }

    #[::validated_data::data_view(list)]
    pub struct Ipv6DhcpRelayDestinations<'a, Mode> (::validated_data::Field<ipv6_dhcp_relay_destinations::Item<'a, Mode>>);

    pub mod ipv6_dhcp_relay_destinations {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub address: ::validated_data::RequiredValue<&'a str, Mode>,
            pub vrf: ::validated_data::Field<&'a str>,
            pub local_interface: ::validated_data::Field<&'a str>,
            pub source_address: ::validated_data::Field<&'a str>,
            pub link_address: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view]
    pub struct Multicast<'a, Mode> {
        pub ipv4: ::validated_data::Field<multicast::Ipv4<'a, Mode>>,
        pub ipv6: ::validated_data::Field<multicast::Ipv6<'a, Mode>>,
    }

    pub mod multicast {

        #[::validated_data::data_view]
        pub struct Ipv4<'a, Mode> {
            pub boundaries: ::validated_data::Field<ipv4::Boundaries<'a, Mode>>,
            #[data_view(rename = "static")]
            pub field_static: ::validated_data::Field<bool>,
        }

        pub mod ipv4 {

            #[::validated_data::data_view(list)]
            pub struct Boundaries<'a, Mode> (::validated_data::Field<boundaries::Item<'a, Mode>>);

            pub mod boundaries {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub boundary: ::validated_data::Field<&'a str>,
                    pub out: ::validated_data::Field<bool>,
                }
            }
        }

        #[::validated_data::data_view]
        pub struct Ipv6<'a, Mode> {
            pub boundaries: ::validated_data::Field<ipv6::Boundaries<'a, Mode>>,
            #[data_view(rename = "static")]
            pub field_static: ::validated_data::Field<bool>,
        }

        pub mod ipv6 {

            #[::validated_data::data_view(list)]
            pub struct Boundaries<'a, Mode> (::validated_data::Field<boundaries::Item<'a, Mode>>);

            pub mod boundaries {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub boundary: ::validated_data::Field<&'a str>,
                }
            }
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(id))]
    pub struct OspfMessageDigestKeys<'a, Mode> (::validated_data::Field<ospf_message_digest_keys::Item<'a, Mode>>);

    pub mod ospf_message_digest_keys {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub id: ::validated_data::Field<i64>,
            pub hash_algorithm: ::validated_data::Field<&'a str>,
            pub key: ::validated_data::Field<&'a str>,
            pub key_type: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view]
    pub struct Pim<'a, Mode> {
        pub ipv4: ::validated_data::Field<pim::Ipv4<'a, Mode>>,
    }

    pub mod pim {

        #[::validated_data::data_view]
        pub struct Ipv4<'a, Mode> {
            pub border_router: ::validated_data::Field<bool>,
            pub dr_priority: ::validated_data::Field<i64>,
            pub sparse_mode: ::validated_data::Field<bool>,
            pub bfd: ::validated_data::Field<bool>,
            pub bidirectional: ::validated_data::Field<bool>,
            pub neighbor_filter: ::validated_data::Field<&'a str>,
            pub hello: ::validated_data::Field<ipv4::Hello<'a, Mode>>,
        }

        pub mod ipv4 {

            #[::validated_data::data_view]
            pub struct Hello<'a, Mode> {
                pub count: ::validated_data::Field<&'a str>,
                pub interval: ::validated_data::Field<i64>,
            }
        }
    }

    #[::validated_data::data_view]
    pub struct MacSecurity<'a, Mode> {
        pub profile: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct TcpMssCeiling<'a, Mode> {
        pub ipv4: ::validated_data::Field<i64>,
        pub ipv6: ::validated_data::Field<i64>,
        pub direction: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct ChannelGroup<'a, Mode> {
        pub id: ::validated_data::Field<i64>,
        pub mode: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct IsisAuthentication<'a, Mode> {
        pub both: ::validated_data::Field<isis_authentication::Both<'a, Mode>>,
        pub level_1: ::validated_data::Field<isis_authentication::Level1<'a, Mode>>,
        pub level_2: ::validated_data::Field<isis_authentication::Level2<'a, Mode>>,
    }

    pub mod isis_authentication {

        #[::validated_data::data_view]
        pub struct Both<'a, Mode> {
            pub key_type: ::validated_data::Field<&'a str>,
            pub key: ::validated_data::Field<&'a str>,
            pub key_ids: ::validated_data::Field<both::KeyIds<'a, Mode>>,
            pub mode: ::validated_data::Field<&'a str>,
            pub sha: ::validated_data::Field<both::Sha<'a, Mode>>,
            pub shared_secret: ::validated_data::Field<both::SharedSecret<'a, Mode>>,
            pub rx_disabled: ::validated_data::Field<bool>,
        }

        pub mod both {

            #[::validated_data::data_view(indexed_list, primary_key(id))]
            pub struct KeyIds<'a, Mode> (::validated_data::Field<key_ids::Item<'a, Mode>>);

            pub mod key_ids {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub id: ::validated_data::RequiredValue<i64, Mode>,
                    pub algorithm: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub key_type: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub key: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub rfc_5310: ::validated_data::Field<bool>,
                }
            }

            #[::validated_data::data_view]
            pub struct Sha<'a, Mode> {
                pub key_id: ::validated_data::RequiredValue<i64, Mode>,
            }

            #[::validated_data::data_view]
            pub struct SharedSecret<'a, Mode> {
                pub profile: ::validated_data::RequiredValue<&'a str, Mode>,
                pub algorithm: ::validated_data::RequiredValue<&'a str, Mode>,
            }
        }

        #[::validated_data::data_view]
        pub struct Level1<'a, Mode> {
            pub key_type: ::validated_data::Field<&'a str>,
            pub key: ::validated_data::Field<&'a str>,
            pub key_ids: ::validated_data::Field<level_1::KeyIds<'a, Mode>>,
            pub mode: ::validated_data::Field<&'a str>,
            pub sha: ::validated_data::Field<level_1::Sha<'a, Mode>>,
            pub shared_secret: ::validated_data::Field<level_1::SharedSecret<'a, Mode>>,
            pub rx_disabled: ::validated_data::Field<bool>,
        }

        pub mod level_1 {

            #[::validated_data::data_view(indexed_list, primary_key(id))]
            pub struct KeyIds<'a, Mode> (::validated_data::Field<key_ids::Item<'a, Mode>>);

            pub mod key_ids {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub id: ::validated_data::RequiredValue<i64, Mode>,
                    pub algorithm: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub key_type: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub key: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub rfc_5310: ::validated_data::Field<bool>,
                }
            }

            #[::validated_data::data_view]
            pub struct Sha<'a, Mode> {
                pub key_id: ::validated_data::RequiredValue<i64, Mode>,
            }

            #[::validated_data::data_view]
            pub struct SharedSecret<'a, Mode> {
                pub profile: ::validated_data::RequiredValue<&'a str, Mode>,
                pub algorithm: ::validated_data::RequiredValue<&'a str, Mode>,
            }
        }

        #[::validated_data::data_view]
        pub struct Level2<'a, Mode> {
            pub key_type: ::validated_data::Field<&'a str>,
            pub key: ::validated_data::Field<&'a str>,
            pub key_ids: ::validated_data::Field<level_2::KeyIds<'a, Mode>>,
            pub mode: ::validated_data::Field<&'a str>,
            pub sha: ::validated_data::Field<level_2::Sha<'a, Mode>>,
            pub shared_secret: ::validated_data::Field<level_2::SharedSecret<'a, Mode>>,
            pub rx_disabled: ::validated_data::Field<bool>,
        }

        pub mod level_2 {

            #[::validated_data::data_view(indexed_list, primary_key(id))]
            pub struct KeyIds<'a, Mode> (::validated_data::Field<key_ids::Item<'a, Mode>>);

            pub mod key_ids {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub id: ::validated_data::RequiredValue<i64, Mode>,
                    pub algorithm: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub key_type: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub key: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub rfc_5310: ::validated_data::Field<bool>,
                }
            }

            #[::validated_data::data_view]
            pub struct Sha<'a, Mode> {
                pub key_id: ::validated_data::RequiredValue<i64, Mode>,
            }

            #[::validated_data::data_view]
            pub struct SharedSecret<'a, Mode> {
                pub profile: ::validated_data::RequiredValue<&'a str, Mode>,
                pub algorithm: ::validated_data::RequiredValue<&'a str, Mode>,
            }
        }
    }

    #[::validated_data::data_view]
    pub struct Poe<'a, Mode> {
        pub disabled: ::validated_data::Field<bool>,
        pub priority: ::validated_data::Field<&'a str>,
        pub reboot: ::validated_data::Field<poe::Reboot<'a, Mode>>,
        pub link_down: ::validated_data::Field<poe::LinkDown<'a, Mode>>,
        pub shutdown: ::validated_data::Field<poe::Shutdown<'a, Mode>>,
        pub limit: ::validated_data::Field<poe::Limit<'a, Mode>>,
        pub negotiation_lldp: ::validated_data::Field<bool>,
        pub legacy_detect: ::validated_data::Field<bool>,
    }

    pub mod poe {

        #[::validated_data::data_view]
        pub struct Reboot<'a, Mode> {
            pub action: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct LinkDown<'a, Mode> {
            pub action: ::validated_data::Field<&'a str>,
            pub power_off_delay: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view]
        pub struct Shutdown<'a, Mode> {
            pub action: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct Limit<'a, Mode> {
            #[data_view(rename = "class")]
            pub field_class: ::validated_data::Field<i64>,
            pub watts: ::validated_data::Field<&'a str>,
            pub fixed: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view]
    pub struct Ptp<'a, Mode> {
        pub enable: ::validated_data::Field<bool>,
        pub announce: ::validated_data::Field<ptp::Announce<'a, Mode>>,
        pub delay_req: ::validated_data::Field<i64>,
        pub delay_mechanism: ::validated_data::Field<&'a str>,
        pub profile: ::validated_data::Field<ptp::Profile<'a, Mode>>,
        pub region: ::validated_data::Field<ptp::Region<'a, Mode>>,
        pub sync_message: ::validated_data::Field<ptp::SyncMessage<'a, Mode>>,
        pub role: ::validated_data::Field<&'a str>,
        pub vlan: ::validated_data::Field<&'a str>,
        pub transport: ::validated_data::Field<&'a str>,
        pub management: ::validated_data::Field<ptp::Management<'a, Mode>>,
    }

    pub mod ptp {

        #[::validated_data::data_view]
        pub struct Announce<'a, Mode> {
            pub interval: ::validated_data::Field<i64>,
            pub timeout: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view]
        pub struct Profile<'a, Mode> {
            pub g8275_1: ::validated_data::Field<profile::G82751<'a, Mode>>,
        }

        pub mod profile {

            #[::validated_data::data_view]
            pub struct G82751<'a, Mode> {
                pub destination_mac_address: ::validated_data::Field<&'a str>,
            }
        }

        #[::validated_data::data_view]
        pub struct Region<'a, Mode> {
            pub domain_number: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view]
        pub struct SyncMessage<'a, Mode> {
            pub interval: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view]
        pub struct Management<'a, Mode> {
            pub drop: ::validated_data::Field<bool>,
        }
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

    #[::validated_data::data_view]
    pub struct Logging<'a, Mode> {
        pub event: ::validated_data::Field<logging::Event<'a, Mode>>,
    }

    pub mod logging {

        #[::validated_data::data_view]
        pub struct Event<'a, Mode> {
            pub link_status: ::validated_data::Field<bool>,
            pub congestion_drops: ::validated_data::Field<bool>,
            pub spanning_tree: ::validated_data::Field<bool>,
            pub storm_control_discards: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view]
    pub struct Lldp<'a, Mode> {
        pub transmit: ::validated_data::Field<bool>,
        pub receive: ::validated_data::Field<bool>,
        pub ztp_vlan: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Dot1x<'a, Mode> {
        pub port_control: ::validated_data::Field<&'a str>,
        pub port_control_force_authorized_phone: ::validated_data::Field<bool>,
        pub reauthentication: ::validated_data::Field<bool>,
        pub pae: ::validated_data::Field<dot1x::Pae<'a, Mode>>,
        pub authentication_failure: ::validated_data::Field<dot1x::AuthenticationFailure<'a, Mode>>,
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
        pub struct Pae<'a, Mode> {
            pub mode: ::validated_data::Field<&'a str>,
            pub supplicant_profile: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct AuthenticationFailure<'a, Mode> {
            pub action: ::validated_data::Field<&'a str>,
            pub allow_vlan: ::validated_data::Field<i64>,
            pub allow_access_list: ::validated_data::Field<&'a str>,
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
                pub phone_action: ::validated_data::Field<unresponsive::PhoneAction<'a, Mode>>,
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

                #[::validated_data::data_view]
                pub struct PhoneAction<'a, Mode> {
                    pub apply_cached_results: ::validated_data::Field<bool>,
                    pub cached_results_timeout: ::validated_data::Field<phone_action::CachedResultsTimeout<'a, Mode>>,
                    pub apply_alternate: ::validated_data::Field<bool>,
                    pub traffic_allow: ::validated_data::Field<bool>,
                }

                pub mod phone_action {

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
    pub struct Shape<'a, Mode> {
        pub rate: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct Qos<'a, Mode> {
        pub trust: ::validated_data::Field<&'a str>,
        pub dscp: ::validated_data::Field<i64>,
        pub cos: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct SpanningTreeBpduguardRateLimit<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub count: ::validated_data::Field<i64>,
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct PriorityFlowControl<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub priorities: ::validated_data::Field<priority_flow_control::Priorities<'a, Mode>>,
    }

    pub mod priority_flow_control {

        #[::validated_data::data_view(indexed_list, primary_key(priority))]
        pub struct Priorities<'a, Mode> (::validated_data::Field<priorities::Item<'a, Mode>>);

        pub mod priorities {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub priority: ::validated_data::Field<i64>,
                pub no_drop: ::validated_data::Field<bool>,
            }
        }
    }

    #[::validated_data::data_view]
    pub struct Bfd<'a, Mode> {
        pub echo: ::validated_data::Field<bool>,
        pub interval: ::validated_data::Field<i64>,
        pub min_rx: ::validated_data::Field<i64>,
        pub multiplier: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct ServicePolicy<'a, Mode> {
        pub pbr: ::validated_data::Field<service_policy::Pbr<'a, Mode>>,
        pub qos: ::validated_data::Field<service_policy::Qos<'a, Mode>>,
    }

    pub mod service_policy {

        #[::validated_data::data_view]
        pub struct Pbr<'a, Mode> {
            pub input: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct Qos<'a, Mode> {
            pub input: ::validated_data::RequiredValue<&'a str, Mode>,
        }
    }

    #[::validated_data::data_view]
    pub struct Mpls<'a, Mode> {
        pub ip: ::validated_data::Field<bool>,
        pub ldp: ::validated_data::Field<mpls::Ldp<'a, Mode>>,
    }

    pub mod mpls {

        #[::validated_data::data_view]
        pub struct Ldp<'a, Mode> {
            pub interface: ::validated_data::Field<bool>,
            pub igp_sync: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view]
    pub struct LacpTimer<'a, Mode> {
        pub mode: ::validated_data::Field<&'a str>,
        pub multiplier: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Transceiver<'a, Mode> {
        pub frequency: ::validated_data::Field<&'a str>,
        pub frequency_unit: ::validated_data::Field<&'a str>,
        pub media: ::validated_data::Field<transceiver::Media<'a, Mode>>,
        pub application_override: ::validated_data::Field<&'a str>,
        pub application_override_lanes: ::validated_data::Field<transceiver::ApplicationOverrideLanes<'a, Mode>>,
        pub power: ::validated_data::Field<transceiver::Power<'a, Mode>>,
        pub transmitter: ::validated_data::Field<transceiver::Transmitter<'a, Mode>>,
    }

    pub mod transceiver {

        #[::validated_data::data_view]
        pub struct Media<'a, Mode> {
            #[data_view(rename = "override")]
            pub field_override: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view(list)]
        pub struct ApplicationOverrideLanes<'a, Mode> (::validated_data::Field<application_override_lanes::Item<'a, Mode>>);

        pub mod application_override_lanes {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                #[data_view(rename = "override")]
                pub field_override: ::validated_data::RequiredValue<i64, Mode>,
                pub first_lane: ::validated_data::RequiredValue<i64, Mode>,
                pub last_lane: ::validated_data::Field<i64>,
            }
        }

        #[::validated_data::data_view]
        pub struct Power<'a, Mode> {
            pub ignore: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct Transmitter<'a, Mode> {
            pub signal_power: ::validated_data::Field<&'a str>,
            pub disabled: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view]
    pub struct TrafficPolicy<'a, Mode> {
        pub input: ::validated_data::Field<&'a str>,
        pub output: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct Bgp<'a, Mode> {
        pub session_tracker: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct IpIgmpHostProxy<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub groups: ::validated_data::Field<ip_igmp_host_proxy::Groups<'a, Mode>>,
        pub report_interval: ::validated_data::Field<i64>,
        pub access_lists: ::validated_data::Field<ip_igmp_host_proxy::AccessLists<'a, Mode>>,
        pub version: ::validated_data::Field<i64>,
    }

    pub mod ip_igmp_host_proxy {

        #[::validated_data::data_view(indexed_list, primary_key(group))]
        pub struct Groups<'a, Mode> (::validated_data::Field<groups::Item<'a, Mode>>);

        pub mod groups {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub group: ::validated_data::Field<&'a str>,
                pub exclude: ::validated_data::Field<item::Exclude<'a, Mode>>,
                pub include: ::validated_data::Field<item::Include<'a, Mode>>,
            }

            pub mod item {

                #[::validated_data::data_view(indexed_list, primary_key(source))]
                pub struct Exclude<'a, Mode> (::validated_data::Field<exclude::Item<'a, Mode>>);

                pub mod exclude {

                    #[::validated_data::data_view]
                    pub struct Item<'a, Mode> {
                        pub source: ::validated_data::Field<&'a str>,
                    }
                }

                #[::validated_data::data_view(indexed_list, primary_key(source))]
                pub struct Include<'a, Mode> (::validated_data::Field<include::Item<'a, Mode>>);

                pub mod include {

                    #[::validated_data::data_view]
                    pub struct Item<'a, Mode> {
                        pub source: ::validated_data::Field<&'a str>,
                    }
                }
            }
        }

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct AccessLists<'a, Mode> (::validated_data::Field<access_lists::Item<'a, Mode>>);

        pub mod access_lists {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
            }
        }
    }

    #[::validated_data::data_view]
    pub struct Metadata<'a, Mode> {
        pub peer: ::validated_data::Field<&'a str>,
        pub peer_interface: ::validated_data::Field<&'a str>,
        pub peer_type: ::validated_data::Field<&'a str>,
        pub peer_key: ::validated_data::Field<&'a str>,
        pub port_profile: ::validated_data::Field<&'a str>,
        pub validate_state: ::validated_data::Field<bool>,
        pub validate_lldp: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view]
    pub struct Sflow<'a, Mode> {
        pub enable: ::validated_data::Field<bool>,
        pub egress: ::validated_data::Field<sflow::Egress<'a, Mode>>,
    }

    pub mod sflow {

        #[::validated_data::data_view]
        pub struct Egress<'a, Mode> {
            pub enable: ::validated_data::Field<bool>,
            pub unmodified_enable: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view]
    pub struct SyncE<'a, Mode> {
        pub enable: ::validated_data::Field<bool>,
        pub priority: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view(indexed_list, primary_key(id))]
    pub struct UcTxQueues<'a, Mode> (::validated_data::Field<uc_tx_queues::Item<'a, Mode>>);

    pub mod uc_tx_queues {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub id: ::validated_data::RequiredValue<i64, Mode>,
            pub random_detect: ::validated_data::Field<item::RandomDetect<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct RandomDetect<'a, Mode> {
                pub ecn: ::validated_data::Field<random_detect::Ecn<'a, Mode>>,
            }

            pub mod random_detect {

                #[::validated_data::data_view]
                pub struct Ecn<'a, Mode> {
                    pub count: ::validated_data::Field<bool>,
                    pub threshold: ::validated_data::Field<ecn::Threshold<'a, Mode>>,
                }

                pub mod ecn {

                    #[::validated_data::data_view]
                    pub struct Threshold<'a, Mode> {
                        pub units: ::validated_data::RequiredValue<&'a str, Mode>,
                        pub min: ::validated_data::RequiredValue<i64, Mode>,
                        pub max: ::validated_data::RequiredValue<i64, Mode>,
                        pub max_probability: ::validated_data::Field<i64>,
                        pub weight: ::validated_data::Field<i64>,
                    }
                }
            }
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(id))]
    pub struct TxQueues<'a, Mode> (::validated_data::Field<tx_queues::Item<'a, Mode>>);

    pub mod tx_queues {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub id: ::validated_data::RequiredValue<i64, Mode>,
            pub scheduler_profile_responsive: ::validated_data::Field<bool>,
            pub random_detect: ::validated_data::Field<item::RandomDetect<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct RandomDetect<'a, Mode> {
                pub ecn: ::validated_data::Field<random_detect::Ecn<'a, Mode>>,
            }

            pub mod random_detect {

                #[::validated_data::data_view]
                pub struct Ecn<'a, Mode> {
                    pub count: ::validated_data::Field<bool>,
                    pub threshold: ::validated_data::Field<ecn::Threshold<'a, Mode>>,
                }

                pub mod ecn {

                    #[::validated_data::data_view]
                    pub struct Threshold<'a, Mode> {
                        pub units: ::validated_data::RequiredValue<&'a str, Mode>,
                        pub min: ::validated_data::RequiredValue<i64, Mode>,
                        pub max: ::validated_data::RequiredValue<i64, Mode>,
                        pub max_probability: ::validated_data::RequiredValue<i64, Mode>,
                        pub weight: ::validated_data::Field<i64>,
                    }
                }
            }
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(id))]
    pub struct VrrpIds<'a, Mode> (::validated_data::Field<vrrp_ids::Item<'a, Mode>>);

    pub mod vrrp_ids {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub id: ::validated_data::Field<i64>,
            pub priority_level: ::validated_data::Field<i64>,
            pub advertisement: ::validated_data::Field<item::Advertisement<'a, Mode>>,
            pub preempt: ::validated_data::Field<item::Preempt<'a, Mode>>,
            pub timers: ::validated_data::Field<item::Timers<'a, Mode>>,
            pub tracked_object: ::validated_data::Field<item::TrackedObject<'a, Mode>>,
            pub ipv4: ::validated_data::Field<item::Ipv4<'a, Mode>>,
            pub ipv6: ::validated_data::Field<item::Ipv6<'a, Mode>>,
            pub peer_authentication: ::validated_data::Field<item::PeerAuthentication<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct Advertisement<'a, Mode> {
                pub interval: ::validated_data::Field<i64>,
            }

            #[::validated_data::data_view]
            pub struct Preempt<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub delay: ::validated_data::Field<preempt::Delay<'a, Mode>>,
            }

            pub mod preempt {

                #[::validated_data::data_view]
                pub struct Delay<'a, Mode> {
                    pub minimum: ::validated_data::Field<i64>,
                    pub reload: ::validated_data::Field<i64>,
                }
            }

            #[::validated_data::data_view]
            pub struct Timers<'a, Mode> {
                pub delay: ::validated_data::Field<timers::Delay<'a, Mode>>,
            }

            pub mod timers {

                #[::validated_data::data_view]
                pub struct Delay<'a, Mode> {
                    pub reload: ::validated_data::Field<i64>,
                }
            }

            #[::validated_data::data_view(indexed_list, primary_key(name))]
            pub struct TrackedObject<'a, Mode> (::validated_data::Field<tracked_object::Item<'a, Mode>>);

            pub mod tracked_object {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub name: ::validated_data::Field<&'a str>,
                    pub decrement: ::validated_data::Field<i64>,
                    pub shutdown: ::validated_data::Field<bool>,
                }
            }

            #[::validated_data::data_view]
            pub struct Ipv4<'a, Mode> {
                pub address: ::validated_data::RequiredValue<&'a str, Mode>,
                pub secondary_addresses: ::validated_data::Field<ipv4::SecondaryAddresses<'a, Mode>>,
                pub version: ::validated_data::Field<i64>,
            }

            pub mod ipv4 {

                #[::validated_data::data_view(list)]
                pub struct SecondaryAddresses<'a, Mode> (::validated_data::Field<&'a str>);
            }

            #[::validated_data::data_view]
            pub struct Ipv6<'a, Mode> {
                pub addresses: ::validated_data::RequiredValue<ipv6::Addresses<'a, Mode>, Mode>,
            }

            pub mod ipv6 {

                #[::validated_data::data_view(list)]
                pub struct Addresses<'a, Mode> (::validated_data::Field<&'a str>);
            }

            #[::validated_data::data_view]
            pub struct PeerAuthentication<'a, Mode> {
                pub mode: ::validated_data::RequiredValue<&'a str, Mode>,
                pub key: ::validated_data::RequiredValue<&'a str, Mode>,
                pub key_type: ::validated_data::Field<&'a str>,
            }
        }
    }

    #[::validated_data::data_view]
    pub struct Switchport<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub mode: ::validated_data::Field<&'a str>,
        pub access_vlan: ::validated_data::Field<i64>,
        pub trunk: ::validated_data::Field<switchport::Trunk<'a, Mode>>,
        pub phone: ::validated_data::Field<switchport::Phone<'a, Mode>>,
        pub pvlan_mapping: ::validated_data::Field<&'a str>,
        pub dot1q: ::validated_data::Field<switchport::Dot1q<'a, Mode>>,
        pub source_interface: ::validated_data::Field<&'a str>,
        pub vlan_translations: ::validated_data::Field<switchport::VlanTranslations<'a, Mode>>,
        pub vlan_forwarding_accept_all: ::validated_data::Field<bool>,
        pub backup_link: ::validated_data::Field<switchport::BackupLink<'a, Mode>>,
        pub backup: ::validated_data::Field<switchport::Backup<'a, Mode>>,
        pub port_security: ::validated_data::Field<switchport::PortSecurity<'a, Mode>>,
        pub tap: ::validated_data::Field<switchport::Tap<'a, Mode>>,
        pub tool: ::validated_data::Field<switchport::Tool<'a, Mode>>,
    }

    pub mod switchport {

        #[::validated_data::data_view]
        pub struct Trunk<'a, Mode> {
            pub allowed_vlan: ::validated_data::Field<&'a str>,
            pub native_vlan: ::validated_data::Field<i64>,
            pub native_vlan_tag: ::validated_data::Field<bool>,
            pub private_vlan_secondary: ::validated_data::Field<bool>,
            pub groups: ::validated_data::Field<trunk::Groups<'a, Mode>>,
        }

        pub mod trunk {

            #[::validated_data::data_view(list)]
            pub struct Groups<'a, Mode> (::validated_data::Field<&'a str>);
        }

        #[::validated_data::data_view]
        pub struct Phone<'a, Mode> {
            pub vlan: ::validated_data::Field<i64>,
            pub trunk: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct Dot1q<'a, Mode> {
            pub ethertype: ::validated_data::Field<i64>,
            pub vlan_tag: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct VlanTranslations<'a, Mode> {
            pub in_required: ::validated_data::Field<bool>,
            pub out_required: ::validated_data::Field<bool>,
            pub direction_in: ::validated_data::Field<vlan_translations::DirectionIn<'a, Mode>>,
            pub direction_out: ::validated_data::Field<vlan_translations::DirectionOut<'a, Mode>>,
            pub direction_both: ::validated_data::Field<vlan_translations::DirectionBoth<'a, Mode>>,
        }

        pub mod vlan_translations {

            #[::validated_data::data_view(list)]
            pub struct DirectionIn<'a, Mode> (::validated_data::Field<direction_in::Item<'a, Mode>>);

            pub mod direction_in {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    #[data_view(rename = "from")]
                    pub field_from: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub to: ::validated_data::RequiredValue<i64, Mode>,
                    pub dot1q_tunnel: ::validated_data::Field<bool>,
                    pub inner_vlan_from: ::validated_data::Field<i64>,
                }
            }

            #[::validated_data::data_view(list)]
            pub struct DirectionOut<'a, Mode> (::validated_data::Field<direction_out::Item<'a, Mode>>);

            pub mod direction_out {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    #[data_view(rename = "from")]
                    pub field_from: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub to: ::validated_data::Field<i64>,
                    pub dot1q_tunnel_to: ::validated_data::Field<&'a str>,
                    pub inner_vlan_to: ::validated_data::Field<i64>,
                }
            }

            #[::validated_data::data_view(list)]
            pub struct DirectionBoth<'a, Mode> (::validated_data::Field<direction_both::Item<'a, Mode>>);

            pub mod direction_both {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    #[data_view(rename = "from")]
                    pub field_from: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub to: ::validated_data::RequiredValue<i64, Mode>,
                    pub dot1q_tunnel: ::validated_data::Field<bool>,
                    pub inner_vlan_from: ::validated_data::Field<i64>,
                    pub network: ::validated_data::Field<bool>,
                }
            }
        }

        #[::validated_data::data_view]
        pub struct BackupLink<'a, Mode> {
            pub interface: ::validated_data::Field<&'a str>,
            pub prefer_vlan: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct Backup<'a, Mode> {
            pub dest_macaddr: ::validated_data::Field<&'a str>,
            pub initial_mac_move_delay: ::validated_data::Field<i64>,
            pub mac_move_burst: ::validated_data::Field<i64>,
            pub mac_move_burst_interval: ::validated_data::Field<i64>,
            pub preemption_delay: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view]
        pub struct PortSecurity<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub mac_address_maximum: ::validated_data::Field<port_security::MacAddressMaximum<'a, Mode>>,
            pub violation: ::validated_data::Field<port_security::Violation<'a, Mode>>,
            pub vlan_default_mac_address_maximum: ::validated_data::Field<i64>,
            pub vlans: ::validated_data::Field<port_security::Vlans<'a, Mode>>,
        }

        pub mod port_security {

            #[::validated_data::data_view]
            pub struct MacAddressMaximum<'a, Mode> {
                pub disabled: ::validated_data::Field<bool>,
                pub limit: ::validated_data::Field<i64>,
            }

            #[::validated_data::data_view]
            pub struct Violation<'a, Mode> {
                pub mode: ::validated_data::Field<&'a str>,
                pub protect_log: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(indexed_list, primary_key(range))]
            pub struct Vlans<'a, Mode> (::validated_data::Field<vlans::Item<'a, Mode>>);

            pub mod vlans {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub range: ::validated_data::Field<&'a str>,
                    pub mac_address_maximum: ::validated_data::RequiredValue<i64, Mode>,
                }
            }
        }

        #[::validated_data::data_view]
        pub struct Tap<'a, Mode> {
            pub allowed_vlan: ::validated_data::Field<&'a str>,
            pub default: ::validated_data::Field<tap::Default<'a, Mode>>,
            pub identity: ::validated_data::Field<tap::Identity<'a, Mode>>,
            pub mpls_pop_all: ::validated_data::Field<bool>,
            pub native_vlan: ::validated_data::Field<i64>,
            pub truncation: ::validated_data::Field<tap::Truncation<'a, Mode>>,
            pub mac_address: ::validated_data::Field<tap::MacAddress<'a, Mode>>,
            pub encapsulation: ::validated_data::Field<tap::Encapsulation<'a, Mode>>,
        }

        pub mod tap {

            #[::validated_data::data_view]
            pub struct Default<'a, Mode> {
                pub groups: ::validated_data::Field<default::Groups<'a, Mode>>,
                pub interfaces: ::validated_data::Field<default::Interfaces<'a, Mode>>,
                pub nexthop_groups: ::validated_data::Field<default::NexthopGroups<'a, Mode>>,
            }

            pub mod default {

                #[::validated_data::data_view(list)]
                pub struct Groups<'a, Mode> (::validated_data::Field<&'a str>);

                #[::validated_data::data_view(list)]
                pub struct Interfaces<'a, Mode> (::validated_data::Field<&'a str>);

                #[::validated_data::data_view(list)]
                pub struct NexthopGroups<'a, Mode> (::validated_data::Field<&'a str>);
            }

            #[::validated_data::data_view]
            pub struct Identity<'a, Mode> {
                pub id: ::validated_data::Field<i64>,
                pub inner_vlan: ::validated_data::Field<i64>,
            }

            #[::validated_data::data_view]
            pub struct Truncation<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub size: ::validated_data::Field<i64>,
            }

            #[::validated_data::data_view]
            pub struct MacAddress<'a, Mode> {
                pub source: ::validated_data::Field<&'a str>,
                pub destination: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct Encapsulation<'a, Mode> {
                pub vxlan_strip: ::validated_data::Field<bool>,
                pub gre: ::validated_data::Field<encapsulation::Gre<'a, Mode>>,
            }

            pub mod encapsulation {

                #[::validated_data::data_view]
                pub struct Gre<'a, Mode> {
                    pub strip: ::validated_data::Field<bool>,
                    pub protocols: ::validated_data::Field<gre::Protocols<'a, Mode>>,
                    pub destinations: ::validated_data::Field<gre::Destinations<'a, Mode>>,
                }

                pub mod gre {

                    #[::validated_data::data_view(indexed_list, primary_key(protocol))]
                    pub struct Protocols<'a, Mode> (::validated_data::Field<protocols::Item<'a, Mode>>);

                    pub mod protocols {

                        #[::validated_data::data_view]
                        pub struct Item<'a, Mode> {
                            pub protocol: ::validated_data::Field<&'a str>,
                            pub strip: ::validated_data::Field<bool>,
                            pub feature_header_length: ::validated_data::Field<i64>,
                            pub re_encapsulation_ethernet_header: ::validated_data::Field<bool>,
                        }
                    }

                    #[::validated_data::data_view(indexed_list, primary_key(destination))]
                    pub struct Destinations<'a, Mode> (::validated_data::Field<destinations::Item<'a, Mode>>);

                    pub mod destinations {

                        #[::validated_data::data_view]
                        pub struct Item<'a, Mode> {
                            pub destination: ::validated_data::Field<&'a str>,
                            pub source: ::validated_data::Field<&'a str>,
                            pub strip: ::validated_data::Field<bool>,
                            pub protocols: ::validated_data::Field<item::Protocols<'a, Mode>>,
                        }

                        pub mod item {

                            #[::validated_data::data_view(indexed_list, primary_key(protocol))]
                            pub struct Protocols<'a, Mode> (::validated_data::Field<protocols::Item<'a, Mode>>);

                            pub mod protocols {

                                #[::validated_data::data_view]
                                pub struct Item<'a, Mode> {
                                    pub protocol: ::validated_data::Field<&'a str>,
                                    pub strip: ::validated_data::Field<bool>,
                                    pub feature_header_length: ::validated_data::Field<i64>,
                                    pub re_encapsulation_ethernet_header: ::validated_data::Field<bool>,
                                }
                            }
                        }
                    }
                }
            }
        }

        #[::validated_data::data_view]
        pub struct Tool<'a, Mode> {
            pub mpls_pop_all: ::validated_data::Field<bool>,
            pub encapsulation: ::validated_data::Field<tool::Encapsulation<'a, Mode>>,
            pub allowed_vlan: ::validated_data::Field<&'a str>,
            pub identity: ::validated_data::Field<tool::Identity<'a, Mode>>,
            pub groups: ::validated_data::Field<tool::Groups<'a, Mode>>,
            pub dot1q_remove_outer_vlan_tag: ::validated_data::Field<&'a str>,
        }

        pub mod tool {

            #[::validated_data::data_view]
            pub struct Encapsulation<'a, Mode> {
                pub dot1br_strip: ::validated_data::Field<bool>,
                pub vn_tag_strip: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view]
            pub struct Identity<'a, Mode> {
                pub tag: ::validated_data::Field<&'a str>,
                pub dot1q_dzgre_source: ::validated_data::Field<&'a str>,
                pub qinq_dzgre_source: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view(list)]
            pub struct Groups<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }

    #[::validated_data::data_view]
    pub struct TrafficEngineering<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub administrative_groups: ::validated_data::Field<traffic_engineering::AdministrativeGroups<'a, Mode>>,
        pub srlgs: ::validated_data::Field<traffic_engineering::Srlgs<'a, Mode>>,
        pub metric: ::validated_data::Field<i64>,
        pub bandwidth: ::validated_data::Field<traffic_engineering::Bandwidth<'a, Mode>>,
        pub min_delay_static: ::validated_data::Field<traffic_engineering::MinDelayStatic<'a, Mode>>,
        pub min_delay_dynamic: ::validated_data::Field<traffic_engineering::MinDelayDynamic<'a, Mode>>,
    }

    pub mod traffic_engineering {

        #[::validated_data::data_view(list)]
        pub struct AdministrativeGroups<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list)]
        pub struct Srlgs<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view]
        pub struct Bandwidth<'a, Mode> {
            pub number: ::validated_data::RequiredValue<i64, Mode>,
            pub unit: ::validated_data::RequiredValue<&'a str, Mode>,
        }

        #[::validated_data::data_view]
        pub struct MinDelayStatic<'a, Mode> {
            pub number: ::validated_data::RequiredValue<i64, Mode>,
            pub unit: ::validated_data::RequiredValue<&'a str, Mode>,
        }

        #[::validated_data::data_view]
        pub struct MinDelayDynamic<'a, Mode> {
            pub twamp_light_fallback: ::validated_data::Field<min_delay_dynamic::TwampLightFallback<'a, Mode>>,
        }

        pub mod min_delay_dynamic {

            #[::validated_data::data_view]
            pub struct TwampLightFallback<'a, Mode> {
                pub number: ::validated_data::RequiredValue<i64, Mode>,
                pub unit: ::validated_data::RequiredValue<&'a str, Mode>,
            }
        }
    }

    #[::validated_data::data_view(list)]
    pub struct MonitorLinkFlapProfiles<'a, Mode> (::validated_data::Field<&'a str>);
}
