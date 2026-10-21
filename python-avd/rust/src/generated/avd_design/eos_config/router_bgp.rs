// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct RouterBgp<'a, Mode> {
    #[data_view(rename = "as")]
    pub field_as: ::data_views::Field<&'a str>,
    pub as_notation: ::data_views::Field<&'a str>,
    pub router_id: ::data_views::Field<&'a str>,
    pub timers: ::data_views::Field<Timers<'a, Mode>>,
    pub distance: ::data_views::Field<Distance<'a, Mode>>,
    pub graceful_restart: ::data_views::Field<GracefulRestart<'a, Mode>>,
    pub graceful_restart_helper: ::data_views::Field<GracefulRestartHelper<'a, Mode>>,
    pub maximum_paths: ::data_views::Field<MaximumPaths<'a, Mode>>,
    pub route_distinguisher: ::data_views::Field<RouteDistinguisher<'a, Mode>>,
    pub updates: ::data_views::Field<Updates<'a, Mode>>,
    pub bgp_cluster_id: ::data_views::Field<&'a str>,
    pub bgp_defaults: ::data_views::Field<BgpDefaults<'a, Mode>>,
    pub bgp: ::data_views::Field<Bgp<'a, Mode>>,
    pub listen_ranges: ::data_views::Field<ListenRanges<'a, Mode>>,
    pub neighbor_default: ::data_views::Field<NeighborDefault<'a, Mode>>,
    pub peer_groups: ::data_views::Field<PeerGroups<'a, Mode>>,
    pub neighbors: ::data_views::Field<Neighbors<'a, Mode>>,
    pub neighbor_interfaces: ::data_views::Field<NeighborInterfaces<'a, Mode>>,
    pub aggregate_addresses: ::data_views::Field<AggregateAddresses<'a, Mode>>,
    pub redistribute: ::data_views::Field<Redistribute<'a, Mode>>,
    pub vlan_aware_bundles: ::data_views::Field<VlanAwareBundles<'a, Mode>>,
    pub vlans: ::data_views::Field<Vlans<'a, Mode>>,
    pub vpws: ::data_views::Field<Vpws<'a, Mode>>,
    pub address_family_evpn: ::data_views::Field<AddressFamilyEvpn<'a, Mode>>,
    pub address_family_rtc: ::data_views::Field<AddressFamilyRtc<'a, Mode>>,
    pub address_family_ipv4: ::data_views::Field<AddressFamilyIpv4<'a, Mode>>,
    pub address_family_ipv4_labeled_unicast: ::data_views::Field<AddressFamilyIpv4LabeledUnicast<'a, Mode>>,
    pub address_family_ipv4_multicast: ::data_views::Field<AddressFamilyIpv4Multicast<'a, Mode>>,
    pub address_family_ipv4_sr_te: ::data_views::Field<AddressFamilyIpv4SrTe<'a, Mode>>,
    pub address_family_ipv6: ::data_views::Field<AddressFamilyIpv6<'a, Mode>>,
    pub address_family_ipv6_multicast: ::data_views::Field<AddressFamilyIpv6Multicast<'a, Mode>>,
    pub address_family_ipv6_sr_te: ::data_views::Field<AddressFamilyIpv6SrTe<'a, Mode>>,
    pub address_family_link_state: ::data_views::Field<AddressFamilyLinkState<'a, Mode>>,
    pub address_family_flow_spec_ipv4: ::data_views::Field<AddressFamilyFlowSpecIpv4<'a, Mode>>,
    pub address_family_flow_spec_ipv6: ::data_views::Field<AddressFamilyFlowSpecIpv6<'a, Mode>>,
    pub address_family_path_selection: ::data_views::Field<AddressFamilyPathSelection<'a, Mode>>,
    pub address_family_vpn_ipv4: ::data_views::Field<AddressFamilyVpnIpv4<'a, Mode>>,
    pub address_family_vpn_ipv6: ::data_views::Field<AddressFamilyVpnIpv6<'a, Mode>>,
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
    pub session_trackers: ::data_views::Field<SessionTrackers<'a, Mode>>,
    pub eos_cli: ::data_views::Field<&'a str>,
}

#[::data_views::data_view]
pub struct Timers<'a, Mode> {
    pub keepalive_time: ::data_views::Field<i64>,
    pub hold_time: ::data_views::Field<i64>,
    pub min_hold_time: ::data_views::Field<i64>,
    pub send_failure_hold_time: ::data_views::Field<i64>,
}

#[::data_views::data_view(python(name = "EOSConfig.RouterBgp.Distance", schema = Dict(892)))]
pub struct Distance<'a, Mode> {
    #[data_view(schema = Int(1))]
    pub external_routes: ::data_views::RequiredValue<i64, Mode>,
    #[data_view(schema = Int(1))]
    pub internal_routes: ::data_views::RequiredValue<i64, Mode>,
    #[data_view(schema = Int(1))]
    pub local_routes: ::data_views::RequiredValue<i64, Mode>,
}

#[::data_views::data_view]
pub struct GracefulRestart<'a, Mode> {
    pub enabled: ::data_views::Field<bool>,
    pub restart_time: ::data_views::Field<i64>,
    pub stalepath_time: ::data_views::Field<i64>,
}

#[::data_views::data_view]
pub struct GracefulRestartHelper<'a, Mode> {
    pub enabled: ::data_views::Field<bool>,
    pub restart_time: ::data_views::Field<i64>,
    pub long_lived: ::data_views::Field<bool>,
}

#[::data_views::data_view]
pub struct MaximumPaths<'a, Mode> {
    pub paths: ::data_views::RequiredValue<i64, Mode>,
    pub ecmp: ::data_views::Field<i64>,
}

#[::data_views::data_view]
pub struct RouteDistinguisher<'a, Mode> {
    pub assignment_auto: ::data_views::Field<route_distinguisher::AssignmentAuto<'a, Mode>>,
}

pub mod route_distinguisher {

    #[::data_views::data_view]
    pub struct AssignmentAuto<'a, Mode> {
        pub range: ::data_views::Field<assignment_auto::Range<'a, Mode>>,
        pub address_families: ::data_views::Field<assignment_auto::AddressFamilies<'a, Mode>>,
    }

    pub mod assignment_auto {

        #[::data_views::data_view]
        pub struct Range<'a, Mode> {
            pub start: ::data_views::RequiredValue<i64, Mode>,
            pub end: ::data_views::RequiredValue<i64, Mode>,
        }

        #[::data_views::data_view(list)]
        pub struct AddressFamilies<'a, Mode> (::data_views::Field<&'a str>);
    }
}

#[::data_views::data_view]
pub struct Updates<'a, Mode> {
    pub wait_for_convergence: ::data_views::Field<bool>,
    pub wait_install: ::data_views::Field<bool>,
}

#[::data_views::data_view(list)]
pub struct BgpDefaults<'a, Mode> (::data_views::Field<&'a str>);

#[::data_views::data_view]
pub struct Bgp<'a, Mode> {
    pub convergence: ::data_views::Field<bgp::Convergence<'a, Mode>>,
    pub default: ::data_views::Field<bgp::Default<'a, Mode>>,
    pub route_reflector_preserve_attributes: ::data_views::Field<bgp::RouteReflectorPreserveAttributes<'a, Mode>>,
    pub bestpath: ::data_views::Field<bgp::Bestpath<'a, Mode>>,
    pub additional_paths: ::data_views::Field<bgp::AdditionalPaths<'a, Mode>>,
    pub redistribute_internal: ::data_views::Field<bool>,
    pub labeled_unicast: ::data_views::Field<bgp::LabeledUnicast<'a, Mode>>,
}

pub mod bgp {

    #[::data_views::data_view]
    pub struct Convergence<'a, Mode> {
        pub slow_peer_time: ::data_views::Field<i64>,
        pub time: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct Default<'a, Mode> {
        pub ipv4_unicast: ::data_views::Field<bool>,
        pub ipv4_unicast_transport_ipv6: ::data_views::Field<bool>,
    }

    #[::data_views::data_view]
    pub struct RouteReflectorPreserveAttributes<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub always: ::data_views::Field<bool>,
    }

    #[::data_views::data_view]
    pub struct Bestpath<'a, Mode> {
        pub d_path: ::data_views::Field<bool>,
    }

    #[::data_views::data_view]
    pub struct AdditionalPaths<'a, Mode> {
        pub receive: ::data_views::Field<bool>,
        pub send: ::data_views::Field<&'a str>,
        pub send_limit: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct LabeledUnicast<'a, Mode> {
        pub rib: ::data_views::Field<labeled_unicast::Rib<'a, Mode>>,
    }

    pub mod labeled_unicast {

        #[::data_views::data_view]
        pub struct Rib<'a, Mode> {
            pub ip: ::data_views::Field<rib::Ip<'a, Mode>>,
            pub tunnel: ::data_views::Field<rib::Tunnel<'a, Mode>>,
        }

        pub mod rib {

            #[::data_views::data_view]
            pub struct Ip<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct Tunnel<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
            }
        }
    }
}

#[::data_views::data_view(list)]
pub struct ListenRanges<'a, Mode> (::data_views::Field<listen_ranges::Item<'a, Mode>>);

pub mod listen_ranges {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub prefix: ::data_views::Field<&'a str>,
        pub peer_id_include_router_id: ::data_views::Field<bool>,
        pub peer_group: ::data_views::Field<&'a str>,
        pub peer_filter: ::data_views::Field<&'a str>,
        pub remote_as: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view]
pub struct NeighborDefault<'a, Mode> {
    pub send_community: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct PeerGroups<'a, Mode> (::data_views::Field<peer_groups::Item<'a, Mode>>);

pub mod peer_groups {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub metadata: ::data_views::Field<item::Metadata<'a, Mode>>,
        pub remote_as: ::data_views::Field<&'a str>,
        pub local_as: ::data_views::Field<&'a str>,
        pub description: ::data_views::Field<&'a str>,
        pub shutdown: ::data_views::Field<bool>,
        pub as_path: ::data_views::Field<item::AsPath<'a, Mode>>,
        pub remove_private_as: ::data_views::Field<item::RemovePrivateAs<'a, Mode>>,
        pub remove_private_as_ingress: ::data_views::Field<item::RemovePrivateAsIngress<'a, Mode>>,
        pub next_hop_unchanged: ::data_views::Field<bool>,
        pub update_source: ::data_views::Field<&'a str>,
        pub route_reflector_client: ::data_views::Field<bool>,
        pub bfd: ::data_views::Field<bool>,
        pub bfd_timers: ::data_views::Field<item::BfdTimers<'a, Mode>>,
        pub ebgp_multihop: ::data_views::Field<i64>,
        pub next_hop_peer: ::data_views::Field<bool>,
        pub next_hop_self: ::data_views::Field<bool>,
        pub password: ::data_views::Field<&'a str>,
        pub password_type: ::data_views::Field<&'a str>,
        pub passive: ::data_views::Field<bool>,
        pub default_originate: ::data_views::Field<item::DefaultOriginate<'a, Mode>>,
        pub enforce_first_as: ::data_views::Field<bool>,
        pub send_community: ::data_views::Field<&'a str>,
        pub maximum_routes: ::data_views::Field<i64>,
        pub maximum_routes_warning_limit: ::data_views::Field<&'a str>,
        pub maximum_routes_warning_only: ::data_views::Field<bool>,
        pub maximum_accepted_routes: ::data_views::Field<item::MaximumAcceptedRoutes<'a, Mode>>,
        pub missing_policy: ::data_views::Field<item::MissingPolicy<'a, Mode>>,
        pub link_bandwidth: ::data_views::Field<item::LinkBandwidth<'a, Mode>>,
        pub allowas_in: ::data_views::Field<item::AllowasIn<'a, Mode>>,
        pub weight: ::data_views::Field<i64>,
        pub timers: ::data_views::Field<&'a str>,
        pub rib_in_pre_policy_retain: ::data_views::Field<item::RibInPrePolicyRetain<'a, Mode>>,
        pub route_map_in: ::data_views::Field<&'a str>,
        pub route_map_out: ::data_views::Field<&'a str>,
        pub peer_tag_in: ::data_views::Field<&'a str>,
        pub peer_tag_out_discard: ::data_views::Field<&'a str>,
        pub session_tracker: ::data_views::Field<&'a str>,
        pub shared_secret: ::data_views::Field<item::SharedSecret<'a, Mode>>,
        pub ttl_maximum_hops: ::data_views::Field<i64>,
        pub maximum_advertised_routes: ::data_views::Field<i64>,
        pub maximum_advertised_routes_warning_limit: ::data_views::Field<&'a str>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct Metadata<'a, Mode> {
            #[data_view(rename = "type")]
            pub field_type: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct AsPath<'a, Mode> {
            pub remote_as_replace_out: ::data_views::Field<bool>,
            pub prepend_own_disabled: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct RemovePrivateAs<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub all: ::data_views::Field<bool>,
            pub replace_as: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct RemovePrivateAsIngress<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub replace_as: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct BfdTimers<'a, Mode> {
            pub interval: ::data_views::RequiredValue<i64, Mode>,
            pub min_rx: ::data_views::RequiredValue<i64, Mode>,
            pub multiplier: ::data_views::RequiredValue<i64, Mode>,
        }

        #[::data_views::data_view]
        pub struct DefaultOriginate<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub always: ::data_views::Field<bool>,
            pub route_map: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct MaximumAcceptedRoutes<'a, Mode> {
            pub limit: ::data_views::RequiredValue<i64, Mode>,
            pub warning_limit: ::data_views::Field<maximum_accepted_routes::WarningLimit<'a, Mode>>,
        }

        pub mod maximum_accepted_routes {

            #[::data_views::data_view]
            pub struct WarningLimit<'a, Mode> {
                pub count: ::data_views::Field<i64>,
                pub percent: ::data_views::Field<i64>,
            }
        }

        #[::data_views::data_view]
        pub struct MissingPolicy<'a, Mode> {
            pub direction_in: ::data_views::Field<missing_policy::DirectionIn<'a, Mode>>,
            pub direction_out: ::data_views::Field<missing_policy::DirectionOut<'a, Mode>>,
        }

        pub mod missing_policy {

            #[::data_views::data_view]
            pub struct DirectionIn<'a, Mode> {
                pub action: ::data_views::RequiredValue<&'a str, Mode>,
                pub include_community_list: ::data_views::Field<bool>,
                pub include_prefix_list: ::data_views::Field<bool>,
                pub include_sub_route_map: ::data_views::Field<bool>,
            }

            #[::data_views::data_view]
            pub struct DirectionOut<'a, Mode> {
                pub action: ::data_views::RequiredValue<&'a str, Mode>,
                pub include_community_list: ::data_views::Field<bool>,
                pub include_prefix_list: ::data_views::Field<bool>,
                pub include_sub_route_map: ::data_views::Field<bool>,
            }
        }

        #[::data_views::data_view]
        pub struct LinkBandwidth<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub default: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct AllowasIn<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub times: ::data_views::Field<i64>,
        }

        #[::data_views::data_view]
        pub struct RibInPrePolicyRetain<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub all: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct SharedSecret<'a, Mode> {
            pub profile: ::data_views::RequiredValue<&'a str, Mode>,
            pub hash_algorithm: ::data_views::RequiredValue<&'a str, Mode>,
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(ip_address))]
pub struct Neighbors<'a, Mode> (::data_views::Field<neighbors::Item<'a, Mode>>);

pub mod neighbors {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub ip_address: ::data_views::Field<&'a str>,
        pub peer_group: ::data_views::Field<&'a str>,
        pub remote_as: ::data_views::Field<&'a str>,
        pub local_as: ::data_views::Field<&'a str>,
        pub as_path: ::data_views::Field<item::AsPath<'a, Mode>>,
        pub metadata: ::data_views::Field<item::Metadata<'a, Mode>>,
        pub description: ::data_views::Field<&'a str>,
        pub route_reflector_client: ::data_views::Field<bool>,
        pub password: ::data_views::Field<&'a str>,
        pub password_type: ::data_views::Field<&'a str>,
        pub passive: ::data_views::Field<bool>,
        pub shutdown: ::data_views::Field<bool>,
        pub update_source: ::data_views::Field<&'a str>,
        pub bfd: ::data_views::Field<bool>,
        pub bfd_timers: ::data_views::Field<item::BfdTimers<'a, Mode>>,
        pub weight: ::data_views::Field<i64>,
        pub timers: ::data_views::Field<&'a str>,
        pub route_map_in: ::data_views::Field<&'a str>,
        pub route_map_out: ::data_views::Field<&'a str>,
        pub peer_tag_in: ::data_views::Field<&'a str>,
        pub peer_tag_out_discard: ::data_views::Field<&'a str>,
        pub default_originate: ::data_views::Field<item::DefaultOriginate<'a, Mode>>,
        pub enforce_first_as: ::data_views::Field<bool>,
        pub send_community: ::data_views::Field<&'a str>,
        pub maximum_routes: ::data_views::Field<i64>,
        pub maximum_routes_warning_limit: ::data_views::Field<&'a str>,
        pub maximum_routes_warning_only: ::data_views::Field<bool>,
        pub maximum_accepted_routes: ::data_views::Field<item::MaximumAcceptedRoutes<'a, Mode>>,
        pub missing_policy: ::data_views::Field<item::MissingPolicy<'a, Mode>>,
        pub allowas_in: ::data_views::Field<item::AllowasIn<'a, Mode>>,
        pub ebgp_multihop: ::data_views::Field<i64>,
        pub next_hop_peer: ::data_views::Field<bool>,
        pub next_hop_self: ::data_views::Field<bool>,
        pub link_bandwidth: ::data_views::Field<item::LinkBandwidth<'a, Mode>>,
        pub rib_in_pre_policy_retain: ::data_views::Field<item::RibInPrePolicyRetain<'a, Mode>>,
        pub remove_private_as: ::data_views::Field<item::RemovePrivateAs<'a, Mode>>,
        pub remove_private_as_ingress: ::data_views::Field<item::RemovePrivateAsIngress<'a, Mode>>,
        pub session_tracker: ::data_views::Field<&'a str>,
        pub shared_secret: ::data_views::Field<item::SharedSecret<'a, Mode>>,
        pub ttl_maximum_hops: ::data_views::Field<i64>,
        pub maximum_advertised_routes: ::data_views::Field<i64>,
        pub maximum_advertised_routes_warning_limit: ::data_views::Field<&'a str>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct AsPath<'a, Mode> {
            pub remote_as_replace_out: ::data_views::Field<bool>,
            pub prepend_own_disabled: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct Metadata<'a, Mode> {
            pub peer: ::data_views::Field<&'a str>,
            pub validate_state: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct BfdTimers<'a, Mode> {
            pub interval: ::data_views::RequiredValue<i64, Mode>,
            pub min_rx: ::data_views::RequiredValue<i64, Mode>,
            pub multiplier: ::data_views::RequiredValue<i64, Mode>,
        }

        #[::data_views::data_view]
        pub struct DefaultOriginate<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub always: ::data_views::Field<bool>,
            pub route_map: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct MaximumAcceptedRoutes<'a, Mode> {
            pub limit: ::data_views::RequiredValue<i64, Mode>,
            pub warning_limit: ::data_views::Field<maximum_accepted_routes::WarningLimit<'a, Mode>>,
        }

        pub mod maximum_accepted_routes {

            #[::data_views::data_view]
            pub struct WarningLimit<'a, Mode> {
                pub count: ::data_views::Field<i64>,
                pub percent: ::data_views::Field<i64>,
            }
        }

        #[::data_views::data_view]
        pub struct MissingPolicy<'a, Mode> {
            pub direction_in: ::data_views::Field<missing_policy::DirectionIn<'a, Mode>>,
            pub direction_out: ::data_views::Field<missing_policy::DirectionOut<'a, Mode>>,
        }

        pub mod missing_policy {

            #[::data_views::data_view]
            pub struct DirectionIn<'a, Mode> {
                pub action: ::data_views::RequiredValue<&'a str, Mode>,
                pub include_community_list: ::data_views::Field<bool>,
                pub include_prefix_list: ::data_views::Field<bool>,
                pub include_sub_route_map: ::data_views::Field<bool>,
            }

            #[::data_views::data_view]
            pub struct DirectionOut<'a, Mode> {
                pub action: ::data_views::RequiredValue<&'a str, Mode>,
                pub include_community_list: ::data_views::Field<bool>,
                pub include_prefix_list: ::data_views::Field<bool>,
                pub include_sub_route_map: ::data_views::Field<bool>,
            }
        }

        #[::data_views::data_view]
        pub struct AllowasIn<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub times: ::data_views::Field<i64>,
        }

        #[::data_views::data_view]
        pub struct LinkBandwidth<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub default: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct RibInPrePolicyRetain<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub all: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct RemovePrivateAs<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub all: ::data_views::Field<bool>,
            pub replace_as: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct RemovePrivateAsIngress<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub replace_as: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct SharedSecret<'a, Mode> {
            pub profile: ::data_views::RequiredValue<&'a str, Mode>,
            pub hash_algorithm: ::data_views::RequiredValue<&'a str, Mode>,
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct NeighborInterfaces<'a, Mode> (::data_views::Field<neighbor_interfaces::Item<'a, Mode>>);

pub mod neighbor_interfaces {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub remote_as: ::data_views::Field<&'a str>,
        pub metadata: ::data_views::Field<item::Metadata<'a, Mode>>,
        pub peer_group: ::data_views::Field<&'a str>,
        pub description: ::data_views::Field<&'a str>,
        pub peer_filter: ::data_views::Field<&'a str>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct Metadata<'a, Mode> {
            pub peer: ::data_views::Field<&'a str>,
            pub validate_state: ::data_views::Field<bool>,
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(prefix))]
pub struct AggregateAddresses<'a, Mode> (::data_views::Field<aggregate_addresses::Item<'a, Mode>>);

pub mod aggregate_addresses {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub prefix: ::data_views::Field<&'a str>,
        pub advertise_only: ::data_views::Field<bool>,
        pub as_set: ::data_views::Field<bool>,
        pub summary_only: ::data_views::Field<bool>,
        pub attribute_map: ::data_views::Field<&'a str>,
        pub match_map: ::data_views::Field<&'a str>,
        pub attribute: ::data_views::Field<item::Attribute<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct Attribute<'a, Mode> {
            pub rcf: ::data_views::Field<&'a str>,
        }
    }
}

#[::data_views::data_view]
pub struct Redistribute<'a, Mode> {
    pub attached_host: ::data_views::Field<redistribute::AttachedHost<'a, Mode>>,
    pub bgp: ::data_views::Field<redistribute::Bgp<'a, Mode>>,
    pub connected: ::data_views::Field<redistribute::Connected<'a, Mode>>,
    pub dynamic: ::data_views::Field<redistribute::Dynamic<'a, Mode>>,
    pub isis: ::data_views::Field<redistribute::Isis<'a, Mode>>,
    pub ospf: ::data_views::Field<redistribute::Ospf<'a, Mode>>,
    pub ospfv3: ::data_views::Field<redistribute::Ospfv3<'a, Mode>>,
    pub rip: ::data_views::Field<redistribute::Rip<'a, Mode>>,
    #[data_view(rename = "static")]
    pub field_static: ::data_views::Field<redistribute::FieldStatic<'a, Mode>>,
    pub user: ::data_views::Field<redistribute::User<'a, Mode>>,
}

pub mod redistribute {

    #[::data_views::data_view]
    pub struct AttachedHost<'a, Mode> {
        pub enabled: ::data_views::RequiredValue<bool, Mode>,
        pub route_map: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view]
    pub struct Bgp<'a, Mode> {
        pub enabled: ::data_views::RequiredValue<bool, Mode>,
        pub route_map: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view]
    pub struct Connected<'a, Mode> {
        pub enabled: ::data_views::RequiredValue<bool, Mode>,
        pub route_map: ::data_views::Field<&'a str>,
        pub rcf: ::data_views::Field<&'a str>,
        pub include_leaked: ::data_views::Field<bool>,
    }

    #[::data_views::data_view]
    pub struct Dynamic<'a, Mode> {
        pub enabled: ::data_views::RequiredValue<bool, Mode>,
        pub route_map: ::data_views::Field<&'a str>,
        pub rcf: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view]
    pub struct Isis<'a, Mode> {
        pub enabled: ::data_views::RequiredValue<bool, Mode>,
        pub isis_level: ::data_views::Field<&'a str>,
        pub route_map: ::data_views::Field<&'a str>,
        pub rcf: ::data_views::Field<&'a str>,
        pub include_leaked: ::data_views::Field<bool>,
    }

    #[::data_views::data_view]
    pub struct Ospf<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub match_external: ::data_views::Field<ospf::MatchExternal<'a, Mode>>,
        pub match_internal: ::data_views::Field<ospf::MatchInternal<'a, Mode>>,
        pub match_nssa_external: ::data_views::Field<ospf::MatchNssaExternal<'a, Mode>>,
        pub route_map: ::data_views::Field<&'a str>,
        pub include_leaked: ::data_views::Field<bool>,
    }

    pub mod ospf {

        #[::data_views::data_view]
        pub struct MatchExternal<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct MatchInternal<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct MatchNssaExternal<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub nssa_type: ::data_views::Field<i64>,
            pub route_map: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }
    }

    #[::data_views::data_view]
    pub struct Ospfv3<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub match_external: ::data_views::Field<ospfv3::MatchExternal<'a, Mode>>,
        pub match_internal: ::data_views::Field<ospfv3::MatchInternal<'a, Mode>>,
        pub match_nssa_external: ::data_views::Field<ospfv3::MatchNssaExternal<'a, Mode>>,
        pub route_map: ::data_views::Field<&'a str>,
        pub include_leaked: ::data_views::Field<bool>,
    }

    pub mod ospfv3 {

        #[::data_views::data_view]
        pub struct MatchExternal<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct MatchInternal<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct MatchNssaExternal<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub nssa_type: ::data_views::Field<i64>,
            pub route_map: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }
    }

    #[::data_views::data_view]
    pub struct Rip<'a, Mode> {
        pub enabled: ::data_views::RequiredValue<bool, Mode>,
        pub route_map: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view]
    pub struct FieldStatic<'a, Mode> {
        pub enabled: ::data_views::RequiredValue<bool, Mode>,
        pub route_map: ::data_views::Field<&'a str>,
        pub rcf: ::data_views::Field<&'a str>,
        pub include_leaked: ::data_views::Field<bool>,
    }

    #[::data_views::data_view]
    pub struct User<'a, Mode> {
        pub enabled: ::data_views::RequiredValue<bool, Mode>,
        pub rcf: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct VlanAwareBundles<'a, Mode> (::data_views::Field<vlan_aware_bundles::Item<'a, Mode>>);

pub mod vlan_aware_bundles {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub metadata: ::data_views::Field<item::Metadata<'a, Mode>>,
        pub rd: ::data_views::Field<&'a str>,
        pub rd_evpn_domain: ::data_views::Field<item::RdEvpnDomain<'a, Mode>>,
        pub route_targets: ::data_views::Field<item::RouteTargets<'a, Mode>>,
        pub redistribute_routes: ::data_views::Field<item::RedistributeRoutes<'a, Mode>>,
        pub no_redistribute_routes: ::data_views::Field<item::NoRedistributeRoutes<'a, Mode>>,
        pub vlan: ::data_views::Field<&'a str>,
        pub eos_cli: ::data_views::Field<&'a str>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct Metadata<'a, Mode> {
            pub tenants: ::data_views::Field<metadata::Tenants<'a, Mode>>,
            pub description: ::data_views::Field<&'a str>,
        }

        pub mod metadata {

            #[::data_views::data_view(list)]
            pub struct Tenants<'a, Mode> (::data_views::Field<&'a str>);
        }

        #[::data_views::data_view]
        pub struct RdEvpnDomain<'a, Mode> {
            pub domain: ::data_views::Field<&'a str>,
            pub rd: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct RouteTargets<'a, Mode> {
            pub both: ::data_views::Field<route_targets::Both<'a, Mode>>,
            #[data_view(rename = "import")]
            pub field_import: ::data_views::Field<route_targets::Import<'a, Mode>>,
            pub export: ::data_views::Field<route_targets::Export<'a, Mode>>,
            pub import_evpn_domains: ::data_views::Field<route_targets::ImportEvpnDomains<'a, Mode>>,
            pub export_evpn_domains: ::data_views::Field<route_targets::ExportEvpnDomains<'a, Mode>>,
            pub import_export_evpn_domains: ::data_views::Field<route_targets::ImportExportEvpnDomains<'a, Mode>>,
        }

        pub mod route_targets {

            #[::data_views::data_view(list)]
            pub struct Both<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list)]
            pub struct Import<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list)]
            pub struct Export<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list)]
            pub struct ImportEvpnDomains<'a, Mode> (::data_views::Field<import_evpn_domains::Item<'a, Mode>>);

            pub mod import_evpn_domains {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub domain: ::data_views::Field<&'a str>,
                    pub route_target: ::data_views::Field<&'a str>,
                }
            }

            #[::data_views::data_view(list)]
            pub struct ExportEvpnDomains<'a, Mode> (::data_views::Field<export_evpn_domains::Item<'a, Mode>>);

            pub mod export_evpn_domains {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub domain: ::data_views::Field<&'a str>,
                    pub route_target: ::data_views::Field<&'a str>,
                }
            }

            #[::data_views::data_view(list)]
            pub struct ImportExportEvpnDomains<'a, Mode> (::data_views::Field<import_export_evpn_domains::Item<'a, Mode>>);

            pub mod import_export_evpn_domains {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub domain: ::data_views::Field<&'a str>,
                    pub route_target: ::data_views::Field<&'a str>,
                }
            }
        }

        #[::data_views::data_view(list)]
        pub struct RedistributeRoutes<'a, Mode> (::data_views::Field<&'a str>);

        #[::data_views::data_view(list)]
        pub struct NoRedistributeRoutes<'a, Mode> (::data_views::Field<&'a str>);
    }
}

#[::data_views::data_view(indexed_list, primary_key(id))]
pub struct Vlans<'a, Mode> (::data_views::Field<vlans::Item<'a, Mode>>);

pub mod vlans {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub id: ::data_views::Field<i64>,
        pub metadata: ::data_views::Field<item::Metadata<'a, Mode>>,
        pub rd: ::data_views::Field<&'a str>,
        pub rd_evpn_domain: ::data_views::Field<item::RdEvpnDomain<'a, Mode>>,
        pub route_targets: ::data_views::Field<item::RouteTargets<'a, Mode>>,
        pub redistribute_routes: ::data_views::Field<item::RedistributeRoutes<'a, Mode>>,
        pub no_redistribute_routes: ::data_views::Field<item::NoRedistributeRoutes<'a, Mode>>,
        pub eos_cli: ::data_views::Field<&'a str>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct Metadata<'a, Mode> {
            pub tenants: ::data_views::Field<metadata::Tenants<'a, Mode>>,
        }

        pub mod metadata {

            #[::data_views::data_view(list)]
            pub struct Tenants<'a, Mode> (::data_views::Field<&'a str>);
        }

        #[::data_views::data_view]
        pub struct RdEvpnDomain<'a, Mode> {
            pub domain: ::data_views::Field<&'a str>,
            pub rd: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct RouteTargets<'a, Mode> {
            pub both: ::data_views::Field<route_targets::Both<'a, Mode>>,
            #[data_view(rename = "import")]
            pub field_import: ::data_views::Field<route_targets::Import<'a, Mode>>,
            pub export: ::data_views::Field<route_targets::Export<'a, Mode>>,
            pub import_evpn_domains: ::data_views::Field<route_targets::ImportEvpnDomains<'a, Mode>>,
            pub export_evpn_domains: ::data_views::Field<route_targets::ExportEvpnDomains<'a, Mode>>,
            pub import_export_evpn_domains: ::data_views::Field<route_targets::ImportExportEvpnDomains<'a, Mode>>,
        }

        pub mod route_targets {

            #[::data_views::data_view(list)]
            pub struct Both<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list)]
            pub struct Import<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list)]
            pub struct Export<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list)]
            pub struct ImportEvpnDomains<'a, Mode> (::data_views::Field<import_evpn_domains::Item<'a, Mode>>);

            pub mod import_evpn_domains {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub domain: ::data_views::Field<&'a str>,
                    pub route_target: ::data_views::Field<&'a str>,
                }
            }

            #[::data_views::data_view(list)]
            pub struct ExportEvpnDomains<'a, Mode> (::data_views::Field<export_evpn_domains::Item<'a, Mode>>);

            pub mod export_evpn_domains {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub domain: ::data_views::Field<&'a str>,
                    pub route_target: ::data_views::Field<&'a str>,
                }
            }

            #[::data_views::data_view(list)]
            pub struct ImportExportEvpnDomains<'a, Mode> (::data_views::Field<import_export_evpn_domains::Item<'a, Mode>>);

            pub mod import_export_evpn_domains {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub domain: ::data_views::Field<&'a str>,
                    pub route_target: ::data_views::Field<&'a str>,
                }
            }
        }

        #[::data_views::data_view(list)]
        pub struct RedistributeRoutes<'a, Mode> (::data_views::Field<&'a str>);

        #[::data_views::data_view(list)]
        pub struct NoRedistributeRoutes<'a, Mode> (::data_views::Field<&'a str>);
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Vpws<'a, Mode> (::data_views::Field<vpws::Item<'a, Mode>>);

pub mod vpws {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub rd: ::data_views::Field<&'a str>,
        pub route_targets: ::data_views::Field<item::RouteTargets<'a, Mode>>,
        pub mpls_control_word: ::data_views::Field<bool>,
        pub label_flow: ::data_views::Field<bool>,
        pub mtu: ::data_views::Field<i64>,
        pub pseudowires: ::data_views::Field<item::Pseudowires<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct RouteTargets<'a, Mode> {
            pub import_export: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct Pseudowires<'a, Mode> (::data_views::Field<pseudowires::Item<'a, Mode>>);

        pub mod pseudowires {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
                pub id_local: ::data_views::Field<i64>,
                pub id_remote: ::data_views::Field<i64>,
            }
        }
    }
}

#[::data_views::data_view]
pub struct AddressFamilyEvpn<'a, Mode> {
    pub domain_identifier: ::data_views::Field<&'a str>,
    pub domain_identifier_remote: ::data_views::Field<&'a str>,
    pub neighbor_default: ::data_views::Field<address_family_evpn::NeighborDefault<'a, Mode>>,
    pub next_hop_mpls_resolution_ribs: ::data_views::Field<address_family_evpn::NextHopMplsResolutionRibs<'a, Mode>>,
    pub neighbors: ::data_views::Field<address_family_evpn::Neighbors<'a, Mode>>,
    pub peer_groups: ::data_views::Field<address_family_evpn::PeerGroups<'a, Mode>>,
    pub evpn_hostflap_detection: ::data_views::Field<address_family_evpn::EvpnHostflapDetection<'a, Mode>>,
    pub next_hop: ::data_views::Field<address_family_evpn::NextHop<'a, Mode>>,
    pub route: ::data_views::Field<address_family_evpn::Route<'a, Mode>>,
    pub next_hop_unchanged: ::data_views::Field<bool>,
    pub bgp: ::data_views::Field<address_family_evpn::Bgp<'a, Mode>>,
    pub layer_2_fec_in_place_update: ::data_views::Field<address_family_evpn::Layer2FecInPlaceUpdate<'a, Mode>>,
    pub evpn_ethernet_segment: ::data_views::Field<address_family_evpn::EvpnEthernetSegment<'a, Mode>>,
}

pub mod address_family_evpn {

    #[::data_views::data_view]
    pub struct NeighborDefault<'a, Mode> {
        pub encapsulation: ::data_views::Field<&'a str>,
        pub next_hop_self_source_interface: ::data_views::Field<&'a str>,
        pub next_hop_self_received_evpn_routes: ::data_views::Field<neighbor_default::NextHopSelfReceivedEvpnRoutes<'a, Mode>>,
    }

    pub mod neighbor_default {

        #[::data_views::data_view]
        pub struct NextHopSelfReceivedEvpnRoutes<'a, Mode> {
            pub enable: ::data_views::Field<bool>,
            pub inter_domain: ::data_views::Field<bool>,
        }
    }

    #[::data_views::data_view(list)]
    pub struct NextHopMplsResolutionRibs<'a, Mode> (::data_views::Field<next_hop_mpls_resolution_ribs::Item<'a, Mode>>);

    pub mod next_hop_mpls_resolution_ribs {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub rib_type: ::data_views::RequiredValue<&'a str, Mode>,
            pub rib_name: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::data_views::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub route_map_in: ::data_views::Field<&'a str>,
            pub route_map_out: ::data_views::Field<&'a str>,
            pub peer_tag_in: ::data_views::Field<&'a str>,
            pub peer_tag_out_discard: ::data_views::Field<&'a str>,
            pub rcf_in: ::data_views::Field<&'a str>,
            pub rcf_out: ::data_views::Field<&'a str>,
            pub default_route: ::data_views::Field<item::DefaultRoute<'a, Mode>>,
            pub additional_paths: ::data_views::Field<item::AdditionalPaths<'a, Mode>>,
            pub encapsulation: ::data_views::Field<&'a str>,
            pub next_hop_self_source_interface: ::data_views::Field<&'a str>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct DefaultRoute<'a, Mode> {
                pub enabled: ::data_views::Field<bool>,
                pub rcf: ::data_views::Field<&'a str>,
                pub route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub receive: ::data_views::Field<bool>,
                pub send: ::data_views::Field<&'a str>,
                pub send_limit: ::data_views::Field<i64>,
            }
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::data_views::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub route_map_in: ::data_views::Field<&'a str>,
            pub route_map_out: ::data_views::Field<&'a str>,
            pub peer_tag_in: ::data_views::Field<&'a str>,
            pub peer_tag_out_discard: ::data_views::Field<&'a str>,
            pub rcf_in: ::data_views::Field<&'a str>,
            pub rcf_out: ::data_views::Field<&'a str>,
            pub default_route: ::data_views::Field<item::DefaultRoute<'a, Mode>>,
            pub domain_remote: ::data_views::Field<bool>,
            pub encapsulation: ::data_views::Field<&'a str>,
            pub next_hop_self_source_interface: ::data_views::Field<&'a str>,
            pub additional_paths: ::data_views::Field<item::AdditionalPaths<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct DefaultRoute<'a, Mode> {
                pub enabled: ::data_views::Field<bool>,
                pub rcf: ::data_views::Field<&'a str>,
                pub route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub receive: ::data_views::Field<bool>,
                pub send: ::data_views::Field<&'a str>,
                pub send_limit: ::data_views::Field<i64>,
            }
        }
    }

    #[::data_views::data_view]
    pub struct EvpnHostflapDetection<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub window: ::data_views::Field<i64>,
        pub threshold: ::data_views::Field<i64>,
        pub expiry_timeout: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct NextHop<'a, Mode> {
        pub resolution_disabled: ::data_views::Field<bool>,
    }

    #[::data_views::data_view]
    pub struct Route<'a, Mode> {
        pub import_match_failure_action: ::data_views::Field<&'a str>,
        pub import_ethernet_segment_ip_mass_withdraw: ::data_views::Field<bool>,
        pub import_overlay_index_gateway: ::data_views::Field<bool>,
        pub export_ethernet_segment_ip_mass_withdraw: ::data_views::Field<bool>,
    }

    #[::data_views::data_view]
    pub struct Bgp<'a, Mode> {
        pub additional_paths: ::data_views::Field<bgp::AdditionalPaths<'a, Mode>>,
    }

    pub mod bgp {

        #[::data_views::data_view]
        pub struct AdditionalPaths<'a, Mode> {
            pub receive: ::data_views::Field<bool>,
            pub send: ::data_views::Field<&'a str>,
            pub send_limit: ::data_views::Field<i64>,
        }
    }

    #[::data_views::data_view]
    pub struct Layer2FecInPlaceUpdate<'a, Mode> {
        pub enabled: ::data_views::RequiredValue<bool, Mode>,
        pub timeout: ::data_views::Field<i64>,
    }

    #[::data_views::data_view(indexed_list, primary_key(domain))]
    pub struct EvpnEthernetSegment<'a, Mode> (::data_views::Field<evpn_ethernet_segment::Item<'a, Mode>>);

    pub mod evpn_ethernet_segment {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub domain: ::data_views::Field<&'a str>,
            pub identifier: ::data_views::Field<&'a str>,
            pub route_target_import: ::data_views::Field<&'a str>,
        }
    }
}

#[::data_views::data_view]
pub struct AddressFamilyRtc<'a, Mode> {
    pub peer_groups: ::data_views::Field<address_family_rtc::PeerGroups<'a, Mode>>,
}

pub mod address_family_rtc {

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::data_views::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub default_route_target: ::data_views::Field<item::DefaultRouteTarget<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct DefaultRouteTarget<'a, Mode> {
                pub only: ::data_views::Field<bool>,
                pub encoding_origin_as_omit: ::data_views::Field<&'a str>,
            }
        }
    }
}

#[::data_views::data_view]
pub struct AddressFamilyIpv4<'a, Mode> {
    pub networks: ::data_views::Field<address_family_ipv4::Networks<'a, Mode>>,
    pub bgp: ::data_views::Field<address_family_ipv4::Bgp<'a, Mode>>,
    pub peer_groups: ::data_views::Field<address_family_ipv4::PeerGroups<'a, Mode>>,
    pub neighbors: ::data_views::Field<address_family_ipv4::Neighbors<'a, Mode>>,
    pub redistribute: ::data_views::Field<address_family_ipv4::Redistribute<'a, Mode>>,
    pub next_hop: ::data_views::Field<address_family_ipv4::NextHop<'a, Mode>>,
}

pub mod address_family_ipv4 {

    #[::data_views::data_view(indexed_list, primary_key(prefix))]
    pub struct Networks<'a, Mode> (::data_views::Field<networks::Item<'a, Mode>>);

    pub mod networks {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub prefix: ::data_views::Field<&'a str>,
            pub route_map: ::data_views::Field<&'a str>,
            pub rcf: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view]
    pub struct Bgp<'a, Mode> {
        pub additional_paths: ::data_views::Field<bgp::AdditionalPaths<'a, Mode>>,
        pub redistribute_internal: ::data_views::Field<bool>,
    }

    pub mod bgp {

        #[::data_views::data_view]
        pub struct AdditionalPaths<'a, Mode> {
            pub install: ::data_views::Field<bool>,
            pub install_ecmp_primary: ::data_views::Field<bool>,
            pub receive: ::data_views::Field<bool>,
            pub send: ::data_views::Field<&'a str>,
            pub send_limit: ::data_views::Field<i64>,
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::data_views::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub route_map_in: ::data_views::Field<&'a str>,
            pub route_map_out: ::data_views::Field<&'a str>,
            pub peer_tag_in: ::data_views::Field<&'a str>,
            pub peer_tag_out_discard: ::data_views::Field<&'a str>,
            pub rcf_in: ::data_views::Field<&'a str>,
            pub rcf_out: ::data_views::Field<&'a str>,
            pub default_originate: ::data_views::Field<item::DefaultOriginate<'a, Mode>>,
            pub prefix_list_in: ::data_views::Field<&'a str>,
            pub prefix_list_out: ::data_views::Field<&'a str>,
            pub additional_paths: ::data_views::Field<item::AdditionalPaths<'a, Mode>>,
            pub next_hop: ::data_views::Field<item::NextHop<'a, Mode>>,
            pub maximum_advertised_routes: ::data_views::Field<i64>,
            pub maximum_advertised_routes_warning_limit: ::data_views::Field<&'a str>,
            pub maximum_accepted_routes: ::data_views::Field<item::MaximumAcceptedRoutes<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view(python(name = "EOSConfig.RouterBgp.AddressFamilyIpv4.PeerGroupsItemBase.DefaultOriginate", schema = Dict(977)))]
            pub struct DefaultOriginate<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub always: ::data_views::Field<bool>,
                #[data_view(schema = Str(169))]
                pub route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub prefix_list: ::data_views::Field<&'a str>,
                pub receive: ::data_views::Field<bool>,
                pub send: ::data_views::Field<&'a str>,
                pub send_limit: ::data_views::Field<i64>,
            }

            #[::data_views::data_view(python(name = "EOSConfig.RouterBgp.AddressFamilyIpv4.PeerGroupsItemBase.NextHop", schema = Dict(980)))]
            pub struct NextHop<'a, Mode> {
                #[data_view(schema = Dict(979))]
                pub address_family_ipv6: ::data_views::Field<next_hop::AddressFamilyIpv6<'a, Mode>>,
            }

            pub mod next_hop {

                #[::data_views::data_view(python(name = "EOSConfig.RouterBgp.AddressFamilyIpv4.PeerGroupsItemBase.NextHop.AddressFamilyIpv6", schema = Dict(979)))]
                pub struct AddressFamilyIpv6<'a, Mode> {
                    #[data_view(schema = Bool(1))]
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    #[data_view(schema = Bool(7))]
                    pub originate: ::data_views::Field<bool>,
                }
            }

            #[::data_views::data_view]
            pub struct MaximumAcceptedRoutes<'a, Mode> {
                pub limit: ::data_views::RequiredValue<i64, Mode>,
                pub warning_limit: ::data_views::Field<maximum_accepted_routes::WarningLimit<'a, Mode>>,
            }

            pub mod maximum_accepted_routes {

                #[::data_views::data_view]
                pub struct WarningLimit<'a, Mode> {
                    pub count: ::data_views::Field<i64>,
                    pub percent: ::data_views::Field<i64>,
                }
            }
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::data_views::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub route_map_in: ::data_views::Field<&'a str>,
            pub route_map_out: ::data_views::Field<&'a str>,
            pub peer_tag_in: ::data_views::Field<&'a str>,
            pub peer_tag_out_discard: ::data_views::Field<&'a str>,
            pub rcf_in: ::data_views::Field<&'a str>,
            pub rcf_out: ::data_views::Field<&'a str>,
            pub prefix_list_in: ::data_views::Field<&'a str>,
            pub prefix_list_out: ::data_views::Field<&'a str>,
            pub default_originate: ::data_views::Field<item::DefaultOriginate<'a, Mode>>,
            pub additional_paths: ::data_views::Field<item::AdditionalPaths<'a, Mode>>,
            pub next_hop: ::data_views::Field<item::NextHop<'a, Mode>>,
            pub maximum_advertised_routes: ::data_views::Field<i64>,
            pub maximum_advertised_routes_warning_limit: ::data_views::Field<&'a str>,
            pub maximum_accepted_routes: ::data_views::Field<item::MaximumAcceptedRoutes<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct DefaultOriginate<'a, Mode> {
                pub always: ::data_views::Field<bool>,
                pub route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub prefix_list: ::data_views::Field<&'a str>,
                pub receive: ::data_views::Field<bool>,
                pub send: ::data_views::Field<&'a str>,
                pub send_limit: ::data_views::Field<i64>,
            }

            #[::data_views::data_view]
            pub struct NextHop<'a, Mode> {
                pub address_family_ipv6: ::data_views::Field<next_hop::AddressFamilyIpv6<'a, Mode>>,
            }

            pub mod next_hop {

                #[::data_views::data_view]
                pub struct AddressFamilyIpv6<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub originate: ::data_views::Field<bool>,
                }
            }

            #[::data_views::data_view]
            pub struct MaximumAcceptedRoutes<'a, Mode> {
                pub limit: ::data_views::RequiredValue<i64, Mode>,
                pub warning_limit: ::data_views::Field<maximum_accepted_routes::WarningLimit<'a, Mode>>,
            }

            pub mod maximum_accepted_routes {

                #[::data_views::data_view]
                pub struct WarningLimit<'a, Mode> {
                    pub count: ::data_views::Field<i64>,
                    pub percent: ::data_views::Field<i64>,
                }
            }
        }
    }

    #[::data_views::data_view]
    pub struct Redistribute<'a, Mode> {
        pub attached_host: ::data_views::Field<redistribute::AttachedHost<'a, Mode>>,
        pub bgp: ::data_views::Field<redistribute::Bgp<'a, Mode>>,
        pub connected: ::data_views::Field<redistribute::Connected<'a, Mode>>,
        pub dynamic: ::data_views::Field<redistribute::Dynamic<'a, Mode>>,
        pub isis: ::data_views::Field<redistribute::Isis<'a, Mode>>,
        pub ospf: ::data_views::Field<redistribute::Ospf<'a, Mode>>,
        pub ospfv3: ::data_views::Field<redistribute::Ospfv3<'a, Mode>>,
        pub rip: ::data_views::Field<redistribute::Rip<'a, Mode>>,
        #[data_view(rename = "static")]
        pub field_static: ::data_views::Field<redistribute::FieldStatic<'a, Mode>>,
        pub user: ::data_views::Field<redistribute::User<'a, Mode>>,
    }

    pub mod redistribute {

        #[::data_views::data_view]
        pub struct AttachedHost<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct Bgp<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct Connected<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
            pub rcf: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct Dynamic<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
            pub rcf: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct Isis<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub isis_level: ::data_views::Field<&'a str>,
            pub route_map: ::data_views::Field<&'a str>,
            pub rcf: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct Ospf<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub match_external: ::data_views::Field<ospf::MatchExternal<'a, Mode>>,
            pub match_internal: ::data_views::Field<ospf::MatchInternal<'a, Mode>>,
            pub match_nssa_external: ::data_views::Field<ospf::MatchNssaExternal<'a, Mode>>,
            pub route_map: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }

        pub mod ospf {

            #[::data_views::data_view]
            pub struct MatchExternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
                pub include_leaked: ::data_views::Field<bool>,
            }

            #[::data_views::data_view]
            pub struct MatchInternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
                pub include_leaked: ::data_views::Field<bool>,
            }

            #[::data_views::data_view]
            pub struct MatchNssaExternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub nssa_type: ::data_views::Field<i64>,
                pub route_map: ::data_views::Field<&'a str>,
                pub include_leaked: ::data_views::Field<bool>,
            }
        }

        #[::data_views::data_view]
        pub struct Ospfv3<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub match_external: ::data_views::Field<ospfv3::MatchExternal<'a, Mode>>,
            pub match_internal: ::data_views::Field<ospfv3::MatchInternal<'a, Mode>>,
            pub match_nssa_external: ::data_views::Field<ospfv3::MatchNssaExternal<'a, Mode>>,
            pub route_map: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }

        pub mod ospfv3 {

            #[::data_views::data_view]
            pub struct MatchExternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
                pub include_leaked: ::data_views::Field<bool>,
            }

            #[::data_views::data_view]
            pub struct MatchInternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
                pub include_leaked: ::data_views::Field<bool>,
            }

            #[::data_views::data_view]
            pub struct MatchNssaExternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub nssa_type: ::data_views::Field<i64>,
                pub route_map: ::data_views::Field<&'a str>,
                pub include_leaked: ::data_views::Field<bool>,
            }
        }

        #[::data_views::data_view]
        pub struct Rip<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct FieldStatic<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
            pub rcf: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct User<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub rcf: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view]
    pub struct NextHop<'a, Mode> {
        pub resolution_disabled: ::data_views::Field<bool>,
    }
}

#[::data_views::data_view]
pub struct AddressFamilyIpv4LabeledUnicast<'a, Mode> {
    pub aigp_session: ::data_views::Field<address_family_ipv4_labeled_unicast::AigpSession<'a, Mode>>,
    pub bgp: ::data_views::Field<address_family_ipv4_labeled_unicast::Bgp<'a, Mode>>,
    pub graceful_restart: ::data_views::Field<bool>,
    pub label_local_termination: ::data_views::Field<&'a str>,
    pub lfib_entry_installation_skipped: ::data_views::Field<bool>,
    pub neighbor_default: ::data_views::Field<address_family_ipv4_labeled_unicast::NeighborDefault<'a, Mode>>,
    pub peer_groups: ::data_views::Field<address_family_ipv4_labeled_unicast::PeerGroups<'a, Mode>>,
    pub neighbors: ::data_views::Field<address_family_ipv4_labeled_unicast::Neighbors<'a, Mode>>,
    pub networks: ::data_views::Field<address_family_ipv4_labeled_unicast::Networks<'a, Mode>>,
    pub next_hop: ::data_views::Field<address_family_ipv4_labeled_unicast::NextHop<'a, Mode>>,
    pub next_hops: ::data_views::Field<address_family_ipv4_labeled_unicast::NextHops<'a, Mode>>,
    pub next_hop_resolution_ribs: ::data_views::Field<address_family_ipv4_labeled_unicast::NextHopResolutionRibs<'a, Mode>>,
    pub tunnel_source_protocols: ::data_views::Field<address_family_ipv4_labeled_unicast::TunnelSourceProtocols<'a, Mode>>,
    pub update_wait_for_convergence: ::data_views::Field<bool>,
}

pub mod address_family_ipv4_labeled_unicast {

    #[::data_views::data_view]
    pub struct AigpSession<'a, Mode> {
        pub confederation: ::data_views::Field<bool>,
        pub ebgp: ::data_views::Field<bool>,
        pub ibgp: ::data_views::Field<bool>,
    }

    #[::data_views::data_view]
    pub struct Bgp<'a, Mode> {
        pub additional_paths: ::data_views::Field<bgp::AdditionalPaths<'a, Mode>>,
        pub missing_policy: ::data_views::Field<bgp::MissingPolicy<'a, Mode>>,
        pub next_hop_unchanged: ::data_views::Field<bool>,
    }

    pub mod bgp {

        #[::data_views::data_view]
        pub struct AdditionalPaths<'a, Mode> {
            pub receive: ::data_views::Field<bool>,
            pub send: ::data_views::Field<&'a str>,
            pub send_limit: ::data_views::Field<i64>,
        }

        #[::data_views::data_view]
        pub struct MissingPolicy<'a, Mode> {
            pub direction_in: ::data_views::Field<missing_policy::DirectionIn<'a, Mode>>,
            pub direction_out: ::data_views::Field<missing_policy::DirectionOut<'a, Mode>>,
        }

        pub mod missing_policy {

            #[::data_views::data_view]
            pub struct DirectionIn<'a, Mode> {
                pub action: ::data_views::RequiredValue<&'a str, Mode>,
                pub include_community_list: ::data_views::Field<bool>,
                pub include_prefix_list: ::data_views::Field<bool>,
                pub include_sub_route_map: ::data_views::Field<bool>,
            }

            #[::data_views::data_view]
            pub struct DirectionOut<'a, Mode> {
                pub action: ::data_views::RequiredValue<&'a str, Mode>,
                pub include_community_list: ::data_views::Field<bool>,
                pub include_prefix_list: ::data_views::Field<bool>,
                pub include_sub_route_map: ::data_views::Field<bool>,
            }
        }
    }

    #[::data_views::data_view]
    pub struct NeighborDefault<'a, Mode> {
        pub next_hop_self: ::data_views::Field<bool>,
    }

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::data_views::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub additional_paths: ::data_views::Field<item::AdditionalPaths<'a, Mode>>,
            pub aigp_session: ::data_views::Field<bool>,
            pub graceful_restart: ::data_views::Field<bool>,
            pub graceful_restart_helper: ::data_views::Field<item::GracefulRestartHelper<'a, Mode>>,
            pub maximum_advertised_routes: ::data_views::Field<i64>,
            pub maximum_advertised_routes_warning_limit: ::data_views::Field<&'a str>,
            pub missing_policy: ::data_views::Field<item::MissingPolicy<'a, Mode>>,
            pub multi_path: ::data_views::Field<bool>,
            pub next_hop_self: ::data_views::Field<bool>,
            pub next_hop_self_source_interface: ::data_views::Field<&'a str>,
            pub next_hop_self_v4_mapped_v6_source_interface: ::data_views::Field<&'a str>,
            pub next_hop_unchanged: ::data_views::Field<bool>,
            pub rcf_in: ::data_views::Field<&'a str>,
            pub rcf_out: ::data_views::Field<&'a str>,
            pub route_map_in: ::data_views::Field<&'a str>,
            pub route_map_out: ::data_views::Field<&'a str>,
            pub peer_tag_in: ::data_views::Field<&'a str>,
            pub peer_tag_out_discard: ::data_views::Field<&'a str>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub receive: ::data_views::Field<bool>,
                pub send: ::data_views::Field<&'a str>,
                pub send_limit: ::data_views::Field<i64>,
            }

            #[::data_views::data_view]
            pub struct GracefulRestartHelper<'a, Mode> {
                pub stale_route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct MissingPolicy<'a, Mode> {
                pub direction_in: ::data_views::Field<missing_policy::DirectionIn<'a, Mode>>,
                pub direction_out: ::data_views::Field<missing_policy::DirectionOut<'a, Mode>>,
            }

            pub mod missing_policy {

                #[::data_views::data_view]
                pub struct DirectionIn<'a, Mode> {
                    pub action: ::data_views::RequiredValue<&'a str, Mode>,
                    pub include_community_list: ::data_views::Field<bool>,
                    pub include_prefix_list: ::data_views::Field<bool>,
                    pub include_sub_route_map: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct DirectionOut<'a, Mode> {
                    pub action: ::data_views::RequiredValue<&'a str, Mode>,
                    pub include_community_list: ::data_views::Field<bool>,
                    pub include_prefix_list: ::data_views::Field<bool>,
                    pub include_sub_route_map: ::data_views::Field<bool>,
                }
            }
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::data_views::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub additional_paths: ::data_views::Field<item::AdditionalPaths<'a, Mode>>,
            pub aigp_session: ::data_views::Field<bool>,
            pub graceful_restart: ::data_views::Field<bool>,
            pub graceful_restart_helper: ::data_views::Field<item::GracefulRestartHelper<'a, Mode>>,
            pub maximum_advertised_routes: ::data_views::Field<i64>,
            pub maximum_advertised_routes_warning_limit: ::data_views::Field<&'a str>,
            pub missing_policy: ::data_views::Field<item::MissingPolicy<'a, Mode>>,
            pub multi_path: ::data_views::Field<bool>,
            pub next_hop_self: ::data_views::Field<bool>,
            pub next_hop_self_source_interface: ::data_views::Field<&'a str>,
            pub next_hop_self_v4_mapped_v6_source_interface: ::data_views::Field<&'a str>,
            pub next_hop_unchanged: ::data_views::Field<bool>,
            pub rcf_in: ::data_views::Field<&'a str>,
            pub rcf_out: ::data_views::Field<&'a str>,
            pub route_map_in: ::data_views::Field<&'a str>,
            pub route_map_out: ::data_views::Field<&'a str>,
            pub peer_tag_in: ::data_views::Field<&'a str>,
            pub peer_tag_out_discard: ::data_views::Field<&'a str>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub receive: ::data_views::Field<bool>,
                pub send: ::data_views::Field<&'a str>,
                pub send_limit: ::data_views::Field<i64>,
            }

            #[::data_views::data_view]
            pub struct GracefulRestartHelper<'a, Mode> {
                pub stale_route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct MissingPolicy<'a, Mode> {
                pub direction_in: ::data_views::Field<missing_policy::DirectionIn<'a, Mode>>,
                pub direction_out: ::data_views::Field<missing_policy::DirectionOut<'a, Mode>>,
            }

            pub mod missing_policy {

                #[::data_views::data_view]
                pub struct DirectionIn<'a, Mode> {
                    pub action: ::data_views::RequiredValue<&'a str, Mode>,
                    pub include_community_list: ::data_views::Field<bool>,
                    pub include_prefix_list: ::data_views::Field<bool>,
                    pub include_sub_route_map: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct DirectionOut<'a, Mode> {
                    pub action: ::data_views::RequiredValue<&'a str, Mode>,
                    pub include_community_list: ::data_views::Field<bool>,
                    pub include_prefix_list: ::data_views::Field<bool>,
                    pub include_sub_route_map: ::data_views::Field<bool>,
                }
            }
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(prefix))]
    pub struct Networks<'a, Mode> (::data_views::Field<networks::Item<'a, Mode>>);

    pub mod networks {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub prefix: ::data_views::Field<&'a str>,
            pub route_map: ::data_views::Field<&'a str>,
            pub rcf: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view]
    pub struct NextHop<'a, Mode> {
        pub resolution_disabled: ::data_views::Field<bool>,
    }

    #[::data_views::data_view(indexed_list, primary_key(ip_address))]
    pub struct NextHops<'a, Mode> (::data_views::Field<next_hops::Item<'a, Mode>>);

    pub mod next_hops {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::data_views::Field<&'a str>,
            pub lfib_backup_ip_forwarding: ::data_views::Field<bool>,
        }
    }

    #[::data_views::data_view(list)]
    pub struct NextHopResolutionRibs<'a, Mode> (::data_views::Field<next_hop_resolution_ribs::Item<'a, Mode>>);

    pub mod next_hop_resolution_ribs {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub rib_type: ::data_views::RequiredValue<&'a str, Mode>,
            pub rib_name: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(protocol))]
    pub struct TunnelSourceProtocols<'a, Mode> (::data_views::Field<tunnel_source_protocols::Item<'a, Mode>>);

    pub mod tunnel_source_protocols {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub protocol: ::data_views::Field<&'a str>,
            pub rcf: ::data_views::Field<&'a str>,
        }
    }
}

#[::data_views::data_view]
pub struct AddressFamilyIpv4Multicast<'a, Mode> {
    pub bgp: ::data_views::Field<address_family_ipv4_multicast::Bgp<'a, Mode>>,
    pub peer_groups: ::data_views::Field<address_family_ipv4_multicast::PeerGroups<'a, Mode>>,
    pub neighbors: ::data_views::Field<address_family_ipv4_multicast::Neighbors<'a, Mode>>,
    pub redistribute: ::data_views::Field<address_family_ipv4_multicast::Redistribute<'a, Mode>>,
}

pub mod address_family_ipv4_multicast {

    #[::data_views::data_view]
    pub struct Bgp<'a, Mode> {
        pub additional_paths: ::data_views::Field<bgp::AdditionalPaths<'a, Mode>>,
    }

    pub mod bgp {

        #[::data_views::data_view]
        pub struct AdditionalPaths<'a, Mode> {
            pub receive: ::data_views::Field<bool>,
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::data_views::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub route_map_in: ::data_views::Field<&'a str>,
            pub route_map_out: ::data_views::Field<&'a str>,
            pub peer_tag_in: ::data_views::Field<&'a str>,
            pub peer_tag_out_discard: ::data_views::Field<&'a str>,
            pub additional_paths: ::data_views::Field<item::AdditionalPaths<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub receive: ::data_views::Field<bool>,
            }
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::data_views::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub route_map_in: ::data_views::Field<&'a str>,
            pub route_map_out: ::data_views::Field<&'a str>,
            pub peer_tag_in: ::data_views::Field<&'a str>,
            pub peer_tag_out_discard: ::data_views::Field<&'a str>,
            pub additional_paths: ::data_views::Field<item::AdditionalPaths<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub receive: ::data_views::Field<bool>,
            }
        }
    }

    #[::data_views::data_view]
    pub struct Redistribute<'a, Mode> {
        pub attached_host: ::data_views::Field<redistribute::AttachedHost<'a, Mode>>,
        pub connected: ::data_views::Field<redistribute::Connected<'a, Mode>>,
        pub isis: ::data_views::Field<redistribute::Isis<'a, Mode>>,
        pub ospf: ::data_views::Field<redistribute::Ospf<'a, Mode>>,
        pub ospfv3: ::data_views::Field<redistribute::Ospfv3<'a, Mode>>,
        #[data_view(rename = "static")]
        pub field_static: ::data_views::Field<redistribute::FieldStatic<'a, Mode>>,
    }

    pub mod redistribute {

        #[::data_views::data_view]
        pub struct AttachedHost<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct Connected<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct Isis<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub isis_level: ::data_views::Field<&'a str>,
            pub route_map: ::data_views::Field<&'a str>,
            pub rcf: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct Ospf<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub match_external: ::data_views::Field<ospf::MatchExternal<'a, Mode>>,
            pub match_internal: ::data_views::Field<ospf::MatchInternal<'a, Mode>>,
            pub match_nssa_external: ::data_views::Field<ospf::MatchNssaExternal<'a, Mode>>,
            pub route_map: ::data_views::Field<&'a str>,
        }

        pub mod ospf {

            #[::data_views::data_view]
            pub struct MatchExternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct MatchInternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct MatchNssaExternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub nssa_type: ::data_views::Field<i64>,
                pub route_map: ::data_views::Field<&'a str>,
            }
        }

        #[::data_views::data_view]
        pub struct Ospfv3<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub match_external: ::data_views::Field<ospfv3::MatchExternal<'a, Mode>>,
            pub match_internal: ::data_views::Field<ospfv3::MatchInternal<'a, Mode>>,
            pub match_nssa_external: ::data_views::Field<ospfv3::MatchNssaExternal<'a, Mode>>,
            pub route_map: ::data_views::Field<&'a str>,
        }

        pub mod ospfv3 {

            #[::data_views::data_view]
            pub struct MatchExternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct MatchInternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct MatchNssaExternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub nssa_type: ::data_views::Field<i64>,
                pub route_map: ::data_views::Field<&'a str>,
            }
        }

        #[::data_views::data_view]
        pub struct FieldStatic<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
        }
    }
}

#[::data_views::data_view]
pub struct AddressFamilyIpv4SrTe<'a, Mode> {
    pub neighbors: ::data_views::Field<address_family_ipv4_sr_te::Neighbors<'a, Mode>>,
    pub peer_groups: ::data_views::Field<address_family_ipv4_sr_te::PeerGroups<'a, Mode>>,
}

pub mod address_family_ipv4_sr_te {

    #[::data_views::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::data_views::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub route_map_in: ::data_views::Field<&'a str>,
            pub route_map_out: ::data_views::Field<&'a str>,
            pub peer_tag_in: ::data_views::Field<&'a str>,
            pub peer_tag_out_discard: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::data_views::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub route_map_in: ::data_views::Field<&'a str>,
            pub route_map_out: ::data_views::Field<&'a str>,
            pub peer_tag_in: ::data_views::Field<&'a str>,
            pub peer_tag_out_discard: ::data_views::Field<&'a str>,
        }
    }
}

#[::data_views::data_view]
pub struct AddressFamilyIpv6<'a, Mode> {
    pub networks: ::data_views::Field<address_family_ipv6::Networks<'a, Mode>>,
    pub bgp: ::data_views::Field<address_family_ipv6::Bgp<'a, Mode>>,
    pub peer_groups: ::data_views::Field<address_family_ipv6::PeerGroups<'a, Mode>>,
    pub neighbors: ::data_views::Field<address_family_ipv6::Neighbors<'a, Mode>>,
    pub redistribute: ::data_views::Field<address_family_ipv6::Redistribute<'a, Mode>>,
    pub next_hop: ::data_views::Field<address_family_ipv6::NextHop<'a, Mode>>,
}

pub mod address_family_ipv6 {

    #[::data_views::data_view(indexed_list, primary_key(prefix))]
    pub struct Networks<'a, Mode> (::data_views::Field<networks::Item<'a, Mode>>);

    pub mod networks {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub prefix: ::data_views::Field<&'a str>,
            pub route_map: ::data_views::Field<&'a str>,
            pub rcf: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view]
    pub struct Bgp<'a, Mode> {
        pub redistribute_internal: ::data_views::Field<bool>,
        pub additional_paths: ::data_views::Field<bgp::AdditionalPaths<'a, Mode>>,
    }

    pub mod bgp {

        #[::data_views::data_view]
        pub struct AdditionalPaths<'a, Mode> {
            pub install: ::data_views::Field<bool>,
            pub install_ecmp_primary: ::data_views::Field<bool>,
            pub receive: ::data_views::Field<bool>,
            pub send: ::data_views::Field<&'a str>,
            pub send_limit: ::data_views::Field<i64>,
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::data_views::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub route_map_in: ::data_views::Field<&'a str>,
            pub route_map_out: ::data_views::Field<&'a str>,
            pub peer_tag_in: ::data_views::Field<&'a str>,
            pub peer_tag_out_discard: ::data_views::Field<&'a str>,
            pub rcf_in: ::data_views::Field<&'a str>,
            pub rcf_out: ::data_views::Field<&'a str>,
            pub prefix_list_in: ::data_views::Field<&'a str>,
            pub prefix_list_out: ::data_views::Field<&'a str>,
            pub additional_paths: ::data_views::Field<item::AdditionalPaths<'a, Mode>>,
            pub default_originate: ::data_views::Field<item::DefaultOriginate<'a, Mode>>,
            pub maximum_advertised_routes: ::data_views::Field<i64>,
            pub maximum_advertised_routes_warning_limit: ::data_views::Field<&'a str>,
            pub maximum_accepted_routes: ::data_views::Field<item::MaximumAcceptedRoutes<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub prefix_list: ::data_views::Field<&'a str>,
                pub receive: ::data_views::Field<bool>,
                pub send: ::data_views::Field<&'a str>,
                pub send_limit: ::data_views::Field<i64>,
            }

            #[::data_views::data_view]
            pub struct DefaultOriginate<'a, Mode> {
                pub enabled: ::data_views::Field<bool>,
                pub always: ::data_views::Field<bool>,
                pub route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct MaximumAcceptedRoutes<'a, Mode> {
                pub limit: ::data_views::RequiredValue<i64, Mode>,
                pub warning_limit: ::data_views::Field<maximum_accepted_routes::WarningLimit<'a, Mode>>,
            }

            pub mod maximum_accepted_routes {

                #[::data_views::data_view]
                pub struct WarningLimit<'a, Mode> {
                    pub count: ::data_views::Field<i64>,
                    pub percent: ::data_views::Field<i64>,
                }
            }
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::data_views::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub route_map_in: ::data_views::Field<&'a str>,
            pub route_map_out: ::data_views::Field<&'a str>,
            pub peer_tag_in: ::data_views::Field<&'a str>,
            pub peer_tag_out_discard: ::data_views::Field<&'a str>,
            pub rcf_in: ::data_views::Field<&'a str>,
            pub rcf_out: ::data_views::Field<&'a str>,
            pub prefix_list_in: ::data_views::Field<&'a str>,
            pub prefix_list_out: ::data_views::Field<&'a str>,
            pub default_originate: ::data_views::Field<item::DefaultOriginate<'a, Mode>>,
            pub additional_paths: ::data_views::Field<item::AdditionalPaths<'a, Mode>>,
            pub maximum_advertised_routes: ::data_views::Field<i64>,
            pub maximum_advertised_routes_warning_limit: ::data_views::Field<&'a str>,
            pub maximum_accepted_routes: ::data_views::Field<item::MaximumAcceptedRoutes<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct DefaultOriginate<'a, Mode> {
                pub enabled: ::data_views::Field<bool>,
                pub always: ::data_views::Field<bool>,
                pub route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub prefix_list: ::data_views::Field<&'a str>,
                pub receive: ::data_views::Field<bool>,
                pub send: ::data_views::Field<&'a str>,
                pub send_limit: ::data_views::Field<i64>,
            }

            #[::data_views::data_view]
            pub struct MaximumAcceptedRoutes<'a, Mode> {
                pub limit: ::data_views::RequiredValue<i64, Mode>,
                pub warning_limit: ::data_views::Field<maximum_accepted_routes::WarningLimit<'a, Mode>>,
            }

            pub mod maximum_accepted_routes {

                #[::data_views::data_view]
                pub struct WarningLimit<'a, Mode> {
                    pub count: ::data_views::Field<i64>,
                    pub percent: ::data_views::Field<i64>,
                }
            }
        }
    }

    #[::data_views::data_view]
    pub struct Redistribute<'a, Mode> {
        pub attached_host: ::data_views::Field<redistribute::AttachedHost<'a, Mode>>,
        pub bgp: ::data_views::Field<redistribute::Bgp<'a, Mode>>,
        pub connected: ::data_views::Field<redistribute::Connected<'a, Mode>>,
        pub dhcp: ::data_views::Field<redistribute::Dhcp<'a, Mode>>,
        pub dynamic: ::data_views::Field<redistribute::Dynamic<'a, Mode>>,
        pub isis: ::data_views::Field<redistribute::Isis<'a, Mode>>,
        pub ospfv3: ::data_views::Field<redistribute::Ospfv3<'a, Mode>>,
        #[data_view(rename = "static")]
        pub field_static: ::data_views::Field<redistribute::FieldStatic<'a, Mode>>,
        pub user: ::data_views::Field<redistribute::User<'a, Mode>>,
    }

    pub mod redistribute {

        #[::data_views::data_view]
        pub struct AttachedHost<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct Bgp<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct Connected<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
            pub rcf: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct Dhcp<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct Dynamic<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
            pub rcf: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct Isis<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub isis_level: ::data_views::Field<&'a str>,
            pub route_map: ::data_views::Field<&'a str>,
            pub rcf: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct Ospfv3<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub match_external: ::data_views::Field<ospfv3::MatchExternal<'a, Mode>>,
            pub match_internal: ::data_views::Field<ospfv3::MatchInternal<'a, Mode>>,
            pub match_nssa_external: ::data_views::Field<ospfv3::MatchNssaExternal<'a, Mode>>,
            pub route_map: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }

        pub mod ospfv3 {

            #[::data_views::data_view]
            pub struct MatchExternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
                pub include_leaked: ::data_views::Field<bool>,
            }

            #[::data_views::data_view]
            pub struct MatchInternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
                pub include_leaked: ::data_views::Field<bool>,
            }

            #[::data_views::data_view]
            pub struct MatchNssaExternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub nssa_type: ::data_views::Field<i64>,
                pub route_map: ::data_views::Field<&'a str>,
                pub include_leaked: ::data_views::Field<bool>,
            }
        }

        #[::data_views::data_view]
        pub struct FieldStatic<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
            pub rcf: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct User<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub rcf: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view]
    pub struct NextHop<'a, Mode> {
        pub resolution_disabled: ::data_views::Field<bool>,
    }
}

#[::data_views::data_view]
pub struct AddressFamilyIpv6Multicast<'a, Mode> {
    pub bgp: ::data_views::Field<address_family_ipv6_multicast::Bgp<'a, Mode>>,
    pub neighbors: ::data_views::Field<address_family_ipv6_multicast::Neighbors<'a, Mode>>,
    pub peer_groups: ::data_views::Field<address_family_ipv6_multicast::PeerGroups<'a, Mode>>,
    pub networks: ::data_views::Field<address_family_ipv6_multicast::Networks<'a, Mode>>,
    pub redistribute: ::data_views::Field<address_family_ipv6_multicast::Redistribute<'a, Mode>>,
}

pub mod address_family_ipv6_multicast {

    #[::data_views::data_view]
    pub struct Bgp<'a, Mode> {
        pub missing_policy: ::data_views::Field<bgp::MissingPolicy<'a, Mode>>,
        pub additional_paths: ::data_views::Field<bgp::AdditionalPaths<'a, Mode>>,
    }

    pub mod bgp {

        #[::data_views::data_view]
        pub struct MissingPolicy<'a, Mode> {
            pub direction_in_action: ::data_views::Field<&'a str>,
            pub direction_out_action: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct AdditionalPaths<'a, Mode> {
            pub receive: ::data_views::Field<bool>,
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::data_views::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub route_map_in: ::data_views::Field<&'a str>,
            pub route_map_out: ::data_views::Field<&'a str>,
            pub peer_tag_in: ::data_views::Field<&'a str>,
            pub peer_tag_out_discard: ::data_views::Field<&'a str>,
            pub additional_paths: ::data_views::Field<item::AdditionalPaths<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub receive: ::data_views::Field<bool>,
            }
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::data_views::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub additional_paths: ::data_views::Field<item::AdditionalPaths<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub receive: ::data_views::Field<bool>,
            }
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(prefix))]
    pub struct Networks<'a, Mode> (::data_views::Field<networks::Item<'a, Mode>>);

    pub mod networks {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub prefix: ::data_views::Field<&'a str>,
            pub route_map: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view]
    pub struct Redistribute<'a, Mode> {
        pub connected: ::data_views::Field<redistribute::Connected<'a, Mode>>,
        pub isis: ::data_views::Field<redistribute::Isis<'a, Mode>>,
        pub ospf: ::data_views::Field<redistribute::Ospf<'a, Mode>>,
        pub ospfv3: ::data_views::Field<redistribute::Ospfv3<'a, Mode>>,
        #[data_view(rename = "static")]
        pub field_static: ::data_views::Field<redistribute::FieldStatic<'a, Mode>>,
    }

    pub mod redistribute {

        #[::data_views::data_view]
        pub struct Connected<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct Isis<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub isis_level: ::data_views::Field<&'a str>,
            pub route_map: ::data_views::Field<&'a str>,
            pub rcf: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct Ospf<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub match_external: ::data_views::Field<ospf::MatchExternal<'a, Mode>>,
            pub match_internal: ::data_views::Field<ospf::MatchInternal<'a, Mode>>,
            pub match_nssa_external: ::data_views::Field<ospf::MatchNssaExternal<'a, Mode>>,
            pub route_map: ::data_views::Field<&'a str>,
        }

        pub mod ospf {

            #[::data_views::data_view]
            pub struct MatchExternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct MatchInternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct MatchNssaExternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub nssa_type: ::data_views::Field<i64>,
                pub route_map: ::data_views::Field<&'a str>,
            }
        }

        #[::data_views::data_view]
        pub struct Ospfv3<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub match_external: ::data_views::Field<ospfv3::MatchExternal<'a, Mode>>,
            pub match_internal: ::data_views::Field<ospfv3::MatchInternal<'a, Mode>>,
            pub match_nssa_external: ::data_views::Field<ospfv3::MatchNssaExternal<'a, Mode>>,
            pub route_map: ::data_views::Field<&'a str>,
        }

        pub mod ospfv3 {

            #[::data_views::data_view]
            pub struct MatchExternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct MatchInternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct MatchNssaExternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub nssa_type: ::data_views::Field<i64>,
                pub route_map: ::data_views::Field<&'a str>,
            }
        }

        #[::data_views::data_view]
        pub struct FieldStatic<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
        }
    }
}

#[::data_views::data_view]
pub struct AddressFamilyIpv6SrTe<'a, Mode> {
    pub neighbors: ::data_views::Field<address_family_ipv6_sr_te::Neighbors<'a, Mode>>,
    pub peer_groups: ::data_views::Field<address_family_ipv6_sr_te::PeerGroups<'a, Mode>>,
}

pub mod address_family_ipv6_sr_te {

    #[::data_views::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::data_views::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub route_map_in: ::data_views::Field<&'a str>,
            pub route_map_out: ::data_views::Field<&'a str>,
            pub peer_tag_in: ::data_views::Field<&'a str>,
            pub peer_tag_out_discard: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::data_views::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub route_map_in: ::data_views::Field<&'a str>,
            pub route_map_out: ::data_views::Field<&'a str>,
            pub peer_tag_in: ::data_views::Field<&'a str>,
            pub peer_tag_out_discard: ::data_views::Field<&'a str>,
        }
    }
}

#[::data_views::data_view]
pub struct AddressFamilyLinkState<'a, Mode> {
    pub bgp: ::data_views::Field<address_family_link_state::Bgp<'a, Mode>>,
    pub peer_groups: ::data_views::Field<address_family_link_state::PeerGroups<'a, Mode>>,
    pub neighbors: ::data_views::Field<address_family_link_state::Neighbors<'a, Mode>>,
    pub path_selection: ::data_views::Field<address_family_link_state::PathSelection<'a, Mode>>,
}

pub mod address_family_link_state {

    #[::data_views::data_view]
    pub struct Bgp<'a, Mode> {
        pub missing_policy: ::data_views::Field<bgp::MissingPolicy<'a, Mode>>,
    }

    pub mod bgp {

        #[::data_views::data_view]
        pub struct MissingPolicy<'a, Mode> {
            pub direction_in_action: ::data_views::Field<&'a str>,
            pub direction_out_action: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::data_views::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub missing_policy: ::data_views::Field<item::MissingPolicy<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct MissingPolicy<'a, Mode> {
                pub direction_in_action: ::data_views::Field<&'a str>,
                pub direction_out_action: ::data_views::Field<&'a str>,
            }
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::data_views::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub missing_policy: ::data_views::Field<item::MissingPolicy<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct MissingPolicy<'a, Mode> {
                pub direction_in_action: ::data_views::Field<&'a str>,
                pub direction_out_action: ::data_views::Field<&'a str>,
            }
        }
    }

    #[::data_views::data_view]
    pub struct PathSelection<'a, Mode> {
        pub roles: ::data_views::Field<path_selection::Roles<'a, Mode>>,
    }

    pub mod path_selection {

        #[::data_views::data_view]
        pub struct Roles<'a, Mode> {
            pub producer: ::data_views::Field<bool>,
            pub consumer: ::data_views::Field<bool>,
            pub propagator: ::data_views::Field<bool>,
        }
    }
}

#[::data_views::data_view]
pub struct AddressFamilyFlowSpecIpv4<'a, Mode> {
    pub bgp: ::data_views::Field<address_family_flow_spec_ipv4::Bgp<'a, Mode>>,
    pub neighbors: ::data_views::Field<address_family_flow_spec_ipv4::Neighbors<'a, Mode>>,
    pub peer_groups: ::data_views::Field<address_family_flow_spec_ipv4::PeerGroups<'a, Mode>>,
}

pub mod address_family_flow_spec_ipv4 {

    #[::data_views::data_view]
    pub struct Bgp<'a, Mode> {
        pub missing_policy: ::data_views::Field<bgp::MissingPolicy<'a, Mode>>,
    }

    pub mod bgp {

        #[::data_views::data_view]
        pub struct MissingPolicy<'a, Mode> {
            pub direction_in_action: ::data_views::Field<&'a str>,
            pub direction_out_action: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::data_views::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::data_views::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
        }
    }
}

#[::data_views::data_view]
pub struct AddressFamilyFlowSpecIpv6<'a, Mode> {
    pub bgp: ::data_views::Field<address_family_flow_spec_ipv6::Bgp<'a, Mode>>,
    pub neighbors: ::data_views::Field<address_family_flow_spec_ipv6::Neighbors<'a, Mode>>,
    pub peer_groups: ::data_views::Field<address_family_flow_spec_ipv6::PeerGroups<'a, Mode>>,
}

pub mod address_family_flow_spec_ipv6 {

    #[::data_views::data_view]
    pub struct Bgp<'a, Mode> {
        pub missing_policy: ::data_views::Field<bgp::MissingPolicy<'a, Mode>>,
    }

    pub mod bgp {

        #[::data_views::data_view]
        pub struct MissingPolicy<'a, Mode> {
            pub direction_in_action: ::data_views::Field<&'a str>,
            pub direction_out_action: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::data_views::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::data_views::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
        }
    }
}

#[::data_views::data_view]
pub struct AddressFamilyPathSelection<'a, Mode> {
    pub bgp: ::data_views::Field<address_family_path_selection::Bgp<'a, Mode>>,
    pub neighbors: ::data_views::Field<address_family_path_selection::Neighbors<'a, Mode>>,
    pub peer_groups: ::data_views::Field<address_family_path_selection::PeerGroups<'a, Mode>>,
}

pub mod address_family_path_selection {

    #[::data_views::data_view]
    pub struct Bgp<'a, Mode> {
        pub additional_paths: ::data_views::Field<bgp::AdditionalPaths<'a, Mode>>,
    }

    pub mod bgp {

        #[::data_views::data_view]
        pub struct AdditionalPaths<'a, Mode> {
            pub receive: ::data_views::Field<bool>,
            pub send: ::data_views::Field<&'a str>,
            pub send_limit: ::data_views::Field<i64>,
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::data_views::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub additional_paths: ::data_views::Field<item::AdditionalPaths<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub receive: ::data_views::Field<bool>,
                pub send: ::data_views::Field<&'a str>,
                pub send_limit: ::data_views::Field<i64>,
            }
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::data_views::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub additional_paths: ::data_views::Field<item::AdditionalPaths<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub receive: ::data_views::Field<bool>,
                pub send: ::data_views::Field<&'a str>,
                pub send_limit: ::data_views::Field<i64>,
            }
        }
    }
}

#[::data_views::data_view]
pub struct AddressFamilyVpnIpv4<'a, Mode> {
    pub domain_identifier: ::data_views::Field<&'a str>,
    pub peer_groups: ::data_views::Field<address_family_vpn_ipv4::PeerGroups<'a, Mode>>,
    pub route: ::data_views::Field<address_family_vpn_ipv4::Route<'a, Mode>>,
    pub neighbors: ::data_views::Field<address_family_vpn_ipv4::Neighbors<'a, Mode>>,
    pub neighbor_default_encapsulation_mpls_next_hop_self: ::data_views::Field<address_family_vpn_ipv4::NeighborDefaultEncapsulationMplsNextHopSelf<'a, Mode>>,
    pub next_hop: ::data_views::Field<address_family_vpn_ipv4::NextHop<'a, Mode>>,
}

pub mod address_family_vpn_ipv4 {

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::data_views::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub route_map_in: ::data_views::Field<&'a str>,
            pub route_map_out: ::data_views::Field<&'a str>,
            pub peer_tag_in: ::data_views::Field<&'a str>,
            pub peer_tag_out_discard: ::data_views::Field<&'a str>,
            pub rcf_in: ::data_views::Field<&'a str>,
            pub rcf_out: ::data_views::Field<&'a str>,
            pub default_route: ::data_views::Field<item::DefaultRoute<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct DefaultRoute<'a, Mode> {
                pub enabled: ::data_views::Field<bool>,
                pub rcf: ::data_views::Field<&'a str>,
                pub route_map: ::data_views::Field<&'a str>,
            }
        }
    }

    #[::data_views::data_view]
    pub struct Route<'a, Mode> {
        pub import_match_failure_action: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::data_views::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub route_map_in: ::data_views::Field<&'a str>,
            pub route_map_out: ::data_views::Field<&'a str>,
            pub peer_tag_in: ::data_views::Field<&'a str>,
            pub peer_tag_out_discard: ::data_views::Field<&'a str>,
            pub rcf_in: ::data_views::Field<&'a str>,
            pub rcf_out: ::data_views::Field<&'a str>,
            pub default_route: ::data_views::Field<item::DefaultRoute<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct DefaultRoute<'a, Mode> {
                pub enabled: ::data_views::Field<bool>,
                pub rcf: ::data_views::Field<&'a str>,
                pub route_map: ::data_views::Field<&'a str>,
            }
        }
    }

    #[::data_views::data_view]
    pub struct NeighborDefaultEncapsulationMplsNextHopSelf<'a, Mode> {
        pub source_interface: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view]
    pub struct NextHop<'a, Mode> {
        pub resolution_disabled: ::data_views::Field<bool>,
    }
}

#[::data_views::data_view]
pub struct AddressFamilyVpnIpv6<'a, Mode> {
    pub domain_identifier: ::data_views::Field<&'a str>,
    pub peer_groups: ::data_views::Field<address_family_vpn_ipv6::PeerGroups<'a, Mode>>,
    pub route: ::data_views::Field<address_family_vpn_ipv6::Route<'a, Mode>>,
    pub neighbors: ::data_views::Field<address_family_vpn_ipv6::Neighbors<'a, Mode>>,
    pub neighbor_default_encapsulation_mpls_next_hop_self: ::data_views::Field<address_family_vpn_ipv6::NeighborDefaultEncapsulationMplsNextHopSelf<'a, Mode>>,
    pub next_hop: ::data_views::Field<address_family_vpn_ipv6::NextHop<'a, Mode>>,
}

pub mod address_family_vpn_ipv6 {

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::data_views::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub route_map_in: ::data_views::Field<&'a str>,
            pub route_map_out: ::data_views::Field<&'a str>,
            pub peer_tag_in: ::data_views::Field<&'a str>,
            pub peer_tag_out_discard: ::data_views::Field<&'a str>,
            pub rcf_in: ::data_views::Field<&'a str>,
            pub rcf_out: ::data_views::Field<&'a str>,
            pub default_route: ::data_views::Field<item::DefaultRoute<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct DefaultRoute<'a, Mode> {
                pub enabled: ::data_views::Field<bool>,
                pub rcf: ::data_views::Field<&'a str>,
                pub route_map: ::data_views::Field<&'a str>,
            }
        }
    }

    #[::data_views::data_view]
    pub struct Route<'a, Mode> {
        pub import_match_failure_action: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::data_views::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::data_views::Field<&'a str>,
            pub activate: ::data_views::Field<bool>,
            pub route_map_in: ::data_views::Field<&'a str>,
            pub route_map_out: ::data_views::Field<&'a str>,
            pub peer_tag_in: ::data_views::Field<&'a str>,
            pub peer_tag_out_discard: ::data_views::Field<&'a str>,
            pub rcf_in: ::data_views::Field<&'a str>,
            pub rcf_out: ::data_views::Field<&'a str>,
            pub default_route: ::data_views::Field<item::DefaultRoute<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct DefaultRoute<'a, Mode> {
                pub enabled: ::data_views::Field<bool>,
                pub rcf: ::data_views::Field<&'a str>,
                pub route_map: ::data_views::Field<&'a str>,
            }
        }
    }

    #[::data_views::data_view]
    pub struct NeighborDefaultEncapsulationMplsNextHopSelf<'a, Mode> {
        pub source_interface: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view]
    pub struct NextHop<'a, Mode> {
        pub resolution_disabled: ::data_views::Field<bool>,
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub bgp: ::data_views::Field<item::Bgp<'a, Mode>>,
        pub rd: ::data_views::Field<&'a str>,
        pub rd_evpn_domain: ::data_views::Field<item::RdEvpnDomain<'a, Mode>>,
        pub evpn_multicast: ::data_views::Field<bool>,
        pub evpn_multicast_address_family: ::data_views::Field<item::EvpnMulticastAddressFamily<'a, Mode>>,
        pub evpn_multicast_gateway_dr_election: ::data_views::Field<item::EvpnMulticastGatewayDrElection<'a, Mode>>,
        pub default_route_exports: ::data_views::Field<item::DefaultRouteExports<'a, Mode>>,
        pub route_targets: ::data_views::Field<item::RouteTargets<'a, Mode>>,
        pub router_id: ::data_views::Field<&'a str>,
        pub timers: ::data_views::Field<&'a str>,
        pub graceful_restart: ::data_views::Field<item::GracefulRestart<'a, Mode>>,
        pub no_graceful_restart: ::data_views::Field<bool>,
        pub networks: ::data_views::Field<item::Networks<'a, Mode>>,
        pub maximum_paths: ::data_views::Field<item::MaximumPaths<'a, Mode>>,
        pub updates: ::data_views::Field<item::Updates<'a, Mode>>,
        pub listen_ranges: ::data_views::Field<item::ListenRanges<'a, Mode>>,
        pub neighbors: ::data_views::Field<item::Neighbors<'a, Mode>>,
        pub neighbor_interfaces: ::data_views::Field<item::NeighborInterfaces<'a, Mode>>,
        pub redistribute: ::data_views::Field<item::Redistribute<'a, Mode>>,
        pub aggregate_addresses: ::data_views::Field<item::AggregateAddresses<'a, Mode>>,
        pub address_family_ipv4: ::data_views::Field<item::AddressFamilyIpv4<'a, Mode>>,
        pub address_family_ipv6: ::data_views::Field<item::AddressFamilyIpv6<'a, Mode>>,
        pub address_family_ipv4_multicast: ::data_views::Field<item::AddressFamilyIpv4Multicast<'a, Mode>>,
        pub address_family_ipv6_multicast: ::data_views::Field<item::AddressFamilyIpv6Multicast<'a, Mode>>,
        pub address_family_flow_spec_ipv4: ::data_views::Field<item::AddressFamilyFlowSpecIpv4<'a, Mode>>,
        pub address_family_flow_spec_ipv6: ::data_views::Field<item::AddressFamilyFlowSpecIpv6<'a, Mode>>,
        pub eos_cli: ::data_views::Field<&'a str>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct Bgp<'a, Mode> {
            pub redistribute_internal: ::data_views::Field<bool>,
            pub additional_paths: ::data_views::Field<bgp::AdditionalPaths<'a, Mode>>,
        }

        pub mod bgp {

            #[::data_views::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub install: ::data_views::Field<bool>,
                pub install_ecmp_primary: ::data_views::Field<bool>,
                pub receive: ::data_views::Field<bool>,
                pub send: ::data_views::Field<&'a str>,
                pub send_limit: ::data_views::Field<i64>,
            }
        }

        #[::data_views::data_view]
        pub struct RdEvpnDomain<'a, Mode> {
            pub domain: ::data_views::RequiredValue<&'a str, Mode>,
            pub rd: ::data_views::RequiredValue<&'a str, Mode>,
        }

        #[::data_views::data_view]
        pub struct EvpnMulticastAddressFamily<'a, Mode> {
            pub ipv4: ::data_views::Field<evpn_multicast_address_family::Ipv4<'a, Mode>>,
        }

        pub mod evpn_multicast_address_family {

            #[::data_views::data_view]
            pub struct Ipv4<'a, Mode> {
                pub transit: ::data_views::Field<bool>,
            }
        }

        #[::data_views::data_view]
        pub struct EvpnMulticastGatewayDrElection<'a, Mode> {
            pub algorithm: ::data_views::RequiredValue<&'a str, Mode>,
            pub preference_value: ::data_views::Field<i64>,
        }

        #[::data_views::data_view(indexed_list, primary_key(address_family))]
        pub struct DefaultRouteExports<'a, Mode> (::data_views::Field<default_route_exports::Item<'a, Mode>>);

        pub mod default_route_exports {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub address_family: ::data_views::Field<&'a str>,
                pub always: ::data_views::Field<bool>,
                pub route_map: ::data_views::Field<&'a str>,
                pub rcf: ::data_views::Field<&'a str>,
            }
        }

        #[::data_views::data_view]
        pub struct RouteTargets<'a, Mode> {
            #[data_view(rename = "import")]
            pub field_import: ::data_views::Field<route_targets::Import<'a, Mode>>,
            pub export: ::data_views::Field<route_targets::Export<'a, Mode>>,
            pub import_evpn_domains: ::data_views::Field<route_targets::ImportEvpnDomains<'a, Mode>>,
            pub export_evpn_domains: ::data_views::Field<route_targets::ExportEvpnDomains<'a, Mode>>,
        }

        pub mod route_targets {

            #[::data_views::data_view(indexed_list, primary_key(address_family))]
            pub struct Import<'a, Mode> (::data_views::Field<import::Item<'a, Mode>>);

            pub mod import {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub address_family: ::data_views::Field<&'a str>,
                    pub route_targets: ::data_views::Field<item::RouteTargets<'a, Mode>>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub rcf: ::data_views::Field<&'a str>,
                    pub vpn_route_filter_rcf: ::data_views::Field<&'a str>,
                }

                pub mod item {

                    #[::data_views::data_view(list)]
                    pub struct RouteTargets<'a, Mode> (::data_views::Field<&'a str>);
                }
            }

            #[::data_views::data_view(indexed_list, primary_key(address_family))]
            pub struct Export<'a, Mode> (::data_views::Field<export::Item<'a, Mode>>);

            pub mod export {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub address_family: ::data_views::Field<&'a str>,
                    pub route_targets: ::data_views::Field<item::RouteTargets<'a, Mode>>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub rcf: ::data_views::Field<&'a str>,
                    pub vrf_route_filter_rcf: ::data_views::Field<&'a str>,
                }

                pub mod item {

                    #[::data_views::data_view(list)]
                    pub struct RouteTargets<'a, Mode> (::data_views::Field<&'a str>);
                }
            }

            #[::data_views::data_view(indexed_list, primary_key(route_target))]
            pub struct ImportEvpnDomains<'a, Mode> (::data_views::Field<import_evpn_domains::Item<'a, Mode>>);

            pub mod import_evpn_domains {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub route_target: ::data_views::Field<&'a str>,
                    pub domain: ::data_views::RequiredValue<&'a str, Mode>,
                }
            }

            #[::data_views::data_view(indexed_list, primary_key(route_target))]
            pub struct ExportEvpnDomains<'a, Mode> (::data_views::Field<export_evpn_domains::Item<'a, Mode>>);

            pub mod export_evpn_domains {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub route_target: ::data_views::Field<&'a str>,
                    pub domain: ::data_views::RequiredValue<&'a str, Mode>,
                }
            }
        }

        #[::data_views::data_view]
        pub struct GracefulRestart<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub restart_time: ::data_views::Field<i64>,
            pub stalepath_time: ::data_views::Field<i64>,
        }

        #[::data_views::data_view(indexed_list, primary_key(prefix))]
        pub struct Networks<'a, Mode> (::data_views::Field<networks::Item<'a, Mode>>);

        pub mod networks {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub prefix: ::data_views::Field<&'a str>,
                pub route_map: ::data_views::Field<&'a str>,
            }
        }

        #[::data_views::data_view]
        pub struct MaximumPaths<'a, Mode> {
            pub paths: ::data_views::RequiredValue<i64, Mode>,
            pub ecmp: ::data_views::Field<i64>,
        }

        #[::data_views::data_view]
        pub struct Updates<'a, Mode> {
            pub wait_for_convergence: ::data_views::Field<bool>,
            pub wait_install: ::data_views::Field<bool>,
        }

        #[::data_views::data_view(list)]
        pub struct ListenRanges<'a, Mode> (::data_views::Field<listen_ranges::Item<'a, Mode>>);

        pub mod listen_ranges {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub prefix: ::data_views::Field<&'a str>,
                pub peer_id_include_router_id: ::data_views::Field<bool>,
                pub peer_group: ::data_views::Field<&'a str>,
                pub peer_filter: ::data_views::Field<&'a str>,
                pub remote_as: ::data_views::Field<&'a str>,
            }
        }

        #[::data_views::data_view(indexed_list, primary_key(ip_address))]
        pub struct Neighbors<'a, Mode> (::data_views::Field<neighbors::Item<'a, Mode>>);

        pub mod neighbors {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub ip_address: ::data_views::Field<&'a str>,
                pub peer_group: ::data_views::Field<&'a str>,
                pub remote_as: ::data_views::Field<&'a str>,
                pub password: ::data_views::Field<&'a str>,
                pub password_type: ::data_views::Field<&'a str>,
                pub passive: ::data_views::Field<bool>,
                pub remove_private_as: ::data_views::Field<item::RemovePrivateAs<'a, Mode>>,
                pub remove_private_as_ingress: ::data_views::Field<item::RemovePrivateAsIngress<'a, Mode>>,
                pub weight: ::data_views::Field<i64>,
                pub local_as: ::data_views::Field<&'a str>,
                pub as_path: ::data_views::Field<item::AsPath<'a, Mode>>,
                pub description: ::data_views::Field<&'a str>,
                pub route_reflector_client: ::data_views::Field<bool>,
                pub ebgp_multihop: ::data_views::Field<i64>,
                pub next_hop_peer: ::data_views::Field<bool>,
                pub next_hop_self: ::data_views::Field<bool>,
                pub shutdown: ::data_views::Field<bool>,
                pub bfd: ::data_views::Field<bool>,
                pub bfd_timers: ::data_views::Field<item::BfdTimers<'a, Mode>>,
                pub timers: ::data_views::Field<&'a str>,
                pub rib_in_pre_policy_retain: ::data_views::Field<item::RibInPrePolicyRetain<'a, Mode>>,
                pub send_community: ::data_views::Field<&'a str>,
                pub maximum_routes: ::data_views::Field<i64>,
                pub maximum_routes_warning_limit: ::data_views::Field<&'a str>,
                pub maximum_routes_warning_only: ::data_views::Field<bool>,
                pub maximum_accepted_routes: ::data_views::Field<item::MaximumAcceptedRoutes<'a, Mode>>,
                pub allowas_in: ::data_views::Field<item::AllowasIn<'a, Mode>>,
                pub default_originate: ::data_views::Field<item::DefaultOriginate<'a, Mode>>,
                pub enforce_first_as: ::data_views::Field<bool>,
                pub update_source: ::data_views::Field<&'a str>,
                pub route_map_in: ::data_views::Field<&'a str>,
                pub route_map_out: ::data_views::Field<&'a str>,
                pub peer_tag_in: ::data_views::Field<&'a str>,
                pub peer_tag_out_discard: ::data_views::Field<&'a str>,
                pub additional_paths: ::data_views::Field<item::AdditionalPaths<'a, Mode>>,
                pub metadata: ::data_views::Field<item::Metadata<'a, Mode>>,
            }

            pub mod item {

                #[::data_views::data_view]
                pub struct RemovePrivateAs<'a, Mode> {
                    pub enabled: ::data_views::Field<bool>,
                    pub all: ::data_views::Field<bool>,
                    pub replace_as: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct RemovePrivateAsIngress<'a, Mode> {
                    pub enabled: ::data_views::Field<bool>,
                    pub replace_as: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct AsPath<'a, Mode> {
                    pub remote_as_replace_out: ::data_views::Field<bool>,
                    pub prepend_own_disabled: ::data_views::Field<bool>,
                }

                #[::data_views::data_view(python(name = "EOSConfig.RouterBgp.VrfsItemBase.NeighborsItemBase.BfdTimers", schema = Dict(916)))]
                pub struct BfdTimers<'a, Mode> {
                    #[data_view(schema = Int(378))]
                    pub interval: ::data_views::RequiredValue<i64, Mode>,
                    #[data_view(schema = Int(379))]
                    pub min_rx: ::data_views::RequiredValue<i64, Mode>,
                    #[data_view(schema = Int(380))]
                    pub multiplier: ::data_views::RequiredValue<i64, Mode>,
                }

                #[::data_views::data_view]
                pub struct RibInPrePolicyRetain<'a, Mode> {
                    pub enabled: ::data_views::Field<bool>,
                    pub all: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct MaximumAcceptedRoutes<'a, Mode> {
                    pub limit: ::data_views::RequiredValue<i64, Mode>,
                    pub warning_limit: ::data_views::Field<maximum_accepted_routes::WarningLimit<'a, Mode>>,
                }

                pub mod maximum_accepted_routes {

                    #[::data_views::data_view]
                    pub struct WarningLimit<'a, Mode> {
                        pub count: ::data_views::Field<i64>,
                        pub percent: ::data_views::Field<i64>,
                    }
                }

                #[::data_views::data_view]
                pub struct AllowasIn<'a, Mode> {
                    pub enabled: ::data_views::Field<bool>,
                    pub times: ::data_views::Field<i64>,
                }

                #[::data_views::data_view]
                pub struct DefaultOriginate<'a, Mode> {
                    pub enabled: ::data_views::Field<bool>,
                    pub always: ::data_views::Field<bool>,
                    pub route_map: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view]
                pub struct AdditionalPaths<'a, Mode> {
                    pub receive: ::data_views::Field<bool>,
                    pub send: ::data_views::Field<&'a str>,
                    pub send_limit: ::data_views::Field<i64>,
                }

                #[::data_views::data_view]
                pub struct Metadata<'a, Mode> {
                    pub validate_state: ::data_views::Field<bool>,
                }
            }
        }

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct NeighborInterfaces<'a, Mode> (::data_views::Field<neighbor_interfaces::Item<'a, Mode>>);

        pub mod neighbor_interfaces {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
                pub remote_as: ::data_views::Field<&'a str>,
                pub peer_group: ::data_views::Field<&'a str>,
                pub peer_filter: ::data_views::Field<&'a str>,
                pub description: ::data_views::Field<&'a str>,
                pub metadata: ::data_views::Field<item::Metadata<'a, Mode>>,
            }

            pub mod item {

                #[::data_views::data_view]
                pub struct Metadata<'a, Mode> {
                    pub validate_state: ::data_views::Field<bool>,
                }
            }
        }

        #[::data_views::data_view]
        pub struct Redistribute<'a, Mode> {
            pub attached_host: ::data_views::Field<redistribute::AttachedHost<'a, Mode>>,
            pub bgp: ::data_views::Field<redistribute::Bgp<'a, Mode>>,
            pub connected: ::data_views::Field<redistribute::Connected<'a, Mode>>,
            pub dynamic: ::data_views::Field<redistribute::Dynamic<'a, Mode>>,
            pub isis: ::data_views::Field<redistribute::Isis<'a, Mode>>,
            pub ospf: ::data_views::Field<redistribute::Ospf<'a, Mode>>,
            pub ospfv3: ::data_views::Field<redistribute::Ospfv3<'a, Mode>>,
            pub rip: ::data_views::Field<redistribute::Rip<'a, Mode>>,
            #[data_view(rename = "static")]
            pub field_static: ::data_views::Field<redistribute::FieldStatic<'a, Mode>>,
            pub user: ::data_views::Field<redistribute::User<'a, Mode>>,
        }

        pub mod redistribute {

            #[::data_views::data_view]
            pub struct AttachedHost<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct Bgp<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct Connected<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
                pub rcf: ::data_views::Field<&'a str>,
                pub include_leaked: ::data_views::Field<bool>,
            }

            #[::data_views::data_view]
            pub struct Dynamic<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
                pub rcf: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct Isis<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub isis_level: ::data_views::Field<&'a str>,
                pub route_map: ::data_views::Field<&'a str>,
                pub rcf: ::data_views::Field<&'a str>,
                pub include_leaked: ::data_views::Field<bool>,
            }

            #[::data_views::data_view]
            pub struct Ospf<'a, Mode> {
                pub enabled: ::data_views::Field<bool>,
                pub match_external: ::data_views::Field<ospf::MatchExternal<'a, Mode>>,
                pub match_internal: ::data_views::Field<ospf::MatchInternal<'a, Mode>>,
                pub match_nssa_external: ::data_views::Field<ospf::MatchNssaExternal<'a, Mode>>,
                pub route_map: ::data_views::Field<&'a str>,
                pub include_leaked: ::data_views::Field<bool>,
            }

            pub mod ospf {

                #[::data_views::data_view]
                pub struct MatchExternal<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct MatchInternal<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct MatchNssaExternal<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub nssa_type: ::data_views::Field<i64>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }
            }

            #[::data_views::data_view]
            pub struct Ospfv3<'a, Mode> {
                pub enabled: ::data_views::Field<bool>,
                pub match_external: ::data_views::Field<ospfv3::MatchExternal<'a, Mode>>,
                pub match_internal: ::data_views::Field<ospfv3::MatchInternal<'a, Mode>>,
                pub match_nssa_external: ::data_views::Field<ospfv3::MatchNssaExternal<'a, Mode>>,
                pub route_map: ::data_views::Field<&'a str>,
                pub include_leaked: ::data_views::Field<bool>,
            }

            pub mod ospfv3 {

                #[::data_views::data_view]
                pub struct MatchExternal<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct MatchInternal<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct MatchNssaExternal<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub nssa_type: ::data_views::Field<i64>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }
            }

            #[::data_views::data_view]
            pub struct Rip<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct FieldStatic<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
                pub rcf: ::data_views::Field<&'a str>,
                pub include_leaked: ::data_views::Field<bool>,
            }

            #[::data_views::data_view]
            pub struct User<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub rcf: ::data_views::Field<&'a str>,
            }
        }

        #[::data_views::data_view(indexed_list, primary_key(prefix))]
        pub struct AggregateAddresses<'a, Mode> (::data_views::Field<aggregate_addresses::Item<'a, Mode>>);

        pub mod aggregate_addresses {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub prefix: ::data_views::Field<&'a str>,
                pub advertise_only: ::data_views::Field<bool>,
                pub as_set: ::data_views::Field<bool>,
                pub summary_only: ::data_views::Field<bool>,
                pub attribute_map: ::data_views::Field<&'a str>,
                pub match_map: ::data_views::Field<&'a str>,
                pub attribute: ::data_views::Field<item::Attribute<'a, Mode>>,
            }

            pub mod item {

                #[::data_views::data_view]
                pub struct Attribute<'a, Mode> {
                    pub rcf: ::data_views::Field<&'a str>,
                }
            }
        }

        #[::data_views::data_view]
        pub struct AddressFamilyIpv4<'a, Mode> {
            pub bgp: ::data_views::Field<address_family_ipv4::Bgp<'a, Mode>>,
            pub neighbors: ::data_views::Field<address_family_ipv4::Neighbors<'a, Mode>>,
            pub networks: ::data_views::Field<address_family_ipv4::Networks<'a, Mode>>,
            pub redistribute: ::data_views::Field<address_family_ipv4::Redistribute<'a, Mode>>,
        }

        pub mod address_family_ipv4 {

            #[::data_views::data_view]
            pub struct Bgp<'a, Mode> {
                pub missing_policy: ::data_views::Field<bgp::MissingPolicy<'a, Mode>>,
                pub additional_paths: ::data_views::Field<bgp::AdditionalPaths<'a, Mode>>,
                pub redistribute_internal: ::data_views::Field<bool>,
            }

            pub mod bgp {

                #[::data_views::data_view]
                pub struct MissingPolicy<'a, Mode> {
                    pub direction_in_action: ::data_views::Field<&'a str>,
                    pub direction_out_action: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view]
                pub struct AdditionalPaths<'a, Mode> {
                    pub install: ::data_views::Field<bool>,
                    pub install_ecmp_primary: ::data_views::Field<bool>,
                    pub receive: ::data_views::Field<bool>,
                    pub send: ::data_views::Field<&'a str>,
                    pub send_limit: ::data_views::Field<i64>,
                }
            }

            #[::data_views::data_view(indexed_list, primary_key(ip_address))]
            pub struct Neighbors<'a, Mode> (::data_views::Field<neighbors::Item<'a, Mode>>);

            pub mod neighbors {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub ip_address: ::data_views::Field<&'a str>,
                    pub activate: ::data_views::Field<bool>,
                    pub route_map_in: ::data_views::Field<&'a str>,
                    pub route_map_out: ::data_views::Field<&'a str>,
                    pub peer_tag_in: ::data_views::Field<&'a str>,
                    pub peer_tag_out_discard: ::data_views::Field<&'a str>,
                    pub rcf_in: ::data_views::Field<&'a str>,
                    pub rcf_out: ::data_views::Field<&'a str>,
                    pub prefix_list_in: ::data_views::Field<&'a str>,
                    pub prefix_list_out: ::data_views::Field<&'a str>,
                    pub next_hop: ::data_views::Field<item::NextHop<'a, Mode>>,
                    pub additional_paths: ::data_views::Field<item::AdditionalPaths<'a, Mode>>,
                }

                pub mod item {

                    #[::data_views::data_view]
                    pub struct NextHop<'a, Mode> {
                        pub address_family_ipv6: ::data_views::Field<next_hop::AddressFamilyIpv6<'a, Mode>>,
                    }

                    pub mod next_hop {

                        #[::data_views::data_view]
                        pub struct AddressFamilyIpv6<'a, Mode> {
                            pub enabled: ::data_views::RequiredValue<bool, Mode>,
                            pub originate: ::data_views::Field<bool>,
                        }
                    }

                    #[::data_views::data_view]
                    pub struct AdditionalPaths<'a, Mode> {
                        pub receive: ::data_views::Field<bool>,
                        pub send: ::data_views::Field<&'a str>,
                        pub send_limit: ::data_views::Field<i64>,
                    }
                }
            }

            #[::data_views::data_view(indexed_list, primary_key(prefix))]
            pub struct Networks<'a, Mode> (::data_views::Field<networks::Item<'a, Mode>>);

            pub mod networks {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub prefix: ::data_views::Field<&'a str>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub rcf: ::data_views::Field<&'a str>,
                }
            }

            #[::data_views::data_view]
            pub struct Redistribute<'a, Mode> {
                pub attached_host: ::data_views::Field<redistribute::AttachedHost<'a, Mode>>,
                pub bgp: ::data_views::Field<redistribute::Bgp<'a, Mode>>,
                pub connected: ::data_views::Field<redistribute::Connected<'a, Mode>>,
                pub dynamic: ::data_views::Field<redistribute::Dynamic<'a, Mode>>,
                pub isis: ::data_views::Field<redistribute::Isis<'a, Mode>>,
                pub ospf: ::data_views::Field<redistribute::Ospf<'a, Mode>>,
                pub ospfv3: ::data_views::Field<redistribute::Ospfv3<'a, Mode>>,
                pub rip: ::data_views::Field<redistribute::Rip<'a, Mode>>,
                #[data_view(rename = "static")]
                pub field_static: ::data_views::Field<redistribute::FieldStatic<'a, Mode>>,
                pub user: ::data_views::Field<redistribute::User<'a, Mode>>,
            }

            pub mod redistribute {

                #[::data_views::data_view]
                pub struct AttachedHost<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view]
                pub struct Bgp<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view]
                pub struct Connected<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub rcf: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct Dynamic<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub rcf: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view]
                pub struct Isis<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub isis_level: ::data_views::Field<&'a str>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub rcf: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct Ospf<'a, Mode> {
                    pub enabled: ::data_views::Field<bool>,
                    pub match_external: ::data_views::Field<ospf::MatchExternal<'a, Mode>>,
                    pub match_internal: ::data_views::Field<ospf::MatchInternal<'a, Mode>>,
                    pub match_nssa_external: ::data_views::Field<ospf::MatchNssaExternal<'a, Mode>>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                pub mod ospf {

                    #[::data_views::data_view]
                    pub struct MatchExternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub route_map: ::data_views::Field<&'a str>,
                        pub include_leaked: ::data_views::Field<bool>,
                    }

                    #[::data_views::data_view]
                    pub struct MatchInternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub route_map: ::data_views::Field<&'a str>,
                        pub include_leaked: ::data_views::Field<bool>,
                    }

                    #[::data_views::data_view]
                    pub struct MatchNssaExternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub nssa_type: ::data_views::Field<i64>,
                        pub route_map: ::data_views::Field<&'a str>,
                        pub include_leaked: ::data_views::Field<bool>,
                    }
                }

                #[::data_views::data_view]
                pub struct Ospfv3<'a, Mode> {
                    pub enabled: ::data_views::Field<bool>,
                    pub match_external: ::data_views::Field<ospfv3::MatchExternal<'a, Mode>>,
                    pub match_internal: ::data_views::Field<ospfv3::MatchInternal<'a, Mode>>,
                    pub match_nssa_external: ::data_views::Field<ospfv3::MatchNssaExternal<'a, Mode>>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                pub mod ospfv3 {

                    #[::data_views::data_view]
                    pub struct MatchExternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub route_map: ::data_views::Field<&'a str>,
                        pub include_leaked: ::data_views::Field<bool>,
                    }

                    #[::data_views::data_view]
                    pub struct MatchInternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub route_map: ::data_views::Field<&'a str>,
                        pub include_leaked: ::data_views::Field<bool>,
                    }

                    #[::data_views::data_view]
                    pub struct MatchNssaExternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub nssa_type: ::data_views::Field<i64>,
                        pub route_map: ::data_views::Field<&'a str>,
                        pub include_leaked: ::data_views::Field<bool>,
                    }
                }

                #[::data_views::data_view]
                pub struct Rip<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view]
                pub struct FieldStatic<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub rcf: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct User<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub rcf: ::data_views::Field<&'a str>,
                }
            }
        }

        #[::data_views::data_view]
        pub struct AddressFamilyIpv6<'a, Mode> {
            pub bgp: ::data_views::Field<address_family_ipv6::Bgp<'a, Mode>>,
            pub neighbors: ::data_views::Field<address_family_ipv6::Neighbors<'a, Mode>>,
            pub networks: ::data_views::Field<address_family_ipv6::Networks<'a, Mode>>,
            pub redistribute: ::data_views::Field<address_family_ipv6::Redistribute<'a, Mode>>,
        }

        pub mod address_family_ipv6 {

            #[::data_views::data_view]
            pub struct Bgp<'a, Mode> {
                pub missing_policy: ::data_views::Field<bgp::MissingPolicy<'a, Mode>>,
                pub additional_paths: ::data_views::Field<bgp::AdditionalPaths<'a, Mode>>,
                pub redistribute_internal: ::data_views::Field<bool>,
            }

            pub mod bgp {

                #[::data_views::data_view]
                pub struct MissingPolicy<'a, Mode> {
                    pub direction_in_action: ::data_views::Field<&'a str>,
                    pub direction_out_action: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view]
                pub struct AdditionalPaths<'a, Mode> {
                    pub install: ::data_views::Field<bool>,
                    pub install_ecmp_primary: ::data_views::Field<bool>,
                    pub receive: ::data_views::Field<bool>,
                    pub send: ::data_views::Field<&'a str>,
                    pub send_limit: ::data_views::Field<i64>,
                }
            }

            #[::data_views::data_view(indexed_list, primary_key(ip_address))]
            pub struct Neighbors<'a, Mode> (::data_views::Field<neighbors::Item<'a, Mode>>);

            pub mod neighbors {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub ip_address: ::data_views::Field<&'a str>,
                    pub activate: ::data_views::Field<bool>,
                    pub route_map_in: ::data_views::Field<&'a str>,
                    pub route_map_out: ::data_views::Field<&'a str>,
                    pub peer_tag_in: ::data_views::Field<&'a str>,
                    pub peer_tag_out_discard: ::data_views::Field<&'a str>,
                    pub rcf_in: ::data_views::Field<&'a str>,
                    pub rcf_out: ::data_views::Field<&'a str>,
                    pub prefix_list_in: ::data_views::Field<&'a str>,
                    pub prefix_list_out: ::data_views::Field<&'a str>,
                    pub additional_paths: ::data_views::Field<item::AdditionalPaths<'a, Mode>>,
                }

                pub mod item {

                    #[::data_views::data_view]
                    pub struct AdditionalPaths<'a, Mode> {
                        pub receive: ::data_views::Field<bool>,
                        pub send: ::data_views::Field<&'a str>,
                        pub send_limit: ::data_views::Field<i64>,
                    }
                }
            }

            #[::data_views::data_view(indexed_list, primary_key(prefix))]
            pub struct Networks<'a, Mode> (::data_views::Field<networks::Item<'a, Mode>>);

            pub mod networks {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub prefix: ::data_views::Field<&'a str>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub rcf: ::data_views::Field<&'a str>,
                }
            }

            #[::data_views::data_view]
            pub struct Redistribute<'a, Mode> {
                pub attached_host: ::data_views::Field<redistribute::AttachedHost<'a, Mode>>,
                pub bgp: ::data_views::Field<redistribute::Bgp<'a, Mode>>,
                pub connected: ::data_views::Field<redistribute::Connected<'a, Mode>>,
                pub dhcp: ::data_views::Field<redistribute::Dhcp<'a, Mode>>,
                pub dynamic: ::data_views::Field<redistribute::Dynamic<'a, Mode>>,
                pub isis: ::data_views::Field<redistribute::Isis<'a, Mode>>,
                pub ospfv3: ::data_views::Field<redistribute::Ospfv3<'a, Mode>>,
                #[data_view(rename = "static")]
                pub field_static: ::data_views::Field<redistribute::FieldStatic<'a, Mode>>,
                pub user: ::data_views::Field<redistribute::User<'a, Mode>>,
            }

            pub mod redistribute {

                #[::data_views::data_view]
                pub struct AttachedHost<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view]
                pub struct Bgp<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view]
                pub struct Connected<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub rcf: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct Dhcp<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view]
                pub struct Dynamic<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub rcf: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view]
                pub struct Isis<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub isis_level: ::data_views::Field<&'a str>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub rcf: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct Ospfv3<'a, Mode> {
                    pub enabled: ::data_views::Field<bool>,
                    pub match_external: ::data_views::Field<ospfv3::MatchExternal<'a, Mode>>,
                    pub match_internal: ::data_views::Field<ospfv3::MatchInternal<'a, Mode>>,
                    pub match_nssa_external: ::data_views::Field<ospfv3::MatchNssaExternal<'a, Mode>>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                pub mod ospfv3 {

                    #[::data_views::data_view]
                    pub struct MatchExternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub route_map: ::data_views::Field<&'a str>,
                        pub include_leaked: ::data_views::Field<bool>,
                    }

                    #[::data_views::data_view]
                    pub struct MatchInternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub route_map: ::data_views::Field<&'a str>,
                        pub include_leaked: ::data_views::Field<bool>,
                    }

                    #[::data_views::data_view]
                    pub struct MatchNssaExternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub nssa_type: ::data_views::Field<i64>,
                        pub route_map: ::data_views::Field<&'a str>,
                        pub include_leaked: ::data_views::Field<bool>,
                    }
                }

                #[::data_views::data_view]
                pub struct FieldStatic<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub rcf: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct User<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub rcf: ::data_views::Field<&'a str>,
                }
            }
        }

        #[::data_views::data_view]
        pub struct AddressFamilyIpv4Multicast<'a, Mode> {
            pub bgp: ::data_views::Field<address_family_ipv4_multicast::Bgp<'a, Mode>>,
            pub neighbors: ::data_views::Field<address_family_ipv4_multicast::Neighbors<'a, Mode>>,
            pub networks: ::data_views::Field<address_family_ipv4_multicast::Networks<'a, Mode>>,
            pub redistribute: ::data_views::Field<address_family_ipv4_multicast::Redistribute<'a, Mode>>,
        }

        pub mod address_family_ipv4_multicast {

            #[::data_views::data_view]
            pub struct Bgp<'a, Mode> {
                pub missing_policy: ::data_views::Field<bgp::MissingPolicy<'a, Mode>>,
                pub additional_paths: ::data_views::Field<bgp::AdditionalPaths<'a, Mode>>,
            }

            pub mod bgp {

                #[::data_views::data_view]
                pub struct MissingPolicy<'a, Mode> {
                    pub direction_in_action: ::data_views::Field<&'a str>,
                    pub direction_out_action: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view]
                pub struct AdditionalPaths<'a, Mode> {
                    pub receive: ::data_views::Field<bool>,
                }
            }

            #[::data_views::data_view(indexed_list, primary_key(ip_address))]
            pub struct Neighbors<'a, Mode> (::data_views::Field<neighbors::Item<'a, Mode>>);

            pub mod neighbors {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub ip_address: ::data_views::Field<&'a str>,
                    pub activate: ::data_views::Field<bool>,
                    pub route_map_in: ::data_views::Field<&'a str>,
                    pub route_map_out: ::data_views::Field<&'a str>,
                    pub peer_tag_in: ::data_views::Field<&'a str>,
                    pub peer_tag_out_discard: ::data_views::Field<&'a str>,
                    pub additional_paths: ::data_views::Field<item::AdditionalPaths<'a, Mode>>,
                }

                pub mod item {

                    #[::data_views::data_view]
                    pub struct AdditionalPaths<'a, Mode> {
                        pub receive: ::data_views::Field<bool>,
                    }
                }
            }

            #[::data_views::data_view(indexed_list, primary_key(prefix))]
            pub struct Networks<'a, Mode> (::data_views::Field<networks::Item<'a, Mode>>);

            pub mod networks {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub prefix: ::data_views::Field<&'a str>,
                    pub route_map: ::data_views::Field<&'a str>,
                }
            }

            #[::data_views::data_view]
            pub struct Redistribute<'a, Mode> {
                pub attached_host: ::data_views::Field<redistribute::AttachedHost<'a, Mode>>,
                pub connected: ::data_views::Field<redistribute::Connected<'a, Mode>>,
                pub isis: ::data_views::Field<redistribute::Isis<'a, Mode>>,
                pub ospf: ::data_views::Field<redistribute::Ospf<'a, Mode>>,
                pub ospfv3: ::data_views::Field<redistribute::Ospfv3<'a, Mode>>,
                #[data_view(rename = "static")]
                pub field_static: ::data_views::Field<redistribute::FieldStatic<'a, Mode>>,
            }

            pub mod redistribute {

                #[::data_views::data_view]
                pub struct AttachedHost<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view]
                pub struct Connected<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view]
                pub struct Isis<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub isis_level: ::data_views::Field<&'a str>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub rcf: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct Ospf<'a, Mode> {
                    pub enabled: ::data_views::Field<bool>,
                    pub match_external: ::data_views::Field<ospf::MatchExternal<'a, Mode>>,
                    pub match_internal: ::data_views::Field<ospf::MatchInternal<'a, Mode>>,
                    pub match_nssa_external: ::data_views::Field<ospf::MatchNssaExternal<'a, Mode>>,
                    pub route_map: ::data_views::Field<&'a str>,
                }

                pub mod ospf {

                    #[::data_views::data_view]
                    pub struct MatchExternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub route_map: ::data_views::Field<&'a str>,
                    }

                    #[::data_views::data_view]
                    pub struct MatchInternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub route_map: ::data_views::Field<&'a str>,
                    }

                    #[::data_views::data_view]
                    pub struct MatchNssaExternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub nssa_type: ::data_views::Field<i64>,
                        pub route_map: ::data_views::Field<&'a str>,
                    }
                }

                #[::data_views::data_view]
                pub struct Ospfv3<'a, Mode> {
                    pub enabled: ::data_views::Field<bool>,
                    pub match_external: ::data_views::Field<ospfv3::MatchExternal<'a, Mode>>,
                    pub match_internal: ::data_views::Field<ospfv3::MatchInternal<'a, Mode>>,
                    pub match_nssa_external: ::data_views::Field<ospfv3::MatchNssaExternal<'a, Mode>>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                pub mod ospfv3 {

                    #[::data_views::data_view]
                    pub struct MatchExternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub route_map: ::data_views::Field<&'a str>,
                        pub include_leaked: ::data_views::Field<bool>,
                    }

                    #[::data_views::data_view]
                    pub struct MatchInternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub route_map: ::data_views::Field<&'a str>,
                        pub include_leaked: ::data_views::Field<bool>,
                    }

                    #[::data_views::data_view]
                    pub struct MatchNssaExternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub nssa_type: ::data_views::Field<i64>,
                        pub route_map: ::data_views::Field<&'a str>,
                        pub include_leaked: ::data_views::Field<bool>,
                    }
                }

                #[::data_views::data_view]
                pub struct FieldStatic<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                }
            }
        }

        #[::data_views::data_view]
        pub struct AddressFamilyIpv6Multicast<'a, Mode> {
            pub bgp: ::data_views::Field<address_family_ipv6_multicast::Bgp<'a, Mode>>,
            pub neighbors: ::data_views::Field<address_family_ipv6_multicast::Neighbors<'a, Mode>>,
            pub networks: ::data_views::Field<address_family_ipv6_multicast::Networks<'a, Mode>>,
            pub redistribute: ::data_views::Field<address_family_ipv6_multicast::Redistribute<'a, Mode>>,
        }

        pub mod address_family_ipv6_multicast {

            #[::data_views::data_view]
            pub struct Bgp<'a, Mode> {
                pub missing_policy: ::data_views::Field<bgp::MissingPolicy<'a, Mode>>,
                pub additional_paths: ::data_views::Field<bgp::AdditionalPaths<'a, Mode>>,
            }

            pub mod bgp {

                #[::data_views::data_view]
                pub struct MissingPolicy<'a, Mode> {
                    pub direction_in_action: ::data_views::Field<&'a str>,
                    pub direction_out_action: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view]
                pub struct AdditionalPaths<'a, Mode> {
                    pub receive: ::data_views::Field<bool>,
                }
            }

            #[::data_views::data_view(indexed_list, primary_key(ip_address))]
            pub struct Neighbors<'a, Mode> (::data_views::Field<neighbors::Item<'a, Mode>>);

            pub mod neighbors {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub ip_address: ::data_views::Field<&'a str>,
                    pub activate: ::data_views::Field<bool>,
                    pub route_map_in: ::data_views::Field<&'a str>,
                    pub route_map_out: ::data_views::Field<&'a str>,
                    pub peer_tag_in: ::data_views::Field<&'a str>,
                    pub peer_tag_out_discard: ::data_views::Field<&'a str>,
                    pub additional_paths: ::data_views::Field<item::AdditionalPaths<'a, Mode>>,
                }

                pub mod item {

                    #[::data_views::data_view]
                    pub struct AdditionalPaths<'a, Mode> {
                        pub receive: ::data_views::Field<bool>,
                    }
                }
            }

            #[::data_views::data_view(indexed_list, primary_key(prefix))]
            pub struct Networks<'a, Mode> (::data_views::Field<networks::Item<'a, Mode>>);

            pub mod networks {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub prefix: ::data_views::Field<&'a str>,
                    pub route_map: ::data_views::Field<&'a str>,
                }
            }

            #[::data_views::data_view]
            pub struct Redistribute<'a, Mode> {
                pub connected: ::data_views::Field<redistribute::Connected<'a, Mode>>,
                pub isis: ::data_views::Field<redistribute::Isis<'a, Mode>>,
                pub ospf: ::data_views::Field<redistribute::Ospf<'a, Mode>>,
                pub ospfv3: ::data_views::Field<redistribute::Ospfv3<'a, Mode>>,
                #[data_view(rename = "static")]
                pub field_static: ::data_views::Field<redistribute::FieldStatic<'a, Mode>>,
            }

            pub mod redistribute {

                #[::data_views::data_view]
                pub struct Connected<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view]
                pub struct Isis<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub isis_level: ::data_views::Field<&'a str>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub rcf: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct Ospf<'a, Mode> {
                    pub enabled: ::data_views::Field<bool>,
                    pub match_external: ::data_views::Field<ospf::MatchExternal<'a, Mode>>,
                    pub match_internal: ::data_views::Field<ospf::MatchInternal<'a, Mode>>,
                    pub match_nssa_external: ::data_views::Field<ospf::MatchNssaExternal<'a, Mode>>,
                    pub route_map: ::data_views::Field<&'a str>,
                }

                pub mod ospf {

                    #[::data_views::data_view]
                    pub struct MatchExternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub route_map: ::data_views::Field<&'a str>,
                    }

                    #[::data_views::data_view]
                    pub struct MatchInternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub route_map: ::data_views::Field<&'a str>,
                    }

                    #[::data_views::data_view]
                    pub struct MatchNssaExternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub nssa_type: ::data_views::Field<i64>,
                        pub route_map: ::data_views::Field<&'a str>,
                    }
                }

                #[::data_views::data_view]
                pub struct Ospfv3<'a, Mode> {
                    pub enabled: ::data_views::Field<bool>,
                    pub match_external: ::data_views::Field<ospfv3::MatchExternal<'a, Mode>>,
                    pub match_internal: ::data_views::Field<ospfv3::MatchInternal<'a, Mode>>,
                    pub match_nssa_external: ::data_views::Field<ospfv3::MatchNssaExternal<'a, Mode>>,
                    pub route_map: ::data_views::Field<&'a str>,
                }

                pub mod ospfv3 {

                    #[::data_views::data_view]
                    pub struct MatchExternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub route_map: ::data_views::Field<&'a str>,
                    }

                    #[::data_views::data_view]
                    pub struct MatchInternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub route_map: ::data_views::Field<&'a str>,
                    }

                    #[::data_views::data_view]
                    pub struct MatchNssaExternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub nssa_type: ::data_views::Field<i64>,
                        pub route_map: ::data_views::Field<&'a str>,
                    }
                }

                #[::data_views::data_view]
                pub struct FieldStatic<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                }
            }
        }

        #[::data_views::data_view]
        pub struct AddressFamilyFlowSpecIpv4<'a, Mode> {
            pub bgp: ::data_views::Field<address_family_flow_spec_ipv4::Bgp<'a, Mode>>,
            pub neighbors: ::data_views::Field<address_family_flow_spec_ipv4::Neighbors<'a, Mode>>,
        }

        pub mod address_family_flow_spec_ipv4 {

            #[::data_views::data_view]
            pub struct Bgp<'a, Mode> {
                pub missing_policy: ::data_views::Field<bgp::MissingPolicy<'a, Mode>>,
            }

            pub mod bgp {

                #[::data_views::data_view]
                pub struct MissingPolicy<'a, Mode> {
                    pub direction_in_action: ::data_views::Field<&'a str>,
                    pub direction_out_action: ::data_views::Field<&'a str>,
                }
            }

            #[::data_views::data_view(indexed_list, primary_key(ip_address))]
            pub struct Neighbors<'a, Mode> (::data_views::Field<neighbors::Item<'a, Mode>>);

            pub mod neighbors {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub ip_address: ::data_views::Field<&'a str>,
                    pub activate: ::data_views::Field<bool>,
                }
            }
        }

        #[::data_views::data_view]
        pub struct AddressFamilyFlowSpecIpv6<'a, Mode> {
            pub bgp: ::data_views::Field<address_family_flow_spec_ipv6::Bgp<'a, Mode>>,
            pub neighbors: ::data_views::Field<address_family_flow_spec_ipv6::Neighbors<'a, Mode>>,
        }

        pub mod address_family_flow_spec_ipv6 {

            #[::data_views::data_view]
            pub struct Bgp<'a, Mode> {
                pub missing_policy: ::data_views::Field<bgp::MissingPolicy<'a, Mode>>,
            }

            pub mod bgp {

                #[::data_views::data_view]
                pub struct MissingPolicy<'a, Mode> {
                    pub direction_in_action: ::data_views::Field<&'a str>,
                    pub direction_out_action: ::data_views::Field<&'a str>,
                }
            }

            #[::data_views::data_view(indexed_list, primary_key(ip_address))]
            pub struct Neighbors<'a, Mode> (::data_views::Field<neighbors::Item<'a, Mode>>);

            pub mod neighbors {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub ip_address: ::data_views::Field<&'a str>,
                    pub activate: ::data_views::Field<bool>,
                }
            }
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct SessionTrackers<'a, Mode> (::data_views::Field<session_trackers::Item<'a, Mode>>);

pub mod session_trackers {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub recovery_delay: ::data_views::Field<i64>,
    }
}
