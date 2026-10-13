// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesignSviProfilesItems"))]
pub struct Item<'a, Mode> {
    pub profile: ::validated_data::Field<&'a str>,
    pub parent_profile: ::validated_data::Field<&'a str>,
    pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
    pub name: ::validated_data::Field<&'a str>,
    pub enabled: ::validated_data::Field<bool>,
    pub autostate: ::validated_data::Field<bool>,
    pub description: ::validated_data::Field<&'a str>,
    pub arp_gratuitous_accept: ::validated_data::Field<bool>,
    pub ip_address: ::validated_data::Field<&'a str>,
    pub ip_address_secondaries: ::validated_data::Field<item::IpAddressSecondaries<'a, Mode>>,
    pub ipv6_address: ::validated_data::Field<&'a str>,
    pub ipv6_enable: ::validated_data::Field<bool>,
    pub ip_address_virtual: ::validated_data::Field<&'a str>,
    pub ipv6_address_virtuals: ::validated_data::Field<item::Ipv6AddressVirtuals<'a, Mode>>,
    pub ipv6_nd: ::validated_data::Field<item::Ipv6Nd<'a, Mode>>,
    pub ipv6_dhcp_relay: ::validated_data::Field<item::Ipv6DhcpRelay<'a, Mode>>,
    pub ip_address_virtual_secondaries: ::validated_data::Field<item::IpAddressVirtualSecondaries<'a, Mode>>,
    pub ip_virtual_router_addresses: ::validated_data::Field<item::IpVirtualRouterAddresses<'a, Mode>>,
    pub ipv6_virtual_router_addresses: ::validated_data::Field<item::Ipv6VirtualRouterAddresses<'a, Mode>>,
    pub ipv4_acl_in: ::validated_data::Field<&'a str>,
    pub ipv4_acl_out: ::validated_data::Field<&'a str>,
    pub ipv6_acl_in: ::validated_data::Field<&'a str>,
    pub ipv6_acl_out: ::validated_data::Field<&'a str>,
    pub ip_helpers: ::validated_data::Field<item::IpHelpers<'a, Mode>>,
    pub static_routes: ::validated_data::Field<item::StaticRoutes<'a, Mode>>,
    pub ipv6_static_routes: ::validated_data::Field<item::Ipv6StaticRoutes<'a, Mode>>,
    pub vni_override: ::validated_data::Field<i64>,
    pub rt_override: ::validated_data::Field<&'a str>,
    pub rd_override: ::validated_data::Field<&'a str>,
    pub trunk_groups: ::validated_data::Field<item::TrunkGroups<'a, Mode>>,
    pub evpn_l2_multicast: ::validated_data::Field<item::EvpnL2Multicast<'a, Mode>>,
    pub evpn_redistribute_router_mac_system: ::validated_data::Field<bool>,
    pub vxlan_flood_multicast: ::validated_data::Field<item::VxlanFloodMulticast<'a, Mode>>,
    pub evpn_l3_multicast: ::validated_data::Field<item::EvpnL3Multicast<'a, Mode>>,
    pub igmp_snooping: ::validated_data::Field<item::IgmpSnooping<'a, Mode>>,
    pub igmp_snooping_enabled: ::validated_data::Field<bool>,
    pub igmp_snooping_querier: ::validated_data::Field<item::IgmpSnoopingQuerier<'a, Mode>>,
    pub vxlan: ::validated_data::Field<bool>,
    pub spanning_tree_priority: ::validated_data::Field<i64>,
    pub mtu: ::validated_data::Field<i64>,
    pub ospf: ::validated_data::Field<item::Ospf<'a, Mode>>,
    pub bgp: ::validated_data::Field<item::Bgp<'a, Mode>>,
    pub raw_eos_cli: ::validated_data::Field<&'a str>,
    #[data_view(relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_cli_config_gen::vlan_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
    pub evpn_l2_multi_domain: ::validated_data::Field<bool>,
}

pub mod item {

    #[::validated_data::data_view(indexed_list, primary_key(node), python(name = "AVDDesignSviProfilesItemsNodesList", item_name = "AVDDesignSviProfilesItemsNodesListIndexedItem"))]
    pub struct Nodes<'a, Mode> (::validated_data::Field<nodes::Item<'a, Mode>>);

    pub mod nodes {

        #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsNodesItems"))]
        pub struct Item<'a, Mode> {
            pub node: ::validated_data::Field<&'a str>,
            pub name: ::validated_data::Field<&'a str>,
            pub enabled: ::validated_data::Field<bool>,
            pub autostate: ::validated_data::Field<bool>,
            pub description: ::validated_data::Field<&'a str>,
            pub arp_gratuitous_accept: ::validated_data::Field<bool>,
            pub ip_address: ::validated_data::Field<&'a str>,
            pub ip_address_secondaries: ::validated_data::Field<item::IpAddressSecondaries<'a, Mode>>,
            pub ipv6_address: ::validated_data::Field<&'a str>,
            pub ipv6_enable: ::validated_data::Field<bool>,
            pub ip_address_virtual: ::validated_data::Field<&'a str>,
            pub ipv6_address_virtuals: ::validated_data::Field<item::Ipv6AddressVirtuals<'a, Mode>>,
            pub ipv6_nd: ::validated_data::Field<item::Ipv6Nd<'a, Mode>>,
            pub ipv6_dhcp_relay: ::validated_data::Field<item::Ipv6DhcpRelay<'a, Mode>>,
            pub ip_address_virtual_secondaries: ::validated_data::Field<item::IpAddressVirtualSecondaries<'a, Mode>>,
            pub ip_virtual_router_addresses: ::validated_data::Field<item::IpVirtualRouterAddresses<'a, Mode>>,
            pub ipv6_virtual_router_addresses: ::validated_data::Field<item::Ipv6VirtualRouterAddresses<'a, Mode>>,
            pub ipv4_acl_in: ::validated_data::Field<&'a str>,
            pub ipv4_acl_out: ::validated_data::Field<&'a str>,
            pub ipv6_acl_in: ::validated_data::Field<&'a str>,
            pub ipv6_acl_out: ::validated_data::Field<&'a str>,
            pub ip_helpers: ::validated_data::Field<item::IpHelpers<'a, Mode>>,
            pub static_routes: ::validated_data::Field<item::StaticRoutes<'a, Mode>>,
            pub ipv6_static_routes: ::validated_data::Field<item::Ipv6StaticRoutes<'a, Mode>>,
            pub vni_override: ::validated_data::Field<i64>,
            pub rt_override: ::validated_data::Field<&'a str>,
            pub rd_override: ::validated_data::Field<&'a str>,
            pub trunk_groups: ::validated_data::Field<item::TrunkGroups<'a, Mode>>,
            pub evpn_l2_multicast: ::validated_data::Field<item::EvpnL2Multicast<'a, Mode>>,
            pub evpn_redistribute_router_mac_system: ::validated_data::Field<bool>,
            pub vxlan_flood_multicast: ::validated_data::Field<item::VxlanFloodMulticast<'a, Mode>>,
            pub evpn_l3_multicast: ::validated_data::Field<item::EvpnL3Multicast<'a, Mode>>,
            pub igmp_snooping: ::validated_data::Field<item::IgmpSnooping<'a, Mode>>,
            pub igmp_snooping_enabled: ::validated_data::Field<bool>,
            pub igmp_snooping_querier: ::validated_data::Field<item::IgmpSnoopingQuerier<'a, Mode>>,
            pub vxlan: ::validated_data::Field<bool>,
            pub spanning_tree_priority: ::validated_data::Field<i64>,
            pub mtu: ::validated_data::Field<i64>,
            pub ospf: ::validated_data::Field<item::Ospf<'a, Mode>>,
            pub bgp: ::validated_data::Field<item::Bgp<'a, Mode>>,
            pub raw_eos_cli: ::validated_data::Field<&'a str>,
            #[data_view(relaxed)]
            pub structured_config: ::validated_data::Field<super::super::super::super::eos_cli_config_gen::vlan_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
            pub evpn_l2_multi_domain: ::validated_data::Field<bool>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "AVDDesignSviProfilesItemsNodesItemsIpAddressSecondariesList"))]
            pub struct IpAddressSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "AVDDesignSviProfilesItemsNodesItemsIpv6AddressVirtualsList"))]
            pub struct Ipv6AddressVirtuals<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsNodesItemsIpv6Nd"))]
            pub struct Ipv6Nd<'a, Mode> {
                pub advertise_ipv6_address_virtuals: ::validated_data::Field<bool>,
                pub valid_lifetime: ::validated_data::Field<&'a str>,
                pub preferred_lifetime: ::validated_data::Field<&'a str>,
                pub ra_dns_servers: ::validated_data::Field<ipv6_nd::RaDnsServers<'a, Mode>>,
            }

            pub mod ipv6_nd {

                #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsNodesItemsIpv6NdRaDnsServers"))]
                pub struct RaDnsServers<'a, Mode> {
                    pub servers: ::validated_data::Field<ra_dns_servers::Servers<'a, Mode>>,
                    pub dns_servers_lifetime: ::validated_data::Field<i64>,
                }

                pub mod ra_dns_servers {

                    #[::validated_data::data_view(indexed_list, primary_key(address), python(name = "AVDDesignSviProfilesItemsNodesItemsIpv6NdRaDnsServersServersList", item_name = "AVDDesignSviProfilesItemsNodesItemsIpv6NdRaDnsServersServersListIndexedItem"))]
                    pub struct Servers<'a, Mode> (::validated_data::Field<servers::Item<'a, Mode>>);

                    pub mod servers {

                        #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsNodesItemsIpv6NdRaDnsServersServersItems"))]
                        pub struct Item<'a, Mode> {
                            pub address: ::validated_data::Field<&'a str>,
                            pub lifetime: ::validated_data::Field<i64>,
                        }
                    }
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsNodesItemsIpv6DhcpRelay"))]
            pub struct Ipv6DhcpRelay<'a, Mode> {
                pub destinations: ::validated_data::Field<ipv6_dhcp_relay::Destinations<'a, Mode>>,
            }

            pub mod ipv6_dhcp_relay {

                #[::validated_data::data_view(indexed_list, primary_key(address), python(name = "AVDDesignSviProfilesItemsNodesItemsIpv6DhcpRelayDestinationsList", item_name = "AVDDesignSviProfilesItemsNodesItemsIpv6DhcpRelayDestinationsListIndexedItem"))]
                pub struct Destinations<'a, Mode> (::validated_data::Field<destinations::Item<'a, Mode>>);

                pub mod destinations {

                    #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsNodesItemsIpv6DhcpRelayDestinationsItems"))]
                    pub struct Item<'a, Mode> {
                        pub address: ::validated_data::Field<&'a str>,
                        pub vrf: ::validated_data::Field<&'a str>,
                        pub local_interface: ::validated_data::Field<&'a str>,
                        pub source_address: ::validated_data::Field<&'a str>,
                        pub link_address: ::validated_data::Field<&'a str>,
                    }
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesignSviProfilesItemsNodesItemsIpAddressVirtualSecondariesList"))]
            pub struct IpAddressVirtualSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "AVDDesignSviProfilesItemsNodesItemsIpVirtualRouterAddressesList"))]
            pub struct IpVirtualRouterAddresses<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "AVDDesignSviProfilesItemsNodesItemsIpv6VirtualRouterAddressesList"))]
            pub struct Ipv6VirtualRouterAddresses<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(indexed_list, primary_key(ip_helper), python(name = "AVDDesignSviProfilesItemsNodesItemsIpHelpersList", item_name = "AVDDesignSviProfilesItemsNodesItemsIpHelpersListIndexedItem"))]
            pub struct IpHelpers<'a, Mode> (::validated_data::Field<ip_helpers::Item<'a, Mode>>);

            pub mod ip_helpers {

                #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsNodesItemsIpHelpersItems"))]
                pub struct Item<'a, Mode> {
                    pub ip_helper: ::validated_data::Field<&'a str>,
                    pub source_interface: ::validated_data::Field<&'a str>,
                    pub source_vrf: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesignSviProfilesItemsNodesItemsStaticRoutesList"))]
            pub struct StaticRoutes<'a, Mode> (::validated_data::Field<static_routes::Item<'a, Mode>>);

            pub mod static_routes {

                #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsNodesItemsStaticRoutesItems"))]
                pub struct Item<'a, Mode> {
                    pub prefix: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub next_hop: ::validated_data::Field<&'a str>,
                    pub track_bfd: ::validated_data::Field<bool>,
                    pub distance: ::validated_data::Field<i64>,
                    pub tag: ::validated_data::Field<i64>,
                    pub name: ::validated_data::Field<&'a str>,
                    pub metric: ::validated_data::Field<i64>,
                    pub interface: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesignSviProfilesItemsNodesItemsIpv6StaticRoutesList"))]
            pub struct Ipv6StaticRoutes<'a, Mode> (::validated_data::Field<ipv6_static_routes::Item<'a, Mode>>);

            pub mod ipv6_static_routes {

                #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsNodesItemsIpv6StaticRoutesItems"))]
                pub struct Item<'a, Mode> {
                    pub prefix: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub next_hop: ::validated_data::Field<&'a str>,
                    pub track_bfd: ::validated_data::Field<bool>,
                    pub distance: ::validated_data::Field<i64>,
                    pub tag: ::validated_data::Field<i64>,
                    pub name: ::validated_data::Field<&'a str>,
                    pub metric: ::validated_data::Field<i64>,
                    pub interface: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesignSviProfilesItemsNodesItemsTrunkGroupsList"))]
            pub struct TrunkGroups<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsNodesItemsEvpnL2Multicast"))]
            pub struct EvpnL2Multicast<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub always_redistribute_igmp: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsNodesItemsVxlanFloodMulticast"))]
            pub struct VxlanFloodMulticast<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub underlay_multicast_group: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsNodesItemsEvpnL3Multicast"))]
            pub struct EvpnL3Multicast<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsNodesItemsIgmpSnooping"))]
            pub struct IgmpSnooping<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub querier: ::validated_data::Field<igmp_snooping::Querier<'a, Mode>>,
                pub fast_leave: ::validated_data::Field<bool>,
            }

            pub mod igmp_snooping {

                #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsNodesItemsIgmpSnoopingQuerier"))]
                pub struct Querier<'a, Mode> {
                    pub enabled: ::validated_data::Field<bool>,
                    pub source_address: ::validated_data::Field<&'a str>,
                    pub version: ::validated_data::Field<i64>,
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsNodesItemsIgmpSnoopingQuerier2"))]
            pub struct IgmpSnoopingQuerier<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub source_address: ::validated_data::Field<&'a str>,
                pub version: ::validated_data::Field<i64>,
                pub fast_leave: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsNodesItemsOspf"))]
            pub struct Ospf<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub point_to_point: ::validated_data::Field<bool>,
                pub area: ::validated_data::Field<&'a str>,
                pub cost: ::validated_data::Field<i64>,
                pub authentication: ::validated_data::Field<&'a str>,
                pub simple_auth_key: ::validated_data::Field<&'a str>,
                pub cleartext_simple_auth_key: ::validated_data::Field<&'a str>,
                pub message_digest_keys: ::validated_data::Field<ospf::MessageDigestKeys<'a, Mode>>,
            }

            pub mod ospf {

                #[::validated_data::data_view(indexed_list, primary_key(id), python(name = "AVDDesignSviProfilesItemsNodesItemsOspfMessageDigestKeysList", item_name = "AVDDesignSviProfilesItemsNodesItemsOspfMessageDigestKeysListIndexedItem"))]
                pub struct MessageDigestKeys<'a, Mode> (::validated_data::Field<message_digest_keys::Item<'a, Mode>>);

                pub mod message_digest_keys {

                    #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsNodesItemsOspfMessageDigestKeysItems"))]
                    pub struct Item<'a, Mode> {
                        pub id: ::validated_data::Field<i64>,
                        pub hash_algorithm: ::validated_data::Field<&'a str>,
                        pub key: ::validated_data::Field<&'a str>,
                        pub cleartext_key: ::validated_data::Field<&'a str>,
                    }
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsNodesItemsBgp"))]
            pub struct Bgp<'a, Mode> {
                #[data_view(relaxed)]
                pub structured_config: ::validated_data::Field<super::super::super::super::super::eos_cli_config_gen::router_bgp::vlans::Item<'a, ::validated_data::RelaxedValidated>>,
                pub raw_eos_cli: ::validated_data::Field<&'a str>,
            }

            pub mod bgp {
            }
        }
    }

    #[::validated_data::data_view(list, python(name = "AVDDesignSviProfilesItemsIpAddressSecondariesList"))]
    pub struct IpAddressSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list, python(name = "AVDDesignSviProfilesItemsIpv6AddressVirtualsList"))]
    pub struct Ipv6AddressVirtuals<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsIpv6Nd"))]
    pub struct Ipv6Nd<'a, Mode> {
        pub advertise_ipv6_address_virtuals: ::validated_data::Field<bool>,
        pub valid_lifetime: ::validated_data::Field<&'a str>,
        pub preferred_lifetime: ::validated_data::Field<&'a str>,
        pub ra_dns_servers: ::validated_data::Field<ipv6_nd::RaDnsServers<'a, Mode>>,
    }

    pub mod ipv6_nd {

        #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsIpv6NdRaDnsServers"))]
        pub struct RaDnsServers<'a, Mode> {
            pub servers: ::validated_data::Field<ra_dns_servers::Servers<'a, Mode>>,
            pub dns_servers_lifetime: ::validated_data::Field<i64>,
        }

        pub mod ra_dns_servers {

            #[::validated_data::data_view(indexed_list, primary_key(address), python(name = "AVDDesignSviProfilesItemsIpv6NdRaDnsServersServersList", item_name = "AVDDesignSviProfilesItemsIpv6NdRaDnsServersServersListIndexedItem"))]
            pub struct Servers<'a, Mode> (::validated_data::Field<servers::Item<'a, Mode>>);

            pub mod servers {

                #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsIpv6NdRaDnsServersServersItems"))]
                pub struct Item<'a, Mode> {
                    pub address: ::validated_data::Field<&'a str>,
                    pub lifetime: ::validated_data::Field<i64>,
                }
            }
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsIpv6DhcpRelay"))]
    pub struct Ipv6DhcpRelay<'a, Mode> {
        pub destinations: ::validated_data::Field<ipv6_dhcp_relay::Destinations<'a, Mode>>,
    }

    pub mod ipv6_dhcp_relay {

        #[::validated_data::data_view(indexed_list, primary_key(address), python(name = "AVDDesignSviProfilesItemsIpv6DhcpRelayDestinationsList", item_name = "AVDDesignSviProfilesItemsIpv6DhcpRelayDestinationsListIndexedItem"))]
        pub struct Destinations<'a, Mode> (::validated_data::Field<destinations::Item<'a, Mode>>);

        pub mod destinations {

            #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsIpv6DhcpRelayDestinationsItems"))]
            pub struct Item<'a, Mode> {
                pub address: ::validated_data::Field<&'a str>,
                pub vrf: ::validated_data::Field<&'a str>,
                pub local_interface: ::validated_data::Field<&'a str>,
                pub source_address: ::validated_data::Field<&'a str>,
                pub link_address: ::validated_data::Field<&'a str>,
            }
        }
    }

    #[::validated_data::data_view(list, python(name = "AVDDesignSviProfilesItemsIpAddressVirtualSecondariesList"))]
    pub struct IpAddressVirtualSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list, python(name = "AVDDesignSviProfilesItemsIpVirtualRouterAddressesList"))]
    pub struct IpVirtualRouterAddresses<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list, python(name = "AVDDesignSviProfilesItemsIpv6VirtualRouterAddressesList"))]
    pub struct Ipv6VirtualRouterAddresses<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(indexed_list, primary_key(ip_helper), python(name = "AVDDesignSviProfilesItemsIpHelpersList", item_name = "AVDDesignSviProfilesItemsIpHelpersListIndexedItem"))]
    pub struct IpHelpers<'a, Mode> (::validated_data::Field<ip_helpers::Item<'a, Mode>>);

    pub mod ip_helpers {

        #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsIpHelpersItems"))]
        pub struct Item<'a, Mode> {
            pub ip_helper: ::validated_data::Field<&'a str>,
            pub source_interface: ::validated_data::Field<&'a str>,
            pub source_vrf: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(list, python(name = "AVDDesignSviProfilesItemsStaticRoutesList"))]
    pub struct StaticRoutes<'a, Mode> (::validated_data::Field<static_routes::Item<'a, Mode>>);

    pub mod static_routes {

        #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsStaticRoutesItems"))]
        pub struct Item<'a, Mode> {
            pub prefix: ::validated_data::RequiredValue<&'a str, Mode>,
            pub next_hop: ::validated_data::Field<&'a str>,
            pub track_bfd: ::validated_data::Field<bool>,
            pub distance: ::validated_data::Field<i64>,
            pub tag: ::validated_data::Field<i64>,
            pub name: ::validated_data::Field<&'a str>,
            pub metric: ::validated_data::Field<i64>,
            pub interface: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(list, python(name = "AVDDesignSviProfilesItemsIpv6StaticRoutesList"))]
    pub struct Ipv6StaticRoutes<'a, Mode> (::validated_data::Field<ipv6_static_routes::Item<'a, Mode>>);

    pub mod ipv6_static_routes {

        #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsIpv6StaticRoutesItems"))]
        pub struct Item<'a, Mode> {
            pub prefix: ::validated_data::RequiredValue<&'a str, Mode>,
            pub next_hop: ::validated_data::Field<&'a str>,
            pub track_bfd: ::validated_data::Field<bool>,
            pub distance: ::validated_data::Field<i64>,
            pub tag: ::validated_data::Field<i64>,
            pub name: ::validated_data::Field<&'a str>,
            pub metric: ::validated_data::Field<i64>,
            pub interface: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(list, python(name = "AVDDesignSviProfilesItemsTrunkGroupsList"))]
    pub struct TrunkGroups<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsEvpnL2Multicast"))]
    pub struct EvpnL2Multicast<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub always_redistribute_igmp: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsVxlanFloodMulticast"))]
    pub struct VxlanFloodMulticast<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub underlay_multicast_group: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsEvpnL3Multicast"))]
    pub struct EvpnL3Multicast<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsIgmpSnooping"))]
    pub struct IgmpSnooping<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub querier: ::validated_data::Field<igmp_snooping::Querier<'a, Mode>>,
        pub fast_leave: ::validated_data::Field<bool>,
    }

    pub mod igmp_snooping {

        #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsIgmpSnoopingQuerier"))]
        pub struct Querier<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub source_address: ::validated_data::Field<&'a str>,
            pub version: ::validated_data::Field<i64>,
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsIgmpSnoopingQuerier2"))]
    pub struct IgmpSnoopingQuerier<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub source_address: ::validated_data::Field<&'a str>,
        pub version: ::validated_data::Field<i64>,
        pub fast_leave: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsOspf"))]
    pub struct Ospf<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub point_to_point: ::validated_data::Field<bool>,
        pub area: ::validated_data::Field<&'a str>,
        pub cost: ::validated_data::Field<i64>,
        pub authentication: ::validated_data::Field<&'a str>,
        pub simple_auth_key: ::validated_data::Field<&'a str>,
        pub cleartext_simple_auth_key: ::validated_data::Field<&'a str>,
        pub message_digest_keys: ::validated_data::Field<ospf::MessageDigestKeys<'a, Mode>>,
    }

    pub mod ospf {

        #[::validated_data::data_view(indexed_list, primary_key(id), python(name = "AVDDesignSviProfilesItemsOspfMessageDigestKeysList", item_name = "AVDDesignSviProfilesItemsOspfMessageDigestKeysListIndexedItem"))]
        pub struct MessageDigestKeys<'a, Mode> (::validated_data::Field<message_digest_keys::Item<'a, Mode>>);

        pub mod message_digest_keys {

            #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsOspfMessageDigestKeysItems"))]
            pub struct Item<'a, Mode> {
                pub id: ::validated_data::Field<i64>,
                pub hash_algorithm: ::validated_data::Field<&'a str>,
                pub key: ::validated_data::Field<&'a str>,
                pub cleartext_key: ::validated_data::Field<&'a str>,
            }
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesignSviProfilesItemsBgp"))]
    pub struct Bgp<'a, Mode> {
        #[data_view(relaxed)]
        pub structured_config: ::validated_data::Field<super::super::super::eos_cli_config_gen::router_bgp::vlans::Item<'a, ::validated_data::RelaxedValidated>>,
        pub raw_eos_cli: ::validated_data::Field<&'a str>,
    }

    pub mod bgp {
    }
}
