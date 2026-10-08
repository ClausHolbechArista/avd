// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar mac_vrf_vni_base("mac_vrf_vni_base", 1) -> i64;
        scalar mac_vrf_id_base("mac_vrf_id_base", 2) -> i64;
        scalar vlan_aware_bundle_number_base("vlan_aware_bundle_number_base", 3) -> i64;
        scalar pseudowire_rt_base("pseudowire_rt_base", 4) -> i64;
        scalar enable_mlag_ibgp_peering_vrfs("enable_mlag_ibgp_peering_vrfs", 5) -> bool;
        scalar redistribute_mlag_ibgp_peering_vrfs("redistribute_mlag_ibgp_peering_vrfs", 6) -> bool;
        scalar evpn_vlan_bundle("evpn_vlan_bundle", 7) -> &'a str;
        model bgp_peer_groups("bgp_peer_groups", 8) -> item::BgpPeerGroups<'a>;
        model igmp_snooping("igmp_snooping", 9) -> item::IgmpSnooping<'a>;
        model evpn_l2_multicast("evpn_l2_multicast", 10) -> item::EvpnL2Multicast<'a>;
        model vxlan_flood_multicast("vxlan_flood_multicast", 11) -> item::VxlanFloodMulticast<'a>;
        model evpn_l3_multicast("evpn_l3_multicast", 12) -> item::EvpnL3Multicast<'a>;
        model pim_rp_addresses("pim_rp_addresses", 13) -> item::PimRpAddresses<'a>;
        model igmp_snooping_querier("igmp_snooping_querier", 14) -> item::IgmpSnoopingQuerier<'a>;
        scalar evpn_l2_multi_domain("evpn_l2_multi_domain", 15) -> bool;
        model vrfs("vrfs", 16) -> item::Vrfs<'a>;
        model l2vlans("l2vlans", 17) -> item::L2vlans<'a>;
        model vpws("vpws", 18) -> item::Vpws<'a>;
        model point_to_point_services("point_to_point_services", 19) -> item::PointToPointServices<'a>;
    }
}

pub mod item {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct BgpPeerGroups {
            model item (0) -> bgp_peer_groups::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod bgp_peer_groups {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar password("password", 1) -> &'a str;
                scalar cleartext_password("cleartext_password", 2) -> &'a str;
                model nodes("nodes", 3) -> item::Nodes<'a>;
                model address_family_ipv4("address_family_ipv4", 4) -> item::AddressFamilyIpv4<'a>;
                model address_family_ipv6("address_family_ipv6", 5) -> item::AddressFamilyIpv6<'a>;
                model listen_ranges("listen_ranges", 6) -> item::ListenRanges<'a>;
                model metadata("metadata", 7) -> item::Metadata<'a>;
                scalar remote_as("remote_as", 8) -> &'a str;
                scalar local_as("local_as", 9) -> &'a str;
                scalar description("description", 10) -> &'a str;
                scalar shutdown("shutdown", 11) -> bool;
                model as_path("as_path", 12) -> item::AsPath<'a>;
                model remove_private_as("remove_private_as", 13) -> item::RemovePrivateAs<'a>;
                model remove_private_as_ingress("remove_private_as_ingress", 14) -> item::RemovePrivateAsIngress<'a>;
                scalar next_hop_unchanged("next_hop_unchanged", 15) -> bool;
                scalar update_source("update_source", 16) -> &'a str;
                scalar route_reflector_client("route_reflector_client", 17) -> bool;
                scalar bfd("bfd", 18) -> bool;
                model bfd_timers("bfd_timers", 19) -> item::BfdTimers<'a>;
                scalar ebgp_multihop("ebgp_multihop", 20) -> i64;
                scalar next_hop_peer("next_hop_peer", 21) -> bool;
                scalar next_hop_self("next_hop_self", 22) -> bool;
                scalar password_type("password_type", 23) -> &'a str;
                scalar passive("passive", 24) -> bool;
                model default_originate("default_originate", 25) -> item::DefaultOriginate<'a>;
                scalar enforce_first_as("enforce_first_as", 26) -> bool;
                scalar send_community("send_community", 27) -> &'a str;
                scalar maximum_routes("maximum_routes", 28) -> i64;
                scalar maximum_routes_warning_limit("maximum_routes_warning_limit", 29) -> &'a str;
                scalar maximum_routes_warning_only("maximum_routes_warning_only", 30) -> bool;
                model maximum_accepted_routes("maximum_accepted_routes", 31) -> item::MaximumAcceptedRoutes<'a>;
                model missing_policy("missing_policy", 32) -> item::MissingPolicy<'a>;
                model link_bandwidth("link_bandwidth", 33) -> item::LinkBandwidth<'a>;
                model allowas_in("allowas_in", 34) -> item::AllowasIn<'a>;
                scalar weight("weight", 35) -> i64;
                scalar timers("timers", 36) -> &'a str;
                model rib_in_pre_policy_retain("rib_in_pre_policy_retain", 37) -> item::RibInPrePolicyRetain<'a>;
                scalar route_map_in("route_map_in", 38) -> &'a str;
                scalar route_map_out("route_map_out", 39) -> &'a str;
                scalar peer_tag_in("peer_tag_in", 40) -> &'a str;
                scalar peer_tag_out_discard("peer_tag_out_discard", 41) -> &'a str;
                scalar session_tracker("session_tracker", 42) -> &'a str;
                model shared_secret("shared_secret", 43) -> item::SharedSecret<'a>;
                scalar ttl_maximum_hops("ttl_maximum_hops", 44) -> i64;
                scalar maximum_advertised_routes("maximum_advertised_routes", 45) -> i64;
                scalar maximum_advertised_routes_warning_limit("maximum_advertised_routes_warning_limit", 46) -> &'a str;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Nodes {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AddressFamilyIpv4 {
                    scalar activate("activate", 0) -> bool;
                    scalar route_map_in("route_map_in", 1) -> &'a str;
                    scalar route_map_out("route_map_out", 2) -> &'a str;
                    scalar rcf_in("rcf_in", 3) -> &'a str;
                    scalar rcf_out("rcf_out", 4) -> &'a str;
                    model default_originate("default_originate", 5) -> super::super::super::super::super::eos_cli_config_gen::router_bgp::address_family_ipv4::peer_groups::item::DefaultOriginate<'a>;
                    model next_hop("next_hop", 6) -> super::super::super::super::super::eos_cli_config_gen::router_bgp::address_family_ipv4::peer_groups::item::NextHop<'a>;
                    scalar prefix_list_in("prefix_list_in", 7) -> &'a str;
                    scalar prefix_list_out("prefix_list_out", 8) -> &'a str;
                }
            }

            pub mod address_family_ipv4 {
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AddressFamilyIpv6 {
                    scalar activate("activate", 0) -> bool;
                    scalar route_map_in("route_map_in", 1) -> &'a str;
                    scalar route_map_out("route_map_out", 2) -> &'a str;
                    scalar rcf_in("rcf_in", 3) -> &'a str;
                    scalar rcf_out("rcf_out", 4) -> &'a str;
                    model default_originate("default_originate", 5) -> address_family_ipv6::DefaultOriginate<'a>;
                    scalar prefix_list_in("prefix_list_in", 6) -> &'a str;
                    scalar prefix_list_out("prefix_list_out", 7) -> &'a str;
                }
            }

            pub mod address_family_ipv6 {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct DefaultOriginate {
                        scalar enabled("enabled", 0) -> bool;
                        scalar always("always", 1) -> bool;
                        scalar route_map("route_map", 2) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct ListenRanges {
                    model item (0) -> listen_ranges::Item<'a>;
                }
            }

            pub mod listen_ranges {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar prefix("prefix", 0) -> &'a str;
                        scalar remote_as("remote_as", 1) -> &'a str;
                        scalar peer_id_include_router_id("peer_id_include_router_id", 2) -> bool;
                        scalar peer_filter("peer_filter", 3) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Metadata {
                    scalar field_type("type", 0) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AsPath {
                    scalar remote_as_replace_out("remote_as_replace_out", 0) -> bool;
                    scalar prepend_own_disabled("prepend_own_disabled", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct RemovePrivateAs {
                    scalar enabled("enabled", 0) -> bool;
                    scalar all("all", 1) -> bool;
                    scalar replace_as("replace_as", 2) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct RemovePrivateAsIngress {
                    scalar enabled("enabled", 0) -> bool;
                    scalar replace_as("replace_as", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct BfdTimers {
                    scalar interval("interval", 0) -> i64;
                    scalar min_rx("min_rx", 1) -> i64;
                    scalar multiplier("multiplier", 2) -> i64;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct DefaultOriginate {
                    scalar enabled("enabled", 0) -> bool;
                    scalar always("always", 1) -> bool;
                    scalar route_map("route_map", 2) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MaximumAcceptedRoutes {
                    scalar limit("limit", 0) -> i64;
                    model warning_limit("warning_limit", 1) -> maximum_accepted_routes::WarningLimit<'a>;
                }
            }

            pub mod maximum_accepted_routes {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct WarningLimit {
                        scalar count("count", 0) -> i64;
                        scalar percent("percent", 1) -> i64;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MissingPolicy {
                    model direction_in("direction_in", 0) -> missing_policy::DirectionIn<'a>;
                    model direction_out("direction_out", 1) -> missing_policy::DirectionOut<'a>;
                }
            }

            pub mod missing_policy {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct DirectionIn {
                        scalar action("action", 0) -> &'a str;
                        scalar include_community_list("include_community_list", 1) -> bool;
                        scalar include_prefix_list("include_prefix_list", 2) -> bool;
                        scalar include_sub_route_map("include_sub_route_map", 3) -> bool;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct DirectionOut {
                        scalar action("action", 0) -> &'a str;
                        scalar include_community_list("include_community_list", 1) -> bool;
                        scalar include_prefix_list("include_prefix_list", 2) -> bool;
                        scalar include_sub_route_map("include_sub_route_map", 3) -> bool;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct LinkBandwidth {
                    scalar enabled("enabled", 0) -> bool;
                    scalar default("default", 1) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AllowasIn {
                    scalar enabled("enabled", 0) -> bool;
                    scalar times("times", 1) -> i64;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct RibInPrePolicyRetain {
                    scalar enabled("enabled", 0) -> bool;
                    scalar all("all", 1) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct SharedSecret {
                    scalar profile("profile", 0) -> &'a str;
                    scalar hash_algorithm("hash_algorithm", 1) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct IgmpSnooping {
            model querier("querier", 0) -> igmp_snooping::Querier<'a>;
            scalar fast_leave("fast_leave", 1) -> bool;
        }
    }

    pub mod igmp_snooping {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Querier {
                scalar enabled("enabled", 0) -> bool;
                scalar source_address("source_address", 1) -> &'a str;
                scalar version("version", 2) -> i64;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct EvpnL2Multicast {
            scalar enabled("enabled", 0) -> bool;
            scalar underlay_l2_multicast_group_ipv4_pool("underlay_l2_multicast_group_ipv4_pool", 1) -> &'a str;
            scalar underlay_l2_multicast_group_ipv4_pool_offset("underlay_l2_multicast_group_ipv4_pool_offset", 2) -> i64;
            scalar fast_leave("fast_leave", 3) -> bool;
            scalar always_redistribute_igmp("always_redistribute_igmp", 4) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct VxlanFloodMulticast {
            scalar enabled("enabled", 0) -> bool;
            scalar underlay_l2_multicast_group_ipv4_pool("underlay_l2_multicast_group_ipv4_pool", 1) -> &'a str;
            scalar underlay_l2_multicast_group_ipv4_pool_offset("underlay_l2_multicast_group_ipv4_pool_offset", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct EvpnL3Multicast {
            scalar enabled("enabled", 0) -> bool;
            scalar evpn_underlay_l3_multicast_group_ipv4_pool("evpn_underlay_l3_multicast_group_ipv4_pool", 1) -> &'a str;
            scalar evpn_underlay_l3_multicast_group_ipv4_pool_offset("evpn_underlay_l3_multicast_group_ipv4_pool_offset", 2) -> i64;
            model evpn_peg("evpn_peg", 3) -> evpn_l3_multicast::EvpnPeg<'a>;
        }
    }

    pub mod evpn_l3_multicast {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct EvpnPeg {
                model item (0) -> evpn_peg::Item<'a>;
            }
        }

        pub mod evpn_peg {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    model nodes("nodes", 0) -> item::Nodes<'a>;
                    scalar transit("transit", 1) -> bool;
                }
            }

            pub mod item {

                ::validation::define_archive_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Nodes {
                        scalar item (0) -> &'a str;
                    }
                }
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PimRpAddresses {
            model item (0) -> pim_rp_addresses::Item<'a>;
        }
    }

    pub mod pim_rp_addresses {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                model rps("rps", 0) -> item::Rps<'a>;
                model nodes("nodes", 1) -> item::Nodes<'a>;
                model groups("groups", 2) -> item::Groups<'a>;
                scalar access_list_name("access_list_name", 3) -> &'a str;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Rps {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Nodes {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Groups {
                    scalar item (0) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct IgmpSnoopingQuerier {
            scalar enabled("enabled", 0) -> bool;
            scalar source_address("source_address", 1) -> &'a str;
            scalar version("version", 2) -> i64;
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Vrfs {
            model item (0) -> vrfs::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod vrfs {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                model address_families("address_families", 1) -> item::AddressFamilies<'a>;
                scalar description("description", 2) -> &'a str;
                scalar vrf_vni("vrf_vni", 3) -> i64;
                scalar vrf_id("vrf_id", 4) -> i64;
                scalar rd_override("rd_override", 5) -> &'a str;
                scalar rt_override("rt_override", 6) -> &'a str;
                scalar rt_import("rt_import", 7) -> bool;
                scalar rt_export("rt_export", 8) -> bool;
                scalar rt_import_evpn_remote("rt_import_evpn_remote", 9) -> bool;
                scalar rt_export_evpn_remote("rt_export_evpn_remote", 10) -> bool;
                scalar evpn_vlan_bundle("evpn_vlan_bundle", 11) -> &'a str;
                scalar mlag_ibgp_peering_ipv4_pool("mlag_ibgp_peering_ipv4_pool", 12) -> &'a str;
                scalar mlag_ibgp_peering_ipv6_pool("mlag_ibgp_peering_ipv6_pool", 13) -> &'a str;
                model ip_helpers("ip_helpers", 14) -> item::IpHelpers<'a>;
                scalar enable_mlag_ibgp_peering_vrfs("enable_mlag_ibgp_peering_vrfs", 15) -> bool;
                scalar redistribute_mlag_ibgp_peering_vrfs("redistribute_mlag_ibgp_peering_vrfs", 16) -> bool;
                scalar mlag_ibgp_peering_vlan("mlag_ibgp_peering_vlan", 17) -> i64;
                model vtep_diagnostic("vtep_diagnostic", 18) -> item::VtepDiagnostic<'a>;
                model ospf("ospf", 19) -> item::Ospf<'a>;
                scalar redistribute_ospf("redistribute_ospf", 20) -> bool;
                model evpn_l3_multicast("evpn_l3_multicast", 21) -> item::EvpnL3Multicast<'a>;
                model pim_rp_addresses("pim_rp_addresses", 22) -> item::PimRpAddresses<'a>;
                scalar evpn_l2_multi_domain("evpn_l2_multi_domain", 23) -> bool;
                model svis("svis", 24) -> item::Svis<'a>;
                model l3_interfaces("l3_interfaces", 25) -> item::L3Interfaces<'a>;
                model l3_port_channels("l3_port_channels", 26) -> item::L3PortChannels<'a>;
                model loopbacks("loopbacks", 27) -> item::Loopbacks<'a>;
                model static_routes("static_routes", 28) -> item::StaticRoutes<'a>;
                model ipv6_static_routes("ipv6_static_routes", 29) -> item::Ipv6StaticRoutes<'a>;
                scalar redistribute_static("redistribute_static", 30) -> bool;
                scalar redistribute_connected("redistribute_connected", 31) -> bool;
                model static_arp_entries("static_arp_entries", 32) -> item::StaticArpEntries<'a>;
                model bgp_peers("bgp_peers", 33) -> item::BgpPeers<'a>;
                model bgp("bgp", 34) -> item::Bgp<'a>;
                model bgp_peer_groups("bgp_peer_groups", 35) -> item::BgpPeerGroups<'a>;
                model additional_route_targets("additional_route_targets", 36) -> item::AdditionalRouteTargets<'a>;
                model aggregate_addresses("aggregate_addresses", 37) -> item::AggregateAddresses<'a>;
                scalar validate_bgp_peers("validate_bgp_peers", 38) -> bool;
                scalar raw_eos_cli("raw_eos_cli", 39) -> &'a str;
                model structured_config("structured_config", 40) -> super::super::super::super::eos_cli_config_gen::EosCliConfigGen<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AddressFamilies {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct IpHelpers {
                    model item (0) -> ip_helpers::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod ip_helpers {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar ip_helper("ip_helper", 0) -> &'a str;
                        scalar source_interface("source_interface", 1) -> &'a str;
                        scalar source_vrf("source_vrf", 2) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct VtepDiagnostic {
                    scalar loopback("loopback", 0) -> i64;
                    scalar loopback_description("loopback_description", 1) -> &'a str;
                    scalar loopback_ip_range("loopback_ip_range", 2) -> &'a str;
                    scalar loopback_ipv6_range("loopback_ipv6_range", 3) -> &'a str;
                    model loopback_ip_pools("loopback_ip_pools", 4) -> vtep_diagnostic::LoopbackIpPools<'a>;
                    scalar hardware_forwarding("hardware_forwarding", 5) -> bool;
                }
            }

            pub mod vtep_diagnostic {

                ::validation::define_archive_indexed_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct LoopbackIpPools {
                        model item (0) -> loopback_ip_pools::Item<'a>;
                        primary_key_fields: [0];
                    }
                }

                pub mod loopback_ip_pools {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            scalar pod("pod", 0) -> &'a str;
                            scalar ipv4_pool("ipv4_pool", 1) -> &'a str;
                            scalar ipv6_pool("ipv6_pool", 2) -> &'a str;
                        }
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Ospf {
                    scalar enabled("enabled", 0) -> bool;
                    scalar process_id("process_id", 1) -> i64;
                    scalar router_id("router_id", 2) -> &'a str;
                    scalar max_lsa("max_lsa", 3) -> i64;
                    scalar bfd("bfd", 4) -> bool;
                    model redistribute_bgp("redistribute_bgp", 5) -> ospf::RedistributeBgp<'a>;
                    model redistribute_connected("redistribute_connected", 6) -> ospf::RedistributeConnected<'a>;
                    scalar authentication("authentication", 7) -> &'a str;
                    scalar cleartext_simple_auth_key("cleartext_simple_auth_key", 8) -> &'a str;
                    model message_digest_keys("message_digest_keys", 9) -> ospf::MessageDigestKeys<'a>;
                    model nodes("nodes", 10) -> ospf::Nodes<'a>;
                    model structured_config("structured_config", 11) -> super::super::super::super::super::eos_cli_config_gen::router_ospf::process_ids::Item<'a>;
                }
            }

            pub mod ospf {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct RedistributeBgp {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct RedistributeConnected {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                    }
                }

                ::validation::define_archive_indexed_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct MessageDigestKeys {
                        model item (0) -> message_digest_keys::Item<'a>;
                        primary_key_fields: [0];
                    }
                }

                pub mod message_digest_keys {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            scalar id("id", 0) -> i64;
                            scalar hash_algorithm("hash_algorithm", 1) -> &'a str;
                            scalar cleartext_key("cleartext_key", 2) -> &'a str;
                        }
                    }
                }

                ::validation::define_archive_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Nodes {
                        scalar item (0) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct EvpnL3Multicast {
                    scalar enabled("enabled", 0) -> bool;
                    scalar evpn_underlay_l3_multicast_group("evpn_underlay_l3_multicast_group", 1) -> &'a str;
                    model evpn_peg("evpn_peg", 2) -> evpn_l3_multicast::EvpnPeg<'a>;
                }
            }

            pub mod evpn_l3_multicast {

                ::validation::define_archive_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct EvpnPeg {
                        model item (0) -> evpn_peg::Item<'a>;
                    }
                }

                pub mod evpn_peg {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            model nodes("nodes", 0) -> item::Nodes<'a>;
                            scalar transit("transit", 1) -> bool;
                        }
                    }

                    pub mod item {

                        ::validation::define_archive_list_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Nodes {
                                scalar item (0) -> &'a str;
                            }
                        }
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct PimRpAddresses {
                    model item (0) -> pim_rp_addresses::Item<'a>;
                }
            }

            pub mod pim_rp_addresses {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        model rps("rps", 0) -> item::Rps<'a>;
                        model nodes("nodes", 1) -> item::Nodes<'a>;
                        model groups("groups", 2) -> item::Groups<'a>;
                        scalar access_list_name("access_list_name", 3) -> &'a str;
                    }
                }

                pub mod item {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Rps {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Nodes {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Groups {
                            scalar item (0) -> &'a str;
                        }
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Svis {
                    model item (0) -> svis::Item<'a>;
                }
            }

            pub mod svis {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar id("id", 0) -> i64;
                        scalar name("name", 1) -> &'a str;
                        model address_locking("address_locking", 2) -> super::super::super::super::super::super::eos_cli_config_gen::vlans::item::address_locking::AddressFamily<'a>;
                        scalar profile("profile", 3) -> &'a str;
                        model tags("tags", 4) -> item::Tags<'a>;
                        scalar evpn_vlan_bundle("evpn_vlan_bundle", 5) -> &'a str;
                        model nodes("nodes", 6) -> item::Nodes<'a>;
                        scalar enabled("enabled", 7) -> bool;
                        scalar description("description", 8) -> &'a str;
                        scalar arp_gratuitous_accept("arp_gratuitous_accept", 9) -> bool;
                        scalar ip_address("ip_address", 10) -> &'a str;
                        model ip_address_secondaries("ip_address_secondaries", 11) -> item::IpAddressSecondaries<'a>;
                        scalar ipv6_address("ipv6_address", 12) -> &'a str;
                        scalar ipv6_enable("ipv6_enable", 13) -> bool;
                        scalar ip_address_virtual("ip_address_virtual", 14) -> &'a str;
                        model ipv6_address_virtuals("ipv6_address_virtuals", 15) -> item::Ipv6AddressVirtuals<'a>;
                        model ipv6_nd("ipv6_nd", 16) -> item::Ipv6Nd<'a>;
                        model ipv6_dhcp_relay("ipv6_dhcp_relay", 17) -> item::Ipv6DhcpRelay<'a>;
                        model ip_address_virtual_secondaries("ip_address_virtual_secondaries", 18) -> item::IpAddressVirtualSecondaries<'a>;
                        model ip_virtual_router_addresses("ip_virtual_router_addresses", 19) -> item::IpVirtualRouterAddresses<'a>;
                        model ipv6_virtual_router_addresses("ipv6_virtual_router_addresses", 20) -> item::Ipv6VirtualRouterAddresses<'a>;
                        scalar ipv4_acl_in("ipv4_acl_in", 21) -> &'a str;
                        scalar ipv4_acl_out("ipv4_acl_out", 22) -> &'a str;
                        scalar ipv6_acl_in("ipv6_acl_in", 23) -> &'a str;
                        scalar ipv6_acl_out("ipv6_acl_out", 24) -> &'a str;
                        model ip_helpers("ip_helpers", 25) -> item::IpHelpers<'a>;
                        model static_routes("static_routes", 26) -> item::StaticRoutes<'a>;
                        model ipv6_static_routes("ipv6_static_routes", 27) -> item::Ipv6StaticRoutes<'a>;
                        scalar vni_override("vni_override", 28) -> i64;
                        scalar rt_override("rt_override", 29) -> &'a str;
                        scalar rd_override("rd_override", 30) -> &'a str;
                        model trunk_groups("trunk_groups", 31) -> item::TrunkGroups<'a>;
                        model evpn_l2_multicast("evpn_l2_multicast", 32) -> item::EvpnL2Multicast<'a>;
                        scalar evpn_redistribute_router_mac_system("evpn_redistribute_router_mac_system", 33) -> bool;
                        model vxlan_flood_multicast("vxlan_flood_multicast", 34) -> item::VxlanFloodMulticast<'a>;
                        model evpn_l3_multicast("evpn_l3_multicast", 35) -> item::EvpnL3Multicast<'a>;
                        model igmp_snooping("igmp_snooping", 36) -> item::IgmpSnooping<'a>;
                        scalar igmp_snooping_enabled("igmp_snooping_enabled", 37) -> bool;
                        model igmp_snooping_querier("igmp_snooping_querier", 38) -> item::IgmpSnoopingQuerier<'a>;
                        scalar vxlan("vxlan", 39) -> bool;
                        scalar spanning_tree_priority("spanning_tree_priority", 40) -> i64;
                        scalar mtu("mtu", 41) -> i64;
                        model ospf("ospf", 42) -> item::Ospf<'a>;
                        model bgp("bgp", 43) -> item::Bgp<'a>;
                        scalar raw_eos_cli("raw_eos_cli", 44) -> &'a str;
                        model structured_config("structured_config", 45) -> super::super::super::super::super::super::eos_cli_config_gen::vlan_interfaces::Item<'a>;
                        scalar evpn_l2_multi_domain("evpn_l2_multi_domain", 46) -> bool;
                    }
                }

                pub mod item {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Tags {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_indexed_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Nodes {
                            model item (0) -> nodes::Item<'a>;
                            primary_key_fields: [0];
                        }
                    }

                    pub mod nodes {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Item {
                                scalar node("node", 0) -> &'a str;
                                model tags("tags", 1) -> item::Tags<'a>;
                                scalar name("name", 2) -> &'a str;
                                scalar enabled("enabled", 3) -> bool;
                                scalar description("description", 4) -> &'a str;
                                scalar arp_gratuitous_accept("arp_gratuitous_accept", 5) -> bool;
                                scalar ip_address("ip_address", 6) -> &'a str;
                                model ip_address_secondaries("ip_address_secondaries", 7) -> item::IpAddressSecondaries<'a>;
                                scalar ipv6_address("ipv6_address", 8) -> &'a str;
                                scalar ipv6_enable("ipv6_enable", 9) -> bool;
                                scalar ip_address_virtual("ip_address_virtual", 10) -> &'a str;
                                model ipv6_address_virtuals("ipv6_address_virtuals", 11) -> item::Ipv6AddressVirtuals<'a>;
                                model ipv6_nd("ipv6_nd", 12) -> item::Ipv6Nd<'a>;
                                model ipv6_dhcp_relay("ipv6_dhcp_relay", 13) -> item::Ipv6DhcpRelay<'a>;
                                model ip_address_virtual_secondaries("ip_address_virtual_secondaries", 14) -> item::IpAddressVirtualSecondaries<'a>;
                                model ip_virtual_router_addresses("ip_virtual_router_addresses", 15) -> item::IpVirtualRouterAddresses<'a>;
                                model ipv6_virtual_router_addresses("ipv6_virtual_router_addresses", 16) -> item::Ipv6VirtualRouterAddresses<'a>;
                                scalar ipv4_acl_in("ipv4_acl_in", 17) -> &'a str;
                                scalar ipv4_acl_out("ipv4_acl_out", 18) -> &'a str;
                                scalar ipv6_acl_in("ipv6_acl_in", 19) -> &'a str;
                                scalar ipv6_acl_out("ipv6_acl_out", 20) -> &'a str;
                                model ip_helpers("ip_helpers", 21) -> item::IpHelpers<'a>;
                                model static_routes("static_routes", 22) -> item::StaticRoutes<'a>;
                                model ipv6_static_routes("ipv6_static_routes", 23) -> item::Ipv6StaticRoutes<'a>;
                                scalar vni_override("vni_override", 24) -> i64;
                                scalar rt_override("rt_override", 25) -> &'a str;
                                scalar rd_override("rd_override", 26) -> &'a str;
                                model trunk_groups("trunk_groups", 27) -> item::TrunkGroups<'a>;
                                model evpn_l2_multicast("evpn_l2_multicast", 28) -> item::EvpnL2Multicast<'a>;
                                scalar evpn_redistribute_router_mac_system("evpn_redistribute_router_mac_system", 29) -> bool;
                                model vxlan_flood_multicast("vxlan_flood_multicast", 30) -> item::VxlanFloodMulticast<'a>;
                                model evpn_l3_multicast("evpn_l3_multicast", 31) -> item::EvpnL3Multicast<'a>;
                                model igmp_snooping("igmp_snooping", 32) -> item::IgmpSnooping<'a>;
                                scalar igmp_snooping_enabled("igmp_snooping_enabled", 33) -> bool;
                                model igmp_snooping_querier("igmp_snooping_querier", 34) -> item::IgmpSnoopingQuerier<'a>;
                                scalar vxlan("vxlan", 35) -> bool;
                                scalar spanning_tree_priority("spanning_tree_priority", 36) -> i64;
                                scalar mtu("mtu", 37) -> i64;
                                model ospf("ospf", 38) -> item::Ospf<'a>;
                                model bgp("bgp", 39) -> item::Bgp<'a>;
                                scalar raw_eos_cli("raw_eos_cli", 40) -> &'a str;
                                model structured_config("structured_config", 41) -> super::super::super::super::super::super::super::super::eos_cli_config_gen::vlan_interfaces::Item<'a>;
                                scalar evpn_l2_multi_domain("evpn_l2_multi_domain", 42) -> bool;
                            }
                        }

                        pub mod item {

                            ::validation::define_archive_list_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct Tags {
                                    scalar item (0) -> &'a str;
                                }
                            }

                            ::validation::define_archive_list_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct IpAddressSecondaries {
                                    scalar item (0) -> &'a str;
                                }
                            }

                            ::validation::define_archive_list_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct Ipv6AddressVirtuals {
                                    scalar item (0) -> &'a str;
                                }
                            }

                            ::validation::define_archive_dict_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct Ipv6Nd {
                                    scalar advertise_ipv6_address_virtuals("advertise_ipv6_address_virtuals", 0) -> bool;
                                    scalar valid_lifetime("valid_lifetime", 1) -> &'a str;
                                    scalar preferred_lifetime("preferred_lifetime", 2) -> &'a str;
                                    model ra_dns_servers("ra_dns_servers", 3) -> ipv6_nd::RaDnsServers<'a>;
                                }
                            }

                            pub mod ipv6_nd {

                                ::validation::define_archive_dict_view! {
                                    #[derive(Clone, Copy, Debug)]
                                    pub struct RaDnsServers {
                                        model servers("servers", 0) -> ra_dns_servers::Servers<'a>;
                                        scalar dns_servers_lifetime("dns_servers_lifetime", 1) -> i64;
                                    }
                                }

                                pub mod ra_dns_servers {

                                    ::validation::define_archive_indexed_list_view! {
                                        #[derive(Clone, Copy, Debug)]
                                        pub struct Servers {
                                            model item (0) -> servers::Item<'a>;
                                            primary_key_fields: [0];
                                        }
                                    }

                                    pub mod servers {

                                        ::validation::define_archive_dict_view! {
                                            #[derive(Clone, Copy, Debug)]
                                            pub struct Item {
                                                scalar address("address", 0) -> &'a str;
                                                scalar lifetime("lifetime", 1) -> i64;
                                            }
                                        }
                                    }
                                }
                            }

                            ::validation::define_archive_dict_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct Ipv6DhcpRelay {
                                    model destinations("destinations", 0) -> ipv6_dhcp_relay::Destinations<'a>;
                                }
                            }

                            pub mod ipv6_dhcp_relay {

                                ::validation::define_archive_indexed_list_view! {
                                    #[derive(Clone, Copy, Debug)]
                                    pub struct Destinations {
                                        model item (0) -> destinations::Item<'a>;
                                        primary_key_fields: [0];
                                    }
                                }

                                pub mod destinations {

                                    ::validation::define_archive_dict_view! {
                                        #[derive(Clone, Copy, Debug)]
                                        pub struct Item {
                                            scalar address("address", 0) -> &'a str;
                                            scalar vrf("vrf", 1) -> &'a str;
                                            scalar local_interface("local_interface", 2) -> &'a str;
                                            scalar source_address("source_address", 3) -> &'a str;
                                            scalar link_address("link_address", 4) -> &'a str;
                                        }
                                    }
                                }
                            }

                            ::validation::define_archive_list_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct IpAddressVirtualSecondaries {
                                    scalar item (0) -> &'a str;
                                }
                            }

                            ::validation::define_archive_list_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct IpVirtualRouterAddresses {
                                    scalar item (0) -> &'a str;
                                }
                            }

                            ::validation::define_archive_list_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct Ipv6VirtualRouterAddresses {
                                    scalar item (0) -> &'a str;
                                }
                            }

                            ::validation::define_archive_indexed_list_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct IpHelpers {
                                    model item (0) -> ip_helpers::Item<'a>;
                                    primary_key_fields: [0];
                                }
                            }

                            pub mod ip_helpers {

                                ::validation::define_archive_dict_view! {
                                    #[derive(Clone, Copy, Debug)]
                                    pub struct Item {
                                        scalar ip_helper("ip_helper", 0) -> &'a str;
                                        scalar source_interface("source_interface", 1) -> &'a str;
                                        scalar source_vrf("source_vrf", 2) -> &'a str;
                                    }
                                }
                            }

                            ::validation::define_archive_list_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct StaticRoutes {
                                    model item (0) -> static_routes::Item<'a>;
                                }
                            }

                            pub mod static_routes {

                                ::validation::define_archive_dict_view! {
                                    #[derive(Clone, Copy, Debug)]
                                    pub struct Item {
                                        scalar prefix("prefix", 0) -> &'a str;
                                        scalar next_hop("next_hop", 1) -> &'a str;
                                        scalar track_bfd("track_bfd", 2) -> bool;
                                        scalar distance("distance", 3) -> i64;
                                        scalar tag("tag", 4) -> i64;
                                        scalar name("name", 5) -> &'a str;
                                        scalar metric("metric", 6) -> i64;
                                        scalar interface("interface", 7) -> &'a str;
                                    }
                                }
                            }

                            ::validation::define_archive_list_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct Ipv6StaticRoutes {
                                    model item (0) -> ipv6_static_routes::Item<'a>;
                                }
                            }

                            pub mod ipv6_static_routes {

                                ::validation::define_archive_dict_view! {
                                    #[derive(Clone, Copy, Debug)]
                                    pub struct Item {
                                        scalar prefix("prefix", 0) -> &'a str;
                                        scalar next_hop("next_hop", 1) -> &'a str;
                                        scalar track_bfd("track_bfd", 2) -> bool;
                                        scalar distance("distance", 3) -> i64;
                                        scalar tag("tag", 4) -> i64;
                                        scalar name("name", 5) -> &'a str;
                                        scalar metric("metric", 6) -> i64;
                                        scalar interface("interface", 7) -> &'a str;
                                    }
                                }
                            }

                            ::validation::define_archive_list_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct TrunkGroups {
                                    scalar item (0) -> &'a str;
                                }
                            }

                            ::validation::define_archive_dict_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct EvpnL2Multicast {
                                    scalar enabled("enabled", 0) -> bool;
                                    scalar always_redistribute_igmp("always_redistribute_igmp", 1) -> bool;
                                }
                            }

                            ::validation::define_archive_dict_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct VxlanFloodMulticast {
                                    scalar enabled("enabled", 0) -> bool;
                                    scalar underlay_multicast_group("underlay_multicast_group", 1) -> &'a str;
                                }
                            }

                            ::validation::define_archive_dict_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct EvpnL3Multicast {
                                    scalar enabled("enabled", 0) -> bool;
                                }
                            }

                            ::validation::define_archive_dict_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct IgmpSnooping {
                                    scalar enabled("enabled", 0) -> bool;
                                    model querier("querier", 1) -> igmp_snooping::Querier<'a>;
                                    scalar fast_leave("fast_leave", 2) -> bool;
                                }
                            }

                            pub mod igmp_snooping {

                                ::validation::define_archive_dict_view! {
                                    #[derive(Clone, Copy, Debug)]
                                    pub struct Querier {
                                        scalar enabled("enabled", 0) -> bool;
                                        scalar source_address("source_address", 1) -> &'a str;
                                        scalar version("version", 2) -> i64;
                                    }
                                }
                            }

                            ::validation::define_archive_dict_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct IgmpSnoopingQuerier {
                                    scalar enabled("enabled", 0) -> bool;
                                    scalar source_address("source_address", 1) -> &'a str;
                                    scalar version("version", 2) -> i64;
                                    scalar fast_leave("fast_leave", 3) -> bool;
                                }
                            }

                            ::validation::define_archive_dict_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct Ospf {
                                    scalar enabled("enabled", 0) -> bool;
                                    scalar point_to_point("point_to_point", 1) -> bool;
                                    scalar area("area", 2) -> &'a str;
                                    scalar cost("cost", 3) -> i64;
                                    scalar authentication("authentication", 4) -> &'a str;
                                    scalar simple_auth_key("simple_auth_key", 5) -> &'a str;
                                    scalar cleartext_simple_auth_key("cleartext_simple_auth_key", 6) -> &'a str;
                                    model message_digest_keys("message_digest_keys", 7) -> ospf::MessageDigestKeys<'a>;
                                }
                            }

                            pub mod ospf {

                                ::validation::define_archive_indexed_list_view! {
                                    #[derive(Clone, Copy, Debug)]
                                    pub struct MessageDigestKeys {
                                        model item (0) -> message_digest_keys::Item<'a>;
                                        primary_key_fields: [0];
                                    }
                                }

                                pub mod message_digest_keys {

                                    ::validation::define_archive_dict_view! {
                                        #[derive(Clone, Copy, Debug)]
                                        pub struct Item {
                                            scalar id("id", 0) -> i64;
                                            scalar hash_algorithm("hash_algorithm", 1) -> &'a str;
                                            scalar key("key", 2) -> &'a str;
                                            scalar cleartext_key("cleartext_key", 3) -> &'a str;
                                        }
                                    }
                                }
                            }

                            ::validation::define_archive_dict_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct Bgp {
                                    model structured_config("structured_config", 0) -> super::super::super::super::super::super::super::super::super::eos_cli_config_gen::router_bgp::vlans::Item<'a>;
                                    scalar raw_eos_cli("raw_eos_cli", 1) -> &'a str;
                                }
                            }

                            pub mod bgp {
                            }
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct IpAddressSecondaries {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Ipv6AddressVirtuals {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Ipv6Nd {
                            scalar advertise_ipv6_address_virtuals("advertise_ipv6_address_virtuals", 0) -> bool;
                            scalar valid_lifetime("valid_lifetime", 1) -> &'a str;
                            scalar preferred_lifetime("preferred_lifetime", 2) -> &'a str;
                            model ra_dns_servers("ra_dns_servers", 3) -> ipv6_nd::RaDnsServers<'a>;
                        }
                    }

                    pub mod ipv6_nd {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct RaDnsServers {
                                model servers("servers", 0) -> ra_dns_servers::Servers<'a>;
                                scalar dns_servers_lifetime("dns_servers_lifetime", 1) -> i64;
                            }
                        }

                        pub mod ra_dns_servers {

                            ::validation::define_archive_indexed_list_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct Servers {
                                    model item (0) -> servers::Item<'a>;
                                    primary_key_fields: [0];
                                }
                            }

                            pub mod servers {

                                ::validation::define_archive_dict_view! {
                                    #[derive(Clone, Copy, Debug)]
                                    pub struct Item {
                                        scalar address("address", 0) -> &'a str;
                                        scalar lifetime("lifetime", 1) -> i64;
                                    }
                                }
                            }
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Ipv6DhcpRelay {
                            model destinations("destinations", 0) -> ipv6_dhcp_relay::Destinations<'a>;
                        }
                    }

                    pub mod ipv6_dhcp_relay {

                        ::validation::define_archive_indexed_list_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Destinations {
                                model item (0) -> destinations::Item<'a>;
                                primary_key_fields: [0];
                            }
                        }

                        pub mod destinations {

                            ::validation::define_archive_dict_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct Item {
                                    scalar address("address", 0) -> &'a str;
                                    scalar vrf("vrf", 1) -> &'a str;
                                    scalar local_interface("local_interface", 2) -> &'a str;
                                    scalar source_address("source_address", 3) -> &'a str;
                                    scalar link_address("link_address", 4) -> &'a str;
                                }
                            }
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct IpAddressVirtualSecondaries {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct IpVirtualRouterAddresses {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Ipv6VirtualRouterAddresses {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_indexed_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct IpHelpers {
                            model item (0) -> ip_helpers::Item<'a>;
                            primary_key_fields: [0];
                        }
                    }

                    pub mod ip_helpers {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Item {
                                scalar ip_helper("ip_helper", 0) -> &'a str;
                                scalar source_interface("source_interface", 1) -> &'a str;
                                scalar source_vrf("source_vrf", 2) -> &'a str;
                            }
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct StaticRoutes {
                            model item (0) -> static_routes::Item<'a>;
                        }
                    }

                    pub mod static_routes {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Item {
                                scalar prefix("prefix", 0) -> &'a str;
                                scalar next_hop("next_hop", 1) -> &'a str;
                                scalar track_bfd("track_bfd", 2) -> bool;
                                scalar distance("distance", 3) -> i64;
                                scalar tag("tag", 4) -> i64;
                                scalar name("name", 5) -> &'a str;
                                scalar metric("metric", 6) -> i64;
                                scalar interface("interface", 7) -> &'a str;
                            }
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Ipv6StaticRoutes {
                            model item (0) -> ipv6_static_routes::Item<'a>;
                        }
                    }

                    pub mod ipv6_static_routes {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Item {
                                scalar prefix("prefix", 0) -> &'a str;
                                scalar next_hop("next_hop", 1) -> &'a str;
                                scalar track_bfd("track_bfd", 2) -> bool;
                                scalar distance("distance", 3) -> i64;
                                scalar tag("tag", 4) -> i64;
                                scalar name("name", 5) -> &'a str;
                                scalar metric("metric", 6) -> i64;
                                scalar interface("interface", 7) -> &'a str;
                            }
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct TrunkGroups {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct EvpnL2Multicast {
                            scalar enabled("enabled", 0) -> bool;
                            scalar always_redistribute_igmp("always_redistribute_igmp", 1) -> bool;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct VxlanFloodMulticast {
                            scalar enabled("enabled", 0) -> bool;
                            scalar underlay_multicast_group("underlay_multicast_group", 1) -> &'a str;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct EvpnL3Multicast {
                            scalar enabled("enabled", 0) -> bool;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct IgmpSnooping {
                            scalar enabled("enabled", 0) -> bool;
                            model querier("querier", 1) -> igmp_snooping::Querier<'a>;
                            scalar fast_leave("fast_leave", 2) -> bool;
                        }
                    }

                    pub mod igmp_snooping {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Querier {
                                scalar enabled("enabled", 0) -> bool;
                                scalar source_address("source_address", 1) -> &'a str;
                                scalar version("version", 2) -> i64;
                            }
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct IgmpSnoopingQuerier {
                            scalar enabled("enabled", 0) -> bool;
                            scalar source_address("source_address", 1) -> &'a str;
                            scalar version("version", 2) -> i64;
                            scalar fast_leave("fast_leave", 3) -> bool;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Ospf {
                            scalar enabled("enabled", 0) -> bool;
                            scalar point_to_point("point_to_point", 1) -> bool;
                            scalar area("area", 2) -> &'a str;
                            scalar cost("cost", 3) -> i64;
                            scalar authentication("authentication", 4) -> &'a str;
                            scalar simple_auth_key("simple_auth_key", 5) -> &'a str;
                            scalar cleartext_simple_auth_key("cleartext_simple_auth_key", 6) -> &'a str;
                            model message_digest_keys("message_digest_keys", 7) -> ospf::MessageDigestKeys<'a>;
                        }
                    }

                    pub mod ospf {

                        ::validation::define_archive_indexed_list_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct MessageDigestKeys {
                                model item (0) -> message_digest_keys::Item<'a>;
                                primary_key_fields: [0];
                            }
                        }

                        pub mod message_digest_keys {

                            ::validation::define_archive_dict_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct Item {
                                    scalar id("id", 0) -> i64;
                                    scalar hash_algorithm("hash_algorithm", 1) -> &'a str;
                                    scalar key("key", 2) -> &'a str;
                                    scalar cleartext_key("cleartext_key", 3) -> &'a str;
                                }
                            }
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Bgp {
                            model structured_config("structured_config", 0) -> super::super::super::super::super::super::super::eos_cli_config_gen::router_bgp::vlans::Item<'a>;
                            scalar raw_eos_cli("raw_eos_cli", 1) -> &'a str;
                        }
                    }

                    pub mod bgp {
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct L3Interfaces {
                    model item (0) -> l3_interfaces::Item<'a>;
                }
            }

            pub mod l3_interfaces {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        model interfaces("interfaces", 0) -> item::Interfaces<'a>;
                        model encapsulation_dot1q_vlan("encapsulation_dot1q_vlan", 1) -> item::EncapsulationDot1qVlan<'a>;
                        model ip_addresses("ip_addresses", 2) -> item::IpAddresses<'a>;
                        model ipv6_addresses("ipv6_addresses", 3) -> item::Ipv6Addresses<'a>;
                        model static_routes("static_routes", 4) -> item::StaticRoutes<'a>;
                        model ipv6_static_routes("ipv6_static_routes", 5) -> item::Ipv6StaticRoutes<'a>;
                        model nodes("nodes", 6) -> item::Nodes<'a>;
                        scalar arp_gratuitous_accept("arp_gratuitous_accept", 7) -> bool;
                        scalar description("description", 8) -> &'a str;
                        model descriptions("descriptions", 9) -> item::Descriptions<'a>;
                        scalar enabled("enabled", 10) -> bool;
                        scalar mtu("mtu", 11) -> i64;
                        scalar ipv4_acl_in("ipv4_acl_in", 12) -> &'a str;
                        scalar ipv4_acl_out("ipv4_acl_out", 13) -> &'a str;
                        scalar ipv6_acl_in("ipv6_acl_in", 14) -> &'a str;
                        scalar ipv6_acl_out("ipv6_acl_out", 15) -> &'a str;
                        model ospf("ospf", 16) -> item::Ospf<'a>;
                        model pim("pim", 17) -> item::Pim<'a>;
                        model flow_tracking("flow_tracking", 18) -> item::FlowTracking<'a>;
                        scalar sflow("sflow", 19) -> bool;
                        model monitor_sessions("monitor_sessions", 20) -> item::MonitorSessions<'a>;
                        model campus_link_type("campus_link_type", 21) -> item::CampusLinkType<'a>;
                        model structured_config("structured_config", 22) -> super::super::super::super::super::super::eos_cli_config_gen::ethernet_interfaces::Item<'a>;
                        scalar raw_eos_cli("raw_eos_cli", 23) -> &'a str;
                    }
                }

                pub mod item {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Interfaces {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct EncapsulationDot1qVlan {
                            scalar item (0) -> i64;
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct IpAddresses {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Ipv6Addresses {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct StaticRoutes {
                            model item (0) -> static_routes::Item<'a>;
                        }
                    }

                    pub mod static_routes {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Item {
                                scalar prefix("prefix", 0) -> &'a str;
                                scalar next_hop("next_hop", 1) -> &'a str;
                                scalar track_bfd("track_bfd", 2) -> bool;
                                scalar distance("distance", 3) -> i64;
                                scalar tag("tag", 4) -> i64;
                                scalar name("name", 5) -> &'a str;
                                scalar metric("metric", 6) -> i64;
                                scalar interface("interface", 7) -> &'a str;
                            }
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Ipv6StaticRoutes {
                            model item (0) -> ipv6_static_routes::Item<'a>;
                        }
                    }

                    pub mod ipv6_static_routes {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Item {
                                scalar prefix("prefix", 0) -> &'a str;
                                scalar next_hop("next_hop", 1) -> &'a str;
                                scalar track_bfd("track_bfd", 2) -> bool;
                                scalar distance("distance", 3) -> i64;
                                scalar tag("tag", 4) -> i64;
                                scalar name("name", 5) -> &'a str;
                                scalar metric("metric", 6) -> i64;
                                scalar interface("interface", 7) -> &'a str;
                            }
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Nodes {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Descriptions {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Ospf {
                            scalar enabled("enabled", 0) -> bool;
                            scalar point_to_point("point_to_point", 1) -> bool;
                            scalar area("area", 2) -> &'a str;
                            scalar cost("cost", 3) -> i64;
                            scalar authentication("authentication", 4) -> &'a str;
                            scalar simple_auth_key("simple_auth_key", 5) -> &'a str;
                            scalar cleartext_simple_auth_key("cleartext_simple_auth_key", 6) -> &'a str;
                            model message_digest_keys("message_digest_keys", 7) -> ospf::MessageDigestKeys<'a>;
                        }
                    }

                    pub mod ospf {

                        ::validation::define_archive_indexed_list_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct MessageDigestKeys {
                                model item (0) -> message_digest_keys::Item<'a>;
                                primary_key_fields: [0];
                            }
                        }

                        pub mod message_digest_keys {

                            ::validation::define_archive_dict_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct Item {
                                    scalar id("id", 0) -> i64;
                                    scalar hash_algorithm("hash_algorithm", 1) -> &'a str;
                                    scalar key("key", 2) -> &'a str;
                                    scalar cleartext_key("cleartext_key", 3) -> &'a str;
                                }
                            }
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Pim {
                            scalar enabled("enabled", 0) -> bool;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct FlowTracking {
                            scalar enabled("enabled", 0) -> bool;
                            scalar name("name", 1) -> &'a str;
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MonitorSessions {
                            model item (0) -> monitor_sessions::Item<'a>;
                        }
                    }

                    pub mod monitor_sessions {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Item {
                                scalar name("name", 0) -> &'a str;
                                scalar role("role", 1) -> &'a str;
                                model source_settings("source_settings", 2) -> item::SourceSettings<'a>;
                                model session_settings("session_settings", 3) -> item::SessionSettings<'a>;
                            }
                        }

                        pub mod item {

                            ::validation::define_archive_dict_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct SourceSettings {
                                    scalar direction("direction", 0) -> &'a str;
                                    model access_group("access_group", 1) -> source_settings::AccessGroup<'a>;
                                }
                            }

                            pub mod source_settings {

                                ::validation::define_archive_dict_view! {
                                    #[derive(Clone, Copy, Debug)]
                                    pub struct AccessGroup {
                                        scalar field_type("type", 0) -> &'a str;
                                        scalar name("name", 1) -> &'a str;
                                        scalar priority("priority", 2) -> i64;
                                    }
                                }
                            }

                            ::validation::define_archive_dict_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct SessionSettings {
                                    scalar encapsulation_gre_metadata_tx("encapsulation_gre_metadata_tx", 0) -> bool;
                                    scalar header_remove_size("header_remove_size", 1) -> i64;
                                    model access_group("access_group", 2) -> session_settings::AccessGroup<'a>;
                                    scalar rate_limit_per_ingress_chip("rate_limit_per_ingress_chip", 3) -> &'a str;
                                    scalar rate_limit_per_egress_chip("rate_limit_per_egress_chip", 4) -> &'a str;
                                    scalar sample("sample", 5) -> i64;
                                    model truncate("truncate", 6) -> session_settings::Truncate<'a>;
                                }
                            }

                            pub mod session_settings {

                                ::validation::define_archive_dict_view! {
                                    #[derive(Clone, Copy, Debug)]
                                    pub struct AccessGroup {
                                        scalar field_type("type", 0) -> &'a str;
                                        scalar name("name", 1) -> &'a str;
                                    }
                                }

                                ::validation::define_archive_dict_view! {
                                    #[derive(Clone, Copy, Debug)]
                                    pub struct Truncate {
                                        scalar enabled("enabled", 0) -> bool;
                                        scalar size("size", 1) -> i64;
                                    }
                                }
                            }
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct CampusLinkType {
                            scalar item (0) -> &'a str;
                        }
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct L3PortChannels {
                    model item (0) -> l3_port_channels::Item<'a>;
                }
            }

            pub mod l3_port_channels {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar name("name", 0) -> &'a str;
                        scalar node("node", 1) -> &'a str;
                        scalar arp_gratuitous_accept("arp_gratuitous_accept", 2) -> bool;
                        scalar description("description", 3) -> &'a str;
                        scalar mode("mode", 4) -> &'a str;
                        model member_interfaces("member_interfaces", 5) -> item::MemberInterfaces<'a>;
                        scalar ip_address("ip_address", 6) -> &'a str;
                        model ip_address_secondaries("ip_address_secondaries", 7) -> item::IpAddressSecondaries<'a>;
                        model ipv6_addresses("ipv6_addresses", 8) -> item::Ipv6Addresses<'a>;
                        scalar encapsulation_dot1q_vlan("encapsulation_dot1q_vlan", 9) -> i64;
                        scalar enabled("enabled", 10) -> bool;
                        scalar peer("peer", 11) -> &'a str;
                        scalar peer_port_channel("peer_port_channel", 12) -> &'a str;
                        scalar mtu("mtu", 13) -> i64;
                        scalar ipv4_acl_in("ipv4_acl_in", 14) -> &'a str;
                        scalar ipv4_acl_out("ipv4_acl_out", 15) -> &'a str;
                        scalar ipv6_acl_in("ipv6_acl_in", 16) -> &'a str;
                        scalar ipv6_acl_out("ipv6_acl_out", 17) -> &'a str;
                        model static_routes("static_routes", 18) -> item::StaticRoutes<'a>;
                        model ipv6_static_routes("ipv6_static_routes", 19) -> item::Ipv6StaticRoutes<'a>;
                        model ospf("ospf", 20) -> item::Ospf<'a>;
                        model flow_tracking("flow_tracking", 21) -> item::FlowTracking<'a>;
                        model structured_config("structured_config", 22) -> super::super::super::super::super::super::eos_cli_config_gen::port_channel_interfaces::Item<'a>;
                        scalar raw_eos_cli("raw_eos_cli", 23) -> &'a str;
                    }
                }

                pub mod item {

                    ::validation::define_archive_indexed_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MemberInterfaces {
                            model item (0) -> member_interfaces::Item<'a>;
                            primary_key_fields: [0];
                        }
                    }

                    pub mod member_interfaces {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Item {
                                scalar name("name", 0) -> &'a str;
                                scalar description("description", 1) -> &'a str;
                                scalar peer("peer", 2) -> &'a str;
                                scalar peer_interface("peer_interface", 3) -> &'a str;
                                scalar speed("speed", 4) -> &'a str;
                                model structured_config("structured_config", 5) -> super::super::super::super::super::super::super::super::eos_cli_config_gen::ethernet_interfaces::Item<'a>;
                            }
                        }

                        pub mod item {
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct IpAddressSecondaries {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Ipv6Addresses {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct StaticRoutes {
                            model item (0) -> static_routes::Item<'a>;
                        }
                    }

                    pub mod static_routes {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Item {
                                scalar prefix("prefix", 0) -> &'a str;
                                scalar next_hop("next_hop", 1) -> &'a str;
                                scalar track_bfd("track_bfd", 2) -> bool;
                                scalar distance("distance", 3) -> i64;
                                scalar tag("tag", 4) -> i64;
                                scalar name("name", 5) -> &'a str;
                                scalar metric("metric", 6) -> i64;
                                scalar interface("interface", 7) -> &'a str;
                            }
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Ipv6StaticRoutes {
                            model item (0) -> ipv6_static_routes::Item<'a>;
                        }
                    }

                    pub mod ipv6_static_routes {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Item {
                                scalar prefix("prefix", 0) -> &'a str;
                                scalar next_hop("next_hop", 1) -> &'a str;
                                scalar track_bfd("track_bfd", 2) -> bool;
                                scalar distance("distance", 3) -> i64;
                                scalar tag("tag", 4) -> i64;
                                scalar name("name", 5) -> &'a str;
                                scalar metric("metric", 6) -> i64;
                                scalar interface("interface", 7) -> &'a str;
                            }
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Ospf {
                            scalar enabled("enabled", 0) -> bool;
                            scalar point_to_point("point_to_point", 1) -> bool;
                            scalar area("area", 2) -> &'a str;
                            scalar cost("cost", 3) -> i64;
                            scalar authentication("authentication", 4) -> &'a str;
                            scalar simple_auth_key("simple_auth_key", 5) -> &'a str;
                            scalar cleartext_simple_auth_key("cleartext_simple_auth_key", 6) -> &'a str;
                            model message_digest_keys("message_digest_keys", 7) -> ospf::MessageDigestKeys<'a>;
                        }
                    }

                    pub mod ospf {

                        ::validation::define_archive_indexed_list_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct MessageDigestKeys {
                                model item (0) -> message_digest_keys::Item<'a>;
                                primary_key_fields: [0];
                            }
                        }

                        pub mod message_digest_keys {

                            ::validation::define_archive_dict_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct Item {
                                    scalar id("id", 0) -> i64;
                                    scalar hash_algorithm("hash_algorithm", 1) -> &'a str;
                                    scalar key("key", 2) -> &'a str;
                                    scalar cleartext_key("cleartext_key", 3) -> &'a str;
                                }
                            }
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct FlowTracking {
                            scalar enabled("enabled", 0) -> bool;
                            scalar name("name", 1) -> &'a str;
                        }
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Loopbacks {
                    model item (0) -> loopbacks::Item<'a>;
                }
            }

            pub mod loopbacks {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar node("node", 0) -> &'a str;
                        scalar loopback("loopback", 1) -> i64;
                        scalar ip_address("ip_address", 2) -> &'a str;
                        scalar description("description", 3) -> &'a str;
                        scalar enabled("enabled", 4) -> bool;
                        model ospf("ospf", 5) -> item::Ospf<'a>;
                        scalar hardware_forwarding("hardware_forwarding", 6) -> bool;
                        scalar raw_eos_cli("raw_eos_cli", 7) -> &'a str;
                    }
                }

                pub mod item {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Ospf {
                            scalar enabled("enabled", 0) -> bool;
                            scalar area("area", 1) -> &'a str;
                        }
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct StaticRoutes {
                    model item (0) -> static_routes::Item<'a>;
                }
            }

            pub mod static_routes {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        model nodes("nodes", 0) -> item::Nodes<'a>;
                        scalar prefix("prefix", 1) -> &'a str;
                        scalar next_hop("next_hop", 2) -> &'a str;
                        scalar track_bfd("track_bfd", 3) -> bool;
                        scalar distance("distance", 4) -> i64;
                        scalar tag("tag", 5) -> i64;
                        scalar name("name", 6) -> &'a str;
                        scalar metric("metric", 7) -> i64;
                        scalar interface("interface", 8) -> &'a str;
                    }
                }

                pub mod item {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Nodes {
                            scalar item (0) -> &'a str;
                        }
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Ipv6StaticRoutes {
                    model item (0) -> ipv6_static_routes::Item<'a>;
                }
            }

            pub mod ipv6_static_routes {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        model nodes("nodes", 0) -> item::Nodes<'a>;
                        scalar prefix("prefix", 1) -> &'a str;
                        scalar next_hop("next_hop", 2) -> &'a str;
                        scalar track_bfd("track_bfd", 3) -> bool;
                        scalar distance("distance", 4) -> i64;
                        scalar tag("tag", 5) -> i64;
                        scalar name("name", 6) -> &'a str;
                        scalar metric("metric", 7) -> i64;
                        scalar interface("interface", 8) -> &'a str;
                    }
                }

                pub mod item {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Nodes {
                            scalar item (0) -> &'a str;
                        }
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct StaticArpEntries {
                    model item (0) -> static_arp_entries::Item<'a>;
                }
            }

            pub mod static_arp_entries {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar ipv4_address("ipv4_address", 0) -> &'a str;
                        scalar mac_address("mac_address", 1) -> &'a str;
                        model nodes("nodes", 2) -> item::Nodes<'a>;
                    }
                }

                pub mod item {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Nodes {
                            scalar item (0) -> &'a str;
                        }
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct BgpPeers {
                    model item (0) -> bgp_peers::Item<'a>;
                }
            }

            pub mod bgp_peers {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar ip_address("ip_address", 0) -> &'a str;
                        scalar peer_group("peer_group", 1) -> &'a str;
                        scalar remote_as("remote_as", 2) -> &'a str;
                        scalar description("description", 3) -> &'a str;
                        scalar password("password", 4) -> &'a str;
                        scalar cleartext_password("cleartext_password", 5) -> &'a str;
                        scalar send_community("send_community", 6) -> &'a str;
                        scalar next_hop_self("next_hop_self", 7) -> bool;
                        scalar timers("timers", 8) -> &'a str;
                        scalar maximum_routes("maximum_routes", 9) -> i64;
                        scalar maximum_routes_warning_only("maximum_routes_warning_only", 10) -> bool;
                        model default_originate("default_originate", 11) -> item::DefaultOriginate<'a>;
                        scalar update_source("update_source", 12) -> &'a str;
                        scalar ebgp_multihop("ebgp_multihop", 13) -> i64;
                        model nodes("nodes", 14) -> item::Nodes<'a>;
                        scalar set_ipv4_next_hop("set_ipv4_next_hop", 15) -> &'a str;
                        scalar set_ipv6_next_hop("set_ipv6_next_hop", 16) -> &'a str;
                        scalar route_map_out("route_map_out", 17) -> &'a str;
                        scalar route_map_in("route_map_in", 18) -> &'a str;
                        scalar prefix_list_in("prefix_list_in", 19) -> &'a str;
                        scalar prefix_list_out("prefix_list_out", 20) -> &'a str;
                        scalar local_as("local_as", 21) -> &'a str;
                        scalar weight("weight", 22) -> i64;
                        scalar bfd("bfd", 23) -> bool;
                        model bfd_timers("bfd_timers", 24) -> super::super::super::super::super::super::eos_cli_config_gen::router_bgp::vrfs::item::neighbors::item::BfdTimers<'a>;
                        scalar route_reflector_client("route_reflector_client", 25) -> bool;
                        scalar shutdown("shutdown", 26) -> bool;
                    }
                }

                pub mod item {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct DefaultOriginate {
                            scalar always("always", 0) -> bool;
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Nodes {
                            scalar item (0) -> &'a str;
                        }
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Bgp {
                    scalar enabled("enabled", 0) -> bool;
                    scalar router_id("router_id", 1) -> &'a str;
                    model graceful_restart("graceful_restart", 2) -> bgp::GracefulRestart<'a>;
                    scalar raw_eos_cli("raw_eos_cli", 3) -> &'a str;
                    model structured_config("structured_config", 4) -> super::super::super::super::super::eos_cli_config_gen::router_bgp::vrfs::Item<'a>;
                }
            }

            pub mod bgp {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct GracefulRestart {
                        scalar enabled("enabled", 0) -> bool;
                        scalar restart_time("restart_time", 1) -> i64;
                    }
                }
            }

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct BgpPeerGroups {
                    model item (0) -> bgp_peer_groups::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod bgp_peer_groups {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar name("name", 0) -> &'a str;
                        model nodes("nodes", 1) -> item::Nodes<'a>;
                        scalar password("password", 2) -> &'a str;
                        scalar cleartext_password("cleartext_password", 3) -> &'a str;
                        model address_family_ipv4("address_family_ipv4", 4) -> item::AddressFamilyIpv4<'a>;
                        model address_family_ipv6("address_family_ipv6", 5) -> item::AddressFamilyIpv6<'a>;
                        model listen_ranges("listen_ranges", 6) -> item::ListenRanges<'a>;
                        model metadata("metadata", 7) -> item::Metadata<'a>;
                        scalar remote_as("remote_as", 8) -> &'a str;
                        scalar local_as("local_as", 9) -> &'a str;
                        scalar description("description", 10) -> &'a str;
                        scalar shutdown("shutdown", 11) -> bool;
                        model as_path("as_path", 12) -> item::AsPath<'a>;
                        model remove_private_as("remove_private_as", 13) -> item::RemovePrivateAs<'a>;
                        model remove_private_as_ingress("remove_private_as_ingress", 14) -> item::RemovePrivateAsIngress<'a>;
                        scalar next_hop_unchanged("next_hop_unchanged", 15) -> bool;
                        scalar update_source("update_source", 16) -> &'a str;
                        scalar route_reflector_client("route_reflector_client", 17) -> bool;
                        scalar bfd("bfd", 18) -> bool;
                        model bfd_timers("bfd_timers", 19) -> item::BfdTimers<'a>;
                        scalar ebgp_multihop("ebgp_multihop", 20) -> i64;
                        scalar next_hop_peer("next_hop_peer", 21) -> bool;
                        scalar next_hop_self("next_hop_self", 22) -> bool;
                        scalar password_type("password_type", 23) -> &'a str;
                        scalar passive("passive", 24) -> bool;
                        model default_originate("default_originate", 25) -> item::DefaultOriginate<'a>;
                        scalar enforce_first_as("enforce_first_as", 26) -> bool;
                        scalar send_community("send_community", 27) -> &'a str;
                        scalar maximum_routes("maximum_routes", 28) -> i64;
                        scalar maximum_routes_warning_limit("maximum_routes_warning_limit", 29) -> &'a str;
                        scalar maximum_routes_warning_only("maximum_routes_warning_only", 30) -> bool;
                        model maximum_accepted_routes("maximum_accepted_routes", 31) -> item::MaximumAcceptedRoutes<'a>;
                        model missing_policy("missing_policy", 32) -> item::MissingPolicy<'a>;
                        model link_bandwidth("link_bandwidth", 33) -> item::LinkBandwidth<'a>;
                        model allowas_in("allowas_in", 34) -> item::AllowasIn<'a>;
                        scalar weight("weight", 35) -> i64;
                        scalar timers("timers", 36) -> &'a str;
                        model rib_in_pre_policy_retain("rib_in_pre_policy_retain", 37) -> item::RibInPrePolicyRetain<'a>;
                        scalar route_map_in("route_map_in", 38) -> &'a str;
                        scalar route_map_out("route_map_out", 39) -> &'a str;
                        scalar peer_tag_in("peer_tag_in", 40) -> &'a str;
                        scalar peer_tag_out_discard("peer_tag_out_discard", 41) -> &'a str;
                        scalar session_tracker("session_tracker", 42) -> &'a str;
                        model shared_secret("shared_secret", 43) -> item::SharedSecret<'a>;
                        scalar ttl_maximum_hops("ttl_maximum_hops", 44) -> i64;
                        scalar maximum_advertised_routes("maximum_advertised_routes", 45) -> i64;
                        scalar maximum_advertised_routes_warning_limit("maximum_advertised_routes_warning_limit", 46) -> &'a str;
                    }
                }

                pub mod item {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Nodes {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct AddressFamilyIpv4 {
                            scalar activate("activate", 0) -> bool;
                            scalar route_map_in("route_map_in", 1) -> &'a str;
                            scalar route_map_out("route_map_out", 2) -> &'a str;
                            scalar rcf_in("rcf_in", 3) -> &'a str;
                            scalar rcf_out("rcf_out", 4) -> &'a str;
                            model default_originate("default_originate", 5) -> super::super::super::super::super::super::super::eos_cli_config_gen::router_bgp::address_family_ipv4::peer_groups::item::DefaultOriginate<'a>;
                            model next_hop("next_hop", 6) -> super::super::super::super::super::super::super::eos_cli_config_gen::router_bgp::address_family_ipv4::peer_groups::item::NextHop<'a>;
                            scalar prefix_list_in("prefix_list_in", 7) -> &'a str;
                            scalar prefix_list_out("prefix_list_out", 8) -> &'a str;
                        }
                    }

                    pub mod address_family_ipv4 {
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct AddressFamilyIpv6 {
                            scalar activate("activate", 0) -> bool;
                            scalar route_map_in("route_map_in", 1) -> &'a str;
                            scalar route_map_out("route_map_out", 2) -> &'a str;
                            scalar rcf_in("rcf_in", 3) -> &'a str;
                            scalar rcf_out("rcf_out", 4) -> &'a str;
                            model default_originate("default_originate", 5) -> address_family_ipv6::DefaultOriginate<'a>;
                            scalar prefix_list_in("prefix_list_in", 6) -> &'a str;
                            scalar prefix_list_out("prefix_list_out", 7) -> &'a str;
                        }
                    }

                    pub mod address_family_ipv6 {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct DefaultOriginate {
                                scalar enabled("enabled", 0) -> bool;
                                scalar always("always", 1) -> bool;
                                scalar route_map("route_map", 2) -> &'a str;
                            }
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct ListenRanges {
                            model item (0) -> listen_ranges::Item<'a>;
                        }
                    }

                    pub mod listen_ranges {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Item {
                                scalar prefix("prefix", 0) -> &'a str;
                                scalar remote_as("remote_as", 1) -> &'a str;
                                scalar peer_id_include_router_id("peer_id_include_router_id", 2) -> bool;
                                scalar peer_filter("peer_filter", 3) -> &'a str;
                            }
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Metadata {
                            scalar field_type("type", 0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct AsPath {
                            scalar remote_as_replace_out("remote_as_replace_out", 0) -> bool;
                            scalar prepend_own_disabled("prepend_own_disabled", 1) -> bool;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct RemovePrivateAs {
                            scalar enabled("enabled", 0) -> bool;
                            scalar all("all", 1) -> bool;
                            scalar replace_as("replace_as", 2) -> bool;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct RemovePrivateAsIngress {
                            scalar enabled("enabled", 0) -> bool;
                            scalar replace_as("replace_as", 1) -> bool;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct BfdTimers {
                            scalar interval("interval", 0) -> i64;
                            scalar min_rx("min_rx", 1) -> i64;
                            scalar multiplier("multiplier", 2) -> i64;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct DefaultOriginate {
                            scalar enabled("enabled", 0) -> bool;
                            scalar always("always", 1) -> bool;
                            scalar route_map("route_map", 2) -> &'a str;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MaximumAcceptedRoutes {
                            scalar limit("limit", 0) -> i64;
                            model warning_limit("warning_limit", 1) -> maximum_accepted_routes::WarningLimit<'a>;
                        }
                    }

                    pub mod maximum_accepted_routes {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct WarningLimit {
                                scalar count("count", 0) -> i64;
                                scalar percent("percent", 1) -> i64;
                            }
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MissingPolicy {
                            model direction_in("direction_in", 0) -> missing_policy::DirectionIn<'a>;
                            model direction_out("direction_out", 1) -> missing_policy::DirectionOut<'a>;
                        }
                    }

                    pub mod missing_policy {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct DirectionIn {
                                scalar action("action", 0) -> &'a str;
                                scalar include_community_list("include_community_list", 1) -> bool;
                                scalar include_prefix_list("include_prefix_list", 2) -> bool;
                                scalar include_sub_route_map("include_sub_route_map", 3) -> bool;
                            }
                        }

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct DirectionOut {
                                scalar action("action", 0) -> &'a str;
                                scalar include_community_list("include_community_list", 1) -> bool;
                                scalar include_prefix_list("include_prefix_list", 2) -> bool;
                                scalar include_sub_route_map("include_sub_route_map", 3) -> bool;
                            }
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct LinkBandwidth {
                            scalar enabled("enabled", 0) -> bool;
                            scalar default("default", 1) -> &'a str;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct AllowasIn {
                            scalar enabled("enabled", 0) -> bool;
                            scalar times("times", 1) -> i64;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct RibInPrePolicyRetain {
                            scalar enabled("enabled", 0) -> bool;
                            scalar all("all", 1) -> bool;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct SharedSecret {
                            scalar profile("profile", 0) -> &'a str;
                            scalar hash_algorithm("hash_algorithm", 1) -> &'a str;
                        }
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AdditionalRouteTargets {
                    model item (0) -> additional_route_targets::Item<'a>;
                }
            }

            pub mod additional_route_targets {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar field_type("type", 0) -> &'a str;
                        scalar address_family("address_family", 1) -> &'a str;
                        scalar route_target("route_target", 2) -> &'a str;
                        model nodes("nodes", 3) -> item::Nodes<'a>;
                    }
                }

                pub mod item {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Nodes {
                            scalar item (0) -> &'a str;
                        }
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AggregateAddresses {
                    model item (0) -> aggregate_addresses::Item<'a>;
                }
            }

            pub mod aggregate_addresses {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        model nodes("nodes", 0) -> item::Nodes<'a>;
                        scalar prefix("prefix", 1) -> &'a str;
                        scalar advertise_only("advertise_only", 2) -> bool;
                        scalar as_set("as_set", 3) -> bool;
                        scalar summary_only("summary_only", 4) -> bool;
                        scalar attribute_map("attribute_map", 5) -> &'a str;
                        scalar match_map("match_map", 6) -> &'a str;
                        model attribute("attribute", 7) -> item::Attribute<'a>;
                    }
                }

                pub mod item {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Nodes {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Attribute {
                            scalar rcf("rcf", 0) -> &'a str;
                        }
                    }
                }
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct L2vlans {
            model item (0) -> l2vlans::Item<'a>;
        }
    }

    pub mod l2vlans {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar id("id", 0) -> i64;
                scalar name("name", 1) -> &'a str;
                scalar profile("profile", 2) -> &'a str;
                model tags("tags", 3) -> item::Tags<'a>;
                model address_locking("address_locking", 4) -> super::super::super::super::eos_cli_config_gen::vlans::item::address_locking::AddressFamily<'a>;
                scalar vni_override("vni_override", 5) -> i64;
                scalar rt_override("rt_override", 6) -> &'a str;
                scalar rd_override("rd_override", 7) -> &'a str;
                scalar vxlan("vxlan", 8) -> bool;
                scalar spanning_tree_priority("spanning_tree_priority", 9) -> i64;
                scalar evpn_vlan_bundle("evpn_vlan_bundle", 10) -> &'a str;
                model trunk_groups("trunk_groups", 11) -> item::TrunkGroups<'a>;
                scalar evpn_l2_multi_domain("evpn_l2_multi_domain", 12) -> bool;
                model evpn_l2_multicast("evpn_l2_multicast", 13) -> item::EvpnL2Multicast<'a>;
                model vxlan_flood_multicast("vxlan_flood_multicast", 14) -> item::VxlanFloodMulticast<'a>;
                model igmp_snooping("igmp_snooping", 15) -> item::IgmpSnooping<'a>;
                scalar igmp_snooping_enabled("igmp_snooping_enabled", 16) -> bool;
                model igmp_snooping_querier("igmp_snooping_querier", 17) -> item::IgmpSnoopingQuerier<'a>;
                model bgp("bgp", 18) -> item::Bgp<'a>;
                model private_vlan("private_vlan", 19) -> item::PrivateVlan<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Tags {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct TrunkGroups {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct EvpnL2Multicast {
                    scalar enabled("enabled", 0) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct VxlanFloodMulticast {
                    scalar enabled("enabled", 0) -> bool;
                    scalar underlay_multicast_group("underlay_multicast_group", 1) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct IgmpSnooping {
                    scalar enabled("enabled", 0) -> bool;
                    model querier("querier", 1) -> igmp_snooping::Querier<'a>;
                    scalar fast_leave("fast_leave", 2) -> bool;
                }
            }

            pub mod igmp_snooping {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Querier {
                        scalar enabled("enabled", 0) -> bool;
                        scalar source_address("source_address", 1) -> &'a str;
                        scalar version("version", 2) -> i64;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct IgmpSnoopingQuerier {
                    scalar enabled("enabled", 0) -> bool;
                    scalar source_address("source_address", 1) -> &'a str;
                    scalar version("version", 2) -> i64;
                    scalar fast_leave("fast_leave", 3) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Bgp {
                    model structured_config("structured_config", 0) -> super::super::super::super::super::eos_cli_config_gen::router_bgp::vlans::Item<'a>;
                    scalar raw_eos_cli("raw_eos_cli", 1) -> &'a str;
                }
            }

            pub mod bgp {
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct PrivateVlan {
                    scalar field_type("type", 0) -> &'a str;
                    scalar primary_vlan("primary_vlan", 1) -> i64;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Vpws {
            scalar mpls_control_word("mpls_control_word", 0) -> bool;
            scalar mtu("mtu", 1) -> i64;
            scalar label_flow("label_flow", 2) -> bool;
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PointToPointServices {
            model item (0) -> point_to_point_services::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod point_to_point_services {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar field_type("type", 1) -> &'a str;
                model subinterfaces("subinterfaces", 2) -> item::Subinterfaces<'a>;
                model endpoints("endpoints", 3) -> item::Endpoints<'a>;
                scalar lldp_disable("lldp_disable", 4) -> bool;
            }
        }

        pub mod item {

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Subinterfaces {
                    model item (0) -> subinterfaces::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod subinterfaces {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar number("number", 0) -> i64;
                        model port_channel("port_channel", 1) -> item::PortChannel<'a>;
                        model structured_config("structured_config", 2) -> super::super::super::super::super::super::eos_cli_config_gen::ethernet_interfaces::Item<'a>;
                        scalar raw_eos_cli("raw_eos_cli", 3) -> &'a str;
                    }
                }

                pub mod item {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct PortChannel {
                            model structured_config("structured_config", 0) -> super::super::super::super::super::super::super::eos_cli_config_gen::port_channel_interfaces::Item<'a>;
                            scalar raw_eos_cli("raw_eos_cli", 1) -> &'a str;
                        }
                    }

                    pub mod port_channel {
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Endpoints {
                    model item (0) -> endpoints::Item<'a>;
                }
            }

            pub mod endpoints {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar id("id", 0) -> i64;
                        model nodes("nodes", 1) -> item::Nodes<'a>;
                        model interfaces("interfaces", 2) -> item::Interfaces<'a>;
                        model port_channel("port_channel", 3) -> item::PortChannel<'a>;
                    }
                }

                pub mod item {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Nodes {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Interfaces {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct PortChannel {
                            scalar mode("mode", 0) -> &'a str;
                            scalar short_esi("short_esi", 1) -> &'a str;
                        }
                    }
                }
            }
        }
    }
}
