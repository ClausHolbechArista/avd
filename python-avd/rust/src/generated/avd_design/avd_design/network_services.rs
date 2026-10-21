// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.NetworkServices", schema = List(753), item_name = "AVDDesign.NetworkServicesItem"))]
pub struct NetworkServices<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase", schema = Dict(1706)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1473))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = Int(599))]
    pub mac_vrf_vni_base: ::data_views::Field<i64>,
    #[data_view(schema = Int(600))]
    pub mac_vrf_id_base: ::data_views::Field<i64>,
    #[data_view(schema = Int(601))]
    pub vlan_aware_bundle_number_base: ::data_views::Field<i64>,
    #[data_view(schema = Int(602))]
    pub pseudowire_rt_base: ::data_views::Field<i64>,
    #[data_view(schema = Bool(560))]
    pub enable_mlag_ibgp_peering_vrfs: ::data_views::Field<bool>,
    #[data_view(schema = Bool(561))]
    pub redistribute_mlag_ibgp_peering_vrfs: ::data_views::Field<bool>,
    #[data_view(schema = Str(1474))]
    pub evpn_vlan_bundle: ::data_views::Field<&'a str>,
    #[data_view(schema = List(689))]
    pub bgp_peer_groups: ::data_views::Field<item::BgpPeerGroups<'a, Mode>>,
    #[data_view(schema = Dict(1640))]
    pub igmp_snooping: ::data_views::Field<item::IgmpSnooping<'a, Mode>>,
    #[data_view(schema = Dict(1641))]
    pub evpn_l2_multicast: ::data_views::Field<item::EvpnL2Multicast<'a, Mode>>,
    #[data_view(schema = Dict(1642))]
    pub vxlan_flood_multicast: ::data_views::Field<item::VxlanFloodMulticast<'a, Mode>>,
    #[data_view(schema = Dict(1644))]
    pub evpn_l3_multicast: ::data_views::Field<item::EvpnL3Multicast<'a, Mode>>,
    #[data_view(schema = List(695))]
    pub pim_rp_addresses: ::data_views::Field<item::PimRpAddresses<'a, Mode>>,
    #[data_view(schema = Dict(1646))]
    pub igmp_snooping_querier: ::data_views::Field<item::IgmpSnoopingQuerier<'a, Mode>>,
    #[data_view(schema = Bool(568))]
    pub evpn_l2_multi_domain: ::data_views::Field<bool>,
    #[data_view(schema = List(745))]
    pub vrfs: ::data_views::Field<item::Vrfs<'a, Mode>>,
    #[data_view(schema = List(747))]
    pub l2vlans: ::data_views::Field<item::L2vlans<'a, Mode>>,
    #[data_view(schema = Dict(1698))]
    pub vpws: ::data_views::Field<item::Vpws<'a, Mode>>,
    #[data_view(schema = List(752))]
    pub point_to_point_services: ::data_views::Field<item::PointToPointServices<'a, Mode>>,
}

pub mod item {

    #[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.NetworkServicesItemBase.BgpPeerGroups", schema = List(689), item_name = "AVDDesign.NetworkServicesItemBase.BgpPeerGroupsItem"))]
    pub struct BgpPeerGroups<'a, Mode> (::data_views::Field<bgp_peer_groups::Item<'a, Mode>>);

    pub mod bgp_peer_groups {

        #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.BgpPeerGroupsItemBase", schema = Dict(1638)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(1475))]
            pub name: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1476))]
            pub password: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1109))]
            pub cleartext_password: ::data_views::Field<&'a str>,
            #[data_view(schema = List(687))]
            pub nodes: ::data_views::Field<item::Nodes<'a, Mode>>,
            #[data_view(schema = Dict(1634))]
            pub address_family_ipv4: ::data_views::Field<item::AddressFamilyIpv4<'a, Mode>>,
            #[data_view(schema = Dict(1636))]
            pub address_family_ipv6: ::data_views::Field<item::AddressFamilyIpv6<'a, Mode>>,
            #[data_view(schema = List(688))]
            pub listen_ranges: ::data_views::Field<item::ListenRanges<'a, Mode>>,
            #[data_view(schema = Dict(912))]
            pub metadata: ::data_views::Field<item::Metadata<'a, Mode>>,
            #[data_view(schema = Str(758))]
            pub remote_as: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(758))]
            pub local_as: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(3))]
            pub description: ::data_views::Field<&'a str>,
            #[data_view(schema = Bool(7))]
            pub shutdown: ::data_views::Field<bool>,
            #[data_view(schema = Dict(913))]
            pub as_path: ::data_views::Field<item::AsPath<'a, Mode>>,
            #[data_view(schema = Dict(914))]
            pub remove_private_as: ::data_views::Field<item::RemovePrivateAs<'a, Mode>>,
            #[data_view(schema = Dict(915))]
            pub remove_private_as_ingress: ::data_views::Field<item::RemovePrivateAsIngress<'a, Mode>>,
            #[data_view(schema = Bool(7))]
            pub next_hop_unchanged: ::data_views::Field<bool>,
            #[data_view(schema = Str(771))]
            pub update_source: ::data_views::Field<&'a str>,
            #[data_view(schema = Bool(7))]
            pub route_reflector_client: ::data_views::Field<bool>,
            #[data_view(schema = Bool(323))]
            pub bfd: ::data_views::Field<bool>,
            #[data_view(schema = Dict(916))]
            pub bfd_timers: ::data_views::Field<item::BfdTimers<'a, Mode>>,
            #[data_view(schema = Int(381))]
            pub ebgp_multihop: ::data_views::Field<i64>,
            #[data_view(schema = Bool(7))]
            pub next_hop_peer: ::data_views::Field<bool>,
            #[data_view(schema = Bool(7))]
            pub next_hop_self: ::data_views::Field<bool>,
            #[data_view(schema = Str(772))]
            pub password_type: ::data_views::Field<&'a str>,
            #[data_view(schema = Bool(7))]
            pub passive: ::data_views::Field<bool>,
            #[data_view(schema = Dict(917))]
            pub default_originate: ::data_views::Field<item::DefaultOriginate<'a, Mode>>,
            #[data_view(schema = Bool(324))]
            pub enforce_first_as: ::data_views::Field<bool>,
            #[data_view(schema = Str(773))]
            pub send_community: ::data_views::Field<&'a str>,
            #[data_view(schema = Int(382))]
            pub maximum_routes: ::data_views::Field<i64>,
            #[data_view(schema = Str(774))]
            pub maximum_routes_warning_limit: ::data_views::Field<&'a str>,
            #[data_view(schema = Bool(7))]
            pub maximum_routes_warning_only: ::data_views::Field<bool>,
            #[data_view(schema = Dict(919))]
            pub maximum_accepted_routes: ::data_views::Field<item::MaximumAcceptedRoutes<'a, Mode>>,
            #[data_view(schema = Dict(922))]
            pub missing_policy: ::data_views::Field<item::MissingPolicy<'a, Mode>>,
            #[data_view(schema = Dict(923))]
            pub link_bandwidth: ::data_views::Field<item::LinkBandwidth<'a, Mode>>,
            #[data_view(schema = Dict(924))]
            pub allowas_in: ::data_views::Field<item::AllowasIn<'a, Mode>>,
            #[data_view(schema = Int(110))]
            pub weight: ::data_views::Field<i64>,
            #[data_view(schema = Str(777))]
            pub timers: ::data_views::Field<&'a str>,
            #[data_view(schema = Dict(925))]
            pub rib_in_pre_policy_retain: ::data_views::Field<item::RibInPrePolicyRetain<'a, Mode>>,
            #[data_view(schema = Str(778))]
            pub route_map_in: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(779))]
            pub route_map_out: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(780))]
            pub peer_tag_in: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(781))]
            pub peer_tag_out_discard: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(3))]
            pub session_tracker: ::data_views::Field<&'a str>,
            #[data_view(schema = Dict(926))]
            pub shared_secret: ::data_views::Field<item::SharedSecret<'a, Mode>>,
            #[data_view(schema = Int(387))]
            pub ttl_maximum_hops: ::data_views::Field<i64>,
            #[data_view(schema = Int(388))]
            pub maximum_advertised_routes: ::data_views::Field<i64>,
            #[data_view(schema = Str(784))]
            pub maximum_advertised_routes_warning_limit: ::data_views::Field<&'a str>,
        }

        pub mod item {

            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.BgpPeerGroupsItemBase.Nodes", schema = List(687)))]
            pub struct Nodes<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.BgpPeerGroupsItemBase.AddressFamilyIpv4", schema = Dict(1634)))]
            pub struct AddressFamilyIpv4<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub activate: ::data_views::Field<bool>,
                #[data_view(schema = Str(778))]
                pub route_map_in: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(779))]
                pub route_map_out: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(803))]
                pub rcf_in: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(804))]
                pub rcf_out: ::data_views::Field<&'a str>,
                #[data_view(schema = Dict(977))]
                pub default_originate: ::data_views::Field<super::super::super::super::super::eos_config::router_bgp::address_family_ipv4::peer_groups::item::DefaultOriginate<'a, Mode>>,
                #[data_view(schema = Dict(980))]
                pub next_hop: ::data_views::Field<super::super::super::super::super::eos_config::router_bgp::address_family_ipv4::peer_groups::item::NextHop<'a, Mode>>,
                #[data_view(schema = Str(812))]
                pub prefix_list_in: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(813))]
                pub prefix_list_out: ::data_views::Field<&'a str>,
            }

            pub mod address_family_ipv4 {
            }

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.BgpPeerGroupsItemBase.AddressFamilyIpv6", schema = Dict(1636)))]
            pub struct AddressFamilyIpv6<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub activate: ::data_views::Field<bool>,
                #[data_view(schema = Str(778))]
                pub route_map_in: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(779))]
                pub route_map_out: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(803))]
                pub rcf_in: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(804))]
                pub rcf_out: ::data_views::Field<&'a str>,
                #[data_view(schema = Dict(1635))]
                pub default_originate: ::data_views::Field<address_family_ipv6::DefaultOriginate<'a, Mode>>,
                #[data_view(schema = Str(812))]
                pub prefix_list_in: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(813))]
                pub prefix_list_out: ::data_views::Field<&'a str>,
            }

            pub mod address_family_ipv6 {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.BgpPeerGroupsItemBase.AddressFamilyIpv6.DefaultOriginate", schema = Dict(1635)))]
                pub struct DefaultOriginate<'a, Mode> {
                    #[data_view(schema = Bool(1))]
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    #[data_view(schema = Bool(336))]
                    pub always: ::data_views::Field<bool>,
                    #[data_view(schema = Str(169))]
                    pub route_map: ::data_views::Field<&'a str>,
                }
            }

            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.BgpPeerGroupsItemBase.ListenRanges", schema = List(688)))]
            pub struct ListenRanges<'a, Mode> (::data_views::Field<listen_ranges::Item<'a, Mode>>);

            pub mod listen_ranges {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.BgpPeerGroupsItemBase.ListenRangesItem", schema = Dict(1637)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(1477))]
                    pub prefix: ::data_views::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Str(1478))]
                    pub remote_as: ::data_views::Field<&'a str>,
                    #[data_view(schema = Bool(320))]
                    pub peer_id_include_router_id: ::data_views::Field<bool>,
                    #[data_view(schema = Str(1479))]
                    pub peer_filter: ::data_views::Field<&'a str>,
                }
            }

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.BgpPeerGroupsItemBase.Metadata", schema = Dict(912)))]
            pub struct Metadata<'a, Mode> {
                #[data_view(schema = Str(278), rename = "type")]
                pub field_type: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.BgpPeerGroupsItemBase.AsPath", schema = Dict(913)))]
            pub struct AsPath<'a, Mode> {
                #[data_view(schema = Bool(321))]
                pub remote_as_replace_out: ::data_views::Field<bool>,
                #[data_view(schema = Bool(322))]
                pub prepend_own_disabled: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.BgpPeerGroupsItemBase.RemovePrivateAs", schema = Dict(914)))]
            pub struct RemovePrivateAs<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::data_views::Field<bool>,
                #[data_view(schema = Bool(7))]
                pub all: ::data_views::Field<bool>,
                #[data_view(schema = Bool(7))]
                pub replace_as: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.BgpPeerGroupsItemBase.RemovePrivateAsIngress", schema = Dict(915)))]
            pub struct RemovePrivateAsIngress<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::data_views::Field<bool>,
                #[data_view(schema = Bool(7))]
                pub replace_as: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.BgpPeerGroupsItemBase.BfdTimers", schema = Dict(916)))]
            pub struct BfdTimers<'a, Mode> {
                #[data_view(schema = Int(378))]
                pub interval: ::data_views::RequiredValue<i64, Mode>,
                #[data_view(schema = Int(379))]
                pub min_rx: ::data_views::RequiredValue<i64, Mode>,
                #[data_view(schema = Int(380))]
                pub multiplier: ::data_views::RequiredValue<i64, Mode>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.BgpPeerGroupsItemBase.DefaultOriginate", schema = Dict(917)))]
            pub struct DefaultOriginate<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::data_views::Field<bool>,
                #[data_view(schema = Bool(7))]
                pub always: ::data_views::Field<bool>,
                #[data_view(schema = Str(169))]
                pub route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.BgpPeerGroupsItemBase.MaximumAcceptedRoutes", schema = Dict(919)))]
            pub struct MaximumAcceptedRoutes<'a, Mode> {
                #[data_view(schema = Int(383))]
                pub limit: ::data_views::RequiredValue<i64, Mode>,
                #[data_view(schema = Dict(918))]
                pub warning_limit: ::data_views::Field<maximum_accepted_routes::WarningLimit<'a, Mode>>,
            }

            pub mod maximum_accepted_routes {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.BgpPeerGroupsItemBase.MaximumAcceptedRoutes.WarningLimit", schema = Dict(918)))]
                pub struct WarningLimit<'a, Mode> {
                    #[data_view(schema = Int(384))]
                    pub count: ::data_views::Field<i64>,
                    #[data_view(schema = Int(385))]
                    pub percent: ::data_views::Field<i64>,
                }
            }

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.BgpPeerGroupsItemBase.MissingPolicy", schema = Dict(922)))]
            pub struct MissingPolicy<'a, Mode> {
                #[data_view(schema = Dict(920))]
                pub direction_in: ::data_views::Field<missing_policy::DirectionIn<'a, Mode>>,
                #[data_view(schema = Dict(921))]
                pub direction_out: ::data_views::Field<missing_policy::DirectionOut<'a, Mode>>,
            }

            pub mod missing_policy {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.BgpPeerGroupsItemBase.MissingPolicy.DirectionIn", schema = Dict(920)))]
                pub struct DirectionIn<'a, Mode> {
                    #[data_view(schema = Str(775))]
                    pub action: ::data_views::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Bool(325))]
                    pub include_community_list: ::data_views::Field<bool>,
                    #[data_view(schema = Bool(326))]
                    pub include_prefix_list: ::data_views::Field<bool>,
                    #[data_view(schema = Bool(327))]
                    pub include_sub_route_map: ::data_views::Field<bool>,
                }

                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.BgpPeerGroupsItemBase.MissingPolicy.DirectionOut", schema = Dict(921)))]
                pub struct DirectionOut<'a, Mode> {
                    #[data_view(schema = Str(775))]
                    pub action: ::data_views::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Bool(325))]
                    pub include_community_list: ::data_views::Field<bool>,
                    #[data_view(schema = Bool(326))]
                    pub include_prefix_list: ::data_views::Field<bool>,
                    #[data_view(schema = Bool(327))]
                    pub include_sub_route_map: ::data_views::Field<bool>,
                }
            }

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.BgpPeerGroupsItemBase.LinkBandwidth", schema = Dict(923)))]
            pub struct LinkBandwidth<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::data_views::Field<bool>,
                #[data_view(schema = Str(776))]
                pub default: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.BgpPeerGroupsItemBase.AllowasIn", schema = Dict(924)))]
            pub struct AllowasIn<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::data_views::Field<bool>,
                #[data_view(schema = Int(386))]
                pub times: ::data_views::Field<i64>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.BgpPeerGroupsItemBase.RibInPrePolicyRetain", schema = Dict(925)))]
            pub struct RibInPrePolicyRetain<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::data_views::Field<bool>,
                #[data_view(schema = Bool(7))]
                pub all: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.BgpPeerGroupsItemBase.SharedSecret", schema = Dict(926)))]
            pub struct SharedSecret<'a, Mode> {
                #[data_view(schema = Str(782))]
                pub profile: ::data_views::RequiredValue<&'a str, Mode>,
                #[data_view(schema = Str(783))]
                pub hash_algorithm: ::data_views::RequiredValue<&'a str, Mode>,
            }
        }
    }

    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.IgmpSnooping", schema = Dict(1640)))]
    pub struct IgmpSnooping<'a, Mode> {
        #[data_view(schema = Dict(1639))]
        pub querier: ::data_views::Field<igmp_snooping::Querier<'a, Mode>>,
        #[data_view(schema = Bool(562))]
        pub fast_leave: ::data_views::Field<bool>,
    }

    pub mod igmp_snooping {

        #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.IgmpSnooping.Querier", schema = Dict(1639)))]
        pub struct Querier<'a, Mode> {
            #[data_view(schema = Bool(547))]
            pub enabled: ::data_views::Field<bool>,
            #[data_view(schema = Str(1480))]
            pub source_address: ::data_views::Field<&'a str>,
            #[data_view(schema = Int(595))]
            pub version: ::data_views::Field<i64>,
        }
    }

    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.EvpnL2Multicast", schema = Dict(1641)))]
    pub struct EvpnL2Multicast<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub enabled: ::data_views::Field<bool>,
        #[data_view(schema = Str(1170))]
        pub underlay_l2_multicast_group_ipv4_pool: ::data_views::Field<&'a str>,
        #[data_view(schema = Int(603))]
        pub underlay_l2_multicast_group_ipv4_pool_offset: ::data_views::Field<i64>,
        #[data_view(schema = Bool(563))]
        pub fast_leave: ::data_views::Field<bool>,
        #[data_view(schema = Bool(564))]
        pub always_redistribute_igmp: ::data_views::Field<bool>,
    }

    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VxlanFloodMulticast", schema = Dict(1642)))]
    pub struct VxlanFloodMulticast<'a, Mode> {
        #[data_view(schema = Bool(565))]
        pub enabled: ::data_views::RequiredValue<bool, Mode>,
        #[data_view(schema = Str(1481))]
        pub underlay_l2_multicast_group_ipv4_pool: ::data_views::Field<&'a str>,
        #[data_view(schema = Int(603))]
        pub underlay_l2_multicast_group_ipv4_pool_offset: ::data_views::Field<i64>,
    }

    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.EvpnL3Multicast", schema = Dict(1644)))]
    pub struct EvpnL3Multicast<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub enabled: ::data_views::Field<bool>,
        #[data_view(schema = Str(1482))]
        pub evpn_underlay_l3_multicast_group_ipv4_pool: ::data_views::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Int(603))]
        pub evpn_underlay_l3_multicast_group_ipv4_pool_offset: ::data_views::Field<i64>,
        #[data_view(schema = List(691))]
        pub evpn_peg: ::data_views::Field<evpn_l3_multicast::EvpnPeg<'a, Mode>>,
    }

    pub mod evpn_l3_multicast {

        #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.EvpnL3Multicast.EvpnPeg", schema = List(691)))]
        pub struct EvpnPeg<'a, Mode> (::data_views::Field<evpn_peg::Item<'a, Mode>>);

        pub mod evpn_peg {

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.EvpnL3Multicast.EvpnPegItem", schema = Dict(1643)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = List(690))]
                pub nodes: ::data_views::Field<item::Nodes<'a, Mode>>,
                #[data_view(schema = Bool(566))]
                pub transit: ::data_views::Field<bool>,
            }

            pub mod item {

                #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.EvpnL3Multicast.EvpnPegItem.Nodes", schema = List(690)))]
                pub struct Nodes<'a, Mode> (::data_views::Field<&'a str>);
            }
        }
    }

    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.PimRpAddresses", schema = List(695)))]
    pub struct PimRpAddresses<'a, Mode> (::data_views::Field<pim_rp_addresses::Item<'a, Mode>>);

    pub mod pim_rp_addresses {

        #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.PimRpAddressesItem", schema = Dict(1645)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = List(692))]
            pub rps: ::data_views::Field<item::Rps<'a, Mode>>,
            #[data_view(schema = List(693))]
            pub nodes: ::data_views::Field<item::Nodes<'a, Mode>>,
            #[data_view(schema = List(694))]
            pub groups: ::data_views::Field<item::Groups<'a, Mode>>,
            #[data_view(schema = Str(1485))]
            pub access_list_name: ::data_views::Field<&'a str>,
        }

        pub mod item {

            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.PimRpAddressesItem.Rps", schema = List(692)))]
            pub struct Rps<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.PimRpAddressesItem.Nodes", schema = List(693)))]
            pub struct Nodes<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.PimRpAddressesItem.Groups", schema = List(694)))]
            pub struct Groups<'a, Mode> (::data_views::Field<&'a str>);
        }
    }

    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.IgmpSnoopingQuerier", schema = Dict(1646)))]
    pub struct IgmpSnoopingQuerier<'a, Mode> {
        #[data_view(schema = Bool(567))]
        pub enabled: ::data_views::Field<bool>,
        #[data_view(schema = Str(1480))]
        pub source_address: ::data_views::Field<&'a str>,
        #[data_view(schema = Int(595))]
        pub version: ::data_views::Field<i64>,
    }

    #[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.NetworkServicesItemBase.Vrfs", schema = List(745), item_name = "AVDDesign.NetworkServicesItemBase.VrfsItem"))]
    pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

    pub mod vrfs {

        #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase", schema = Dict(1696)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(49))]
            pub name: ::data_views::Field<&'a str>,
            #[data_view(schema = List(696))]
            pub address_families: ::data_views::Field<item::AddressFamilies<'a, Mode>>,
            #[data_view(schema = Str(1486))]
            pub description: ::data_views::Field<&'a str>,
            #[data_view(schema = Int(604))]
            pub vrf_vni: ::data_views::Field<i64>,
            #[data_view(schema = Int(605))]
            pub vrf_id: ::data_views::Field<i64>,
            #[data_view(schema = Str(1487))]
            pub rd_override: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1488))]
            pub rt_override: ::data_views::Field<&'a str>,
            #[data_view(schema = Bool(569))]
            pub rt_import: ::data_views::Field<bool>,
            #[data_view(schema = Bool(570))]
            pub rt_export: ::data_views::Field<bool>,
            #[data_view(schema = Bool(571))]
            pub rt_import_evpn_remote: ::data_views::Field<bool>,
            #[data_view(schema = Bool(572))]
            pub rt_export_evpn_remote: ::data_views::Field<bool>,
            #[data_view(schema = Str(1489))]
            pub evpn_vlan_bundle: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1490))]
            pub mlag_ibgp_peering_ipv4_pool: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1491))]
            pub mlag_ibgp_peering_ipv6_pool: ::data_views::Field<&'a str>,
            #[data_view(schema = List(697))]
            pub ip_helpers: ::data_views::Field<item::IpHelpers<'a, Mode>>,
            #[data_view(schema = Bool(573))]
            pub enable_mlag_ibgp_peering_vrfs: ::data_views::Field<bool>,
            #[data_view(schema = Bool(574))]
            pub redistribute_mlag_ibgp_peering_vrfs: ::data_views::Field<bool>,
            #[data_view(schema = Int(606))]
            pub mlag_ibgp_peering_vlan: ::data_views::Field<i64>,
            #[data_view(schema = Dict(1649))]
            pub vtep_diagnostic: ::data_views::Field<item::VtepDiagnostic<'a, Mode>>,
            #[data_view(schema = Dict(1654))]
            pub ospf: ::data_views::Field<item::Ospf<'a, Mode>>,
            #[data_view(schema = Bool(576))]
            pub redistribute_ospf: ::data_views::Field<bool>,
            #[data_view(schema = Dict(1656))]
            pub evpn_l3_multicast: ::data_views::Field<item::EvpnL3Multicast<'a, Mode>>,
            #[data_view(schema = List(704))]
            pub pim_rp_addresses: ::data_views::Field<item::PimRpAddresses<'a, Mode>>,
            #[data_view(schema = Bool(578))]
            pub evpn_l2_multi_domain: ::data_views::Field<bool>,
            #[data_view(schema = List(716))]
            pub svis: ::data_views::Field<item::Svis<'a, Mode>>,
            #[data_view(schema = List(725))]
            pub l3_interfaces: ::data_views::Field<item::L3Interfaces<'a, Mode>>,
            #[data_view(schema = List(731))]
            pub l3_port_channels: ::data_views::Field<item::L3PortChannels<'a, Mode>>,
            #[data_view(schema = List(732))]
            pub loopbacks: ::data_views::Field<item::Loopbacks<'a, Mode>>,
            #[data_view(schema = List(733))]
            pub static_routes: ::data_views::Field<item::StaticRoutes<'a, Mode>>,
            #[data_view(schema = List(734))]
            pub ipv6_static_routes: ::data_views::Field<item::Ipv6StaticRoutes<'a, Mode>>,
            #[data_view(schema = Bool(591))]
            pub redistribute_static: ::data_views::Field<bool>,
            #[data_view(schema = Bool(592))]
            pub redistribute_connected: ::data_views::Field<bool>,
            #[data_view(schema = List(736))]
            pub static_arp_entries: ::data_views::Field<item::StaticArpEntries<'a, Mode>>,
            #[data_view(schema = List(738))]
            pub bgp_peers: ::data_views::Field<item::BgpPeers<'a, Mode>>,
            #[data_view(schema = Dict(1691))]
            pub bgp: ::data_views::Field<item::Bgp<'a, Mode>>,
            #[data_view(schema = List(740))]
            pub bgp_peer_groups: ::data_views::Field<item::BgpPeerGroups<'a, Mode>>,
            #[data_view(schema = List(742))]
            pub additional_route_targets: ::data_views::Field<item::AdditionalRouteTargets<'a, Mode>>,
            #[data_view(schema = List(744))]
            pub aggregate_addresses: ::data_views::Field<item::AggregateAddresses<'a, Mode>>,
            #[data_view(schema = Bool(596))]
            pub validate_bgp_peers: ::data_views::Field<bool>,
            #[data_view(schema = Str(1568))]
            pub raw_eos_cli: ::data_views::Field<&'a str>,
            #[data_view(schema = Dict(1695), relaxed)]
            pub structured_config: ::data_views::Field<super::super::super::super::eos_config::EosConfig<'a, ::data_views::RelaxedValidated>>,
        }

        pub mod item {

            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.AddressFamilies", schema = List(696)))]
            pub struct AddressFamilies<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(indexed_list, primary_key(ip_helper), python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.IpHelpers", schema = List(697), item_name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.IpHelpersItem"))]
            pub struct IpHelpers<'a, Mode> (::data_views::Field<ip_helpers::Item<'a, Mode>>);

            pub mod ip_helpers {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.IpHelpersItemBase", schema = Dict(1647)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(1492))]
                    pub ip_helper: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(1493))]
                    pub source_interface: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(1494))]
                    pub source_vrf: ::data_views::Field<&'a str>,
                }
            }

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.VtepDiagnostic", schema = Dict(1649)))]
            pub struct VtepDiagnostic<'a, Mode> {
                #[data_view(schema = Int(607))]
                pub loopback: ::data_views::Field<i64>,
                #[data_view(schema = Str(1495))]
                pub loopback_description: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(1496))]
                pub loopback_ip_range: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(1497))]
                pub loopback_ipv6_range: ::data_views::Field<&'a str>,
                #[data_view(schema = List(698))]
                pub loopback_ip_pools: ::data_views::Field<vtep_diagnostic::LoopbackIpPools<'a, Mode>>,
                #[data_view(schema = Bool(575))]
                pub hardware_forwarding: ::data_views::Field<bool>,
            }

            pub mod vtep_diagnostic {

                #[::data_views::data_view(indexed_list, primary_key(pod), python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.VtepDiagnostic.LoopbackIpPools", schema = List(698), item_name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.VtepDiagnostic.LoopbackIpPoolsItem"))]
                pub struct LoopbackIpPools<'a, Mode> (::data_views::Field<loopback_ip_pools::Item<'a, Mode>>);

                pub mod loopback_ip_pools {

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.VtepDiagnostic.LoopbackIpPoolsItemBase", schema = Dict(1648)))]
                    pub struct Item<'a, Mode> {
                        #[data_view(schema = Str(1498))]
                        pub pod: ::data_views::Field<&'a str>,
                        #[data_view(schema = Str(1170))]
                        pub ipv4_pool: ::data_views::Field<&'a str>,
                        #[data_view(schema = Str(1171))]
                        pub ipv6_pool: ::data_views::Field<&'a str>,
                    }
                }
            }

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.Ospf", schema = Dict(1654)))]
            pub struct Ospf<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::data_views::Field<bool>,
                #[data_view(schema = Int(608))]
                pub process_id: ::data_views::Field<i64>,
                #[data_view(schema = Str(1499))]
                pub router_id: ::data_views::Field<&'a str>,
                #[data_view(schema = Int(77))]
                pub max_lsa: ::data_views::Field<i64>,
                #[data_view(schema = Bool(340))]
                pub bfd: ::data_views::Field<bool>,
                #[data_view(schema = Dict(1650))]
                pub redistribute_bgp: ::data_views::Field<ospf::RedistributeBgp<'a, Mode>>,
                #[data_view(schema = Dict(1651))]
                pub redistribute_connected: ::data_views::Field<ospf::RedistributeConnected<'a, Mode>>,
                #[data_view(schema = Str(1500))]
                pub authentication: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(1501))]
                pub cleartext_simple_auth_key: ::data_views::Field<&'a str>,
                #[data_view(schema = List(699))]
                pub message_digest_keys: ::data_views::Field<ospf::MessageDigestKeys<'a, Mode>>,
                #[data_view(schema = List(700))]
                pub nodes: ::data_views::Field<ospf::Nodes<'a, Mode>>,
                #[data_view(schema = Dict(1653), relaxed)]
                pub structured_config: ::data_views::Field<super::super::super::super::super::eos_config::router_ospf::process_ids::Item<'a, ::data_views::RelaxedValidated>>,
            }

            pub mod ospf {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.Ospf.RedistributeBgp", schema = Dict(1650)))]
                pub struct RedistributeBgp<'a, Mode> {
                    #[data_view(schema = Bool(37))]
                    pub enabled: ::data_views::Field<bool>,
                    #[data_view(schema = Str(169))]
                    pub route_map: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.Ospf.RedistributeConnected", schema = Dict(1651)))]
                pub struct RedistributeConnected<'a, Mode> {
                    #[data_view(schema = Bool(340))]
                    pub enabled: ::data_views::Field<bool>,
                    #[data_view(schema = Str(169))]
                    pub route_map: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view(indexed_list, primary_key(id), python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.Ospf.MessageDigestKeys", schema = List(699), item_name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.Ospf.MessageDigestKeysItem"))]
                pub struct MessageDigestKeys<'a, Mode> (::data_views::Field<message_digest_keys::Item<'a, Mode>>);

                pub mod message_digest_keys {

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.Ospf.MessageDigestKeysItemBase", schema = Dict(1652)))]
                    pub struct Item<'a, Mode> {
                        #[data_view(schema = Int(77))]
                        pub id: ::data_views::Field<i64>,
                        #[data_view(schema = Str(1502))]
                        pub hash_algorithm: ::data_views::Field<&'a str>,
                        #[data_view(schema = Str(1503))]
                        pub cleartext_key: ::data_views::RequiredValue<&'a str, Mode>,
                    }
                }

                #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.Ospf.Nodes", schema = List(700)))]
                pub struct Nodes<'a, Mode> (::data_views::Field<&'a str>);
            }

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.EvpnL3Multicast", schema = Dict(1656)))]
            pub struct EvpnL3Multicast<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::data_views::Field<bool>,
                #[data_view(schema = Str(1504))]
                pub evpn_underlay_l3_multicast_group: ::data_views::Field<&'a str>,
                #[data_view(schema = List(702))]
                pub evpn_peg: ::data_views::Field<evpn_l3_multicast::EvpnPeg<'a, Mode>>,
            }

            pub mod evpn_l3_multicast {

                #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.EvpnL3Multicast.EvpnPeg", schema = List(702)))]
                pub struct EvpnPeg<'a, Mode> (::data_views::Field<evpn_peg::Item<'a, Mode>>);

                pub mod evpn_peg {

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.EvpnL3Multicast.EvpnPegItem", schema = Dict(1655)))]
                    pub struct Item<'a, Mode> {
                        #[data_view(schema = List(701))]
                        pub nodes: ::data_views::Field<item::Nodes<'a, Mode>>,
                        #[data_view(schema = Bool(577))]
                        pub transit: ::data_views::Field<bool>,
                    }

                    pub mod item {

                        #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.EvpnL3Multicast.EvpnPegItem.Nodes", schema = List(701)))]
                        pub struct Nodes<'a, Mode> (::data_views::Field<&'a str>);
                    }
                }
            }

            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.PimRpAddresses", schema = List(704)))]
            pub struct PimRpAddresses<'a, Mode> (::data_views::Field<pim_rp_addresses::Item<'a, Mode>>);

            pub mod pim_rp_addresses {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.PimRpAddressesItem", schema = Dict(1657)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = List(703))]
                    pub rps: ::data_views::Field<item::Rps<'a, Mode>>,
                    #[data_view(schema = List(693))]
                    pub nodes: ::data_views::Field<item::Nodes<'a, Mode>>,
                    #[data_view(schema = List(694))]
                    pub groups: ::data_views::Field<item::Groups<'a, Mode>>,
                    #[data_view(schema = Str(1505))]
                    pub access_list_name: ::data_views::Field<&'a str>,
                }

                pub mod item {

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.PimRpAddressesItem.Rps", schema = List(703)))]
                    pub struct Rps<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.PimRpAddressesItem.Nodes", schema = List(693)))]
                    pub struct Nodes<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.PimRpAddressesItem.Groups", schema = List(694)))]
                    pub struct Groups<'a, Mode> (::data_views::Field<&'a str>);
                }
            }

            #[::data_views::data_view(list, primary_key(id), python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.Svis", schema = List(716), item_name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItem"))]
            pub struct Svis<'a, Mode> (::data_views::Field<svis::Item<'a, Mode>>);

            pub mod svis {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase", schema = Dict(1674)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Int(609))]
                    pub id: ::data_views::Field<i64>,
                    #[data_view(schema = Str(1506))]
                    pub name: ::data_views::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Dict(1658))]
                    pub address_locking: ::data_views::Field<super::super::super::super::super::super::eos_config::vlans::item::address_locking::AddressFamily<'a, Mode>>,
                    #[data_view(schema = Str(1507))]
                    pub profile: ::data_views::Field<&'a str>,
                    #[data_view(schema = List(705))]
                    pub tags: ::data_views::Field<item::Tags<'a, Mode>>,
                    #[data_view(schema = Str(1509))]
                    pub evpn_vlan_bundle: ::data_views::Field<&'a str>,
                    #[data_view(schema = List(715))]
                    pub nodes: ::data_views::Field<item::Nodes<'a, Mode>>,
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
                    pub structured_config: ::data_views::Field<super::super::super::super::super::super::eos_config::vlan_interfaces::Item<'a, ::data_views::RelaxedValidated>>,
                    #[data_view(schema = Bool(588))]
                    pub evpn_l2_multi_domain: ::data_views::Field<bool>,
                }

                pub mod item {

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Tags", schema = List(705)))]
                    pub struct Tags<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(indexed_list, primary_key(node), python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Nodes", schema = List(715), item_name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItem"))]
                    pub struct Nodes<'a, Mode> (::data_views::Field<nodes::Item<'a, Mode>>);

                    pub mod nodes {

                        #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase", schema = Dict(1673)))]
                        pub struct Item<'a, Mode> {
                            #[data_view(schema = Str(1510))]
                            pub node: ::data_views::Field<&'a str>,
                            #[data_view(schema = List(705))]
                            pub tags: ::data_views::Field<item::Tags<'a, Mode>>,
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
                            pub structured_config: ::data_views::Field<super::super::super::super::super::super::super::super::eos_config::vlan_interfaces::Item<'a, ::data_views::RelaxedValidated>>,
                            #[data_view(schema = Bool(588))]
                            pub evpn_l2_multi_domain: ::data_views::Field<bool>,
                        }

                        pub mod item {

                            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Tags", schema = List(705)))]
                            pub struct Tags<'a, Mode> (::data_views::Field<&'a str>);

                            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.IpAddressSecondaries", schema = List(222)))]
                            pub struct IpAddressSecondaries<'a, Mode> (::data_views::Field<&'a str>);

                            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6AddressVirtuals", schema = List(706)))]
                            pub struct Ipv6AddressVirtuals<'a, Mode> (::data_views::Field<&'a str>);

                            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6Nd", schema = Dict(1660)))]
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

                                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6Nd.RaDnsServers", schema = Dict(1659)))]
                                pub struct RaDnsServers<'a, Mode> {
                                    #[data_view(schema = List(105))]
                                    pub servers: ::data_views::Field<ra_dns_servers::Servers<'a, Mode>>,
                                    #[data_view(schema = Int(95))]
                                    pub dns_servers_lifetime: ::data_views::Field<i64>,
                                }

                                pub mod ra_dns_servers {

                                    #[::data_views::data_view(indexed_list, primary_key(address), python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6Nd.RaDnsServers.Servers", schema = List(105), item_name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6Nd.RaDnsServers.ServersItem"))]
                                    pub struct Servers<'a, Mode> (::data_views::Field<servers::Item<'a, Mode>>);

                                    pub mod servers {

                                        #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6Nd.RaDnsServers.ServersItemBase", schema = Dict(236)))]
                                        pub struct Item<'a, Mode> {
                                            #[data_view(schema = Str(208))]
                                            pub address: ::data_views::Field<&'a str>,
                                            #[data_view(schema = Int(94))]
                                            pub lifetime: ::data_views::Field<i64>,
                                        }
                                    }
                                }
                            }

                            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6DhcpRelay", schema = Dict(1662)))]
                            pub struct Ipv6DhcpRelay<'a, Mode> {
                                #[data_view(schema = List(707))]
                                pub destinations: ::data_views::Field<ipv6_dhcp_relay::Destinations<'a, Mode>>,
                            }

                            pub mod ipv6_dhcp_relay {

                                #[::data_views::data_view(indexed_list, primary_key(address), python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6DhcpRelay.Destinations", schema = List(707), item_name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6DhcpRelay.DestinationsItem"))]
                                pub struct Destinations<'a, Mode> (::data_views::Field<destinations::Item<'a, Mode>>);

                                pub mod destinations {

                                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6DhcpRelay.DestinationsItemBase", schema = Dict(1661)))]
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

                            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.IpAddressVirtualSecondaries", schema = List(708)))]
                            pub struct IpAddressVirtualSecondaries<'a, Mode> (::data_views::Field<&'a str>);

                            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.IpVirtualRouterAddresses", schema = List(709)))]
                            pub struct IpVirtualRouterAddresses<'a, Mode> (::data_views::Field<&'a str>);

                            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6VirtualRouterAddresses", schema = List(710)))]
                            pub struct Ipv6VirtualRouterAddresses<'a, Mode> (::data_views::Field<&'a str>);

                            #[::data_views::data_view(indexed_list, primary_key(ip_helper), python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.IpHelpers", schema = List(697), item_name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.IpHelpersItem"))]
                            pub struct IpHelpers<'a, Mode> (::data_views::Field<ip_helpers::Item<'a, Mode>>);

                            pub mod ip_helpers {

                                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.IpHelpersItemBase", schema = Dict(1647)))]
                                pub struct Item<'a, Mode> {
                                    #[data_view(schema = Str(1492))]
                                    pub ip_helper: ::data_views::Field<&'a str>,
                                    #[data_view(schema = Str(1493))]
                                    pub source_interface: ::data_views::Field<&'a str>,
                                    #[data_view(schema = Str(1494))]
                                    pub source_vrf: ::data_views::Field<&'a str>,
                                }
                            }

                            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.StaticRoutes", schema = List(711)))]
                            pub struct StaticRoutes<'a, Mode> (::data_views::Field<static_routes::Item<'a, Mode>>);

                            pub mod static_routes {

                                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.StaticRoutesItem", schema = Dict(1663)))]
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

                            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6StaticRoutes", schema = List(712)))]
                            pub struct Ipv6StaticRoutes<'a, Mode> (::data_views::Field<ipv6_static_routes::Item<'a, Mode>>);

                            pub mod ipv6_static_routes {

                                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6StaticRoutesItem", schema = Dict(1663)))]
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

                            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.TrunkGroups", schema = List(713)))]
                            pub struct TrunkGroups<'a, Mode> (::data_views::Field<&'a str>);

                            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.EvpnL2Multicast", schema = Dict(1664)))]
                            pub struct EvpnL2Multicast<'a, Mode> {
                                #[data_view(schema = Bool(7))]
                                pub enabled: ::data_views::Field<bool>,
                                #[data_view(schema = Bool(583))]
                                pub always_redistribute_igmp: ::data_views::Field<bool>,
                            }

                            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.VxlanFloodMulticast", schema = Dict(1665)))]
                            pub struct VxlanFloodMulticast<'a, Mode> {
                                #[data_view(schema = Bool(585))]
                                pub enabled: ::data_views::Field<bool>,
                                #[data_view(schema = Str(1528))]
                                pub underlay_multicast_group: ::data_views::Field<&'a str>,
                            }

                            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.EvpnL3Multicast", schema = Dict(1666)))]
                            pub struct EvpnL3Multicast<'a, Mode> {
                                #[data_view(schema = Bool(7))]
                                pub enabled: ::data_views::Field<bool>,
                            }

                            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.IgmpSnooping", schema = Dict(1608)))]
                            pub struct IgmpSnooping<'a, Mode> {
                                #[data_view(schema = Bool(546))]
                                pub enabled: ::data_views::Field<bool>,
                                #[data_view(schema = Dict(1607))]
                                pub querier: ::data_views::Field<igmp_snooping::Querier<'a, Mode>>,
                                #[data_view(schema = Bool(548))]
                                pub fast_leave: ::data_views::Field<bool>,
                            }

                            pub mod igmp_snooping {

                                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.IgmpSnooping.Querier", schema = Dict(1607)))]
                                pub struct Querier<'a, Mode> {
                                    #[data_view(schema = Bool(547))]
                                    pub enabled: ::data_views::Field<bool>,
                                    #[data_view(schema = Str(1437))]
                                    pub source_address: ::data_views::Field<&'a str>,
                                    #[data_view(schema = Int(595))]
                                    pub version: ::data_views::Field<i64>,
                                }
                            }

                            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.IgmpSnoopingQuerier", schema = Dict(1667)))]
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

                            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ospf", schema = Dict(1669)))]
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

                                #[::data_views::data_view(indexed_list, primary_key(id), python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ospf.MessageDigestKeys", schema = List(714), item_name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ospf.MessageDigestKeysItem"))]
                                pub struct MessageDigestKeys<'a, Mode> (::data_views::Field<message_digest_keys::Item<'a, Mode>>);

                                pub mod message_digest_keys {

                                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ospf.MessageDigestKeysItemBase", schema = Dict(1668)))]
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

                            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Bgp", schema = Dict(1671)))]
                            pub struct Bgp<'a, Mode> {
                                #[data_view(schema = Dict(1670), relaxed)]
                                pub structured_config: ::data_views::Field<super::super::super::super::super::super::super::super::super::eos_config::router_bgp::vlans::Item<'a, ::data_views::RelaxedValidated>>,
                                #[data_view(schema = Str(1535))]
                                pub raw_eos_cli: ::data_views::Field<&'a str>,
                            }

                            pub mod bgp {
                            }
                        }
                    }

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.IpAddressSecondaries", schema = List(222)))]
                    pub struct IpAddressSecondaries<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6AddressVirtuals", schema = List(706)))]
                    pub struct Ipv6AddressVirtuals<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6Nd", schema = Dict(1660)))]
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

                        #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6Nd.RaDnsServers", schema = Dict(1659)))]
                        pub struct RaDnsServers<'a, Mode> {
                            #[data_view(schema = List(105))]
                            pub servers: ::data_views::Field<ra_dns_servers::Servers<'a, Mode>>,
                            #[data_view(schema = Int(95))]
                            pub dns_servers_lifetime: ::data_views::Field<i64>,
                        }

                        pub mod ra_dns_servers {

                            #[::data_views::data_view(indexed_list, primary_key(address), python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6Nd.RaDnsServers.Servers", schema = List(105), item_name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6Nd.RaDnsServers.ServersItem"))]
                            pub struct Servers<'a, Mode> (::data_views::Field<servers::Item<'a, Mode>>);

                            pub mod servers {

                                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6Nd.RaDnsServers.ServersItemBase", schema = Dict(236)))]
                                pub struct Item<'a, Mode> {
                                    #[data_view(schema = Str(208))]
                                    pub address: ::data_views::Field<&'a str>,
                                    #[data_view(schema = Int(94))]
                                    pub lifetime: ::data_views::Field<i64>,
                                }
                            }
                        }
                    }

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6DhcpRelay", schema = Dict(1662)))]
                    pub struct Ipv6DhcpRelay<'a, Mode> {
                        #[data_view(schema = List(707))]
                        pub destinations: ::data_views::Field<ipv6_dhcp_relay::Destinations<'a, Mode>>,
                    }

                    pub mod ipv6_dhcp_relay {

                        #[::data_views::data_view(indexed_list, primary_key(address), python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6DhcpRelay.Destinations", schema = List(707), item_name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6DhcpRelay.DestinationsItem"))]
                        pub struct Destinations<'a, Mode> (::data_views::Field<destinations::Item<'a, Mode>>);

                        pub mod destinations {

                            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6DhcpRelay.DestinationsItemBase", schema = Dict(1661)))]
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

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.IpAddressVirtualSecondaries", schema = List(708)))]
                    pub struct IpAddressVirtualSecondaries<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.IpVirtualRouterAddresses", schema = List(709)))]
                    pub struct IpVirtualRouterAddresses<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6VirtualRouterAddresses", schema = List(710)))]
                    pub struct Ipv6VirtualRouterAddresses<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(indexed_list, primary_key(ip_helper), python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.IpHelpers", schema = List(697), item_name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.IpHelpersItem"))]
                    pub struct IpHelpers<'a, Mode> (::data_views::Field<ip_helpers::Item<'a, Mode>>);

                    pub mod ip_helpers {

                        #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.IpHelpersItemBase", schema = Dict(1647)))]
                        pub struct Item<'a, Mode> {
                            #[data_view(schema = Str(1492))]
                            pub ip_helper: ::data_views::Field<&'a str>,
                            #[data_view(schema = Str(1493))]
                            pub source_interface: ::data_views::Field<&'a str>,
                            #[data_view(schema = Str(1494))]
                            pub source_vrf: ::data_views::Field<&'a str>,
                        }
                    }

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.StaticRoutes", schema = List(711)))]
                    pub struct StaticRoutes<'a, Mode> (::data_views::Field<static_routes::Item<'a, Mode>>);

                    pub mod static_routes {

                        #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.StaticRoutesItem", schema = Dict(1663)))]
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

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6StaticRoutes", schema = List(712)))]
                    pub struct Ipv6StaticRoutes<'a, Mode> (::data_views::Field<ipv6_static_routes::Item<'a, Mode>>);

                    pub mod ipv6_static_routes {

                        #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6StaticRoutesItem", schema = Dict(1663)))]
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

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.TrunkGroups", schema = List(713)))]
                    pub struct TrunkGroups<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.EvpnL2Multicast", schema = Dict(1664)))]
                    pub struct EvpnL2Multicast<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub enabled: ::data_views::Field<bool>,
                        #[data_view(schema = Bool(583))]
                        pub always_redistribute_igmp: ::data_views::Field<bool>,
                    }

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.VxlanFloodMulticast", schema = Dict(1665)))]
                    pub struct VxlanFloodMulticast<'a, Mode> {
                        #[data_view(schema = Bool(585))]
                        pub enabled: ::data_views::Field<bool>,
                        #[data_view(schema = Str(1528))]
                        pub underlay_multicast_group: ::data_views::Field<&'a str>,
                    }

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.EvpnL3Multicast", schema = Dict(1666)))]
                    pub struct EvpnL3Multicast<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub enabled: ::data_views::Field<bool>,
                    }

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.IgmpSnooping", schema = Dict(1608)))]
                    pub struct IgmpSnooping<'a, Mode> {
                        #[data_view(schema = Bool(546))]
                        pub enabled: ::data_views::Field<bool>,
                        #[data_view(schema = Dict(1607))]
                        pub querier: ::data_views::Field<igmp_snooping::Querier<'a, Mode>>,
                        #[data_view(schema = Bool(548))]
                        pub fast_leave: ::data_views::Field<bool>,
                    }

                    pub mod igmp_snooping {

                        #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.IgmpSnooping.Querier", schema = Dict(1607)))]
                        pub struct Querier<'a, Mode> {
                            #[data_view(schema = Bool(547))]
                            pub enabled: ::data_views::Field<bool>,
                            #[data_view(schema = Str(1437))]
                            pub source_address: ::data_views::Field<&'a str>,
                            #[data_view(schema = Int(595))]
                            pub version: ::data_views::Field<i64>,
                        }
                    }

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.IgmpSnoopingQuerier", schema = Dict(1667)))]
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

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ospf", schema = Dict(1669)))]
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

                        #[::data_views::data_view(indexed_list, primary_key(id), python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ospf.MessageDigestKeys", schema = List(714), item_name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ospf.MessageDigestKeysItem"))]
                        pub struct MessageDigestKeys<'a, Mode> (::data_views::Field<message_digest_keys::Item<'a, Mode>>);

                        pub mod message_digest_keys {

                            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ospf.MessageDigestKeysItemBase", schema = Dict(1668)))]
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

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Bgp", schema = Dict(1671)))]
                    pub struct Bgp<'a, Mode> {
                        #[data_view(schema = Dict(1670), relaxed)]
                        pub structured_config: ::data_views::Field<super::super::super::super::super::super::super::eos_config::router_bgp::vlans::Item<'a, ::data_views::RelaxedValidated>>,
                        #[data_view(schema = Str(1535))]
                        pub raw_eos_cli: ::data_views::Field<&'a str>,
                    }

                    pub mod bgp {
                    }
                }
            }

            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3Interfaces", schema = List(725)))]
            pub struct L3Interfaces<'a, Mode> (::data_views::Field<l3_interfaces::Item<'a, Mode>>);

            pub mod l3_interfaces {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem", schema = Dict(1678)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = List(717))]
                    pub interfaces: ::data_views::Field<item::Interfaces<'a, Mode>>,
                    #[data_view(schema = List(718))]
                    pub encapsulation_dot1q_vlan: ::data_views::Field<item::EncapsulationDot1qVlan<'a, Mode>>,
                    #[data_view(schema = List(222))]
                    pub ip_addresses: ::data_views::Field<item::IpAddresses<'a, Mode>>,
                    #[data_view(schema = List(719))]
                    pub ipv6_addresses: ::data_views::Field<item::Ipv6Addresses<'a, Mode>>,
                    #[data_view(schema = List(720))]
                    pub static_routes: ::data_views::Field<item::StaticRoutes<'a, Mode>>,
                    #[data_view(schema = List(721))]
                    pub ipv6_static_routes: ::data_views::Field<item::Ipv6StaticRoutes<'a, Mode>>,
                    #[data_view(schema = List(722))]
                    pub nodes: ::data_views::Field<item::Nodes<'a, Mode>>,
                    #[data_view(schema = Bool(124))]
                    pub arp_gratuitous_accept: ::data_views::Field<bool>,
                    #[data_view(schema = Str(3))]
                    pub description: ::data_views::Field<&'a str>,
                    #[data_view(schema = List(723))]
                    pub descriptions: ::data_views::Field<item::Descriptions<'a, Mode>>,
                    #[data_view(schema = Bool(37))]
                    pub enabled: ::data_views::Field<bool>,
                    #[data_view(schema = Int(77))]
                    pub mtu: ::data_views::Field<i64>,
                    #[data_view(schema = Str(49))]
                    pub ipv4_acl_in: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(49))]
                    pub ipv4_acl_out: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(1540))]
                    pub ipv6_acl_in: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(1541))]
                    pub ipv6_acl_out: ::data_views::Field<&'a str>,
                    #[data_view(schema = Dict(1676))]
                    pub ospf: ::data_views::Field<item::Ospf<'a, Mode>>,
                    #[data_view(schema = Dict(1677))]
                    pub pim: ::data_views::Field<item::Pim<'a, Mode>>,
                    #[data_view(schema = Dict(1504))]
                    pub flow_tracking: ::data_views::Field<item::FlowTracking<'a, Mode>>,
                    #[data_view(schema = Bool(589))]
                    pub sflow: ::data_views::Field<bool>,
                    #[data_view(schema = List(572))]
                    pub monitor_sessions: ::data_views::Field<item::MonitorSessions<'a, Mode>>,
                    #[data_view(schema = List(574))]
                    pub campus_link_type: ::data_views::Field<item::CampusLinkType<'a, Mode>>,
                    #[data_view(schema = Dict(1432), relaxed)]
                    pub structured_config: ::data_views::Field<super::super::super::super::super::super::eos_config::ethernet_interfaces::Item<'a, ::data_views::RelaxedValidated>>,
                    #[data_view(schema = Str(1544))]
                    pub raw_eos_cli: ::data_views::Field<&'a str>,
                }

                pub mod item {

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.Interfaces", schema = List(717)))]
                    pub struct Interfaces<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.EncapsulationDot1qVlan", schema = List(718)))]
                    pub struct EncapsulationDot1qVlan<'a, Mode> (::data_views::Field<i64>);

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.IpAddresses", schema = List(222)))]
                    pub struct IpAddresses<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.Ipv6Addresses", schema = List(719)))]
                    pub struct Ipv6Addresses<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.StaticRoutes", schema = List(720)))]
                    pub struct StaticRoutes<'a, Mode> (::data_views::Field<static_routes::Item<'a, Mode>>);

                    pub mod static_routes {

                        #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.StaticRoutesItem", schema = Dict(1663)))]
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

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.Ipv6StaticRoutes", schema = List(721)))]
                    pub struct Ipv6StaticRoutes<'a, Mode> (::data_views::Field<ipv6_static_routes::Item<'a, Mode>>);

                    pub mod ipv6_static_routes {

                        #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.Ipv6StaticRoutesItem", schema = Dict(1663)))]
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

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.Nodes", schema = List(722)))]
                    pub struct Nodes<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.Descriptions", schema = List(723)))]
                    pub struct Descriptions<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.Ospf", schema = Dict(1676)))]
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
                        #[data_view(schema = Str(1542))]
                        pub simple_auth_key: ::data_views::Field<&'a str>,
                        #[data_view(schema = Str(1532))]
                        pub cleartext_simple_auth_key: ::data_views::Field<&'a str>,
                        #[data_view(schema = List(724))]
                        pub message_digest_keys: ::data_views::Field<ospf::MessageDigestKeys<'a, Mode>>,
                    }

                    pub mod ospf {

                        #[::data_views::data_view(indexed_list, primary_key(id), python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.Ospf.MessageDigestKeys", schema = List(724), item_name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.Ospf.MessageDigestKeysItem"))]
                        pub struct MessageDigestKeys<'a, Mode> (::data_views::Field<message_digest_keys::Item<'a, Mode>>);

                        pub mod message_digest_keys {

                            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.Ospf.MessageDigestKeysItemBase", schema = Dict(1675)))]
                            pub struct Item<'a, Mode> {
                                #[data_view(schema = Int(77))]
                                pub id: ::data_views::Field<i64>,
                                #[data_view(schema = Str(1502))]
                                pub hash_algorithm: ::data_views::Field<&'a str>,
                                #[data_view(schema = Str(1543))]
                                pub key: ::data_views::Field<&'a str>,
                                #[data_view(schema = Str(1534))]
                                pub cleartext_key: ::data_views::Field<&'a str>,
                            }
                        }
                    }

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.Pim", schema = Dict(1677)))]
                    pub struct Pim<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub enabled: ::data_views::Field<bool>,
                    }

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.FlowTracking", schema = Dict(1504)))]
                    pub struct FlowTracking<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub enabled: ::data_views::Field<bool>,
                        #[data_view(schema = Str(1155))]
                        pub name: ::data_views::Field<&'a str>,
                    }

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.MonitorSessions", schema = List(572)))]
                    pub struct MonitorSessions<'a, Mode> (::data_views::Field<monitor_sessions::Item<'a, Mode>>);

                    pub mod monitor_sessions {

                        #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.MonitorSessionsItem", schema = Dict(1430)))]
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

                            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.MonitorSessionsItem.SourceSettings", schema = Dict(1427)))]
                            pub struct SourceSettings<'a, Mode> {
                                #[data_view(schema = Str(593))]
                                pub direction: ::data_views::Field<&'a str>,
                                #[data_view(schema = Dict(1426))]
                                pub access_group: ::data_views::Field<source_settings::AccessGroup<'a, Mode>>,
                            }

                            pub mod source_settings {

                                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.MonitorSessionsItem.SourceSettings.AccessGroup", schema = Dict(1426)))]
                                pub struct AccessGroup<'a, Mode> {
                                    #[data_view(schema = Str(594), rename = "type")]
                                    pub field_type: ::data_views::Field<&'a str>,
                                    #[data_view(schema = Str(1161))]
                                    pub name: ::data_views::Field<&'a str>,
                                    #[data_view(schema = Int(77))]
                                    pub priority: ::data_views::Field<i64>,
                                }
                            }

                            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.MonitorSessionsItem.SessionSettings", schema = Dict(1429)))]
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

                                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.MonitorSessionsItem.SessionSettings.AccessGroup", schema = Dict(1428)))]
                                pub struct AccessGroup<'a, Mode> {
                                    #[data_view(schema = Str(594), rename = "type")]
                                    pub field_type: ::data_views::Field<&'a str>,
                                    #[data_view(schema = Str(1161))]
                                    pub name: ::data_views::Field<&'a str>,
                                }

                                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.MonitorSessionsItem.SessionSettings.Truncate", schema = Dict(666)))]
                                pub struct Truncate<'a, Mode> {
                                    #[data_view(schema = Bool(7))]
                                    pub enabled: ::data_views::Field<bool>,
                                    #[data_view(schema = Int(274))]
                                    pub size: ::data_views::Field<i64>,
                                }
                            }
                        }
                    }

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.CampusLinkType", schema = List(574)))]
                    pub struct CampusLinkType<'a, Mode> (::data_views::Field<&'a str>);
                }
            }

            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3PortChannels", schema = List(731)))]
            pub struct L3PortChannels<'a, Mode> (::data_views::Field<l3_port_channels::Item<'a, Mode>>);

            pub mod l3_port_channels {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem", schema = Dict(1681)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(1352))]
                    pub name: ::data_views::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Str(1545))]
                    pub node: ::data_views::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Bool(124))]
                    pub arp_gratuitous_accept: ::data_views::Field<bool>,
                    #[data_view(schema = Str(1353))]
                    pub description: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(1354))]
                    pub mode: ::data_views::Field<&'a str>,
                    #[data_view(schema = List(726))]
                    pub member_interfaces: ::data_views::Field<item::MemberInterfaces<'a, Mode>>,
                    #[data_view(schema = Str(1546))]
                    pub ip_address: ::data_views::Field<&'a str>,
                    #[data_view(schema = List(727))]
                    pub ip_address_secondaries: ::data_views::Field<item::IpAddressSecondaries<'a, Mode>>,
                    #[data_view(schema = List(728))]
                    pub ipv6_addresses: ::data_views::Field<item::Ipv6Addresses<'a, Mode>>,
                    #[data_view(schema = Int(565))]
                    pub encapsulation_dot1q_vlan: ::data_views::Field<i64>,
                    #[data_view(schema = Bool(480))]
                    pub enabled: ::data_views::Field<bool>,
                    #[data_view(schema = Str(1333))]
                    pub peer: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(1359))]
                    pub peer_port_channel: ::data_views::Field<&'a str>,
                    #[data_view(schema = Int(613))]
                    pub mtu: ::data_views::Field<i64>,
                    #[data_view(schema = Str(1547))]
                    pub ipv4_acl_in: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(1548))]
                    pub ipv4_acl_out: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(1549))]
                    pub ipv6_acl_in: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(1550))]
                    pub ipv6_acl_out: ::data_views::Field<&'a str>,
                    #[data_view(schema = List(729))]
                    pub static_routes: ::data_views::Field<item::StaticRoutes<'a, Mode>>,
                    #[data_view(schema = List(730))]
                    pub ipv6_static_routes: ::data_views::Field<item::Ipv6StaticRoutes<'a, Mode>>,
                    #[data_view(schema = Dict(1680))]
                    pub ospf: ::data_views::Field<item::Ospf<'a, Mode>>,
                    #[data_view(schema = Dict(1511))]
                    pub flow_tracking: ::data_views::Field<item::FlowTracking<'a, Mode>>,
                    #[data_view(schema = Dict(1415), relaxed)]
                    pub structured_config: ::data_views::Field<super::super::super::super::super::super::eos_config::port_channel_interfaces::Item<'a, ::data_views::RelaxedValidated>>,
                    #[data_view(schema = Str(1552))]
                    pub raw_eos_cli: ::data_views::Field<&'a str>,
                }

                pub mod item {

                    #[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.MemberInterfaces", schema = List(726), item_name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.MemberInterfacesItem"))]
                    pub struct MemberInterfaces<'a, Mode> (::data_views::Field<member_interfaces::Item<'a, Mode>>);

                    pub mod member_interfaces {

                        #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.MemberInterfacesItemBase", schema = Dict(1679)))]
                        pub struct Item<'a, Mode> {
                            #[data_view(schema = Str(1355))]
                            pub name: ::data_views::RequiredValue<&'a str, Mode>,
                            #[data_view(schema = Str(1356))]
                            pub description: ::data_views::Field<&'a str>,
                            #[data_view(schema = Str(1357))]
                            pub peer: ::data_views::Field<&'a str>,
                            #[data_view(schema = Str(1334))]
                            pub peer_interface: ::data_views::Field<&'a str>,
                            #[data_view(schema = Str(178))]
                            pub speed: ::data_views::Field<&'a str>,
                            #[data_view(schema = Dict(1507), relaxed)]
                            pub structured_config: ::data_views::Field<super::super::super::super::super::super::super::super::eos_config::ethernet_interfaces::Item<'a, ::data_views::RelaxedValidated>>,
                        }

                        pub mod item {
                        }
                    }

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.IpAddressSecondaries", schema = List(727)))]
                    pub struct IpAddressSecondaries<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.Ipv6Addresses", schema = List(728)))]
                    pub struct Ipv6Addresses<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.StaticRoutes", schema = List(729)))]
                    pub struct StaticRoutes<'a, Mode> (::data_views::Field<static_routes::Item<'a, Mode>>);

                    pub mod static_routes {

                        #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.StaticRoutesItem", schema = Dict(1663)))]
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

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.Ipv6StaticRoutes", schema = List(730)))]
                    pub struct Ipv6StaticRoutes<'a, Mode> (::data_views::Field<ipv6_static_routes::Item<'a, Mode>>);

                    pub mod ipv6_static_routes {

                        #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.Ipv6StaticRoutesItem", schema = Dict(1663)))]
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

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.Ospf", schema = Dict(1680)))]
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
                        #[data_view(schema = Str(1551))]
                        pub simple_auth_key: ::data_views::Field<&'a str>,
                        #[data_view(schema = Str(1532))]
                        pub cleartext_simple_auth_key: ::data_views::Field<&'a str>,
                        #[data_view(schema = List(724))]
                        pub message_digest_keys: ::data_views::Field<ospf::MessageDigestKeys<'a, Mode>>,
                    }

                    pub mod ospf {

                        #[::data_views::data_view(indexed_list, primary_key(id), python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.Ospf.MessageDigestKeys", schema = List(724), item_name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.Ospf.MessageDigestKeysItem"))]
                        pub struct MessageDigestKeys<'a, Mode> (::data_views::Field<message_digest_keys::Item<'a, Mode>>);

                        pub mod message_digest_keys {

                            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.Ospf.MessageDigestKeysItemBase", schema = Dict(1675)))]
                            pub struct Item<'a, Mode> {
                                #[data_view(schema = Int(77))]
                                pub id: ::data_views::Field<i64>,
                                #[data_view(schema = Str(1502))]
                                pub hash_algorithm: ::data_views::Field<&'a str>,
                                #[data_view(schema = Str(1543))]
                                pub key: ::data_views::Field<&'a str>,
                                #[data_view(schema = Str(1534))]
                                pub cleartext_key: ::data_views::Field<&'a str>,
                            }
                        }
                    }

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.FlowTracking", schema = Dict(1511)))]
                    pub struct FlowTracking<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub enabled: ::data_views::Field<bool>,
                        #[data_view(schema = Str(1155))]
                        pub name: ::data_views::Field<&'a str>,
                    }
                }
            }

            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.Loopbacks", schema = List(732)))]
            pub struct Loopbacks<'a, Mode> (::data_views::Field<loopbacks::Item<'a, Mode>>);

            pub mod loopbacks {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.LoopbacksItem", schema = Dict(1683)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(2))]
                    pub node: ::data_views::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Int(614))]
                    pub loopback: ::data_views::RequiredValue<i64, Mode>,
                    #[data_view(schema = Str(2))]
                    pub ip_address: ::data_views::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Str(3))]
                    pub description: ::data_views::Field<&'a str>,
                    #[data_view(schema = Bool(37))]
                    pub enabled: ::data_views::Field<bool>,
                    #[data_view(schema = Dict(1682))]
                    pub ospf: ::data_views::Field<item::Ospf<'a, Mode>>,
                    #[data_view(schema = Bool(590))]
                    pub hardware_forwarding: ::data_views::Field<bool>,
                    #[data_view(schema = Str(1553))]
                    pub raw_eos_cli: ::data_views::Field<&'a str>,
                }

                pub mod item {

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.LoopbacksItem.Ospf", schema = Dict(1682)))]
                    pub struct Ospf<'a, Mode> {
                        #[data_view(schema = Bool(340))]
                        pub enabled: ::data_views::Field<bool>,
                        #[data_view(schema = Str(1529))]
                        pub area: ::data_views::Field<&'a str>,
                    }
                }
            }

            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.StaticRoutes", schema = List(733)))]
            pub struct StaticRoutes<'a, Mode> (::data_views::Field<static_routes::Item<'a, Mode>>);

            pub mod static_routes {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.StaticRoutesItem", schema = Dict(1684)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = List(11))]
                    pub nodes: ::data_views::Field<item::Nodes<'a, Mode>>,
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

                pub mod item {

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.StaticRoutesItem.Nodes", schema = List(11)))]
                    pub struct Nodes<'a, Mode> (::data_views::Field<&'a str>);
                }
            }

            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.Ipv6StaticRoutes", schema = List(734)))]
            pub struct Ipv6StaticRoutes<'a, Mode> (::data_views::Field<ipv6_static_routes::Item<'a, Mode>>);

            pub mod ipv6_static_routes {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.Ipv6StaticRoutesItem", schema = Dict(1685)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = List(11))]
                    pub nodes: ::data_views::Field<item::Nodes<'a, Mode>>,
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

                pub mod item {

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.Ipv6StaticRoutesItem.Nodes", schema = List(11)))]
                    pub struct Nodes<'a, Mode> (::data_views::Field<&'a str>);
                }
            }

            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.StaticArpEntries", schema = List(736)))]
            pub struct StaticArpEntries<'a, Mode> (::data_views::Field<static_arp_entries::Item<'a, Mode>>);

            pub mod static_arp_entries {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.StaticArpEntriesItem", schema = Dict(1686)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(58))]
                    pub ipv4_address: ::data_views::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Str(60))]
                    pub mac_address: ::data_views::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = List(735))]
                    pub nodes: ::data_views::Field<item::Nodes<'a, Mode>>,
                }

                pub mod item {

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.StaticArpEntriesItem.Nodes", schema = List(735)))]
                    pub struct Nodes<'a, Mode> (::data_views::Field<&'a str>);
                }
            }

            #[::data_views::data_view(list, primary_key(ip_address), python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeers", schema = List(738), item_name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeersItem"))]
            pub struct BgpPeers<'a, Mode> (::data_views::Field<bgp_peers::Item<'a, Mode>>);

            pub mod bgp_peers {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeersItemBase", schema = Dict(1688)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(1556))]
                    pub ip_address: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(767))]
                    pub peer_group: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(758))]
                    pub remote_as: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(3))]
                    pub description: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(1476))]
                    pub password: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(1109))]
                    pub cleartext_password: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(1557))]
                    pub send_community: ::data_views::Field<&'a str>,
                    #[data_view(schema = Bool(7))]
                    pub next_hop_self: ::data_views::Field<bool>,
                    #[data_view(schema = Str(1558))]
                    pub timers: ::data_views::Field<&'a str>,
                    #[data_view(schema = Int(382))]
                    pub maximum_routes: ::data_views::Field<i64>,
                    #[data_view(schema = Bool(7))]
                    pub maximum_routes_warning_only: ::data_views::Field<bool>,
                    #[data_view(schema = Dict(1687))]
                    pub default_originate: ::data_views::Field<item::DefaultOriginate<'a, Mode>>,
                    #[data_view(schema = Str(3))]
                    pub update_source: ::data_views::Field<&'a str>,
                    #[data_view(schema = Int(381))]
                    pub ebgp_multihop: ::data_views::Field<i64>,
                    #[data_view(schema = List(737))]
                    pub nodes: ::data_views::Field<item::Nodes<'a, Mode>>,
                    #[data_view(schema = Str(1559))]
                    pub set_ipv4_next_hop: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(1560))]
                    pub set_ipv6_next_hop: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(169))]
                    pub route_map_out: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(169))]
                    pub route_map_in: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(1561))]
                    pub prefix_list_in: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(1562))]
                    pub prefix_list_out: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(1563))]
                    pub local_as: ::data_views::Field<&'a str>,
                    #[data_view(schema = Int(110))]
                    pub weight: ::data_views::Field<i64>,
                    #[data_view(schema = Bool(7))]
                    pub bfd: ::data_views::Field<bool>,
                    #[data_view(schema = Dict(916))]
                    pub bfd_timers: ::data_views::Field<super::super::super::super::super::super::eos_config::router_bgp::vrfs::item::neighbors::item::BfdTimers<'a, Mode>>,
                    #[data_view(schema = Bool(593))]
                    pub route_reflector_client: ::data_views::Field<bool>,
                    #[data_view(schema = Bool(7))]
                    pub shutdown: ::data_views::Field<bool>,
                }

                pub mod item {

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeersItemBase.DefaultOriginate", schema = Dict(1687)))]
                    pub struct DefaultOriginate<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub always: ::data_views::Field<bool>,
                    }

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeersItemBase.Nodes", schema = List(737)))]
                    pub struct Nodes<'a, Mode> (::data_views::Field<&'a str>);
                }
            }

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.Bgp", schema = Dict(1691)))]
            pub struct Bgp<'a, Mode> {
                #[data_view(schema = Bool(594))]
                pub enabled: ::data_views::Field<bool>,
                #[data_view(schema = Str(1564))]
                pub router_id: ::data_views::Field<&'a str>,
                #[data_view(schema = Dict(1689))]
                pub graceful_restart: ::data_views::Field<bgp::GracefulRestart<'a, Mode>>,
                #[data_view(schema = Str(1565))]
                pub raw_eos_cli: ::data_views::Field<&'a str>,
                #[data_view(schema = Dict(1690), relaxed)]
                pub structured_config: ::data_views::Field<super::super::super::super::super::eos_config::router_bgp::vrfs::Item<'a, ::data_views::RelaxedValidated>>,
            }

            pub mod bgp {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.Bgp.GracefulRestart", schema = Dict(1689)))]
                pub struct GracefulRestart<'a, Mode> {
                    #[data_view(schema = Bool(595))]
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    #[data_view(schema = Int(499))]
                    pub restart_time: ::data_views::Field<i64>,
                }
            }

            #[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroups", schema = List(740), item_name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItem"))]
            pub struct BgpPeerGroups<'a, Mode> (::data_views::Field<bgp_peer_groups::Item<'a, Mode>>);

            pub mod bgp_peer_groups {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase", schema = Dict(1692)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(1475))]
                    pub name: ::data_views::Field<&'a str>,
                    #[data_view(schema = List(687))]
                    pub nodes: ::data_views::Field<item::Nodes<'a, Mode>>,
                    #[data_view(schema = Str(1476))]
                    pub password: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(1109))]
                    pub cleartext_password: ::data_views::Field<&'a str>,
                    #[data_view(schema = Dict(1634))]
                    pub address_family_ipv4: ::data_views::Field<item::AddressFamilyIpv4<'a, Mode>>,
                    #[data_view(schema = Dict(1636))]
                    pub address_family_ipv6: ::data_views::Field<item::AddressFamilyIpv6<'a, Mode>>,
                    #[data_view(schema = List(739))]
                    pub listen_ranges: ::data_views::Field<item::ListenRanges<'a, Mode>>,
                    #[data_view(schema = Dict(912))]
                    pub metadata: ::data_views::Field<item::Metadata<'a, Mode>>,
                    #[data_view(schema = Str(758))]
                    pub remote_as: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(758))]
                    pub local_as: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(3))]
                    pub description: ::data_views::Field<&'a str>,
                    #[data_view(schema = Bool(7))]
                    pub shutdown: ::data_views::Field<bool>,
                    #[data_view(schema = Dict(913))]
                    pub as_path: ::data_views::Field<item::AsPath<'a, Mode>>,
                    #[data_view(schema = Dict(914))]
                    pub remove_private_as: ::data_views::Field<item::RemovePrivateAs<'a, Mode>>,
                    #[data_view(schema = Dict(915))]
                    pub remove_private_as_ingress: ::data_views::Field<item::RemovePrivateAsIngress<'a, Mode>>,
                    #[data_view(schema = Bool(7))]
                    pub next_hop_unchanged: ::data_views::Field<bool>,
                    #[data_view(schema = Str(771))]
                    pub update_source: ::data_views::Field<&'a str>,
                    #[data_view(schema = Bool(7))]
                    pub route_reflector_client: ::data_views::Field<bool>,
                    #[data_view(schema = Bool(323))]
                    pub bfd: ::data_views::Field<bool>,
                    #[data_view(schema = Dict(916))]
                    pub bfd_timers: ::data_views::Field<item::BfdTimers<'a, Mode>>,
                    #[data_view(schema = Int(381))]
                    pub ebgp_multihop: ::data_views::Field<i64>,
                    #[data_view(schema = Bool(7))]
                    pub next_hop_peer: ::data_views::Field<bool>,
                    #[data_view(schema = Bool(7))]
                    pub next_hop_self: ::data_views::Field<bool>,
                    #[data_view(schema = Str(772))]
                    pub password_type: ::data_views::Field<&'a str>,
                    #[data_view(schema = Bool(7))]
                    pub passive: ::data_views::Field<bool>,
                    #[data_view(schema = Dict(917))]
                    pub default_originate: ::data_views::Field<item::DefaultOriginate<'a, Mode>>,
                    #[data_view(schema = Bool(324))]
                    pub enforce_first_as: ::data_views::Field<bool>,
                    #[data_view(schema = Str(773))]
                    pub send_community: ::data_views::Field<&'a str>,
                    #[data_view(schema = Int(382))]
                    pub maximum_routes: ::data_views::Field<i64>,
                    #[data_view(schema = Str(774))]
                    pub maximum_routes_warning_limit: ::data_views::Field<&'a str>,
                    #[data_view(schema = Bool(7))]
                    pub maximum_routes_warning_only: ::data_views::Field<bool>,
                    #[data_view(schema = Dict(919))]
                    pub maximum_accepted_routes: ::data_views::Field<item::MaximumAcceptedRoutes<'a, Mode>>,
                    #[data_view(schema = Dict(922))]
                    pub missing_policy: ::data_views::Field<item::MissingPolicy<'a, Mode>>,
                    #[data_view(schema = Dict(923))]
                    pub link_bandwidth: ::data_views::Field<item::LinkBandwidth<'a, Mode>>,
                    #[data_view(schema = Dict(924))]
                    pub allowas_in: ::data_views::Field<item::AllowasIn<'a, Mode>>,
                    #[data_view(schema = Int(110))]
                    pub weight: ::data_views::Field<i64>,
                    #[data_view(schema = Str(777))]
                    pub timers: ::data_views::Field<&'a str>,
                    #[data_view(schema = Dict(925))]
                    pub rib_in_pre_policy_retain: ::data_views::Field<item::RibInPrePolicyRetain<'a, Mode>>,
                    #[data_view(schema = Str(778))]
                    pub route_map_in: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(779))]
                    pub route_map_out: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(780))]
                    pub peer_tag_in: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(781))]
                    pub peer_tag_out_discard: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(3))]
                    pub session_tracker: ::data_views::Field<&'a str>,
                    #[data_view(schema = Dict(926))]
                    pub shared_secret: ::data_views::Field<item::SharedSecret<'a, Mode>>,
                    #[data_view(schema = Int(387))]
                    pub ttl_maximum_hops: ::data_views::Field<i64>,
                    #[data_view(schema = Int(388))]
                    pub maximum_advertised_routes: ::data_views::Field<i64>,
                    #[data_view(schema = Str(784))]
                    pub maximum_advertised_routes_warning_limit: ::data_views::Field<&'a str>,
                }

                pub mod item {

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.Nodes", schema = List(687)))]
                    pub struct Nodes<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.AddressFamilyIpv4", schema = Dict(1634)))]
                    pub struct AddressFamilyIpv4<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub activate: ::data_views::Field<bool>,
                        #[data_view(schema = Str(778))]
                        pub route_map_in: ::data_views::Field<&'a str>,
                        #[data_view(schema = Str(779))]
                        pub route_map_out: ::data_views::Field<&'a str>,
                        #[data_view(schema = Str(803))]
                        pub rcf_in: ::data_views::Field<&'a str>,
                        #[data_view(schema = Str(804))]
                        pub rcf_out: ::data_views::Field<&'a str>,
                        #[data_view(schema = Dict(977))]
                        pub default_originate: ::data_views::Field<super::super::super::super::super::super::super::eos_config::router_bgp::address_family_ipv4::peer_groups::item::DefaultOriginate<'a, Mode>>,
                        #[data_view(schema = Dict(980))]
                        pub next_hop: ::data_views::Field<super::super::super::super::super::super::super::eos_config::router_bgp::address_family_ipv4::peer_groups::item::NextHop<'a, Mode>>,
                        #[data_view(schema = Str(812))]
                        pub prefix_list_in: ::data_views::Field<&'a str>,
                        #[data_view(schema = Str(813))]
                        pub prefix_list_out: ::data_views::Field<&'a str>,
                    }

                    pub mod address_family_ipv4 {
                    }

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.AddressFamilyIpv6", schema = Dict(1636)))]
                    pub struct AddressFamilyIpv6<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub activate: ::data_views::Field<bool>,
                        #[data_view(schema = Str(778))]
                        pub route_map_in: ::data_views::Field<&'a str>,
                        #[data_view(schema = Str(779))]
                        pub route_map_out: ::data_views::Field<&'a str>,
                        #[data_view(schema = Str(803))]
                        pub rcf_in: ::data_views::Field<&'a str>,
                        #[data_view(schema = Str(804))]
                        pub rcf_out: ::data_views::Field<&'a str>,
                        #[data_view(schema = Dict(1635))]
                        pub default_originate: ::data_views::Field<address_family_ipv6::DefaultOriginate<'a, Mode>>,
                        #[data_view(schema = Str(812))]
                        pub prefix_list_in: ::data_views::Field<&'a str>,
                        #[data_view(schema = Str(813))]
                        pub prefix_list_out: ::data_views::Field<&'a str>,
                    }

                    pub mod address_family_ipv6 {

                        #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.AddressFamilyIpv6.DefaultOriginate", schema = Dict(1635)))]
                        pub struct DefaultOriginate<'a, Mode> {
                            #[data_view(schema = Bool(1))]
                            pub enabled: ::data_views::RequiredValue<bool, Mode>,
                            #[data_view(schema = Bool(336))]
                            pub always: ::data_views::Field<bool>,
                            #[data_view(schema = Str(169))]
                            pub route_map: ::data_views::Field<&'a str>,
                        }
                    }

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.ListenRanges", schema = List(739)))]
                    pub struct ListenRanges<'a, Mode> (::data_views::Field<listen_ranges::Item<'a, Mode>>);

                    pub mod listen_ranges {

                        #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.ListenRangesItem", schema = Dict(1637)))]
                        pub struct Item<'a, Mode> {
                            #[data_view(schema = Str(1477))]
                            pub prefix: ::data_views::RequiredValue<&'a str, Mode>,
                            #[data_view(schema = Str(1478))]
                            pub remote_as: ::data_views::Field<&'a str>,
                            #[data_view(schema = Bool(320))]
                            pub peer_id_include_router_id: ::data_views::Field<bool>,
                            #[data_view(schema = Str(1479))]
                            pub peer_filter: ::data_views::Field<&'a str>,
                        }
                    }

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.Metadata", schema = Dict(912)))]
                    pub struct Metadata<'a, Mode> {
                        #[data_view(schema = Str(278), rename = "type")]
                        pub field_type: ::data_views::Field<&'a str>,
                    }

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.AsPath", schema = Dict(913)))]
                    pub struct AsPath<'a, Mode> {
                        #[data_view(schema = Bool(321))]
                        pub remote_as_replace_out: ::data_views::Field<bool>,
                        #[data_view(schema = Bool(322))]
                        pub prepend_own_disabled: ::data_views::Field<bool>,
                    }

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.RemovePrivateAs", schema = Dict(914)))]
                    pub struct RemovePrivateAs<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub enabled: ::data_views::Field<bool>,
                        #[data_view(schema = Bool(7))]
                        pub all: ::data_views::Field<bool>,
                        #[data_view(schema = Bool(7))]
                        pub replace_as: ::data_views::Field<bool>,
                    }

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.RemovePrivateAsIngress", schema = Dict(915)))]
                    pub struct RemovePrivateAsIngress<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub enabled: ::data_views::Field<bool>,
                        #[data_view(schema = Bool(7))]
                        pub replace_as: ::data_views::Field<bool>,
                    }

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.BfdTimers", schema = Dict(916)))]
                    pub struct BfdTimers<'a, Mode> {
                        #[data_view(schema = Int(378))]
                        pub interval: ::data_views::RequiredValue<i64, Mode>,
                        #[data_view(schema = Int(379))]
                        pub min_rx: ::data_views::RequiredValue<i64, Mode>,
                        #[data_view(schema = Int(380))]
                        pub multiplier: ::data_views::RequiredValue<i64, Mode>,
                    }

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.DefaultOriginate", schema = Dict(917)))]
                    pub struct DefaultOriginate<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub enabled: ::data_views::Field<bool>,
                        #[data_view(schema = Bool(7))]
                        pub always: ::data_views::Field<bool>,
                        #[data_view(schema = Str(169))]
                        pub route_map: ::data_views::Field<&'a str>,
                    }

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.MaximumAcceptedRoutes", schema = Dict(919)))]
                    pub struct MaximumAcceptedRoutes<'a, Mode> {
                        #[data_view(schema = Int(383))]
                        pub limit: ::data_views::RequiredValue<i64, Mode>,
                        #[data_view(schema = Dict(918))]
                        pub warning_limit: ::data_views::Field<maximum_accepted_routes::WarningLimit<'a, Mode>>,
                    }

                    pub mod maximum_accepted_routes {

                        #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.MaximumAcceptedRoutes.WarningLimit", schema = Dict(918)))]
                        pub struct WarningLimit<'a, Mode> {
                            #[data_view(schema = Int(384))]
                            pub count: ::data_views::Field<i64>,
                            #[data_view(schema = Int(385))]
                            pub percent: ::data_views::Field<i64>,
                        }
                    }

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.MissingPolicy", schema = Dict(922)))]
                    pub struct MissingPolicy<'a, Mode> {
                        #[data_view(schema = Dict(920))]
                        pub direction_in: ::data_views::Field<missing_policy::DirectionIn<'a, Mode>>,
                        #[data_view(schema = Dict(921))]
                        pub direction_out: ::data_views::Field<missing_policy::DirectionOut<'a, Mode>>,
                    }

                    pub mod missing_policy {

                        #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.MissingPolicy.DirectionIn", schema = Dict(920)))]
                        pub struct DirectionIn<'a, Mode> {
                            #[data_view(schema = Str(775))]
                            pub action: ::data_views::RequiredValue<&'a str, Mode>,
                            #[data_view(schema = Bool(325))]
                            pub include_community_list: ::data_views::Field<bool>,
                            #[data_view(schema = Bool(326))]
                            pub include_prefix_list: ::data_views::Field<bool>,
                            #[data_view(schema = Bool(327))]
                            pub include_sub_route_map: ::data_views::Field<bool>,
                        }

                        #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.MissingPolicy.DirectionOut", schema = Dict(921)))]
                        pub struct DirectionOut<'a, Mode> {
                            #[data_view(schema = Str(775))]
                            pub action: ::data_views::RequiredValue<&'a str, Mode>,
                            #[data_view(schema = Bool(325))]
                            pub include_community_list: ::data_views::Field<bool>,
                            #[data_view(schema = Bool(326))]
                            pub include_prefix_list: ::data_views::Field<bool>,
                            #[data_view(schema = Bool(327))]
                            pub include_sub_route_map: ::data_views::Field<bool>,
                        }
                    }

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.LinkBandwidth", schema = Dict(923)))]
                    pub struct LinkBandwidth<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub enabled: ::data_views::Field<bool>,
                        #[data_view(schema = Str(776))]
                        pub default: ::data_views::Field<&'a str>,
                    }

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.AllowasIn", schema = Dict(924)))]
                    pub struct AllowasIn<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub enabled: ::data_views::Field<bool>,
                        #[data_view(schema = Int(386))]
                        pub times: ::data_views::Field<i64>,
                    }

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.RibInPrePolicyRetain", schema = Dict(925)))]
                    pub struct RibInPrePolicyRetain<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub enabled: ::data_views::Field<bool>,
                        #[data_view(schema = Bool(7))]
                        pub all: ::data_views::Field<bool>,
                    }

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.SharedSecret", schema = Dict(926)))]
                    pub struct SharedSecret<'a, Mode> {
                        #[data_view(schema = Str(782))]
                        pub profile: ::data_views::RequiredValue<&'a str, Mode>,
                        #[data_view(schema = Str(783))]
                        pub hash_algorithm: ::data_views::RequiredValue<&'a str, Mode>,
                    }
                }
            }

            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.AdditionalRouteTargets", schema = List(742)))]
            pub struct AdditionalRouteTargets<'a, Mode> (::data_views::Field<additional_route_targets::Item<'a, Mode>>);

            pub mod additional_route_targets {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.AdditionalRouteTargetsItem", schema = Dict(1693)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(1566), rename = "type")]
                    pub field_type: ::data_views::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Str(2))]
                    pub address_family: ::data_views::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Str(2))]
                    pub route_target: ::data_views::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = List(741))]
                    pub nodes: ::data_views::Field<item::Nodes<'a, Mode>>,
                }

                pub mod item {

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.AdditionalRouteTargetsItem.Nodes", schema = List(741)))]
                    pub struct Nodes<'a, Mode> (::data_views::Field<&'a str>);
                }
            }

            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.AggregateAddresses", schema = List(744)))]
            pub struct AggregateAddresses<'a, Mode> (::data_views::Field<aggregate_addresses::Item<'a, Mode>>);

            pub mod aggregate_addresses {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.AggregateAddressesItem", schema = Dict(1694)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = List(743))]
                    pub nodes: ::data_views::Field<item::Nodes<'a, Mode>>,
                    #[data_view(schema = Str(766))]
                    pub prefix: ::data_views::Field<&'a str>,
                    #[data_view(schema = Bool(7))]
                    pub advertise_only: ::data_views::Field<bool>,
                    #[data_view(schema = Bool(7))]
                    pub as_set: ::data_views::Field<bool>,
                    #[data_view(schema = Bool(7))]
                    pub summary_only: ::data_views::Field<bool>,
                    #[data_view(schema = Str(169))]
                    pub attribute_map: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(169))]
                    pub match_map: ::data_views::Field<&'a str>,
                    #[data_view(schema = Dict(932))]
                    pub attribute: ::data_views::Field<item::Attribute<'a, Mode>>,
                }

                pub mod item {

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.AggregateAddressesItem.Nodes", schema = List(743)))]
                    pub struct Nodes<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.VrfsItemBase.AggregateAddressesItem.Attribute", schema = Dict(932)))]
                    pub struct Attribute<'a, Mode> {
                        #[data_view(schema = Str(788))]
                        pub rcf: ::data_views::Field<&'a str>,
                    }
                }
            }
        }
    }

    #[::data_views::data_view(list, primary_key(id), python(name = "AVDDesign.NetworkServicesItemBase.L2vlans", schema = List(747), item_name = "AVDDesign.NetworkServicesItemBase.L2vlansItem"))]
    pub struct L2vlans<'a, Mode> (::data_views::Field<l2vlans::Item<'a, Mode>>);

    pub mod l2vlans {

        #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.L2vlansItemBase", schema = Dict(1697)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Int(476))]
            pub id: ::data_views::Field<i64>,
            #[data_view(schema = Str(1506))]
            pub name: ::data_views::RequiredValue<&'a str, Mode>,
            #[data_view(schema = Str(1569))]
            pub profile: ::data_views::Field<&'a str>,
            #[data_view(schema = List(746))]
            pub tags: ::data_views::Field<item::Tags<'a, Mode>>,
            #[data_view(schema = Dict(1340))]
            pub address_locking: ::data_views::Field<super::super::super::super::eos_config::vlans::item::address_locking::AddressFamily<'a, Mode>>,
            #[data_view(schema = Int(593))]
            pub vni_override: ::data_views::Field<i64>,
            #[data_view(schema = Str(1432))]
            pub rt_override: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1433))]
            pub rd_override: ::data_views::Field<&'a str>,
            #[data_view(schema = Bool(543))]
            pub vxlan: ::data_views::Field<bool>,
            #[data_view(schema = Int(594))]
            pub spanning_tree_priority: ::data_views::Field<i64>,
            #[data_view(schema = Str(1434))]
            pub evpn_vlan_bundle: ::data_views::Field<&'a str>,
            #[data_view(schema = List(667))]
            pub trunk_groups: ::data_views::Field<item::TrunkGroups<'a, Mode>>,
            #[data_view(schema = Bool(544))]
            pub evpn_l2_multi_domain: ::data_views::Field<bool>,
            #[data_view(schema = Dict(1605))]
            pub evpn_l2_multicast: ::data_views::Field<item::EvpnL2Multicast<'a, Mode>>,
            #[data_view(schema = Dict(1606))]
            pub vxlan_flood_multicast: ::data_views::Field<item::VxlanFloodMulticast<'a, Mode>>,
            #[data_view(schema = Dict(1608))]
            pub igmp_snooping: ::data_views::Field<item::IgmpSnooping<'a, Mode>>,
            #[data_view(schema = Bool(549))]
            pub igmp_snooping_enabled: ::data_views::Field<bool>,
            #[data_view(schema = Dict(1609))]
            pub igmp_snooping_querier: ::data_views::Field<item::IgmpSnoopingQuerier<'a, Mode>>,
            #[data_view(schema = Dict(1611))]
            pub bgp: ::data_views::Field<item::Bgp<'a, Mode>>,
            #[data_view(schema = Dict(1612))]
            pub private_vlan: ::data_views::Field<item::PrivateVlan<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.L2vlansItemBase.Tags", schema = List(746)))]
            pub struct Tags<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.L2vlansItemBase.TrunkGroups", schema = List(667)))]
            pub struct TrunkGroups<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.L2vlansItemBase.EvpnL2Multicast", schema = Dict(1605)))]
            pub struct EvpnL2Multicast<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.L2vlansItemBase.VxlanFloodMulticast", schema = Dict(1606)))]
            pub struct VxlanFloodMulticast<'a, Mode> {
                #[data_view(schema = Bool(545))]
                pub enabled: ::data_views::Field<bool>,
                #[data_view(schema = Str(1436))]
                pub underlay_multicast_group: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.L2vlansItemBase.IgmpSnooping", schema = Dict(1608)))]
            pub struct IgmpSnooping<'a, Mode> {
                #[data_view(schema = Bool(546))]
                pub enabled: ::data_views::Field<bool>,
                #[data_view(schema = Dict(1607))]
                pub querier: ::data_views::Field<igmp_snooping::Querier<'a, Mode>>,
                #[data_view(schema = Bool(548))]
                pub fast_leave: ::data_views::Field<bool>,
            }

            pub mod igmp_snooping {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.L2vlansItemBase.IgmpSnooping.Querier", schema = Dict(1607)))]
                pub struct Querier<'a, Mode> {
                    #[data_view(schema = Bool(547))]
                    pub enabled: ::data_views::Field<bool>,
                    #[data_view(schema = Str(1437))]
                    pub source_address: ::data_views::Field<&'a str>,
                    #[data_view(schema = Int(595))]
                    pub version: ::data_views::Field<i64>,
                }
            }

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.L2vlansItemBase.IgmpSnoopingQuerier", schema = Dict(1609)))]
            pub struct IgmpSnoopingQuerier<'a, Mode> {
                #[data_view(schema = Bool(550))]
                pub enabled: ::data_views::Field<bool>,
                #[data_view(schema = Str(1438))]
                pub source_address: ::data_views::Field<&'a str>,
                #[data_view(schema = Int(595))]
                pub version: ::data_views::Field<i64>,
                #[data_view(schema = Bool(548))]
                pub fast_leave: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.L2vlansItemBase.Bgp", schema = Dict(1611)))]
            pub struct Bgp<'a, Mode> {
                #[data_view(schema = Dict(1610), relaxed)]
                pub structured_config: ::data_views::Field<super::super::super::super::super::eos_config::router_bgp::vlans::Item<'a, ::data_views::RelaxedValidated>>,
                #[data_view(schema = Str(1439))]
                pub raw_eos_cli: ::data_views::Field<&'a str>,
            }

            pub mod bgp {
            }

            #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.L2vlansItemBase.PrivateVlan", schema = Dict(1612)))]
            pub struct PrivateVlan<'a, Mode> {
                #[data_view(schema = Str(1440), rename = "type")]
                pub field_type: ::data_views::RequiredValue<&'a str, Mode>,
                #[data_view(schema = Int(596))]
                pub primary_vlan: ::data_views::RequiredValue<i64, Mode>,
            }
        }
    }

    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.Vpws", schema = Dict(1698)))]
    pub struct Vpws<'a, Mode> {
        #[data_view(schema = Bool(597))]
        pub mpls_control_word: ::data_views::Field<bool>,
        #[data_view(schema = Int(615))]
        pub mtu: ::data_views::Field<i64>,
        #[data_view(schema = Bool(598))]
        pub label_flow: ::data_views::Field<bool>,
    }

    #[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.NetworkServicesItemBase.PointToPointServices", schema = List(752), item_name = "AVDDesign.NetworkServicesItemBase.PointToPointServicesItem"))]
    pub struct PointToPointServices<'a, Mode> (::data_views::Field<point_to_point_services::Item<'a, Mode>>);

    pub mod point_to_point_services {

        #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.PointToPointServicesItemBase", schema = Dict(1705)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(798))]
            pub name: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1571), rename = "type")]
            pub field_type: ::data_views::Field<&'a str>,
            #[data_view(schema = List(748))]
            pub subinterfaces: ::data_views::Field<item::Subinterfaces<'a, Mode>>,
            #[data_view(schema = List(751))]
            pub endpoints: ::data_views::Field<item::Endpoints<'a, Mode>>,
            #[data_view(schema = Bool(599))]
            pub lldp_disable: ::data_views::Field<bool>,
        }

        pub mod item {

            #[::data_views::data_view(indexed_list, primary_key(number), python(name = "AVDDesign.NetworkServicesItemBase.PointToPointServicesItemBase.Subinterfaces", schema = List(748), item_name = "AVDDesign.NetworkServicesItemBase.PointToPointServicesItemBase.SubinterfacesItem"))]
            pub struct Subinterfaces<'a, Mode> (::data_views::Field<subinterfaces::Item<'a, Mode>>);

            pub mod subinterfaces {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.PointToPointServicesItemBase.SubinterfacesItemBase", schema = Dict(1702)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Int(616))]
                    pub number: ::data_views::Field<i64>,
                    #[data_view(schema = Dict(1700))]
                    pub port_channel: ::data_views::Field<item::PortChannel<'a, Mode>>,
                    #[data_view(schema = Dict(1701), relaxed)]
                    pub structured_config: ::data_views::Field<super::super::super::super::super::super::eos_config::ethernet_interfaces::Item<'a, ::data_views::RelaxedValidated>>,
                    #[data_view(schema = Str(1573))]
                    pub raw_eos_cli: ::data_views::Field<&'a str>,
                }

                pub mod item {

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.PointToPointServicesItemBase.SubinterfacesItemBase.PortChannel", schema = Dict(1700)))]
                    pub struct PortChannel<'a, Mode> {
                        #[data_view(schema = Dict(1699), relaxed)]
                        pub structured_config: ::data_views::Field<super::super::super::super::super::super::super::eos_config::port_channel_interfaces::Item<'a, ::data_views::RelaxedValidated>>,
                        #[data_view(schema = Str(1572))]
                        pub raw_eos_cli: ::data_views::Field<&'a str>,
                    }

                    pub mod port_channel {
                    }
                }
            }

            #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.PointToPointServicesItemBase.Endpoints", schema = List(751)))]
            pub struct Endpoints<'a, Mode> (::data_views::Field<endpoints::Item<'a, Mode>>);

            pub mod endpoints {

                #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.PointToPointServicesItemBase.EndpointsItem", schema = Dict(1704)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Int(617))]
                    pub id: ::data_views::RequiredValue<i64, Mode>,
                    #[data_view(schema = List(749))]
                    pub nodes: ::data_views::RequiredValue<item::Nodes<'a, Mode>, Mode>,
                    #[data_view(schema = List(750))]
                    pub interfaces: ::data_views::RequiredValue<item::Interfaces<'a, Mode>, Mode>,
                    #[data_view(schema = Dict(1703))]
                    pub port_channel: ::data_views::Field<item::PortChannel<'a, Mode>>,
                }

                pub mod item {

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.PointToPointServicesItemBase.EndpointsItem.Nodes", schema = List(749)))]
                    pub struct Nodes<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(list, python(name = "AVDDesign.NetworkServicesItemBase.PointToPointServicesItemBase.EndpointsItem.Interfaces", schema = List(750)))]
                    pub struct Interfaces<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(python(name = "AVDDesign.NetworkServicesItemBase.PointToPointServicesItemBase.EndpointsItem.PortChannel", schema = Dict(1703)))]
                    pub struct PortChannel<'a, Mode> {
                        #[data_view(schema = Str(1574))]
                        pub mode: ::data_views::Field<&'a str>,
                        #[data_view(schema = Str(3))]
                        pub short_esi: ::data_views::Field<&'a str>,
                    }
                }
            }
        }
    }
}
