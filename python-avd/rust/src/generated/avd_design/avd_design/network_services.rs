// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItems"))]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub mac_vrf_vni_base: ::validated_data::Field<i64>,
    pub mac_vrf_id_base: ::validated_data::Field<i64>,
    pub vlan_aware_bundle_number_base: ::validated_data::Field<i64>,
    pub pseudowire_rt_base: ::validated_data::Field<i64>,
    pub enable_mlag_ibgp_peering_vrfs: ::validated_data::Field<bool>,
    pub redistribute_mlag_ibgp_peering_vrfs: ::validated_data::Field<bool>,
    pub evpn_vlan_bundle: ::validated_data::Field<&'a str>,
    pub bgp_peer_groups: ::validated_data::Field<item::BgpPeerGroups<'a, Mode>>,
    pub igmp_snooping: ::validated_data::Field<item::IgmpSnooping<'a, Mode>>,
    pub evpn_l2_multicast: ::validated_data::Field<item::EvpnL2Multicast<'a, Mode>>,
    pub vxlan_flood_multicast: ::validated_data::Field<item::VxlanFloodMulticast<'a, Mode>>,
    pub evpn_l3_multicast: ::validated_data::Field<item::EvpnL3Multicast<'a, Mode>>,
    pub pim_rp_addresses: ::validated_data::Field<item::PimRpAddresses<'a, Mode>>,
    pub igmp_snooping_querier: ::validated_data::Field<item::IgmpSnoopingQuerier<'a, Mode>>,
    pub evpn_l2_multi_domain: ::validated_data::Field<bool>,
    pub vrfs: ::validated_data::Field<item::Vrfs<'a, Mode>>,
    pub l2vlans: ::validated_data::Field<item::L2vlans<'a, Mode>>,
    pub vpws: ::validated_data::Field<item::Vpws<'a, Mode>>,
    pub point_to_point_services: ::validated_data::Field<item::PointToPointServices<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesignNetworkServicesItemsBgpPeerGroupsList", item_name = "AVDDesignNetworkServicesItemsBgpPeerGroupsListIndexedItem"))]
    pub struct BgpPeerGroups<'a, Mode> (::validated_data::Field<bgp_peer_groups::Item<'a, Mode>>);

    pub mod bgp_peer_groups {

        #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsBgpPeerGroupsItems"))]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub password: ::validated_data::Field<&'a str>,
            pub cleartext_password: ::validated_data::Field<&'a str>,
            pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
            pub address_family_ipv4: ::validated_data::Field<item::AddressFamilyIpv4<'a, Mode>>,
            pub address_family_ipv6: ::validated_data::Field<item::AddressFamilyIpv6<'a, Mode>>,
            pub listen_ranges: ::validated_data::Field<item::ListenRanges<'a, Mode>>,
            pub metadata: ::validated_data::Field<item::Metadata<'a, Mode>>,
            pub remote_as: ::validated_data::Field<&'a str>,
            pub local_as: ::validated_data::Field<&'a str>,
            pub description: ::validated_data::Field<&'a str>,
            pub shutdown: ::validated_data::Field<bool>,
            pub as_path: ::validated_data::Field<item::AsPath<'a, Mode>>,
            pub remove_private_as: ::validated_data::Field<item::RemovePrivateAs<'a, Mode>>,
            pub remove_private_as_ingress: ::validated_data::Field<item::RemovePrivateAsIngress<'a, Mode>>,
            pub next_hop_unchanged: ::validated_data::Field<bool>,
            pub update_source: ::validated_data::Field<&'a str>,
            pub route_reflector_client: ::validated_data::Field<bool>,
            pub bfd: ::validated_data::Field<bool>,
            pub bfd_timers: ::validated_data::Field<item::BfdTimers<'a, Mode>>,
            pub ebgp_multihop: ::validated_data::Field<i64>,
            pub next_hop_peer: ::validated_data::Field<bool>,
            pub next_hop_self: ::validated_data::Field<bool>,
            pub password_type: ::validated_data::Field<&'a str>,
            pub passive: ::validated_data::Field<bool>,
            pub default_originate: ::validated_data::Field<item::DefaultOriginate<'a, Mode>>,
            pub enforce_first_as: ::validated_data::Field<bool>,
            pub send_community: ::validated_data::Field<&'a str>,
            pub maximum_routes: ::validated_data::Field<i64>,
            pub maximum_routes_warning_limit: ::validated_data::Field<&'a str>,
            pub maximum_routes_warning_only: ::validated_data::Field<bool>,
            pub maximum_accepted_routes: ::validated_data::Field<item::MaximumAcceptedRoutes<'a, Mode>>,
            pub missing_policy: ::validated_data::Field<item::MissingPolicy<'a, Mode>>,
            pub link_bandwidth: ::validated_data::Field<item::LinkBandwidth<'a, Mode>>,
            pub allowas_in: ::validated_data::Field<item::AllowasIn<'a, Mode>>,
            pub weight: ::validated_data::Field<i64>,
            pub timers: ::validated_data::Field<&'a str>,
            pub rib_in_pre_policy_retain: ::validated_data::Field<item::RibInPrePolicyRetain<'a, Mode>>,
            pub route_map_in: ::validated_data::Field<&'a str>,
            pub route_map_out: ::validated_data::Field<&'a str>,
            pub peer_tag_in: ::validated_data::Field<&'a str>,
            pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
            pub session_tracker: ::validated_data::Field<&'a str>,
            pub shared_secret: ::validated_data::Field<item::SharedSecret<'a, Mode>>,
            pub ttl_maximum_hops: ::validated_data::Field<i64>,
            pub maximum_advertised_routes: ::validated_data::Field<i64>,
            pub maximum_advertised_routes_warning_limit: ::validated_data::Field<&'a str>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsNodesList"))]
            pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsAddressFamilyIpv4"))]
            pub struct AddressFamilyIpv4<'a, Mode> {
                pub activate: ::validated_data::Field<bool>,
                pub route_map_in: ::validated_data::Field<&'a str>,
                pub route_map_out: ::validated_data::Field<&'a str>,
                pub rcf_in: ::validated_data::Field<&'a str>,
                pub rcf_out: ::validated_data::Field<&'a str>,
                pub default_originate: ::validated_data::Field<super::super::super::super::super::eos_cli_config_gen::router_bgp::address_family_ipv4::peer_groups::item::DefaultOriginate<'a, Mode>>,
                pub next_hop: ::validated_data::Field<super::super::super::super::super::eos_cli_config_gen::router_bgp::address_family_ipv4::peer_groups::item::NextHop<'a, Mode>>,
                pub prefix_list_in: ::validated_data::Field<&'a str>,
                pub prefix_list_out: ::validated_data::Field<&'a str>,
            }

            pub mod address_family_ipv4 {
            }

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsAddressFamilyIpv6"))]
            pub struct AddressFamilyIpv6<'a, Mode> {
                pub activate: ::validated_data::Field<bool>,
                pub route_map_in: ::validated_data::Field<&'a str>,
                pub route_map_out: ::validated_data::Field<&'a str>,
                pub rcf_in: ::validated_data::Field<&'a str>,
                pub rcf_out: ::validated_data::Field<&'a str>,
                pub default_originate: ::validated_data::Field<address_family_ipv6::DefaultOriginate<'a, Mode>>,
                pub prefix_list_in: ::validated_data::Field<&'a str>,
                pub prefix_list_out: ::validated_data::Field<&'a str>,
            }

            pub mod address_family_ipv6 {

                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsAddressFamilyIpv6DefaultOriginate"))]
                pub struct DefaultOriginate<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub always: ::validated_data::Field<bool>,
                    pub route_map: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsListenRangesList"))]
            pub struct ListenRanges<'a, Mode> (::validated_data::Field<listen_ranges::Item<'a, Mode>>);

            pub mod listen_ranges {

                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsListenRangesItems"))]
                pub struct Item<'a, Mode> {
                    pub prefix: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub remote_as: ::validated_data::Field<&'a str>,
                    pub peer_id_include_router_id: ::validated_data::Field<bool>,
                    pub peer_filter: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsMetadata"))]
            pub struct Metadata<'a, Mode> {
                #[data_view(rename = "type")]
                pub field_type: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsAsPath"))]
            pub struct AsPath<'a, Mode> {
                pub remote_as_replace_out: ::validated_data::Field<bool>,
                pub prepend_own_disabled: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsRemovePrivateAs"))]
            pub struct RemovePrivateAs<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub all: ::validated_data::Field<bool>,
                pub replace_as: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsRemovePrivateAsIngress"))]
            pub struct RemovePrivateAsIngress<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub replace_as: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsBfdTimers"))]
            pub struct BfdTimers<'a, Mode> {
                pub interval: ::validated_data::RequiredValue<i64, Mode>,
                pub min_rx: ::validated_data::RequiredValue<i64, Mode>,
                pub multiplier: ::validated_data::RequiredValue<i64, Mode>,
            }

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsDefaultOriginate"))]
            pub struct DefaultOriginate<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub always: ::validated_data::Field<bool>,
                pub route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsMaximumAcceptedRoutes"))]
            pub struct MaximumAcceptedRoutes<'a, Mode> {
                pub limit: ::validated_data::RequiredValue<i64, Mode>,
                pub warning_limit: ::validated_data::Field<maximum_accepted_routes::WarningLimit<'a, Mode>>,
            }

            pub mod maximum_accepted_routes {

                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsMaximumAcceptedRoutesWarningLimit"))]
                pub struct WarningLimit<'a, Mode> {
                    pub count: ::validated_data::Field<i64>,
                    pub percent: ::validated_data::Field<i64>,
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsMissingPolicy"))]
            pub struct MissingPolicy<'a, Mode> {
                pub direction_in: ::validated_data::Field<missing_policy::DirectionIn<'a, Mode>>,
                pub direction_out: ::validated_data::Field<missing_policy::DirectionOut<'a, Mode>>,
            }

            pub mod missing_policy {

                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsMissingPolicyDirectionIn"))]
                pub struct DirectionIn<'a, Mode> {
                    pub action: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub include_community_list: ::validated_data::Field<bool>,
                    pub include_prefix_list: ::validated_data::Field<bool>,
                    pub include_sub_route_map: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsMissingPolicyDirectionOut"))]
                pub struct DirectionOut<'a, Mode> {
                    pub action: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub include_community_list: ::validated_data::Field<bool>,
                    pub include_prefix_list: ::validated_data::Field<bool>,
                    pub include_sub_route_map: ::validated_data::Field<bool>,
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsLinkBandwidth"))]
            pub struct LinkBandwidth<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub default: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsAllowasIn"))]
            pub struct AllowasIn<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub times: ::validated_data::Field<i64>,
            }

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsRibInPrePolicyRetain"))]
            pub struct RibInPrePolicyRetain<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub all: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsSharedSecret"))]
            pub struct SharedSecret<'a, Mode> {
                pub profile: ::validated_data::RequiredValue<&'a str, Mode>,
                pub hash_algorithm: ::validated_data::RequiredValue<&'a str, Mode>,
            }
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsIgmpSnooping"))]
    pub struct IgmpSnooping<'a, Mode> {
        pub querier: ::validated_data::Field<igmp_snooping::Querier<'a, Mode>>,
        pub fast_leave: ::validated_data::Field<bool>,
    }

    pub mod igmp_snooping {

        #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsIgmpSnoopingQuerier"))]
        pub struct Querier<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub source_address: ::validated_data::Field<&'a str>,
            pub version: ::validated_data::Field<i64>,
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsEvpnL2Multicast"))]
    pub struct EvpnL2Multicast<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub underlay_l2_multicast_group_ipv4_pool: ::validated_data::Field<&'a str>,
        pub underlay_l2_multicast_group_ipv4_pool_offset: ::validated_data::Field<i64>,
        pub fast_leave: ::validated_data::Field<bool>,
        pub always_redistribute_igmp: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVxlanFloodMulticast"))]
    pub struct VxlanFloodMulticast<'a, Mode> {
        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
        pub underlay_l2_multicast_group_ipv4_pool: ::validated_data::Field<&'a str>,
        pub underlay_l2_multicast_group_ipv4_pool_offset: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsEvpnL3Multicast"))]
    pub struct EvpnL3Multicast<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub evpn_underlay_l3_multicast_group_ipv4_pool: ::validated_data::RequiredValue<&'a str, Mode>,
        pub evpn_underlay_l3_multicast_group_ipv4_pool_offset: ::validated_data::Field<i64>,
        pub evpn_peg: ::validated_data::Field<evpn_l3_multicast::EvpnPeg<'a, Mode>>,
    }

    pub mod evpn_l3_multicast {

        #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsEvpnL3MulticastEvpnPegList"))]
        pub struct EvpnPeg<'a, Mode> (::validated_data::Field<evpn_peg::Item<'a, Mode>>);

        pub mod evpn_peg {

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsEvpnL3MulticastEvpnPegItems"))]
            pub struct Item<'a, Mode> {
                pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
                pub transit: ::validated_data::Field<bool>,
            }

            pub mod item {

                #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsEvpnL3MulticastEvpnPegItemsNodesList"))]
                pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);
            }
        }
    }

    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsPimRpAddressesList"))]
    pub struct PimRpAddresses<'a, Mode> (::validated_data::Field<pim_rp_addresses::Item<'a, Mode>>);

    pub mod pim_rp_addresses {

        #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsPimRpAddressesItems"))]
        pub struct Item<'a, Mode> {
            pub rps: ::validated_data::Field<item::Rps<'a, Mode>>,
            pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
            pub groups: ::validated_data::Field<item::Groups<'a, Mode>>,
            pub access_list_name: ::validated_data::Field<&'a str>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsPimRpAddressesItemsRpsList"))]
            pub struct Rps<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsPimRpAddressesItemsNodesList"))]
            pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsPimRpAddressesItemsGroupsList"))]
            pub struct Groups<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsIgmpSnoopingQuerier2"))]
    pub struct IgmpSnoopingQuerier<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub source_address: ::validated_data::Field<&'a str>,
        pub version: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesignNetworkServicesItemsVrfsList", item_name = "AVDDesignNetworkServicesItemsVrfsListIndexedItem"))]
    pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

    pub mod vrfs {

        #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItems"))]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub address_families: ::validated_data::Field<item::AddressFamilies<'a, Mode>>,
            pub description: ::validated_data::Field<&'a str>,
            pub vrf_vni: ::validated_data::Field<i64>,
            pub vrf_id: ::validated_data::Field<i64>,
            pub rd_override: ::validated_data::Field<&'a str>,
            pub rt_override: ::validated_data::Field<&'a str>,
            pub rt_import: ::validated_data::Field<bool>,
            pub rt_export: ::validated_data::Field<bool>,
            pub rt_import_evpn_remote: ::validated_data::Field<bool>,
            pub rt_export_evpn_remote: ::validated_data::Field<bool>,
            pub evpn_vlan_bundle: ::validated_data::Field<&'a str>,
            pub mlag_ibgp_peering_ipv4_pool: ::validated_data::Field<&'a str>,
            pub mlag_ibgp_peering_ipv6_pool: ::validated_data::Field<&'a str>,
            pub ip_helpers: ::validated_data::Field<item::IpHelpers<'a, Mode>>,
            pub enable_mlag_ibgp_peering_vrfs: ::validated_data::Field<bool>,
            pub redistribute_mlag_ibgp_peering_vrfs: ::validated_data::Field<bool>,
            pub mlag_ibgp_peering_vlan: ::validated_data::Field<i64>,
            pub vtep_diagnostic: ::validated_data::Field<item::VtepDiagnostic<'a, Mode>>,
            pub ospf: ::validated_data::Field<item::Ospf<'a, Mode>>,
            pub redistribute_ospf: ::validated_data::Field<bool>,
            pub evpn_l3_multicast: ::validated_data::Field<item::EvpnL3Multicast<'a, Mode>>,
            pub pim_rp_addresses: ::validated_data::Field<item::PimRpAddresses<'a, Mode>>,
            pub evpn_l2_multi_domain: ::validated_data::Field<bool>,
            pub svis: ::validated_data::Field<item::Svis<'a, Mode>>,
            pub l3_interfaces: ::validated_data::Field<item::L3Interfaces<'a, Mode>>,
            pub l3_port_channels: ::validated_data::Field<item::L3PortChannels<'a, Mode>>,
            pub loopbacks: ::validated_data::Field<item::Loopbacks<'a, Mode>>,
            pub static_routes: ::validated_data::Field<item::StaticRoutes<'a, Mode>>,
            pub ipv6_static_routes: ::validated_data::Field<item::Ipv6StaticRoutes<'a, Mode>>,
            pub redistribute_static: ::validated_data::Field<bool>,
            pub redistribute_connected: ::validated_data::Field<bool>,
            pub static_arp_entries: ::validated_data::Field<item::StaticArpEntries<'a, Mode>>,
            pub bgp_peers: ::validated_data::Field<item::BgpPeers<'a, Mode>>,
            pub bgp: ::validated_data::Field<item::Bgp<'a, Mode>>,
            pub bgp_peer_groups: ::validated_data::Field<item::BgpPeerGroups<'a, Mode>>,
            pub additional_route_targets: ::validated_data::Field<item::AdditionalRouteTargets<'a, Mode>>,
            pub aggregate_addresses: ::validated_data::Field<item::AggregateAddresses<'a, Mode>>,
            pub validate_bgp_peers: ::validated_data::Field<bool>,
            pub raw_eos_cli: ::validated_data::Field<&'a str>,
            #[data_view(relaxed)]
            pub structured_config: ::validated_data::Field<super::super::super::super::eos_cli_config_gen::EosCliConfigGen<'a, ::validated_data::RelaxedValidated>>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsAddressFamiliesList"))]
            pub struct AddressFamilies<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(indexed_list, primary_key(ip_helper), python(name = "AVDDesignNetworkServicesItemsVrfsItemsIpHelpersList", item_name = "AVDDesignNetworkServicesItemsVrfsItemsIpHelpersListIndexedItem"))]
            pub struct IpHelpers<'a, Mode> (::validated_data::Field<ip_helpers::Item<'a, Mode>>);

            pub mod ip_helpers {

                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsIpHelpersItems"))]
                pub struct Item<'a, Mode> {
                    pub ip_helper: ::validated_data::Field<&'a str>,
                    pub source_interface: ::validated_data::Field<&'a str>,
                    pub source_vrf: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsVtepDiagnostic"))]
            pub struct VtepDiagnostic<'a, Mode> {
                pub loopback: ::validated_data::Field<i64>,
                pub loopback_description: ::validated_data::Field<&'a str>,
                pub loopback_ip_range: ::validated_data::Field<&'a str>,
                pub loopback_ipv6_range: ::validated_data::Field<&'a str>,
                pub loopback_ip_pools: ::validated_data::Field<vtep_diagnostic::LoopbackIpPools<'a, Mode>>,
                pub hardware_forwarding: ::validated_data::Field<bool>,
            }

            pub mod vtep_diagnostic {

                #[::validated_data::data_view(indexed_list, primary_key(pod), python(name = "AVDDesignNetworkServicesItemsVrfsItemsVtepDiagnosticLoopbackIpPoolsList", item_name = "AVDDesignNetworkServicesItemsVrfsItemsVtepDiagnosticLoopbackIpPoolsListIndexedItem"))]
                pub struct LoopbackIpPools<'a, Mode> (::validated_data::Field<loopback_ip_pools::Item<'a, Mode>>);

                pub mod loopback_ip_pools {

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsVtepDiagnosticLoopbackIpPoolsItems"))]
                    pub struct Item<'a, Mode> {
                        pub pod: ::validated_data::Field<&'a str>,
                        pub ipv4_pool: ::validated_data::Field<&'a str>,
                        pub ipv6_pool: ::validated_data::Field<&'a str>,
                    }
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsOspf"))]
            pub struct Ospf<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub process_id: ::validated_data::Field<i64>,
                pub router_id: ::validated_data::Field<&'a str>,
                pub max_lsa: ::validated_data::Field<i64>,
                pub bfd: ::validated_data::Field<bool>,
                pub redistribute_bgp: ::validated_data::Field<ospf::RedistributeBgp<'a, Mode>>,
                pub redistribute_connected: ::validated_data::Field<ospf::RedistributeConnected<'a, Mode>>,
                pub authentication: ::validated_data::Field<&'a str>,
                pub cleartext_simple_auth_key: ::validated_data::Field<&'a str>,
                pub message_digest_keys: ::validated_data::Field<ospf::MessageDigestKeys<'a, Mode>>,
                pub nodes: ::validated_data::Field<ospf::Nodes<'a, Mode>>,
                #[data_view(relaxed)]
                pub structured_config: ::validated_data::Field<super::super::super::super::super::eos_cli_config_gen::router_ospf::process_ids::Item<'a, ::validated_data::RelaxedValidated>>,
            }

            pub mod ospf {

                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsOspfRedistributeBgp"))]
                pub struct RedistributeBgp<'a, Mode> {
                    pub enabled: ::validated_data::Field<bool>,
                    pub route_map: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsOspfRedistributeConnected"))]
                pub struct RedistributeConnected<'a, Mode> {
                    pub enabled: ::validated_data::Field<bool>,
                    pub route_map: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view(indexed_list, primary_key(id), python(name = "AVDDesignNetworkServicesItemsVrfsItemsOspfMessageDigestKeysList", item_name = "AVDDesignNetworkServicesItemsVrfsItemsOspfMessageDigestKeysListIndexedItem"))]
                pub struct MessageDigestKeys<'a, Mode> (::validated_data::Field<message_digest_keys::Item<'a, Mode>>);

                pub mod message_digest_keys {

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsOspfMessageDigestKeysItems"))]
                    pub struct Item<'a, Mode> {
                        pub id: ::validated_data::Field<i64>,
                        pub hash_algorithm: ::validated_data::Field<&'a str>,
                        pub cleartext_key: ::validated_data::RequiredValue<&'a str, Mode>,
                    }
                }

                #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsOspfNodesList"))]
                pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);
            }

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsEvpnL3Multicast"))]
            pub struct EvpnL3Multicast<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub evpn_underlay_l3_multicast_group: ::validated_data::Field<&'a str>,
                pub evpn_peg: ::validated_data::Field<evpn_l3_multicast::EvpnPeg<'a, Mode>>,
            }

            pub mod evpn_l3_multicast {

                #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsEvpnL3MulticastEvpnPegList"))]
                pub struct EvpnPeg<'a, Mode> (::validated_data::Field<evpn_peg::Item<'a, Mode>>);

                pub mod evpn_peg {

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsEvpnL3MulticastEvpnPegItems"))]
                    pub struct Item<'a, Mode> {
                        pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
                        pub transit: ::validated_data::Field<bool>,
                    }

                    pub mod item {

                        #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsEvpnL3MulticastEvpnPegItemsNodesList"))]
                        pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);
                    }
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsPimRpAddressesList"))]
            pub struct PimRpAddresses<'a, Mode> (::validated_data::Field<pim_rp_addresses::Item<'a, Mode>>);

            pub mod pim_rp_addresses {

                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsPimRpAddressesItems"))]
                pub struct Item<'a, Mode> {
                    pub rps: ::validated_data::Field<item::Rps<'a, Mode>>,
                    pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
                    pub groups: ::validated_data::Field<item::Groups<'a, Mode>>,
                    pub access_list_name: ::validated_data::Field<&'a str>,
                }

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsPimRpAddressesItemsRpsList"))]
                    pub struct Rps<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsPimRpAddressesItemsNodesList"))]
                    pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsPimRpAddressesItemsGroupsList"))]
                    pub struct Groups<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }

            #[::validated_data::data_view(list, primary_key(id), python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisList", item_name = "AVDDesignNetworkServicesItemsVrfsItemsSvisListKeyedItem"))]
            pub struct Svis<'a, Mode> (::validated_data::Field<svis::Item<'a, Mode>>);

            pub mod svis {

                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItems"))]
                pub struct Item<'a, Mode> {
                    pub id: ::validated_data::Field<i64>,
                    pub name: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub address_locking: ::validated_data::Field<super::super::super::super::super::super::eos_cli_config_gen::vlans::item::address_locking::AddressFamily<'a, Mode>>,
                    pub profile: ::validated_data::Field<&'a str>,
                    pub tags: ::validated_data::Field<item::Tags<'a, Mode>>,
                    pub evpn_vlan_bundle: ::validated_data::Field<&'a str>,
                    pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
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
                    pub structured_config: ::validated_data::Field<super::super::super::super::super::super::eos_cli_config_gen::vlan_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
                    pub evpn_l2_multi_domain: ::validated_data::Field<bool>,
                }

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsTagsList"))]
                    pub struct Tags<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(indexed_list, primary_key(node), python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesList", item_name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesListIndexedItem"))]
                    pub struct Nodes<'a, Mode> (::validated_data::Field<nodes::Item<'a, Mode>>);

                    pub mod nodes {

                        #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItems"))]
                        pub struct Item<'a, Mode> {
                            pub node: ::validated_data::Field<&'a str>,
                            pub tags: ::validated_data::Field<item::Tags<'a, Mode>>,
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
                            pub structured_config: ::validated_data::Field<super::super::super::super::super::super::super::super::eos_cli_config_gen::vlan_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
                            pub evpn_l2_multi_domain: ::validated_data::Field<bool>,
                        }

                        pub mod item {

                            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsTagsList"))]
                            pub struct Tags<'a, Mode> (::validated_data::Field<&'a str>);

                            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpAddressSecondariesList"))]
                            pub struct IpAddressSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

                            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6AddressVirtualsList"))]
                            pub struct Ipv6AddressVirtuals<'a, Mode> (::validated_data::Field<&'a str>);

                            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6Nd"))]
                            pub struct Ipv6Nd<'a, Mode> {
                                pub advertise_ipv6_address_virtuals: ::validated_data::Field<bool>,
                                pub valid_lifetime: ::validated_data::Field<&'a str>,
                                pub preferred_lifetime: ::validated_data::Field<&'a str>,
                                pub ra_dns_servers: ::validated_data::Field<ipv6_nd::RaDnsServers<'a, Mode>>,
                            }

                            pub mod ipv6_nd {

                                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6NdRaDnsServers"))]
                                pub struct RaDnsServers<'a, Mode> {
                                    pub servers: ::validated_data::Field<ra_dns_servers::Servers<'a, Mode>>,
                                    pub dns_servers_lifetime: ::validated_data::Field<i64>,
                                }

                                pub mod ra_dns_servers {

                                    #[::validated_data::data_view(indexed_list, primary_key(address), python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6NdRaDnsServersServersList", item_name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6NdRaDnsServersServersListIndexedItem"))]
                                    pub struct Servers<'a, Mode> (::validated_data::Field<servers::Item<'a, Mode>>);

                                    pub mod servers {

                                        #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6NdRaDnsServersServersItems"))]
                                        pub struct Item<'a, Mode> {
                                            pub address: ::validated_data::Field<&'a str>,
                                            pub lifetime: ::validated_data::Field<i64>,
                                        }
                                    }
                                }
                            }

                            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6DhcpRelay"))]
                            pub struct Ipv6DhcpRelay<'a, Mode> {
                                pub destinations: ::validated_data::Field<ipv6_dhcp_relay::Destinations<'a, Mode>>,
                            }

                            pub mod ipv6_dhcp_relay {

                                #[::validated_data::data_view(indexed_list, primary_key(address), python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6DhcpRelayDestinationsList", item_name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6DhcpRelayDestinationsListIndexedItem"))]
                                pub struct Destinations<'a, Mode> (::validated_data::Field<destinations::Item<'a, Mode>>);

                                pub mod destinations {

                                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6DhcpRelayDestinationsItems"))]
                                    pub struct Item<'a, Mode> {
                                        pub address: ::validated_data::Field<&'a str>,
                                        pub vrf: ::validated_data::Field<&'a str>,
                                        pub local_interface: ::validated_data::Field<&'a str>,
                                        pub source_address: ::validated_data::Field<&'a str>,
                                        pub link_address: ::validated_data::Field<&'a str>,
                                    }
                                }
                            }

                            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpAddressVirtualSecondariesList"))]
                            pub struct IpAddressVirtualSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

                            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpVirtualRouterAddressesList"))]
                            pub struct IpVirtualRouterAddresses<'a, Mode> (::validated_data::Field<&'a str>);

                            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6VirtualRouterAddressesList"))]
                            pub struct Ipv6VirtualRouterAddresses<'a, Mode> (::validated_data::Field<&'a str>);

                            #[::validated_data::data_view(indexed_list, primary_key(ip_helper), python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpHelpersList", item_name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpHelpersListIndexedItem"))]
                            pub struct IpHelpers<'a, Mode> (::validated_data::Field<ip_helpers::Item<'a, Mode>>);

                            pub mod ip_helpers {

                                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpHelpersItems"))]
                                pub struct Item<'a, Mode> {
                                    pub ip_helper: ::validated_data::Field<&'a str>,
                                    pub source_interface: ::validated_data::Field<&'a str>,
                                    pub source_vrf: ::validated_data::Field<&'a str>,
                                }
                            }

                            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsStaticRoutesList"))]
                            pub struct StaticRoutes<'a, Mode> (::validated_data::Field<static_routes::Item<'a, Mode>>);

                            pub mod static_routes {

                                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsStaticRoutesItems"))]
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

                            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6StaticRoutesList"))]
                            pub struct Ipv6StaticRoutes<'a, Mode> (::validated_data::Field<ipv6_static_routes::Item<'a, Mode>>);

                            pub mod ipv6_static_routes {

                                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6StaticRoutesItems"))]
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

                            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsTrunkGroupsList"))]
                            pub struct TrunkGroups<'a, Mode> (::validated_data::Field<&'a str>);

                            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsEvpnL2Multicast"))]
                            pub struct EvpnL2Multicast<'a, Mode> {
                                pub enabled: ::validated_data::Field<bool>,
                                pub always_redistribute_igmp: ::validated_data::Field<bool>,
                            }

                            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsVxlanFloodMulticast"))]
                            pub struct VxlanFloodMulticast<'a, Mode> {
                                pub enabled: ::validated_data::Field<bool>,
                                pub underlay_multicast_group: ::validated_data::Field<&'a str>,
                            }

                            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsEvpnL3Multicast"))]
                            pub struct EvpnL3Multicast<'a, Mode> {
                                pub enabled: ::validated_data::Field<bool>,
                            }

                            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIgmpSnooping"))]
                            pub struct IgmpSnooping<'a, Mode> {
                                pub enabled: ::validated_data::Field<bool>,
                                pub querier: ::validated_data::Field<igmp_snooping::Querier<'a, Mode>>,
                                pub fast_leave: ::validated_data::Field<bool>,
                            }

                            pub mod igmp_snooping {

                                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIgmpSnoopingQuerier"))]
                                pub struct Querier<'a, Mode> {
                                    pub enabled: ::validated_data::Field<bool>,
                                    pub source_address: ::validated_data::Field<&'a str>,
                                    pub version: ::validated_data::Field<i64>,
                                }
                            }

                            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIgmpSnoopingQuerier2"))]
                            pub struct IgmpSnoopingQuerier<'a, Mode> {
                                pub enabled: ::validated_data::Field<bool>,
                                pub source_address: ::validated_data::Field<&'a str>,
                                pub version: ::validated_data::Field<i64>,
                                pub fast_leave: ::validated_data::Field<bool>,
                            }

                            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsOspf"))]
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

                                #[::validated_data::data_view(indexed_list, primary_key(id), python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsOspfMessageDigestKeysList", item_name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsOspfMessageDigestKeysListIndexedItem"))]
                                pub struct MessageDigestKeys<'a, Mode> (::validated_data::Field<message_digest_keys::Item<'a, Mode>>);

                                pub mod message_digest_keys {

                                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsOspfMessageDigestKeysItems"))]
                                    pub struct Item<'a, Mode> {
                                        pub id: ::validated_data::Field<i64>,
                                        pub hash_algorithm: ::validated_data::Field<&'a str>,
                                        pub key: ::validated_data::Field<&'a str>,
                                        pub cleartext_key: ::validated_data::Field<&'a str>,
                                    }
                                }
                            }

                            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsBgp"))]
                            pub struct Bgp<'a, Mode> {
                                #[data_view(relaxed)]
                                pub structured_config: ::validated_data::Field<super::super::super::super::super::super::super::super::super::eos_cli_config_gen::router_bgp::vlans::Item<'a, ::validated_data::RelaxedValidated>>,
                                pub raw_eos_cli: ::validated_data::Field<&'a str>,
                            }

                            pub mod bgp {
                            }
                        }
                    }

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpAddressSecondariesList"))]
                    pub struct IpAddressSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6AddressVirtualsList"))]
                    pub struct Ipv6AddressVirtuals<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6Nd"))]
                    pub struct Ipv6Nd<'a, Mode> {
                        pub advertise_ipv6_address_virtuals: ::validated_data::Field<bool>,
                        pub valid_lifetime: ::validated_data::Field<&'a str>,
                        pub preferred_lifetime: ::validated_data::Field<&'a str>,
                        pub ra_dns_servers: ::validated_data::Field<ipv6_nd::RaDnsServers<'a, Mode>>,
                    }

                    pub mod ipv6_nd {

                        #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6NdRaDnsServers"))]
                        pub struct RaDnsServers<'a, Mode> {
                            pub servers: ::validated_data::Field<ra_dns_servers::Servers<'a, Mode>>,
                            pub dns_servers_lifetime: ::validated_data::Field<i64>,
                        }

                        pub mod ra_dns_servers {

                            #[::validated_data::data_view(indexed_list, primary_key(address), python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6NdRaDnsServersServersList", item_name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6NdRaDnsServersServersListIndexedItem"))]
                            pub struct Servers<'a, Mode> (::validated_data::Field<servers::Item<'a, Mode>>);

                            pub mod servers {

                                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6NdRaDnsServersServersItems"))]
                                pub struct Item<'a, Mode> {
                                    pub address: ::validated_data::Field<&'a str>,
                                    pub lifetime: ::validated_data::Field<i64>,
                                }
                            }
                        }
                    }

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6DhcpRelay"))]
                    pub struct Ipv6DhcpRelay<'a, Mode> {
                        pub destinations: ::validated_data::Field<ipv6_dhcp_relay::Destinations<'a, Mode>>,
                    }

                    pub mod ipv6_dhcp_relay {

                        #[::validated_data::data_view(indexed_list, primary_key(address), python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6DhcpRelayDestinationsList", item_name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6DhcpRelayDestinationsListIndexedItem"))]
                        pub struct Destinations<'a, Mode> (::validated_data::Field<destinations::Item<'a, Mode>>);

                        pub mod destinations {

                            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6DhcpRelayDestinationsItems"))]
                            pub struct Item<'a, Mode> {
                                pub address: ::validated_data::Field<&'a str>,
                                pub vrf: ::validated_data::Field<&'a str>,
                                pub local_interface: ::validated_data::Field<&'a str>,
                                pub source_address: ::validated_data::Field<&'a str>,
                                pub link_address: ::validated_data::Field<&'a str>,
                            }
                        }
                    }

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpAddressVirtualSecondariesList"))]
                    pub struct IpAddressVirtualSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpVirtualRouterAddressesList"))]
                    pub struct IpVirtualRouterAddresses<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6VirtualRouterAddressesList"))]
                    pub struct Ipv6VirtualRouterAddresses<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(indexed_list, primary_key(ip_helper), python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpHelpersList", item_name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpHelpersListIndexedItem"))]
                    pub struct IpHelpers<'a, Mode> (::validated_data::Field<ip_helpers::Item<'a, Mode>>);

                    pub mod ip_helpers {

                        #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpHelpersItems"))]
                        pub struct Item<'a, Mode> {
                            pub ip_helper: ::validated_data::Field<&'a str>,
                            pub source_interface: ::validated_data::Field<&'a str>,
                            pub source_vrf: ::validated_data::Field<&'a str>,
                        }
                    }

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsStaticRoutesList"))]
                    pub struct StaticRoutes<'a, Mode> (::validated_data::Field<static_routes::Item<'a, Mode>>);

                    pub mod static_routes {

                        #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsStaticRoutesItems"))]
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

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6StaticRoutesList"))]
                    pub struct Ipv6StaticRoutes<'a, Mode> (::validated_data::Field<ipv6_static_routes::Item<'a, Mode>>);

                    pub mod ipv6_static_routes {

                        #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6StaticRoutesItems"))]
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

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsTrunkGroupsList"))]
                    pub struct TrunkGroups<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsEvpnL2Multicast"))]
                    pub struct EvpnL2Multicast<'a, Mode> {
                        pub enabled: ::validated_data::Field<bool>,
                        pub always_redistribute_igmp: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsVxlanFloodMulticast"))]
                    pub struct VxlanFloodMulticast<'a, Mode> {
                        pub enabled: ::validated_data::Field<bool>,
                        pub underlay_multicast_group: ::validated_data::Field<&'a str>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsEvpnL3Multicast"))]
                    pub struct EvpnL3Multicast<'a, Mode> {
                        pub enabled: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIgmpSnooping"))]
                    pub struct IgmpSnooping<'a, Mode> {
                        pub enabled: ::validated_data::Field<bool>,
                        pub querier: ::validated_data::Field<igmp_snooping::Querier<'a, Mode>>,
                        pub fast_leave: ::validated_data::Field<bool>,
                    }

                    pub mod igmp_snooping {

                        #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIgmpSnoopingQuerier"))]
                        pub struct Querier<'a, Mode> {
                            pub enabled: ::validated_data::Field<bool>,
                            pub source_address: ::validated_data::Field<&'a str>,
                            pub version: ::validated_data::Field<i64>,
                        }
                    }

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIgmpSnoopingQuerier2"))]
                    pub struct IgmpSnoopingQuerier<'a, Mode> {
                        pub enabled: ::validated_data::Field<bool>,
                        pub source_address: ::validated_data::Field<&'a str>,
                        pub version: ::validated_data::Field<i64>,
                        pub fast_leave: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsOspf"))]
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

                        #[::validated_data::data_view(indexed_list, primary_key(id), python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsOspfMessageDigestKeysList", item_name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsOspfMessageDigestKeysListIndexedItem"))]
                        pub struct MessageDigestKeys<'a, Mode> (::validated_data::Field<message_digest_keys::Item<'a, Mode>>);

                        pub mod message_digest_keys {

                            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsOspfMessageDigestKeysItems"))]
                            pub struct Item<'a, Mode> {
                                pub id: ::validated_data::Field<i64>,
                                pub hash_algorithm: ::validated_data::Field<&'a str>,
                                pub key: ::validated_data::Field<&'a str>,
                                pub cleartext_key: ::validated_data::Field<&'a str>,
                            }
                        }
                    }

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsBgp"))]
                    pub struct Bgp<'a, Mode> {
                        #[data_view(relaxed)]
                        pub structured_config: ::validated_data::Field<super::super::super::super::super::super::super::eos_cli_config_gen::router_bgp::vlans::Item<'a, ::validated_data::RelaxedValidated>>,
                        pub raw_eos_cli: ::validated_data::Field<&'a str>,
                    }

                    pub mod bgp {
                    }
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesList"))]
            pub struct L3Interfaces<'a, Mode> (::validated_data::Field<l3_interfaces::Item<'a, Mode>>);

            pub mod l3_interfaces {

                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItems"))]
                pub struct Item<'a, Mode> {
                    pub interfaces: ::validated_data::Field<item::Interfaces<'a, Mode>>,
                    pub encapsulation_dot1q_vlan: ::validated_data::Field<item::EncapsulationDot1qVlan<'a, Mode>>,
                    pub ip_addresses: ::validated_data::Field<item::IpAddresses<'a, Mode>>,
                    pub ipv6_addresses: ::validated_data::Field<item::Ipv6Addresses<'a, Mode>>,
                    pub static_routes: ::validated_data::Field<item::StaticRoutes<'a, Mode>>,
                    pub ipv6_static_routes: ::validated_data::Field<item::Ipv6StaticRoutes<'a, Mode>>,
                    pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
                    pub arp_gratuitous_accept: ::validated_data::Field<bool>,
                    pub description: ::validated_data::Field<&'a str>,
                    pub descriptions: ::validated_data::Field<item::Descriptions<'a, Mode>>,
                    pub enabled: ::validated_data::Field<bool>,
                    pub mtu: ::validated_data::Field<i64>,
                    pub ipv4_acl_in: ::validated_data::Field<&'a str>,
                    pub ipv4_acl_out: ::validated_data::Field<&'a str>,
                    pub ipv6_acl_in: ::validated_data::Field<&'a str>,
                    pub ipv6_acl_out: ::validated_data::Field<&'a str>,
                    pub ospf: ::validated_data::Field<item::Ospf<'a, Mode>>,
                    pub pim: ::validated_data::Field<item::Pim<'a, Mode>>,
                    pub flow_tracking: ::validated_data::Field<item::FlowTracking<'a, Mode>>,
                    pub sflow: ::validated_data::Field<bool>,
                    pub monitor_sessions: ::validated_data::Field<item::MonitorSessions<'a, Mode>>,
                    pub campus_link_type: ::validated_data::Field<item::CampusLinkType<'a, Mode>>,
                    #[data_view(relaxed)]
                    pub structured_config: ::validated_data::Field<super::super::super::super::super::super::eos_cli_config_gen::ethernet_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
                    pub raw_eos_cli: ::validated_data::Field<&'a str>,
                }

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsInterfacesList"))]
                    pub struct Interfaces<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsEncapsulationDot1qVlanList"))]
                    pub struct EncapsulationDot1qVlan<'a, Mode> (::validated_data::Field<i64>);

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsIpAddressesList"))]
                    pub struct IpAddresses<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsIpv6AddressesList"))]
                    pub struct Ipv6Addresses<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsStaticRoutesList"))]
                    pub struct StaticRoutes<'a, Mode> (::validated_data::Field<static_routes::Item<'a, Mode>>);

                    pub mod static_routes {

                        #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsStaticRoutesItems"))]
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

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsIpv6StaticRoutesList"))]
                    pub struct Ipv6StaticRoutes<'a, Mode> (::validated_data::Field<ipv6_static_routes::Item<'a, Mode>>);

                    pub mod ipv6_static_routes {

                        #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsIpv6StaticRoutesItems"))]
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

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsNodesList"))]
                    pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsDescriptionsList"))]
                    pub struct Descriptions<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsOspf"))]
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

                        #[::validated_data::data_view(indexed_list, primary_key(id), python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsOspfMessageDigestKeysList", item_name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsOspfMessageDigestKeysListIndexedItem"))]
                        pub struct MessageDigestKeys<'a, Mode> (::validated_data::Field<message_digest_keys::Item<'a, Mode>>);

                        pub mod message_digest_keys {

                            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsOspfMessageDigestKeysItems"))]
                            pub struct Item<'a, Mode> {
                                pub id: ::validated_data::Field<i64>,
                                pub hash_algorithm: ::validated_data::Field<&'a str>,
                                pub key: ::validated_data::Field<&'a str>,
                                pub cleartext_key: ::validated_data::Field<&'a str>,
                            }
                        }
                    }

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsPim"))]
                    pub struct Pim<'a, Mode> {
                        pub enabled: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsFlowTracking"))]
                    pub struct FlowTracking<'a, Mode> {
                        pub enabled: ::validated_data::Field<bool>,
                        pub name: ::validated_data::Field<&'a str>,
                    }

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsMonitorSessionsList"))]
                    pub struct MonitorSessions<'a, Mode> (::validated_data::Field<monitor_sessions::Item<'a, Mode>>);

                    pub mod monitor_sessions {

                        #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsMonitorSessionsItems"))]
                        pub struct Item<'a, Mode> {
                            pub name: ::validated_data::RequiredValue<&'a str, Mode>,
                            pub role: ::validated_data::Field<&'a str>,
                            pub source_settings: ::validated_data::Field<item::SourceSettings<'a, Mode>>,
                            pub session_settings: ::validated_data::Field<item::SessionSettings<'a, Mode>>,
                        }

                        pub mod item {

                            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsMonitorSessionsItemsSourceSettings"))]
                            pub struct SourceSettings<'a, Mode> {
                                pub direction: ::validated_data::Field<&'a str>,
                                pub access_group: ::validated_data::Field<source_settings::AccessGroup<'a, Mode>>,
                            }

                            pub mod source_settings {

                                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsMonitorSessionsItemsSourceSettingsAccessGroup"))]
                                pub struct AccessGroup<'a, Mode> {
                                    #[data_view(rename = "type")]
                                    pub field_type: ::validated_data::Field<&'a str>,
                                    pub name: ::validated_data::Field<&'a str>,
                                    pub priority: ::validated_data::Field<i64>,
                                }
                            }

                            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsMonitorSessionsItemsSessionSettings"))]
                            pub struct SessionSettings<'a, Mode> {
                                pub encapsulation_gre_metadata_tx: ::validated_data::Field<bool>,
                                pub header_remove_size: ::validated_data::Field<i64>,
                                pub access_group: ::validated_data::Field<session_settings::AccessGroup<'a, Mode>>,
                                pub rate_limit_per_ingress_chip: ::validated_data::Field<&'a str>,
                                pub rate_limit_per_egress_chip: ::validated_data::Field<&'a str>,
                                pub sample: ::validated_data::Field<i64>,
                                pub truncate: ::validated_data::Field<session_settings::Truncate<'a, Mode>>,
                            }

                            pub mod session_settings {

                                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsMonitorSessionsItemsSessionSettingsAccessGroup"))]
                                pub struct AccessGroup<'a, Mode> {
                                    #[data_view(rename = "type")]
                                    pub field_type: ::validated_data::Field<&'a str>,
                                    pub name: ::validated_data::Field<&'a str>,
                                }

                                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsMonitorSessionsItemsSessionSettingsTruncate"))]
                                pub struct Truncate<'a, Mode> {
                                    pub enabled: ::validated_data::Field<bool>,
                                    pub size: ::validated_data::Field<i64>,
                                }
                            }
                        }
                    }

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsCampusLinkTypeList"))]
                    pub struct CampusLinkType<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsList"))]
            pub struct L3PortChannels<'a, Mode> (::validated_data::Field<l3_port_channels::Item<'a, Mode>>);

            pub mod l3_port_channels {

                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItems"))]
                pub struct Item<'a, Mode> {
                    pub name: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub node: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub arp_gratuitous_accept: ::validated_data::Field<bool>,
                    pub description: ::validated_data::Field<&'a str>,
                    pub mode: ::validated_data::Field<&'a str>,
                    pub member_interfaces: ::validated_data::Field<item::MemberInterfaces<'a, Mode>>,
                    pub ip_address: ::validated_data::Field<&'a str>,
                    pub ip_address_secondaries: ::validated_data::Field<item::IpAddressSecondaries<'a, Mode>>,
                    pub ipv6_addresses: ::validated_data::Field<item::Ipv6Addresses<'a, Mode>>,
                    pub encapsulation_dot1q_vlan: ::validated_data::Field<i64>,
                    pub enabled: ::validated_data::Field<bool>,
                    pub peer: ::validated_data::Field<&'a str>,
                    pub peer_port_channel: ::validated_data::Field<&'a str>,
                    pub mtu: ::validated_data::Field<i64>,
                    pub ipv4_acl_in: ::validated_data::Field<&'a str>,
                    pub ipv4_acl_out: ::validated_data::Field<&'a str>,
                    pub ipv6_acl_in: ::validated_data::Field<&'a str>,
                    pub ipv6_acl_out: ::validated_data::Field<&'a str>,
                    pub static_routes: ::validated_data::Field<item::StaticRoutes<'a, Mode>>,
                    pub ipv6_static_routes: ::validated_data::Field<item::Ipv6StaticRoutes<'a, Mode>>,
                    pub ospf: ::validated_data::Field<item::Ospf<'a, Mode>>,
                    pub flow_tracking: ::validated_data::Field<item::FlowTracking<'a, Mode>>,
                    #[data_view(relaxed)]
                    pub structured_config: ::validated_data::Field<super::super::super::super::super::super::eos_cli_config_gen::port_channel_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
                    pub raw_eos_cli: ::validated_data::Field<&'a str>,
                }

                pub mod item {

                    #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsMemberInterfacesList", item_name = "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsMemberInterfacesListIndexedItem"))]
                    pub struct MemberInterfaces<'a, Mode> (::validated_data::Field<member_interfaces::Item<'a, Mode>>);

                    pub mod member_interfaces {

                        #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsMemberInterfacesItems"))]
                        pub struct Item<'a, Mode> {
                            pub name: ::validated_data::RequiredValue<&'a str, Mode>,
                            pub description: ::validated_data::Field<&'a str>,
                            pub peer: ::validated_data::Field<&'a str>,
                            pub peer_interface: ::validated_data::Field<&'a str>,
                            pub speed: ::validated_data::Field<&'a str>,
                            #[data_view(relaxed)]
                            pub structured_config: ::validated_data::Field<super::super::super::super::super::super::super::super::eos_cli_config_gen::ethernet_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
                        }

                        pub mod item {
                        }
                    }

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsIpAddressSecondariesList"))]
                    pub struct IpAddressSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsIpv6AddressesList"))]
                    pub struct Ipv6Addresses<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsStaticRoutesList"))]
                    pub struct StaticRoutes<'a, Mode> (::validated_data::Field<static_routes::Item<'a, Mode>>);

                    pub mod static_routes {

                        #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsStaticRoutesItems"))]
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

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsIpv6StaticRoutesList"))]
                    pub struct Ipv6StaticRoutes<'a, Mode> (::validated_data::Field<ipv6_static_routes::Item<'a, Mode>>);

                    pub mod ipv6_static_routes {

                        #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsIpv6StaticRoutesItems"))]
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

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsOspf"))]
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

                        #[::validated_data::data_view(indexed_list, primary_key(id), python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsOspfMessageDigestKeysList", item_name = "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsOspfMessageDigestKeysListIndexedItem"))]
                        pub struct MessageDigestKeys<'a, Mode> (::validated_data::Field<message_digest_keys::Item<'a, Mode>>);

                        pub mod message_digest_keys {

                            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsOspfMessageDigestKeysItems"))]
                            pub struct Item<'a, Mode> {
                                pub id: ::validated_data::Field<i64>,
                                pub hash_algorithm: ::validated_data::Field<&'a str>,
                                pub key: ::validated_data::Field<&'a str>,
                                pub cleartext_key: ::validated_data::Field<&'a str>,
                            }
                        }
                    }

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsFlowTracking"))]
                    pub struct FlowTracking<'a, Mode> {
                        pub enabled: ::validated_data::Field<bool>,
                        pub name: ::validated_data::Field<&'a str>,
                    }
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsLoopbacksList"))]
            pub struct Loopbacks<'a, Mode> (::validated_data::Field<loopbacks::Item<'a, Mode>>);

            pub mod loopbacks {

                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsLoopbacksItems"))]
                pub struct Item<'a, Mode> {
                    pub node: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub loopback: ::validated_data::RequiredValue<i64, Mode>,
                    pub ip_address: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub description: ::validated_data::Field<&'a str>,
                    pub enabled: ::validated_data::Field<bool>,
                    pub ospf: ::validated_data::Field<item::Ospf<'a, Mode>>,
                    pub hardware_forwarding: ::validated_data::Field<bool>,
                    pub raw_eos_cli: ::validated_data::Field<&'a str>,
                }

                pub mod item {

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsLoopbacksItemsOspf"))]
                    pub struct Ospf<'a, Mode> {
                        pub enabled: ::validated_data::Field<bool>,
                        pub area: ::validated_data::Field<&'a str>,
                    }
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsStaticRoutesList"))]
            pub struct StaticRoutes<'a, Mode> (::validated_data::Field<static_routes::Item<'a, Mode>>);

            pub mod static_routes {

                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsStaticRoutesItems"))]
                pub struct Item<'a, Mode> {
                    pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
                    pub prefix: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub next_hop: ::validated_data::Field<&'a str>,
                    pub track_bfd: ::validated_data::Field<bool>,
                    pub distance: ::validated_data::Field<i64>,
                    pub tag: ::validated_data::Field<i64>,
                    pub name: ::validated_data::Field<&'a str>,
                    pub metric: ::validated_data::Field<i64>,
                    pub interface: ::validated_data::Field<&'a str>,
                }

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsStaticRoutesItemsNodesList"))]
                    pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsIpv6StaticRoutesList"))]
            pub struct Ipv6StaticRoutes<'a, Mode> (::validated_data::Field<ipv6_static_routes::Item<'a, Mode>>);

            pub mod ipv6_static_routes {

                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsIpv6StaticRoutesItems"))]
                pub struct Item<'a, Mode> {
                    pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
                    pub prefix: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub next_hop: ::validated_data::Field<&'a str>,
                    pub track_bfd: ::validated_data::Field<bool>,
                    pub distance: ::validated_data::Field<i64>,
                    pub tag: ::validated_data::Field<i64>,
                    pub name: ::validated_data::Field<&'a str>,
                    pub metric: ::validated_data::Field<i64>,
                    pub interface: ::validated_data::Field<&'a str>,
                }

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsIpv6StaticRoutesItemsNodesList"))]
                    pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsStaticArpEntriesList"))]
            pub struct StaticArpEntries<'a, Mode> (::validated_data::Field<static_arp_entries::Item<'a, Mode>>);

            pub mod static_arp_entries {

                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsStaticArpEntriesItems"))]
                pub struct Item<'a, Mode> {
                    pub ipv4_address: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub mac_address: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
                }

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsStaticArpEntriesItemsNodesList"))]
                    pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }

            #[::validated_data::data_view(list, primary_key(ip_address), python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeersList", item_name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeersListKeyedItem"))]
            pub struct BgpPeers<'a, Mode> (::validated_data::Field<bgp_peers::Item<'a, Mode>>);

            pub mod bgp_peers {

                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeersItems"))]
                pub struct Item<'a, Mode> {
                    pub ip_address: ::validated_data::Field<&'a str>,
                    pub peer_group: ::validated_data::Field<&'a str>,
                    pub remote_as: ::validated_data::Field<&'a str>,
                    pub description: ::validated_data::Field<&'a str>,
                    pub password: ::validated_data::Field<&'a str>,
                    pub cleartext_password: ::validated_data::Field<&'a str>,
                    pub send_community: ::validated_data::Field<&'a str>,
                    pub next_hop_self: ::validated_data::Field<bool>,
                    pub timers: ::validated_data::Field<&'a str>,
                    pub maximum_routes: ::validated_data::Field<i64>,
                    pub maximum_routes_warning_only: ::validated_data::Field<bool>,
                    pub default_originate: ::validated_data::Field<item::DefaultOriginate<'a, Mode>>,
                    pub update_source: ::validated_data::Field<&'a str>,
                    pub ebgp_multihop: ::validated_data::Field<i64>,
                    pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
                    pub set_ipv4_next_hop: ::validated_data::Field<&'a str>,
                    pub set_ipv6_next_hop: ::validated_data::Field<&'a str>,
                    pub route_map_out: ::validated_data::Field<&'a str>,
                    pub route_map_in: ::validated_data::Field<&'a str>,
                    pub prefix_list_in: ::validated_data::Field<&'a str>,
                    pub prefix_list_out: ::validated_data::Field<&'a str>,
                    pub local_as: ::validated_data::Field<&'a str>,
                    pub weight: ::validated_data::Field<i64>,
                    pub bfd: ::validated_data::Field<bool>,
                    pub bfd_timers: ::validated_data::Field<super::super::super::super::super::super::eos_cli_config_gen::router_bgp::vrfs::item::neighbors::item::BfdTimers<'a, Mode>>,
                    pub route_reflector_client: ::validated_data::Field<bool>,
                    pub shutdown: ::validated_data::Field<bool>,
                }

                pub mod item {

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeersItemsDefaultOriginate"))]
                    pub struct DefaultOriginate<'a, Mode> {
                        pub always: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeersItemsNodesList"))]
                    pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgp"))]
            pub struct Bgp<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub router_id: ::validated_data::Field<&'a str>,
                pub graceful_restart: ::validated_data::Field<bgp::GracefulRestart<'a, Mode>>,
                pub raw_eos_cli: ::validated_data::Field<&'a str>,
                #[data_view(relaxed)]
                pub structured_config: ::validated_data::Field<super::super::super::super::super::eos_cli_config_gen::router_bgp::vrfs::Item<'a, ::validated_data::RelaxedValidated>>,
            }

            pub mod bgp {

                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpGracefulRestart"))]
                pub struct GracefulRestart<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub restart_time: ::validated_data::Field<i64>,
                }
            }

            #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsList", item_name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsListIndexedItem"))]
            pub struct BgpPeerGroups<'a, Mode> (::validated_data::Field<bgp_peer_groups::Item<'a, Mode>>);

            pub mod bgp_peer_groups {

                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItems"))]
                pub struct Item<'a, Mode> {
                    pub name: ::validated_data::Field<&'a str>,
                    pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
                    pub password: ::validated_data::Field<&'a str>,
                    pub cleartext_password: ::validated_data::Field<&'a str>,
                    pub address_family_ipv4: ::validated_data::Field<item::AddressFamilyIpv4<'a, Mode>>,
                    pub address_family_ipv6: ::validated_data::Field<item::AddressFamilyIpv6<'a, Mode>>,
                    pub listen_ranges: ::validated_data::Field<item::ListenRanges<'a, Mode>>,
                    pub metadata: ::validated_data::Field<item::Metadata<'a, Mode>>,
                    pub remote_as: ::validated_data::Field<&'a str>,
                    pub local_as: ::validated_data::Field<&'a str>,
                    pub description: ::validated_data::Field<&'a str>,
                    pub shutdown: ::validated_data::Field<bool>,
                    pub as_path: ::validated_data::Field<item::AsPath<'a, Mode>>,
                    pub remove_private_as: ::validated_data::Field<item::RemovePrivateAs<'a, Mode>>,
                    pub remove_private_as_ingress: ::validated_data::Field<item::RemovePrivateAsIngress<'a, Mode>>,
                    pub next_hop_unchanged: ::validated_data::Field<bool>,
                    pub update_source: ::validated_data::Field<&'a str>,
                    pub route_reflector_client: ::validated_data::Field<bool>,
                    pub bfd: ::validated_data::Field<bool>,
                    pub bfd_timers: ::validated_data::Field<item::BfdTimers<'a, Mode>>,
                    pub ebgp_multihop: ::validated_data::Field<i64>,
                    pub next_hop_peer: ::validated_data::Field<bool>,
                    pub next_hop_self: ::validated_data::Field<bool>,
                    pub password_type: ::validated_data::Field<&'a str>,
                    pub passive: ::validated_data::Field<bool>,
                    pub default_originate: ::validated_data::Field<item::DefaultOriginate<'a, Mode>>,
                    pub enforce_first_as: ::validated_data::Field<bool>,
                    pub send_community: ::validated_data::Field<&'a str>,
                    pub maximum_routes: ::validated_data::Field<i64>,
                    pub maximum_routes_warning_limit: ::validated_data::Field<&'a str>,
                    pub maximum_routes_warning_only: ::validated_data::Field<bool>,
                    pub maximum_accepted_routes: ::validated_data::Field<item::MaximumAcceptedRoutes<'a, Mode>>,
                    pub missing_policy: ::validated_data::Field<item::MissingPolicy<'a, Mode>>,
                    pub link_bandwidth: ::validated_data::Field<item::LinkBandwidth<'a, Mode>>,
                    pub allowas_in: ::validated_data::Field<item::AllowasIn<'a, Mode>>,
                    pub weight: ::validated_data::Field<i64>,
                    pub timers: ::validated_data::Field<&'a str>,
                    pub rib_in_pre_policy_retain: ::validated_data::Field<item::RibInPrePolicyRetain<'a, Mode>>,
                    pub route_map_in: ::validated_data::Field<&'a str>,
                    pub route_map_out: ::validated_data::Field<&'a str>,
                    pub peer_tag_in: ::validated_data::Field<&'a str>,
                    pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
                    pub session_tracker: ::validated_data::Field<&'a str>,
                    pub shared_secret: ::validated_data::Field<item::SharedSecret<'a, Mode>>,
                    pub ttl_maximum_hops: ::validated_data::Field<i64>,
                    pub maximum_advertised_routes: ::validated_data::Field<i64>,
                    pub maximum_advertised_routes_warning_limit: ::validated_data::Field<&'a str>,
                }

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsNodesList"))]
                    pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsAddressFamilyIpv4"))]
                    pub struct AddressFamilyIpv4<'a, Mode> {
                        pub activate: ::validated_data::Field<bool>,
                        pub route_map_in: ::validated_data::Field<&'a str>,
                        pub route_map_out: ::validated_data::Field<&'a str>,
                        pub rcf_in: ::validated_data::Field<&'a str>,
                        pub rcf_out: ::validated_data::Field<&'a str>,
                        pub default_originate: ::validated_data::Field<super::super::super::super::super::super::super::eos_cli_config_gen::router_bgp::address_family_ipv4::peer_groups::item::DefaultOriginate<'a, Mode>>,
                        pub next_hop: ::validated_data::Field<super::super::super::super::super::super::super::eos_cli_config_gen::router_bgp::address_family_ipv4::peer_groups::item::NextHop<'a, Mode>>,
                        pub prefix_list_in: ::validated_data::Field<&'a str>,
                        pub prefix_list_out: ::validated_data::Field<&'a str>,
                    }

                    pub mod address_family_ipv4 {
                    }

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsAddressFamilyIpv6"))]
                    pub struct AddressFamilyIpv6<'a, Mode> {
                        pub activate: ::validated_data::Field<bool>,
                        pub route_map_in: ::validated_data::Field<&'a str>,
                        pub route_map_out: ::validated_data::Field<&'a str>,
                        pub rcf_in: ::validated_data::Field<&'a str>,
                        pub rcf_out: ::validated_data::Field<&'a str>,
                        pub default_originate: ::validated_data::Field<address_family_ipv6::DefaultOriginate<'a, Mode>>,
                        pub prefix_list_in: ::validated_data::Field<&'a str>,
                        pub prefix_list_out: ::validated_data::Field<&'a str>,
                    }

                    pub mod address_family_ipv6 {

                        #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsAddressFamilyIpv6DefaultOriginate"))]
                        pub struct DefaultOriginate<'a, Mode> {
                            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                            pub always: ::validated_data::Field<bool>,
                            pub route_map: ::validated_data::Field<&'a str>,
                        }
                    }

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsListenRangesList"))]
                    pub struct ListenRanges<'a, Mode> (::validated_data::Field<listen_ranges::Item<'a, Mode>>);

                    pub mod listen_ranges {

                        #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsListenRangesItems"))]
                        pub struct Item<'a, Mode> {
                            pub prefix: ::validated_data::RequiredValue<&'a str, Mode>,
                            pub remote_as: ::validated_data::Field<&'a str>,
                            pub peer_id_include_router_id: ::validated_data::Field<bool>,
                            pub peer_filter: ::validated_data::Field<&'a str>,
                        }
                    }

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsMetadata"))]
                    pub struct Metadata<'a, Mode> {
                        #[data_view(rename = "type")]
                        pub field_type: ::validated_data::Field<&'a str>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsAsPath"))]
                    pub struct AsPath<'a, Mode> {
                        pub remote_as_replace_out: ::validated_data::Field<bool>,
                        pub prepend_own_disabled: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsRemovePrivateAs"))]
                    pub struct RemovePrivateAs<'a, Mode> {
                        pub enabled: ::validated_data::Field<bool>,
                        pub all: ::validated_data::Field<bool>,
                        pub replace_as: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsRemovePrivateAsIngress"))]
                    pub struct RemovePrivateAsIngress<'a, Mode> {
                        pub enabled: ::validated_data::Field<bool>,
                        pub replace_as: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsBfdTimers"))]
                    pub struct BfdTimers<'a, Mode> {
                        pub interval: ::validated_data::RequiredValue<i64, Mode>,
                        pub min_rx: ::validated_data::RequiredValue<i64, Mode>,
                        pub multiplier: ::validated_data::RequiredValue<i64, Mode>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsDefaultOriginate"))]
                    pub struct DefaultOriginate<'a, Mode> {
                        pub enabled: ::validated_data::Field<bool>,
                        pub always: ::validated_data::Field<bool>,
                        pub route_map: ::validated_data::Field<&'a str>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsMaximumAcceptedRoutes"))]
                    pub struct MaximumAcceptedRoutes<'a, Mode> {
                        pub limit: ::validated_data::RequiredValue<i64, Mode>,
                        pub warning_limit: ::validated_data::Field<maximum_accepted_routes::WarningLimit<'a, Mode>>,
                    }

                    pub mod maximum_accepted_routes {

                        #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsMaximumAcceptedRoutesWarningLimit"))]
                        pub struct WarningLimit<'a, Mode> {
                            pub count: ::validated_data::Field<i64>,
                            pub percent: ::validated_data::Field<i64>,
                        }
                    }

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsMissingPolicy"))]
                    pub struct MissingPolicy<'a, Mode> {
                        pub direction_in: ::validated_data::Field<missing_policy::DirectionIn<'a, Mode>>,
                        pub direction_out: ::validated_data::Field<missing_policy::DirectionOut<'a, Mode>>,
                    }

                    pub mod missing_policy {

                        #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsMissingPolicyDirectionIn"))]
                        pub struct DirectionIn<'a, Mode> {
                            pub action: ::validated_data::RequiredValue<&'a str, Mode>,
                            pub include_community_list: ::validated_data::Field<bool>,
                            pub include_prefix_list: ::validated_data::Field<bool>,
                            pub include_sub_route_map: ::validated_data::Field<bool>,
                        }

                        #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsMissingPolicyDirectionOut"))]
                        pub struct DirectionOut<'a, Mode> {
                            pub action: ::validated_data::RequiredValue<&'a str, Mode>,
                            pub include_community_list: ::validated_data::Field<bool>,
                            pub include_prefix_list: ::validated_data::Field<bool>,
                            pub include_sub_route_map: ::validated_data::Field<bool>,
                        }
                    }

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsLinkBandwidth"))]
                    pub struct LinkBandwidth<'a, Mode> {
                        pub enabled: ::validated_data::Field<bool>,
                        pub default: ::validated_data::Field<&'a str>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsAllowasIn"))]
                    pub struct AllowasIn<'a, Mode> {
                        pub enabled: ::validated_data::Field<bool>,
                        pub times: ::validated_data::Field<i64>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsRibInPrePolicyRetain"))]
                    pub struct RibInPrePolicyRetain<'a, Mode> {
                        pub enabled: ::validated_data::Field<bool>,
                        pub all: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsSharedSecret"))]
                    pub struct SharedSecret<'a, Mode> {
                        pub profile: ::validated_data::RequiredValue<&'a str, Mode>,
                        pub hash_algorithm: ::validated_data::RequiredValue<&'a str, Mode>,
                    }
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsAdditionalRouteTargetsList"))]
            pub struct AdditionalRouteTargets<'a, Mode> (::validated_data::Field<additional_route_targets::Item<'a, Mode>>);

            pub mod additional_route_targets {

                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsAdditionalRouteTargetsItems"))]
                pub struct Item<'a, Mode> {
                    #[data_view(rename = "type")]
                    pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub address_family: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub route_target: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
                }

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsAdditionalRouteTargetsItemsNodesList"))]
                    pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsAggregateAddressesList"))]
            pub struct AggregateAddresses<'a, Mode> (::validated_data::Field<aggregate_addresses::Item<'a, Mode>>);

            pub mod aggregate_addresses {

                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsAggregateAddressesItems"))]
                pub struct Item<'a, Mode> {
                    pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
                    pub prefix: ::validated_data::Field<&'a str>,
                    pub advertise_only: ::validated_data::Field<bool>,
                    pub as_set: ::validated_data::Field<bool>,
                    pub summary_only: ::validated_data::Field<bool>,
                    pub attribute_map: ::validated_data::Field<&'a str>,
                    pub match_map: ::validated_data::Field<&'a str>,
                    pub attribute: ::validated_data::Field<item::Attribute<'a, Mode>>,
                }

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsVrfsItemsAggregateAddressesItemsNodesList"))]
                    pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVrfsItemsAggregateAddressesItemsAttribute"))]
                    pub struct Attribute<'a, Mode> {
                        pub rcf: ::validated_data::Field<&'a str>,
                    }
                }
            }
        }
    }

    #[::validated_data::data_view(list, primary_key(id), python(name = "AVDDesignNetworkServicesItemsL2vlansList", item_name = "AVDDesignNetworkServicesItemsL2vlansListKeyedItem"))]
    pub struct L2vlans<'a, Mode> (::validated_data::Field<l2vlans::Item<'a, Mode>>);

    pub mod l2vlans {

        #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsL2vlansItems"))]
        pub struct Item<'a, Mode> {
            pub id: ::validated_data::Field<i64>,
            pub name: ::validated_data::RequiredValue<&'a str, Mode>,
            pub profile: ::validated_data::Field<&'a str>,
            pub tags: ::validated_data::Field<item::Tags<'a, Mode>>,
            pub address_locking: ::validated_data::Field<super::super::super::super::eos_cli_config_gen::vlans::item::address_locking::AddressFamily<'a, Mode>>,
            pub vni_override: ::validated_data::Field<i64>,
            pub rt_override: ::validated_data::Field<&'a str>,
            pub rd_override: ::validated_data::Field<&'a str>,
            pub vxlan: ::validated_data::Field<bool>,
            pub spanning_tree_priority: ::validated_data::Field<i64>,
            pub evpn_vlan_bundle: ::validated_data::Field<&'a str>,
            pub trunk_groups: ::validated_data::Field<item::TrunkGroups<'a, Mode>>,
            pub evpn_l2_multi_domain: ::validated_data::Field<bool>,
            pub evpn_l2_multicast: ::validated_data::Field<item::EvpnL2Multicast<'a, Mode>>,
            pub vxlan_flood_multicast: ::validated_data::Field<item::VxlanFloodMulticast<'a, Mode>>,
            pub igmp_snooping: ::validated_data::Field<item::IgmpSnooping<'a, Mode>>,
            pub igmp_snooping_enabled: ::validated_data::Field<bool>,
            pub igmp_snooping_querier: ::validated_data::Field<item::IgmpSnoopingQuerier<'a, Mode>>,
            pub bgp: ::validated_data::Field<item::Bgp<'a, Mode>>,
            pub private_vlan: ::validated_data::Field<item::PrivateVlan<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsL2vlansItemsTagsList"))]
            pub struct Tags<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsL2vlansItemsTrunkGroupsList"))]
            pub struct TrunkGroups<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsL2vlansItemsEvpnL2Multicast"))]
            pub struct EvpnL2Multicast<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsL2vlansItemsVxlanFloodMulticast"))]
            pub struct VxlanFloodMulticast<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub underlay_multicast_group: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsL2vlansItemsIgmpSnooping"))]
            pub struct IgmpSnooping<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub querier: ::validated_data::Field<igmp_snooping::Querier<'a, Mode>>,
                pub fast_leave: ::validated_data::Field<bool>,
            }

            pub mod igmp_snooping {

                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsL2vlansItemsIgmpSnoopingQuerier"))]
                pub struct Querier<'a, Mode> {
                    pub enabled: ::validated_data::Field<bool>,
                    pub source_address: ::validated_data::Field<&'a str>,
                    pub version: ::validated_data::Field<i64>,
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsL2vlansItemsIgmpSnoopingQuerier2"))]
            pub struct IgmpSnoopingQuerier<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub source_address: ::validated_data::Field<&'a str>,
                pub version: ::validated_data::Field<i64>,
                pub fast_leave: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsL2vlansItemsBgp"))]
            pub struct Bgp<'a, Mode> {
                #[data_view(relaxed)]
                pub structured_config: ::validated_data::Field<super::super::super::super::super::eos_cli_config_gen::router_bgp::vlans::Item<'a, ::validated_data::RelaxedValidated>>,
                pub raw_eos_cli: ::validated_data::Field<&'a str>,
            }

            pub mod bgp {
            }

            #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsL2vlansItemsPrivateVlan"))]
            pub struct PrivateVlan<'a, Mode> {
                #[data_view(rename = "type")]
                pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
                pub primary_vlan: ::validated_data::RequiredValue<i64, Mode>,
            }
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsVpws"))]
    pub struct Vpws<'a, Mode> {
        pub mpls_control_word: ::validated_data::Field<bool>,
        pub mtu: ::validated_data::Field<i64>,
        pub label_flow: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesignNetworkServicesItemsPointToPointServicesList", item_name = "AVDDesignNetworkServicesItemsPointToPointServicesListIndexedItem"))]
    pub struct PointToPointServices<'a, Mode> (::validated_data::Field<point_to_point_services::Item<'a, Mode>>);

    pub mod point_to_point_services {

        #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsPointToPointServicesItems"))]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            #[data_view(rename = "type")]
            pub field_type: ::validated_data::Field<&'a str>,
            pub subinterfaces: ::validated_data::Field<item::Subinterfaces<'a, Mode>>,
            pub endpoints: ::validated_data::Field<item::Endpoints<'a, Mode>>,
            pub lldp_disable: ::validated_data::Field<bool>,
        }

        pub mod item {

            #[::validated_data::data_view(indexed_list, primary_key(number), python(name = "AVDDesignNetworkServicesItemsPointToPointServicesItemsSubinterfacesList", item_name = "AVDDesignNetworkServicesItemsPointToPointServicesItemsSubinterfacesListIndexedItem"))]
            pub struct Subinterfaces<'a, Mode> (::validated_data::Field<subinterfaces::Item<'a, Mode>>);

            pub mod subinterfaces {

                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsPointToPointServicesItemsSubinterfacesItems"))]
                pub struct Item<'a, Mode> {
                    pub number: ::validated_data::Field<i64>,
                    pub port_channel: ::validated_data::Field<item::PortChannel<'a, Mode>>,
                    #[data_view(relaxed)]
                    pub structured_config: ::validated_data::Field<super::super::super::super::super::super::eos_cli_config_gen::ethernet_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
                    pub raw_eos_cli: ::validated_data::Field<&'a str>,
                }

                pub mod item {

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsPointToPointServicesItemsSubinterfacesItemsPortChannel"))]
                    pub struct PortChannel<'a, Mode> {
                        #[data_view(relaxed)]
                        pub structured_config: ::validated_data::Field<super::super::super::super::super::super::super::eos_cli_config_gen::port_channel_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
                        pub raw_eos_cli: ::validated_data::Field<&'a str>,
                    }

                    pub mod port_channel {
                    }
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsPointToPointServicesItemsEndpointsList"))]
            pub struct Endpoints<'a, Mode> (::validated_data::Field<endpoints::Item<'a, Mode>>);

            pub mod endpoints {

                #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsPointToPointServicesItemsEndpointsItems"))]
                pub struct Item<'a, Mode> {
                    pub id: ::validated_data::RequiredValue<i64, Mode>,
                    pub nodes: ::validated_data::RequiredValue<item::Nodes<'a, Mode>, Mode>,
                    pub interfaces: ::validated_data::RequiredValue<item::Interfaces<'a, Mode>, Mode>,
                    pub port_channel: ::validated_data::Field<item::PortChannel<'a, Mode>>,
                }

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsPointToPointServicesItemsEndpointsItemsNodesList"))]
                    pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list, python(name = "AVDDesignNetworkServicesItemsPointToPointServicesItemsEndpointsItemsInterfacesList"))]
                    pub struct Interfaces<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(python(name = "AVDDesignNetworkServicesItemsPointToPointServicesItemsEndpointsItemsPortChannel"))]
                    pub struct PortChannel<'a, Mode> {
                        pub mode: ::validated_data::Field<&'a str>,
                        pub short_esi: ::validated_data::Field<&'a str>,
                    }
                }
            }
        }
    }
}
