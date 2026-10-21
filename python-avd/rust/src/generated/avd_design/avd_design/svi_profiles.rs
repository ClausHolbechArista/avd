// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(indexed_list, primary_key(profile), python(name = "AVDDesign.SviProfiles", schema = List(781), item_name = "AVDDesign.SviProfilesItem"))]
pub struct SviProfiles<'a, Mode> (::validated_data::Field<Item<'a, Mode>>);

#[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase", schema = Dict(1763)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(608))]
    pub profile: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1642))]
    pub parent_profile: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(780))]
    pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
    #[data_view(schema = Str(1511))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(579))]
    pub enabled: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(580))]
    pub autostate: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1512))]
    pub description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(124))]
    pub arp_gratuitous_accept: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1513))]
    pub ip_address: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(222))]
    pub ip_address_secondaries: ::validated_data::Field<item::IpAddressSecondaries<'a, Mode>>,
    #[data_view(schema = Str(1514))]
    pub ipv6_address: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(581))]
    pub ipv6_enable: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1515))]
    pub ip_address_virtual: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(706))]
    pub ipv6_address_virtuals: ::validated_data::Field<item::Ipv6AddressVirtuals<'a, Mode>>,
    #[data_view(schema = Dict(1660))]
    pub ipv6_nd: ::validated_data::Field<item::Ipv6Nd<'a, Mode>>,
    #[data_view(schema = Dict(1662))]
    pub ipv6_dhcp_relay: ::validated_data::Field<item::Ipv6DhcpRelay<'a, Mode>>,
    #[data_view(schema = List(708))]
    pub ip_address_virtual_secondaries: ::validated_data::Field<item::IpAddressVirtualSecondaries<'a, Mode>>,
    #[data_view(schema = List(709))]
    pub ip_virtual_router_addresses: ::validated_data::Field<item::IpVirtualRouterAddresses<'a, Mode>>,
    #[data_view(schema = List(710))]
    pub ipv6_virtual_router_addresses: ::validated_data::Field<item::Ipv6VirtualRouterAddresses<'a, Mode>>,
    #[data_view(schema = Str(1520))]
    pub ipv4_acl_in: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1521))]
    pub ipv4_acl_out: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1522))]
    pub ipv6_acl_in: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1523))]
    pub ipv6_acl_out: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(697))]
    pub ip_helpers: ::validated_data::Field<item::IpHelpers<'a, Mode>>,
    #[data_view(schema = List(711))]
    pub static_routes: ::validated_data::Field<item::StaticRoutes<'a, Mode>>,
    #[data_view(schema = List(712))]
    pub ipv6_static_routes: ::validated_data::Field<item::Ipv6StaticRoutes<'a, Mode>>,
    #[data_view(schema = Int(610))]
    pub vni_override: ::validated_data::Field<i64>,
    #[data_view(schema = Str(1525))]
    pub rt_override: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1526))]
    pub rd_override: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(713))]
    pub trunk_groups: ::validated_data::Field<item::TrunkGroups<'a, Mode>>,
    #[data_view(schema = Dict(1664))]
    pub evpn_l2_multicast: ::validated_data::Field<item::EvpnL2Multicast<'a, Mode>>,
    #[data_view(schema = Bool(584))]
    pub evpn_redistribute_router_mac_system: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1665))]
    pub vxlan_flood_multicast: ::validated_data::Field<item::VxlanFloodMulticast<'a, Mode>>,
    #[data_view(schema = Dict(1666))]
    pub evpn_l3_multicast: ::validated_data::Field<item::EvpnL3Multicast<'a, Mode>>,
    #[data_view(schema = Dict(1608))]
    pub igmp_snooping: ::validated_data::Field<item::IgmpSnooping<'a, Mode>>,
    #[data_view(schema = Bool(586))]
    pub igmp_snooping_enabled: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1667))]
    pub igmp_snooping_querier: ::validated_data::Field<item::IgmpSnoopingQuerier<'a, Mode>>,
    #[data_view(schema = Bool(587))]
    pub vxlan: ::validated_data::Field<bool>,
    #[data_view(schema = Int(594))]
    pub spanning_tree_priority: ::validated_data::Field<i64>,
    #[data_view(schema = Int(611))]
    pub mtu: ::validated_data::Field<i64>,
    #[data_view(schema = Dict(1669))]
    pub ospf: ::validated_data::Field<item::Ospf<'a, Mode>>,
    #[data_view(schema = Dict(1671))]
    pub bgp: ::validated_data::Field<item::Bgp<'a, Mode>>,
    #[data_view(schema = Str(1536))]
    pub raw_eos_cli: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1672), relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_config::vlan_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
    #[data_view(schema = Bool(588))]
    pub evpn_l2_multi_domain: ::validated_data::Field<bool>,
}

pub mod item {

    #[::validated_data::data_view(indexed_list, primary_key(node), python(name = "AVDDesign.SviProfilesItemBase.Nodes", schema = List(780), item_name = "AVDDesign.SviProfilesItemBase.NodesItem"))]
    pub struct Nodes<'a, Mode> (::validated_data::Field<nodes::Item<'a, Mode>>);

    pub mod nodes {

        #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase", schema = Dict(1762)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(1510))]
            pub node: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1511))]
            pub name: ::validated_data::Field<&'a str>,
            #[data_view(schema = Bool(579))]
            pub enabled: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(580))]
            pub autostate: ::validated_data::Field<bool>,
            #[data_view(schema = Str(1512))]
            pub description: ::validated_data::Field<&'a str>,
            #[data_view(schema = Bool(124))]
            pub arp_gratuitous_accept: ::validated_data::Field<bool>,
            #[data_view(schema = Str(1513))]
            pub ip_address: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(222))]
            pub ip_address_secondaries: ::validated_data::Field<item::IpAddressSecondaries<'a, Mode>>,
            #[data_view(schema = Str(1514))]
            pub ipv6_address: ::validated_data::Field<&'a str>,
            #[data_view(schema = Bool(581))]
            pub ipv6_enable: ::validated_data::Field<bool>,
            #[data_view(schema = Str(1515))]
            pub ip_address_virtual: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(706))]
            pub ipv6_address_virtuals: ::validated_data::Field<item::Ipv6AddressVirtuals<'a, Mode>>,
            #[data_view(schema = Dict(1660))]
            pub ipv6_nd: ::validated_data::Field<item::Ipv6Nd<'a, Mode>>,
            #[data_view(schema = Dict(1662))]
            pub ipv6_dhcp_relay: ::validated_data::Field<item::Ipv6DhcpRelay<'a, Mode>>,
            #[data_view(schema = List(708))]
            pub ip_address_virtual_secondaries: ::validated_data::Field<item::IpAddressVirtualSecondaries<'a, Mode>>,
            #[data_view(schema = List(709))]
            pub ip_virtual_router_addresses: ::validated_data::Field<item::IpVirtualRouterAddresses<'a, Mode>>,
            #[data_view(schema = List(710))]
            pub ipv6_virtual_router_addresses: ::validated_data::Field<item::Ipv6VirtualRouterAddresses<'a, Mode>>,
            #[data_view(schema = Str(1520))]
            pub ipv4_acl_in: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1521))]
            pub ipv4_acl_out: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1522))]
            pub ipv6_acl_in: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1523))]
            pub ipv6_acl_out: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(697))]
            pub ip_helpers: ::validated_data::Field<item::IpHelpers<'a, Mode>>,
            #[data_view(schema = List(711))]
            pub static_routes: ::validated_data::Field<item::StaticRoutes<'a, Mode>>,
            #[data_view(schema = List(712))]
            pub ipv6_static_routes: ::validated_data::Field<item::Ipv6StaticRoutes<'a, Mode>>,
            #[data_view(schema = Int(610))]
            pub vni_override: ::validated_data::Field<i64>,
            #[data_view(schema = Str(1525))]
            pub rt_override: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1526))]
            pub rd_override: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(713))]
            pub trunk_groups: ::validated_data::Field<item::TrunkGroups<'a, Mode>>,
            #[data_view(schema = Dict(1664))]
            pub evpn_l2_multicast: ::validated_data::Field<item::EvpnL2Multicast<'a, Mode>>,
            #[data_view(schema = Bool(584))]
            pub evpn_redistribute_router_mac_system: ::validated_data::Field<bool>,
            #[data_view(schema = Dict(1665))]
            pub vxlan_flood_multicast: ::validated_data::Field<item::VxlanFloodMulticast<'a, Mode>>,
            #[data_view(schema = Dict(1666))]
            pub evpn_l3_multicast: ::validated_data::Field<item::EvpnL3Multicast<'a, Mode>>,
            #[data_view(schema = Dict(1608))]
            pub igmp_snooping: ::validated_data::Field<item::IgmpSnooping<'a, Mode>>,
            #[data_view(schema = Bool(586))]
            pub igmp_snooping_enabled: ::validated_data::Field<bool>,
            #[data_view(schema = Dict(1667))]
            pub igmp_snooping_querier: ::validated_data::Field<item::IgmpSnoopingQuerier<'a, Mode>>,
            #[data_view(schema = Bool(587))]
            pub vxlan: ::validated_data::Field<bool>,
            #[data_view(schema = Int(594))]
            pub spanning_tree_priority: ::validated_data::Field<i64>,
            #[data_view(schema = Int(611))]
            pub mtu: ::validated_data::Field<i64>,
            #[data_view(schema = Dict(1669))]
            pub ospf: ::validated_data::Field<item::Ospf<'a, Mode>>,
            #[data_view(schema = Dict(1671))]
            pub bgp: ::validated_data::Field<item::Bgp<'a, Mode>>,
            #[data_view(schema = Str(1536))]
            pub raw_eos_cli: ::validated_data::Field<&'a str>,
            #[data_view(schema = Dict(1672), relaxed)]
            pub structured_config: ::validated_data::Field<super::super::super::super::eos_config::vlan_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
            #[data_view(schema = Bool(588))]
            pub evpn_l2_multi_domain: ::validated_data::Field<bool>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IpAddressSecondaries", schema = List(222)))]
            pub struct IpAddressSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6AddressVirtuals", schema = List(706)))]
            pub struct Ipv6AddressVirtuals<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6Nd", schema = Dict(1660)))]
            pub struct Ipv6Nd<'a, Mode> {
                #[data_view(schema = Bool(582))]
                pub advertise_ipv6_address_virtuals: ::validated_data::Field<bool>,
                #[data_view(schema = Str(1051))]
                pub valid_lifetime: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(1051))]
                pub preferred_lifetime: ::validated_data::Field<&'a str>,
                #[data_view(schema = Dict(1659))]
                pub ra_dns_servers: ::validated_data::Field<ipv6_nd::RaDnsServers<'a, Mode>>,
            }

            pub mod ipv6_nd {

                #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6Nd.RaDnsServers", schema = Dict(1659)))]
                pub struct RaDnsServers<'a, Mode> {
                    #[data_view(schema = List(105))]
                    pub servers: ::validated_data::Field<ra_dns_servers::Servers<'a, Mode>>,
                    #[data_view(schema = Int(95))]
                    pub dns_servers_lifetime: ::validated_data::Field<i64>,
                }

                pub mod ra_dns_servers {

                    #[::validated_data::data_view(indexed_list, primary_key(address), python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6Nd.RaDnsServers.Servers", schema = List(105), item_name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6Nd.RaDnsServers.ServersItem"))]
                    pub struct Servers<'a, Mode> (::validated_data::Field<servers::Item<'a, Mode>>);

                    pub mod servers {

                        #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6Nd.RaDnsServers.ServersItemBase", schema = Dict(236)))]
                        pub struct Item<'a, Mode> {
                            #[data_view(schema = Str(208))]
                            pub address: ::validated_data::Field<&'a str>,
                            #[data_view(schema = Int(94))]
                            pub lifetime: ::validated_data::Field<i64>,
                        }
                    }
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6DhcpRelay", schema = Dict(1662)))]
            pub struct Ipv6DhcpRelay<'a, Mode> {
                #[data_view(schema = List(707))]
                pub destinations: ::validated_data::Field<ipv6_dhcp_relay::Destinations<'a, Mode>>,
            }

            pub mod ipv6_dhcp_relay {

                #[::validated_data::data_view(indexed_list, primary_key(address), python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6DhcpRelay.Destinations", schema = List(707), item_name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6DhcpRelay.DestinationsItem"))]
                pub struct Destinations<'a, Mode> (::validated_data::Field<destinations::Item<'a, Mode>>);

                pub mod destinations {

                    #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6DhcpRelay.DestinationsItemBase", schema = Dict(1661)))]
                    pub struct Item<'a, Mode> {
                        #[data_view(schema = Str(1052))]
                        pub address: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Str(212))]
                        pub vrf: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Str(1516))]
                        pub local_interface: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Str(1517))]
                        pub source_address: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Str(215))]
                        pub link_address: ::validated_data::Field<&'a str>,
                    }
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IpAddressVirtualSecondaries", schema = List(708)))]
            pub struct IpAddressVirtualSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IpVirtualRouterAddresses", schema = List(709)))]
            pub struct IpVirtualRouterAddresses<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6VirtualRouterAddresses", schema = List(710)))]
            pub struct Ipv6VirtualRouterAddresses<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(indexed_list, primary_key(ip_helper), python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IpHelpers", schema = List(697), item_name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IpHelpersItem"))]
            pub struct IpHelpers<'a, Mode> (::validated_data::Field<ip_helpers::Item<'a, Mode>>);

            pub mod ip_helpers {

                #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IpHelpersItemBase", schema = Dict(1647)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(1492))]
                    pub ip_helper: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1493))]
                    pub source_interface: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1494))]
                    pub source_vrf: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.StaticRoutes", schema = List(711)))]
            pub struct StaticRoutes<'a, Mode> (::validated_data::Field<static_routes::Item<'a, Mode>>);

            pub mod static_routes {

                #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.StaticRoutesItem", schema = Dict(1663)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(2))]
                    pub prefix: ::validated_data::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Str(3))]
                    pub next_hop: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Bool(223))]
                    pub track_bfd: ::validated_data::Field<bool>,
                    #[data_view(schema = Int(204))]
                    pub distance: ::validated_data::Field<i64>,
                    #[data_view(schema = Int(88))]
                    pub tag: ::validated_data::Field<i64>,
                    #[data_view(schema = Str(1524))]
                    pub name: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Int(88))]
                    pub metric: ::validated_data::Field<i64>,
                    #[data_view(schema = Str(3))]
                    pub interface: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6StaticRoutes", schema = List(712)))]
            pub struct Ipv6StaticRoutes<'a, Mode> (::validated_data::Field<ipv6_static_routes::Item<'a, Mode>>);

            pub mod ipv6_static_routes {

                #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6StaticRoutesItem", schema = Dict(1663)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(2))]
                    pub prefix: ::validated_data::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Str(3))]
                    pub next_hop: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Bool(223))]
                    pub track_bfd: ::validated_data::Field<bool>,
                    #[data_view(schema = Int(204))]
                    pub distance: ::validated_data::Field<i64>,
                    #[data_view(schema = Int(88))]
                    pub tag: ::validated_data::Field<i64>,
                    #[data_view(schema = Str(1524))]
                    pub name: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Int(88))]
                    pub metric: ::validated_data::Field<i64>,
                    #[data_view(schema = Str(3))]
                    pub interface: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.TrunkGroups", schema = List(713)))]
            pub struct TrunkGroups<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.EvpnL2Multicast", schema = Dict(1664)))]
            pub struct EvpnL2Multicast<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(583))]
                pub always_redistribute_igmp: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.VxlanFloodMulticast", schema = Dict(1665)))]
            pub struct VxlanFloodMulticast<'a, Mode> {
                #[data_view(schema = Bool(585))]
                pub enabled: ::validated_data::Field<bool>,
                #[data_view(schema = Str(1528))]
                pub underlay_multicast_group: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.EvpnL3Multicast", schema = Dict(1666)))]
            pub struct EvpnL3Multicast<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IgmpSnooping", schema = Dict(1608)))]
            pub struct IgmpSnooping<'a, Mode> {
                #[data_view(schema = Bool(546))]
                pub enabled: ::validated_data::Field<bool>,
                #[data_view(schema = Dict(1607))]
                pub querier: ::validated_data::Field<igmp_snooping::Querier<'a, Mode>>,
                #[data_view(schema = Bool(548))]
                pub fast_leave: ::validated_data::Field<bool>,
            }

            pub mod igmp_snooping {

                #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IgmpSnooping.Querier", schema = Dict(1607)))]
                pub struct Querier<'a, Mode> {
                    #[data_view(schema = Bool(547))]
                    pub enabled: ::validated_data::Field<bool>,
                    #[data_view(schema = Str(1437))]
                    pub source_address: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Int(595))]
                    pub version: ::validated_data::Field<i64>,
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IgmpSnoopingQuerier", schema = Dict(1667)))]
            pub struct IgmpSnoopingQuerier<'a, Mode> {
                #[data_view(schema = Bool(550))]
                pub enabled: ::validated_data::Field<bool>,
                #[data_view(schema = Str(1437))]
                pub source_address: ::validated_data::Field<&'a str>,
                #[data_view(schema = Int(595))]
                pub version: ::validated_data::Field<i64>,
                #[data_view(schema = Bool(548))]
                pub fast_leave: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ospf", schema = Dict(1669)))]
            pub struct Ospf<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(340))]
                pub point_to_point: ::validated_data::Field<bool>,
                #[data_view(schema = Str(1529))]
                pub area: ::validated_data::Field<&'a str>,
                #[data_view(schema = Int(612))]
                pub cost: ::validated_data::Field<i64>,
                #[data_view(schema = Str(1530))]
                pub authentication: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(1531))]
                pub simple_auth_key: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(1532))]
                pub cleartext_simple_auth_key: ::validated_data::Field<&'a str>,
                #[data_view(schema = List(714))]
                pub message_digest_keys: ::validated_data::Field<ospf::MessageDigestKeys<'a, Mode>>,
            }

            pub mod ospf {

                #[::validated_data::data_view(indexed_list, primary_key(id), python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ospf.MessageDigestKeys", schema = List(714), item_name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ospf.MessageDigestKeysItem"))]
                pub struct MessageDigestKeys<'a, Mode> (::validated_data::Field<message_digest_keys::Item<'a, Mode>>);

                pub mod message_digest_keys {

                    #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ospf.MessageDigestKeysItemBase", schema = Dict(1668)))]
                    pub struct Item<'a, Mode> {
                        #[data_view(schema = Int(77))]
                        pub id: ::validated_data::Field<i64>,
                        #[data_view(schema = Str(1502))]
                        pub hash_algorithm: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Str(1533))]
                        pub key: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Str(1534))]
                        pub cleartext_key: ::validated_data::Field<&'a str>,
                    }
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Bgp", schema = Dict(1671)))]
            pub struct Bgp<'a, Mode> {
                #[data_view(schema = Dict(1670), relaxed)]
                pub structured_config: ::validated_data::Field<super::super::super::super::super::eos_config::router_bgp::vlans::Item<'a, ::validated_data::RelaxedValidated>>,
                #[data_view(schema = Str(1535))]
                pub raw_eos_cli: ::validated_data::Field<&'a str>,
            }

            pub mod bgp {
            }
        }
    }

    #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.IpAddressSecondaries", schema = List(222)))]
    pub struct IpAddressSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.Ipv6AddressVirtuals", schema = List(706)))]
    pub struct Ipv6AddressVirtuals<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.Ipv6Nd", schema = Dict(1660)))]
    pub struct Ipv6Nd<'a, Mode> {
        #[data_view(schema = Bool(582))]
        pub advertise_ipv6_address_virtuals: ::validated_data::Field<bool>,
        #[data_view(schema = Str(1051))]
        pub valid_lifetime: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1051))]
        pub preferred_lifetime: ::validated_data::Field<&'a str>,
        #[data_view(schema = Dict(1659))]
        pub ra_dns_servers: ::validated_data::Field<ipv6_nd::RaDnsServers<'a, Mode>>,
    }

    pub mod ipv6_nd {

        #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.Ipv6Nd.RaDnsServers", schema = Dict(1659)))]
        pub struct RaDnsServers<'a, Mode> {
            #[data_view(schema = List(105))]
            pub servers: ::validated_data::Field<ra_dns_servers::Servers<'a, Mode>>,
            #[data_view(schema = Int(95))]
            pub dns_servers_lifetime: ::validated_data::Field<i64>,
        }

        pub mod ra_dns_servers {

            #[::validated_data::data_view(indexed_list, primary_key(address), python(name = "AVDDesign.SviProfilesItemBase.Ipv6Nd.RaDnsServers.Servers", schema = List(105), item_name = "AVDDesign.SviProfilesItemBase.Ipv6Nd.RaDnsServers.ServersItem"))]
            pub struct Servers<'a, Mode> (::validated_data::Field<servers::Item<'a, Mode>>);

            pub mod servers {

                #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.Ipv6Nd.RaDnsServers.ServersItemBase", schema = Dict(236)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(208))]
                    pub address: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Int(94))]
                    pub lifetime: ::validated_data::Field<i64>,
                }
            }
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.Ipv6DhcpRelay", schema = Dict(1662)))]
    pub struct Ipv6DhcpRelay<'a, Mode> {
        #[data_view(schema = List(707))]
        pub destinations: ::validated_data::Field<ipv6_dhcp_relay::Destinations<'a, Mode>>,
    }

    pub mod ipv6_dhcp_relay {

        #[::validated_data::data_view(indexed_list, primary_key(address), python(name = "AVDDesign.SviProfilesItemBase.Ipv6DhcpRelay.Destinations", schema = List(707), item_name = "AVDDesign.SviProfilesItemBase.Ipv6DhcpRelay.DestinationsItem"))]
        pub struct Destinations<'a, Mode> (::validated_data::Field<destinations::Item<'a, Mode>>);

        pub mod destinations {

            #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.Ipv6DhcpRelay.DestinationsItemBase", schema = Dict(1661)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(1052))]
                pub address: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(212))]
                pub vrf: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(1516))]
                pub local_interface: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(1517))]
                pub source_address: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(215))]
                pub link_address: ::validated_data::Field<&'a str>,
            }
        }
    }

    #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.IpAddressVirtualSecondaries", schema = List(708)))]
    pub struct IpAddressVirtualSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.IpVirtualRouterAddresses", schema = List(709)))]
    pub struct IpVirtualRouterAddresses<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.Ipv6VirtualRouterAddresses", schema = List(710)))]
    pub struct Ipv6VirtualRouterAddresses<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(indexed_list, primary_key(ip_helper), python(name = "AVDDesign.SviProfilesItemBase.IpHelpers", schema = List(697), item_name = "AVDDesign.SviProfilesItemBase.IpHelpersItem"))]
    pub struct IpHelpers<'a, Mode> (::validated_data::Field<ip_helpers::Item<'a, Mode>>);

    pub mod ip_helpers {

        #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.IpHelpersItemBase", schema = Dict(1647)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(1492))]
            pub ip_helper: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1493))]
            pub source_interface: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1494))]
            pub source_vrf: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.StaticRoutes", schema = List(711)))]
    pub struct StaticRoutes<'a, Mode> (::validated_data::Field<static_routes::Item<'a, Mode>>);

    pub mod static_routes {

        #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.StaticRoutesItem", schema = Dict(1663)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(2))]
            pub prefix: ::validated_data::RequiredValue<&'a str, Mode>,
            #[data_view(schema = Str(3))]
            pub next_hop: ::validated_data::Field<&'a str>,
            #[data_view(schema = Bool(223))]
            pub track_bfd: ::validated_data::Field<bool>,
            #[data_view(schema = Int(204))]
            pub distance: ::validated_data::Field<i64>,
            #[data_view(schema = Int(88))]
            pub tag: ::validated_data::Field<i64>,
            #[data_view(schema = Str(1524))]
            pub name: ::validated_data::Field<&'a str>,
            #[data_view(schema = Int(88))]
            pub metric: ::validated_data::Field<i64>,
            #[data_view(schema = Str(3))]
            pub interface: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.Ipv6StaticRoutes", schema = List(712)))]
    pub struct Ipv6StaticRoutes<'a, Mode> (::validated_data::Field<ipv6_static_routes::Item<'a, Mode>>);

    pub mod ipv6_static_routes {

        #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.Ipv6StaticRoutesItem", schema = Dict(1663)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(2))]
            pub prefix: ::validated_data::RequiredValue<&'a str, Mode>,
            #[data_view(schema = Str(3))]
            pub next_hop: ::validated_data::Field<&'a str>,
            #[data_view(schema = Bool(223))]
            pub track_bfd: ::validated_data::Field<bool>,
            #[data_view(schema = Int(204))]
            pub distance: ::validated_data::Field<i64>,
            #[data_view(schema = Int(88))]
            pub tag: ::validated_data::Field<i64>,
            #[data_view(schema = Str(1524))]
            pub name: ::validated_data::Field<&'a str>,
            #[data_view(schema = Int(88))]
            pub metric: ::validated_data::Field<i64>,
            #[data_view(schema = Str(3))]
            pub interface: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.TrunkGroups", schema = List(713)))]
    pub struct TrunkGroups<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.EvpnL2Multicast", schema = Dict(1664)))]
    pub struct EvpnL2Multicast<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub enabled: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(583))]
        pub always_redistribute_igmp: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.VxlanFloodMulticast", schema = Dict(1665)))]
    pub struct VxlanFloodMulticast<'a, Mode> {
        #[data_view(schema = Bool(585))]
        pub enabled: ::validated_data::Field<bool>,
        #[data_view(schema = Str(1528))]
        pub underlay_multicast_group: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.EvpnL3Multicast", schema = Dict(1666)))]
    pub struct EvpnL3Multicast<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub enabled: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.IgmpSnooping", schema = Dict(1608)))]
    pub struct IgmpSnooping<'a, Mode> {
        #[data_view(schema = Bool(546))]
        pub enabled: ::validated_data::Field<bool>,
        #[data_view(schema = Dict(1607))]
        pub querier: ::validated_data::Field<igmp_snooping::Querier<'a, Mode>>,
        #[data_view(schema = Bool(548))]
        pub fast_leave: ::validated_data::Field<bool>,
    }

    pub mod igmp_snooping {

        #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.IgmpSnooping.Querier", schema = Dict(1607)))]
        pub struct Querier<'a, Mode> {
            #[data_view(schema = Bool(547))]
            pub enabled: ::validated_data::Field<bool>,
            #[data_view(schema = Str(1437))]
            pub source_address: ::validated_data::Field<&'a str>,
            #[data_view(schema = Int(595))]
            pub version: ::validated_data::Field<i64>,
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.IgmpSnoopingQuerier", schema = Dict(1667)))]
    pub struct IgmpSnoopingQuerier<'a, Mode> {
        #[data_view(schema = Bool(550))]
        pub enabled: ::validated_data::Field<bool>,
        #[data_view(schema = Str(1437))]
        pub source_address: ::validated_data::Field<&'a str>,
        #[data_view(schema = Int(595))]
        pub version: ::validated_data::Field<i64>,
        #[data_view(schema = Bool(548))]
        pub fast_leave: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.Ospf", schema = Dict(1669)))]
    pub struct Ospf<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub enabled: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(340))]
        pub point_to_point: ::validated_data::Field<bool>,
        #[data_view(schema = Str(1529))]
        pub area: ::validated_data::Field<&'a str>,
        #[data_view(schema = Int(612))]
        pub cost: ::validated_data::Field<i64>,
        #[data_view(schema = Str(1530))]
        pub authentication: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1531))]
        pub simple_auth_key: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1532))]
        pub cleartext_simple_auth_key: ::validated_data::Field<&'a str>,
        #[data_view(schema = List(714))]
        pub message_digest_keys: ::validated_data::Field<ospf::MessageDigestKeys<'a, Mode>>,
    }

    pub mod ospf {

        #[::validated_data::data_view(indexed_list, primary_key(id), python(name = "AVDDesign.SviProfilesItemBase.Ospf.MessageDigestKeys", schema = List(714), item_name = "AVDDesign.SviProfilesItemBase.Ospf.MessageDigestKeysItem"))]
        pub struct MessageDigestKeys<'a, Mode> (::validated_data::Field<message_digest_keys::Item<'a, Mode>>);

        pub mod message_digest_keys {

            #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.Ospf.MessageDigestKeysItemBase", schema = Dict(1668)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Int(77))]
                pub id: ::validated_data::Field<i64>,
                #[data_view(schema = Str(1502))]
                pub hash_algorithm: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(1533))]
                pub key: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(1534))]
                pub cleartext_key: ::validated_data::Field<&'a str>,
            }
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesign.SviProfilesItemBase.Bgp", schema = Dict(1671)))]
    pub struct Bgp<'a, Mode> {
        #[data_view(schema = Dict(1670), relaxed)]
        pub structured_config: ::validated_data::Field<super::super::super::eos_config::router_bgp::vlans::Item<'a, ::validated_data::RelaxedValidated>>,
        #[data_view(schema = Str(1535))]
        pub raw_eos_cli: ::validated_data::Field<&'a str>,
    }

    pub mod bgp {
    }
}
