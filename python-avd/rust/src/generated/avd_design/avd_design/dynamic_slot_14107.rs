// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign._DynamicKeys.NetworkServices", schema = List(803), item_name = "AVDDesign._DynamicKeys.NetworkServicesItem"))]
pub struct DynamicSlot14107<'a, Mode> (::validated_data::Field<Item<'a, Mode>>);

#[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase", schema = Dict(1706)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1473))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(599))]
    pub mac_vrf_vni_base: ::validated_data::Field<i64>,
    #[data_view(schema = Int(600))]
    pub mac_vrf_id_base: ::validated_data::Field<i64>,
    #[data_view(schema = Int(601))]
    pub vlan_aware_bundle_number_base: ::validated_data::Field<i64>,
    #[data_view(schema = Int(602))]
    pub pseudowire_rt_base: ::validated_data::Field<i64>,
    #[data_view(schema = Bool(560))]
    pub enable_mlag_ibgp_peering_vrfs: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(561))]
    pub redistribute_mlag_ibgp_peering_vrfs: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1474))]
    pub evpn_vlan_bundle: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(689))]
    pub bgp_peer_groups: ::validated_data::Field<item::BgpPeerGroups<'a, Mode>>,
    #[data_view(schema = Dict(1640))]
    pub igmp_snooping: ::validated_data::Field<item::IgmpSnooping<'a, Mode>>,
    #[data_view(schema = Dict(1641))]
    pub evpn_l2_multicast: ::validated_data::Field<item::EvpnL2Multicast<'a, Mode>>,
    #[data_view(schema = Dict(1642))]
    pub vxlan_flood_multicast: ::validated_data::Field<item::VxlanFloodMulticast<'a, Mode>>,
    #[data_view(schema = Dict(1644))]
    pub evpn_l3_multicast: ::validated_data::Field<item::EvpnL3Multicast<'a, Mode>>,
    #[data_view(schema = List(695))]
    pub pim_rp_addresses: ::validated_data::Field<item::PimRpAddresses<'a, Mode>>,
    #[data_view(schema = Dict(1646))]
    pub igmp_snooping_querier: ::validated_data::Field<item::IgmpSnoopingQuerier<'a, Mode>>,
    #[data_view(schema = Bool(568))]
    pub evpn_l2_multi_domain: ::validated_data::Field<bool>,
    #[data_view(schema = List(745))]
    pub vrfs: ::validated_data::Field<item::Vrfs<'a, Mode>>,
    #[data_view(schema = List(747))]
    pub l2vlans: ::validated_data::Field<item::L2vlans<'a, Mode>>,
    #[data_view(schema = Dict(1698))]
    pub vpws: ::validated_data::Field<item::Vpws<'a, Mode>>,
    #[data_view(schema = List(752))]
    pub point_to_point_services: ::validated_data::Field<item::PointToPointServices<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.BgpPeerGroups", schema = List(689), item_name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.BgpPeerGroupsItem"))]
    pub struct BgpPeerGroups<'a, Mode> (::validated_data::Field<bgp_peer_groups::Item<'a, Mode>>);

    pub mod bgp_peer_groups {

        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.BgpPeerGroupsItemBase", schema = Dict(1638)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(1475))]
            pub name: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1476))]
            pub password: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1109))]
            pub cleartext_password: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(687))]
            pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
            #[data_view(schema = Dict(1634))]
            pub address_family_ipv4: ::validated_data::Field<item::AddressFamilyIpv4<'a, Mode>>,
            #[data_view(schema = Dict(1636))]
            pub address_family_ipv6: ::validated_data::Field<item::AddressFamilyIpv6<'a, Mode>>,
            #[data_view(schema = List(688))]
            pub listen_ranges: ::validated_data::Field<item::ListenRanges<'a, Mode>>,
            #[data_view(schema = Dict(912))]
            pub metadata: ::validated_data::Field<item::Metadata<'a, Mode>>,
            #[data_view(schema = Str(758))]
            pub remote_as: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(758))]
            pub local_as: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(3))]
            pub description: ::validated_data::Field<&'a str>,
            #[data_view(schema = Bool(7))]
            pub shutdown: ::validated_data::Field<bool>,
            #[data_view(schema = Dict(913))]
            pub as_path: ::validated_data::Field<item::AsPath<'a, Mode>>,
            #[data_view(schema = Dict(914))]
            pub remove_private_as: ::validated_data::Field<item::RemovePrivateAs<'a, Mode>>,
            #[data_view(schema = Dict(915))]
            pub remove_private_as_ingress: ::validated_data::Field<item::RemovePrivateAsIngress<'a, Mode>>,
            #[data_view(schema = Bool(7))]
            pub next_hop_unchanged: ::validated_data::Field<bool>,
            #[data_view(schema = Str(771))]
            pub update_source: ::validated_data::Field<&'a str>,
            #[data_view(schema = Bool(7))]
            pub route_reflector_client: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(323))]
            pub bfd: ::validated_data::Field<bool>,
            #[data_view(schema = Dict(916))]
            pub bfd_timers: ::validated_data::Field<item::BfdTimers<'a, Mode>>,
            #[data_view(schema = Int(381))]
            pub ebgp_multihop: ::validated_data::Field<i64>,
            #[data_view(schema = Bool(7))]
            pub next_hop_peer: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(7))]
            pub next_hop_self: ::validated_data::Field<bool>,
            #[data_view(schema = Str(772))]
            pub password_type: ::validated_data::Field<&'a str>,
            #[data_view(schema = Bool(7))]
            pub passive: ::validated_data::Field<bool>,
            #[data_view(schema = Dict(917))]
            pub default_originate: ::validated_data::Field<item::DefaultOriginate<'a, Mode>>,
            #[data_view(schema = Bool(324))]
            pub enforce_first_as: ::validated_data::Field<bool>,
            #[data_view(schema = Str(773))]
            pub send_community: ::validated_data::Field<&'a str>,
            #[data_view(schema = Int(382))]
            pub maximum_routes: ::validated_data::Field<i64>,
            #[data_view(schema = Str(774))]
            pub maximum_routes_warning_limit: ::validated_data::Field<&'a str>,
            #[data_view(schema = Bool(7))]
            pub maximum_routes_warning_only: ::validated_data::Field<bool>,
            #[data_view(schema = Dict(919))]
            pub maximum_accepted_routes: ::validated_data::Field<item::MaximumAcceptedRoutes<'a, Mode>>,
            #[data_view(schema = Dict(922))]
            pub missing_policy: ::validated_data::Field<item::MissingPolicy<'a, Mode>>,
            #[data_view(schema = Dict(923))]
            pub link_bandwidth: ::validated_data::Field<item::LinkBandwidth<'a, Mode>>,
            #[data_view(schema = Dict(924))]
            pub allowas_in: ::validated_data::Field<item::AllowasIn<'a, Mode>>,
            #[data_view(schema = Int(110))]
            pub weight: ::validated_data::Field<i64>,
            #[data_view(schema = Str(777))]
            pub timers: ::validated_data::Field<&'a str>,
            #[data_view(schema = Dict(925))]
            pub rib_in_pre_policy_retain: ::validated_data::Field<item::RibInPrePolicyRetain<'a, Mode>>,
            #[data_view(schema = Str(778))]
            pub route_map_in: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(779))]
            pub route_map_out: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(780))]
            pub peer_tag_in: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(781))]
            pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(3))]
            pub session_tracker: ::validated_data::Field<&'a str>,
            #[data_view(schema = Dict(926))]
            pub shared_secret: ::validated_data::Field<item::SharedSecret<'a, Mode>>,
            #[data_view(schema = Int(387))]
            pub ttl_maximum_hops: ::validated_data::Field<i64>,
            #[data_view(schema = Int(388))]
            pub maximum_advertised_routes: ::validated_data::Field<i64>,
            #[data_view(schema = Str(784))]
            pub maximum_advertised_routes_warning_limit: ::validated_data::Field<&'a str>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.BgpPeerGroupsItemBase.Nodes", schema = List(687)))]
            pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.BgpPeerGroupsItemBase.AddressFamilyIpv4", schema = Dict(1634)))]
            pub struct AddressFamilyIpv4<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub activate: ::validated_data::Field<bool>,
                #[data_view(schema = Str(778))]
                pub route_map_in: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(779))]
                pub route_map_out: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(803))]
                pub rcf_in: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(804))]
                pub rcf_out: ::validated_data::Field<&'a str>,
                #[data_view(schema = Dict(977))]
                pub default_originate: ::validated_data::Field<super::super::super::super::super::eos_config::router_bgp::address_family_ipv4::peer_groups::item::DefaultOriginate<'a, Mode>>,
                #[data_view(schema = Dict(980))]
                pub next_hop: ::validated_data::Field<super::super::super::super::super::eos_config::router_bgp::address_family_ipv4::peer_groups::item::NextHop<'a, Mode>>,
                #[data_view(schema = Str(812))]
                pub prefix_list_in: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(813))]
                pub prefix_list_out: ::validated_data::Field<&'a str>,
            }

            pub mod address_family_ipv4 {
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.BgpPeerGroupsItemBase.AddressFamilyIpv6", schema = Dict(1636)))]
            pub struct AddressFamilyIpv6<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub activate: ::validated_data::Field<bool>,
                #[data_view(schema = Str(778))]
                pub route_map_in: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(779))]
                pub route_map_out: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(803))]
                pub rcf_in: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(804))]
                pub rcf_out: ::validated_data::Field<&'a str>,
                #[data_view(schema = Dict(1635))]
                pub default_originate: ::validated_data::Field<address_family_ipv6::DefaultOriginate<'a, Mode>>,
                #[data_view(schema = Str(812))]
                pub prefix_list_in: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(813))]
                pub prefix_list_out: ::validated_data::Field<&'a str>,
            }

            pub mod address_family_ipv6 {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.BgpPeerGroupsItemBase.AddressFamilyIpv6.DefaultOriginate", schema = Dict(1635)))]
                pub struct DefaultOriginate<'a, Mode> {
                    #[data_view(schema = Bool(1))]
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    #[data_view(schema = Bool(336))]
                    pub always: ::validated_data::Field<bool>,
                    #[data_view(schema = Str(169))]
                    pub route_map: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.BgpPeerGroupsItemBase.ListenRanges", schema = List(688)))]
            pub struct ListenRanges<'a, Mode> (::validated_data::Field<listen_ranges::Item<'a, Mode>>);

            pub mod listen_ranges {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.BgpPeerGroupsItemBase.ListenRangesItem", schema = Dict(1637)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(1477))]
                    pub prefix: ::validated_data::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Str(1478))]
                    pub remote_as: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Bool(320))]
                    pub peer_id_include_router_id: ::validated_data::Field<bool>,
                    #[data_view(schema = Str(1479))]
                    pub peer_filter: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.BgpPeerGroupsItemBase.Metadata", schema = Dict(912)))]
            pub struct Metadata<'a, Mode> {
                #[data_view(schema = Str(278), rename = "type")]
                pub field_type: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.BgpPeerGroupsItemBase.AsPath", schema = Dict(913)))]
            pub struct AsPath<'a, Mode> {
                #[data_view(schema = Bool(321))]
                pub remote_as_replace_out: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(322))]
                pub prepend_own_disabled: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.BgpPeerGroupsItemBase.RemovePrivateAs", schema = Dict(914)))]
            pub struct RemovePrivateAs<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(7))]
                pub all: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(7))]
                pub replace_as: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.BgpPeerGroupsItemBase.RemovePrivateAsIngress", schema = Dict(915)))]
            pub struct RemovePrivateAsIngress<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(7))]
                pub replace_as: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.BgpPeerGroupsItemBase.BfdTimers", schema = Dict(916)))]
            pub struct BfdTimers<'a, Mode> {
                #[data_view(schema = Int(378))]
                pub interval: ::validated_data::RequiredValue<i64, Mode>,
                #[data_view(schema = Int(379))]
                pub min_rx: ::validated_data::RequiredValue<i64, Mode>,
                #[data_view(schema = Int(380))]
                pub multiplier: ::validated_data::RequiredValue<i64, Mode>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.BgpPeerGroupsItemBase.DefaultOriginate", schema = Dict(917)))]
            pub struct DefaultOriginate<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(7))]
                pub always: ::validated_data::Field<bool>,
                #[data_view(schema = Str(169))]
                pub route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.BgpPeerGroupsItemBase.MaximumAcceptedRoutes", schema = Dict(919)))]
            pub struct MaximumAcceptedRoutes<'a, Mode> {
                #[data_view(schema = Int(383))]
                pub limit: ::validated_data::RequiredValue<i64, Mode>,
                #[data_view(schema = Dict(918))]
                pub warning_limit: ::validated_data::Field<maximum_accepted_routes::WarningLimit<'a, Mode>>,
            }

            pub mod maximum_accepted_routes {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.BgpPeerGroupsItemBase.MaximumAcceptedRoutes.WarningLimit", schema = Dict(918)))]
                pub struct WarningLimit<'a, Mode> {
                    #[data_view(schema = Int(384))]
                    pub count: ::validated_data::Field<i64>,
                    #[data_view(schema = Int(385))]
                    pub percent: ::validated_data::Field<i64>,
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.BgpPeerGroupsItemBase.MissingPolicy", schema = Dict(922)))]
            pub struct MissingPolicy<'a, Mode> {
                #[data_view(schema = Dict(920))]
                pub direction_in: ::validated_data::Field<missing_policy::DirectionIn<'a, Mode>>,
                #[data_view(schema = Dict(921))]
                pub direction_out: ::validated_data::Field<missing_policy::DirectionOut<'a, Mode>>,
            }

            pub mod missing_policy {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.BgpPeerGroupsItemBase.MissingPolicy.DirectionIn", schema = Dict(920)))]
                pub struct DirectionIn<'a, Mode> {
                    #[data_view(schema = Str(775))]
                    pub action: ::validated_data::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Bool(325))]
                    pub include_community_list: ::validated_data::Field<bool>,
                    #[data_view(schema = Bool(326))]
                    pub include_prefix_list: ::validated_data::Field<bool>,
                    #[data_view(schema = Bool(327))]
                    pub include_sub_route_map: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.BgpPeerGroupsItemBase.MissingPolicy.DirectionOut", schema = Dict(921)))]
                pub struct DirectionOut<'a, Mode> {
                    #[data_view(schema = Str(775))]
                    pub action: ::validated_data::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Bool(325))]
                    pub include_community_list: ::validated_data::Field<bool>,
                    #[data_view(schema = Bool(326))]
                    pub include_prefix_list: ::validated_data::Field<bool>,
                    #[data_view(schema = Bool(327))]
                    pub include_sub_route_map: ::validated_data::Field<bool>,
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.BgpPeerGroupsItemBase.LinkBandwidth", schema = Dict(923)))]
            pub struct LinkBandwidth<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::validated_data::Field<bool>,
                #[data_view(schema = Str(776))]
                pub default: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.BgpPeerGroupsItemBase.AllowasIn", schema = Dict(924)))]
            pub struct AllowasIn<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::validated_data::Field<bool>,
                #[data_view(schema = Int(386))]
                pub times: ::validated_data::Field<i64>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.BgpPeerGroupsItemBase.RibInPrePolicyRetain", schema = Dict(925)))]
            pub struct RibInPrePolicyRetain<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::validated_data::Field<bool>,
                #[data_view(schema = Bool(7))]
                pub all: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.BgpPeerGroupsItemBase.SharedSecret", schema = Dict(926)))]
            pub struct SharedSecret<'a, Mode> {
                #[data_view(schema = Str(782))]
                pub profile: ::validated_data::RequiredValue<&'a str, Mode>,
                #[data_view(schema = Str(783))]
                pub hash_algorithm: ::validated_data::RequiredValue<&'a str, Mode>,
            }
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.IgmpSnooping", schema = Dict(1640)))]
    pub struct IgmpSnooping<'a, Mode> {
        #[data_view(schema = Dict(1639))]
        pub querier: ::validated_data::Field<igmp_snooping::Querier<'a, Mode>>,
        #[data_view(schema = Bool(562))]
        pub fast_leave: ::validated_data::Field<bool>,
    }

    pub mod igmp_snooping {

        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.IgmpSnooping.Querier", schema = Dict(1639)))]
        pub struct Querier<'a, Mode> {
            #[data_view(schema = Bool(547))]
            pub enabled: ::validated_data::Field<bool>,
            #[data_view(schema = Str(1480))]
            pub source_address: ::validated_data::Field<&'a str>,
            #[data_view(schema = Int(595))]
            pub version: ::validated_data::Field<i64>,
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.EvpnL2Multicast", schema = Dict(1641)))]
    pub struct EvpnL2Multicast<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub enabled: ::validated_data::Field<bool>,
        #[data_view(schema = Str(1170))]
        pub underlay_l2_multicast_group_ipv4_pool: ::validated_data::Field<&'a str>,
        #[data_view(schema = Int(603))]
        pub underlay_l2_multicast_group_ipv4_pool_offset: ::validated_data::Field<i64>,
        #[data_view(schema = Bool(563))]
        pub fast_leave: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(564))]
        pub always_redistribute_igmp: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VxlanFloodMulticast", schema = Dict(1642)))]
    pub struct VxlanFloodMulticast<'a, Mode> {
        #[data_view(schema = Bool(565))]
        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
        #[data_view(schema = Str(1481))]
        pub underlay_l2_multicast_group_ipv4_pool: ::validated_data::Field<&'a str>,
        #[data_view(schema = Int(603))]
        pub underlay_l2_multicast_group_ipv4_pool_offset: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.EvpnL3Multicast", schema = Dict(1644)))]
    pub struct EvpnL3Multicast<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub enabled: ::validated_data::Field<bool>,
        #[data_view(schema = Str(1482))]
        pub evpn_underlay_l3_multicast_group_ipv4_pool: ::validated_data::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Int(603))]
        pub evpn_underlay_l3_multicast_group_ipv4_pool_offset: ::validated_data::Field<i64>,
        #[data_view(schema = List(691))]
        pub evpn_peg: ::validated_data::Field<evpn_l3_multicast::EvpnPeg<'a, Mode>>,
    }

    pub mod evpn_l3_multicast {

        #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.EvpnL3Multicast.EvpnPeg", schema = List(691)))]
        pub struct EvpnPeg<'a, Mode> (::validated_data::Field<evpn_peg::Item<'a, Mode>>);

        pub mod evpn_peg {

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.EvpnL3Multicast.EvpnPegItem", schema = Dict(1643)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = List(690))]
                pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
                #[data_view(schema = Bool(566))]
                pub transit: ::validated_data::Field<bool>,
            }

            pub mod item {

                #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.EvpnL3Multicast.EvpnPegItem.Nodes", schema = List(690)))]
                pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);
            }
        }
    }

    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.PimRpAddresses", schema = List(695)))]
    pub struct PimRpAddresses<'a, Mode> (::validated_data::Field<pim_rp_addresses::Item<'a, Mode>>);

    pub mod pim_rp_addresses {

        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.PimRpAddressesItem", schema = Dict(1645)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = List(692))]
            pub rps: ::validated_data::Field<item::Rps<'a, Mode>>,
            #[data_view(schema = List(693))]
            pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
            #[data_view(schema = List(694))]
            pub groups: ::validated_data::Field<item::Groups<'a, Mode>>,
            #[data_view(schema = Str(1485))]
            pub access_list_name: ::validated_data::Field<&'a str>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.PimRpAddressesItem.Rps", schema = List(692)))]
            pub struct Rps<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.PimRpAddressesItem.Nodes", schema = List(693)))]
            pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.PimRpAddressesItem.Groups", schema = List(694)))]
            pub struct Groups<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.IgmpSnoopingQuerier", schema = Dict(1646)))]
    pub struct IgmpSnoopingQuerier<'a, Mode> {
        #[data_view(schema = Bool(567))]
        pub enabled: ::validated_data::Field<bool>,
        #[data_view(schema = Str(1480))]
        pub source_address: ::validated_data::Field<&'a str>,
        #[data_view(schema = Int(595))]
        pub version: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.Vrfs", schema = List(745), item_name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItem"))]
    pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

    pub mod vrfs {

        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase", schema = Dict(1696)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(49))]
            pub name: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(696))]
            pub address_families: ::validated_data::Field<item::AddressFamilies<'a, Mode>>,
            #[data_view(schema = Str(1486))]
            pub description: ::validated_data::Field<&'a str>,
            #[data_view(schema = Int(604))]
            pub vrf_vni: ::validated_data::Field<i64>,
            #[data_view(schema = Int(605))]
            pub vrf_id: ::validated_data::Field<i64>,
            #[data_view(schema = Str(1487))]
            pub rd_override: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1488))]
            pub rt_override: ::validated_data::Field<&'a str>,
            #[data_view(schema = Bool(569))]
            pub rt_import: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(570))]
            pub rt_export: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(571))]
            pub rt_import_evpn_remote: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(572))]
            pub rt_export_evpn_remote: ::validated_data::Field<bool>,
            #[data_view(schema = Str(1489))]
            pub evpn_vlan_bundle: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1490))]
            pub mlag_ibgp_peering_ipv4_pool: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1491))]
            pub mlag_ibgp_peering_ipv6_pool: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(697))]
            pub ip_helpers: ::validated_data::Field<item::IpHelpers<'a, Mode>>,
            #[data_view(schema = Bool(573))]
            pub enable_mlag_ibgp_peering_vrfs: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(574))]
            pub redistribute_mlag_ibgp_peering_vrfs: ::validated_data::Field<bool>,
            #[data_view(schema = Int(606))]
            pub mlag_ibgp_peering_vlan: ::validated_data::Field<i64>,
            #[data_view(schema = Dict(1649))]
            pub vtep_diagnostic: ::validated_data::Field<item::VtepDiagnostic<'a, Mode>>,
            #[data_view(schema = Dict(1654))]
            pub ospf: ::validated_data::Field<item::Ospf<'a, Mode>>,
            #[data_view(schema = Bool(576))]
            pub redistribute_ospf: ::validated_data::Field<bool>,
            #[data_view(schema = Dict(1656))]
            pub evpn_l3_multicast: ::validated_data::Field<item::EvpnL3Multicast<'a, Mode>>,
            #[data_view(schema = List(704))]
            pub pim_rp_addresses: ::validated_data::Field<item::PimRpAddresses<'a, Mode>>,
            #[data_view(schema = Bool(578))]
            pub evpn_l2_multi_domain: ::validated_data::Field<bool>,
            #[data_view(schema = List(716))]
            pub svis: ::validated_data::Field<item::Svis<'a, Mode>>,
            #[data_view(schema = List(725))]
            pub l3_interfaces: ::validated_data::Field<item::L3Interfaces<'a, Mode>>,
            #[data_view(schema = List(731))]
            pub l3_port_channels: ::validated_data::Field<item::L3PortChannels<'a, Mode>>,
            #[data_view(schema = List(732))]
            pub loopbacks: ::validated_data::Field<item::Loopbacks<'a, Mode>>,
            #[data_view(schema = List(733))]
            pub static_routes: ::validated_data::Field<item::StaticRoutes<'a, Mode>>,
            #[data_view(schema = List(734))]
            pub ipv6_static_routes: ::validated_data::Field<item::Ipv6StaticRoutes<'a, Mode>>,
            #[data_view(schema = Bool(591))]
            pub redistribute_static: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(592))]
            pub redistribute_connected: ::validated_data::Field<bool>,
            #[data_view(schema = List(736))]
            pub static_arp_entries: ::validated_data::Field<item::StaticArpEntries<'a, Mode>>,
            #[data_view(schema = List(738))]
            pub bgp_peers: ::validated_data::Field<item::BgpPeers<'a, Mode>>,
            #[data_view(schema = Dict(1691))]
            pub bgp: ::validated_data::Field<item::Bgp<'a, Mode>>,
            #[data_view(schema = List(740))]
            pub bgp_peer_groups: ::validated_data::Field<item::BgpPeerGroups<'a, Mode>>,
            #[data_view(schema = List(742))]
            pub additional_route_targets: ::validated_data::Field<item::AdditionalRouteTargets<'a, Mode>>,
            #[data_view(schema = List(744))]
            pub aggregate_addresses: ::validated_data::Field<item::AggregateAddresses<'a, Mode>>,
            #[data_view(schema = Bool(596))]
            pub validate_bgp_peers: ::validated_data::Field<bool>,
            #[data_view(schema = Str(1568))]
            pub raw_eos_cli: ::validated_data::Field<&'a str>,
            #[data_view(schema = Dict(1695), relaxed)]
            pub structured_config: ::validated_data::Field<super::super::super::super::eos_config::EosConfig<'a, ::validated_data::RelaxedValidated>>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.AddressFamilies", schema = List(696)))]
            pub struct AddressFamilies<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(indexed_list, primary_key(ip_helper), python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.IpHelpers", schema = List(697), item_name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.IpHelpersItem"))]
            pub struct IpHelpers<'a, Mode> (::validated_data::Field<ip_helpers::Item<'a, Mode>>);

            pub mod ip_helpers {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.IpHelpersItemBase", schema = Dict(1647)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(1492))]
                    pub ip_helper: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1493))]
                    pub source_interface: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1494))]
                    pub source_vrf: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.VtepDiagnostic", schema = Dict(1649)))]
            pub struct VtepDiagnostic<'a, Mode> {
                #[data_view(schema = Int(607))]
                pub loopback: ::validated_data::Field<i64>,
                #[data_view(schema = Str(1495))]
                pub loopback_description: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(1496))]
                pub loopback_ip_range: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(1497))]
                pub loopback_ipv6_range: ::validated_data::Field<&'a str>,
                #[data_view(schema = List(698))]
                pub loopback_ip_pools: ::validated_data::Field<vtep_diagnostic::LoopbackIpPools<'a, Mode>>,
                #[data_view(schema = Bool(575))]
                pub hardware_forwarding: ::validated_data::Field<bool>,
            }

            pub mod vtep_diagnostic {

                #[::validated_data::data_view(indexed_list, primary_key(pod), python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.VtepDiagnostic.LoopbackIpPools", schema = List(698), item_name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.VtepDiagnostic.LoopbackIpPoolsItem"))]
                pub struct LoopbackIpPools<'a, Mode> (::validated_data::Field<loopback_ip_pools::Item<'a, Mode>>);

                pub mod loopback_ip_pools {

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.VtepDiagnostic.LoopbackIpPoolsItemBase", schema = Dict(1648)))]
                    pub struct Item<'a, Mode> {
                        #[data_view(schema = Str(1498))]
                        pub pod: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Str(1170))]
                        pub ipv4_pool: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Str(1171))]
                        pub ipv6_pool: ::validated_data::Field<&'a str>,
                    }
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.Ospf", schema = Dict(1654)))]
            pub struct Ospf<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::validated_data::Field<bool>,
                #[data_view(schema = Int(608))]
                pub process_id: ::validated_data::Field<i64>,
                #[data_view(schema = Str(1499))]
                pub router_id: ::validated_data::Field<&'a str>,
                #[data_view(schema = Int(77))]
                pub max_lsa: ::validated_data::Field<i64>,
                #[data_view(schema = Bool(340))]
                pub bfd: ::validated_data::Field<bool>,
                #[data_view(schema = Dict(1650))]
                pub redistribute_bgp: ::validated_data::Field<ospf::RedistributeBgp<'a, Mode>>,
                #[data_view(schema = Dict(1651))]
                pub redistribute_connected: ::validated_data::Field<ospf::RedistributeConnected<'a, Mode>>,
                #[data_view(schema = Str(1500))]
                pub authentication: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(1501))]
                pub cleartext_simple_auth_key: ::validated_data::Field<&'a str>,
                #[data_view(schema = List(699))]
                pub message_digest_keys: ::validated_data::Field<ospf::MessageDigestKeys<'a, Mode>>,
                #[data_view(schema = List(700))]
                pub nodes: ::validated_data::Field<ospf::Nodes<'a, Mode>>,
                #[data_view(schema = Dict(1653), relaxed)]
                pub structured_config: ::validated_data::Field<super::super::super::super::super::eos_config::router_ospf::process_ids::Item<'a, ::validated_data::RelaxedValidated>>,
            }

            pub mod ospf {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.Ospf.RedistributeBgp", schema = Dict(1650)))]
                pub struct RedistributeBgp<'a, Mode> {
                    #[data_view(schema = Bool(37))]
                    pub enabled: ::validated_data::Field<bool>,
                    #[data_view(schema = Str(169))]
                    pub route_map: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.Ospf.RedistributeConnected", schema = Dict(1651)))]
                pub struct RedistributeConnected<'a, Mode> {
                    #[data_view(schema = Bool(340))]
                    pub enabled: ::validated_data::Field<bool>,
                    #[data_view(schema = Str(169))]
                    pub route_map: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view(indexed_list, primary_key(id), python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.Ospf.MessageDigestKeys", schema = List(699), item_name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.Ospf.MessageDigestKeysItem"))]
                pub struct MessageDigestKeys<'a, Mode> (::validated_data::Field<message_digest_keys::Item<'a, Mode>>);

                pub mod message_digest_keys {

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.Ospf.MessageDigestKeysItemBase", schema = Dict(1652)))]
                    pub struct Item<'a, Mode> {
                        #[data_view(schema = Int(77))]
                        pub id: ::validated_data::Field<i64>,
                        #[data_view(schema = Str(1502))]
                        pub hash_algorithm: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Str(1503))]
                        pub cleartext_key: ::validated_data::RequiredValue<&'a str, Mode>,
                    }
                }

                #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.Ospf.Nodes", schema = List(700)))]
                pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.EvpnL3Multicast", schema = Dict(1656)))]
            pub struct EvpnL3Multicast<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::validated_data::Field<bool>,
                #[data_view(schema = Str(1504))]
                pub evpn_underlay_l3_multicast_group: ::validated_data::Field<&'a str>,
                #[data_view(schema = List(702))]
                pub evpn_peg: ::validated_data::Field<evpn_l3_multicast::EvpnPeg<'a, Mode>>,
            }

            pub mod evpn_l3_multicast {

                #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.EvpnL3Multicast.EvpnPeg", schema = List(702)))]
                pub struct EvpnPeg<'a, Mode> (::validated_data::Field<evpn_peg::Item<'a, Mode>>);

                pub mod evpn_peg {

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.EvpnL3Multicast.EvpnPegItem", schema = Dict(1655)))]
                    pub struct Item<'a, Mode> {
                        #[data_view(schema = List(701))]
                        pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
                        #[data_view(schema = Bool(577))]
                        pub transit: ::validated_data::Field<bool>,
                    }

                    pub mod item {

                        #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.EvpnL3Multicast.EvpnPegItem.Nodes", schema = List(701)))]
                        pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);
                    }
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.PimRpAddresses", schema = List(704)))]
            pub struct PimRpAddresses<'a, Mode> (::validated_data::Field<pim_rp_addresses::Item<'a, Mode>>);

            pub mod pim_rp_addresses {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.PimRpAddressesItem", schema = Dict(1657)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = List(703))]
                    pub rps: ::validated_data::Field<item::Rps<'a, Mode>>,
                    #[data_view(schema = List(693))]
                    pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
                    #[data_view(schema = List(694))]
                    pub groups: ::validated_data::Field<item::Groups<'a, Mode>>,
                    #[data_view(schema = Str(1505))]
                    pub access_list_name: ::validated_data::Field<&'a str>,
                }

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.PimRpAddressesItem.Rps", schema = List(703)))]
                    pub struct Rps<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.PimRpAddressesItem.Nodes", schema = List(693)))]
                    pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.PimRpAddressesItem.Groups", schema = List(694)))]
                    pub struct Groups<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }

            #[::validated_data::data_view(list, primary_key(id), python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.Svis", schema = List(716), item_name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItem"))]
            pub struct Svis<'a, Mode> (::validated_data::Field<svis::Item<'a, Mode>>);

            pub mod svis {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase", schema = Dict(1674)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Int(609))]
                    pub id: ::validated_data::Field<i64>,
                    #[data_view(schema = Str(1506))]
                    pub name: ::validated_data::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Dict(1658))]
                    pub address_locking: ::validated_data::Field<super::super::super::super::super::super::eos_config::vlans::item::address_locking::AddressFamily<'a, Mode>>,
                    #[data_view(schema = Str(1507))]
                    pub profile: ::validated_data::Field<&'a str>,
                    #[data_view(schema = List(705))]
                    pub tags: ::validated_data::Field<item::Tags<'a, Mode>>,
                    #[data_view(schema = Str(1509))]
                    pub evpn_vlan_bundle: ::validated_data::Field<&'a str>,
                    #[data_view(schema = List(715))]
                    pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
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
                    pub structured_config: ::validated_data::Field<super::super::super::super::super::super::eos_config::vlan_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
                    #[data_view(schema = Bool(588))]
                    pub evpn_l2_multi_domain: ::validated_data::Field<bool>,
                }

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Tags", schema = List(705)))]
                    pub struct Tags<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(indexed_list, primary_key(node), python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Nodes", schema = List(715), item_name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItem"))]
                    pub struct Nodes<'a, Mode> (::validated_data::Field<nodes::Item<'a, Mode>>);

                    pub mod nodes {

                        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase", schema = Dict(1673)))]
                        pub struct Item<'a, Mode> {
                            #[data_view(schema = Str(1510))]
                            pub node: ::validated_data::Field<&'a str>,
                            #[data_view(schema = List(705))]
                            pub tags: ::validated_data::Field<item::Tags<'a, Mode>>,
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
                            pub structured_config: ::validated_data::Field<super::super::super::super::super::super::super::super::eos_config::vlan_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
                            #[data_view(schema = Bool(588))]
                            pub evpn_l2_multi_domain: ::validated_data::Field<bool>,
                        }

                        pub mod item {

                            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Tags", schema = List(705)))]
                            pub struct Tags<'a, Mode> (::validated_data::Field<&'a str>);

                            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.IpAddressSecondaries", schema = List(222)))]
                            pub struct IpAddressSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

                            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6AddressVirtuals", schema = List(706)))]
                            pub struct Ipv6AddressVirtuals<'a, Mode> (::validated_data::Field<&'a str>);

                            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6Nd", schema = Dict(1660)))]
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

                                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6Nd.RaDnsServers", schema = Dict(1659)))]
                                pub struct RaDnsServers<'a, Mode> {
                                    #[data_view(schema = List(105))]
                                    pub servers: ::validated_data::Field<ra_dns_servers::Servers<'a, Mode>>,
                                    #[data_view(schema = Int(95))]
                                    pub dns_servers_lifetime: ::validated_data::Field<i64>,
                                }

                                pub mod ra_dns_servers {

                                    #[::validated_data::data_view(indexed_list, primary_key(address), python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6Nd.RaDnsServers.Servers", schema = List(105), item_name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6Nd.RaDnsServers.ServersItem"))]
                                    pub struct Servers<'a, Mode> (::validated_data::Field<servers::Item<'a, Mode>>);

                                    pub mod servers {

                                        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6Nd.RaDnsServers.ServersItemBase", schema = Dict(236)))]
                                        pub struct Item<'a, Mode> {
                                            #[data_view(schema = Str(208))]
                                            pub address: ::validated_data::Field<&'a str>,
                                            #[data_view(schema = Int(94))]
                                            pub lifetime: ::validated_data::Field<i64>,
                                        }
                                    }
                                }
                            }

                            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6DhcpRelay", schema = Dict(1662)))]
                            pub struct Ipv6DhcpRelay<'a, Mode> {
                                #[data_view(schema = List(707))]
                                pub destinations: ::validated_data::Field<ipv6_dhcp_relay::Destinations<'a, Mode>>,
                            }

                            pub mod ipv6_dhcp_relay {

                                #[::validated_data::data_view(indexed_list, primary_key(address), python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6DhcpRelay.Destinations", schema = List(707), item_name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6DhcpRelay.DestinationsItem"))]
                                pub struct Destinations<'a, Mode> (::validated_data::Field<destinations::Item<'a, Mode>>);

                                pub mod destinations {

                                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6DhcpRelay.DestinationsItemBase", schema = Dict(1661)))]
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

                            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.IpAddressVirtualSecondaries", schema = List(708)))]
                            pub struct IpAddressVirtualSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

                            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.IpVirtualRouterAddresses", schema = List(709)))]
                            pub struct IpVirtualRouterAddresses<'a, Mode> (::validated_data::Field<&'a str>);

                            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6VirtualRouterAddresses", schema = List(710)))]
                            pub struct Ipv6VirtualRouterAddresses<'a, Mode> (::validated_data::Field<&'a str>);

                            #[::validated_data::data_view(indexed_list, primary_key(ip_helper), python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.IpHelpers", schema = List(697), item_name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.IpHelpersItem"))]
                            pub struct IpHelpers<'a, Mode> (::validated_data::Field<ip_helpers::Item<'a, Mode>>);

                            pub mod ip_helpers {

                                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.IpHelpersItemBase", schema = Dict(1647)))]
                                pub struct Item<'a, Mode> {
                                    #[data_view(schema = Str(1492))]
                                    pub ip_helper: ::validated_data::Field<&'a str>,
                                    #[data_view(schema = Str(1493))]
                                    pub source_interface: ::validated_data::Field<&'a str>,
                                    #[data_view(schema = Str(1494))]
                                    pub source_vrf: ::validated_data::Field<&'a str>,
                                }
                            }

                            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.StaticRoutes", schema = List(711)))]
                            pub struct StaticRoutes<'a, Mode> (::validated_data::Field<static_routes::Item<'a, Mode>>);

                            pub mod static_routes {

                                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.StaticRoutesItem", schema = Dict(1663)))]
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

                            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6StaticRoutes", schema = List(712)))]
                            pub struct Ipv6StaticRoutes<'a, Mode> (::validated_data::Field<ipv6_static_routes::Item<'a, Mode>>);

                            pub mod ipv6_static_routes {

                                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ipv6StaticRoutesItem", schema = Dict(1663)))]
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

                            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.TrunkGroups", schema = List(713)))]
                            pub struct TrunkGroups<'a, Mode> (::validated_data::Field<&'a str>);

                            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.EvpnL2Multicast", schema = Dict(1664)))]
                            pub struct EvpnL2Multicast<'a, Mode> {
                                #[data_view(schema = Bool(7))]
                                pub enabled: ::validated_data::Field<bool>,
                                #[data_view(schema = Bool(583))]
                                pub always_redistribute_igmp: ::validated_data::Field<bool>,
                            }

                            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.VxlanFloodMulticast", schema = Dict(1665)))]
                            pub struct VxlanFloodMulticast<'a, Mode> {
                                #[data_view(schema = Bool(585))]
                                pub enabled: ::validated_data::Field<bool>,
                                #[data_view(schema = Str(1528))]
                                pub underlay_multicast_group: ::validated_data::Field<&'a str>,
                            }

                            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.EvpnL3Multicast", schema = Dict(1666)))]
                            pub struct EvpnL3Multicast<'a, Mode> {
                                #[data_view(schema = Bool(7))]
                                pub enabled: ::validated_data::Field<bool>,
                            }

                            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.IgmpSnooping", schema = Dict(1608)))]
                            pub struct IgmpSnooping<'a, Mode> {
                                #[data_view(schema = Bool(546))]
                                pub enabled: ::validated_data::Field<bool>,
                                #[data_view(schema = Dict(1607))]
                                pub querier: ::validated_data::Field<igmp_snooping::Querier<'a, Mode>>,
                                #[data_view(schema = Bool(548))]
                                pub fast_leave: ::validated_data::Field<bool>,
                            }

                            pub mod igmp_snooping {

                                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.IgmpSnooping.Querier", schema = Dict(1607)))]
                                pub struct Querier<'a, Mode> {
                                    #[data_view(schema = Bool(547))]
                                    pub enabled: ::validated_data::Field<bool>,
                                    #[data_view(schema = Str(1437))]
                                    pub source_address: ::validated_data::Field<&'a str>,
                                    #[data_view(schema = Int(595))]
                                    pub version: ::validated_data::Field<i64>,
                                }
                            }

                            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.IgmpSnoopingQuerier", schema = Dict(1667)))]
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

                            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ospf", schema = Dict(1669)))]
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

                                #[::validated_data::data_view(indexed_list, primary_key(id), python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ospf.MessageDigestKeys", schema = List(714), item_name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ospf.MessageDigestKeysItem"))]
                                pub struct MessageDigestKeys<'a, Mode> (::validated_data::Field<message_digest_keys::Item<'a, Mode>>);

                                pub mod message_digest_keys {

                                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Ospf.MessageDigestKeysItemBase", schema = Dict(1668)))]
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

                            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.NodesItemBase.Bgp", schema = Dict(1671)))]
                            pub struct Bgp<'a, Mode> {
                                #[data_view(schema = Dict(1670), relaxed)]
                                pub structured_config: ::validated_data::Field<super::super::super::super::super::super::super::super::super::eos_config::router_bgp::vlans::Item<'a, ::validated_data::RelaxedValidated>>,
                                #[data_view(schema = Str(1535))]
                                pub raw_eos_cli: ::validated_data::Field<&'a str>,
                            }

                            pub mod bgp {
                            }
                        }
                    }

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.IpAddressSecondaries", schema = List(222)))]
                    pub struct IpAddressSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6AddressVirtuals", schema = List(706)))]
                    pub struct Ipv6AddressVirtuals<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6Nd", schema = Dict(1660)))]
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

                        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6Nd.RaDnsServers", schema = Dict(1659)))]
                        pub struct RaDnsServers<'a, Mode> {
                            #[data_view(schema = List(105))]
                            pub servers: ::validated_data::Field<ra_dns_servers::Servers<'a, Mode>>,
                            #[data_view(schema = Int(95))]
                            pub dns_servers_lifetime: ::validated_data::Field<i64>,
                        }

                        pub mod ra_dns_servers {

                            #[::validated_data::data_view(indexed_list, primary_key(address), python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6Nd.RaDnsServers.Servers", schema = List(105), item_name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6Nd.RaDnsServers.ServersItem"))]
                            pub struct Servers<'a, Mode> (::validated_data::Field<servers::Item<'a, Mode>>);

                            pub mod servers {

                                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6Nd.RaDnsServers.ServersItemBase", schema = Dict(236)))]
                                pub struct Item<'a, Mode> {
                                    #[data_view(schema = Str(208))]
                                    pub address: ::validated_data::Field<&'a str>,
                                    #[data_view(schema = Int(94))]
                                    pub lifetime: ::validated_data::Field<i64>,
                                }
                            }
                        }
                    }

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6DhcpRelay", schema = Dict(1662)))]
                    pub struct Ipv6DhcpRelay<'a, Mode> {
                        #[data_view(schema = List(707))]
                        pub destinations: ::validated_data::Field<ipv6_dhcp_relay::Destinations<'a, Mode>>,
                    }

                    pub mod ipv6_dhcp_relay {

                        #[::validated_data::data_view(indexed_list, primary_key(address), python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6DhcpRelay.Destinations", schema = List(707), item_name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6DhcpRelay.DestinationsItem"))]
                        pub struct Destinations<'a, Mode> (::validated_data::Field<destinations::Item<'a, Mode>>);

                        pub mod destinations {

                            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6DhcpRelay.DestinationsItemBase", schema = Dict(1661)))]
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

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.IpAddressVirtualSecondaries", schema = List(708)))]
                    pub struct IpAddressVirtualSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.IpVirtualRouterAddresses", schema = List(709)))]
                    pub struct IpVirtualRouterAddresses<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6VirtualRouterAddresses", schema = List(710)))]
                    pub struct Ipv6VirtualRouterAddresses<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(indexed_list, primary_key(ip_helper), python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.IpHelpers", schema = List(697), item_name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.IpHelpersItem"))]
                    pub struct IpHelpers<'a, Mode> (::validated_data::Field<ip_helpers::Item<'a, Mode>>);

                    pub mod ip_helpers {

                        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.IpHelpersItemBase", schema = Dict(1647)))]
                        pub struct Item<'a, Mode> {
                            #[data_view(schema = Str(1492))]
                            pub ip_helper: ::validated_data::Field<&'a str>,
                            #[data_view(schema = Str(1493))]
                            pub source_interface: ::validated_data::Field<&'a str>,
                            #[data_view(schema = Str(1494))]
                            pub source_vrf: ::validated_data::Field<&'a str>,
                        }
                    }

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.StaticRoutes", schema = List(711)))]
                    pub struct StaticRoutes<'a, Mode> (::validated_data::Field<static_routes::Item<'a, Mode>>);

                    pub mod static_routes {

                        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.StaticRoutesItem", schema = Dict(1663)))]
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

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6StaticRoutes", schema = List(712)))]
                    pub struct Ipv6StaticRoutes<'a, Mode> (::validated_data::Field<ipv6_static_routes::Item<'a, Mode>>);

                    pub mod ipv6_static_routes {

                        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ipv6StaticRoutesItem", schema = Dict(1663)))]
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

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.TrunkGroups", schema = List(713)))]
                    pub struct TrunkGroups<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.EvpnL2Multicast", schema = Dict(1664)))]
                    pub struct EvpnL2Multicast<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub enabled: ::validated_data::Field<bool>,
                        #[data_view(schema = Bool(583))]
                        pub always_redistribute_igmp: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.VxlanFloodMulticast", schema = Dict(1665)))]
                    pub struct VxlanFloodMulticast<'a, Mode> {
                        #[data_view(schema = Bool(585))]
                        pub enabled: ::validated_data::Field<bool>,
                        #[data_view(schema = Str(1528))]
                        pub underlay_multicast_group: ::validated_data::Field<&'a str>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.EvpnL3Multicast", schema = Dict(1666)))]
                    pub struct EvpnL3Multicast<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub enabled: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.IgmpSnooping", schema = Dict(1608)))]
                    pub struct IgmpSnooping<'a, Mode> {
                        #[data_view(schema = Bool(546))]
                        pub enabled: ::validated_data::Field<bool>,
                        #[data_view(schema = Dict(1607))]
                        pub querier: ::validated_data::Field<igmp_snooping::Querier<'a, Mode>>,
                        #[data_view(schema = Bool(548))]
                        pub fast_leave: ::validated_data::Field<bool>,
                    }

                    pub mod igmp_snooping {

                        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.IgmpSnooping.Querier", schema = Dict(1607)))]
                        pub struct Querier<'a, Mode> {
                            #[data_view(schema = Bool(547))]
                            pub enabled: ::validated_data::Field<bool>,
                            #[data_view(schema = Str(1437))]
                            pub source_address: ::validated_data::Field<&'a str>,
                            #[data_view(schema = Int(595))]
                            pub version: ::validated_data::Field<i64>,
                        }
                    }

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.IgmpSnoopingQuerier", schema = Dict(1667)))]
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

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ospf", schema = Dict(1669)))]
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

                        #[::validated_data::data_view(indexed_list, primary_key(id), python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ospf.MessageDigestKeys", schema = List(714), item_name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ospf.MessageDigestKeysItem"))]
                        pub struct MessageDigestKeys<'a, Mode> (::validated_data::Field<message_digest_keys::Item<'a, Mode>>);

                        pub mod message_digest_keys {

                            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Ospf.MessageDigestKeysItemBase", schema = Dict(1668)))]
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

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.SvisItemBase.Bgp", schema = Dict(1671)))]
                    pub struct Bgp<'a, Mode> {
                        #[data_view(schema = Dict(1670), relaxed)]
                        pub structured_config: ::validated_data::Field<super::super::super::super::super::super::super::eos_config::router_bgp::vlans::Item<'a, ::validated_data::RelaxedValidated>>,
                        #[data_view(schema = Str(1535))]
                        pub raw_eos_cli: ::validated_data::Field<&'a str>,
                    }

                    pub mod bgp {
                    }
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3Interfaces", schema = List(725)))]
            pub struct L3Interfaces<'a, Mode> (::validated_data::Field<l3_interfaces::Item<'a, Mode>>);

            pub mod l3_interfaces {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem", schema = Dict(1678)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = List(717))]
                    pub interfaces: ::validated_data::Field<item::Interfaces<'a, Mode>>,
                    #[data_view(schema = List(718))]
                    pub encapsulation_dot1q_vlan: ::validated_data::Field<item::EncapsulationDot1qVlan<'a, Mode>>,
                    #[data_view(schema = List(222))]
                    pub ip_addresses: ::validated_data::Field<item::IpAddresses<'a, Mode>>,
                    #[data_view(schema = List(719))]
                    pub ipv6_addresses: ::validated_data::Field<item::Ipv6Addresses<'a, Mode>>,
                    #[data_view(schema = List(720))]
                    pub static_routes: ::validated_data::Field<item::StaticRoutes<'a, Mode>>,
                    #[data_view(schema = List(721))]
                    pub ipv6_static_routes: ::validated_data::Field<item::Ipv6StaticRoutes<'a, Mode>>,
                    #[data_view(schema = List(722))]
                    pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
                    #[data_view(schema = Bool(124))]
                    pub arp_gratuitous_accept: ::validated_data::Field<bool>,
                    #[data_view(schema = Str(3))]
                    pub description: ::validated_data::Field<&'a str>,
                    #[data_view(schema = List(723))]
                    pub descriptions: ::validated_data::Field<item::Descriptions<'a, Mode>>,
                    #[data_view(schema = Bool(37))]
                    pub enabled: ::validated_data::Field<bool>,
                    #[data_view(schema = Int(77))]
                    pub mtu: ::validated_data::Field<i64>,
                    #[data_view(schema = Str(49))]
                    pub ipv4_acl_in: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(49))]
                    pub ipv4_acl_out: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1540))]
                    pub ipv6_acl_in: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1541))]
                    pub ipv6_acl_out: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Dict(1676))]
                    pub ospf: ::validated_data::Field<item::Ospf<'a, Mode>>,
                    #[data_view(schema = Dict(1677))]
                    pub pim: ::validated_data::Field<item::Pim<'a, Mode>>,
                    #[data_view(schema = Dict(1504))]
                    pub flow_tracking: ::validated_data::Field<item::FlowTracking<'a, Mode>>,
                    #[data_view(schema = Bool(589))]
                    pub sflow: ::validated_data::Field<bool>,
                    #[data_view(schema = List(572))]
                    pub monitor_sessions: ::validated_data::Field<item::MonitorSessions<'a, Mode>>,
                    #[data_view(schema = List(574))]
                    pub campus_link_type: ::validated_data::Field<item::CampusLinkType<'a, Mode>>,
                    #[data_view(schema = Dict(1432), relaxed)]
                    pub structured_config: ::validated_data::Field<super::super::super::super::super::super::eos_config::ethernet_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
                    #[data_view(schema = Str(1544))]
                    pub raw_eos_cli: ::validated_data::Field<&'a str>,
                }

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.Interfaces", schema = List(717)))]
                    pub struct Interfaces<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.EncapsulationDot1qVlan", schema = List(718)))]
                    pub struct EncapsulationDot1qVlan<'a, Mode> (::validated_data::Field<i64>);

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.IpAddresses", schema = List(222)))]
                    pub struct IpAddresses<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.Ipv6Addresses", schema = List(719)))]
                    pub struct Ipv6Addresses<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.StaticRoutes", schema = List(720)))]
                    pub struct StaticRoutes<'a, Mode> (::validated_data::Field<static_routes::Item<'a, Mode>>);

                    pub mod static_routes {

                        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.StaticRoutesItem", schema = Dict(1663)))]
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

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.Ipv6StaticRoutes", schema = List(721)))]
                    pub struct Ipv6StaticRoutes<'a, Mode> (::validated_data::Field<ipv6_static_routes::Item<'a, Mode>>);

                    pub mod ipv6_static_routes {

                        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.Ipv6StaticRoutesItem", schema = Dict(1663)))]
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

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.Nodes", schema = List(722)))]
                    pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.Descriptions", schema = List(723)))]
                    pub struct Descriptions<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.Ospf", schema = Dict(1676)))]
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
                        #[data_view(schema = Str(1542))]
                        pub simple_auth_key: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Str(1532))]
                        pub cleartext_simple_auth_key: ::validated_data::Field<&'a str>,
                        #[data_view(schema = List(724))]
                        pub message_digest_keys: ::validated_data::Field<ospf::MessageDigestKeys<'a, Mode>>,
                    }

                    pub mod ospf {

                        #[::validated_data::data_view(indexed_list, primary_key(id), python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.Ospf.MessageDigestKeys", schema = List(724), item_name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.Ospf.MessageDigestKeysItem"))]
                        pub struct MessageDigestKeys<'a, Mode> (::validated_data::Field<message_digest_keys::Item<'a, Mode>>);

                        pub mod message_digest_keys {

                            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.Ospf.MessageDigestKeysItemBase", schema = Dict(1675)))]
                            pub struct Item<'a, Mode> {
                                #[data_view(schema = Int(77))]
                                pub id: ::validated_data::Field<i64>,
                                #[data_view(schema = Str(1502))]
                                pub hash_algorithm: ::validated_data::Field<&'a str>,
                                #[data_view(schema = Str(1543))]
                                pub key: ::validated_data::Field<&'a str>,
                                #[data_view(schema = Str(1534))]
                                pub cleartext_key: ::validated_data::Field<&'a str>,
                            }
                        }
                    }

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.Pim", schema = Dict(1677)))]
                    pub struct Pim<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub enabled: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.FlowTracking", schema = Dict(1504)))]
                    pub struct FlowTracking<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub enabled: ::validated_data::Field<bool>,
                        #[data_view(schema = Str(1155))]
                        pub name: ::validated_data::Field<&'a str>,
                    }

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.MonitorSessions", schema = List(572)))]
                    pub struct MonitorSessions<'a, Mode> (::validated_data::Field<monitor_sessions::Item<'a, Mode>>);

                    pub mod monitor_sessions {

                        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.MonitorSessionsItem", schema = Dict(1430)))]
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

                            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.MonitorSessionsItem.SourceSettings", schema = Dict(1427)))]
                            pub struct SourceSettings<'a, Mode> {
                                #[data_view(schema = Str(593))]
                                pub direction: ::validated_data::Field<&'a str>,
                                #[data_view(schema = Dict(1426))]
                                pub access_group: ::validated_data::Field<source_settings::AccessGroup<'a, Mode>>,
                            }

                            pub mod source_settings {

                                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.MonitorSessionsItem.SourceSettings.AccessGroup", schema = Dict(1426)))]
                                pub struct AccessGroup<'a, Mode> {
                                    #[data_view(schema = Str(594), rename = "type")]
                                    pub field_type: ::validated_data::Field<&'a str>,
                                    #[data_view(schema = Str(1161))]
                                    pub name: ::validated_data::Field<&'a str>,
                                    #[data_view(schema = Int(77))]
                                    pub priority: ::validated_data::Field<i64>,
                                }
                            }

                            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.MonitorSessionsItem.SessionSettings", schema = Dict(1429)))]
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

                                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.MonitorSessionsItem.SessionSettings.AccessGroup", schema = Dict(1428)))]
                                pub struct AccessGroup<'a, Mode> {
                                    #[data_view(schema = Str(594), rename = "type")]
                                    pub field_type: ::validated_data::Field<&'a str>,
                                    #[data_view(schema = Str(1161))]
                                    pub name: ::validated_data::Field<&'a str>,
                                }

                                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.MonitorSessionsItem.SessionSettings.Truncate", schema = Dict(666)))]
                                pub struct Truncate<'a, Mode> {
                                    #[data_view(schema = Bool(7))]
                                    pub enabled: ::validated_data::Field<bool>,
                                    #[data_view(schema = Int(274))]
                                    pub size: ::validated_data::Field<i64>,
                                }
                            }
                        }
                    }

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3InterfacesItem.CampusLinkType", schema = List(574)))]
                    pub struct CampusLinkType<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3PortChannels", schema = List(731)))]
            pub struct L3PortChannels<'a, Mode> (::validated_data::Field<l3_port_channels::Item<'a, Mode>>);

            pub mod l3_port_channels {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem", schema = Dict(1681)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(1352))]
                    pub name: ::validated_data::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Str(1545))]
                    pub node: ::validated_data::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Bool(124))]
                    pub arp_gratuitous_accept: ::validated_data::Field<bool>,
                    #[data_view(schema = Str(1353))]
                    pub description: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1354))]
                    pub mode: ::validated_data::Field<&'a str>,
                    #[data_view(schema = List(726))]
                    pub member_interfaces: ::validated_data::Field<item::MemberInterfaces<'a, Mode>>,
                    #[data_view(schema = Str(1546))]
                    pub ip_address: ::validated_data::Field<&'a str>,
                    #[data_view(schema = List(727))]
                    pub ip_address_secondaries: ::validated_data::Field<item::IpAddressSecondaries<'a, Mode>>,
                    #[data_view(schema = List(728))]
                    pub ipv6_addresses: ::validated_data::Field<item::Ipv6Addresses<'a, Mode>>,
                    #[data_view(schema = Int(565))]
                    pub encapsulation_dot1q_vlan: ::validated_data::Field<i64>,
                    #[data_view(schema = Bool(480))]
                    pub enabled: ::validated_data::Field<bool>,
                    #[data_view(schema = Str(1333))]
                    pub peer: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1359))]
                    pub peer_port_channel: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Int(613))]
                    pub mtu: ::validated_data::Field<i64>,
                    #[data_view(schema = Str(1547))]
                    pub ipv4_acl_in: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1548))]
                    pub ipv4_acl_out: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1549))]
                    pub ipv6_acl_in: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1550))]
                    pub ipv6_acl_out: ::validated_data::Field<&'a str>,
                    #[data_view(schema = List(729))]
                    pub static_routes: ::validated_data::Field<item::StaticRoutes<'a, Mode>>,
                    #[data_view(schema = List(730))]
                    pub ipv6_static_routes: ::validated_data::Field<item::Ipv6StaticRoutes<'a, Mode>>,
                    #[data_view(schema = Dict(1680))]
                    pub ospf: ::validated_data::Field<item::Ospf<'a, Mode>>,
                    #[data_view(schema = Dict(1511))]
                    pub flow_tracking: ::validated_data::Field<item::FlowTracking<'a, Mode>>,
                    #[data_view(schema = Dict(1415), relaxed)]
                    pub structured_config: ::validated_data::Field<super::super::super::super::super::super::eos_config::port_channel_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
                    #[data_view(schema = Str(1552))]
                    pub raw_eos_cli: ::validated_data::Field<&'a str>,
                }

                pub mod item {

                    #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.MemberInterfaces", schema = List(726), item_name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.MemberInterfacesItem"))]
                    pub struct MemberInterfaces<'a, Mode> (::validated_data::Field<member_interfaces::Item<'a, Mode>>);

                    pub mod member_interfaces {

                        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.MemberInterfacesItemBase", schema = Dict(1679)))]
                        pub struct Item<'a, Mode> {
                            #[data_view(schema = Str(1355))]
                            pub name: ::validated_data::RequiredValue<&'a str, Mode>,
                            #[data_view(schema = Str(1356))]
                            pub description: ::validated_data::Field<&'a str>,
                            #[data_view(schema = Str(1357))]
                            pub peer: ::validated_data::Field<&'a str>,
                            #[data_view(schema = Str(1334))]
                            pub peer_interface: ::validated_data::Field<&'a str>,
                            #[data_view(schema = Str(178))]
                            pub speed: ::validated_data::Field<&'a str>,
                            #[data_view(schema = Dict(1507), relaxed)]
                            pub structured_config: ::validated_data::Field<super::super::super::super::super::super::super::super::eos_config::ethernet_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
                        }

                        pub mod item {
                        }
                    }

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.IpAddressSecondaries", schema = List(727)))]
                    pub struct IpAddressSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.Ipv6Addresses", schema = List(728)))]
                    pub struct Ipv6Addresses<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.StaticRoutes", schema = List(729)))]
                    pub struct StaticRoutes<'a, Mode> (::validated_data::Field<static_routes::Item<'a, Mode>>);

                    pub mod static_routes {

                        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.StaticRoutesItem", schema = Dict(1663)))]
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

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.Ipv6StaticRoutes", schema = List(730)))]
                    pub struct Ipv6StaticRoutes<'a, Mode> (::validated_data::Field<ipv6_static_routes::Item<'a, Mode>>);

                    pub mod ipv6_static_routes {

                        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.Ipv6StaticRoutesItem", schema = Dict(1663)))]
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

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.Ospf", schema = Dict(1680)))]
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
                        #[data_view(schema = Str(1551))]
                        pub simple_auth_key: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Str(1532))]
                        pub cleartext_simple_auth_key: ::validated_data::Field<&'a str>,
                        #[data_view(schema = List(724))]
                        pub message_digest_keys: ::validated_data::Field<ospf::MessageDigestKeys<'a, Mode>>,
                    }

                    pub mod ospf {

                        #[::validated_data::data_view(indexed_list, primary_key(id), python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.Ospf.MessageDigestKeys", schema = List(724), item_name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.Ospf.MessageDigestKeysItem"))]
                        pub struct MessageDigestKeys<'a, Mode> (::validated_data::Field<message_digest_keys::Item<'a, Mode>>);

                        pub mod message_digest_keys {

                            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.Ospf.MessageDigestKeysItemBase", schema = Dict(1675)))]
                            pub struct Item<'a, Mode> {
                                #[data_view(schema = Int(77))]
                                pub id: ::validated_data::Field<i64>,
                                #[data_view(schema = Str(1502))]
                                pub hash_algorithm: ::validated_data::Field<&'a str>,
                                #[data_view(schema = Str(1543))]
                                pub key: ::validated_data::Field<&'a str>,
                                #[data_view(schema = Str(1534))]
                                pub cleartext_key: ::validated_data::Field<&'a str>,
                            }
                        }
                    }

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.L3PortChannelsItem.FlowTracking", schema = Dict(1511)))]
                    pub struct FlowTracking<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub enabled: ::validated_data::Field<bool>,
                        #[data_view(schema = Str(1155))]
                        pub name: ::validated_data::Field<&'a str>,
                    }
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.Loopbacks", schema = List(732)))]
            pub struct Loopbacks<'a, Mode> (::validated_data::Field<loopbacks::Item<'a, Mode>>);

            pub mod loopbacks {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.LoopbacksItem", schema = Dict(1683)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(2))]
                    pub node: ::validated_data::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Int(614))]
                    pub loopback: ::validated_data::RequiredValue<i64, Mode>,
                    #[data_view(schema = Str(2))]
                    pub ip_address: ::validated_data::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Str(3))]
                    pub description: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Bool(37))]
                    pub enabled: ::validated_data::Field<bool>,
                    #[data_view(schema = Dict(1682))]
                    pub ospf: ::validated_data::Field<item::Ospf<'a, Mode>>,
                    #[data_view(schema = Bool(590))]
                    pub hardware_forwarding: ::validated_data::Field<bool>,
                    #[data_view(schema = Str(1553))]
                    pub raw_eos_cli: ::validated_data::Field<&'a str>,
                }

                pub mod item {

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.LoopbacksItem.Ospf", schema = Dict(1682)))]
                    pub struct Ospf<'a, Mode> {
                        #[data_view(schema = Bool(340))]
                        pub enabled: ::validated_data::Field<bool>,
                        #[data_view(schema = Str(1529))]
                        pub area: ::validated_data::Field<&'a str>,
                    }
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.StaticRoutes", schema = List(733)))]
            pub struct StaticRoutes<'a, Mode> (::validated_data::Field<static_routes::Item<'a, Mode>>);

            pub mod static_routes {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.StaticRoutesItem", schema = Dict(1684)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = List(11))]
                    pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
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

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.StaticRoutesItem.Nodes", schema = List(11)))]
                    pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.Ipv6StaticRoutes", schema = List(734)))]
            pub struct Ipv6StaticRoutes<'a, Mode> (::validated_data::Field<ipv6_static_routes::Item<'a, Mode>>);

            pub mod ipv6_static_routes {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.Ipv6StaticRoutesItem", schema = Dict(1685)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = List(11))]
                    pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
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

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.Ipv6StaticRoutesItem.Nodes", schema = List(11)))]
                    pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.StaticArpEntries", schema = List(736)))]
            pub struct StaticArpEntries<'a, Mode> (::validated_data::Field<static_arp_entries::Item<'a, Mode>>);

            pub mod static_arp_entries {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.StaticArpEntriesItem", schema = Dict(1686)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(58))]
                    pub ipv4_address: ::validated_data::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Str(60))]
                    pub mac_address: ::validated_data::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = List(735))]
                    pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
                }

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.StaticArpEntriesItem.Nodes", schema = List(735)))]
                    pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }

            #[::validated_data::data_view(list, primary_key(ip_address), python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeers", schema = List(738), item_name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeersItem"))]
            pub struct BgpPeers<'a, Mode> (::validated_data::Field<bgp_peers::Item<'a, Mode>>);

            pub mod bgp_peers {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeersItemBase", schema = Dict(1688)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(1556))]
                    pub ip_address: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(767))]
                    pub peer_group: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(758))]
                    pub remote_as: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(3))]
                    pub description: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1476))]
                    pub password: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1109))]
                    pub cleartext_password: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1557))]
                    pub send_community: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Bool(7))]
                    pub next_hop_self: ::validated_data::Field<bool>,
                    #[data_view(schema = Str(1558))]
                    pub timers: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Int(382))]
                    pub maximum_routes: ::validated_data::Field<i64>,
                    #[data_view(schema = Bool(7))]
                    pub maximum_routes_warning_only: ::validated_data::Field<bool>,
                    #[data_view(schema = Dict(1687))]
                    pub default_originate: ::validated_data::Field<item::DefaultOriginate<'a, Mode>>,
                    #[data_view(schema = Str(3))]
                    pub update_source: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Int(381))]
                    pub ebgp_multihop: ::validated_data::Field<i64>,
                    #[data_view(schema = List(737))]
                    pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
                    #[data_view(schema = Str(1559))]
                    pub set_ipv4_next_hop: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1560))]
                    pub set_ipv6_next_hop: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(169))]
                    pub route_map_out: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(169))]
                    pub route_map_in: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1561))]
                    pub prefix_list_in: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1562))]
                    pub prefix_list_out: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1563))]
                    pub local_as: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Int(110))]
                    pub weight: ::validated_data::Field<i64>,
                    #[data_view(schema = Bool(7))]
                    pub bfd: ::validated_data::Field<bool>,
                    #[data_view(schema = Dict(916))]
                    pub bfd_timers: ::validated_data::Field<super::super::super::super::super::super::eos_config::router_bgp::vrfs::item::neighbors::item::BfdTimers<'a, Mode>>,
                    #[data_view(schema = Bool(593))]
                    pub route_reflector_client: ::validated_data::Field<bool>,
                    #[data_view(schema = Bool(7))]
                    pub shutdown: ::validated_data::Field<bool>,
                }

                pub mod item {

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeersItemBase.DefaultOriginate", schema = Dict(1687)))]
                    pub struct DefaultOriginate<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub always: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeersItemBase.Nodes", schema = List(737)))]
                    pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.Bgp", schema = Dict(1691)))]
            pub struct Bgp<'a, Mode> {
                #[data_view(schema = Bool(594))]
                pub enabled: ::validated_data::Field<bool>,
                #[data_view(schema = Str(1564))]
                pub router_id: ::validated_data::Field<&'a str>,
                #[data_view(schema = Dict(1689))]
                pub graceful_restart: ::validated_data::Field<bgp::GracefulRestart<'a, Mode>>,
                #[data_view(schema = Str(1565))]
                pub raw_eos_cli: ::validated_data::Field<&'a str>,
                #[data_view(schema = Dict(1690), relaxed)]
                pub structured_config: ::validated_data::Field<super::super::super::super::super::eos_config::router_bgp::vrfs::Item<'a, ::validated_data::RelaxedValidated>>,
            }

            pub mod bgp {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.Bgp.GracefulRestart", schema = Dict(1689)))]
                pub struct GracefulRestart<'a, Mode> {
                    #[data_view(schema = Bool(595))]
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    #[data_view(schema = Int(499))]
                    pub restart_time: ::validated_data::Field<i64>,
                }
            }

            #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroups", schema = List(740), item_name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItem"))]
            pub struct BgpPeerGroups<'a, Mode> (::validated_data::Field<bgp_peer_groups::Item<'a, Mode>>);

            pub mod bgp_peer_groups {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase", schema = Dict(1692)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(1475))]
                    pub name: ::validated_data::Field<&'a str>,
                    #[data_view(schema = List(687))]
                    pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
                    #[data_view(schema = Str(1476))]
                    pub password: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1109))]
                    pub cleartext_password: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Dict(1634))]
                    pub address_family_ipv4: ::validated_data::Field<item::AddressFamilyIpv4<'a, Mode>>,
                    #[data_view(schema = Dict(1636))]
                    pub address_family_ipv6: ::validated_data::Field<item::AddressFamilyIpv6<'a, Mode>>,
                    #[data_view(schema = List(739))]
                    pub listen_ranges: ::validated_data::Field<item::ListenRanges<'a, Mode>>,
                    #[data_view(schema = Dict(912))]
                    pub metadata: ::validated_data::Field<item::Metadata<'a, Mode>>,
                    #[data_view(schema = Str(758))]
                    pub remote_as: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(758))]
                    pub local_as: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(3))]
                    pub description: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Bool(7))]
                    pub shutdown: ::validated_data::Field<bool>,
                    #[data_view(schema = Dict(913))]
                    pub as_path: ::validated_data::Field<item::AsPath<'a, Mode>>,
                    #[data_view(schema = Dict(914))]
                    pub remove_private_as: ::validated_data::Field<item::RemovePrivateAs<'a, Mode>>,
                    #[data_view(schema = Dict(915))]
                    pub remove_private_as_ingress: ::validated_data::Field<item::RemovePrivateAsIngress<'a, Mode>>,
                    #[data_view(schema = Bool(7))]
                    pub next_hop_unchanged: ::validated_data::Field<bool>,
                    #[data_view(schema = Str(771))]
                    pub update_source: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Bool(7))]
                    pub route_reflector_client: ::validated_data::Field<bool>,
                    #[data_view(schema = Bool(323))]
                    pub bfd: ::validated_data::Field<bool>,
                    #[data_view(schema = Dict(916))]
                    pub bfd_timers: ::validated_data::Field<item::BfdTimers<'a, Mode>>,
                    #[data_view(schema = Int(381))]
                    pub ebgp_multihop: ::validated_data::Field<i64>,
                    #[data_view(schema = Bool(7))]
                    pub next_hop_peer: ::validated_data::Field<bool>,
                    #[data_view(schema = Bool(7))]
                    pub next_hop_self: ::validated_data::Field<bool>,
                    #[data_view(schema = Str(772))]
                    pub password_type: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Bool(7))]
                    pub passive: ::validated_data::Field<bool>,
                    #[data_view(schema = Dict(917))]
                    pub default_originate: ::validated_data::Field<item::DefaultOriginate<'a, Mode>>,
                    #[data_view(schema = Bool(324))]
                    pub enforce_first_as: ::validated_data::Field<bool>,
                    #[data_view(schema = Str(773))]
                    pub send_community: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Int(382))]
                    pub maximum_routes: ::validated_data::Field<i64>,
                    #[data_view(schema = Str(774))]
                    pub maximum_routes_warning_limit: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Bool(7))]
                    pub maximum_routes_warning_only: ::validated_data::Field<bool>,
                    #[data_view(schema = Dict(919))]
                    pub maximum_accepted_routes: ::validated_data::Field<item::MaximumAcceptedRoutes<'a, Mode>>,
                    #[data_view(schema = Dict(922))]
                    pub missing_policy: ::validated_data::Field<item::MissingPolicy<'a, Mode>>,
                    #[data_view(schema = Dict(923))]
                    pub link_bandwidth: ::validated_data::Field<item::LinkBandwidth<'a, Mode>>,
                    #[data_view(schema = Dict(924))]
                    pub allowas_in: ::validated_data::Field<item::AllowasIn<'a, Mode>>,
                    #[data_view(schema = Int(110))]
                    pub weight: ::validated_data::Field<i64>,
                    #[data_view(schema = Str(777))]
                    pub timers: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Dict(925))]
                    pub rib_in_pre_policy_retain: ::validated_data::Field<item::RibInPrePolicyRetain<'a, Mode>>,
                    #[data_view(schema = Str(778))]
                    pub route_map_in: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(779))]
                    pub route_map_out: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(780))]
                    pub peer_tag_in: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(781))]
                    pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(3))]
                    pub session_tracker: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Dict(926))]
                    pub shared_secret: ::validated_data::Field<item::SharedSecret<'a, Mode>>,
                    #[data_view(schema = Int(387))]
                    pub ttl_maximum_hops: ::validated_data::Field<i64>,
                    #[data_view(schema = Int(388))]
                    pub maximum_advertised_routes: ::validated_data::Field<i64>,
                    #[data_view(schema = Str(784))]
                    pub maximum_advertised_routes_warning_limit: ::validated_data::Field<&'a str>,
                }

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.Nodes", schema = List(687)))]
                    pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.AddressFamilyIpv4", schema = Dict(1634)))]
                    pub struct AddressFamilyIpv4<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub activate: ::validated_data::Field<bool>,
                        #[data_view(schema = Str(778))]
                        pub route_map_in: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Str(779))]
                        pub route_map_out: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Str(803))]
                        pub rcf_in: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Str(804))]
                        pub rcf_out: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Dict(977))]
                        pub default_originate: ::validated_data::Field<super::super::super::super::super::super::super::eos_config::router_bgp::address_family_ipv4::peer_groups::item::DefaultOriginate<'a, Mode>>,
                        #[data_view(schema = Dict(980))]
                        pub next_hop: ::validated_data::Field<super::super::super::super::super::super::super::eos_config::router_bgp::address_family_ipv4::peer_groups::item::NextHop<'a, Mode>>,
                        #[data_view(schema = Str(812))]
                        pub prefix_list_in: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Str(813))]
                        pub prefix_list_out: ::validated_data::Field<&'a str>,
                    }

                    pub mod address_family_ipv4 {
                    }

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.AddressFamilyIpv6", schema = Dict(1636)))]
                    pub struct AddressFamilyIpv6<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub activate: ::validated_data::Field<bool>,
                        #[data_view(schema = Str(778))]
                        pub route_map_in: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Str(779))]
                        pub route_map_out: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Str(803))]
                        pub rcf_in: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Str(804))]
                        pub rcf_out: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Dict(1635))]
                        pub default_originate: ::validated_data::Field<address_family_ipv6::DefaultOriginate<'a, Mode>>,
                        #[data_view(schema = Str(812))]
                        pub prefix_list_in: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Str(813))]
                        pub prefix_list_out: ::validated_data::Field<&'a str>,
                    }

                    pub mod address_family_ipv6 {

                        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.AddressFamilyIpv6.DefaultOriginate", schema = Dict(1635)))]
                        pub struct DefaultOriginate<'a, Mode> {
                            #[data_view(schema = Bool(1))]
                            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                            #[data_view(schema = Bool(336))]
                            pub always: ::validated_data::Field<bool>,
                            #[data_view(schema = Str(169))]
                            pub route_map: ::validated_data::Field<&'a str>,
                        }
                    }

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.ListenRanges", schema = List(739)))]
                    pub struct ListenRanges<'a, Mode> (::validated_data::Field<listen_ranges::Item<'a, Mode>>);

                    pub mod listen_ranges {

                        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.ListenRangesItem", schema = Dict(1637)))]
                        pub struct Item<'a, Mode> {
                            #[data_view(schema = Str(1477))]
                            pub prefix: ::validated_data::RequiredValue<&'a str, Mode>,
                            #[data_view(schema = Str(1478))]
                            pub remote_as: ::validated_data::Field<&'a str>,
                            #[data_view(schema = Bool(320))]
                            pub peer_id_include_router_id: ::validated_data::Field<bool>,
                            #[data_view(schema = Str(1479))]
                            pub peer_filter: ::validated_data::Field<&'a str>,
                        }
                    }

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.Metadata", schema = Dict(912)))]
                    pub struct Metadata<'a, Mode> {
                        #[data_view(schema = Str(278), rename = "type")]
                        pub field_type: ::validated_data::Field<&'a str>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.AsPath", schema = Dict(913)))]
                    pub struct AsPath<'a, Mode> {
                        #[data_view(schema = Bool(321))]
                        pub remote_as_replace_out: ::validated_data::Field<bool>,
                        #[data_view(schema = Bool(322))]
                        pub prepend_own_disabled: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.RemovePrivateAs", schema = Dict(914)))]
                    pub struct RemovePrivateAs<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub enabled: ::validated_data::Field<bool>,
                        #[data_view(schema = Bool(7))]
                        pub all: ::validated_data::Field<bool>,
                        #[data_view(schema = Bool(7))]
                        pub replace_as: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.RemovePrivateAsIngress", schema = Dict(915)))]
                    pub struct RemovePrivateAsIngress<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub enabled: ::validated_data::Field<bool>,
                        #[data_view(schema = Bool(7))]
                        pub replace_as: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.BfdTimers", schema = Dict(916)))]
                    pub struct BfdTimers<'a, Mode> {
                        #[data_view(schema = Int(378))]
                        pub interval: ::validated_data::RequiredValue<i64, Mode>,
                        #[data_view(schema = Int(379))]
                        pub min_rx: ::validated_data::RequiredValue<i64, Mode>,
                        #[data_view(schema = Int(380))]
                        pub multiplier: ::validated_data::RequiredValue<i64, Mode>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.DefaultOriginate", schema = Dict(917)))]
                    pub struct DefaultOriginate<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub enabled: ::validated_data::Field<bool>,
                        #[data_view(schema = Bool(7))]
                        pub always: ::validated_data::Field<bool>,
                        #[data_view(schema = Str(169))]
                        pub route_map: ::validated_data::Field<&'a str>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.MaximumAcceptedRoutes", schema = Dict(919)))]
                    pub struct MaximumAcceptedRoutes<'a, Mode> {
                        #[data_view(schema = Int(383))]
                        pub limit: ::validated_data::RequiredValue<i64, Mode>,
                        #[data_view(schema = Dict(918))]
                        pub warning_limit: ::validated_data::Field<maximum_accepted_routes::WarningLimit<'a, Mode>>,
                    }

                    pub mod maximum_accepted_routes {

                        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.MaximumAcceptedRoutes.WarningLimit", schema = Dict(918)))]
                        pub struct WarningLimit<'a, Mode> {
                            #[data_view(schema = Int(384))]
                            pub count: ::validated_data::Field<i64>,
                            #[data_view(schema = Int(385))]
                            pub percent: ::validated_data::Field<i64>,
                        }
                    }

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.MissingPolicy", schema = Dict(922)))]
                    pub struct MissingPolicy<'a, Mode> {
                        #[data_view(schema = Dict(920))]
                        pub direction_in: ::validated_data::Field<missing_policy::DirectionIn<'a, Mode>>,
                        #[data_view(schema = Dict(921))]
                        pub direction_out: ::validated_data::Field<missing_policy::DirectionOut<'a, Mode>>,
                    }

                    pub mod missing_policy {

                        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.MissingPolicy.DirectionIn", schema = Dict(920)))]
                        pub struct DirectionIn<'a, Mode> {
                            #[data_view(schema = Str(775))]
                            pub action: ::validated_data::RequiredValue<&'a str, Mode>,
                            #[data_view(schema = Bool(325))]
                            pub include_community_list: ::validated_data::Field<bool>,
                            #[data_view(schema = Bool(326))]
                            pub include_prefix_list: ::validated_data::Field<bool>,
                            #[data_view(schema = Bool(327))]
                            pub include_sub_route_map: ::validated_data::Field<bool>,
                        }

                        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.MissingPolicy.DirectionOut", schema = Dict(921)))]
                        pub struct DirectionOut<'a, Mode> {
                            #[data_view(schema = Str(775))]
                            pub action: ::validated_data::RequiredValue<&'a str, Mode>,
                            #[data_view(schema = Bool(325))]
                            pub include_community_list: ::validated_data::Field<bool>,
                            #[data_view(schema = Bool(326))]
                            pub include_prefix_list: ::validated_data::Field<bool>,
                            #[data_view(schema = Bool(327))]
                            pub include_sub_route_map: ::validated_data::Field<bool>,
                        }
                    }

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.LinkBandwidth", schema = Dict(923)))]
                    pub struct LinkBandwidth<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub enabled: ::validated_data::Field<bool>,
                        #[data_view(schema = Str(776))]
                        pub default: ::validated_data::Field<&'a str>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.AllowasIn", schema = Dict(924)))]
                    pub struct AllowasIn<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub enabled: ::validated_data::Field<bool>,
                        #[data_view(schema = Int(386))]
                        pub times: ::validated_data::Field<i64>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.RibInPrePolicyRetain", schema = Dict(925)))]
                    pub struct RibInPrePolicyRetain<'a, Mode> {
                        #[data_view(schema = Bool(7))]
                        pub enabled: ::validated_data::Field<bool>,
                        #[data_view(schema = Bool(7))]
                        pub all: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.BgpPeerGroupsItemBase.SharedSecret", schema = Dict(926)))]
                    pub struct SharedSecret<'a, Mode> {
                        #[data_view(schema = Str(782))]
                        pub profile: ::validated_data::RequiredValue<&'a str, Mode>,
                        #[data_view(schema = Str(783))]
                        pub hash_algorithm: ::validated_data::RequiredValue<&'a str, Mode>,
                    }
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.AdditionalRouteTargets", schema = List(742)))]
            pub struct AdditionalRouteTargets<'a, Mode> (::validated_data::Field<additional_route_targets::Item<'a, Mode>>);

            pub mod additional_route_targets {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.AdditionalRouteTargetsItem", schema = Dict(1693)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(1566), rename = "type")]
                    pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Str(2))]
                    pub address_family: ::validated_data::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Str(2))]
                    pub route_target: ::validated_data::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = List(741))]
                    pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
                }

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.AdditionalRouteTargetsItem.Nodes", schema = List(741)))]
                    pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.AggregateAddresses", schema = List(744)))]
            pub struct AggregateAddresses<'a, Mode> (::validated_data::Field<aggregate_addresses::Item<'a, Mode>>);

            pub mod aggregate_addresses {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.AggregateAddressesItem", schema = Dict(1694)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = List(743))]
                    pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
                    #[data_view(schema = Str(766))]
                    pub prefix: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Bool(7))]
                    pub advertise_only: ::validated_data::Field<bool>,
                    #[data_view(schema = Bool(7))]
                    pub as_set: ::validated_data::Field<bool>,
                    #[data_view(schema = Bool(7))]
                    pub summary_only: ::validated_data::Field<bool>,
                    #[data_view(schema = Str(169))]
                    pub attribute_map: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(169))]
                    pub match_map: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Dict(932))]
                    pub attribute: ::validated_data::Field<item::Attribute<'a, Mode>>,
                }

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.AggregateAddressesItem.Nodes", schema = List(743)))]
                    pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.VrfsItemBase.AggregateAddressesItem.Attribute", schema = Dict(932)))]
                    pub struct Attribute<'a, Mode> {
                        #[data_view(schema = Str(788))]
                        pub rcf: ::validated_data::Field<&'a str>,
                    }
                }
            }
        }
    }

    #[::validated_data::data_view(list, primary_key(id), python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.L2vlans", schema = List(747), item_name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.L2vlansItem"))]
    pub struct L2vlans<'a, Mode> (::validated_data::Field<l2vlans::Item<'a, Mode>>);

    pub mod l2vlans {

        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.L2vlansItemBase", schema = Dict(1697)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Int(476))]
            pub id: ::validated_data::Field<i64>,
            #[data_view(schema = Str(1506))]
            pub name: ::validated_data::RequiredValue<&'a str, Mode>,
            #[data_view(schema = Str(1569))]
            pub profile: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(746))]
            pub tags: ::validated_data::Field<item::Tags<'a, Mode>>,
            #[data_view(schema = Dict(1340))]
            pub address_locking: ::validated_data::Field<super::super::super::super::eos_config::vlans::item::address_locking::AddressFamily<'a, Mode>>,
            #[data_view(schema = Int(593))]
            pub vni_override: ::validated_data::Field<i64>,
            #[data_view(schema = Str(1432))]
            pub rt_override: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1433))]
            pub rd_override: ::validated_data::Field<&'a str>,
            #[data_view(schema = Bool(543))]
            pub vxlan: ::validated_data::Field<bool>,
            #[data_view(schema = Int(594))]
            pub spanning_tree_priority: ::validated_data::Field<i64>,
            #[data_view(schema = Str(1434))]
            pub evpn_vlan_bundle: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(667))]
            pub trunk_groups: ::validated_data::Field<item::TrunkGroups<'a, Mode>>,
            #[data_view(schema = Bool(544))]
            pub evpn_l2_multi_domain: ::validated_data::Field<bool>,
            #[data_view(schema = Dict(1605))]
            pub evpn_l2_multicast: ::validated_data::Field<item::EvpnL2Multicast<'a, Mode>>,
            #[data_view(schema = Dict(1606))]
            pub vxlan_flood_multicast: ::validated_data::Field<item::VxlanFloodMulticast<'a, Mode>>,
            #[data_view(schema = Dict(1608))]
            pub igmp_snooping: ::validated_data::Field<item::IgmpSnooping<'a, Mode>>,
            #[data_view(schema = Bool(549))]
            pub igmp_snooping_enabled: ::validated_data::Field<bool>,
            #[data_view(schema = Dict(1609))]
            pub igmp_snooping_querier: ::validated_data::Field<item::IgmpSnoopingQuerier<'a, Mode>>,
            #[data_view(schema = Dict(1611))]
            pub bgp: ::validated_data::Field<item::Bgp<'a, Mode>>,
            #[data_view(schema = Dict(1612))]
            pub private_vlan: ::validated_data::Field<item::PrivateVlan<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.L2vlansItemBase.Tags", schema = List(746)))]
            pub struct Tags<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.L2vlansItemBase.TrunkGroups", schema = List(667)))]
            pub struct TrunkGroups<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.L2vlansItemBase.EvpnL2Multicast", schema = Dict(1605)))]
            pub struct EvpnL2Multicast<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.L2vlansItemBase.VxlanFloodMulticast", schema = Dict(1606)))]
            pub struct VxlanFloodMulticast<'a, Mode> {
                #[data_view(schema = Bool(545))]
                pub enabled: ::validated_data::Field<bool>,
                #[data_view(schema = Str(1436))]
                pub underlay_multicast_group: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.L2vlansItemBase.IgmpSnooping", schema = Dict(1608)))]
            pub struct IgmpSnooping<'a, Mode> {
                #[data_view(schema = Bool(546))]
                pub enabled: ::validated_data::Field<bool>,
                #[data_view(schema = Dict(1607))]
                pub querier: ::validated_data::Field<igmp_snooping::Querier<'a, Mode>>,
                #[data_view(schema = Bool(548))]
                pub fast_leave: ::validated_data::Field<bool>,
            }

            pub mod igmp_snooping {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.L2vlansItemBase.IgmpSnooping.Querier", schema = Dict(1607)))]
                pub struct Querier<'a, Mode> {
                    #[data_view(schema = Bool(547))]
                    pub enabled: ::validated_data::Field<bool>,
                    #[data_view(schema = Str(1437))]
                    pub source_address: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Int(595))]
                    pub version: ::validated_data::Field<i64>,
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.L2vlansItemBase.IgmpSnoopingQuerier", schema = Dict(1609)))]
            pub struct IgmpSnoopingQuerier<'a, Mode> {
                #[data_view(schema = Bool(550))]
                pub enabled: ::validated_data::Field<bool>,
                #[data_view(schema = Str(1438))]
                pub source_address: ::validated_data::Field<&'a str>,
                #[data_view(schema = Int(595))]
                pub version: ::validated_data::Field<i64>,
                #[data_view(schema = Bool(548))]
                pub fast_leave: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.L2vlansItemBase.Bgp", schema = Dict(1611)))]
            pub struct Bgp<'a, Mode> {
                #[data_view(schema = Dict(1610), relaxed)]
                pub structured_config: ::validated_data::Field<super::super::super::super::super::eos_config::router_bgp::vlans::Item<'a, ::validated_data::RelaxedValidated>>,
                #[data_view(schema = Str(1439))]
                pub raw_eos_cli: ::validated_data::Field<&'a str>,
            }

            pub mod bgp {
            }

            #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.L2vlansItemBase.PrivateVlan", schema = Dict(1612)))]
            pub struct PrivateVlan<'a, Mode> {
                #[data_view(schema = Str(1440), rename = "type")]
                pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
                #[data_view(schema = Int(596))]
                pub primary_vlan: ::validated_data::RequiredValue<i64, Mode>,
            }
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.Vpws", schema = Dict(1698)))]
    pub struct Vpws<'a, Mode> {
        #[data_view(schema = Bool(597))]
        pub mpls_control_word: ::validated_data::Field<bool>,
        #[data_view(schema = Int(615))]
        pub mtu: ::validated_data::Field<i64>,
        #[data_view(schema = Bool(598))]
        pub label_flow: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.PointToPointServices", schema = List(752), item_name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.PointToPointServicesItem"))]
    pub struct PointToPointServices<'a, Mode> (::validated_data::Field<point_to_point_services::Item<'a, Mode>>);

    pub mod point_to_point_services {

        #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.PointToPointServicesItemBase", schema = Dict(1705)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(798))]
            pub name: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1571), rename = "type")]
            pub field_type: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(748))]
            pub subinterfaces: ::validated_data::Field<item::Subinterfaces<'a, Mode>>,
            #[data_view(schema = List(751))]
            pub endpoints: ::validated_data::Field<item::Endpoints<'a, Mode>>,
            #[data_view(schema = Bool(599))]
            pub lldp_disable: ::validated_data::Field<bool>,
        }

        pub mod item {

            #[::validated_data::data_view(indexed_list, primary_key(number), python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.PointToPointServicesItemBase.Subinterfaces", schema = List(748), item_name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.PointToPointServicesItemBase.SubinterfacesItem"))]
            pub struct Subinterfaces<'a, Mode> (::validated_data::Field<subinterfaces::Item<'a, Mode>>);

            pub mod subinterfaces {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.PointToPointServicesItemBase.SubinterfacesItemBase", schema = Dict(1702)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Int(616))]
                    pub number: ::validated_data::Field<i64>,
                    #[data_view(schema = Dict(1700))]
                    pub port_channel: ::validated_data::Field<item::PortChannel<'a, Mode>>,
                    #[data_view(schema = Dict(1701), relaxed)]
                    pub structured_config: ::validated_data::Field<super::super::super::super::super::super::eos_config::ethernet_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
                    #[data_view(schema = Str(1573))]
                    pub raw_eos_cli: ::validated_data::Field<&'a str>,
                }

                pub mod item {

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.PointToPointServicesItemBase.SubinterfacesItemBase.PortChannel", schema = Dict(1700)))]
                    pub struct PortChannel<'a, Mode> {
                        #[data_view(schema = Dict(1699), relaxed)]
                        pub structured_config: ::validated_data::Field<super::super::super::super::super::super::super::eos_config::port_channel_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
                        #[data_view(schema = Str(1572))]
                        pub raw_eos_cli: ::validated_data::Field<&'a str>,
                    }

                    pub mod port_channel {
                    }
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.PointToPointServicesItemBase.Endpoints", schema = List(751)))]
            pub struct Endpoints<'a, Mode> (::validated_data::Field<endpoints::Item<'a, Mode>>);

            pub mod endpoints {

                #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.PointToPointServicesItemBase.EndpointsItem", schema = Dict(1704)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Int(617))]
                    pub id: ::validated_data::RequiredValue<i64, Mode>,
                    #[data_view(schema = List(749))]
                    pub nodes: ::validated_data::RequiredValue<item::Nodes<'a, Mode>, Mode>,
                    #[data_view(schema = List(750))]
                    pub interfaces: ::validated_data::RequiredValue<item::Interfaces<'a, Mode>, Mode>,
                    #[data_view(schema = Dict(1703))]
                    pub port_channel: ::validated_data::Field<item::PortChannel<'a, Mode>>,
                }

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.PointToPointServicesItemBase.EndpointsItem.Nodes", schema = List(749)))]
                    pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list, python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.PointToPointServicesItemBase.EndpointsItem.Interfaces", schema = List(750)))]
                    pub struct Interfaces<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NetworkServicesItemBase.PointToPointServicesItemBase.EndpointsItem.PortChannel", schema = Dict(1703)))]
                    pub struct PortChannel<'a, Mode> {
                        #[data_view(schema = Str(1574))]
                        pub mode: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Str(3))]
                        pub short_esi: ::validated_data::Field<&'a str>,
                    }
                }
            }
        }
    }
}
