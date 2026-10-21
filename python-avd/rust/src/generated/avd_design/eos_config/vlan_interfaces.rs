// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct VlanInterfaces<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view]
pub struct Item<'a, Mode> {
    pub name: ::data_views::Field<&'a str>,
    pub description: ::data_views::Field<&'a str>,
    pub logging: ::data_views::Field<item::Logging<'a, Mode>>,
    pub shutdown: ::data_views::Field<bool>,
    pub vrf: ::data_views::Field<&'a str>,
    pub arp_aging_timeout: ::data_views::Field<i64>,
    pub arp_cache_dynamic_capacity: ::data_views::Field<i64>,
    pub arp_gratuitous_accept: ::data_views::Field<bool>,
    pub arp_monitor_mac_address: ::data_views::Field<bool>,
    pub ip_proxy_arp: ::data_views::Field<bool>,
    pub ip_directed_broadcast: ::data_views::Field<bool>,
    pub ip_address: ::data_views::Field<&'a str>,
    pub dhcp_client_accept_default_route: ::data_views::Field<bool>,
    pub ip_address_secondaries: ::data_views::Field<item::IpAddressSecondaries<'a, Mode>>,
    pub ip_virtual_router_addresses: ::data_views::Field<item::IpVirtualRouterAddresses<'a, Mode>>,
    pub ip_address_virtual: ::data_views::Field<&'a str>,
    pub ip_address_virtual_secondaries: ::data_views::Field<item::IpAddressVirtualSecondaries<'a, Mode>>,
    pub ip_verify_unicast_source_reachable_via: ::data_views::Field<&'a str>,
    pub ip_igmp: ::data_views::Field<bool>,
    pub ip_igmp_version: ::data_views::Field<i64>,
    pub ip_igmp_querier_address_virtual: ::data_views::Field<bool>,
    pub ip_igmp_host_proxy: ::data_views::Field<item::IpIgmpHostProxy<'a, Mode>>,
    pub ip_helpers: ::data_views::Field<item::IpHelpers<'a, Mode>>,
    pub ip_dhcp_relay_all_subnets: ::data_views::Field<bool>,
    pub ip_nat: ::data_views::Field<item::IpNat<'a, Mode>>,
    pub dhcp_server_ipv4: ::data_views::Field<bool>,
    pub dhcp_server_ipv6: ::data_views::Field<bool>,
    pub ipv6_enable: ::data_views::Field<bool>,
    pub ipv6_address: ::data_views::Field<&'a str>,
    pub ipv6_addresses: ::data_views::Field<item::Ipv6Addresses<'a, Mode>>,
    pub ipv6_address_auto_config: ::data_views::Field<bool>,
    pub ipv6_address_virtuals: ::data_views::Field<item::Ipv6AddressVirtuals<'a, Mode>>,
    pub ipv6_address_link_local: ::data_views::Field<&'a str>,
    pub ipv6_virtual_router_addresses: ::data_views::Field<item::Ipv6VirtualRouterAddresses<'a, Mode>>,
    pub ipv6_nd_ra_disabled: ::data_views::Field<bool>,
    pub ipv6_nd_managed_config_flag: ::data_views::Field<bool>,
    pub ipv6_nd_other_config_flag: ::data_views::Field<bool>,
    pub ipv6_nd_cache: ::data_views::Field<item::Ipv6NdCache<'a, Mode>>,
    pub ipv6_nd_prefixes: ::data_views::Field<item::Ipv6NdPrefixes<'a, Mode>>,
    pub ipv6_dhcp_relay_destinations: ::data_views::Field<item::Ipv6DhcpRelayDestinations<'a, Mode>>,
    pub ipv6_dhcp_relay_all_subnets: ::data_views::Field<bool>,
    pub ipv6_nd: ::data_views::Field<item::Ipv6Nd<'a, Mode>>,
    pub access_group_in: ::data_views::Field<&'a str>,
    pub access_group_out: ::data_views::Field<&'a str>,
    pub ipv6_access_group_in: ::data_views::Field<&'a str>,
    pub ipv6_access_group_out: ::data_views::Field<&'a str>,
    pub multicast: ::data_views::Field<item::Multicast<'a, Mode>>,
    pub ospf_network_point_to_point: ::data_views::Field<bool>,
    pub ospf_area: ::data_views::Field<&'a str>,
    pub ipv6_ospf: ::data_views::Field<item::Ipv6Ospf<'a, Mode>>,
    pub ospfv3: ::data_views::Field<item::Ospfv3<'a, Mode>>,
    pub ospf_cost: ::data_views::Field<i64>,
    pub ospf_authentication: ::data_views::Field<&'a str>,
    pub ospf_authentication_key: ::data_views::Field<&'a str>,
    pub ospf_authentication_key_type: ::data_views::Field<&'a str>,
    pub ospf_message_digest_keys: ::data_views::Field<item::OspfMessageDigestKeys<'a, Mode>>,
    pub pim: ::data_views::Field<item::Pim<'a, Mode>>,
    pub isis_enable: ::data_views::Field<&'a str>,
    pub isis_bfd: ::data_views::Field<bool>,
    pub isis_passive: ::data_views::Field<bool>,
    pub isis_metric: ::data_views::Field<i64>,
    pub isis_network_point_to_point: ::data_views::Field<bool>,
    pub isis_circuit_type: ::data_views::Field<&'a str>,
    pub isis_hello_padding: ::data_views::Field<bool>,
    pub isis_authentication: ::data_views::Field<item::IsisAuthentication<'a, Mode>>,
    pub mtu: ::data_views::Field<i64>,
    pub no_autostate: ::data_views::Field<bool>,
    pub vrrp_ids: ::data_views::Field<item::VrrpIds<'a, Mode>>,
    pub ip_attached_host_route_export: ::data_views::Field<item::IpAttachedHostRouteExport<'a, Mode>>,
    pub ipv6_attached_host_route_export: ::data_views::Field<item::Ipv6AttachedHostRouteExport<'a, Mode>>,
    pub bfd: ::data_views::Field<item::Bfd<'a, Mode>>,
    pub service_policy: ::data_views::Field<item::ServicePolicy<'a, Mode>>,
    pub tcp_mss_ceiling: ::data_views::Field<item::TcpMssCeiling<'a, Mode>>,
    pub traffic_policy: ::data_views::Field<item::TrafficPolicy<'a, Mode>>,
    pub mpls: ::data_views::Field<item::Mpls<'a, Mode>>,
    pub ntp_serve: ::data_views::Field<bool>,
    pub pvlan_mapping: ::data_views::Field<&'a str>,
    pub metadata: ::data_views::Field<item::Metadata<'a, Mode>>,
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
        }
    }

    #[::data_views::data_view(list)]
    pub struct IpAddressSecondaries<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(list)]
    pub struct IpVirtualRouterAddresses<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(list)]
    pub struct IpAddressVirtualSecondaries<'a, Mode> (::data_views::Field<&'a str>);

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

    #[::data_views::data_view(indexed_list, primary_key(ip_helper))]
    pub struct IpHelpers<'a, Mode> (::data_views::Field<ip_helpers::Item<'a, Mode>>);

    pub mod ip_helpers {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub ip_helper: ::data_views::Field<&'a str>,
            pub source_interface: ::data_views::Field<&'a str>,
            pub vrf: ::data_views::Field<&'a str>,
        }
    }

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

    #[::data_views::data_view(list)]
    pub struct Ipv6AddressVirtuals<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(list)]
    pub struct Ipv6VirtualRouterAddresses<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view]
    pub struct Ipv6NdCache<'a, Mode> {
        pub dynamic_capacity: ::data_views::Field<i64>,
        pub expire: ::data_views::Field<i64>,
        pub refresh_always: ::data_views::Field<bool>,
    }

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

    #[::data_views::data_view(indexed_list, primary_key(address))]
    pub struct Ipv6DhcpRelayDestinations<'a, Mode> (::data_views::Field<ipv6_dhcp_relay_destinations::Item<'a, Mode>>);

    pub mod ipv6_dhcp_relay_destinations {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub address: ::data_views::Field<&'a str>,
            pub vrf: ::data_views::Field<&'a str>,
            pub local_interface: ::data_views::Field<&'a str>,
            pub source_address: ::data_views::Field<&'a str>,
            pub link_address: ::data_views::Field<&'a str>,
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
    pub struct Multicast<'a, Mode> {
        pub ipv4: ::data_views::Field<multicast::Ipv4<'a, Mode>>,
        pub ipv6: ::data_views::Field<multicast::Ipv6<'a, Mode>>,
    }

    pub mod multicast {

        #[::data_views::data_view]
        pub struct Ipv4<'a, Mode> {
            pub boundaries: ::data_views::Field<ipv4::Boundaries<'a, Mode>>,
            pub source_route_export: ::data_views::Field<ipv4::SourceRouteExport<'a, Mode>>,
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

            #[::data_views::data_view]
            pub struct SourceRouteExport<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub administrative_distance: ::data_views::Field<i64>,
            }
        }

        #[::data_views::data_view]
        pub struct Ipv6<'a, Mode> {
            pub boundaries: ::data_views::Field<ipv6::Boundaries<'a, Mode>>,
            pub source_route_export: ::data_views::Field<ipv6::SourceRouteExport<'a, Mode>>,
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

            #[::data_views::data_view]
            pub struct SourceRouteExport<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub administrative_distance: ::data_views::Field<i64>,
            }
        }
    }

    #[::data_views::data_view]
    pub struct Ipv6Ospf<'a, Mode> {
        pub process: ::data_views::Field<ipv6_ospf::Process<'a, Mode>>,
        pub network_point_to_point: ::data_views::Field<bool>,
    }

    pub mod ipv6_ospf {

        #[::data_views::data_view]
        pub struct Process<'a, Mode> {
            pub id: ::data_views::RequiredValue<i64, Mode>,
            pub area: ::data_views::RequiredValue<&'a str, Mode>,
        }
    }

    #[::data_views::data_view]
    pub struct Ospfv3<'a, Mode> {
        pub ipv4: ::data_views::Field<ospfv3::Ipv4<'a, Mode>>,
        pub ipv6: ::data_views::Field<ospfv3::Ipv6<'a, Mode>>,
        pub passive_interface: ::data_views::Field<bool>,
        pub network_point_to_point: ::data_views::Field<bool>,
    }

    pub mod ospfv3 {

        #[::data_views::data_view]
        pub struct Ipv4<'a, Mode> {
            pub area: ::data_views::RequiredValue<&'a str, Mode>,
        }

        #[::data_views::data_view]
        pub struct Ipv6<'a, Mode> {
            pub area: ::data_views::RequiredValue<&'a str, Mode>,
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
    pub struct Pim<'a, Mode> {
        pub ipv4: ::data_views::Field<pim::Ipv4<'a, Mode>>,
    }

    pub mod pim {

        #[::data_views::data_view]
        pub struct Ipv4<'a, Mode> {
            pub border_router: ::data_views::Field<bool>,
            pub dr_priority: ::data_views::Field<i64>,
            pub sparse_mode: ::data_views::Field<bool>,
            pub local_interface: ::data_views::Field<&'a str>,
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
    pub struct IpAttachedHostRouteExport<'a, Mode> {
        pub enabled: ::data_views::RequiredValue<bool, Mode>,
        pub distance: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct Ipv6AttachedHostRouteExport<'a, Mode> {
        pub enabled: ::data_views::RequiredValue<bool, Mode>,
        pub distance: ::data_views::Field<i64>,
        pub prefix_length: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct Bfd<'a, Mode> {
        pub echo: ::data_views::Field<bool>,
        pub interval: ::data_views::Field<i64>,
        pub min_rx: ::data_views::Field<i64>,
        pub multiplier: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct ServicePolicy<'a, Mode> {
        pub pbr: ::data_views::Field<service_policy::Pbr<'a, Mode>>,
    }

    pub mod service_policy {

        #[::data_views::data_view]
        pub struct Pbr<'a, Mode> {
            pub input: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view]
    pub struct TcpMssCeiling<'a, Mode> {
        pub ipv4: ::data_views::Field<i64>,
        pub ipv6: ::data_views::Field<i64>,
        pub direction: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view]
    pub struct TrafficPolicy<'a, Mode> {
        pub input: ::data_views::Field<&'a str>,
        pub output: ::data_views::Field<&'a str>,
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
    pub struct Metadata<'a, Mode> {
        pub tenants: ::data_views::Field<metadata::Tenants<'a, Mode>>,
        pub tags: ::data_views::Field<metadata::Tags<'a, Mode>>,
        #[data_view(rename = "type")]
        pub field_type: ::data_views::Field<&'a str>,
    }

    pub mod metadata {

        #[::data_views::data_view(list)]
        pub struct Tenants<'a, Mode> (::data_views::Field<&'a str>);

        #[::data_views::data_view(list)]
        pub struct Tags<'a, Mode> (::data_views::Field<&'a str>);
    }
}
