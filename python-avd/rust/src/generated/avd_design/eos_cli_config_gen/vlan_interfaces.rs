// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub description: ::validated_data::Field<&'a str>,
    pub logging: ::validated_data::Field<item::Logging<'a, Mode>>,
    pub shutdown: ::validated_data::Field<bool>,
    pub vrf: ::validated_data::Field<&'a str>,
    pub arp_aging_timeout: ::validated_data::Field<i64>,
    pub arp_cache_dynamic_capacity: ::validated_data::Field<i64>,
    pub arp_gratuitous_accept: ::validated_data::Field<bool>,
    pub arp_monitor_mac_address: ::validated_data::Field<bool>,
    pub ip_proxy_arp: ::validated_data::Field<bool>,
    pub ip_directed_broadcast: ::validated_data::Field<bool>,
    pub ip_address: ::validated_data::Field<&'a str>,
    pub dhcp_client_accept_default_route: ::validated_data::Field<bool>,
    pub ip_address_secondaries: ::validated_data::Field<item::IpAddressSecondaries<'a, Mode>>,
    pub ip_virtual_router_addresses: ::validated_data::Field<item::IpVirtualRouterAddresses<'a, Mode>>,
    pub ip_address_virtual: ::validated_data::Field<&'a str>,
    pub ip_address_virtual_secondaries: ::validated_data::Field<item::IpAddressVirtualSecondaries<'a, Mode>>,
    pub ip_verify_unicast_source_reachable_via: ::validated_data::Field<&'a str>,
    pub ip_igmp: ::validated_data::Field<bool>,
    pub ip_igmp_version: ::validated_data::Field<i64>,
    pub ip_igmp_querier_address_virtual: ::validated_data::Field<bool>,
    pub ip_igmp_host_proxy: ::validated_data::Field<item::IpIgmpHostProxy<'a, Mode>>,
    pub ip_helpers: ::validated_data::Field<item::IpHelpers<'a, Mode>>,
    pub ip_dhcp_relay_all_subnets: ::validated_data::Field<bool>,
    pub ip_nat: ::validated_data::Field<item::IpNat<'a, Mode>>,
    pub dhcp_server_ipv4: ::validated_data::Field<bool>,
    pub dhcp_server_ipv6: ::validated_data::Field<bool>,
    pub ipv6_enable: ::validated_data::Field<bool>,
    pub ipv6_address: ::validated_data::Field<&'a str>,
    pub ipv6_addresses: ::validated_data::Field<item::Ipv6Addresses<'a, Mode>>,
    pub ipv6_address_auto_config: ::validated_data::Field<bool>,
    pub ipv6_address_virtuals: ::validated_data::Field<item::Ipv6AddressVirtuals<'a, Mode>>,
    pub ipv6_address_link_local: ::validated_data::Field<&'a str>,
    pub ipv6_virtual_router_addresses: ::validated_data::Field<item::Ipv6VirtualRouterAddresses<'a, Mode>>,
    pub ipv6_nd_ra_disabled: ::validated_data::Field<bool>,
    pub ipv6_nd_managed_config_flag: ::validated_data::Field<bool>,
    pub ipv6_nd_other_config_flag: ::validated_data::Field<bool>,
    pub ipv6_nd_cache: ::validated_data::Field<item::Ipv6NdCache<'a, Mode>>,
    pub ipv6_nd_prefixes: ::validated_data::Field<item::Ipv6NdPrefixes<'a, Mode>>,
    pub ipv6_dhcp_relay_destinations: ::validated_data::Field<item::Ipv6DhcpRelayDestinations<'a, Mode>>,
    pub ipv6_dhcp_relay_all_subnets: ::validated_data::Field<bool>,
    pub ipv6_nd: ::validated_data::Field<item::Ipv6Nd<'a, Mode>>,
    pub access_group_in: ::validated_data::Field<&'a str>,
    pub access_group_out: ::validated_data::Field<&'a str>,
    pub ipv6_access_group_in: ::validated_data::Field<&'a str>,
    pub ipv6_access_group_out: ::validated_data::Field<&'a str>,
    pub multicast: ::validated_data::Field<item::Multicast<'a, Mode>>,
    pub ospf_network_point_to_point: ::validated_data::Field<bool>,
    pub ospf_area: ::validated_data::Field<&'a str>,
    pub ipv6_ospf: ::validated_data::Field<item::Ipv6Ospf<'a, Mode>>,
    pub ospfv3: ::validated_data::Field<item::Ospfv3<'a, Mode>>,
    pub ospf_cost: ::validated_data::Field<i64>,
    pub ospf_authentication: ::validated_data::Field<&'a str>,
    pub ospf_authentication_key: ::validated_data::Field<&'a str>,
    pub ospf_authentication_key_type: ::validated_data::Field<&'a str>,
    pub ospf_message_digest_keys: ::validated_data::Field<item::OspfMessageDigestKeys<'a, Mode>>,
    pub pim: ::validated_data::Field<item::Pim<'a, Mode>>,
    pub isis_enable: ::validated_data::Field<&'a str>,
    pub isis_bfd: ::validated_data::Field<bool>,
    pub isis_passive: ::validated_data::Field<bool>,
    pub isis_metric: ::validated_data::Field<i64>,
    pub isis_network_point_to_point: ::validated_data::Field<bool>,
    pub isis_circuit_type: ::validated_data::Field<&'a str>,
    pub isis_hello_padding: ::validated_data::Field<bool>,
    pub isis_authentication: ::validated_data::Field<item::IsisAuthentication<'a, Mode>>,
    pub mtu: ::validated_data::Field<i64>,
    pub no_autostate: ::validated_data::Field<bool>,
    pub vrrp_ids: ::validated_data::Field<item::VrrpIds<'a, Mode>>,
    pub ip_attached_host_route_export: ::validated_data::Field<item::IpAttachedHostRouteExport<'a, Mode>>,
    pub ipv6_attached_host_route_export: ::validated_data::Field<item::Ipv6AttachedHostRouteExport<'a, Mode>>,
    pub bfd: ::validated_data::Field<item::Bfd<'a, Mode>>,
    pub service_policy: ::validated_data::Field<item::ServicePolicy<'a, Mode>>,
    pub tcp_mss_ceiling: ::validated_data::Field<item::TcpMssCeiling<'a, Mode>>,
    pub traffic_policy: ::validated_data::Field<item::TrafficPolicy<'a, Mode>>,
    pub mpls: ::validated_data::Field<item::Mpls<'a, Mode>>,
    pub ntp_serve: ::validated_data::Field<bool>,
    pub pvlan_mapping: ::validated_data::Field<&'a str>,
    pub metadata: ::validated_data::Field<item::Metadata<'a, Mode>>,
    pub eos_cli: ::validated_data::Field<&'a str>,
}

pub mod item {

    #[::validated_data::data_view]
    pub struct Logging<'a, Mode> {
        pub event: ::validated_data::Field<logging::Event<'a, Mode>>,
    }

    pub mod logging {

        #[::validated_data::data_view]
        pub struct Event<'a, Mode> {
            pub link_status: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view(list)]
    pub struct IpAddressSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list)]
    pub struct IpVirtualRouterAddresses<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list)]
    pub struct IpAddressVirtualSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

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

    #[::validated_data::data_view(list)]
    pub struct Ipv6AddressVirtuals<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list)]
    pub struct Ipv6VirtualRouterAddresses<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view]
    pub struct Ipv6NdCache<'a, Mode> {
        pub dynamic_capacity: ::validated_data::Field<i64>,
        pub expire: ::validated_data::Field<i64>,
        pub refresh_always: ::validated_data::Field<bool>,
    }

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

    #[::validated_data::data_view(indexed_list, primary_key(address))]
    pub struct Ipv6DhcpRelayDestinations<'a, Mode> (::validated_data::Field<ipv6_dhcp_relay_destinations::Item<'a, Mode>>);

    pub mod ipv6_dhcp_relay_destinations {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub address: ::validated_data::Field<&'a str>,
            pub vrf: ::validated_data::Field<&'a str>,
            pub local_interface: ::validated_data::Field<&'a str>,
            pub source_address: ::validated_data::Field<&'a str>,
            pub link_address: ::validated_data::Field<&'a str>,
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

    #[::validated_data::data_view]
    pub struct Multicast<'a, Mode> {
        pub ipv4: ::validated_data::Field<multicast::Ipv4<'a, Mode>>,
        pub ipv6: ::validated_data::Field<multicast::Ipv6<'a, Mode>>,
    }

    pub mod multicast {

        #[::validated_data::data_view]
        pub struct Ipv4<'a, Mode> {
            pub boundaries: ::validated_data::Field<ipv4::Boundaries<'a, Mode>>,
            pub source_route_export: ::validated_data::Field<ipv4::SourceRouteExport<'a, Mode>>,
            #[data_view(rename = "static")]
            pub field_static: ::validated_data::Field<bool>,
        }

        pub mod ipv4 {

            #[::validated_data::data_view(indexed_list, primary_key(boundary))]
            pub struct Boundaries<'a, Mode> (::validated_data::Field<boundaries::Item<'a, Mode>>);

            pub mod boundaries {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub boundary: ::validated_data::Field<&'a str>,
                    pub out: ::validated_data::Field<bool>,
                }
            }

            #[::validated_data::data_view]
            pub struct SourceRouteExport<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub administrative_distance: ::validated_data::Field<i64>,
            }
        }

        #[::validated_data::data_view]
        pub struct Ipv6<'a, Mode> {
            pub boundaries: ::validated_data::Field<ipv6::Boundaries<'a, Mode>>,
            pub source_route_export: ::validated_data::Field<ipv6::SourceRouteExport<'a, Mode>>,
            #[data_view(rename = "static")]
            pub field_static: ::validated_data::Field<bool>,
        }

        pub mod ipv6 {

            #[::validated_data::data_view(indexed_list, primary_key(boundary))]
            pub struct Boundaries<'a, Mode> (::validated_data::Field<boundaries::Item<'a, Mode>>);

            pub mod boundaries {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub boundary: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view]
            pub struct SourceRouteExport<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub administrative_distance: ::validated_data::Field<i64>,
            }
        }
    }

    #[::validated_data::data_view]
    pub struct Ipv6Ospf<'a, Mode> {
        pub process: ::validated_data::Field<ipv6_ospf::Process<'a, Mode>>,
        pub network_point_to_point: ::validated_data::Field<bool>,
    }

    pub mod ipv6_ospf {

        #[::validated_data::data_view]
        pub struct Process<'a, Mode> {
            pub id: ::validated_data::RequiredValue<i64, Mode>,
            pub area: ::validated_data::RequiredValue<&'a str, Mode>,
        }
    }

    #[::validated_data::data_view]
    pub struct Ospfv3<'a, Mode> {
        pub ipv4: ::validated_data::Field<ospfv3::Ipv4<'a, Mode>>,
        pub ipv6: ::validated_data::Field<ospfv3::Ipv6<'a, Mode>>,
        pub passive_interface: ::validated_data::Field<bool>,
        pub network_point_to_point: ::validated_data::Field<bool>,
    }

    pub mod ospfv3 {

        #[::validated_data::data_view]
        pub struct Ipv4<'a, Mode> {
            pub area: ::validated_data::RequiredValue<&'a str, Mode>,
        }

        #[::validated_data::data_view]
        pub struct Ipv6<'a, Mode> {
            pub area: ::validated_data::RequiredValue<&'a str, Mode>,
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
            pub local_interface: ::validated_data::Field<&'a str>,
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
    pub struct IpAttachedHostRouteExport<'a, Mode> {
        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
        pub distance: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Ipv6AttachedHostRouteExport<'a, Mode> {
        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
        pub distance: ::validated_data::Field<i64>,
        pub prefix_length: ::validated_data::Field<i64>,
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
    }

    pub mod service_policy {

        #[::validated_data::data_view]
        pub struct Pbr<'a, Mode> {
            pub input: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view]
    pub struct TcpMssCeiling<'a, Mode> {
        pub ipv4: ::validated_data::Field<i64>,
        pub ipv6: ::validated_data::Field<i64>,
        pub direction: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct TrafficPolicy<'a, Mode> {
        pub input: ::validated_data::Field<&'a str>,
        pub output: ::validated_data::Field<&'a str>,
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
    pub struct Metadata<'a, Mode> {
        pub tenants: ::validated_data::Field<metadata::Tenants<'a, Mode>>,
        pub tags: ::validated_data::Field<metadata::Tags<'a, Mode>>,
        #[data_view(rename = "type")]
        pub field_type: ::validated_data::Field<&'a str>,
    }

    pub mod metadata {

        #[::validated_data::data_view(list)]
        pub struct Tenants<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list)]
        pub struct Tags<'a, Mode> (::validated_data::Field<&'a str>);
    }
}
