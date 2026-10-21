// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(profile), python(name = "AVDDesign.SviProfiles", schema = List(781), item_name = "AVDDesign.SviProfilesItem"))]
pub struct SviProfiles<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase", schema = Dict(1763)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(608))]
    pub profile: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1642))]
    pub parent_profile: ::data_views::Field<&'a str>,
    #[data_view(schema = List(780))]
    pub nodes: ::data_views::Field<item::Nodes<'a, Mode>>,
    #[data_view(schema = Str(1511))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(579))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = Bool(580))]
    pub autostate: ::data_views::Field<bool>,
    #[data_view(schema = Str(1512))]
    pub description: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(124))]
    pub arp_gratuitous_accept: ::data_views::Field<bool>,
    #[data_view(schema = Str(1513))]
    pub ip_address: ::data_views::Field<&'a str>,
    #[data_view(schema = List(222))]
    pub ip_address_secondaries: ::data_views::Field<item::IpAddressSecondaries<'a, Mode>>,
    #[data_view(schema = Str(1514))]
    pub ipv6_address: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(581))]
    pub ipv6_enable: ::data_views::Field<bool>,
    #[data_view(schema = Str(1515))]
    pub ip_address_virtual: ::data_views::Field<&'a str>,
    #[data_view(schema = List(706))]
    pub ipv6_address_virtuals: ::data_views::Field<item::Ipv6AddressVirtuals<'a, Mode>>,
    #[data_view(schema = Dict(1660))]
    pub ipv6_nd: ::data_views::Field<item::Ipv6Nd<'a, Mode>>,
    #[data_view(schema = Dict(1662))]
    pub ipv6_dhcp_relay: ::data_views::Field<item::Ipv6DhcpRelay<'a, Mode>>,
    #[data_view(schema = List(708))]
    pub ip_address_virtual_secondaries: ::data_views::Field<item::IpAddressVirtualSecondaries<'a, Mode>>,
    #[data_view(schema = List(709))]
    pub ip_virtual_router_addresses: ::data_views::Field<item::IpVirtualRouterAddresses<'a, Mode>>,
    #[data_view(schema = List(710))]
    pub ipv6_virtual_router_addresses: ::data_views::Field<item::Ipv6VirtualRouterAddresses<'a, Mode>>,
    #[data_view(schema = Str(1520))]
    pub ipv4_acl_in: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1521))]
    pub ipv4_acl_out: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1522))]
    pub ipv6_acl_in: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1523))]
    pub ipv6_acl_out: ::data_views::Field<&'a str>,
    #[data_view(schema = List(697))]
    pub ip_helpers: ::data_views::Field<item::IpHelpers<'a, Mode>>,
    #[data_view(schema = List(711))]
    pub static_routes: ::data_views::Field<item::StaticRoutes<'a, Mode>>,
    #[data_view(schema = List(712))]
    pub ipv6_static_routes: ::data_views::Field<item::Ipv6StaticRoutes<'a, Mode>>,
    #[data_view(schema = Int(610))]
    pub vni_override: ::data_views::Field<i64>,
    #[data_view(schema = Str(1525))]
    pub rt_override: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1526))]
    pub rd_override: ::data_views::Field<&'a str>,
    #[data_view(schema = List(713))]
    pub trunk_groups: ::data_views::Field<item::TrunkGroups<'a, Mode>>,
    #[data_view(schema = Dict(1664))]
    pub evpn_l2_multicast: ::data_views::Field<item::EvpnL2Multicast<'a, Mode>>,
    #[data_view(schema = Bool(584))]
    pub evpn_redistribute_router_mac_system: ::data_views::Field<bool>,
    #[data_view(schema = Dict(1665))]
    pub vxlan_flood_multicast: ::data_views::Field<item::VxlanFloodMulticast<'a, Mode>>,
    #[data_view(schema = Dict(1666))]
    pub evpn_l3_multicast: ::data_views::Field<item::EvpnL3Multicast<'a, Mode>>,
    #[data_view(schema = Dict(1608))]
    pub igmp_snooping: ::data_views::Field<item::IgmpSnooping<'a, Mode>>,
    #[data_view(schema = Bool(586))]
    pub igmp_snooping_enabled: ::data_views::Field<bool>,
    #[data_view(schema = Dict(1667))]
    pub igmp_snooping_querier: ::data_views::Field<item::IgmpSnoopingQuerier<'a, Mode>>,
    #[data_view(schema = Bool(587))]
    pub vxlan: ::data_views::Field<bool>,
    #[data_view(schema = Int(594))]
    pub spanning_tree_priority: ::data_views::Field<i64>,
    #[data_view(schema = Int(611))]
    pub mtu: ::data_views::Field<i64>,
    #[data_view(schema = Dict(1669))]
    pub ospf: ::data_views::Field<item::Ospf<'a, Mode>>,
    #[data_view(schema = Dict(1671))]
    pub bgp: ::data_views::Field<item::Bgp<'a, Mode>>,
    #[data_view(schema = Str(1536))]
    pub raw_eos_cli: ::data_views::Field<&'a str>,
    #[data_view(schema = Dict(1672), relaxed)]
    pub structured_config: ::data_views::Field<super::super::eos_config::vlan_interfaces::Item<'a, ::data_views::RelaxedValidated>>,
    #[data_view(schema = Bool(588))]
    pub evpn_l2_multi_domain: ::data_views::Field<bool>,
}

pub mod item {

    #[::data_views::data_view(indexed_list, primary_key(node), python(name = "AVDDesign.SviProfilesItemBase.Nodes", schema = List(780), item_name = "AVDDesign.SviProfilesItemBase.NodesItem"))]
    pub struct Nodes<'a, Mode> (::data_views::Field<nodes::Item<'a, Mode>>);

    pub mod nodes {

        #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase", schema = Dict(1762)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(1510))]
            pub node: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1511))]
            pub name: ::data_views::Field<&'a str>,
            #[data_view(schema = Bool(579))]
            pub enabled: ::data_views::Field<bool>,
            #[data_view(schema = Bool(580))]
            pub autostate: ::data_views::Field<bool>,
            #[data_view(schema = Str(1512))]
            pub description: ::data_views::Field<&'a str>,
            #[data_view(schema = Bool(124))]
            pub arp_gratuitous_accept: ::data_views::Field<bool>,
            #[data_view(schema = Str(1513))]
            pub ip_address: ::data_views::Field<&'a str>,
            #[data_view(schema = List(222))]
            pub ip_address_secondaries: ::data_views::Field<item::IpAddressSecondaries<'a, Mode>>,
            #[data_view(schema = Str(1514))]
            pub ipv6_address: ::data_views::Field<&'a str>,
            #[data_view(schema = Bool(581))]
            pub ipv6_enable: ::data_views::Field<bool>,
            #[data_view(schema = Str(1515))]
            pub ip_address_virtual: ::data_views::Field<&'a str>,
            #[data_view(schema = List(706))]
            pub ipv6_address_virtuals: ::data_views::Field<item::Ipv6AddressVirtuals<'a, Mode>>,
            #[data_view(schema = Dict(1660))]
            pub ipv6_nd: ::data_views::Field<item::Ipv6Nd<'a, Mode>>,
            #[data_view(schema = Dict(1662))]
            pub ipv6_dhcp_relay: ::data_views::Field<item::Ipv6DhcpRelay<'a, Mode>>,
            #[data_view(schema = List(708))]
            pub ip_address_virtual_secondaries: ::data_views::Field<item::IpAddressVirtualSecondaries<'a, Mode>>,
            #[data_view(schema = List(709))]
            pub ip_virtual_router_addresses: ::data_views::Field<item::IpVirtualRouterAddresses<'a, Mode>>,
            #[data_view(schema = List(710))]
            pub ipv6_virtual_router_addresses: ::data_views::Field<item::Ipv6VirtualRouterAddresses<'a, Mode>>,
            #[data_view(schema = Str(1520))]
            pub ipv4_acl_in: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1521))]
            pub ipv4_acl_out: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1522))]
            pub ipv6_acl_in: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1523))]
            pub ipv6_acl_out: ::data_views::Field<&'a str>,
            #[data_view(schema = List(697))]
            pub ip_helpers: ::data_views::Field<item::IpHelpers<'a, Mode>>,
            #[data_view(schema = List(711))]
            pub static_routes: ::data_views::Field<item::StaticRoutes<'a, Mode>>,
            #[data_view(schema = List(712))]
            pub ipv6_static_routes: ::data_views::Field<item::Ipv6StaticRoutes<'a, Mode>>,
            #[data_view(schema = Int(610))]
            pub vni_override: ::data_views::Field<i64>,
            #[data_view(schema = Str(1525))]
            pub rt_override: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1526))]
            pub rd_override: ::data_views::Field<&'a str>,
            #[data_view(schema = List(713))]
            pub trunk_groups: ::data_views::Field<item::TrunkGroups<'a, Mode>>,
            #[data_view(schema = Dict(1664))]
            pub evpn_l2_multicast: ::data_views::Field<item::EvpnL2Multicast<'a, Mode>>,
            #[data_view(schema = Bool(584))]
            pub evpn_redistribute_router_mac_system: ::data_views::Field<bool>,
            #[data_view(schema = Dict(1665))]
            pub vxlan_flood_multicast: ::data_views::Field<item::VxlanFloodMulticast<'a, Mode>>,
            #[data_view(schema = Dict(1666))]
            pub evpn_l3_multicast: ::data_views::Field<item::EvpnL3Multicast<'a, Mode>>,
            #[data_view(schema = Dict(1608))]
            pub igmp_snooping: ::data_views::Field<item::IgmpSnooping<'a, Mode>>,
            #[data_view(schema = Bool(586))]
            pub igmp_snooping_enabled: ::data_views::Field<bool>,
            #[data_view(schema = Dict(1667))]
            pub igmp_snooping_querier: ::data_views::Field<item::IgmpSnoopingQuerier<'a, Mode>>,
            #[data_view(schema = Bool(587))]
            pub vxlan: ::data_views::Field<bool>,
            #[data_view(schema = Int(594))]
            pub spanning_tree_priority: ::data_views::Field<i64>,
            #[data_view(schema = Int(611))]
            pub mtu: ::data_views::Field<i64>,
            #[data_view(schema = Dict(1669))]
            pub ospf: ::data_views::Field<item::Ospf<'a, Mode>>,
            #[data_view(schema = Dict(1671))]
            pub bgp: ::data_views::Field<item::Bgp<'a, Mode>>,
            #[data_view(schema = Str(1536))]
            pub raw_eos_cli: ::data_views::Field<&'a str>,
            #[data_view(schema = Dict(1672), relaxed)]
            pub structured_config: ::data_views::Field<super::super::super::super::eos_config::vlan_interfaces::Item<'a, ::data_views::RelaxedValidated>>,
            #[data_view(schema = Bool(588))]
            pub evpn_l2_multi_domain: ::data_views::Field<bool>,
        }

        pub mod item {

            #[::data_views::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IpAddressSecondaries", schema = List(222)))]
            pub struct IpAddressSecondaries<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6AddressVirtuals", schema = List(706)))]
            pub struct Ipv6AddressVirtuals<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6Nd", schema = Dict(1660)))]
            pub struct Ipv6Nd<'a, Mode> {
                #[data_view(schema = Bool(582))]
                pub advertise_ipv6_address_virtuals: ::data_views::Field<bool>,
                #[data_view(schema = Str(1051))]
                pub valid_lifetime: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(1051))]
                pub preferred_lifetime: ::data_views::Field<&'a str>,
                #[data_view(schema = Dict(1659))]
                pub ra_dns_servers: ::data_views::Field<ipv6_nd::RaDnsServers<'a, Mode>>,
            }

            pub mod ipv6_nd {

                #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6Nd.RaDnsServers", schema = Dict(1659)))]
                pub struct RaDnsServers<'a, Mode> {
                    #[data_view(schema = List(105))]
                    pub servers: ::data_views::Field<ra_dns_servers::Servers<'a, Mode>>,
                    #[data_view(schema = Int(95))]
                    pub dns_servers_lifetime: ::data_views::Field<i64>,
                }

                pub mod ra_dns_servers {

                    #[::data_views::data_view(indexed_list, primary_key(address), python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6Nd.RaDnsServers.Servers", schema = List(105), item_name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6Nd.RaDnsServers.ServersItem"))]
                    pub struct Servers<'a, Mode> (::data_views::Field<servers::Item<'a, Mode>>);

                    pub mod servers {

                        #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6Nd.RaDnsServers.ServersItemBase", schema = Dict(236)))]
                        pub struct Item<'a, Mode> {
                            #[data_view(schema = Str(208))]
                            pub address: ::data_views::Field<&'a str>,
                            #[data_view(schema = Int(94))]
                            pub lifetime: ::data_views::Field<i64>,
                        }
                    }
                }
            }

            #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6DhcpRelay", schema = Dict(1662)))]
            pub struct Ipv6DhcpRelay<'a, Mode> {
                #[data_view(schema = List(707))]
                pub destinations: ::data_views::Field<ipv6_dhcp_relay::Destinations<'a, Mode>>,
            }

            pub mod ipv6_dhcp_relay {

                #[::data_views::data_view(indexed_list, primary_key(address), python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6DhcpRelay.Destinations", schema = List(707), item_name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6DhcpRelay.DestinationsItem"))]
                pub struct Destinations<'a, Mode> (::data_views::Field<destinations::Item<'a, Mode>>);

                pub mod destinations {

                    #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6DhcpRelay.DestinationsItemBase", schema = Dict(1661)))]
                    pub struct Item<'a, Mode> {
                        #[data_view(schema = Str(1052))]
                        pub address: ::data_views::Field<&'a str>,
                        #[data_view(schema = Str(212))]
                        pub vrf: ::data_views::Field<&'a str>,
                        #[data_view(schema = Str(1516))]
                        pub local_interface: ::data_views::Field<&'a str>,
                        #[data_view(schema = Str(1517))]
                        pub source_address: ::data_views::Field<&'a str>,
                        #[data_view(schema = Str(215))]
                        pub link_address: ::data_views::Field<&'a str>,
                    }
                }
            }

            #[::data_views::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IpAddressVirtualSecondaries", schema = List(708)))]
            pub struct IpAddressVirtualSecondaries<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IpVirtualRouterAddresses", schema = List(709)))]
            pub struct IpVirtualRouterAddresses<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6VirtualRouterAddresses", schema = List(710)))]
            pub struct Ipv6VirtualRouterAddresses<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(indexed_list, primary_key(ip_helper), python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IpHelpers", schema = List(697), item_name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IpHelpersItem"))]
            pub struct IpHelpers<'a, Mode> (::data_views::Field<ip_helpers::Item<'a, Mode>>);

            pub mod ip_helpers {

                #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IpHelpersItemBase", schema = Dict(1647)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(1492))]
                    pub ip_helper: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(1493))]
                    pub source_interface: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(1494))]
                    pub source_vrf: ::data_views::Field<&'a str>,
                }
            }

            #[::data_views::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.StaticRoutes", schema = List(711)))]
            pub struct StaticRoutes<'a, Mode> (::data_views::Field<static_routes::Item<'a, Mode>>);

            pub mod static_routes {

                #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.StaticRoutesItem", schema = Dict(1663)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(2))]
                    pub prefix: ::data_views::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Str(3))]
                    pub next_hop: ::data_views::Field<&'a str>,
                    #[data_view(schema = Bool(223))]
                    pub track_bfd: ::data_views::Field<bool>,
                    #[data_view(schema = Int(204))]
                    pub distance: ::data_views::Field<i64>,
                    #[data_view(schema = Int(88))]
                    pub tag: ::data_views::Field<i64>,
                    #[data_view(schema = Str(1524))]
                    pub name: ::data_views::Field<&'a str>,
                    #[data_view(schema = Int(88))]
                    pub metric: ::data_views::Field<i64>,
                    #[data_view(schema = Str(3))]
                    pub interface: ::data_views::Field<&'a str>,
                }
            }

            #[::data_views::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6StaticRoutes", schema = List(712)))]
            pub struct Ipv6StaticRoutes<'a, Mode> (::data_views::Field<ipv6_static_routes::Item<'a, Mode>>);

            pub mod ipv6_static_routes {

                #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ipv6StaticRoutesItem", schema = Dict(1663)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(2))]
                    pub prefix: ::data_views::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Str(3))]
                    pub next_hop: ::data_views::Field<&'a str>,
                    #[data_view(schema = Bool(223))]
                    pub track_bfd: ::data_views::Field<bool>,
                    #[data_view(schema = Int(204))]
                    pub distance: ::data_views::Field<i64>,
                    #[data_view(schema = Int(88))]
                    pub tag: ::data_views::Field<i64>,
                    #[data_view(schema = Str(1524))]
                    pub name: ::data_views::Field<&'a str>,
                    #[data_view(schema = Int(88))]
                    pub metric: ::data_views::Field<i64>,
                    #[data_view(schema = Str(3))]
                    pub interface: ::data_views::Field<&'a str>,
                }
            }

            #[::data_views::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.TrunkGroups", schema = List(713)))]
            pub struct TrunkGroups<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.EvpnL2Multicast", schema = Dict(1664)))]
            pub struct EvpnL2Multicast<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::data_views::Field<bool>,
                #[data_view(schema = Bool(583))]
                pub always_redistribute_igmp: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.VxlanFloodMulticast", schema = Dict(1665)))]
            pub struct VxlanFloodMulticast<'a, Mode> {
                #[data_view(schema = Bool(585))]
                pub enabled: ::data_views::Field<bool>,
                #[data_view(schema = Str(1528))]
                pub underlay_multicast_group: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.EvpnL3Multicast", schema = Dict(1666)))]
            pub struct EvpnL3Multicast<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IgmpSnooping", schema = Dict(1608)))]
            pub struct IgmpSnooping<'a, Mode> {
                #[data_view(schema = Bool(546))]
                pub enabled: ::data_views::Field<bool>,
                #[data_view(schema = Dict(1607))]
                pub querier: ::data_views::Field<igmp_snooping::Querier<'a, Mode>>,
                #[data_view(schema = Bool(548))]
                pub fast_leave: ::data_views::Field<bool>,
            }

            pub mod igmp_snooping {

                #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IgmpSnooping.Querier", schema = Dict(1607)))]
                pub struct Querier<'a, Mode> {
                    #[data_view(schema = Bool(547))]
                    pub enabled: ::data_views::Field<bool>,
                    #[data_view(schema = Str(1437))]
                    pub source_address: ::data_views::Field<&'a str>,
                    #[data_view(schema = Int(595))]
                    pub version: ::data_views::Field<i64>,
                }
            }

            #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.IgmpSnoopingQuerier", schema = Dict(1667)))]
            pub struct IgmpSnoopingQuerier<'a, Mode> {
                #[data_view(schema = Bool(550))]
                pub enabled: ::data_views::Field<bool>,
                #[data_view(schema = Str(1437))]
                pub source_address: ::data_views::Field<&'a str>,
                #[data_view(schema = Int(595))]
                pub version: ::data_views::Field<i64>,
                #[data_view(schema = Bool(548))]
                pub fast_leave: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ospf", schema = Dict(1669)))]
            pub struct Ospf<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::data_views::Field<bool>,
                #[data_view(schema = Bool(340))]
                pub point_to_point: ::data_views::Field<bool>,
                #[data_view(schema = Str(1529))]
                pub area: ::data_views::Field<&'a str>,
                #[data_view(schema = Int(612))]
                pub cost: ::data_views::Field<i64>,
                #[data_view(schema = Str(1530))]
                pub authentication: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(1531))]
                pub simple_auth_key: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(1532))]
                pub cleartext_simple_auth_key: ::data_views::Field<&'a str>,
                #[data_view(schema = List(714))]
                pub message_digest_keys: ::data_views::Field<ospf::MessageDigestKeys<'a, Mode>>,
            }

            pub mod ospf {

                #[::data_views::data_view(indexed_list, primary_key(id), python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ospf.MessageDigestKeys", schema = List(714), item_name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ospf.MessageDigestKeysItem"))]
                pub struct MessageDigestKeys<'a, Mode> (::data_views::Field<message_digest_keys::Item<'a, Mode>>);

                pub mod message_digest_keys {

                    #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Ospf.MessageDigestKeysItemBase", schema = Dict(1668)))]
                    pub struct Item<'a, Mode> {
                        #[data_view(schema = Int(77))]
                        pub id: ::data_views::Field<i64>,
                        #[data_view(schema = Str(1502))]
                        pub hash_algorithm: ::data_views::Field<&'a str>,
                        #[data_view(schema = Str(1533))]
                        pub key: ::data_views::Field<&'a str>,
                        #[data_view(schema = Str(1534))]
                        pub cleartext_key: ::data_views::Field<&'a str>,
                    }
                }
            }

            #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.NodesItemBase.Bgp", schema = Dict(1671)))]
            pub struct Bgp<'a, Mode> {
                #[data_view(schema = Dict(1670), relaxed)]
                pub structured_config: ::data_views::Field<super::super::super::super::super::eos_config::router_bgp::vlans::Item<'a, ::data_views::RelaxedValidated>>,
                #[data_view(schema = Str(1535))]
                pub raw_eos_cli: ::data_views::Field<&'a str>,
            }

            pub mod bgp {
            }
        }
    }

    #[::data_views::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.IpAddressSecondaries", schema = List(222)))]
    pub struct IpAddressSecondaries<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.Ipv6AddressVirtuals", schema = List(706)))]
    pub struct Ipv6AddressVirtuals<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.Ipv6Nd", schema = Dict(1660)))]
    pub struct Ipv6Nd<'a, Mode> {
        #[data_view(schema = Bool(582))]
        pub advertise_ipv6_address_virtuals: ::data_views::Field<bool>,
        #[data_view(schema = Str(1051))]
        pub valid_lifetime: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1051))]
        pub preferred_lifetime: ::data_views::Field<&'a str>,
        #[data_view(schema = Dict(1659))]
        pub ra_dns_servers: ::data_views::Field<ipv6_nd::RaDnsServers<'a, Mode>>,
    }

    pub mod ipv6_nd {

        #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.Ipv6Nd.RaDnsServers", schema = Dict(1659)))]
        pub struct RaDnsServers<'a, Mode> {
            #[data_view(schema = List(105))]
            pub servers: ::data_views::Field<ra_dns_servers::Servers<'a, Mode>>,
            #[data_view(schema = Int(95))]
            pub dns_servers_lifetime: ::data_views::Field<i64>,
        }

        pub mod ra_dns_servers {

            #[::data_views::data_view(indexed_list, primary_key(address), python(name = "AVDDesign.SviProfilesItemBase.Ipv6Nd.RaDnsServers.Servers", schema = List(105), item_name = "AVDDesign.SviProfilesItemBase.Ipv6Nd.RaDnsServers.ServersItem"))]
            pub struct Servers<'a, Mode> (::data_views::Field<servers::Item<'a, Mode>>);

            pub mod servers {

                #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.Ipv6Nd.RaDnsServers.ServersItemBase", schema = Dict(236)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(208))]
                    pub address: ::data_views::Field<&'a str>,
                    #[data_view(schema = Int(94))]
                    pub lifetime: ::data_views::Field<i64>,
                }
            }
        }
    }

    #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.Ipv6DhcpRelay", schema = Dict(1662)))]
    pub struct Ipv6DhcpRelay<'a, Mode> {
        #[data_view(schema = List(707))]
        pub destinations: ::data_views::Field<ipv6_dhcp_relay::Destinations<'a, Mode>>,
    }

    pub mod ipv6_dhcp_relay {

        #[::data_views::data_view(indexed_list, primary_key(address), python(name = "AVDDesign.SviProfilesItemBase.Ipv6DhcpRelay.Destinations", schema = List(707), item_name = "AVDDesign.SviProfilesItemBase.Ipv6DhcpRelay.DestinationsItem"))]
        pub struct Destinations<'a, Mode> (::data_views::Field<destinations::Item<'a, Mode>>);

        pub mod destinations {

            #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.Ipv6DhcpRelay.DestinationsItemBase", schema = Dict(1661)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(1052))]
                pub address: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(212))]
                pub vrf: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(1516))]
                pub local_interface: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(1517))]
                pub source_address: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(215))]
                pub link_address: ::data_views::Field<&'a str>,
            }
        }
    }

    #[::data_views::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.IpAddressVirtualSecondaries", schema = List(708)))]
    pub struct IpAddressVirtualSecondaries<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.IpVirtualRouterAddresses", schema = List(709)))]
    pub struct IpVirtualRouterAddresses<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.Ipv6VirtualRouterAddresses", schema = List(710)))]
    pub struct Ipv6VirtualRouterAddresses<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(indexed_list, primary_key(ip_helper), python(name = "AVDDesign.SviProfilesItemBase.IpHelpers", schema = List(697), item_name = "AVDDesign.SviProfilesItemBase.IpHelpersItem"))]
    pub struct IpHelpers<'a, Mode> (::data_views::Field<ip_helpers::Item<'a, Mode>>);

    pub mod ip_helpers {

        #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.IpHelpersItemBase", schema = Dict(1647)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(1492))]
            pub ip_helper: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1493))]
            pub source_interface: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1494))]
            pub source_vrf: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.StaticRoutes", schema = List(711)))]
    pub struct StaticRoutes<'a, Mode> (::data_views::Field<static_routes::Item<'a, Mode>>);

    pub mod static_routes {

        #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.StaticRoutesItem", schema = Dict(1663)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(2))]
            pub prefix: ::data_views::RequiredValue<&'a str, Mode>,
            #[data_view(schema = Str(3))]
            pub next_hop: ::data_views::Field<&'a str>,
            #[data_view(schema = Bool(223))]
            pub track_bfd: ::data_views::Field<bool>,
            #[data_view(schema = Int(204))]
            pub distance: ::data_views::Field<i64>,
            #[data_view(schema = Int(88))]
            pub tag: ::data_views::Field<i64>,
            #[data_view(schema = Str(1524))]
            pub name: ::data_views::Field<&'a str>,
            #[data_view(schema = Int(88))]
            pub metric: ::data_views::Field<i64>,
            #[data_view(schema = Str(3))]
            pub interface: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.Ipv6StaticRoutes", schema = List(712)))]
    pub struct Ipv6StaticRoutes<'a, Mode> (::data_views::Field<ipv6_static_routes::Item<'a, Mode>>);

    pub mod ipv6_static_routes {

        #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.Ipv6StaticRoutesItem", schema = Dict(1663)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(2))]
            pub prefix: ::data_views::RequiredValue<&'a str, Mode>,
            #[data_view(schema = Str(3))]
            pub next_hop: ::data_views::Field<&'a str>,
            #[data_view(schema = Bool(223))]
            pub track_bfd: ::data_views::Field<bool>,
            #[data_view(schema = Int(204))]
            pub distance: ::data_views::Field<i64>,
            #[data_view(schema = Int(88))]
            pub tag: ::data_views::Field<i64>,
            #[data_view(schema = Str(1524))]
            pub name: ::data_views::Field<&'a str>,
            #[data_view(schema = Int(88))]
            pub metric: ::data_views::Field<i64>,
            #[data_view(schema = Str(3))]
            pub interface: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view(list, python(name = "AVDDesign.SviProfilesItemBase.TrunkGroups", schema = List(713)))]
    pub struct TrunkGroups<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.EvpnL2Multicast", schema = Dict(1664)))]
    pub struct EvpnL2Multicast<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub enabled: ::data_views::Field<bool>,
        #[data_view(schema = Bool(583))]
        pub always_redistribute_igmp: ::data_views::Field<bool>,
    }

    #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.VxlanFloodMulticast", schema = Dict(1665)))]
    pub struct VxlanFloodMulticast<'a, Mode> {
        #[data_view(schema = Bool(585))]
        pub enabled: ::data_views::Field<bool>,
        #[data_view(schema = Str(1528))]
        pub underlay_multicast_group: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.EvpnL3Multicast", schema = Dict(1666)))]
    pub struct EvpnL3Multicast<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub enabled: ::data_views::Field<bool>,
    }

    #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.IgmpSnooping", schema = Dict(1608)))]
    pub struct IgmpSnooping<'a, Mode> {
        #[data_view(schema = Bool(546))]
        pub enabled: ::data_views::Field<bool>,
        #[data_view(schema = Dict(1607))]
        pub querier: ::data_views::Field<igmp_snooping::Querier<'a, Mode>>,
        #[data_view(schema = Bool(548))]
        pub fast_leave: ::data_views::Field<bool>,
    }

    pub mod igmp_snooping {

        #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.IgmpSnooping.Querier", schema = Dict(1607)))]
        pub struct Querier<'a, Mode> {
            #[data_view(schema = Bool(547))]
            pub enabled: ::data_views::Field<bool>,
            #[data_view(schema = Str(1437))]
            pub source_address: ::data_views::Field<&'a str>,
            #[data_view(schema = Int(595))]
            pub version: ::data_views::Field<i64>,
        }
    }

    #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.IgmpSnoopingQuerier", schema = Dict(1667)))]
    pub struct IgmpSnoopingQuerier<'a, Mode> {
        #[data_view(schema = Bool(550))]
        pub enabled: ::data_views::Field<bool>,
        #[data_view(schema = Str(1437))]
        pub source_address: ::data_views::Field<&'a str>,
        #[data_view(schema = Int(595))]
        pub version: ::data_views::Field<i64>,
        #[data_view(schema = Bool(548))]
        pub fast_leave: ::data_views::Field<bool>,
    }

    #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.Ospf", schema = Dict(1669)))]
    pub struct Ospf<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub enabled: ::data_views::Field<bool>,
        #[data_view(schema = Bool(340))]
        pub point_to_point: ::data_views::Field<bool>,
        #[data_view(schema = Str(1529))]
        pub area: ::data_views::Field<&'a str>,
        #[data_view(schema = Int(612))]
        pub cost: ::data_views::Field<i64>,
        #[data_view(schema = Str(1530))]
        pub authentication: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1531))]
        pub simple_auth_key: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1532))]
        pub cleartext_simple_auth_key: ::data_views::Field<&'a str>,
        #[data_view(schema = List(714))]
        pub message_digest_keys: ::data_views::Field<ospf::MessageDigestKeys<'a, Mode>>,
    }

    pub mod ospf {

        #[::data_views::data_view(indexed_list, primary_key(id), python(name = "AVDDesign.SviProfilesItemBase.Ospf.MessageDigestKeys", schema = List(714), item_name = "AVDDesign.SviProfilesItemBase.Ospf.MessageDigestKeysItem"))]
        pub struct MessageDigestKeys<'a, Mode> (::data_views::Field<message_digest_keys::Item<'a, Mode>>);

        pub mod message_digest_keys {

            #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.Ospf.MessageDigestKeysItemBase", schema = Dict(1668)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Int(77))]
                pub id: ::data_views::Field<i64>,
                #[data_view(schema = Str(1502))]
                pub hash_algorithm: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(1533))]
                pub key: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(1534))]
                pub cleartext_key: ::data_views::Field<&'a str>,
            }
        }
    }

    #[::data_views::data_view(python(name = "AVDDesign.SviProfilesItemBase.Bgp", schema = Dict(1671)))]
    pub struct Bgp<'a, Mode> {
        #[data_view(schema = Dict(1670), relaxed)]
        pub structured_config: ::data_views::Field<super::super::super::eos_config::router_bgp::vlans::Item<'a, ::data_views::RelaxedValidated>>,
        #[data_view(schema = Str(1535))]
        pub raw_eos_cli: ::data_views::Field<&'a str>,
    }

    pub mod bgp {
    }
}
