// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase"))]
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
    pub structured_config: ::validated_data::Field<super::super::eos_config::vlan_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
    pub evpn_l2_multi_domain: ::validated_data::Field<bool>,
}

pub mod item {

    #[::validated_data::data_view(indexed_list, primary_key(node), python(name = "AVDDesign.SviProfilesItemBase.Nodes", item_name = "AVDDesign.SviProfilesItemBase.NodesItem"))]
    pub struct Nodes<'a, Mode> (::validated_data::Field<nodes::Item<'a, Mode>>);

    pub mod nodes {

        #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase"))]
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
            pub structured_config: ::validated_data::Field<super::super::super::super::eos_config::vlan_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
            pub evpn_l2_multi_domain: ::validated_data::Field<bool>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IpAddressSecondaries"))]
            pub struct IpAddressSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6AddressVirtuals"))]
            pub struct Ipv6AddressVirtuals<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6Nd"))]
            pub struct Ipv6Nd<'a, Mode> {
                pub advertise_ipv6_address_virtuals: ::validated_data::Field<bool>,
                pub valid_lifetime: ::validated_data::Field<&'a str>,
                pub preferred_lifetime: ::validated_data::Field<&'a str>,
                pub ra_dns_servers: ::validated_data::Field<ipv6_nd::RaDnsServers<'a, Mode>>,
            }

            pub mod ipv6_nd {

                #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6Nd.RaDnsServers"))]
                pub struct RaDnsServers<'a, Mode> {
                    pub servers: ::validated_data::Field<ra_dns_servers::Servers<'a, Mode>>,
                    pub dns_servers_lifetime: ::validated_data::Field<i64>,
                }

                pub mod ra_dns_servers {

                    #[::validated_data::data_view(indexed_list, primary_key(address), python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6Nd.RaDnsServers.Servers", item_name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6Nd.RaDnsServers.ServersItem"))]
                    pub struct Servers<'a, Mode> (::validated_data::Field<servers::Item<'a, Mode>>);

                    pub mod servers {

                        #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6Nd.RaDnsServers.ServersItemBase"))]
                        pub struct Item<'a, Mode> {
                            pub address: ::validated_data::Field<&'a str>,
                            pub lifetime: ::validated_data::Field<i64>,
                        }
                    }
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6DhcpRelay"))]
            pub struct Ipv6DhcpRelay<'a, Mode> {
                pub destinations: ::validated_data::Field<ipv6_dhcp_relay::Destinations<'a, Mode>>,
            }

            pub mod ipv6_dhcp_relay {

                #[::validated_data::data_view(indexed_list, primary_key(address), python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6DhcpRelay.Destinations", item_name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6DhcpRelay.DestinationsItem"))]
                pub struct Destinations<'a, Mode> (::validated_data::Field<destinations::Item<'a, Mode>>);

                pub mod destinations {

                    #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6DhcpRelay.DestinationsItemBase"))]
                    pub struct Item<'a, Mode> {
                        pub address: ::validated_data::Field<&'a str>,
                        pub vrf: ::validated_data::Field<&'a str>,
                        pub local_interface: ::validated_data::Field<&'a str>,
                        pub source_address: ::validated_data::Field<&'a str>,
                        pub link_address: ::validated_data::Field<&'a str>,
                    }
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IpAddressVirtualSecondaries"))]
            pub struct IpAddressVirtualSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IpVirtualRouterAddresses"))]
            pub struct IpVirtualRouterAddresses<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6VirtualRouterAddresses"))]
            pub struct Ipv6VirtualRouterAddresses<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(indexed_list, primary_key(ip_helper), python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IpHelpers", item_name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IpHelpersItem"))]
            pub struct IpHelpers<'a, Mode> (::validated_data::Field<ip_helpers::Item<'a, Mode>>);

            pub mod ip_helpers {

                #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IpHelpersItemBase"))]
                pub struct Item<'a, Mode> {
                    pub ip_helper: ::validated_data::Field<&'a str>,
                    pub source_interface: ::validated_data::Field<&'a str>,
                    pub source_vrf: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.StaticRoutes"))]
            pub struct StaticRoutes<'a, Mode> (::validated_data::Field<static_routes::Item<'a, Mode>>);

            pub mod static_routes {

                #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.StaticRoutesItem"))]
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

            #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6StaticRoutes"))]
            pub struct Ipv6StaticRoutes<'a, Mode> (::validated_data::Field<ipv6_static_routes::Item<'a, Mode>>);

            pub mod ipv6_static_routes {

                #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6StaticRoutesItem"))]
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

            #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.TrunkGroups"))]
            pub struct TrunkGroups<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.EvpnL2Multicast"))]
            pub struct EvpnL2Multicast<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub always_redistribute_igmp: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.VxlanFloodMulticast"))]
            pub struct VxlanFloodMulticast<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub underlay_multicast_group: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.EvpnL3Multicast"))]
            pub struct EvpnL3Multicast<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IgmpSnooping"))]
            pub struct IgmpSnooping<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub querier: ::validated_data::Field<igmp_snooping::Querier<'a, Mode>>,
                pub fast_leave: ::validated_data::Field<bool>,
            }

            pub mod igmp_snooping {

                #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IgmpSnooping.Querier"))]
                pub struct Querier<'a, Mode> {
                    pub enabled: ::validated_data::Field<bool>,
                    pub source_address: ::validated_data::Field<&'a str>,
                    pub version: ::validated_data::Field<i64>,
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IgmpSnoopingQuerier"))]
            pub struct IgmpSnoopingQuerier<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub source_address: ::validated_data::Field<&'a str>,
                pub version: ::validated_data::Field<i64>,
                pub fast_leave: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ospf"))]
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

                #[::validated_data::data_view(indexed_list, primary_key(id), python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ospf.MessageDigestKeys", item_name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ospf.MessageDigestKeysItem"))]
                pub struct MessageDigestKeys<'a, Mode> (::validated_data::Field<message_digest_keys::Item<'a, Mode>>);

                pub mod message_digest_keys {

                    #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ospf.MessageDigestKeysItemBase"))]
                    pub struct Item<'a, Mode> {
                        pub id: ::validated_data::Field<i64>,
                        pub hash_algorithm: ::validated_data::Field<&'a str>,
                        pub key: ::validated_data::Field<&'a str>,
                        pub cleartext_key: ::validated_data::Field<&'a str>,
                    }
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Bgp"))]
            pub struct Bgp<'a, Mode> {
                #[data_view(relaxed)]
                pub structured_config: ::validated_data::Field<super::super::super::super::super::eos_config::router_bgp::vlans::Item<'a, ::validated_data::RelaxedValidated>>,
                pub raw_eos_cli: ::validated_data::Field<&'a str>,
            }

            pub mod bgp {
            }
        }
    }

    #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.IpAddressSecondaries"))]
    pub struct IpAddressSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.Ipv6AddressVirtuals"))]
    pub struct Ipv6AddressVirtuals<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.Ipv6Nd"))]
    pub struct Ipv6Nd<'a, Mode> {
        pub advertise_ipv6_address_virtuals: ::validated_data::Field<bool>,
        pub valid_lifetime: ::validated_data::Field<&'a str>,
        pub preferred_lifetime: ::validated_data::Field<&'a str>,
        pub ra_dns_servers: ::validated_data::Field<ipv6_nd::RaDnsServers<'a, Mode>>,
    }

    pub mod ipv6_nd {

        #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.Ipv6Nd.RaDnsServers"))]
        pub struct RaDnsServers<'a, Mode> {
            pub servers: ::validated_data::Field<ra_dns_servers::Servers<'a, Mode>>,
            pub dns_servers_lifetime: ::validated_data::Field<i64>,
        }

        pub mod ra_dns_servers {

            #[::validated_data::data_view(indexed_list, primary_key(address), python(name = "AVDDesign.SviProfilesItemBase.Ipv6Nd.RaDnsServers.Servers", item_name = "AVDDesign.SviProfilesItemBase.Ipv6Nd.RaDnsServers.ServersItem"))]
            pub struct Servers<'a, Mode> (::validated_data::Field<servers::Item<'a, Mode>>);

            pub mod servers {

                #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.Ipv6Nd.RaDnsServers.ServersItemBase"))]
                pub struct Item<'a, Mode> {
                    pub address: ::validated_data::Field<&'a str>,
                    pub lifetime: ::validated_data::Field<i64>,
                }
            }
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.Ipv6DhcpRelay"))]
    pub struct Ipv6DhcpRelay<'a, Mode> {
        pub destinations: ::validated_data::Field<ipv6_dhcp_relay::Destinations<'a, Mode>>,
    }

    pub mod ipv6_dhcp_relay {

        #[::validated_data::data_view(indexed_list, primary_key(address), python(name = "AVDDesign.SviProfilesItemBase.Ipv6DhcpRelay.Destinations", item_name = "AVDDesign.SviProfilesItemBase.Ipv6DhcpRelay.DestinationsItem"))]
        pub struct Destinations<'a, Mode> (::validated_data::Field<destinations::Item<'a, Mode>>);

        pub mod destinations {

            #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.Ipv6DhcpRelay.DestinationsItemBase"))]
            pub struct Item<'a, Mode> {
                pub address: ::validated_data::Field<&'a str>,
                pub vrf: ::validated_data::Field<&'a str>,
                pub local_interface: ::validated_data::Field<&'a str>,
                pub source_address: ::validated_data::Field<&'a str>,
                pub link_address: ::validated_data::Field<&'a str>,
            }
        }
    }

    #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.IpAddressVirtualSecondaries"))]
    pub struct IpAddressVirtualSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.IpVirtualRouterAddresses"))]
    pub struct IpVirtualRouterAddresses<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.Ipv6VirtualRouterAddresses"))]
    pub struct Ipv6VirtualRouterAddresses<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(indexed_list, primary_key(ip_helper), python(name = "AVDDesign.SviProfilesItemBase.IpHelpers", item_name = "AVDDesign.SviProfilesItemBase.IpHelpersItem"))]
    pub struct IpHelpers<'a, Mode> (::validated_data::Field<ip_helpers::Item<'a, Mode>>);

    pub mod ip_helpers {

        #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.IpHelpersItemBase"))]
        pub struct Item<'a, Mode> {
            pub ip_helper: ::validated_data::Field<&'a str>,
            pub source_interface: ::validated_data::Field<&'a str>,
            pub source_vrf: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.StaticRoutes"))]
    pub struct StaticRoutes<'a, Mode> (::validated_data::Field<static_routes::Item<'a, Mode>>);

    pub mod static_routes {

        #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.StaticRoutesItem"))]
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

    #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.Ipv6StaticRoutes"))]
    pub struct Ipv6StaticRoutes<'a, Mode> (::validated_data::Field<ipv6_static_routes::Item<'a, Mode>>);

    pub mod ipv6_static_routes {

        #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.Ipv6StaticRoutesItem"))]
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

    #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.TrunkGroups"))]
    pub struct TrunkGroups<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.EvpnL2Multicast"))]
    pub struct EvpnL2Multicast<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub always_redistribute_igmp: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.VxlanFloodMulticast"))]
    pub struct VxlanFloodMulticast<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub underlay_multicast_group: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.EvpnL3Multicast"))]
    pub struct EvpnL3Multicast<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.IgmpSnooping"))]
    pub struct IgmpSnooping<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub querier: ::validated_data::Field<igmp_snooping::Querier<'a, Mode>>,
        pub fast_leave: ::validated_data::Field<bool>,
    }

    pub mod igmp_snooping {

        #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.IgmpSnooping.Querier"))]
        pub struct Querier<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub source_address: ::validated_data::Field<&'a str>,
            pub version: ::validated_data::Field<i64>,
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.IgmpSnoopingQuerier"))]
    pub struct IgmpSnoopingQuerier<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub source_address: ::validated_data::Field<&'a str>,
        pub version: ::validated_data::Field<i64>,
        pub fast_leave: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.Ospf"))]
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

        #[::validated_data::data_view(indexed_list, primary_key(id), python(name = "AVDDesign.SviProfilesItemBase.Ospf.MessageDigestKeys", item_name = "AVDDesign.SviProfilesItemBase.Ospf.MessageDigestKeysItem"))]
        pub struct MessageDigestKeys<'a, Mode> (::validated_data::Field<message_digest_keys::Item<'a, Mode>>);

        pub mod message_digest_keys {

            #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.Ospf.MessageDigestKeysItemBase"))]
            pub struct Item<'a, Mode> {
                pub id: ::validated_data::Field<i64>,
                pub hash_algorithm: ::validated_data::Field<&'a str>,
                pub key: ::validated_data::Field<&'a str>,
                pub cleartext_key: ::validated_data::Field<&'a str>,
            }
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.Bgp"))]
    pub struct Bgp<'a, Mode> {
        #[data_view(relaxed)]
        pub structured_config: ::validated_data::Field<super::super::super::eos_config::router_bgp::vlans::Item<'a, ::validated_data::RelaxedValidated>>,
        pub raw_eos_cli: ::validated_data::Field<&'a str>,
    }

    pub mod bgp {
    }
}
