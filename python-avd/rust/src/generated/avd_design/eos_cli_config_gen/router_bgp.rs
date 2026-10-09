// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Timers<'a, Mode> {
    pub keepalive_time: ::validated_data::Field<i64>,
    pub hold_time: ::validated_data::Field<i64>,
    pub min_hold_time: ::validated_data::Field<i64>,
    pub send_failure_hold_time: ::validated_data::Field<i64>,
}

#[::validated_data::data_view]
pub struct Distance<'a, Mode> {
    pub external_routes: ::validated_data::RequiredValue<i64, Mode>,
    pub internal_routes: ::validated_data::RequiredValue<i64, Mode>,
    pub local_routes: ::validated_data::RequiredValue<i64, Mode>,
}

#[::validated_data::data_view]
pub struct GracefulRestart<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub restart_time: ::validated_data::Field<i64>,
    pub stalepath_time: ::validated_data::Field<i64>,
}

#[::validated_data::data_view]
pub struct GracefulRestartHelper<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub restart_time: ::validated_data::Field<i64>,
    pub long_lived: ::validated_data::Field<bool>,
}

#[::validated_data::data_view]
pub struct MaximumPaths<'a, Mode> {
    pub paths: ::validated_data::RequiredValue<i64, Mode>,
    pub ecmp: ::validated_data::Field<i64>,
}

#[::validated_data::data_view]
pub struct RouteDistinguisher<'a, Mode> {
    pub assignment_auto: ::validated_data::Field<route_distinguisher::AssignmentAuto<'a, Mode>>,
}

pub mod route_distinguisher {

    #[::validated_data::data_view]
    pub struct AssignmentAuto<'a, Mode> {
        pub range: ::validated_data::Field<assignment_auto::Range<'a, Mode>>,
        pub address_families: ::validated_data::Field<assignment_auto::AddressFamilies<'a, Mode>>,
    }

    pub mod assignment_auto {

        #[::validated_data::data_view]
        pub struct Range<'a, Mode> {
            pub start: ::validated_data::RequiredValue<i64, Mode>,
            pub end: ::validated_data::RequiredValue<i64, Mode>,
        }

        #[::validated_data::data_view(list)]
        pub struct AddressFamilies<'a, Mode> (::validated_data::Field<&'a str>);
    }
}

#[::validated_data::data_view]
pub struct Updates<'a, Mode> {
    pub wait_for_convergence: ::validated_data::Field<bool>,
    pub wait_install: ::validated_data::Field<bool>,
}

#[::validated_data::data_view(list)]
pub struct BgpDefaults<'a, Mode> (::validated_data::Field<&'a str>);

#[::validated_data::data_view]
pub struct Bgp<'a, Mode> {
    pub convergence: ::validated_data::Field<bgp::Convergence<'a, Mode>>,
    pub default: ::validated_data::Field<bgp::Default<'a, Mode>>,
    pub route_reflector_preserve_attributes: ::validated_data::Field<bgp::RouteReflectorPreserveAttributes<'a, Mode>>,
    pub bestpath: ::validated_data::Field<bgp::Bestpath<'a, Mode>>,
    pub additional_paths: ::validated_data::Field<bgp::AdditionalPaths<'a, Mode>>,
    pub redistribute_internal: ::validated_data::Field<bool>,
    pub labeled_unicast: ::validated_data::Field<bgp::LabeledUnicast<'a, Mode>>,
}

pub mod bgp {

    #[::validated_data::data_view]
    pub struct Convergence<'a, Mode> {
        pub slow_peer_time: ::validated_data::Field<i64>,
        pub time: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Default<'a, Mode> {
        pub ipv4_unicast: ::validated_data::Field<bool>,
        pub ipv4_unicast_transport_ipv6: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view]
    pub struct RouteReflectorPreserveAttributes<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub always: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view]
    pub struct Bestpath<'a, Mode> {
        pub d_path: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view]
    pub struct AdditionalPaths<'a, Mode> {
        pub receive: ::validated_data::Field<bool>,
        pub send: ::validated_data::Field<&'a str>,
        pub send_limit: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct LabeledUnicast<'a, Mode> {
        pub rib: ::validated_data::Field<labeled_unicast::Rib<'a, Mode>>,
    }

    pub mod labeled_unicast {

        #[::validated_data::data_view]
        pub struct Rib<'a, Mode> {
            pub ip: ::validated_data::Field<rib::Ip<'a, Mode>>,
            pub tunnel: ::validated_data::Field<rib::Tunnel<'a, Mode>>,
        }

        pub mod rib {

            #[::validated_data::data_view]
            pub struct Ip<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct Tunnel<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
            }
        }
    }
}

#[::validated_data::data_view(list)]
pub struct ListenRanges<'a, Mode> (::validated_data::Field<listen_ranges::Item<'a, Mode>>);

pub mod listen_ranges {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub prefix: ::validated_data::Field<&'a str>,
        pub peer_id_include_router_id: ::validated_data::Field<bool>,
        pub peer_group: ::validated_data::Field<&'a str>,
        pub peer_filter: ::validated_data::Field<&'a str>,
        pub remote_as: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view]
pub struct NeighborDefault<'a, Mode> {
    pub send_community: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct PeerGroups<'a, Mode> (::validated_data::Field<peer_groups::Item<'a, Mode>>);

pub mod peer_groups {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
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
        pub password: ::validated_data::Field<&'a str>,
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

        #[::validated_data::data_view]
        pub struct Metadata<'a, Mode> {
            #[data_view(rename = "type")]
            pub field_type: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct AsPath<'a, Mode> {
            pub remote_as_replace_out: ::validated_data::Field<bool>,
            pub prepend_own_disabled: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct RemovePrivateAs<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub all: ::validated_data::Field<bool>,
            pub replace_as: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct RemovePrivateAsIngress<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub replace_as: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct BfdTimers<'a, Mode> {
            pub interval: ::validated_data::RequiredValue<i64, Mode>,
            pub min_rx: ::validated_data::RequiredValue<i64, Mode>,
            pub multiplier: ::validated_data::RequiredValue<i64, Mode>,
        }

        #[::validated_data::data_view]
        pub struct DefaultOriginate<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub always: ::validated_data::Field<bool>,
            pub route_map: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct MaximumAcceptedRoutes<'a, Mode> {
            pub limit: ::validated_data::RequiredValue<i64, Mode>,
            pub warning_limit: ::validated_data::Field<maximum_accepted_routes::WarningLimit<'a, Mode>>,
        }

        pub mod maximum_accepted_routes {

            #[::validated_data::data_view]
            pub struct WarningLimit<'a, Mode> {
                pub count: ::validated_data::Field<i64>,
                pub percent: ::validated_data::Field<i64>,
            }
        }

        #[::validated_data::data_view]
        pub struct MissingPolicy<'a, Mode> {
            pub direction_in: ::validated_data::Field<missing_policy::DirectionIn<'a, Mode>>,
            pub direction_out: ::validated_data::Field<missing_policy::DirectionOut<'a, Mode>>,
        }

        pub mod missing_policy {

            #[::validated_data::data_view]
            pub struct DirectionIn<'a, Mode> {
                pub action: ::validated_data::RequiredValue<&'a str, Mode>,
                pub include_community_list: ::validated_data::Field<bool>,
                pub include_prefix_list: ::validated_data::Field<bool>,
                pub include_sub_route_map: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view]
            pub struct DirectionOut<'a, Mode> {
                pub action: ::validated_data::RequiredValue<&'a str, Mode>,
                pub include_community_list: ::validated_data::Field<bool>,
                pub include_prefix_list: ::validated_data::Field<bool>,
                pub include_sub_route_map: ::validated_data::Field<bool>,
            }
        }

        #[::validated_data::data_view]
        pub struct LinkBandwidth<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub default: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct AllowasIn<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub times: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view]
        pub struct RibInPrePolicyRetain<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub all: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct SharedSecret<'a, Mode> {
            pub profile: ::validated_data::RequiredValue<&'a str, Mode>,
            pub hash_algorithm: ::validated_data::RequiredValue<&'a str, Mode>,
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(ip_address))]
pub struct Neighbors<'a, Mode> (::validated_data::Field<neighbors::Item<'a, Mode>>);

pub mod neighbors {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub ip_address: ::validated_data::Field<&'a str>,
        pub peer_group: ::validated_data::Field<&'a str>,
        pub remote_as: ::validated_data::Field<&'a str>,
        pub local_as: ::validated_data::Field<&'a str>,
        pub as_path: ::validated_data::Field<item::AsPath<'a, Mode>>,
        pub metadata: ::validated_data::Field<item::Metadata<'a, Mode>>,
        pub description: ::validated_data::Field<&'a str>,
        pub route_reflector_client: ::validated_data::Field<bool>,
        pub password: ::validated_data::Field<&'a str>,
        pub password_type: ::validated_data::Field<&'a str>,
        pub passive: ::validated_data::Field<bool>,
        pub shutdown: ::validated_data::Field<bool>,
        pub update_source: ::validated_data::Field<&'a str>,
        pub bfd: ::validated_data::Field<bool>,
        pub bfd_timers: ::validated_data::Field<item::BfdTimers<'a, Mode>>,
        pub weight: ::validated_data::Field<i64>,
        pub timers: ::validated_data::Field<&'a str>,
        pub route_map_in: ::validated_data::Field<&'a str>,
        pub route_map_out: ::validated_data::Field<&'a str>,
        pub peer_tag_in: ::validated_data::Field<&'a str>,
        pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
        pub default_originate: ::validated_data::Field<item::DefaultOriginate<'a, Mode>>,
        pub enforce_first_as: ::validated_data::Field<bool>,
        pub send_community: ::validated_data::Field<&'a str>,
        pub maximum_routes: ::validated_data::Field<i64>,
        pub maximum_routes_warning_limit: ::validated_data::Field<&'a str>,
        pub maximum_routes_warning_only: ::validated_data::Field<bool>,
        pub maximum_accepted_routes: ::validated_data::Field<item::MaximumAcceptedRoutes<'a, Mode>>,
        pub missing_policy: ::validated_data::Field<item::MissingPolicy<'a, Mode>>,
        pub allowas_in: ::validated_data::Field<item::AllowasIn<'a, Mode>>,
        pub ebgp_multihop: ::validated_data::Field<i64>,
        pub next_hop_peer: ::validated_data::Field<bool>,
        pub next_hop_self: ::validated_data::Field<bool>,
        pub link_bandwidth: ::validated_data::Field<item::LinkBandwidth<'a, Mode>>,
        pub rib_in_pre_policy_retain: ::validated_data::Field<item::RibInPrePolicyRetain<'a, Mode>>,
        pub remove_private_as: ::validated_data::Field<item::RemovePrivateAs<'a, Mode>>,
        pub remove_private_as_ingress: ::validated_data::Field<item::RemovePrivateAsIngress<'a, Mode>>,
        pub session_tracker: ::validated_data::Field<&'a str>,
        pub shared_secret: ::validated_data::Field<item::SharedSecret<'a, Mode>>,
        pub ttl_maximum_hops: ::validated_data::Field<i64>,
        pub maximum_advertised_routes: ::validated_data::Field<i64>,
        pub maximum_advertised_routes_warning_limit: ::validated_data::Field<&'a str>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct AsPath<'a, Mode> {
            pub remote_as_replace_out: ::validated_data::Field<bool>,
            pub prepend_own_disabled: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct Metadata<'a, Mode> {
            pub peer: ::validated_data::Field<&'a str>,
            pub validate_state: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct BfdTimers<'a, Mode> {
            pub interval: ::validated_data::RequiredValue<i64, Mode>,
            pub min_rx: ::validated_data::RequiredValue<i64, Mode>,
            pub multiplier: ::validated_data::RequiredValue<i64, Mode>,
        }

        #[::validated_data::data_view]
        pub struct DefaultOriginate<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub always: ::validated_data::Field<bool>,
            pub route_map: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct MaximumAcceptedRoutes<'a, Mode> {
            pub limit: ::validated_data::RequiredValue<i64, Mode>,
            pub warning_limit: ::validated_data::Field<maximum_accepted_routes::WarningLimit<'a, Mode>>,
        }

        pub mod maximum_accepted_routes {

            #[::validated_data::data_view]
            pub struct WarningLimit<'a, Mode> {
                pub count: ::validated_data::Field<i64>,
                pub percent: ::validated_data::Field<i64>,
            }
        }

        #[::validated_data::data_view]
        pub struct MissingPolicy<'a, Mode> {
            pub direction_in: ::validated_data::Field<missing_policy::DirectionIn<'a, Mode>>,
            pub direction_out: ::validated_data::Field<missing_policy::DirectionOut<'a, Mode>>,
        }

        pub mod missing_policy {

            #[::validated_data::data_view]
            pub struct DirectionIn<'a, Mode> {
                pub action: ::validated_data::RequiredValue<&'a str, Mode>,
                pub include_community_list: ::validated_data::Field<bool>,
                pub include_prefix_list: ::validated_data::Field<bool>,
                pub include_sub_route_map: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view]
            pub struct DirectionOut<'a, Mode> {
                pub action: ::validated_data::RequiredValue<&'a str, Mode>,
                pub include_community_list: ::validated_data::Field<bool>,
                pub include_prefix_list: ::validated_data::Field<bool>,
                pub include_sub_route_map: ::validated_data::Field<bool>,
            }
        }

        #[::validated_data::data_view]
        pub struct AllowasIn<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub times: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view]
        pub struct LinkBandwidth<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub default: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct RibInPrePolicyRetain<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub all: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct RemovePrivateAs<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub all: ::validated_data::Field<bool>,
            pub replace_as: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct RemovePrivateAsIngress<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub replace_as: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct SharedSecret<'a, Mode> {
            pub profile: ::validated_data::RequiredValue<&'a str, Mode>,
            pub hash_algorithm: ::validated_data::RequiredValue<&'a str, Mode>,
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct NeighborInterfaces<'a, Mode> (::validated_data::Field<neighbor_interfaces::Item<'a, Mode>>);

pub mod neighbor_interfaces {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub remote_as: ::validated_data::Field<&'a str>,
        pub metadata: ::validated_data::Field<item::Metadata<'a, Mode>>,
        pub peer_group: ::validated_data::Field<&'a str>,
        pub description: ::validated_data::Field<&'a str>,
        pub peer_filter: ::validated_data::Field<&'a str>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct Metadata<'a, Mode> {
            pub peer: ::validated_data::Field<&'a str>,
            pub validate_state: ::validated_data::Field<bool>,
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(prefix))]
pub struct AggregateAddresses<'a, Mode> (::validated_data::Field<aggregate_addresses::Item<'a, Mode>>);

pub mod aggregate_addresses {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub prefix: ::validated_data::Field<&'a str>,
        pub advertise_only: ::validated_data::Field<bool>,
        pub as_set: ::validated_data::Field<bool>,
        pub summary_only: ::validated_data::Field<bool>,
        pub attribute_map: ::validated_data::Field<&'a str>,
        pub match_map: ::validated_data::Field<&'a str>,
        pub attribute: ::validated_data::Field<item::Attribute<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct Attribute<'a, Mode> {
            pub rcf: ::validated_data::Field<&'a str>,
        }
    }
}

#[::validated_data::data_view]
pub struct Redistribute<'a, Mode> {
    pub attached_host: ::validated_data::Field<redistribute::AttachedHost<'a, Mode>>,
    pub bgp: ::validated_data::Field<redistribute::Bgp<'a, Mode>>,
    pub connected: ::validated_data::Field<redistribute::Connected<'a, Mode>>,
    pub dynamic: ::validated_data::Field<redistribute::Dynamic<'a, Mode>>,
    pub isis: ::validated_data::Field<redistribute::Isis<'a, Mode>>,
    pub ospf: ::validated_data::Field<redistribute::Ospf<'a, Mode>>,
    pub ospfv3: ::validated_data::Field<redistribute::Ospfv3<'a, Mode>>,
    pub rip: ::validated_data::Field<redistribute::Rip<'a, Mode>>,
    #[data_view(rename = "static")]
    pub field_static: ::validated_data::Field<redistribute::FieldStatic<'a, Mode>>,
    pub user: ::validated_data::Field<redistribute::User<'a, Mode>>,
}

pub mod redistribute {

    #[::validated_data::data_view]
    pub struct AttachedHost<'a, Mode> {
        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
        pub route_map: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct Bgp<'a, Mode> {
        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
        pub route_map: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct Connected<'a, Mode> {
        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
        pub route_map: ::validated_data::Field<&'a str>,
        pub rcf: ::validated_data::Field<&'a str>,
        pub include_leaked: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view]
    pub struct Dynamic<'a, Mode> {
        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
        pub route_map: ::validated_data::Field<&'a str>,
        pub rcf: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct Isis<'a, Mode> {
        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
        pub isis_level: ::validated_data::Field<&'a str>,
        pub route_map: ::validated_data::Field<&'a str>,
        pub rcf: ::validated_data::Field<&'a str>,
        pub include_leaked: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view]
    pub struct Ospf<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub match_external: ::validated_data::Field<ospf::MatchExternal<'a, Mode>>,
        pub match_internal: ::validated_data::Field<ospf::MatchInternal<'a, Mode>>,
        pub match_nssa_external: ::validated_data::Field<ospf::MatchNssaExternal<'a, Mode>>,
        pub route_map: ::validated_data::Field<&'a str>,
        pub include_leaked: ::validated_data::Field<bool>,
    }

    pub mod ospf {

        #[::validated_data::data_view]
        pub struct MatchExternal<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct MatchInternal<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct MatchNssaExternal<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub nssa_type: ::validated_data::Field<i64>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view]
    pub struct Ospfv3<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub match_external: ::validated_data::Field<ospfv3::MatchExternal<'a, Mode>>,
        pub match_internal: ::validated_data::Field<ospfv3::MatchInternal<'a, Mode>>,
        pub match_nssa_external: ::validated_data::Field<ospfv3::MatchNssaExternal<'a, Mode>>,
        pub route_map: ::validated_data::Field<&'a str>,
        pub include_leaked: ::validated_data::Field<bool>,
    }

    pub mod ospfv3 {

        #[::validated_data::data_view]
        pub struct MatchExternal<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct MatchInternal<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct MatchNssaExternal<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub nssa_type: ::validated_data::Field<i64>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view]
    pub struct Rip<'a, Mode> {
        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
        pub route_map: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct FieldStatic<'a, Mode> {
        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
        pub route_map: ::validated_data::Field<&'a str>,
        pub rcf: ::validated_data::Field<&'a str>,
        pub include_leaked: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view]
    pub struct User<'a, Mode> {
        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
        pub rcf: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct VlanAwareBundles<'a, Mode> (::validated_data::Field<vlan_aware_bundles::Item<'a, Mode>>);

pub mod vlan_aware_bundles {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub metadata: ::validated_data::Field<item::Metadata<'a, Mode>>,
        pub rd: ::validated_data::Field<&'a str>,
        pub rd_evpn_domain: ::validated_data::Field<item::RdEvpnDomain<'a, Mode>>,
        pub route_targets: ::validated_data::Field<item::RouteTargets<'a, Mode>>,
        pub redistribute_routes: ::validated_data::Field<item::RedistributeRoutes<'a, Mode>>,
        pub no_redistribute_routes: ::validated_data::Field<item::NoRedistributeRoutes<'a, Mode>>,
        pub vlan: ::validated_data::Field<&'a str>,
        pub eos_cli: ::validated_data::Field<&'a str>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct Metadata<'a, Mode> {
            pub tenants: ::validated_data::Field<metadata::Tenants<'a, Mode>>,
            pub description: ::validated_data::Field<&'a str>,
        }

        pub mod metadata {

            #[::validated_data::data_view(list)]
            pub struct Tenants<'a, Mode> (::validated_data::Field<&'a str>);
        }

        #[::validated_data::data_view]
        pub struct RdEvpnDomain<'a, Mode> {
            pub domain: ::validated_data::Field<&'a str>,
            pub rd: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct RouteTargets<'a, Mode> {
            pub both: ::validated_data::Field<route_targets::Both<'a, Mode>>,
            #[data_view(rename = "import")]
            pub field_import: ::validated_data::Field<route_targets::Import<'a, Mode>>,
            pub export: ::validated_data::Field<route_targets::Export<'a, Mode>>,
            pub import_evpn_domains: ::validated_data::Field<route_targets::ImportEvpnDomains<'a, Mode>>,
            pub export_evpn_domains: ::validated_data::Field<route_targets::ExportEvpnDomains<'a, Mode>>,
            pub import_export_evpn_domains: ::validated_data::Field<route_targets::ImportExportEvpnDomains<'a, Mode>>,
        }

        pub mod route_targets {

            #[::validated_data::data_view(list)]
            pub struct Both<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list)]
            pub struct Import<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list)]
            pub struct Export<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list)]
            pub struct ImportEvpnDomains<'a, Mode> (::validated_data::Field<import_evpn_domains::Item<'a, Mode>>);

            pub mod import_evpn_domains {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub domain: ::validated_data::Field<&'a str>,
                    pub route_target: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view(list)]
            pub struct ExportEvpnDomains<'a, Mode> (::validated_data::Field<export_evpn_domains::Item<'a, Mode>>);

            pub mod export_evpn_domains {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub domain: ::validated_data::Field<&'a str>,
                    pub route_target: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view(list)]
            pub struct ImportExportEvpnDomains<'a, Mode> (::validated_data::Field<import_export_evpn_domains::Item<'a, Mode>>);

            pub mod import_export_evpn_domains {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub domain: ::validated_data::Field<&'a str>,
                    pub route_target: ::validated_data::Field<&'a str>,
                }
            }
        }

        #[::validated_data::data_view(list)]
        pub struct RedistributeRoutes<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list)]
        pub struct NoRedistributeRoutes<'a, Mode> (::validated_data::Field<&'a str>);
    }
}

#[::validated_data::data_view(indexed_list, primary_key(id))]
pub struct Vlans<'a, Mode> (::validated_data::Field<vlans::Item<'a, Mode>>);

pub mod vlans {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub id: ::validated_data::Field<i64>,
        pub metadata: ::validated_data::Field<item::Metadata<'a, Mode>>,
        pub rd: ::validated_data::Field<&'a str>,
        pub rd_evpn_domain: ::validated_data::Field<item::RdEvpnDomain<'a, Mode>>,
        pub route_targets: ::validated_data::Field<item::RouteTargets<'a, Mode>>,
        pub redistribute_routes: ::validated_data::Field<item::RedistributeRoutes<'a, Mode>>,
        pub no_redistribute_routes: ::validated_data::Field<item::NoRedistributeRoutes<'a, Mode>>,
        pub eos_cli: ::validated_data::Field<&'a str>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct Metadata<'a, Mode> {
            pub tenants: ::validated_data::Field<metadata::Tenants<'a, Mode>>,
        }

        pub mod metadata {

            #[::validated_data::data_view(list)]
            pub struct Tenants<'a, Mode> (::validated_data::Field<&'a str>);
        }

        #[::validated_data::data_view]
        pub struct RdEvpnDomain<'a, Mode> {
            pub domain: ::validated_data::Field<&'a str>,
            pub rd: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct RouteTargets<'a, Mode> {
            pub both: ::validated_data::Field<route_targets::Both<'a, Mode>>,
            #[data_view(rename = "import")]
            pub field_import: ::validated_data::Field<route_targets::Import<'a, Mode>>,
            pub export: ::validated_data::Field<route_targets::Export<'a, Mode>>,
            pub import_evpn_domains: ::validated_data::Field<route_targets::ImportEvpnDomains<'a, Mode>>,
            pub export_evpn_domains: ::validated_data::Field<route_targets::ExportEvpnDomains<'a, Mode>>,
            pub import_export_evpn_domains: ::validated_data::Field<route_targets::ImportExportEvpnDomains<'a, Mode>>,
        }

        pub mod route_targets {

            #[::validated_data::data_view(list)]
            pub struct Both<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list)]
            pub struct Import<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list)]
            pub struct Export<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list)]
            pub struct ImportEvpnDomains<'a, Mode> (::validated_data::Field<import_evpn_domains::Item<'a, Mode>>);

            pub mod import_evpn_domains {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub domain: ::validated_data::Field<&'a str>,
                    pub route_target: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view(list)]
            pub struct ExportEvpnDomains<'a, Mode> (::validated_data::Field<export_evpn_domains::Item<'a, Mode>>);

            pub mod export_evpn_domains {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub domain: ::validated_data::Field<&'a str>,
                    pub route_target: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view(list)]
            pub struct ImportExportEvpnDomains<'a, Mode> (::validated_data::Field<import_export_evpn_domains::Item<'a, Mode>>);

            pub mod import_export_evpn_domains {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub domain: ::validated_data::Field<&'a str>,
                    pub route_target: ::validated_data::Field<&'a str>,
                }
            }
        }

        #[::validated_data::data_view(list)]
        pub struct RedistributeRoutes<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list)]
        pub struct NoRedistributeRoutes<'a, Mode> (::validated_data::Field<&'a str>);
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Vpws<'a, Mode> (::validated_data::Field<vpws::Item<'a, Mode>>);

pub mod vpws {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub rd: ::validated_data::Field<&'a str>,
        pub route_targets: ::validated_data::Field<item::RouteTargets<'a, Mode>>,
        pub mpls_control_word: ::validated_data::Field<bool>,
        pub label_flow: ::validated_data::Field<bool>,
        pub mtu: ::validated_data::Field<i64>,
        pub pseudowires: ::validated_data::Field<item::Pseudowires<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct RouteTargets<'a, Mode> {
            pub import_export: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct Pseudowires<'a, Mode> (::validated_data::Field<pseudowires::Item<'a, Mode>>);

        pub mod pseudowires {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub id_local: ::validated_data::Field<i64>,
                pub id_remote: ::validated_data::Field<i64>,
            }
        }
    }
}

#[::validated_data::data_view]
pub struct AddressFamilyEvpn<'a, Mode> {
    pub domain_identifier: ::validated_data::Field<&'a str>,
    pub domain_identifier_remote: ::validated_data::Field<&'a str>,
    pub neighbor_default: ::validated_data::Field<address_family_evpn::NeighborDefault<'a, Mode>>,
    pub next_hop_mpls_resolution_ribs: ::validated_data::Field<address_family_evpn::NextHopMplsResolutionRibs<'a, Mode>>,
    pub neighbors: ::validated_data::Field<address_family_evpn::Neighbors<'a, Mode>>,
    pub peer_groups: ::validated_data::Field<address_family_evpn::PeerGroups<'a, Mode>>,
    pub evpn_hostflap_detection: ::validated_data::Field<address_family_evpn::EvpnHostflapDetection<'a, Mode>>,
    pub next_hop: ::validated_data::Field<address_family_evpn::NextHop<'a, Mode>>,
    pub route: ::validated_data::Field<address_family_evpn::Route<'a, Mode>>,
    pub next_hop_unchanged: ::validated_data::Field<bool>,
    pub bgp: ::validated_data::Field<address_family_evpn::Bgp<'a, Mode>>,
    pub layer_2_fec_in_place_update: ::validated_data::Field<address_family_evpn::Layer2FecInPlaceUpdate<'a, Mode>>,
    pub evpn_ethernet_segment: ::validated_data::Field<address_family_evpn::EvpnEthernetSegment<'a, Mode>>,
}

pub mod address_family_evpn {

    #[::validated_data::data_view]
    pub struct NeighborDefault<'a, Mode> {
        pub encapsulation: ::validated_data::Field<&'a str>,
        pub next_hop_self_source_interface: ::validated_data::Field<&'a str>,
        pub next_hop_self_received_evpn_routes: ::validated_data::Field<neighbor_default::NextHopSelfReceivedEvpnRoutes<'a, Mode>>,
    }

    pub mod neighbor_default {

        #[::validated_data::data_view]
        pub struct NextHopSelfReceivedEvpnRoutes<'a, Mode> {
            pub enable: ::validated_data::Field<bool>,
            pub inter_domain: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view(list)]
    pub struct NextHopMplsResolutionRibs<'a, Mode> (::validated_data::Field<next_hop_mpls_resolution_ribs::Item<'a, Mode>>);

    pub mod next_hop_mpls_resolution_ribs {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub rib_type: ::validated_data::RequiredValue<&'a str, Mode>,
            pub rib_name: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::validated_data::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub route_map_in: ::validated_data::Field<&'a str>,
            pub route_map_out: ::validated_data::Field<&'a str>,
            pub peer_tag_in: ::validated_data::Field<&'a str>,
            pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
            pub rcf_in: ::validated_data::Field<&'a str>,
            pub rcf_out: ::validated_data::Field<&'a str>,
            pub default_route: ::validated_data::Field<item::DefaultRoute<'a, Mode>>,
            pub additional_paths: ::validated_data::Field<item::AdditionalPaths<'a, Mode>>,
            pub encapsulation: ::validated_data::Field<&'a str>,
            pub next_hop_self_source_interface: ::validated_data::Field<&'a str>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct DefaultRoute<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub rcf: ::validated_data::Field<&'a str>,
                pub route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub receive: ::validated_data::Field<bool>,
                pub send: ::validated_data::Field<&'a str>,
                pub send_limit: ::validated_data::Field<i64>,
            }
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::validated_data::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub route_map_in: ::validated_data::Field<&'a str>,
            pub route_map_out: ::validated_data::Field<&'a str>,
            pub peer_tag_in: ::validated_data::Field<&'a str>,
            pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
            pub rcf_in: ::validated_data::Field<&'a str>,
            pub rcf_out: ::validated_data::Field<&'a str>,
            pub default_route: ::validated_data::Field<item::DefaultRoute<'a, Mode>>,
            pub domain_remote: ::validated_data::Field<bool>,
            pub encapsulation: ::validated_data::Field<&'a str>,
            pub next_hop_self_source_interface: ::validated_data::Field<&'a str>,
            pub additional_paths: ::validated_data::Field<item::AdditionalPaths<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct DefaultRoute<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub rcf: ::validated_data::Field<&'a str>,
                pub route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub receive: ::validated_data::Field<bool>,
                pub send: ::validated_data::Field<&'a str>,
                pub send_limit: ::validated_data::Field<i64>,
            }
        }
    }

    #[::validated_data::data_view]
    pub struct EvpnHostflapDetection<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub window: ::validated_data::Field<i64>,
        pub threshold: ::validated_data::Field<i64>,
        pub expiry_timeout: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct NextHop<'a, Mode> {
        pub resolution_disabled: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view]
    pub struct Route<'a, Mode> {
        pub import_match_failure_action: ::validated_data::Field<&'a str>,
        pub import_ethernet_segment_ip_mass_withdraw: ::validated_data::Field<bool>,
        pub import_overlay_index_gateway: ::validated_data::Field<bool>,
        pub export_ethernet_segment_ip_mass_withdraw: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view]
    pub struct Bgp<'a, Mode> {
        pub additional_paths: ::validated_data::Field<bgp::AdditionalPaths<'a, Mode>>,
    }

    pub mod bgp {

        #[::validated_data::data_view]
        pub struct AdditionalPaths<'a, Mode> {
            pub receive: ::validated_data::Field<bool>,
            pub send: ::validated_data::Field<&'a str>,
            pub send_limit: ::validated_data::Field<i64>,
        }
    }

    #[::validated_data::data_view]
    pub struct Layer2FecInPlaceUpdate<'a, Mode> {
        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
        pub timeout: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(indexed_list, primary_key(domain))]
    pub struct EvpnEthernetSegment<'a, Mode> (::validated_data::Field<evpn_ethernet_segment::Item<'a, Mode>>);

    pub mod evpn_ethernet_segment {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub domain: ::validated_data::Field<&'a str>,
            pub identifier: ::validated_data::Field<&'a str>,
            pub route_target_import: ::validated_data::Field<&'a str>,
        }
    }
}

#[::validated_data::data_view]
pub struct AddressFamilyRtc<'a, Mode> {
    pub peer_groups: ::validated_data::Field<address_family_rtc::PeerGroups<'a, Mode>>,
}

pub mod address_family_rtc {

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::validated_data::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub default_route_target: ::validated_data::Field<item::DefaultRouteTarget<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct DefaultRouteTarget<'a, Mode> {
                pub only: ::validated_data::Field<bool>,
                pub encoding_origin_as_omit: ::validated_data::Field<&'a str>,
            }
        }
    }
}

#[::validated_data::data_view]
pub struct AddressFamilyIpv4<'a, Mode> {
    pub networks: ::validated_data::Field<address_family_ipv4::Networks<'a, Mode>>,
    pub bgp: ::validated_data::Field<address_family_ipv4::Bgp<'a, Mode>>,
    pub peer_groups: ::validated_data::Field<address_family_ipv4::PeerGroups<'a, Mode>>,
    pub neighbors: ::validated_data::Field<address_family_ipv4::Neighbors<'a, Mode>>,
    pub redistribute: ::validated_data::Field<address_family_ipv4::Redistribute<'a, Mode>>,
    pub next_hop: ::validated_data::Field<address_family_ipv4::NextHop<'a, Mode>>,
}

pub mod address_family_ipv4 {

    #[::validated_data::data_view(indexed_list, primary_key(prefix))]
    pub struct Networks<'a, Mode> (::validated_data::Field<networks::Item<'a, Mode>>);

    pub mod networks {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub prefix: ::validated_data::Field<&'a str>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub rcf: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view]
    pub struct Bgp<'a, Mode> {
        pub additional_paths: ::validated_data::Field<bgp::AdditionalPaths<'a, Mode>>,
        pub redistribute_internal: ::validated_data::Field<bool>,
    }

    pub mod bgp {

        #[::validated_data::data_view]
        pub struct AdditionalPaths<'a, Mode> {
            pub install: ::validated_data::Field<bool>,
            pub install_ecmp_primary: ::validated_data::Field<bool>,
            pub receive: ::validated_data::Field<bool>,
            pub send: ::validated_data::Field<&'a str>,
            pub send_limit: ::validated_data::Field<i64>,
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::validated_data::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub route_map_in: ::validated_data::Field<&'a str>,
            pub route_map_out: ::validated_data::Field<&'a str>,
            pub peer_tag_in: ::validated_data::Field<&'a str>,
            pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
            pub rcf_in: ::validated_data::Field<&'a str>,
            pub rcf_out: ::validated_data::Field<&'a str>,
            pub default_originate: ::validated_data::Field<item::DefaultOriginate<'a, Mode>>,
            pub prefix_list_in: ::validated_data::Field<&'a str>,
            pub prefix_list_out: ::validated_data::Field<&'a str>,
            pub additional_paths: ::validated_data::Field<item::AdditionalPaths<'a, Mode>>,
            pub next_hop: ::validated_data::Field<item::NextHop<'a, Mode>>,
            pub maximum_advertised_routes: ::validated_data::Field<i64>,
            pub maximum_advertised_routes_warning_limit: ::validated_data::Field<&'a str>,
            pub maximum_accepted_routes: ::validated_data::Field<item::MaximumAcceptedRoutes<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct DefaultOriginate<'a, Mode> {
                pub always: ::validated_data::Field<bool>,
                pub route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub prefix_list: ::validated_data::Field<&'a str>,
                pub receive: ::validated_data::Field<bool>,
                pub send: ::validated_data::Field<&'a str>,
                pub send_limit: ::validated_data::Field<i64>,
            }

            #[::validated_data::data_view]
            pub struct NextHop<'a, Mode> {
                pub address_family_ipv6: ::validated_data::Field<next_hop::AddressFamilyIpv6<'a, Mode>>,
            }

            pub mod next_hop {

                #[::validated_data::data_view]
                pub struct AddressFamilyIpv6<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub originate: ::validated_data::Field<bool>,
                }
            }

            #[::validated_data::data_view]
            pub struct MaximumAcceptedRoutes<'a, Mode> {
                pub limit: ::validated_data::RequiredValue<i64, Mode>,
                pub warning_limit: ::validated_data::Field<maximum_accepted_routes::WarningLimit<'a, Mode>>,
            }

            pub mod maximum_accepted_routes {

                #[::validated_data::data_view]
                pub struct WarningLimit<'a, Mode> {
                    pub count: ::validated_data::Field<i64>,
                    pub percent: ::validated_data::Field<i64>,
                }
            }
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::validated_data::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub route_map_in: ::validated_data::Field<&'a str>,
            pub route_map_out: ::validated_data::Field<&'a str>,
            pub peer_tag_in: ::validated_data::Field<&'a str>,
            pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
            pub rcf_in: ::validated_data::Field<&'a str>,
            pub rcf_out: ::validated_data::Field<&'a str>,
            pub prefix_list_in: ::validated_data::Field<&'a str>,
            pub prefix_list_out: ::validated_data::Field<&'a str>,
            pub default_originate: ::validated_data::Field<item::DefaultOriginate<'a, Mode>>,
            pub additional_paths: ::validated_data::Field<item::AdditionalPaths<'a, Mode>>,
            pub next_hop: ::validated_data::Field<item::NextHop<'a, Mode>>,
            pub maximum_advertised_routes: ::validated_data::Field<i64>,
            pub maximum_advertised_routes_warning_limit: ::validated_data::Field<&'a str>,
            pub maximum_accepted_routes: ::validated_data::Field<item::MaximumAcceptedRoutes<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct DefaultOriginate<'a, Mode> {
                pub always: ::validated_data::Field<bool>,
                pub route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub prefix_list: ::validated_data::Field<&'a str>,
                pub receive: ::validated_data::Field<bool>,
                pub send: ::validated_data::Field<&'a str>,
                pub send_limit: ::validated_data::Field<i64>,
            }

            #[::validated_data::data_view]
            pub struct NextHop<'a, Mode> {
                pub address_family_ipv6: ::validated_data::Field<next_hop::AddressFamilyIpv6<'a, Mode>>,
            }

            pub mod next_hop {

                #[::validated_data::data_view]
                pub struct AddressFamilyIpv6<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub originate: ::validated_data::Field<bool>,
                }
            }

            #[::validated_data::data_view]
            pub struct MaximumAcceptedRoutes<'a, Mode> {
                pub limit: ::validated_data::RequiredValue<i64, Mode>,
                pub warning_limit: ::validated_data::Field<maximum_accepted_routes::WarningLimit<'a, Mode>>,
            }

            pub mod maximum_accepted_routes {

                #[::validated_data::data_view]
                pub struct WarningLimit<'a, Mode> {
                    pub count: ::validated_data::Field<i64>,
                    pub percent: ::validated_data::Field<i64>,
                }
            }
        }
    }

    #[::validated_data::data_view]
    pub struct Redistribute<'a, Mode> {
        pub attached_host: ::validated_data::Field<redistribute::AttachedHost<'a, Mode>>,
        pub bgp: ::validated_data::Field<redistribute::Bgp<'a, Mode>>,
        pub connected: ::validated_data::Field<redistribute::Connected<'a, Mode>>,
        pub dynamic: ::validated_data::Field<redistribute::Dynamic<'a, Mode>>,
        pub isis: ::validated_data::Field<redistribute::Isis<'a, Mode>>,
        pub ospf: ::validated_data::Field<redistribute::Ospf<'a, Mode>>,
        pub ospfv3: ::validated_data::Field<redistribute::Ospfv3<'a, Mode>>,
        pub rip: ::validated_data::Field<redistribute::Rip<'a, Mode>>,
        #[data_view(rename = "static")]
        pub field_static: ::validated_data::Field<redistribute::FieldStatic<'a, Mode>>,
        pub user: ::validated_data::Field<redistribute::User<'a, Mode>>,
    }

    pub mod redistribute {

        #[::validated_data::data_view]
        pub struct AttachedHost<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct Bgp<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct Connected<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub rcf: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct Dynamic<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub rcf: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct Isis<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub isis_level: ::validated_data::Field<&'a str>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub rcf: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct Ospf<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub match_external: ::validated_data::Field<ospf::MatchExternal<'a, Mode>>,
            pub match_internal: ::validated_data::Field<ospf::MatchInternal<'a, Mode>>,
            pub match_nssa_external: ::validated_data::Field<ospf::MatchNssaExternal<'a, Mode>>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }

        pub mod ospf {

            #[::validated_data::data_view]
            pub struct MatchExternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
                pub include_leaked: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view]
            pub struct MatchInternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
                pub include_leaked: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view]
            pub struct MatchNssaExternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub nssa_type: ::validated_data::Field<i64>,
                pub route_map: ::validated_data::Field<&'a str>,
                pub include_leaked: ::validated_data::Field<bool>,
            }
        }

        #[::validated_data::data_view]
        pub struct Ospfv3<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub match_external: ::validated_data::Field<ospfv3::MatchExternal<'a, Mode>>,
            pub match_internal: ::validated_data::Field<ospfv3::MatchInternal<'a, Mode>>,
            pub match_nssa_external: ::validated_data::Field<ospfv3::MatchNssaExternal<'a, Mode>>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }

        pub mod ospfv3 {

            #[::validated_data::data_view]
            pub struct MatchExternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
                pub include_leaked: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view]
            pub struct MatchInternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
                pub include_leaked: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view]
            pub struct MatchNssaExternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub nssa_type: ::validated_data::Field<i64>,
                pub route_map: ::validated_data::Field<&'a str>,
                pub include_leaked: ::validated_data::Field<bool>,
            }
        }

        #[::validated_data::data_view]
        pub struct Rip<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct FieldStatic<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub rcf: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct User<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub rcf: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view]
    pub struct NextHop<'a, Mode> {
        pub resolution_disabled: ::validated_data::Field<bool>,
    }
}

#[::validated_data::data_view]
pub struct AddressFamilyIpv4LabeledUnicast<'a, Mode> {
    pub aigp_session: ::validated_data::Field<address_family_ipv4_labeled_unicast::AigpSession<'a, Mode>>,
    pub bgp: ::validated_data::Field<address_family_ipv4_labeled_unicast::Bgp<'a, Mode>>,
    pub graceful_restart: ::validated_data::Field<bool>,
    pub label_local_termination: ::validated_data::Field<&'a str>,
    pub lfib_entry_installation_skipped: ::validated_data::Field<bool>,
    pub neighbor_default: ::validated_data::Field<address_family_ipv4_labeled_unicast::NeighborDefault<'a, Mode>>,
    pub peer_groups: ::validated_data::Field<address_family_ipv4_labeled_unicast::PeerGroups<'a, Mode>>,
    pub neighbors: ::validated_data::Field<address_family_ipv4_labeled_unicast::Neighbors<'a, Mode>>,
    pub networks: ::validated_data::Field<address_family_ipv4_labeled_unicast::Networks<'a, Mode>>,
    pub next_hop: ::validated_data::Field<address_family_ipv4_labeled_unicast::NextHop<'a, Mode>>,
    pub next_hops: ::validated_data::Field<address_family_ipv4_labeled_unicast::NextHops<'a, Mode>>,
    pub next_hop_resolution_ribs: ::validated_data::Field<address_family_ipv4_labeled_unicast::NextHopResolutionRibs<'a, Mode>>,
    pub tunnel_source_protocols: ::validated_data::Field<address_family_ipv4_labeled_unicast::TunnelSourceProtocols<'a, Mode>>,
    pub update_wait_for_convergence: ::validated_data::Field<bool>,
}

pub mod address_family_ipv4_labeled_unicast {

    #[::validated_data::data_view]
    pub struct AigpSession<'a, Mode> {
        pub confederation: ::validated_data::Field<bool>,
        pub ebgp: ::validated_data::Field<bool>,
        pub ibgp: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view]
    pub struct Bgp<'a, Mode> {
        pub additional_paths: ::validated_data::Field<bgp::AdditionalPaths<'a, Mode>>,
        pub missing_policy: ::validated_data::Field<bgp::MissingPolicy<'a, Mode>>,
        pub next_hop_unchanged: ::validated_data::Field<bool>,
    }

    pub mod bgp {

        #[::validated_data::data_view]
        pub struct AdditionalPaths<'a, Mode> {
            pub receive: ::validated_data::Field<bool>,
            pub send: ::validated_data::Field<&'a str>,
            pub send_limit: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view]
        pub struct MissingPolicy<'a, Mode> {
            pub direction_in: ::validated_data::Field<missing_policy::DirectionIn<'a, Mode>>,
            pub direction_out: ::validated_data::Field<missing_policy::DirectionOut<'a, Mode>>,
        }

        pub mod missing_policy {

            #[::validated_data::data_view]
            pub struct DirectionIn<'a, Mode> {
                pub action: ::validated_data::RequiredValue<&'a str, Mode>,
                pub include_community_list: ::validated_data::Field<bool>,
                pub include_prefix_list: ::validated_data::Field<bool>,
                pub include_sub_route_map: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view]
            pub struct DirectionOut<'a, Mode> {
                pub action: ::validated_data::RequiredValue<&'a str, Mode>,
                pub include_community_list: ::validated_data::Field<bool>,
                pub include_prefix_list: ::validated_data::Field<bool>,
                pub include_sub_route_map: ::validated_data::Field<bool>,
            }
        }
    }

    #[::validated_data::data_view]
    pub struct NeighborDefault<'a, Mode> {
        pub next_hop_self: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::validated_data::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub additional_paths: ::validated_data::Field<item::AdditionalPaths<'a, Mode>>,
            pub aigp_session: ::validated_data::Field<bool>,
            pub graceful_restart: ::validated_data::Field<bool>,
            pub graceful_restart_helper: ::validated_data::Field<item::GracefulRestartHelper<'a, Mode>>,
            pub maximum_advertised_routes: ::validated_data::Field<i64>,
            pub maximum_advertised_routes_warning_limit: ::validated_data::Field<&'a str>,
            pub missing_policy: ::validated_data::Field<item::MissingPolicy<'a, Mode>>,
            pub multi_path: ::validated_data::Field<bool>,
            pub next_hop_self: ::validated_data::Field<bool>,
            pub next_hop_self_source_interface: ::validated_data::Field<&'a str>,
            pub next_hop_self_v4_mapped_v6_source_interface: ::validated_data::Field<&'a str>,
            pub next_hop_unchanged: ::validated_data::Field<bool>,
            pub rcf_in: ::validated_data::Field<&'a str>,
            pub rcf_out: ::validated_data::Field<&'a str>,
            pub route_map_in: ::validated_data::Field<&'a str>,
            pub route_map_out: ::validated_data::Field<&'a str>,
            pub peer_tag_in: ::validated_data::Field<&'a str>,
            pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub receive: ::validated_data::Field<bool>,
                pub send: ::validated_data::Field<&'a str>,
                pub send_limit: ::validated_data::Field<i64>,
            }

            #[::validated_data::data_view]
            pub struct GracefulRestartHelper<'a, Mode> {
                pub stale_route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct MissingPolicy<'a, Mode> {
                pub direction_in: ::validated_data::Field<missing_policy::DirectionIn<'a, Mode>>,
                pub direction_out: ::validated_data::Field<missing_policy::DirectionOut<'a, Mode>>,
            }

            pub mod missing_policy {

                #[::validated_data::data_view]
                pub struct DirectionIn<'a, Mode> {
                    pub action: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub include_community_list: ::validated_data::Field<bool>,
                    pub include_prefix_list: ::validated_data::Field<bool>,
                    pub include_sub_route_map: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct DirectionOut<'a, Mode> {
                    pub action: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub include_community_list: ::validated_data::Field<bool>,
                    pub include_prefix_list: ::validated_data::Field<bool>,
                    pub include_sub_route_map: ::validated_data::Field<bool>,
                }
            }
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::validated_data::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub additional_paths: ::validated_data::Field<item::AdditionalPaths<'a, Mode>>,
            pub aigp_session: ::validated_data::Field<bool>,
            pub graceful_restart: ::validated_data::Field<bool>,
            pub graceful_restart_helper: ::validated_data::Field<item::GracefulRestartHelper<'a, Mode>>,
            pub maximum_advertised_routes: ::validated_data::Field<i64>,
            pub maximum_advertised_routes_warning_limit: ::validated_data::Field<&'a str>,
            pub missing_policy: ::validated_data::Field<item::MissingPolicy<'a, Mode>>,
            pub multi_path: ::validated_data::Field<bool>,
            pub next_hop_self: ::validated_data::Field<bool>,
            pub next_hop_self_source_interface: ::validated_data::Field<&'a str>,
            pub next_hop_self_v4_mapped_v6_source_interface: ::validated_data::Field<&'a str>,
            pub next_hop_unchanged: ::validated_data::Field<bool>,
            pub rcf_in: ::validated_data::Field<&'a str>,
            pub rcf_out: ::validated_data::Field<&'a str>,
            pub route_map_in: ::validated_data::Field<&'a str>,
            pub route_map_out: ::validated_data::Field<&'a str>,
            pub peer_tag_in: ::validated_data::Field<&'a str>,
            pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub receive: ::validated_data::Field<bool>,
                pub send: ::validated_data::Field<&'a str>,
                pub send_limit: ::validated_data::Field<i64>,
            }

            #[::validated_data::data_view]
            pub struct GracefulRestartHelper<'a, Mode> {
                pub stale_route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct MissingPolicy<'a, Mode> {
                pub direction_in: ::validated_data::Field<missing_policy::DirectionIn<'a, Mode>>,
                pub direction_out: ::validated_data::Field<missing_policy::DirectionOut<'a, Mode>>,
            }

            pub mod missing_policy {

                #[::validated_data::data_view]
                pub struct DirectionIn<'a, Mode> {
                    pub action: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub include_community_list: ::validated_data::Field<bool>,
                    pub include_prefix_list: ::validated_data::Field<bool>,
                    pub include_sub_route_map: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct DirectionOut<'a, Mode> {
                    pub action: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub include_community_list: ::validated_data::Field<bool>,
                    pub include_prefix_list: ::validated_data::Field<bool>,
                    pub include_sub_route_map: ::validated_data::Field<bool>,
                }
            }
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(prefix))]
    pub struct Networks<'a, Mode> (::validated_data::Field<networks::Item<'a, Mode>>);

    pub mod networks {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub prefix: ::validated_data::Field<&'a str>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub rcf: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view]
    pub struct NextHop<'a, Mode> {
        pub resolution_disabled: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
    pub struct NextHops<'a, Mode> (::validated_data::Field<next_hops::Item<'a, Mode>>);

    pub mod next_hops {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::validated_data::Field<&'a str>,
            pub lfib_backup_ip_forwarding: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view(list)]
    pub struct NextHopResolutionRibs<'a, Mode> (::validated_data::Field<next_hop_resolution_ribs::Item<'a, Mode>>);

    pub mod next_hop_resolution_ribs {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub rib_type: ::validated_data::RequiredValue<&'a str, Mode>,
            pub rib_name: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(protocol))]
    pub struct TunnelSourceProtocols<'a, Mode> (::validated_data::Field<tunnel_source_protocols::Item<'a, Mode>>);

    pub mod tunnel_source_protocols {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub protocol: ::validated_data::Field<&'a str>,
            pub rcf: ::validated_data::Field<&'a str>,
        }
    }
}

#[::validated_data::data_view]
pub struct AddressFamilyIpv4Multicast<'a, Mode> {
    pub bgp: ::validated_data::Field<address_family_ipv4_multicast::Bgp<'a, Mode>>,
    pub peer_groups: ::validated_data::Field<address_family_ipv4_multicast::PeerGroups<'a, Mode>>,
    pub neighbors: ::validated_data::Field<address_family_ipv4_multicast::Neighbors<'a, Mode>>,
    pub redistribute: ::validated_data::Field<address_family_ipv4_multicast::Redistribute<'a, Mode>>,
}

pub mod address_family_ipv4_multicast {

    #[::validated_data::data_view]
    pub struct Bgp<'a, Mode> {
        pub additional_paths: ::validated_data::Field<bgp::AdditionalPaths<'a, Mode>>,
    }

    pub mod bgp {

        #[::validated_data::data_view]
        pub struct AdditionalPaths<'a, Mode> {
            pub receive: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::validated_data::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub route_map_in: ::validated_data::Field<&'a str>,
            pub route_map_out: ::validated_data::Field<&'a str>,
            pub peer_tag_in: ::validated_data::Field<&'a str>,
            pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
            pub additional_paths: ::validated_data::Field<item::AdditionalPaths<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub receive: ::validated_data::Field<bool>,
            }
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::validated_data::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub route_map_in: ::validated_data::Field<&'a str>,
            pub route_map_out: ::validated_data::Field<&'a str>,
            pub peer_tag_in: ::validated_data::Field<&'a str>,
            pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
            pub additional_paths: ::validated_data::Field<item::AdditionalPaths<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub receive: ::validated_data::Field<bool>,
            }
        }
    }

    #[::validated_data::data_view]
    pub struct Redistribute<'a, Mode> {
        pub attached_host: ::validated_data::Field<redistribute::AttachedHost<'a, Mode>>,
        pub connected: ::validated_data::Field<redistribute::Connected<'a, Mode>>,
        pub isis: ::validated_data::Field<redistribute::Isis<'a, Mode>>,
        pub ospf: ::validated_data::Field<redistribute::Ospf<'a, Mode>>,
        pub ospfv3: ::validated_data::Field<redistribute::Ospfv3<'a, Mode>>,
        #[data_view(rename = "static")]
        pub field_static: ::validated_data::Field<redistribute::FieldStatic<'a, Mode>>,
    }

    pub mod redistribute {

        #[::validated_data::data_view]
        pub struct AttachedHost<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct Connected<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct Isis<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub isis_level: ::validated_data::Field<&'a str>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub rcf: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct Ospf<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub match_external: ::validated_data::Field<ospf::MatchExternal<'a, Mode>>,
            pub match_internal: ::validated_data::Field<ospf::MatchInternal<'a, Mode>>,
            pub match_nssa_external: ::validated_data::Field<ospf::MatchNssaExternal<'a, Mode>>,
            pub route_map: ::validated_data::Field<&'a str>,
        }

        pub mod ospf {

            #[::validated_data::data_view]
            pub struct MatchExternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct MatchInternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct MatchNssaExternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub nssa_type: ::validated_data::Field<i64>,
                pub route_map: ::validated_data::Field<&'a str>,
            }
        }

        #[::validated_data::data_view]
        pub struct Ospfv3<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub match_external: ::validated_data::Field<ospfv3::MatchExternal<'a, Mode>>,
            pub match_internal: ::validated_data::Field<ospfv3::MatchInternal<'a, Mode>>,
            pub match_nssa_external: ::validated_data::Field<ospfv3::MatchNssaExternal<'a, Mode>>,
            pub route_map: ::validated_data::Field<&'a str>,
        }

        pub mod ospfv3 {

            #[::validated_data::data_view]
            pub struct MatchExternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct MatchInternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct MatchNssaExternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub nssa_type: ::validated_data::Field<i64>,
                pub route_map: ::validated_data::Field<&'a str>,
            }
        }

        #[::validated_data::data_view]
        pub struct FieldStatic<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
        }
    }
}

#[::validated_data::data_view]
pub struct AddressFamilyIpv4SrTe<'a, Mode> {
    pub neighbors: ::validated_data::Field<address_family_ipv4_sr_te::Neighbors<'a, Mode>>,
    pub peer_groups: ::validated_data::Field<address_family_ipv4_sr_te::PeerGroups<'a, Mode>>,
}

pub mod address_family_ipv4_sr_te {

    #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::validated_data::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub route_map_in: ::validated_data::Field<&'a str>,
            pub route_map_out: ::validated_data::Field<&'a str>,
            pub peer_tag_in: ::validated_data::Field<&'a str>,
            pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::validated_data::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub route_map_in: ::validated_data::Field<&'a str>,
            pub route_map_out: ::validated_data::Field<&'a str>,
            pub peer_tag_in: ::validated_data::Field<&'a str>,
            pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
        }
    }
}

#[::validated_data::data_view]
pub struct AddressFamilyIpv6<'a, Mode> {
    pub networks: ::validated_data::Field<address_family_ipv6::Networks<'a, Mode>>,
    pub bgp: ::validated_data::Field<address_family_ipv6::Bgp<'a, Mode>>,
    pub peer_groups: ::validated_data::Field<address_family_ipv6::PeerGroups<'a, Mode>>,
    pub neighbors: ::validated_data::Field<address_family_ipv6::Neighbors<'a, Mode>>,
    pub redistribute: ::validated_data::Field<address_family_ipv6::Redistribute<'a, Mode>>,
    pub next_hop: ::validated_data::Field<address_family_ipv6::NextHop<'a, Mode>>,
}

pub mod address_family_ipv6 {

    #[::validated_data::data_view(indexed_list, primary_key(prefix))]
    pub struct Networks<'a, Mode> (::validated_data::Field<networks::Item<'a, Mode>>);

    pub mod networks {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub prefix: ::validated_data::Field<&'a str>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub rcf: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view]
    pub struct Bgp<'a, Mode> {
        pub redistribute_internal: ::validated_data::Field<bool>,
        pub additional_paths: ::validated_data::Field<bgp::AdditionalPaths<'a, Mode>>,
    }

    pub mod bgp {

        #[::validated_data::data_view]
        pub struct AdditionalPaths<'a, Mode> {
            pub install: ::validated_data::Field<bool>,
            pub install_ecmp_primary: ::validated_data::Field<bool>,
            pub receive: ::validated_data::Field<bool>,
            pub send: ::validated_data::Field<&'a str>,
            pub send_limit: ::validated_data::Field<i64>,
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::validated_data::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub route_map_in: ::validated_data::Field<&'a str>,
            pub route_map_out: ::validated_data::Field<&'a str>,
            pub peer_tag_in: ::validated_data::Field<&'a str>,
            pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
            pub rcf_in: ::validated_data::Field<&'a str>,
            pub rcf_out: ::validated_data::Field<&'a str>,
            pub prefix_list_in: ::validated_data::Field<&'a str>,
            pub prefix_list_out: ::validated_data::Field<&'a str>,
            pub additional_paths: ::validated_data::Field<item::AdditionalPaths<'a, Mode>>,
            pub default_originate: ::validated_data::Field<item::DefaultOriginate<'a, Mode>>,
            pub maximum_advertised_routes: ::validated_data::Field<i64>,
            pub maximum_advertised_routes_warning_limit: ::validated_data::Field<&'a str>,
            pub maximum_accepted_routes: ::validated_data::Field<item::MaximumAcceptedRoutes<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub prefix_list: ::validated_data::Field<&'a str>,
                pub receive: ::validated_data::Field<bool>,
                pub send: ::validated_data::Field<&'a str>,
                pub send_limit: ::validated_data::Field<i64>,
            }

            #[::validated_data::data_view]
            pub struct DefaultOriginate<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub always: ::validated_data::Field<bool>,
                pub route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct MaximumAcceptedRoutes<'a, Mode> {
                pub limit: ::validated_data::RequiredValue<i64, Mode>,
                pub warning_limit: ::validated_data::Field<maximum_accepted_routes::WarningLimit<'a, Mode>>,
            }

            pub mod maximum_accepted_routes {

                #[::validated_data::data_view]
                pub struct WarningLimit<'a, Mode> {
                    pub count: ::validated_data::Field<i64>,
                    pub percent: ::validated_data::Field<i64>,
                }
            }
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::validated_data::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub route_map_in: ::validated_data::Field<&'a str>,
            pub route_map_out: ::validated_data::Field<&'a str>,
            pub peer_tag_in: ::validated_data::Field<&'a str>,
            pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
            pub rcf_in: ::validated_data::Field<&'a str>,
            pub rcf_out: ::validated_data::Field<&'a str>,
            pub prefix_list_in: ::validated_data::Field<&'a str>,
            pub prefix_list_out: ::validated_data::Field<&'a str>,
            pub default_originate: ::validated_data::Field<item::DefaultOriginate<'a, Mode>>,
            pub additional_paths: ::validated_data::Field<item::AdditionalPaths<'a, Mode>>,
            pub maximum_advertised_routes: ::validated_data::Field<i64>,
            pub maximum_advertised_routes_warning_limit: ::validated_data::Field<&'a str>,
            pub maximum_accepted_routes: ::validated_data::Field<item::MaximumAcceptedRoutes<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct DefaultOriginate<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub always: ::validated_data::Field<bool>,
                pub route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub prefix_list: ::validated_data::Field<&'a str>,
                pub receive: ::validated_data::Field<bool>,
                pub send: ::validated_data::Field<&'a str>,
                pub send_limit: ::validated_data::Field<i64>,
            }

            #[::validated_data::data_view]
            pub struct MaximumAcceptedRoutes<'a, Mode> {
                pub limit: ::validated_data::RequiredValue<i64, Mode>,
                pub warning_limit: ::validated_data::Field<maximum_accepted_routes::WarningLimit<'a, Mode>>,
            }

            pub mod maximum_accepted_routes {

                #[::validated_data::data_view]
                pub struct WarningLimit<'a, Mode> {
                    pub count: ::validated_data::Field<i64>,
                    pub percent: ::validated_data::Field<i64>,
                }
            }
        }
    }

    #[::validated_data::data_view]
    pub struct Redistribute<'a, Mode> {
        pub attached_host: ::validated_data::Field<redistribute::AttachedHost<'a, Mode>>,
        pub bgp: ::validated_data::Field<redistribute::Bgp<'a, Mode>>,
        pub connected: ::validated_data::Field<redistribute::Connected<'a, Mode>>,
        pub dhcp: ::validated_data::Field<redistribute::Dhcp<'a, Mode>>,
        pub dynamic: ::validated_data::Field<redistribute::Dynamic<'a, Mode>>,
        pub isis: ::validated_data::Field<redistribute::Isis<'a, Mode>>,
        pub ospfv3: ::validated_data::Field<redistribute::Ospfv3<'a, Mode>>,
        #[data_view(rename = "static")]
        pub field_static: ::validated_data::Field<redistribute::FieldStatic<'a, Mode>>,
        pub user: ::validated_data::Field<redistribute::User<'a, Mode>>,
    }

    pub mod redistribute {

        #[::validated_data::data_view]
        pub struct AttachedHost<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct Bgp<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct Connected<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub rcf: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct Dhcp<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct Dynamic<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub rcf: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct Isis<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub isis_level: ::validated_data::Field<&'a str>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub rcf: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct Ospfv3<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub match_external: ::validated_data::Field<ospfv3::MatchExternal<'a, Mode>>,
            pub match_internal: ::validated_data::Field<ospfv3::MatchInternal<'a, Mode>>,
            pub match_nssa_external: ::validated_data::Field<ospfv3::MatchNssaExternal<'a, Mode>>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }

        pub mod ospfv3 {

            #[::validated_data::data_view]
            pub struct MatchExternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
                pub include_leaked: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view]
            pub struct MatchInternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
                pub include_leaked: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view]
            pub struct MatchNssaExternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub nssa_type: ::validated_data::Field<i64>,
                pub route_map: ::validated_data::Field<&'a str>,
                pub include_leaked: ::validated_data::Field<bool>,
            }
        }

        #[::validated_data::data_view]
        pub struct FieldStatic<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub rcf: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct User<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub rcf: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view]
    pub struct NextHop<'a, Mode> {
        pub resolution_disabled: ::validated_data::Field<bool>,
    }
}

#[::validated_data::data_view]
pub struct AddressFamilyIpv6Multicast<'a, Mode> {
    pub bgp: ::validated_data::Field<address_family_ipv6_multicast::Bgp<'a, Mode>>,
    pub neighbors: ::validated_data::Field<address_family_ipv6_multicast::Neighbors<'a, Mode>>,
    pub peer_groups: ::validated_data::Field<address_family_ipv6_multicast::PeerGroups<'a, Mode>>,
    pub networks: ::validated_data::Field<address_family_ipv6_multicast::Networks<'a, Mode>>,
    pub redistribute: ::validated_data::Field<address_family_ipv6_multicast::Redistribute<'a, Mode>>,
}

pub mod address_family_ipv6_multicast {

    #[::validated_data::data_view]
    pub struct Bgp<'a, Mode> {
        pub missing_policy: ::validated_data::Field<bgp::MissingPolicy<'a, Mode>>,
        pub additional_paths: ::validated_data::Field<bgp::AdditionalPaths<'a, Mode>>,
    }

    pub mod bgp {

        #[::validated_data::data_view]
        pub struct MissingPolicy<'a, Mode> {
            pub direction_in_action: ::validated_data::Field<&'a str>,
            pub direction_out_action: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct AdditionalPaths<'a, Mode> {
            pub receive: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::validated_data::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub route_map_in: ::validated_data::Field<&'a str>,
            pub route_map_out: ::validated_data::Field<&'a str>,
            pub peer_tag_in: ::validated_data::Field<&'a str>,
            pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
            pub additional_paths: ::validated_data::Field<item::AdditionalPaths<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub receive: ::validated_data::Field<bool>,
            }
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::validated_data::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub additional_paths: ::validated_data::Field<item::AdditionalPaths<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub receive: ::validated_data::Field<bool>,
            }
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(prefix))]
    pub struct Networks<'a, Mode> (::validated_data::Field<networks::Item<'a, Mode>>);

    pub mod networks {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub prefix: ::validated_data::Field<&'a str>,
            pub route_map: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view]
    pub struct Redistribute<'a, Mode> {
        pub connected: ::validated_data::Field<redistribute::Connected<'a, Mode>>,
        pub isis: ::validated_data::Field<redistribute::Isis<'a, Mode>>,
        pub ospf: ::validated_data::Field<redistribute::Ospf<'a, Mode>>,
        pub ospfv3: ::validated_data::Field<redistribute::Ospfv3<'a, Mode>>,
        #[data_view(rename = "static")]
        pub field_static: ::validated_data::Field<redistribute::FieldStatic<'a, Mode>>,
    }

    pub mod redistribute {

        #[::validated_data::data_view]
        pub struct Connected<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct Isis<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub isis_level: ::validated_data::Field<&'a str>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub rcf: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct Ospf<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub match_external: ::validated_data::Field<ospf::MatchExternal<'a, Mode>>,
            pub match_internal: ::validated_data::Field<ospf::MatchInternal<'a, Mode>>,
            pub match_nssa_external: ::validated_data::Field<ospf::MatchNssaExternal<'a, Mode>>,
            pub route_map: ::validated_data::Field<&'a str>,
        }

        pub mod ospf {

            #[::validated_data::data_view]
            pub struct MatchExternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct MatchInternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct MatchNssaExternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub nssa_type: ::validated_data::Field<i64>,
                pub route_map: ::validated_data::Field<&'a str>,
            }
        }

        #[::validated_data::data_view]
        pub struct Ospfv3<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub match_external: ::validated_data::Field<ospfv3::MatchExternal<'a, Mode>>,
            pub match_internal: ::validated_data::Field<ospfv3::MatchInternal<'a, Mode>>,
            pub match_nssa_external: ::validated_data::Field<ospfv3::MatchNssaExternal<'a, Mode>>,
            pub route_map: ::validated_data::Field<&'a str>,
        }

        pub mod ospfv3 {

            #[::validated_data::data_view]
            pub struct MatchExternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct MatchInternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct MatchNssaExternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub nssa_type: ::validated_data::Field<i64>,
                pub route_map: ::validated_data::Field<&'a str>,
            }
        }

        #[::validated_data::data_view]
        pub struct FieldStatic<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
        }
    }
}

#[::validated_data::data_view]
pub struct AddressFamilyIpv6SrTe<'a, Mode> {
    pub neighbors: ::validated_data::Field<address_family_ipv6_sr_te::Neighbors<'a, Mode>>,
    pub peer_groups: ::validated_data::Field<address_family_ipv6_sr_te::PeerGroups<'a, Mode>>,
}

pub mod address_family_ipv6_sr_te {

    #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::validated_data::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub route_map_in: ::validated_data::Field<&'a str>,
            pub route_map_out: ::validated_data::Field<&'a str>,
            pub peer_tag_in: ::validated_data::Field<&'a str>,
            pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::validated_data::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub route_map_in: ::validated_data::Field<&'a str>,
            pub route_map_out: ::validated_data::Field<&'a str>,
            pub peer_tag_in: ::validated_data::Field<&'a str>,
            pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
        }
    }
}

#[::validated_data::data_view]
pub struct AddressFamilyLinkState<'a, Mode> {
    pub bgp: ::validated_data::Field<address_family_link_state::Bgp<'a, Mode>>,
    pub peer_groups: ::validated_data::Field<address_family_link_state::PeerGroups<'a, Mode>>,
    pub neighbors: ::validated_data::Field<address_family_link_state::Neighbors<'a, Mode>>,
    pub path_selection: ::validated_data::Field<address_family_link_state::PathSelection<'a, Mode>>,
}

pub mod address_family_link_state {

    #[::validated_data::data_view]
    pub struct Bgp<'a, Mode> {
        pub missing_policy: ::validated_data::Field<bgp::MissingPolicy<'a, Mode>>,
    }

    pub mod bgp {

        #[::validated_data::data_view]
        pub struct MissingPolicy<'a, Mode> {
            pub direction_in_action: ::validated_data::Field<&'a str>,
            pub direction_out_action: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::validated_data::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub missing_policy: ::validated_data::Field<item::MissingPolicy<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct MissingPolicy<'a, Mode> {
                pub direction_in_action: ::validated_data::Field<&'a str>,
                pub direction_out_action: ::validated_data::Field<&'a str>,
            }
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::validated_data::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub missing_policy: ::validated_data::Field<item::MissingPolicy<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct MissingPolicy<'a, Mode> {
                pub direction_in_action: ::validated_data::Field<&'a str>,
                pub direction_out_action: ::validated_data::Field<&'a str>,
            }
        }
    }

    #[::validated_data::data_view]
    pub struct PathSelection<'a, Mode> {
        pub roles: ::validated_data::Field<path_selection::Roles<'a, Mode>>,
    }

    pub mod path_selection {

        #[::validated_data::data_view]
        pub struct Roles<'a, Mode> {
            pub producer: ::validated_data::Field<bool>,
            pub consumer: ::validated_data::Field<bool>,
            pub propagator: ::validated_data::Field<bool>,
        }
    }
}

#[::validated_data::data_view]
pub struct AddressFamilyFlowSpecIpv4<'a, Mode> {
    pub bgp: ::validated_data::Field<address_family_flow_spec_ipv4::Bgp<'a, Mode>>,
    pub neighbors: ::validated_data::Field<address_family_flow_spec_ipv4::Neighbors<'a, Mode>>,
    pub peer_groups: ::validated_data::Field<address_family_flow_spec_ipv4::PeerGroups<'a, Mode>>,
}

pub mod address_family_flow_spec_ipv4 {

    #[::validated_data::data_view]
    pub struct Bgp<'a, Mode> {
        pub missing_policy: ::validated_data::Field<bgp::MissingPolicy<'a, Mode>>,
    }

    pub mod bgp {

        #[::validated_data::data_view]
        pub struct MissingPolicy<'a, Mode> {
            pub direction_in_action: ::validated_data::Field<&'a str>,
            pub direction_out_action: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::validated_data::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::validated_data::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
        }
    }
}

#[::validated_data::data_view]
pub struct AddressFamilyFlowSpecIpv6<'a, Mode> {
    pub bgp: ::validated_data::Field<address_family_flow_spec_ipv6::Bgp<'a, Mode>>,
    pub neighbors: ::validated_data::Field<address_family_flow_spec_ipv6::Neighbors<'a, Mode>>,
    pub peer_groups: ::validated_data::Field<address_family_flow_spec_ipv6::PeerGroups<'a, Mode>>,
}

pub mod address_family_flow_spec_ipv6 {

    #[::validated_data::data_view]
    pub struct Bgp<'a, Mode> {
        pub missing_policy: ::validated_data::Field<bgp::MissingPolicy<'a, Mode>>,
    }

    pub mod bgp {

        #[::validated_data::data_view]
        pub struct MissingPolicy<'a, Mode> {
            pub direction_in_action: ::validated_data::Field<&'a str>,
            pub direction_out_action: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::validated_data::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::validated_data::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
        }
    }
}

#[::validated_data::data_view]
pub struct AddressFamilyPathSelection<'a, Mode> {
    pub bgp: ::validated_data::Field<address_family_path_selection::Bgp<'a, Mode>>,
    pub neighbors: ::validated_data::Field<address_family_path_selection::Neighbors<'a, Mode>>,
    pub peer_groups: ::validated_data::Field<address_family_path_selection::PeerGroups<'a, Mode>>,
}

pub mod address_family_path_selection {

    #[::validated_data::data_view]
    pub struct Bgp<'a, Mode> {
        pub additional_paths: ::validated_data::Field<bgp::AdditionalPaths<'a, Mode>>,
    }

    pub mod bgp {

        #[::validated_data::data_view]
        pub struct AdditionalPaths<'a, Mode> {
            pub receive: ::validated_data::Field<bool>,
            pub send: ::validated_data::Field<&'a str>,
            pub send_limit: ::validated_data::Field<i64>,
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::validated_data::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub additional_paths: ::validated_data::Field<item::AdditionalPaths<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub receive: ::validated_data::Field<bool>,
                pub send: ::validated_data::Field<&'a str>,
                pub send_limit: ::validated_data::Field<i64>,
            }
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::validated_data::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub additional_paths: ::validated_data::Field<item::AdditionalPaths<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub receive: ::validated_data::Field<bool>,
                pub send: ::validated_data::Field<&'a str>,
                pub send_limit: ::validated_data::Field<i64>,
            }
        }
    }
}

#[::validated_data::data_view]
pub struct AddressFamilyVpnIpv4<'a, Mode> {
    pub domain_identifier: ::validated_data::Field<&'a str>,
    pub peer_groups: ::validated_data::Field<address_family_vpn_ipv4::PeerGroups<'a, Mode>>,
    pub route: ::validated_data::Field<address_family_vpn_ipv4::Route<'a, Mode>>,
    pub neighbors: ::validated_data::Field<address_family_vpn_ipv4::Neighbors<'a, Mode>>,
    pub neighbor_default_encapsulation_mpls_next_hop_self: ::validated_data::Field<address_family_vpn_ipv4::NeighborDefaultEncapsulationMplsNextHopSelf<'a, Mode>>,
    pub next_hop: ::validated_data::Field<address_family_vpn_ipv4::NextHop<'a, Mode>>,
}

pub mod address_family_vpn_ipv4 {

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::validated_data::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub route_map_in: ::validated_data::Field<&'a str>,
            pub route_map_out: ::validated_data::Field<&'a str>,
            pub peer_tag_in: ::validated_data::Field<&'a str>,
            pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
            pub rcf_in: ::validated_data::Field<&'a str>,
            pub rcf_out: ::validated_data::Field<&'a str>,
            pub default_route: ::validated_data::Field<item::DefaultRoute<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct DefaultRoute<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub rcf: ::validated_data::Field<&'a str>,
                pub route_map: ::validated_data::Field<&'a str>,
            }
        }
    }

    #[::validated_data::data_view]
    pub struct Route<'a, Mode> {
        pub import_match_failure_action: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::validated_data::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub route_map_in: ::validated_data::Field<&'a str>,
            pub route_map_out: ::validated_data::Field<&'a str>,
            pub peer_tag_in: ::validated_data::Field<&'a str>,
            pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
            pub rcf_in: ::validated_data::Field<&'a str>,
            pub rcf_out: ::validated_data::Field<&'a str>,
            pub default_route: ::validated_data::Field<item::DefaultRoute<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct DefaultRoute<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub rcf: ::validated_data::Field<&'a str>,
                pub route_map: ::validated_data::Field<&'a str>,
            }
        }
    }

    #[::validated_data::data_view]
    pub struct NeighborDefaultEncapsulationMplsNextHopSelf<'a, Mode> {
        pub source_interface: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct NextHop<'a, Mode> {
        pub resolution_disabled: ::validated_data::Field<bool>,
    }
}

#[::validated_data::data_view]
pub struct AddressFamilyVpnIpv6<'a, Mode> {
    pub domain_identifier: ::validated_data::Field<&'a str>,
    pub peer_groups: ::validated_data::Field<address_family_vpn_ipv6::PeerGroups<'a, Mode>>,
    pub route: ::validated_data::Field<address_family_vpn_ipv6::Route<'a, Mode>>,
    pub neighbors: ::validated_data::Field<address_family_vpn_ipv6::Neighbors<'a, Mode>>,
    pub neighbor_default_encapsulation_mpls_next_hop_self: ::validated_data::Field<address_family_vpn_ipv6::NeighborDefaultEncapsulationMplsNextHopSelf<'a, Mode>>,
    pub next_hop: ::validated_data::Field<address_family_vpn_ipv6::NextHop<'a, Mode>>,
}

pub mod address_family_vpn_ipv6 {

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct PeerGroups<'a, Mode> (::validated_data::Field<peer_groups::Item<'a, Mode>>);

    pub mod peer_groups {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub route_map_in: ::validated_data::Field<&'a str>,
            pub route_map_out: ::validated_data::Field<&'a str>,
            pub peer_tag_in: ::validated_data::Field<&'a str>,
            pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
            pub rcf_in: ::validated_data::Field<&'a str>,
            pub rcf_out: ::validated_data::Field<&'a str>,
            pub default_route: ::validated_data::Field<item::DefaultRoute<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct DefaultRoute<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub rcf: ::validated_data::Field<&'a str>,
                pub route_map: ::validated_data::Field<&'a str>,
            }
        }
    }

    #[::validated_data::data_view]
    pub struct Route<'a, Mode> {
        pub import_match_failure_action: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
    pub struct Neighbors<'a, Mode> (::validated_data::Field<neighbors::Item<'a, Mode>>);

    pub mod neighbors {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::validated_data::Field<&'a str>,
            pub activate: ::validated_data::Field<bool>,
            pub route_map_in: ::validated_data::Field<&'a str>,
            pub route_map_out: ::validated_data::Field<&'a str>,
            pub peer_tag_in: ::validated_data::Field<&'a str>,
            pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
            pub rcf_in: ::validated_data::Field<&'a str>,
            pub rcf_out: ::validated_data::Field<&'a str>,
            pub default_route: ::validated_data::Field<item::DefaultRoute<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct DefaultRoute<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub rcf: ::validated_data::Field<&'a str>,
                pub route_map: ::validated_data::Field<&'a str>,
            }
        }
    }

    #[::validated_data::data_view]
    pub struct NeighborDefaultEncapsulationMplsNextHopSelf<'a, Mode> {
        pub source_interface: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct NextHop<'a, Mode> {
        pub resolution_disabled: ::validated_data::Field<bool>,
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub bgp: ::validated_data::Field<item::Bgp<'a, Mode>>,
        pub rd: ::validated_data::Field<&'a str>,
        pub rd_evpn_domain: ::validated_data::Field<item::RdEvpnDomain<'a, Mode>>,
        pub evpn_multicast: ::validated_data::Field<bool>,
        pub evpn_multicast_address_family: ::validated_data::Field<item::EvpnMulticastAddressFamily<'a, Mode>>,
        pub evpn_multicast_gateway_dr_election: ::validated_data::Field<item::EvpnMulticastGatewayDrElection<'a, Mode>>,
        pub default_route_exports: ::validated_data::Field<item::DefaultRouteExports<'a, Mode>>,
        pub route_targets: ::validated_data::Field<item::RouteTargets<'a, Mode>>,
        pub router_id: ::validated_data::Field<&'a str>,
        pub timers: ::validated_data::Field<&'a str>,
        pub graceful_restart: ::validated_data::Field<item::GracefulRestart<'a, Mode>>,
        pub no_graceful_restart: ::validated_data::Field<bool>,
        pub networks: ::validated_data::Field<item::Networks<'a, Mode>>,
        pub maximum_paths: ::validated_data::Field<item::MaximumPaths<'a, Mode>>,
        pub updates: ::validated_data::Field<item::Updates<'a, Mode>>,
        pub listen_ranges: ::validated_data::Field<item::ListenRanges<'a, Mode>>,
        pub neighbors: ::validated_data::Field<item::Neighbors<'a, Mode>>,
        pub neighbor_interfaces: ::validated_data::Field<item::NeighborInterfaces<'a, Mode>>,
        pub redistribute: ::validated_data::Field<item::Redistribute<'a, Mode>>,
        pub aggregate_addresses: ::validated_data::Field<item::AggregateAddresses<'a, Mode>>,
        pub address_family_ipv4: ::validated_data::Field<item::AddressFamilyIpv4<'a, Mode>>,
        pub address_family_ipv6: ::validated_data::Field<item::AddressFamilyIpv6<'a, Mode>>,
        pub address_family_ipv4_multicast: ::validated_data::Field<item::AddressFamilyIpv4Multicast<'a, Mode>>,
        pub address_family_ipv6_multicast: ::validated_data::Field<item::AddressFamilyIpv6Multicast<'a, Mode>>,
        pub address_family_flow_spec_ipv4: ::validated_data::Field<item::AddressFamilyFlowSpecIpv4<'a, Mode>>,
        pub address_family_flow_spec_ipv6: ::validated_data::Field<item::AddressFamilyFlowSpecIpv6<'a, Mode>>,
        pub eos_cli: ::validated_data::Field<&'a str>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct Bgp<'a, Mode> {
            pub redistribute_internal: ::validated_data::Field<bool>,
            pub additional_paths: ::validated_data::Field<bgp::AdditionalPaths<'a, Mode>>,
        }

        pub mod bgp {

            #[::validated_data::data_view]
            pub struct AdditionalPaths<'a, Mode> {
                pub install: ::validated_data::Field<bool>,
                pub install_ecmp_primary: ::validated_data::Field<bool>,
                pub receive: ::validated_data::Field<bool>,
                pub send: ::validated_data::Field<&'a str>,
                pub send_limit: ::validated_data::Field<i64>,
            }
        }

        #[::validated_data::data_view]
        pub struct RdEvpnDomain<'a, Mode> {
            pub domain: ::validated_data::RequiredValue<&'a str, Mode>,
            pub rd: ::validated_data::RequiredValue<&'a str, Mode>,
        }

        #[::validated_data::data_view]
        pub struct EvpnMulticastAddressFamily<'a, Mode> {
            pub ipv4: ::validated_data::Field<evpn_multicast_address_family::Ipv4<'a, Mode>>,
        }

        pub mod evpn_multicast_address_family {

            #[::validated_data::data_view]
            pub struct Ipv4<'a, Mode> {
                pub transit: ::validated_data::Field<bool>,
            }
        }

        #[::validated_data::data_view]
        pub struct EvpnMulticastGatewayDrElection<'a, Mode> {
            pub algorithm: ::validated_data::RequiredValue<&'a str, Mode>,
            pub preference_value: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view(indexed_list, primary_key(address_family))]
        pub struct DefaultRouteExports<'a, Mode> (::validated_data::Field<default_route_exports::Item<'a, Mode>>);

        pub mod default_route_exports {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub address_family: ::validated_data::Field<&'a str>,
                pub always: ::validated_data::Field<bool>,
                pub route_map: ::validated_data::Field<&'a str>,
                pub rcf: ::validated_data::Field<&'a str>,
            }
        }

        #[::validated_data::data_view]
        pub struct RouteTargets<'a, Mode> {
            #[data_view(rename = "import")]
            pub field_import: ::validated_data::Field<route_targets::Import<'a, Mode>>,
            pub export: ::validated_data::Field<route_targets::Export<'a, Mode>>,
            pub import_evpn_domains: ::validated_data::Field<route_targets::ImportEvpnDomains<'a, Mode>>,
            pub export_evpn_domains: ::validated_data::Field<route_targets::ExportEvpnDomains<'a, Mode>>,
        }

        pub mod route_targets {

            #[::validated_data::data_view(indexed_list, primary_key(address_family))]
            pub struct Import<'a, Mode> (::validated_data::Field<import::Item<'a, Mode>>);

            pub mod import {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub address_family: ::validated_data::Field<&'a str>,
                    pub route_targets: ::validated_data::Field<item::RouteTargets<'a, Mode>>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub rcf: ::validated_data::Field<&'a str>,
                    pub vpn_route_filter_rcf: ::validated_data::Field<&'a str>,
                }

                pub mod item {

                    #[::validated_data::data_view(list)]
                    pub struct RouteTargets<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }

            #[::validated_data::data_view(indexed_list, primary_key(address_family))]
            pub struct Export<'a, Mode> (::validated_data::Field<export::Item<'a, Mode>>);

            pub mod export {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub address_family: ::validated_data::Field<&'a str>,
                    pub route_targets: ::validated_data::Field<item::RouteTargets<'a, Mode>>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub rcf: ::validated_data::Field<&'a str>,
                    pub vrf_route_filter_rcf: ::validated_data::Field<&'a str>,
                }

                pub mod item {

                    #[::validated_data::data_view(list)]
                    pub struct RouteTargets<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }

            #[::validated_data::data_view(indexed_list, primary_key(route_target))]
            pub struct ImportEvpnDomains<'a, Mode> (::validated_data::Field<import_evpn_domains::Item<'a, Mode>>);

            pub mod import_evpn_domains {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub route_target: ::validated_data::Field<&'a str>,
                    pub domain: ::validated_data::RequiredValue<&'a str, Mode>,
                }
            }

            #[::validated_data::data_view(indexed_list, primary_key(route_target))]
            pub struct ExportEvpnDomains<'a, Mode> (::validated_data::Field<export_evpn_domains::Item<'a, Mode>>);

            pub mod export_evpn_domains {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub route_target: ::validated_data::Field<&'a str>,
                    pub domain: ::validated_data::RequiredValue<&'a str, Mode>,
                }
            }
        }

        #[::validated_data::data_view]
        pub struct GracefulRestart<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub restart_time: ::validated_data::Field<i64>,
            pub stalepath_time: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view(indexed_list, primary_key(prefix))]
        pub struct Networks<'a, Mode> (::validated_data::Field<networks::Item<'a, Mode>>);

        pub mod networks {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub prefix: ::validated_data::Field<&'a str>,
                pub route_map: ::validated_data::Field<&'a str>,
            }
        }

        #[::validated_data::data_view]
        pub struct MaximumPaths<'a, Mode> {
            pub paths: ::validated_data::RequiredValue<i64, Mode>,
            pub ecmp: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view]
        pub struct Updates<'a, Mode> {
            pub wait_for_convergence: ::validated_data::Field<bool>,
            pub wait_install: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view(list)]
        pub struct ListenRanges<'a, Mode> (::validated_data::Field<listen_ranges::Item<'a, Mode>>);

        pub mod listen_ranges {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub prefix: ::validated_data::Field<&'a str>,
                pub peer_id_include_router_id: ::validated_data::Field<bool>,
                pub peer_group: ::validated_data::Field<&'a str>,
                pub peer_filter: ::validated_data::Field<&'a str>,
                pub remote_as: ::validated_data::Field<&'a str>,
            }
        }

        #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
        pub struct Neighbors<'a, Mode> (::validated_data::Field<neighbors::Item<'a, Mode>>);

        pub mod neighbors {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub ip_address: ::validated_data::Field<&'a str>,
                pub peer_group: ::validated_data::Field<&'a str>,
                pub remote_as: ::validated_data::Field<&'a str>,
                pub password: ::validated_data::Field<&'a str>,
                pub password_type: ::validated_data::Field<&'a str>,
                pub passive: ::validated_data::Field<bool>,
                pub remove_private_as: ::validated_data::Field<item::RemovePrivateAs<'a, Mode>>,
                pub remove_private_as_ingress: ::validated_data::Field<item::RemovePrivateAsIngress<'a, Mode>>,
                pub weight: ::validated_data::Field<i64>,
                pub local_as: ::validated_data::Field<&'a str>,
                pub as_path: ::validated_data::Field<item::AsPath<'a, Mode>>,
                pub description: ::validated_data::Field<&'a str>,
                pub route_reflector_client: ::validated_data::Field<bool>,
                pub ebgp_multihop: ::validated_data::Field<i64>,
                pub next_hop_peer: ::validated_data::Field<bool>,
                pub next_hop_self: ::validated_data::Field<bool>,
                pub shutdown: ::validated_data::Field<bool>,
                pub bfd: ::validated_data::Field<bool>,
                pub bfd_timers: ::validated_data::Field<item::BfdTimers<'a, Mode>>,
                pub timers: ::validated_data::Field<&'a str>,
                pub rib_in_pre_policy_retain: ::validated_data::Field<item::RibInPrePolicyRetain<'a, Mode>>,
                pub send_community: ::validated_data::Field<&'a str>,
                pub maximum_routes: ::validated_data::Field<i64>,
                pub maximum_routes_warning_limit: ::validated_data::Field<&'a str>,
                pub maximum_routes_warning_only: ::validated_data::Field<bool>,
                pub maximum_accepted_routes: ::validated_data::Field<item::MaximumAcceptedRoutes<'a, Mode>>,
                pub allowas_in: ::validated_data::Field<item::AllowasIn<'a, Mode>>,
                pub default_originate: ::validated_data::Field<item::DefaultOriginate<'a, Mode>>,
                pub enforce_first_as: ::validated_data::Field<bool>,
                pub update_source: ::validated_data::Field<&'a str>,
                pub route_map_in: ::validated_data::Field<&'a str>,
                pub route_map_out: ::validated_data::Field<&'a str>,
                pub peer_tag_in: ::validated_data::Field<&'a str>,
                pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
                pub additional_paths: ::validated_data::Field<item::AdditionalPaths<'a, Mode>>,
                pub metadata: ::validated_data::Field<item::Metadata<'a, Mode>>,
            }

            pub mod item {

                #[::validated_data::data_view]
                pub struct RemovePrivateAs<'a, Mode> {
                    pub enabled: ::validated_data::Field<bool>,
                    pub all: ::validated_data::Field<bool>,
                    pub replace_as: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct RemovePrivateAsIngress<'a, Mode> {
                    pub enabled: ::validated_data::Field<bool>,
                    pub replace_as: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct AsPath<'a, Mode> {
                    pub remote_as_replace_out: ::validated_data::Field<bool>,
                    pub prepend_own_disabled: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct BfdTimers<'a, Mode> {
                    pub interval: ::validated_data::RequiredValue<i64, Mode>,
                    pub min_rx: ::validated_data::RequiredValue<i64, Mode>,
                    pub multiplier: ::validated_data::RequiredValue<i64, Mode>,
                }

                #[::validated_data::data_view]
                pub struct RibInPrePolicyRetain<'a, Mode> {
                    pub enabled: ::validated_data::Field<bool>,
                    pub all: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct MaximumAcceptedRoutes<'a, Mode> {
                    pub limit: ::validated_data::RequiredValue<i64, Mode>,
                    pub warning_limit: ::validated_data::Field<maximum_accepted_routes::WarningLimit<'a, Mode>>,
                }

                pub mod maximum_accepted_routes {

                    #[::validated_data::data_view]
                    pub struct WarningLimit<'a, Mode> {
                        pub count: ::validated_data::Field<i64>,
                        pub percent: ::validated_data::Field<i64>,
                    }
                }

                #[::validated_data::data_view]
                pub struct AllowasIn<'a, Mode> {
                    pub enabled: ::validated_data::Field<bool>,
                    pub times: ::validated_data::Field<i64>,
                }

                #[::validated_data::data_view]
                pub struct DefaultOriginate<'a, Mode> {
                    pub enabled: ::validated_data::Field<bool>,
                    pub always: ::validated_data::Field<bool>,
                    pub route_map: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view]
                pub struct AdditionalPaths<'a, Mode> {
                    pub receive: ::validated_data::Field<bool>,
                    pub send: ::validated_data::Field<&'a str>,
                    pub send_limit: ::validated_data::Field<i64>,
                }

                #[::validated_data::data_view]
                pub struct Metadata<'a, Mode> {
                    pub validate_state: ::validated_data::Field<bool>,
                }
            }
        }

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct NeighborInterfaces<'a, Mode> (::validated_data::Field<neighbor_interfaces::Item<'a, Mode>>);

        pub mod neighbor_interfaces {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub remote_as: ::validated_data::Field<&'a str>,
                pub peer_group: ::validated_data::Field<&'a str>,
                pub peer_filter: ::validated_data::Field<&'a str>,
                pub description: ::validated_data::Field<&'a str>,
                pub metadata: ::validated_data::Field<item::Metadata<'a, Mode>>,
            }

            pub mod item {

                #[::validated_data::data_view]
                pub struct Metadata<'a, Mode> {
                    pub validate_state: ::validated_data::Field<bool>,
                }
            }
        }

        #[::validated_data::data_view]
        pub struct Redistribute<'a, Mode> {
            pub attached_host: ::validated_data::Field<redistribute::AttachedHost<'a, Mode>>,
            pub bgp: ::validated_data::Field<redistribute::Bgp<'a, Mode>>,
            pub connected: ::validated_data::Field<redistribute::Connected<'a, Mode>>,
            pub dynamic: ::validated_data::Field<redistribute::Dynamic<'a, Mode>>,
            pub isis: ::validated_data::Field<redistribute::Isis<'a, Mode>>,
            pub ospf: ::validated_data::Field<redistribute::Ospf<'a, Mode>>,
            pub ospfv3: ::validated_data::Field<redistribute::Ospfv3<'a, Mode>>,
            pub rip: ::validated_data::Field<redistribute::Rip<'a, Mode>>,
            #[data_view(rename = "static")]
            pub field_static: ::validated_data::Field<redistribute::FieldStatic<'a, Mode>>,
            pub user: ::validated_data::Field<redistribute::User<'a, Mode>>,
        }

        pub mod redistribute {

            #[::validated_data::data_view]
            pub struct AttachedHost<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct Bgp<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct Connected<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
                pub rcf: ::validated_data::Field<&'a str>,
                pub include_leaked: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view]
            pub struct Dynamic<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
                pub rcf: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct Isis<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub isis_level: ::validated_data::Field<&'a str>,
                pub route_map: ::validated_data::Field<&'a str>,
                pub rcf: ::validated_data::Field<&'a str>,
                pub include_leaked: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view]
            pub struct Ospf<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub match_external: ::validated_data::Field<ospf::MatchExternal<'a, Mode>>,
                pub match_internal: ::validated_data::Field<ospf::MatchInternal<'a, Mode>>,
                pub match_nssa_external: ::validated_data::Field<ospf::MatchNssaExternal<'a, Mode>>,
                pub route_map: ::validated_data::Field<&'a str>,
                pub include_leaked: ::validated_data::Field<bool>,
            }

            pub mod ospf {

                #[::validated_data::data_view]
                pub struct MatchExternal<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct MatchInternal<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct MatchNssaExternal<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub nssa_type: ::validated_data::Field<i64>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }
            }

            #[::validated_data::data_view]
            pub struct Ospfv3<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub match_external: ::validated_data::Field<ospfv3::MatchExternal<'a, Mode>>,
                pub match_internal: ::validated_data::Field<ospfv3::MatchInternal<'a, Mode>>,
                pub match_nssa_external: ::validated_data::Field<ospfv3::MatchNssaExternal<'a, Mode>>,
                pub route_map: ::validated_data::Field<&'a str>,
                pub include_leaked: ::validated_data::Field<bool>,
            }

            pub mod ospfv3 {

                #[::validated_data::data_view]
                pub struct MatchExternal<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct MatchInternal<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct MatchNssaExternal<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub nssa_type: ::validated_data::Field<i64>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }
            }

            #[::validated_data::data_view]
            pub struct Rip<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct FieldStatic<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
                pub rcf: ::validated_data::Field<&'a str>,
                pub include_leaked: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view]
            pub struct User<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub rcf: ::validated_data::Field<&'a str>,
            }
        }

        #[::validated_data::data_view(indexed_list, primary_key(prefix))]
        pub struct AggregateAddresses<'a, Mode> (::validated_data::Field<aggregate_addresses::Item<'a, Mode>>);

        pub mod aggregate_addresses {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub prefix: ::validated_data::Field<&'a str>,
                pub advertise_only: ::validated_data::Field<bool>,
                pub as_set: ::validated_data::Field<bool>,
                pub summary_only: ::validated_data::Field<bool>,
                pub attribute_map: ::validated_data::Field<&'a str>,
                pub match_map: ::validated_data::Field<&'a str>,
                pub attribute: ::validated_data::Field<item::Attribute<'a, Mode>>,
            }

            pub mod item {

                #[::validated_data::data_view]
                pub struct Attribute<'a, Mode> {
                    pub rcf: ::validated_data::Field<&'a str>,
                }
            }
        }

        #[::validated_data::data_view]
        pub struct AddressFamilyIpv4<'a, Mode> {
            pub bgp: ::validated_data::Field<address_family_ipv4::Bgp<'a, Mode>>,
            pub neighbors: ::validated_data::Field<address_family_ipv4::Neighbors<'a, Mode>>,
            pub networks: ::validated_data::Field<address_family_ipv4::Networks<'a, Mode>>,
            pub redistribute: ::validated_data::Field<address_family_ipv4::Redistribute<'a, Mode>>,
        }

        pub mod address_family_ipv4 {

            #[::validated_data::data_view]
            pub struct Bgp<'a, Mode> {
                pub missing_policy: ::validated_data::Field<bgp::MissingPolicy<'a, Mode>>,
                pub additional_paths: ::validated_data::Field<bgp::AdditionalPaths<'a, Mode>>,
                pub redistribute_internal: ::validated_data::Field<bool>,
            }

            pub mod bgp {

                #[::validated_data::data_view]
                pub struct MissingPolicy<'a, Mode> {
                    pub direction_in_action: ::validated_data::Field<&'a str>,
                    pub direction_out_action: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view]
                pub struct AdditionalPaths<'a, Mode> {
                    pub install: ::validated_data::Field<bool>,
                    pub install_ecmp_primary: ::validated_data::Field<bool>,
                    pub receive: ::validated_data::Field<bool>,
                    pub send: ::validated_data::Field<&'a str>,
                    pub send_limit: ::validated_data::Field<i64>,
                }
            }

            #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
            pub struct Neighbors<'a, Mode> (::validated_data::Field<neighbors::Item<'a, Mode>>);

            pub mod neighbors {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub ip_address: ::validated_data::Field<&'a str>,
                    pub activate: ::validated_data::Field<bool>,
                    pub route_map_in: ::validated_data::Field<&'a str>,
                    pub route_map_out: ::validated_data::Field<&'a str>,
                    pub peer_tag_in: ::validated_data::Field<&'a str>,
                    pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
                    pub rcf_in: ::validated_data::Field<&'a str>,
                    pub rcf_out: ::validated_data::Field<&'a str>,
                    pub prefix_list_in: ::validated_data::Field<&'a str>,
                    pub prefix_list_out: ::validated_data::Field<&'a str>,
                    pub next_hop: ::validated_data::Field<item::NextHop<'a, Mode>>,
                    pub additional_paths: ::validated_data::Field<item::AdditionalPaths<'a, Mode>>,
                }

                pub mod item {

                    #[::validated_data::data_view]
                    pub struct NextHop<'a, Mode> {
                        pub address_family_ipv6: ::validated_data::Field<next_hop::AddressFamilyIpv6<'a, Mode>>,
                    }

                    pub mod next_hop {

                        #[::validated_data::data_view]
                        pub struct AddressFamilyIpv6<'a, Mode> {
                            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                            pub originate: ::validated_data::Field<bool>,
                        }
                    }

                    #[::validated_data::data_view]
                    pub struct AdditionalPaths<'a, Mode> {
                        pub receive: ::validated_data::Field<bool>,
                        pub send: ::validated_data::Field<&'a str>,
                        pub send_limit: ::validated_data::Field<i64>,
                    }
                }
            }

            #[::validated_data::data_view(indexed_list, primary_key(prefix))]
            pub struct Networks<'a, Mode> (::validated_data::Field<networks::Item<'a, Mode>>);

            pub mod networks {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub prefix: ::validated_data::Field<&'a str>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub rcf: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view]
            pub struct Redistribute<'a, Mode> {
                pub attached_host: ::validated_data::Field<redistribute::AttachedHost<'a, Mode>>,
                pub bgp: ::validated_data::Field<redistribute::Bgp<'a, Mode>>,
                pub connected: ::validated_data::Field<redistribute::Connected<'a, Mode>>,
                pub dynamic: ::validated_data::Field<redistribute::Dynamic<'a, Mode>>,
                pub isis: ::validated_data::Field<redistribute::Isis<'a, Mode>>,
                pub ospf: ::validated_data::Field<redistribute::Ospf<'a, Mode>>,
                pub ospfv3: ::validated_data::Field<redistribute::Ospfv3<'a, Mode>>,
                pub rip: ::validated_data::Field<redistribute::Rip<'a, Mode>>,
                #[data_view(rename = "static")]
                pub field_static: ::validated_data::Field<redistribute::FieldStatic<'a, Mode>>,
                pub user: ::validated_data::Field<redistribute::User<'a, Mode>>,
            }

            pub mod redistribute {

                #[::validated_data::data_view]
                pub struct AttachedHost<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view]
                pub struct Bgp<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view]
                pub struct Connected<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub rcf: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct Dynamic<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub rcf: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view]
                pub struct Isis<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub isis_level: ::validated_data::Field<&'a str>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub rcf: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct Ospf<'a, Mode> {
                    pub enabled: ::validated_data::Field<bool>,
                    pub match_external: ::validated_data::Field<ospf::MatchExternal<'a, Mode>>,
                    pub match_internal: ::validated_data::Field<ospf::MatchInternal<'a, Mode>>,
                    pub match_nssa_external: ::validated_data::Field<ospf::MatchNssaExternal<'a, Mode>>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                pub mod ospf {

                    #[::validated_data::data_view]
                    pub struct MatchExternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub route_map: ::validated_data::Field<&'a str>,
                        pub include_leaked: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view]
                    pub struct MatchInternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub route_map: ::validated_data::Field<&'a str>,
                        pub include_leaked: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view]
                    pub struct MatchNssaExternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub nssa_type: ::validated_data::Field<i64>,
                        pub route_map: ::validated_data::Field<&'a str>,
                        pub include_leaked: ::validated_data::Field<bool>,
                    }
                }

                #[::validated_data::data_view]
                pub struct Ospfv3<'a, Mode> {
                    pub enabled: ::validated_data::Field<bool>,
                    pub match_external: ::validated_data::Field<ospfv3::MatchExternal<'a, Mode>>,
                    pub match_internal: ::validated_data::Field<ospfv3::MatchInternal<'a, Mode>>,
                    pub match_nssa_external: ::validated_data::Field<ospfv3::MatchNssaExternal<'a, Mode>>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                pub mod ospfv3 {

                    #[::validated_data::data_view]
                    pub struct MatchExternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub route_map: ::validated_data::Field<&'a str>,
                        pub include_leaked: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view]
                    pub struct MatchInternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub route_map: ::validated_data::Field<&'a str>,
                        pub include_leaked: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view]
                    pub struct MatchNssaExternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub nssa_type: ::validated_data::Field<i64>,
                        pub route_map: ::validated_data::Field<&'a str>,
                        pub include_leaked: ::validated_data::Field<bool>,
                    }
                }

                #[::validated_data::data_view]
                pub struct Rip<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view]
                pub struct FieldStatic<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub rcf: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct User<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub rcf: ::validated_data::Field<&'a str>,
                }
            }
        }

        #[::validated_data::data_view]
        pub struct AddressFamilyIpv6<'a, Mode> {
            pub bgp: ::validated_data::Field<address_family_ipv6::Bgp<'a, Mode>>,
            pub neighbors: ::validated_data::Field<address_family_ipv6::Neighbors<'a, Mode>>,
            pub networks: ::validated_data::Field<address_family_ipv6::Networks<'a, Mode>>,
            pub redistribute: ::validated_data::Field<address_family_ipv6::Redistribute<'a, Mode>>,
        }

        pub mod address_family_ipv6 {

            #[::validated_data::data_view]
            pub struct Bgp<'a, Mode> {
                pub missing_policy: ::validated_data::Field<bgp::MissingPolicy<'a, Mode>>,
                pub additional_paths: ::validated_data::Field<bgp::AdditionalPaths<'a, Mode>>,
                pub redistribute_internal: ::validated_data::Field<bool>,
            }

            pub mod bgp {

                #[::validated_data::data_view]
                pub struct MissingPolicy<'a, Mode> {
                    pub direction_in_action: ::validated_data::Field<&'a str>,
                    pub direction_out_action: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view]
                pub struct AdditionalPaths<'a, Mode> {
                    pub install: ::validated_data::Field<bool>,
                    pub install_ecmp_primary: ::validated_data::Field<bool>,
                    pub receive: ::validated_data::Field<bool>,
                    pub send: ::validated_data::Field<&'a str>,
                    pub send_limit: ::validated_data::Field<i64>,
                }
            }

            #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
            pub struct Neighbors<'a, Mode> (::validated_data::Field<neighbors::Item<'a, Mode>>);

            pub mod neighbors {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub ip_address: ::validated_data::Field<&'a str>,
                    pub activate: ::validated_data::Field<bool>,
                    pub route_map_in: ::validated_data::Field<&'a str>,
                    pub route_map_out: ::validated_data::Field<&'a str>,
                    pub peer_tag_in: ::validated_data::Field<&'a str>,
                    pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
                    pub rcf_in: ::validated_data::Field<&'a str>,
                    pub rcf_out: ::validated_data::Field<&'a str>,
                    pub prefix_list_in: ::validated_data::Field<&'a str>,
                    pub prefix_list_out: ::validated_data::Field<&'a str>,
                    pub additional_paths: ::validated_data::Field<item::AdditionalPaths<'a, Mode>>,
                }

                pub mod item {

                    #[::validated_data::data_view]
                    pub struct AdditionalPaths<'a, Mode> {
                        pub receive: ::validated_data::Field<bool>,
                        pub send: ::validated_data::Field<&'a str>,
                        pub send_limit: ::validated_data::Field<i64>,
                    }
                }
            }

            #[::validated_data::data_view(indexed_list, primary_key(prefix))]
            pub struct Networks<'a, Mode> (::validated_data::Field<networks::Item<'a, Mode>>);

            pub mod networks {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub prefix: ::validated_data::Field<&'a str>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub rcf: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view]
            pub struct Redistribute<'a, Mode> {
                pub attached_host: ::validated_data::Field<redistribute::AttachedHost<'a, Mode>>,
                pub bgp: ::validated_data::Field<redistribute::Bgp<'a, Mode>>,
                pub connected: ::validated_data::Field<redistribute::Connected<'a, Mode>>,
                pub dhcp: ::validated_data::Field<redistribute::Dhcp<'a, Mode>>,
                pub dynamic: ::validated_data::Field<redistribute::Dynamic<'a, Mode>>,
                pub isis: ::validated_data::Field<redistribute::Isis<'a, Mode>>,
                pub ospfv3: ::validated_data::Field<redistribute::Ospfv3<'a, Mode>>,
                #[data_view(rename = "static")]
                pub field_static: ::validated_data::Field<redistribute::FieldStatic<'a, Mode>>,
                pub user: ::validated_data::Field<redistribute::User<'a, Mode>>,
            }

            pub mod redistribute {

                #[::validated_data::data_view]
                pub struct AttachedHost<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view]
                pub struct Bgp<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view]
                pub struct Connected<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub rcf: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct Dhcp<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view]
                pub struct Dynamic<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub rcf: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view]
                pub struct Isis<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub isis_level: ::validated_data::Field<&'a str>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub rcf: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct Ospfv3<'a, Mode> {
                    pub enabled: ::validated_data::Field<bool>,
                    pub match_external: ::validated_data::Field<ospfv3::MatchExternal<'a, Mode>>,
                    pub match_internal: ::validated_data::Field<ospfv3::MatchInternal<'a, Mode>>,
                    pub match_nssa_external: ::validated_data::Field<ospfv3::MatchNssaExternal<'a, Mode>>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                pub mod ospfv3 {

                    #[::validated_data::data_view]
                    pub struct MatchExternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub route_map: ::validated_data::Field<&'a str>,
                        pub include_leaked: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view]
                    pub struct MatchInternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub route_map: ::validated_data::Field<&'a str>,
                        pub include_leaked: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view]
                    pub struct MatchNssaExternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub nssa_type: ::validated_data::Field<i64>,
                        pub route_map: ::validated_data::Field<&'a str>,
                        pub include_leaked: ::validated_data::Field<bool>,
                    }
                }

                #[::validated_data::data_view]
                pub struct FieldStatic<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub rcf: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct User<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub rcf: ::validated_data::Field<&'a str>,
                }
            }
        }

        #[::validated_data::data_view]
        pub struct AddressFamilyIpv4Multicast<'a, Mode> {
            pub bgp: ::validated_data::Field<address_family_ipv4_multicast::Bgp<'a, Mode>>,
            pub neighbors: ::validated_data::Field<address_family_ipv4_multicast::Neighbors<'a, Mode>>,
            pub networks: ::validated_data::Field<address_family_ipv4_multicast::Networks<'a, Mode>>,
            pub redistribute: ::validated_data::Field<address_family_ipv4_multicast::Redistribute<'a, Mode>>,
        }

        pub mod address_family_ipv4_multicast {

            #[::validated_data::data_view]
            pub struct Bgp<'a, Mode> {
                pub missing_policy: ::validated_data::Field<bgp::MissingPolicy<'a, Mode>>,
                pub additional_paths: ::validated_data::Field<bgp::AdditionalPaths<'a, Mode>>,
            }

            pub mod bgp {

                #[::validated_data::data_view]
                pub struct MissingPolicy<'a, Mode> {
                    pub direction_in_action: ::validated_data::Field<&'a str>,
                    pub direction_out_action: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view]
                pub struct AdditionalPaths<'a, Mode> {
                    pub receive: ::validated_data::Field<bool>,
                }
            }

            #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
            pub struct Neighbors<'a, Mode> (::validated_data::Field<neighbors::Item<'a, Mode>>);

            pub mod neighbors {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub ip_address: ::validated_data::Field<&'a str>,
                    pub activate: ::validated_data::Field<bool>,
                    pub route_map_in: ::validated_data::Field<&'a str>,
                    pub route_map_out: ::validated_data::Field<&'a str>,
                    pub peer_tag_in: ::validated_data::Field<&'a str>,
                    pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
                    pub additional_paths: ::validated_data::Field<item::AdditionalPaths<'a, Mode>>,
                }

                pub mod item {

                    #[::validated_data::data_view]
                    pub struct AdditionalPaths<'a, Mode> {
                        pub receive: ::validated_data::Field<bool>,
                    }
                }
            }

            #[::validated_data::data_view(indexed_list, primary_key(prefix))]
            pub struct Networks<'a, Mode> (::validated_data::Field<networks::Item<'a, Mode>>);

            pub mod networks {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub prefix: ::validated_data::Field<&'a str>,
                    pub route_map: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view]
            pub struct Redistribute<'a, Mode> {
                pub attached_host: ::validated_data::Field<redistribute::AttachedHost<'a, Mode>>,
                pub connected: ::validated_data::Field<redistribute::Connected<'a, Mode>>,
                pub isis: ::validated_data::Field<redistribute::Isis<'a, Mode>>,
                pub ospf: ::validated_data::Field<redistribute::Ospf<'a, Mode>>,
                pub ospfv3: ::validated_data::Field<redistribute::Ospfv3<'a, Mode>>,
                #[data_view(rename = "static")]
                pub field_static: ::validated_data::Field<redistribute::FieldStatic<'a, Mode>>,
            }

            pub mod redistribute {

                #[::validated_data::data_view]
                pub struct AttachedHost<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view]
                pub struct Connected<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view]
                pub struct Isis<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub isis_level: ::validated_data::Field<&'a str>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub rcf: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct Ospf<'a, Mode> {
                    pub enabled: ::validated_data::Field<bool>,
                    pub match_external: ::validated_data::Field<ospf::MatchExternal<'a, Mode>>,
                    pub match_internal: ::validated_data::Field<ospf::MatchInternal<'a, Mode>>,
                    pub match_nssa_external: ::validated_data::Field<ospf::MatchNssaExternal<'a, Mode>>,
                    pub route_map: ::validated_data::Field<&'a str>,
                }

                pub mod ospf {

                    #[::validated_data::data_view]
                    pub struct MatchExternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub route_map: ::validated_data::Field<&'a str>,
                    }

                    #[::validated_data::data_view]
                    pub struct MatchInternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub route_map: ::validated_data::Field<&'a str>,
                    }

                    #[::validated_data::data_view]
                    pub struct MatchNssaExternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub nssa_type: ::validated_data::Field<i64>,
                        pub route_map: ::validated_data::Field<&'a str>,
                    }
                }

                #[::validated_data::data_view]
                pub struct Ospfv3<'a, Mode> {
                    pub enabled: ::validated_data::Field<bool>,
                    pub match_external: ::validated_data::Field<ospfv3::MatchExternal<'a, Mode>>,
                    pub match_internal: ::validated_data::Field<ospfv3::MatchInternal<'a, Mode>>,
                    pub match_nssa_external: ::validated_data::Field<ospfv3::MatchNssaExternal<'a, Mode>>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                pub mod ospfv3 {

                    #[::validated_data::data_view]
                    pub struct MatchExternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub route_map: ::validated_data::Field<&'a str>,
                        pub include_leaked: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view]
                    pub struct MatchInternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub route_map: ::validated_data::Field<&'a str>,
                        pub include_leaked: ::validated_data::Field<bool>,
                    }

                    #[::validated_data::data_view]
                    pub struct MatchNssaExternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub nssa_type: ::validated_data::Field<i64>,
                        pub route_map: ::validated_data::Field<&'a str>,
                        pub include_leaked: ::validated_data::Field<bool>,
                    }
                }

                #[::validated_data::data_view]
                pub struct FieldStatic<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                }
            }
        }

        #[::validated_data::data_view]
        pub struct AddressFamilyIpv6Multicast<'a, Mode> {
            pub bgp: ::validated_data::Field<address_family_ipv6_multicast::Bgp<'a, Mode>>,
            pub neighbors: ::validated_data::Field<address_family_ipv6_multicast::Neighbors<'a, Mode>>,
            pub networks: ::validated_data::Field<address_family_ipv6_multicast::Networks<'a, Mode>>,
            pub redistribute: ::validated_data::Field<address_family_ipv6_multicast::Redistribute<'a, Mode>>,
        }

        pub mod address_family_ipv6_multicast {

            #[::validated_data::data_view]
            pub struct Bgp<'a, Mode> {
                pub missing_policy: ::validated_data::Field<bgp::MissingPolicy<'a, Mode>>,
                pub additional_paths: ::validated_data::Field<bgp::AdditionalPaths<'a, Mode>>,
            }

            pub mod bgp {

                #[::validated_data::data_view]
                pub struct MissingPolicy<'a, Mode> {
                    pub direction_in_action: ::validated_data::Field<&'a str>,
                    pub direction_out_action: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view]
                pub struct AdditionalPaths<'a, Mode> {
                    pub receive: ::validated_data::Field<bool>,
                }
            }

            #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
            pub struct Neighbors<'a, Mode> (::validated_data::Field<neighbors::Item<'a, Mode>>);

            pub mod neighbors {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub ip_address: ::validated_data::Field<&'a str>,
                    pub activate: ::validated_data::Field<bool>,
                    pub route_map_in: ::validated_data::Field<&'a str>,
                    pub route_map_out: ::validated_data::Field<&'a str>,
                    pub peer_tag_in: ::validated_data::Field<&'a str>,
                    pub peer_tag_out_discard: ::validated_data::Field<&'a str>,
                    pub additional_paths: ::validated_data::Field<item::AdditionalPaths<'a, Mode>>,
                }

                pub mod item {

                    #[::validated_data::data_view]
                    pub struct AdditionalPaths<'a, Mode> {
                        pub receive: ::validated_data::Field<bool>,
                    }
                }
            }

            #[::validated_data::data_view(indexed_list, primary_key(prefix))]
            pub struct Networks<'a, Mode> (::validated_data::Field<networks::Item<'a, Mode>>);

            pub mod networks {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub prefix: ::validated_data::Field<&'a str>,
                    pub route_map: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view]
            pub struct Redistribute<'a, Mode> {
                pub connected: ::validated_data::Field<redistribute::Connected<'a, Mode>>,
                pub isis: ::validated_data::Field<redistribute::Isis<'a, Mode>>,
                pub ospf: ::validated_data::Field<redistribute::Ospf<'a, Mode>>,
                pub ospfv3: ::validated_data::Field<redistribute::Ospfv3<'a, Mode>>,
                #[data_view(rename = "static")]
                pub field_static: ::validated_data::Field<redistribute::FieldStatic<'a, Mode>>,
            }

            pub mod redistribute {

                #[::validated_data::data_view]
                pub struct Connected<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view]
                pub struct Isis<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub isis_level: ::validated_data::Field<&'a str>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub rcf: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct Ospf<'a, Mode> {
                    pub enabled: ::validated_data::Field<bool>,
                    pub match_external: ::validated_data::Field<ospf::MatchExternal<'a, Mode>>,
                    pub match_internal: ::validated_data::Field<ospf::MatchInternal<'a, Mode>>,
                    pub match_nssa_external: ::validated_data::Field<ospf::MatchNssaExternal<'a, Mode>>,
                    pub route_map: ::validated_data::Field<&'a str>,
                }

                pub mod ospf {

                    #[::validated_data::data_view]
                    pub struct MatchExternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub route_map: ::validated_data::Field<&'a str>,
                    }

                    #[::validated_data::data_view]
                    pub struct MatchInternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub route_map: ::validated_data::Field<&'a str>,
                    }

                    #[::validated_data::data_view]
                    pub struct MatchNssaExternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub nssa_type: ::validated_data::Field<i64>,
                        pub route_map: ::validated_data::Field<&'a str>,
                    }
                }

                #[::validated_data::data_view]
                pub struct Ospfv3<'a, Mode> {
                    pub enabled: ::validated_data::Field<bool>,
                    pub match_external: ::validated_data::Field<ospfv3::MatchExternal<'a, Mode>>,
                    pub match_internal: ::validated_data::Field<ospfv3::MatchInternal<'a, Mode>>,
                    pub match_nssa_external: ::validated_data::Field<ospfv3::MatchNssaExternal<'a, Mode>>,
                    pub route_map: ::validated_data::Field<&'a str>,
                }

                pub mod ospfv3 {

                    #[::validated_data::data_view]
                    pub struct MatchExternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub route_map: ::validated_data::Field<&'a str>,
                    }

                    #[::validated_data::data_view]
                    pub struct MatchInternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub route_map: ::validated_data::Field<&'a str>,
                    }

                    #[::validated_data::data_view]
                    pub struct MatchNssaExternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub nssa_type: ::validated_data::Field<i64>,
                        pub route_map: ::validated_data::Field<&'a str>,
                    }
                }

                #[::validated_data::data_view]
                pub struct FieldStatic<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                }
            }
        }

        #[::validated_data::data_view]
        pub struct AddressFamilyFlowSpecIpv4<'a, Mode> {
            pub bgp: ::validated_data::Field<address_family_flow_spec_ipv4::Bgp<'a, Mode>>,
            pub neighbors: ::validated_data::Field<address_family_flow_spec_ipv4::Neighbors<'a, Mode>>,
        }

        pub mod address_family_flow_spec_ipv4 {

            #[::validated_data::data_view]
            pub struct Bgp<'a, Mode> {
                pub missing_policy: ::validated_data::Field<bgp::MissingPolicy<'a, Mode>>,
            }

            pub mod bgp {

                #[::validated_data::data_view]
                pub struct MissingPolicy<'a, Mode> {
                    pub direction_in_action: ::validated_data::Field<&'a str>,
                    pub direction_out_action: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
            pub struct Neighbors<'a, Mode> (::validated_data::Field<neighbors::Item<'a, Mode>>);

            pub mod neighbors {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub ip_address: ::validated_data::Field<&'a str>,
                    pub activate: ::validated_data::Field<bool>,
                }
            }
        }

        #[::validated_data::data_view]
        pub struct AddressFamilyFlowSpecIpv6<'a, Mode> {
            pub bgp: ::validated_data::Field<address_family_flow_spec_ipv6::Bgp<'a, Mode>>,
            pub neighbors: ::validated_data::Field<address_family_flow_spec_ipv6::Neighbors<'a, Mode>>,
        }

        pub mod address_family_flow_spec_ipv6 {

            #[::validated_data::data_view]
            pub struct Bgp<'a, Mode> {
                pub missing_policy: ::validated_data::Field<bgp::MissingPolicy<'a, Mode>>,
            }

            pub mod bgp {

                #[::validated_data::data_view]
                pub struct MissingPolicy<'a, Mode> {
                    pub direction_in_action: ::validated_data::Field<&'a str>,
                    pub direction_out_action: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
            pub struct Neighbors<'a, Mode> (::validated_data::Field<neighbors::Item<'a, Mode>>);

            pub mod neighbors {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub ip_address: ::validated_data::Field<&'a str>,
                    pub activate: ::validated_data::Field<bool>,
                }
            }
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct SessionTrackers<'a, Mode> (::validated_data::Field<session_trackers::Item<'a, Mode>>);

pub mod session_trackers {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub recovery_delay: ::validated_data::Field<i64>,
    }
}
