// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Timers {
        scalar keepalive_time("keepalive_time", 0) -> i64;
        scalar hold_time("hold_time", 1) -> i64;
        scalar min_hold_time("min_hold_time", 2) -> i64;
        scalar send_failure_hold_time("send_failure_hold_time", 3) -> i64;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Distance {
        scalar external_routes("external_routes", 0) -> i64;
        scalar internal_routes("internal_routes", 1) -> i64;
        scalar local_routes("local_routes", 2) -> i64;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct GracefulRestart {
        scalar enabled("enabled", 0) -> bool;
        scalar restart_time("restart_time", 1) -> i64;
        scalar stalepath_time("stalepath_time", 2) -> i64;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct GracefulRestartHelper {
        scalar enabled("enabled", 0) -> bool;
        scalar restart_time("restart_time", 1) -> i64;
        scalar long_lived("long_lived", 2) -> bool;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MaximumPaths {
        scalar paths("paths", 0) -> i64;
        scalar ecmp("ecmp", 1) -> i64;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RouteDistinguisher {
        model assignment_auto("assignment_auto", 0) -> route_distinguisher::AssignmentAuto<'a>;
    }
}

pub mod route_distinguisher {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct AssignmentAuto {
            model range("range", 0) -> assignment_auto::Range<'a>;
            model address_families("address_families", 1) -> assignment_auto::AddressFamilies<'a>;
        }
    }

    pub mod assignment_auto {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Range {
                scalar start("start", 0) -> i64;
                scalar end("end", 1) -> i64;
            }
        }

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AddressFamilies {
                scalar item (0) -> &'a str;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Updates {
        scalar wait_for_convergence("wait_for_convergence", 0) -> bool;
        scalar wait_install("wait_install", 1) -> bool;
    }
}

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct BgpDefaults {
        scalar item (0) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Bgp {
        model convergence("convergence", 0) -> bgp::Convergence<'a>;
        model default("default", 1) -> bgp::Default<'a>;
        model route_reflector_preserve_attributes("route_reflector_preserve_attributes", 2) -> bgp::RouteReflectorPreserveAttributes<'a>;
        model bestpath("bestpath", 3) -> bgp::Bestpath<'a>;
        model additional_paths("additional_paths", 4) -> bgp::AdditionalPaths<'a>;
        scalar redistribute_internal("redistribute_internal", 5) -> bool;
        model labeled_unicast("labeled_unicast", 6) -> bgp::LabeledUnicast<'a>;
    }
}

pub mod bgp {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Convergence {
            scalar slow_peer_time("slow_peer_time", 0) -> i64;
            scalar time("time", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Default {
            scalar ipv4_unicast("ipv4_unicast", 0) -> bool;
            scalar ipv4_unicast_transport_ipv6("ipv4_unicast_transport_ipv6", 1) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct RouteReflectorPreserveAttributes {
            scalar enabled("enabled", 0) -> bool;
            scalar always("always", 1) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Bestpath {
            scalar d_path("d_path", 0) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct AdditionalPaths {
            scalar receive("receive", 0) -> bool;
            scalar send("send", 1) -> &'a str;
            scalar send_limit("send_limit", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct LabeledUnicast {
            model rib("rib", 0) -> labeled_unicast::Rib<'a>;
        }
    }

    pub mod labeled_unicast {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Rib {
                model ip("ip", 0) -> rib::Ip<'a>;
                model tunnel("tunnel", 1) -> rib::Tunnel<'a>;
            }
        }

        pub mod rib {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Ip {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Tunnel {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                }
            }
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
            scalar peer_id_include_router_id("peer_id_include_router_id", 1) -> bool;
            scalar peer_group("peer_group", 2) -> &'a str;
            scalar peer_filter("peer_filter", 3) -> &'a str;
            scalar remote_as("remote_as", 4) -> &'a str;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct NeighborDefault {
        scalar send_community("send_community", 0) -> &'a str;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct PeerGroups {
        model item (0) -> peer_groups::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod peer_groups {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model metadata("metadata", 1) -> item::Metadata<'a>;
            scalar remote_as("remote_as", 2) -> &'a str;
            scalar local_as("local_as", 3) -> &'a str;
            scalar description("description", 4) -> &'a str;
            scalar shutdown("shutdown", 5) -> bool;
            model as_path("as_path", 6) -> item::AsPath<'a>;
            model remove_private_as("remove_private_as", 7) -> item::RemovePrivateAs<'a>;
            model remove_private_as_ingress("remove_private_as_ingress", 8) -> item::RemovePrivateAsIngress<'a>;
            scalar next_hop_unchanged("next_hop_unchanged", 9) -> bool;
            scalar update_source("update_source", 10) -> &'a str;
            scalar route_reflector_client("route_reflector_client", 11) -> bool;
            scalar bfd("bfd", 12) -> bool;
            model bfd_timers("bfd_timers", 13) -> item::BfdTimers<'a>;
            scalar ebgp_multihop("ebgp_multihop", 14) -> i64;
            scalar next_hop_peer("next_hop_peer", 15) -> bool;
            scalar next_hop_self("next_hop_self", 16) -> bool;
            scalar password("password", 17) -> &'a str;
            scalar password_type("password_type", 18) -> &'a str;
            scalar passive("passive", 19) -> bool;
            model default_originate("default_originate", 20) -> item::DefaultOriginate<'a>;
            scalar enforce_first_as("enforce_first_as", 21) -> bool;
            scalar send_community("send_community", 22) -> &'a str;
            scalar maximum_routes("maximum_routes", 23) -> i64;
            scalar maximum_routes_warning_limit("maximum_routes_warning_limit", 24) -> &'a str;
            scalar maximum_routes_warning_only("maximum_routes_warning_only", 25) -> bool;
            model maximum_accepted_routes("maximum_accepted_routes", 26) -> item::MaximumAcceptedRoutes<'a>;
            model missing_policy("missing_policy", 27) -> item::MissingPolicy<'a>;
            model link_bandwidth("link_bandwidth", 28) -> item::LinkBandwidth<'a>;
            model allowas_in("allowas_in", 29) -> item::AllowasIn<'a>;
            scalar weight("weight", 30) -> i64;
            scalar timers("timers", 31) -> &'a str;
            model rib_in_pre_policy_retain("rib_in_pre_policy_retain", 32) -> item::RibInPrePolicyRetain<'a>;
            scalar route_map_in("route_map_in", 33) -> &'a str;
            scalar route_map_out("route_map_out", 34) -> &'a str;
            scalar peer_tag_in("peer_tag_in", 35) -> &'a str;
            scalar peer_tag_out_discard("peer_tag_out_discard", 36) -> &'a str;
            scalar session_tracker("session_tracker", 37) -> &'a str;
            model shared_secret("shared_secret", 38) -> item::SharedSecret<'a>;
            scalar ttl_maximum_hops("ttl_maximum_hops", 39) -> i64;
            scalar maximum_advertised_routes("maximum_advertised_routes", 40) -> i64;
            scalar maximum_advertised_routes_warning_limit("maximum_advertised_routes_warning_limit", 41) -> &'a str;
        }
    }

    pub mod item {

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

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Neighbors {
        model item (0) -> neighbors::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod neighbors {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar ip_address("ip_address", 0) -> &'a str;
            scalar peer_group("peer_group", 1) -> &'a str;
            scalar remote_as("remote_as", 2) -> &'a str;
            scalar local_as("local_as", 3) -> &'a str;
            model as_path("as_path", 4) -> item::AsPath<'a>;
            model metadata("metadata", 5) -> item::Metadata<'a>;
            scalar description("description", 6) -> &'a str;
            scalar route_reflector_client("route_reflector_client", 7) -> bool;
            scalar password("password", 8) -> &'a str;
            scalar password_type("password_type", 9) -> &'a str;
            scalar passive("passive", 10) -> bool;
            scalar shutdown("shutdown", 11) -> bool;
            scalar update_source("update_source", 12) -> &'a str;
            scalar bfd("bfd", 13) -> bool;
            model bfd_timers("bfd_timers", 14) -> item::BfdTimers<'a>;
            scalar weight("weight", 15) -> i64;
            scalar timers("timers", 16) -> &'a str;
            scalar route_map_in("route_map_in", 17) -> &'a str;
            scalar route_map_out("route_map_out", 18) -> &'a str;
            scalar peer_tag_in("peer_tag_in", 19) -> &'a str;
            scalar peer_tag_out_discard("peer_tag_out_discard", 20) -> &'a str;
            model default_originate("default_originate", 21) -> item::DefaultOriginate<'a>;
            scalar enforce_first_as("enforce_first_as", 22) -> bool;
            scalar send_community("send_community", 23) -> &'a str;
            scalar maximum_routes("maximum_routes", 24) -> i64;
            scalar maximum_routes_warning_limit("maximum_routes_warning_limit", 25) -> &'a str;
            scalar maximum_routes_warning_only("maximum_routes_warning_only", 26) -> bool;
            model maximum_accepted_routes("maximum_accepted_routes", 27) -> item::MaximumAcceptedRoutes<'a>;
            model missing_policy("missing_policy", 28) -> item::MissingPolicy<'a>;
            model allowas_in("allowas_in", 29) -> item::AllowasIn<'a>;
            scalar ebgp_multihop("ebgp_multihop", 30) -> i64;
            scalar next_hop_peer("next_hop_peer", 31) -> bool;
            scalar next_hop_self("next_hop_self", 32) -> bool;
            model link_bandwidth("link_bandwidth", 33) -> item::LinkBandwidth<'a>;
            model rib_in_pre_policy_retain("rib_in_pre_policy_retain", 34) -> item::RibInPrePolicyRetain<'a>;
            model remove_private_as("remove_private_as", 35) -> item::RemovePrivateAs<'a>;
            model remove_private_as_ingress("remove_private_as_ingress", 36) -> item::RemovePrivateAsIngress<'a>;
            scalar session_tracker("session_tracker", 37) -> &'a str;
            model shared_secret("shared_secret", 38) -> item::SharedSecret<'a>;
            scalar ttl_maximum_hops("ttl_maximum_hops", 39) -> i64;
            scalar maximum_advertised_routes("maximum_advertised_routes", 40) -> i64;
            scalar maximum_advertised_routes_warning_limit("maximum_advertised_routes_warning_limit", 41) -> &'a str;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AsPath {
                scalar remote_as_replace_out("remote_as_replace_out", 0) -> bool;
                scalar prepend_own_disabled("prepend_own_disabled", 1) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Metadata {
                scalar peer("peer", 0) -> &'a str;
                scalar validate_state("validate_state", 1) -> bool;
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
            pub struct AllowasIn {
                scalar enabled("enabled", 0) -> bool;
                scalar times("times", 1) -> i64;
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
            pub struct RibInPrePolicyRetain {
                scalar enabled("enabled", 0) -> bool;
                scalar all("all", 1) -> bool;
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
            pub struct SharedSecret {
                scalar profile("profile", 0) -> &'a str;
                scalar hash_algorithm("hash_algorithm", 1) -> &'a str;
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct NeighborInterfaces {
        model item (0) -> neighbor_interfaces::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod neighbor_interfaces {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar remote_as("remote_as", 1) -> &'a str;
            model metadata("metadata", 2) -> item::Metadata<'a>;
            scalar peer_group("peer_group", 3) -> &'a str;
            scalar description("description", 4) -> &'a str;
            scalar peer_filter("peer_filter", 5) -> &'a str;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Metadata {
                scalar peer("peer", 0) -> &'a str;
                scalar validate_state("validate_state", 1) -> bool;
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AggregateAddresses {
        model item (0) -> aggregate_addresses::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod aggregate_addresses {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar prefix("prefix", 0) -> &'a str;
            scalar advertise_only("advertise_only", 1) -> bool;
            scalar as_set("as_set", 2) -> bool;
            scalar summary_only("summary_only", 3) -> bool;
            scalar attribute_map("attribute_map", 4) -> &'a str;
            scalar match_map("match_map", 5) -> &'a str;
            model attribute("attribute", 6) -> item::Attribute<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Attribute {
                scalar rcf("rcf", 0) -> &'a str;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Redistribute {
        model attached_host("attached_host", 0) -> redistribute::AttachedHost<'a>;
        model bgp("bgp", 1) -> redistribute::Bgp<'a>;
        model connected("connected", 2) -> redistribute::Connected<'a>;
        model dynamic("dynamic", 3) -> redistribute::Dynamic<'a>;
        model isis("isis", 4) -> redistribute::Isis<'a>;
        model ospf("ospf", 5) -> redistribute::Ospf<'a>;
        model ospfv3("ospfv3", 6) -> redistribute::Ospfv3<'a>;
        model rip("rip", 7) -> redistribute::Rip<'a>;
        model field_static("static", 8) -> redistribute::FieldStatic<'a>;
        model user("user", 9) -> redistribute::User<'a>;
    }
}

pub mod redistribute {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct AttachedHost {
            scalar enabled("enabled", 0) -> bool;
            scalar route_map("route_map", 1) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Bgp {
            scalar enabled("enabled", 0) -> bool;
            scalar route_map("route_map", 1) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Connected {
            scalar enabled("enabled", 0) -> bool;
            scalar route_map("route_map", 1) -> &'a str;
            scalar rcf("rcf", 2) -> &'a str;
            scalar include_leaked("include_leaked", 3) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Dynamic {
            scalar enabled("enabled", 0) -> bool;
            scalar route_map("route_map", 1) -> &'a str;
            scalar rcf("rcf", 2) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Isis {
            scalar enabled("enabled", 0) -> bool;
            scalar isis_level("isis_level", 1) -> &'a str;
            scalar route_map("route_map", 2) -> &'a str;
            scalar rcf("rcf", 3) -> &'a str;
            scalar include_leaked("include_leaked", 4) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ospf {
            scalar enabled("enabled", 0) -> bool;
            model match_external("match_external", 1) -> ospf::MatchExternal<'a>;
            model match_internal("match_internal", 2) -> ospf::MatchInternal<'a>;
            model match_nssa_external("match_nssa_external", 3) -> ospf::MatchNssaExternal<'a>;
            scalar route_map("route_map", 4) -> &'a str;
            scalar include_leaked("include_leaked", 5) -> bool;
        }
    }

    pub mod ospf {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct MatchExternal {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
                scalar include_leaked("include_leaked", 2) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct MatchInternal {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
                scalar include_leaked("include_leaked", 2) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct MatchNssaExternal {
                scalar enabled("enabled", 0) -> bool;
                scalar nssa_type("nssa_type", 1) -> i64;
                scalar route_map("route_map", 2) -> &'a str;
                scalar include_leaked("include_leaked", 3) -> bool;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ospfv3 {
            scalar enabled("enabled", 0) -> bool;
            model match_external("match_external", 1) -> ospfv3::MatchExternal<'a>;
            model match_internal("match_internal", 2) -> ospfv3::MatchInternal<'a>;
            model match_nssa_external("match_nssa_external", 3) -> ospfv3::MatchNssaExternal<'a>;
            scalar route_map("route_map", 4) -> &'a str;
            scalar include_leaked("include_leaked", 5) -> bool;
        }
    }

    pub mod ospfv3 {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct MatchExternal {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
                scalar include_leaked("include_leaked", 2) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct MatchInternal {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
                scalar include_leaked("include_leaked", 2) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct MatchNssaExternal {
                scalar enabled("enabled", 0) -> bool;
                scalar nssa_type("nssa_type", 1) -> i64;
                scalar route_map("route_map", 2) -> &'a str;
                scalar include_leaked("include_leaked", 3) -> bool;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Rip {
            scalar enabled("enabled", 0) -> bool;
            scalar route_map("route_map", 1) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct FieldStatic {
            scalar enabled("enabled", 0) -> bool;
            scalar route_map("route_map", 1) -> &'a str;
            scalar rcf("rcf", 2) -> &'a str;
            scalar include_leaked("include_leaked", 3) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct User {
            scalar enabled("enabled", 0) -> bool;
            scalar rcf("rcf", 1) -> &'a str;
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct VlanAwareBundles {
        model item (0) -> vlan_aware_bundles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod vlan_aware_bundles {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model metadata("metadata", 1) -> item::Metadata<'a>;
            scalar rd("rd", 2) -> &'a str;
            model rd_evpn_domain("rd_evpn_domain", 3) -> item::RdEvpnDomain<'a>;
            model route_targets("route_targets", 4) -> item::RouteTargets<'a>;
            model redistribute_routes("redistribute_routes", 5) -> item::RedistributeRoutes<'a>;
            model no_redistribute_routes("no_redistribute_routes", 6) -> item::NoRedistributeRoutes<'a>;
            scalar vlan("vlan", 7) -> &'a str;
            scalar eos_cli("eos_cli", 8) -> &'a str;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Metadata {
                model tenants("tenants", 0) -> metadata::Tenants<'a>;
                scalar description("description", 1) -> &'a str;
            }
        }

        pub mod metadata {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Tenants {
                    scalar item (0) -> &'a str;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct RdEvpnDomain {
                scalar domain("domain", 0) -> &'a str;
                scalar rd("rd", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct RouteTargets {
                model both("both", 0) -> route_targets::Both<'a>;
                model field_import("import", 1) -> route_targets::Import<'a>;
                model export("export", 2) -> route_targets::Export<'a>;
                model import_evpn_domains("import_evpn_domains", 3) -> route_targets::ImportEvpnDomains<'a>;
                model export_evpn_domains("export_evpn_domains", 4) -> route_targets::ExportEvpnDomains<'a>;
                model import_export_evpn_domains("import_export_evpn_domains", 5) -> route_targets::ImportExportEvpnDomains<'a>;
            }
        }

        pub mod route_targets {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Both {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Import {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Export {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct ImportEvpnDomains {
                    model item (0) -> import_evpn_domains::Item<'a>;
                }
            }

            pub mod import_evpn_domains {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar domain("domain", 0) -> &'a str;
                        scalar route_target("route_target", 1) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct ExportEvpnDomains {
                    model item (0) -> export_evpn_domains::Item<'a>;
                }
            }

            pub mod export_evpn_domains {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar domain("domain", 0) -> &'a str;
                        scalar route_target("route_target", 1) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct ImportExportEvpnDomains {
                    model item (0) -> import_export_evpn_domains::Item<'a>;
                }
            }

            pub mod import_export_evpn_domains {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar domain("domain", 0) -> &'a str;
                        scalar route_target("route_target", 1) -> &'a str;
                    }
                }
            }
        }

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct RedistributeRoutes {
                scalar item (0) -> &'a str;
            }
        }

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct NoRedistributeRoutes {
                scalar item (0) -> &'a str;
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Vlans {
        model item (0) -> vlans::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod vlans {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar id("id", 0) -> i64;
            model metadata("metadata", 1) -> item::Metadata<'a>;
            scalar rd("rd", 2) -> &'a str;
            model rd_evpn_domain("rd_evpn_domain", 3) -> item::RdEvpnDomain<'a>;
            model route_targets("route_targets", 4) -> item::RouteTargets<'a>;
            model redistribute_routes("redistribute_routes", 5) -> item::RedistributeRoutes<'a>;
            model no_redistribute_routes("no_redistribute_routes", 6) -> item::NoRedistributeRoutes<'a>;
            scalar eos_cli("eos_cli", 7) -> &'a str;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Metadata {
                model tenants("tenants", 0) -> metadata::Tenants<'a>;
            }
        }

        pub mod metadata {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Tenants {
                    scalar item (0) -> &'a str;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct RdEvpnDomain {
                scalar domain("domain", 0) -> &'a str;
                scalar rd("rd", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct RouteTargets {
                model both("both", 0) -> route_targets::Both<'a>;
                model field_import("import", 1) -> route_targets::Import<'a>;
                model export("export", 2) -> route_targets::Export<'a>;
                model import_evpn_domains("import_evpn_domains", 3) -> route_targets::ImportEvpnDomains<'a>;
                model export_evpn_domains("export_evpn_domains", 4) -> route_targets::ExportEvpnDomains<'a>;
                model import_export_evpn_domains("import_export_evpn_domains", 5) -> route_targets::ImportExportEvpnDomains<'a>;
            }
        }

        pub mod route_targets {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Both {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Import {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Export {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct ImportEvpnDomains {
                    model item (0) -> import_evpn_domains::Item<'a>;
                }
            }

            pub mod import_evpn_domains {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar domain("domain", 0) -> &'a str;
                        scalar route_target("route_target", 1) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct ExportEvpnDomains {
                    model item (0) -> export_evpn_domains::Item<'a>;
                }
            }

            pub mod export_evpn_domains {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar domain("domain", 0) -> &'a str;
                        scalar route_target("route_target", 1) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct ImportExportEvpnDomains {
                    model item (0) -> import_export_evpn_domains::Item<'a>;
                }
            }

            pub mod import_export_evpn_domains {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar domain("domain", 0) -> &'a str;
                        scalar route_target("route_target", 1) -> &'a str;
                    }
                }
            }
        }

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct RedistributeRoutes {
                scalar item (0) -> &'a str;
            }
        }

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct NoRedistributeRoutes {
                scalar item (0) -> &'a str;
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Vpws {
        model item (0) -> vpws::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod vpws {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar rd("rd", 1) -> &'a str;
            model route_targets("route_targets", 2) -> item::RouteTargets<'a>;
            scalar mpls_control_word("mpls_control_word", 3) -> bool;
            scalar label_flow("label_flow", 4) -> bool;
            scalar mtu("mtu", 5) -> i64;
            model pseudowires("pseudowires", 6) -> item::Pseudowires<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct RouteTargets {
                scalar import_export("import_export", 0) -> &'a str;
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Pseudowires {
                model item (0) -> pseudowires::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod pseudowires {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    scalar id_local("id_local", 1) -> i64;
                    scalar id_remote("id_remote", 2) -> i64;
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AddressFamilyEvpn {
        scalar domain_identifier("domain_identifier", 0) -> &'a str;
        scalar domain_identifier_remote("domain_identifier_remote", 1) -> &'a str;
        model neighbor_default("neighbor_default", 2) -> address_family_evpn::NeighborDefault<'a>;
        model next_hop_mpls_resolution_ribs("next_hop_mpls_resolution_ribs", 3) -> address_family_evpn::NextHopMplsResolutionRibs<'a>;
        model neighbors("neighbors", 4) -> address_family_evpn::Neighbors<'a>;
        model peer_groups("peer_groups", 5) -> address_family_evpn::PeerGroups<'a>;
        model evpn_hostflap_detection("evpn_hostflap_detection", 6) -> address_family_evpn::EvpnHostflapDetection<'a>;
        model next_hop("next_hop", 7) -> address_family_evpn::NextHop<'a>;
        model route("route", 8) -> address_family_evpn::Route<'a>;
        scalar next_hop_unchanged("next_hop_unchanged", 9) -> bool;
        model bgp("bgp", 10) -> address_family_evpn::Bgp<'a>;
        model layer_2_fec_in_place_update("layer_2_fec_in_place_update", 11) -> address_family_evpn::Layer2FecInPlaceUpdate<'a>;
        model evpn_ethernet_segment("evpn_ethernet_segment", 12) -> address_family_evpn::EvpnEthernetSegment<'a>;
    }
}

pub mod address_family_evpn {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct NeighborDefault {
            scalar encapsulation("encapsulation", 0) -> &'a str;
            scalar next_hop_self_source_interface("next_hop_self_source_interface", 1) -> &'a str;
            model next_hop_self_received_evpn_routes("next_hop_self_received_evpn_routes", 2) -> neighbor_default::NextHopSelfReceivedEvpnRoutes<'a>;
        }
    }

    pub mod neighbor_default {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct NextHopSelfReceivedEvpnRoutes {
                scalar enable("enable", 0) -> bool;
                scalar inter_domain("inter_domain", 1) -> bool;
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct NextHopMplsResolutionRibs {
            model item (0) -> next_hop_mpls_resolution_ribs::Item<'a>;
        }
    }

    pub mod next_hop_mpls_resolution_ribs {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar rib_type("rib_type", 0) -> &'a str;
                scalar rib_name("rib_name", 1) -> &'a str;
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Neighbors {
            model item (0) -> neighbors::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod neighbors {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar ip_address("ip_address", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                scalar route_map_in("route_map_in", 2) -> &'a str;
                scalar route_map_out("route_map_out", 3) -> &'a str;
                scalar peer_tag_in("peer_tag_in", 4) -> &'a str;
                scalar peer_tag_out_discard("peer_tag_out_discard", 5) -> &'a str;
                scalar rcf_in("rcf_in", 6) -> &'a str;
                scalar rcf_out("rcf_out", 7) -> &'a str;
                model default_route("default_route", 8) -> item::DefaultRoute<'a>;
                model additional_paths("additional_paths", 9) -> item::AdditionalPaths<'a>;
                scalar encapsulation("encapsulation", 10) -> &'a str;
                scalar next_hop_self_source_interface("next_hop_self_source_interface", 11) -> &'a str;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct DefaultRoute {
                    scalar enabled("enabled", 0) -> bool;
                    scalar rcf("rcf", 1) -> &'a str;
                    scalar route_map("route_map", 2) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AdditionalPaths {
                    scalar receive("receive", 0) -> bool;
                    scalar send("send", 1) -> &'a str;
                    scalar send_limit("send_limit", 2) -> i64;
                }
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PeerGroups {
            model item (0) -> peer_groups::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod peer_groups {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                scalar route_map_in("route_map_in", 2) -> &'a str;
                scalar route_map_out("route_map_out", 3) -> &'a str;
                scalar peer_tag_in("peer_tag_in", 4) -> &'a str;
                scalar peer_tag_out_discard("peer_tag_out_discard", 5) -> &'a str;
                scalar rcf_in("rcf_in", 6) -> &'a str;
                scalar rcf_out("rcf_out", 7) -> &'a str;
                model default_route("default_route", 8) -> item::DefaultRoute<'a>;
                scalar domain_remote("domain_remote", 9) -> bool;
                scalar encapsulation("encapsulation", 10) -> &'a str;
                scalar next_hop_self_source_interface("next_hop_self_source_interface", 11) -> &'a str;
                model additional_paths("additional_paths", 12) -> item::AdditionalPaths<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct DefaultRoute {
                    scalar enabled("enabled", 0) -> bool;
                    scalar rcf("rcf", 1) -> &'a str;
                    scalar route_map("route_map", 2) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AdditionalPaths {
                    scalar receive("receive", 0) -> bool;
                    scalar send("send", 1) -> &'a str;
                    scalar send_limit("send_limit", 2) -> i64;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct EvpnHostflapDetection {
            scalar enabled("enabled", 0) -> bool;
            scalar window("window", 1) -> i64;
            scalar threshold("threshold", 2) -> i64;
            scalar expiry_timeout("expiry_timeout", 3) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct NextHop {
            scalar resolution_disabled("resolution_disabled", 0) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Route {
            scalar import_match_failure_action("import_match_failure_action", 0) -> &'a str;
            scalar import_ethernet_segment_ip_mass_withdraw("import_ethernet_segment_ip_mass_withdraw", 1) -> bool;
            scalar import_overlay_index_gateway("import_overlay_index_gateway", 2) -> bool;
            scalar export_ethernet_segment_ip_mass_withdraw("export_ethernet_segment_ip_mass_withdraw", 3) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Bgp {
            model additional_paths("additional_paths", 0) -> bgp::AdditionalPaths<'a>;
        }
    }

    pub mod bgp {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AdditionalPaths {
                scalar receive("receive", 0) -> bool;
                scalar send("send", 1) -> &'a str;
                scalar send_limit("send_limit", 2) -> i64;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Layer2FecInPlaceUpdate {
            scalar enabled("enabled", 0) -> bool;
            scalar timeout("timeout", 1) -> i64;
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct EvpnEthernetSegment {
            model item (0) -> evpn_ethernet_segment::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod evpn_ethernet_segment {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar domain("domain", 0) -> &'a str;
                scalar identifier("identifier", 1) -> &'a str;
                scalar route_target_import("route_target_import", 2) -> &'a str;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AddressFamilyRtc {
        model peer_groups("peer_groups", 0) -> address_family_rtc::PeerGroups<'a>;
    }
}

pub mod address_family_rtc {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PeerGroups {
            model item (0) -> peer_groups::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod peer_groups {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                model default_route_target("default_route_target", 2) -> item::DefaultRouteTarget<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct DefaultRouteTarget {
                    scalar only("only", 0) -> bool;
                    scalar encoding_origin_as_omit("encoding_origin_as_omit", 1) -> &'a str;
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AddressFamilyIpv4 {
        model networks("networks", 0) -> address_family_ipv4::Networks<'a>;
        model bgp("bgp", 1) -> address_family_ipv4::Bgp<'a>;
        model peer_groups("peer_groups", 2) -> address_family_ipv4::PeerGroups<'a>;
        model neighbors("neighbors", 3) -> address_family_ipv4::Neighbors<'a>;
        model redistribute("redistribute", 4) -> address_family_ipv4::Redistribute<'a>;
        model next_hop("next_hop", 5) -> address_family_ipv4::NextHop<'a>;
    }
}

pub mod address_family_ipv4 {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Networks {
            model item (0) -> networks::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod networks {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar prefix("prefix", 0) -> &'a str;
                scalar route_map("route_map", 1) -> &'a str;
                scalar rcf("rcf", 2) -> &'a str;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Bgp {
            model additional_paths("additional_paths", 0) -> bgp::AdditionalPaths<'a>;
            scalar redistribute_internal("redistribute_internal", 1) -> bool;
        }
    }

    pub mod bgp {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AdditionalPaths {
                scalar install("install", 0) -> bool;
                scalar install_ecmp_primary("install_ecmp_primary", 1) -> bool;
                scalar receive("receive", 2) -> bool;
                scalar send("send", 3) -> &'a str;
                scalar send_limit("send_limit", 4) -> i64;
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PeerGroups {
            model item (0) -> peer_groups::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod peer_groups {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                scalar route_map_in("route_map_in", 2) -> &'a str;
                scalar route_map_out("route_map_out", 3) -> &'a str;
                scalar peer_tag_in("peer_tag_in", 4) -> &'a str;
                scalar peer_tag_out_discard("peer_tag_out_discard", 5) -> &'a str;
                scalar rcf_in("rcf_in", 6) -> &'a str;
                scalar rcf_out("rcf_out", 7) -> &'a str;
                model default_originate("default_originate", 8) -> item::DefaultOriginate<'a>;
                scalar prefix_list_in("prefix_list_in", 9) -> &'a str;
                scalar prefix_list_out("prefix_list_out", 10) -> &'a str;
                model additional_paths("additional_paths", 11) -> item::AdditionalPaths<'a>;
                model next_hop("next_hop", 12) -> item::NextHop<'a>;
                scalar maximum_advertised_routes("maximum_advertised_routes", 13) -> i64;
                scalar maximum_advertised_routes_warning_limit("maximum_advertised_routes_warning_limit", 14) -> &'a str;
                model maximum_accepted_routes("maximum_accepted_routes", 15) -> item::MaximumAcceptedRoutes<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct DefaultOriginate {
                    scalar always("always", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AdditionalPaths {
                    scalar prefix_list("prefix_list", 0) -> &'a str;
                    scalar receive("receive", 1) -> bool;
                    scalar send("send", 2) -> &'a str;
                    scalar send_limit("send_limit", 3) -> i64;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct NextHop {
                    model address_family_ipv6("address_family_ipv6", 0) -> next_hop::AddressFamilyIpv6<'a>;
                }
            }

            pub mod next_hop {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct AddressFamilyIpv6 {
                        scalar enabled("enabled", 0) -> bool;
                        scalar originate("originate", 1) -> bool;
                    }
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
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Neighbors {
            model item (0) -> neighbors::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod neighbors {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar ip_address("ip_address", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                scalar route_map_in("route_map_in", 2) -> &'a str;
                scalar route_map_out("route_map_out", 3) -> &'a str;
                scalar peer_tag_in("peer_tag_in", 4) -> &'a str;
                scalar peer_tag_out_discard("peer_tag_out_discard", 5) -> &'a str;
                scalar rcf_in("rcf_in", 6) -> &'a str;
                scalar rcf_out("rcf_out", 7) -> &'a str;
                scalar prefix_list_in("prefix_list_in", 8) -> &'a str;
                scalar prefix_list_out("prefix_list_out", 9) -> &'a str;
                model default_originate("default_originate", 10) -> item::DefaultOriginate<'a>;
                model additional_paths("additional_paths", 11) -> item::AdditionalPaths<'a>;
                model next_hop("next_hop", 12) -> item::NextHop<'a>;
                scalar maximum_advertised_routes("maximum_advertised_routes", 13) -> i64;
                scalar maximum_advertised_routes_warning_limit("maximum_advertised_routes_warning_limit", 14) -> &'a str;
                model maximum_accepted_routes("maximum_accepted_routes", 15) -> item::MaximumAcceptedRoutes<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct DefaultOriginate {
                    scalar always("always", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AdditionalPaths {
                    scalar prefix_list("prefix_list", 0) -> &'a str;
                    scalar receive("receive", 1) -> bool;
                    scalar send("send", 2) -> &'a str;
                    scalar send_limit("send_limit", 3) -> i64;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct NextHop {
                    model address_family_ipv6("address_family_ipv6", 0) -> next_hop::AddressFamilyIpv6<'a>;
                }
            }

            pub mod next_hop {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct AddressFamilyIpv6 {
                        scalar enabled("enabled", 0) -> bool;
                        scalar originate("originate", 1) -> bool;
                    }
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
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Redistribute {
            model attached_host("attached_host", 0) -> redistribute::AttachedHost<'a>;
            model bgp("bgp", 1) -> redistribute::Bgp<'a>;
            model connected("connected", 2) -> redistribute::Connected<'a>;
            model dynamic("dynamic", 3) -> redistribute::Dynamic<'a>;
            model isis("isis", 4) -> redistribute::Isis<'a>;
            model ospf("ospf", 5) -> redistribute::Ospf<'a>;
            model ospfv3("ospfv3", 6) -> redistribute::Ospfv3<'a>;
            model rip("rip", 7) -> redistribute::Rip<'a>;
            model field_static("static", 8) -> redistribute::FieldStatic<'a>;
            model user("user", 9) -> redistribute::User<'a>;
        }
    }

    pub mod redistribute {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AttachedHost {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Bgp {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Connected {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
                scalar rcf("rcf", 2) -> &'a str;
                scalar include_leaked("include_leaked", 3) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Dynamic {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
                scalar rcf("rcf", 2) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Isis {
                scalar enabled("enabled", 0) -> bool;
                scalar isis_level("isis_level", 1) -> &'a str;
                scalar route_map("route_map", 2) -> &'a str;
                scalar rcf("rcf", 3) -> &'a str;
                scalar include_leaked("include_leaked", 4) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ospf {
                scalar enabled("enabled", 0) -> bool;
                model match_external("match_external", 1) -> ospf::MatchExternal<'a>;
                model match_internal("match_internal", 2) -> ospf::MatchInternal<'a>;
                model match_nssa_external("match_nssa_external", 3) -> ospf::MatchNssaExternal<'a>;
                scalar route_map("route_map", 4) -> &'a str;
                scalar include_leaked("include_leaked", 5) -> bool;
            }
        }

        pub mod ospf {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MatchExternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                    scalar include_leaked("include_leaked", 2) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MatchInternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                    scalar include_leaked("include_leaked", 2) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MatchNssaExternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar nssa_type("nssa_type", 1) -> i64;
                    scalar route_map("route_map", 2) -> &'a str;
                    scalar include_leaked("include_leaked", 3) -> bool;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ospfv3 {
                scalar enabled("enabled", 0) -> bool;
                model match_external("match_external", 1) -> ospfv3::MatchExternal<'a>;
                model match_internal("match_internal", 2) -> ospfv3::MatchInternal<'a>;
                model match_nssa_external("match_nssa_external", 3) -> ospfv3::MatchNssaExternal<'a>;
                scalar route_map("route_map", 4) -> &'a str;
                scalar include_leaked("include_leaked", 5) -> bool;
            }
        }

        pub mod ospfv3 {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MatchExternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                    scalar include_leaked("include_leaked", 2) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MatchInternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                    scalar include_leaked("include_leaked", 2) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MatchNssaExternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar nssa_type("nssa_type", 1) -> i64;
                    scalar route_map("route_map", 2) -> &'a str;
                    scalar include_leaked("include_leaked", 3) -> bool;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Rip {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct FieldStatic {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
                scalar rcf("rcf", 2) -> &'a str;
                scalar include_leaked("include_leaked", 3) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct User {
                scalar enabled("enabled", 0) -> bool;
                scalar rcf("rcf", 1) -> &'a str;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct NextHop {
            scalar resolution_disabled("resolution_disabled", 0) -> bool;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AddressFamilyIpv4LabeledUnicast {
        model aigp_session("aigp_session", 0) -> address_family_ipv4_labeled_unicast::AigpSession<'a>;
        model bgp("bgp", 1) -> address_family_ipv4_labeled_unicast::Bgp<'a>;
        scalar graceful_restart("graceful_restart", 2) -> bool;
        scalar label_local_termination("label_local_termination", 3) -> &'a str;
        scalar lfib_entry_installation_skipped("lfib_entry_installation_skipped", 4) -> bool;
        model neighbor_default("neighbor_default", 5) -> address_family_ipv4_labeled_unicast::NeighborDefault<'a>;
        model peer_groups("peer_groups", 6) -> address_family_ipv4_labeled_unicast::PeerGroups<'a>;
        model neighbors("neighbors", 7) -> address_family_ipv4_labeled_unicast::Neighbors<'a>;
        model networks("networks", 8) -> address_family_ipv4_labeled_unicast::Networks<'a>;
        model next_hop("next_hop", 9) -> address_family_ipv4_labeled_unicast::NextHop<'a>;
        model next_hops("next_hops", 10) -> address_family_ipv4_labeled_unicast::NextHops<'a>;
        model next_hop_resolution_ribs("next_hop_resolution_ribs", 11) -> address_family_ipv4_labeled_unicast::NextHopResolutionRibs<'a>;
        model tunnel_source_protocols("tunnel_source_protocols", 12) -> address_family_ipv4_labeled_unicast::TunnelSourceProtocols<'a>;
        scalar update_wait_for_convergence("update_wait_for_convergence", 13) -> bool;
    }
}

pub mod address_family_ipv4_labeled_unicast {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct AigpSession {
            scalar confederation("confederation", 0) -> bool;
            scalar ebgp("ebgp", 1) -> bool;
            scalar ibgp("ibgp", 2) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Bgp {
            model additional_paths("additional_paths", 0) -> bgp::AdditionalPaths<'a>;
            model missing_policy("missing_policy", 1) -> bgp::MissingPolicy<'a>;
            scalar next_hop_unchanged("next_hop_unchanged", 2) -> bool;
        }
    }

    pub mod bgp {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AdditionalPaths {
                scalar receive("receive", 0) -> bool;
                scalar send("send", 1) -> &'a str;
                scalar send_limit("send_limit", 2) -> i64;
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
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct NeighborDefault {
            scalar next_hop_self("next_hop_self", 0) -> bool;
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PeerGroups {
            model item (0) -> peer_groups::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod peer_groups {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                model additional_paths("additional_paths", 2) -> item::AdditionalPaths<'a>;
                scalar aigp_session("aigp_session", 3) -> bool;
                scalar graceful_restart("graceful_restart", 4) -> bool;
                model graceful_restart_helper("graceful_restart_helper", 5) -> item::GracefulRestartHelper<'a>;
                scalar maximum_advertised_routes("maximum_advertised_routes", 6) -> i64;
                scalar maximum_advertised_routes_warning_limit("maximum_advertised_routes_warning_limit", 7) -> &'a str;
                model missing_policy("missing_policy", 8) -> item::MissingPolicy<'a>;
                scalar multi_path("multi_path", 9) -> bool;
                scalar next_hop_self("next_hop_self", 10) -> bool;
                scalar next_hop_self_source_interface("next_hop_self_source_interface", 11) -> &'a str;
                scalar next_hop_self_v4_mapped_v6_source_interface("next_hop_self_v4_mapped_v6_source_interface", 12) -> &'a str;
                scalar next_hop_unchanged("next_hop_unchanged", 13) -> bool;
                scalar rcf_in("rcf_in", 14) -> &'a str;
                scalar rcf_out("rcf_out", 15) -> &'a str;
                scalar route_map_in("route_map_in", 16) -> &'a str;
                scalar route_map_out("route_map_out", 17) -> &'a str;
                scalar peer_tag_in("peer_tag_in", 18) -> &'a str;
                scalar peer_tag_out_discard("peer_tag_out_discard", 19) -> &'a str;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AdditionalPaths {
                    scalar receive("receive", 0) -> bool;
                    scalar send("send", 1) -> &'a str;
                    scalar send_limit("send_limit", 2) -> i64;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct GracefulRestartHelper {
                    scalar stale_route_map("stale_route_map", 0) -> &'a str;
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
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Neighbors {
            model item (0) -> neighbors::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod neighbors {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar ip_address("ip_address", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                model additional_paths("additional_paths", 2) -> item::AdditionalPaths<'a>;
                scalar aigp_session("aigp_session", 3) -> bool;
                scalar graceful_restart("graceful_restart", 4) -> bool;
                model graceful_restart_helper("graceful_restart_helper", 5) -> item::GracefulRestartHelper<'a>;
                scalar maximum_advertised_routes("maximum_advertised_routes", 6) -> i64;
                scalar maximum_advertised_routes_warning_limit("maximum_advertised_routes_warning_limit", 7) -> &'a str;
                model missing_policy("missing_policy", 8) -> item::MissingPolicy<'a>;
                scalar multi_path("multi_path", 9) -> bool;
                scalar next_hop_self("next_hop_self", 10) -> bool;
                scalar next_hop_self_source_interface("next_hop_self_source_interface", 11) -> &'a str;
                scalar next_hop_self_v4_mapped_v6_source_interface("next_hop_self_v4_mapped_v6_source_interface", 12) -> &'a str;
                scalar next_hop_unchanged("next_hop_unchanged", 13) -> bool;
                scalar rcf_in("rcf_in", 14) -> &'a str;
                scalar rcf_out("rcf_out", 15) -> &'a str;
                scalar route_map_in("route_map_in", 16) -> &'a str;
                scalar route_map_out("route_map_out", 17) -> &'a str;
                scalar peer_tag_in("peer_tag_in", 18) -> &'a str;
                scalar peer_tag_out_discard("peer_tag_out_discard", 19) -> &'a str;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AdditionalPaths {
                    scalar receive("receive", 0) -> bool;
                    scalar send("send", 1) -> &'a str;
                    scalar send_limit("send_limit", 2) -> i64;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct GracefulRestartHelper {
                    scalar stale_route_map("stale_route_map", 0) -> &'a str;
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
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Networks {
            model item (0) -> networks::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod networks {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar prefix("prefix", 0) -> &'a str;
                scalar route_map("route_map", 1) -> &'a str;
                scalar rcf("rcf", 2) -> &'a str;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct NextHop {
            scalar resolution_disabled("resolution_disabled", 0) -> bool;
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct NextHops {
            model item (0) -> next_hops::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod next_hops {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar ip_address("ip_address", 0) -> &'a str;
                scalar lfib_backup_ip_forwarding("lfib_backup_ip_forwarding", 1) -> bool;
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct NextHopResolutionRibs {
            model item (0) -> next_hop_resolution_ribs::Item<'a>;
        }
    }

    pub mod next_hop_resolution_ribs {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar rib_type("rib_type", 0) -> &'a str;
                scalar rib_name("rib_name", 1) -> &'a str;
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct TunnelSourceProtocols {
            model item (0) -> tunnel_source_protocols::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod tunnel_source_protocols {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar protocol("protocol", 0) -> &'a str;
                scalar rcf("rcf", 1) -> &'a str;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AddressFamilyIpv4Multicast {
        model bgp("bgp", 0) -> address_family_ipv4_multicast::Bgp<'a>;
        model peer_groups("peer_groups", 1) -> address_family_ipv4_multicast::PeerGroups<'a>;
        model neighbors("neighbors", 2) -> address_family_ipv4_multicast::Neighbors<'a>;
        model redistribute("redistribute", 3) -> address_family_ipv4_multicast::Redistribute<'a>;
    }
}

pub mod address_family_ipv4_multicast {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Bgp {
            model additional_paths("additional_paths", 0) -> bgp::AdditionalPaths<'a>;
        }
    }

    pub mod bgp {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AdditionalPaths {
                scalar receive("receive", 0) -> bool;
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PeerGroups {
            model item (0) -> peer_groups::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod peer_groups {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                scalar route_map_in("route_map_in", 2) -> &'a str;
                scalar route_map_out("route_map_out", 3) -> &'a str;
                scalar peer_tag_in("peer_tag_in", 4) -> &'a str;
                scalar peer_tag_out_discard("peer_tag_out_discard", 5) -> &'a str;
                model additional_paths("additional_paths", 6) -> item::AdditionalPaths<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AdditionalPaths {
                    scalar receive("receive", 0) -> bool;
                }
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Neighbors {
            model item (0) -> neighbors::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod neighbors {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar ip_address("ip_address", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                scalar route_map_in("route_map_in", 2) -> &'a str;
                scalar route_map_out("route_map_out", 3) -> &'a str;
                scalar peer_tag_in("peer_tag_in", 4) -> &'a str;
                scalar peer_tag_out_discard("peer_tag_out_discard", 5) -> &'a str;
                model additional_paths("additional_paths", 6) -> item::AdditionalPaths<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AdditionalPaths {
                    scalar receive("receive", 0) -> bool;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Redistribute {
            model attached_host("attached_host", 0) -> redistribute::AttachedHost<'a>;
            model connected("connected", 1) -> redistribute::Connected<'a>;
            model isis("isis", 2) -> redistribute::Isis<'a>;
            model ospf("ospf", 3) -> redistribute::Ospf<'a>;
            model ospfv3("ospfv3", 4) -> redistribute::Ospfv3<'a>;
            model field_static("static", 5) -> redistribute::FieldStatic<'a>;
        }
    }

    pub mod redistribute {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AttachedHost {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Connected {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Isis {
                scalar enabled("enabled", 0) -> bool;
                scalar isis_level("isis_level", 1) -> &'a str;
                scalar route_map("route_map", 2) -> &'a str;
                scalar rcf("rcf", 3) -> &'a str;
                scalar include_leaked("include_leaked", 4) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ospf {
                scalar enabled("enabled", 0) -> bool;
                model match_external("match_external", 1) -> ospf::MatchExternal<'a>;
                model match_internal("match_internal", 2) -> ospf::MatchInternal<'a>;
                model match_nssa_external("match_nssa_external", 3) -> ospf::MatchNssaExternal<'a>;
                scalar route_map("route_map", 4) -> &'a str;
            }
        }

        pub mod ospf {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MatchExternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MatchInternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MatchNssaExternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar nssa_type("nssa_type", 1) -> i64;
                    scalar route_map("route_map", 2) -> &'a str;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ospfv3 {
                scalar enabled("enabled", 0) -> bool;
                model match_external("match_external", 1) -> ospfv3::MatchExternal<'a>;
                model match_internal("match_internal", 2) -> ospfv3::MatchInternal<'a>;
                model match_nssa_external("match_nssa_external", 3) -> ospfv3::MatchNssaExternal<'a>;
                scalar route_map("route_map", 4) -> &'a str;
            }
        }

        pub mod ospfv3 {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MatchExternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MatchInternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MatchNssaExternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar nssa_type("nssa_type", 1) -> i64;
                    scalar route_map("route_map", 2) -> &'a str;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct FieldStatic {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AddressFamilyIpv4SrTe {
        model neighbors("neighbors", 0) -> address_family_ipv4_sr_te::Neighbors<'a>;
        model peer_groups("peer_groups", 1) -> address_family_ipv4_sr_te::PeerGroups<'a>;
    }
}

pub mod address_family_ipv4_sr_te {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Neighbors {
            model item (0) -> neighbors::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod neighbors {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar ip_address("ip_address", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                scalar route_map_in("route_map_in", 2) -> &'a str;
                scalar route_map_out("route_map_out", 3) -> &'a str;
                scalar peer_tag_in("peer_tag_in", 4) -> &'a str;
                scalar peer_tag_out_discard("peer_tag_out_discard", 5) -> &'a str;
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PeerGroups {
            model item (0) -> peer_groups::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod peer_groups {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                scalar route_map_in("route_map_in", 2) -> &'a str;
                scalar route_map_out("route_map_out", 3) -> &'a str;
                scalar peer_tag_in("peer_tag_in", 4) -> &'a str;
                scalar peer_tag_out_discard("peer_tag_out_discard", 5) -> &'a str;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AddressFamilyIpv6 {
        model networks("networks", 0) -> address_family_ipv6::Networks<'a>;
        model bgp("bgp", 1) -> address_family_ipv6::Bgp<'a>;
        model peer_groups("peer_groups", 2) -> address_family_ipv6::PeerGroups<'a>;
        model neighbors("neighbors", 3) -> address_family_ipv6::Neighbors<'a>;
        model redistribute("redistribute", 4) -> address_family_ipv6::Redistribute<'a>;
        model next_hop("next_hop", 5) -> address_family_ipv6::NextHop<'a>;
    }
}

pub mod address_family_ipv6 {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Networks {
            model item (0) -> networks::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod networks {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar prefix("prefix", 0) -> &'a str;
                scalar route_map("route_map", 1) -> &'a str;
                scalar rcf("rcf", 2) -> &'a str;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Bgp {
            scalar redistribute_internal("redistribute_internal", 0) -> bool;
            model additional_paths("additional_paths", 1) -> bgp::AdditionalPaths<'a>;
        }
    }

    pub mod bgp {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AdditionalPaths {
                scalar install("install", 0) -> bool;
                scalar install_ecmp_primary("install_ecmp_primary", 1) -> bool;
                scalar receive("receive", 2) -> bool;
                scalar send("send", 3) -> &'a str;
                scalar send_limit("send_limit", 4) -> i64;
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PeerGroups {
            model item (0) -> peer_groups::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod peer_groups {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                scalar route_map_in("route_map_in", 2) -> &'a str;
                scalar route_map_out("route_map_out", 3) -> &'a str;
                scalar peer_tag_in("peer_tag_in", 4) -> &'a str;
                scalar peer_tag_out_discard("peer_tag_out_discard", 5) -> &'a str;
                scalar rcf_in("rcf_in", 6) -> &'a str;
                scalar rcf_out("rcf_out", 7) -> &'a str;
                scalar prefix_list_in("prefix_list_in", 8) -> &'a str;
                scalar prefix_list_out("prefix_list_out", 9) -> &'a str;
                model additional_paths("additional_paths", 10) -> item::AdditionalPaths<'a>;
                model default_originate("default_originate", 11) -> item::DefaultOriginate<'a>;
                scalar maximum_advertised_routes("maximum_advertised_routes", 12) -> i64;
                scalar maximum_advertised_routes_warning_limit("maximum_advertised_routes_warning_limit", 13) -> &'a str;
                model maximum_accepted_routes("maximum_accepted_routes", 14) -> item::MaximumAcceptedRoutes<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AdditionalPaths {
                    scalar prefix_list("prefix_list", 0) -> &'a str;
                    scalar receive("receive", 1) -> bool;
                    scalar send("send", 2) -> &'a str;
                    scalar send_limit("send_limit", 3) -> i64;
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
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Neighbors {
            model item (0) -> neighbors::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod neighbors {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar ip_address("ip_address", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                scalar route_map_in("route_map_in", 2) -> &'a str;
                scalar route_map_out("route_map_out", 3) -> &'a str;
                scalar peer_tag_in("peer_tag_in", 4) -> &'a str;
                scalar peer_tag_out_discard("peer_tag_out_discard", 5) -> &'a str;
                scalar rcf_in("rcf_in", 6) -> &'a str;
                scalar rcf_out("rcf_out", 7) -> &'a str;
                scalar prefix_list_in("prefix_list_in", 8) -> &'a str;
                scalar prefix_list_out("prefix_list_out", 9) -> &'a str;
                model default_originate("default_originate", 10) -> item::DefaultOriginate<'a>;
                model additional_paths("additional_paths", 11) -> item::AdditionalPaths<'a>;
                scalar maximum_advertised_routes("maximum_advertised_routes", 12) -> i64;
                scalar maximum_advertised_routes_warning_limit("maximum_advertised_routes_warning_limit", 13) -> &'a str;
                model maximum_accepted_routes("maximum_accepted_routes", 14) -> item::MaximumAcceptedRoutes<'a>;
            }
        }

        pub mod item {

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
                pub struct AdditionalPaths {
                    scalar prefix_list("prefix_list", 0) -> &'a str;
                    scalar receive("receive", 1) -> bool;
                    scalar send("send", 2) -> &'a str;
                    scalar send_limit("send_limit", 3) -> i64;
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
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Redistribute {
            model attached_host("attached_host", 0) -> redistribute::AttachedHost<'a>;
            model bgp("bgp", 1) -> redistribute::Bgp<'a>;
            model connected("connected", 2) -> redistribute::Connected<'a>;
            model dhcp("dhcp", 3) -> redistribute::Dhcp<'a>;
            model dynamic("dynamic", 4) -> redistribute::Dynamic<'a>;
            model isis("isis", 5) -> redistribute::Isis<'a>;
            model ospfv3("ospfv3", 6) -> redistribute::Ospfv3<'a>;
            model field_static("static", 7) -> redistribute::FieldStatic<'a>;
            model user("user", 8) -> redistribute::User<'a>;
        }
    }

    pub mod redistribute {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AttachedHost {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Bgp {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Connected {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
                scalar rcf("rcf", 2) -> &'a str;
                scalar include_leaked("include_leaked", 3) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Dhcp {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Dynamic {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
                scalar rcf("rcf", 2) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Isis {
                scalar enabled("enabled", 0) -> bool;
                scalar isis_level("isis_level", 1) -> &'a str;
                scalar route_map("route_map", 2) -> &'a str;
                scalar rcf("rcf", 3) -> &'a str;
                scalar include_leaked("include_leaked", 4) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ospfv3 {
                scalar enabled("enabled", 0) -> bool;
                model match_external("match_external", 1) -> ospfv3::MatchExternal<'a>;
                model match_internal("match_internal", 2) -> ospfv3::MatchInternal<'a>;
                model match_nssa_external("match_nssa_external", 3) -> ospfv3::MatchNssaExternal<'a>;
                scalar route_map("route_map", 4) -> &'a str;
                scalar include_leaked("include_leaked", 5) -> bool;
            }
        }

        pub mod ospfv3 {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MatchExternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                    scalar include_leaked("include_leaked", 2) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MatchInternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                    scalar include_leaked("include_leaked", 2) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MatchNssaExternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar nssa_type("nssa_type", 1) -> i64;
                    scalar route_map("route_map", 2) -> &'a str;
                    scalar include_leaked("include_leaked", 3) -> bool;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct FieldStatic {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
                scalar rcf("rcf", 2) -> &'a str;
                scalar include_leaked("include_leaked", 3) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct User {
                scalar enabled("enabled", 0) -> bool;
                scalar rcf("rcf", 1) -> &'a str;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct NextHop {
            scalar resolution_disabled("resolution_disabled", 0) -> bool;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AddressFamilyIpv6Multicast {
        model bgp("bgp", 0) -> address_family_ipv6_multicast::Bgp<'a>;
        model neighbors("neighbors", 1) -> address_family_ipv6_multicast::Neighbors<'a>;
        model peer_groups("peer_groups", 2) -> address_family_ipv6_multicast::PeerGroups<'a>;
        model networks("networks", 3) -> address_family_ipv6_multicast::Networks<'a>;
        model redistribute("redistribute", 4) -> address_family_ipv6_multicast::Redistribute<'a>;
    }
}

pub mod address_family_ipv6_multicast {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Bgp {
            model missing_policy("missing_policy", 0) -> bgp::MissingPolicy<'a>;
            model additional_paths("additional_paths", 1) -> bgp::AdditionalPaths<'a>;
        }
    }

    pub mod bgp {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct MissingPolicy {
                scalar direction_in_action("direction_in_action", 0) -> &'a str;
                scalar direction_out_action("direction_out_action", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AdditionalPaths {
                scalar receive("receive", 0) -> bool;
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Neighbors {
            model item (0) -> neighbors::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod neighbors {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar ip_address("ip_address", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                scalar route_map_in("route_map_in", 2) -> &'a str;
                scalar route_map_out("route_map_out", 3) -> &'a str;
                scalar peer_tag_in("peer_tag_in", 4) -> &'a str;
                scalar peer_tag_out_discard("peer_tag_out_discard", 5) -> &'a str;
                model additional_paths("additional_paths", 6) -> item::AdditionalPaths<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AdditionalPaths {
                    scalar receive("receive", 0) -> bool;
                }
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PeerGroups {
            model item (0) -> peer_groups::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod peer_groups {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                model additional_paths("additional_paths", 2) -> item::AdditionalPaths<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AdditionalPaths {
                    scalar receive("receive", 0) -> bool;
                }
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Networks {
            model item (0) -> networks::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod networks {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar prefix("prefix", 0) -> &'a str;
                scalar route_map("route_map", 1) -> &'a str;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Redistribute {
            model connected("connected", 0) -> redistribute::Connected<'a>;
            model isis("isis", 1) -> redistribute::Isis<'a>;
            model ospf("ospf", 2) -> redistribute::Ospf<'a>;
            model ospfv3("ospfv3", 3) -> redistribute::Ospfv3<'a>;
            model field_static("static", 4) -> redistribute::FieldStatic<'a>;
        }
    }

    pub mod redistribute {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Connected {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Isis {
                scalar enabled("enabled", 0) -> bool;
                scalar isis_level("isis_level", 1) -> &'a str;
                scalar route_map("route_map", 2) -> &'a str;
                scalar rcf("rcf", 3) -> &'a str;
                scalar include_leaked("include_leaked", 4) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ospf {
                scalar enabled("enabled", 0) -> bool;
                model match_external("match_external", 1) -> ospf::MatchExternal<'a>;
                model match_internal("match_internal", 2) -> ospf::MatchInternal<'a>;
                model match_nssa_external("match_nssa_external", 3) -> ospf::MatchNssaExternal<'a>;
                scalar route_map("route_map", 4) -> &'a str;
            }
        }

        pub mod ospf {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MatchExternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MatchInternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MatchNssaExternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar nssa_type("nssa_type", 1) -> i64;
                    scalar route_map("route_map", 2) -> &'a str;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ospfv3 {
                scalar enabled("enabled", 0) -> bool;
                model match_external("match_external", 1) -> ospfv3::MatchExternal<'a>;
                model match_internal("match_internal", 2) -> ospfv3::MatchInternal<'a>;
                model match_nssa_external("match_nssa_external", 3) -> ospfv3::MatchNssaExternal<'a>;
                scalar route_map("route_map", 4) -> &'a str;
            }
        }

        pub mod ospfv3 {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MatchExternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MatchInternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MatchNssaExternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar nssa_type("nssa_type", 1) -> i64;
                    scalar route_map("route_map", 2) -> &'a str;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct FieldStatic {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AddressFamilyIpv6SrTe {
        model neighbors("neighbors", 0) -> address_family_ipv6_sr_te::Neighbors<'a>;
        model peer_groups("peer_groups", 1) -> address_family_ipv6_sr_te::PeerGroups<'a>;
    }
}

pub mod address_family_ipv6_sr_te {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Neighbors {
            model item (0) -> neighbors::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod neighbors {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar ip_address("ip_address", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                scalar route_map_in("route_map_in", 2) -> &'a str;
                scalar route_map_out("route_map_out", 3) -> &'a str;
                scalar peer_tag_in("peer_tag_in", 4) -> &'a str;
                scalar peer_tag_out_discard("peer_tag_out_discard", 5) -> &'a str;
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PeerGroups {
            model item (0) -> peer_groups::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod peer_groups {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                scalar route_map_in("route_map_in", 2) -> &'a str;
                scalar route_map_out("route_map_out", 3) -> &'a str;
                scalar peer_tag_in("peer_tag_in", 4) -> &'a str;
                scalar peer_tag_out_discard("peer_tag_out_discard", 5) -> &'a str;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AddressFamilyLinkState {
        model bgp("bgp", 0) -> address_family_link_state::Bgp<'a>;
        model peer_groups("peer_groups", 1) -> address_family_link_state::PeerGroups<'a>;
        model neighbors("neighbors", 2) -> address_family_link_state::Neighbors<'a>;
        model path_selection("path_selection", 3) -> address_family_link_state::PathSelection<'a>;
    }
}

pub mod address_family_link_state {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Bgp {
            model missing_policy("missing_policy", 0) -> bgp::MissingPolicy<'a>;
        }
    }

    pub mod bgp {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct MissingPolicy {
                scalar direction_in_action("direction_in_action", 0) -> &'a str;
                scalar direction_out_action("direction_out_action", 1) -> &'a str;
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PeerGroups {
            model item (0) -> peer_groups::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod peer_groups {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                model missing_policy("missing_policy", 2) -> item::MissingPolicy<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MissingPolicy {
                    scalar direction_in_action("direction_in_action", 0) -> &'a str;
                    scalar direction_out_action("direction_out_action", 1) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Neighbors {
            model item (0) -> neighbors::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod neighbors {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar ip_address("ip_address", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                model missing_policy("missing_policy", 2) -> item::MissingPolicy<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct MissingPolicy {
                    scalar direction_in_action("direction_in_action", 0) -> &'a str;
                    scalar direction_out_action("direction_out_action", 1) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PathSelection {
            model roles("roles", 0) -> path_selection::Roles<'a>;
        }
    }

    pub mod path_selection {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Roles {
                scalar producer("producer", 0) -> bool;
                scalar consumer("consumer", 1) -> bool;
                scalar propagator("propagator", 2) -> bool;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AddressFamilyFlowSpecIpv4 {
        model bgp("bgp", 0) -> address_family_flow_spec_ipv4::Bgp<'a>;
        model neighbors("neighbors", 1) -> address_family_flow_spec_ipv4::Neighbors<'a>;
        model peer_groups("peer_groups", 2) -> address_family_flow_spec_ipv4::PeerGroups<'a>;
    }
}

pub mod address_family_flow_spec_ipv4 {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Bgp {
            model missing_policy("missing_policy", 0) -> bgp::MissingPolicy<'a>;
        }
    }

    pub mod bgp {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct MissingPolicy {
                scalar direction_in_action("direction_in_action", 0) -> &'a str;
                scalar direction_out_action("direction_out_action", 1) -> &'a str;
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Neighbors {
            model item (0) -> neighbors::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod neighbors {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar ip_address("ip_address", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PeerGroups {
            model item (0) -> peer_groups::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod peer_groups {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AddressFamilyFlowSpecIpv6 {
        model bgp("bgp", 0) -> address_family_flow_spec_ipv6::Bgp<'a>;
        model neighbors("neighbors", 1) -> address_family_flow_spec_ipv6::Neighbors<'a>;
        model peer_groups("peer_groups", 2) -> address_family_flow_spec_ipv6::PeerGroups<'a>;
    }
}

pub mod address_family_flow_spec_ipv6 {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Bgp {
            model missing_policy("missing_policy", 0) -> bgp::MissingPolicy<'a>;
        }
    }

    pub mod bgp {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct MissingPolicy {
                scalar direction_in_action("direction_in_action", 0) -> &'a str;
                scalar direction_out_action("direction_out_action", 1) -> &'a str;
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Neighbors {
            model item (0) -> neighbors::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod neighbors {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar ip_address("ip_address", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PeerGroups {
            model item (0) -> peer_groups::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod peer_groups {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AddressFamilyPathSelection {
        model bgp("bgp", 0) -> address_family_path_selection::Bgp<'a>;
        model neighbors("neighbors", 1) -> address_family_path_selection::Neighbors<'a>;
        model peer_groups("peer_groups", 2) -> address_family_path_selection::PeerGroups<'a>;
    }
}

pub mod address_family_path_selection {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Bgp {
            model additional_paths("additional_paths", 0) -> bgp::AdditionalPaths<'a>;
        }
    }

    pub mod bgp {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AdditionalPaths {
                scalar receive("receive", 0) -> bool;
                scalar send("send", 1) -> &'a str;
                scalar send_limit("send_limit", 2) -> i64;
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Neighbors {
            model item (0) -> neighbors::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod neighbors {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar ip_address("ip_address", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                model additional_paths("additional_paths", 2) -> item::AdditionalPaths<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AdditionalPaths {
                    scalar receive("receive", 0) -> bool;
                    scalar send("send", 1) -> &'a str;
                    scalar send_limit("send_limit", 2) -> i64;
                }
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PeerGroups {
            model item (0) -> peer_groups::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod peer_groups {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                model additional_paths("additional_paths", 2) -> item::AdditionalPaths<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AdditionalPaths {
                    scalar receive("receive", 0) -> bool;
                    scalar send("send", 1) -> &'a str;
                    scalar send_limit("send_limit", 2) -> i64;
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AddressFamilyVpnIpv4 {
        scalar domain_identifier("domain_identifier", 0) -> &'a str;
        model peer_groups("peer_groups", 1) -> address_family_vpn_ipv4::PeerGroups<'a>;
        model route("route", 2) -> address_family_vpn_ipv4::Route<'a>;
        model neighbors("neighbors", 3) -> address_family_vpn_ipv4::Neighbors<'a>;
        model neighbor_default_encapsulation_mpls_next_hop_self("neighbor_default_encapsulation_mpls_next_hop_self", 4) -> address_family_vpn_ipv4::NeighborDefaultEncapsulationMplsNextHopSelf<'a>;
        model next_hop("next_hop", 5) -> address_family_vpn_ipv4::NextHop<'a>;
    }
}

pub mod address_family_vpn_ipv4 {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PeerGroups {
            model item (0) -> peer_groups::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod peer_groups {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                scalar route_map_in("route_map_in", 2) -> &'a str;
                scalar route_map_out("route_map_out", 3) -> &'a str;
                scalar peer_tag_in("peer_tag_in", 4) -> &'a str;
                scalar peer_tag_out_discard("peer_tag_out_discard", 5) -> &'a str;
                scalar rcf_in("rcf_in", 6) -> &'a str;
                scalar rcf_out("rcf_out", 7) -> &'a str;
                model default_route("default_route", 8) -> item::DefaultRoute<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct DefaultRoute {
                    scalar enabled("enabled", 0) -> bool;
                    scalar rcf("rcf", 1) -> &'a str;
                    scalar route_map("route_map", 2) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Route {
            scalar import_match_failure_action("import_match_failure_action", 0) -> &'a str;
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Neighbors {
            model item (0) -> neighbors::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod neighbors {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar ip_address("ip_address", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                scalar route_map_in("route_map_in", 2) -> &'a str;
                scalar route_map_out("route_map_out", 3) -> &'a str;
                scalar peer_tag_in("peer_tag_in", 4) -> &'a str;
                scalar peer_tag_out_discard("peer_tag_out_discard", 5) -> &'a str;
                scalar rcf_in("rcf_in", 6) -> &'a str;
                scalar rcf_out("rcf_out", 7) -> &'a str;
                model default_route("default_route", 8) -> item::DefaultRoute<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct DefaultRoute {
                    scalar enabled("enabled", 0) -> bool;
                    scalar rcf("rcf", 1) -> &'a str;
                    scalar route_map("route_map", 2) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct NeighborDefaultEncapsulationMplsNextHopSelf {
            scalar source_interface("source_interface", 0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct NextHop {
            scalar resolution_disabled("resolution_disabled", 0) -> bool;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AddressFamilyVpnIpv6 {
        scalar domain_identifier("domain_identifier", 0) -> &'a str;
        model peer_groups("peer_groups", 1) -> address_family_vpn_ipv6::PeerGroups<'a>;
        model route("route", 2) -> address_family_vpn_ipv6::Route<'a>;
        model neighbors("neighbors", 3) -> address_family_vpn_ipv6::Neighbors<'a>;
        model neighbor_default_encapsulation_mpls_next_hop_self("neighbor_default_encapsulation_mpls_next_hop_self", 4) -> address_family_vpn_ipv6::NeighborDefaultEncapsulationMplsNextHopSelf<'a>;
        model next_hop("next_hop", 5) -> address_family_vpn_ipv6::NextHop<'a>;
    }
}

pub mod address_family_vpn_ipv6 {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PeerGroups {
            model item (0) -> peer_groups::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod peer_groups {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                scalar route_map_in("route_map_in", 2) -> &'a str;
                scalar route_map_out("route_map_out", 3) -> &'a str;
                scalar peer_tag_in("peer_tag_in", 4) -> &'a str;
                scalar peer_tag_out_discard("peer_tag_out_discard", 5) -> &'a str;
                scalar rcf_in("rcf_in", 6) -> &'a str;
                scalar rcf_out("rcf_out", 7) -> &'a str;
                model default_route("default_route", 8) -> item::DefaultRoute<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct DefaultRoute {
                    scalar enabled("enabled", 0) -> bool;
                    scalar rcf("rcf", 1) -> &'a str;
                    scalar route_map("route_map", 2) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Route {
            scalar import_match_failure_action("import_match_failure_action", 0) -> &'a str;
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Neighbors {
            model item (0) -> neighbors::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod neighbors {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar ip_address("ip_address", 0) -> &'a str;
                scalar activate("activate", 1) -> bool;
                scalar route_map_in("route_map_in", 2) -> &'a str;
                scalar route_map_out("route_map_out", 3) -> &'a str;
                scalar peer_tag_in("peer_tag_in", 4) -> &'a str;
                scalar peer_tag_out_discard("peer_tag_out_discard", 5) -> &'a str;
                scalar rcf_in("rcf_in", 6) -> &'a str;
                scalar rcf_out("rcf_out", 7) -> &'a str;
                model default_route("default_route", 8) -> item::DefaultRoute<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct DefaultRoute {
                    scalar enabled("enabled", 0) -> bool;
                    scalar rcf("rcf", 1) -> &'a str;
                    scalar route_map("route_map", 2) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct NeighborDefaultEncapsulationMplsNextHopSelf {
            scalar source_interface("source_interface", 0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct NextHop {
            scalar resolution_disabled("resolution_disabled", 0) -> bool;
        }
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
            model bgp("bgp", 1) -> item::Bgp<'a>;
            scalar rd("rd", 2) -> &'a str;
            model rd_evpn_domain("rd_evpn_domain", 3) -> item::RdEvpnDomain<'a>;
            scalar evpn_multicast("evpn_multicast", 4) -> bool;
            model evpn_multicast_address_family("evpn_multicast_address_family", 5) -> item::EvpnMulticastAddressFamily<'a>;
            model evpn_multicast_gateway_dr_election("evpn_multicast_gateway_dr_election", 6) -> item::EvpnMulticastGatewayDrElection<'a>;
            model default_route_exports("default_route_exports", 7) -> item::DefaultRouteExports<'a>;
            model route_targets("route_targets", 8) -> item::RouteTargets<'a>;
            scalar router_id("router_id", 9) -> &'a str;
            scalar timers("timers", 10) -> &'a str;
            model graceful_restart("graceful_restart", 11) -> item::GracefulRestart<'a>;
            scalar no_graceful_restart("no_graceful_restart", 12) -> bool;
            model networks("networks", 13) -> item::Networks<'a>;
            model maximum_paths("maximum_paths", 14) -> item::MaximumPaths<'a>;
            model updates("updates", 15) -> item::Updates<'a>;
            model listen_ranges("listen_ranges", 16) -> item::ListenRanges<'a>;
            model neighbors("neighbors", 17) -> item::Neighbors<'a>;
            model neighbor_interfaces("neighbor_interfaces", 18) -> item::NeighborInterfaces<'a>;
            model redistribute("redistribute", 19) -> item::Redistribute<'a>;
            model aggregate_addresses("aggregate_addresses", 20) -> item::AggregateAddresses<'a>;
            model address_family_ipv4("address_family_ipv4", 21) -> item::AddressFamilyIpv4<'a>;
            model address_family_ipv6("address_family_ipv6", 22) -> item::AddressFamilyIpv6<'a>;
            model address_family_ipv4_multicast("address_family_ipv4_multicast", 23) -> item::AddressFamilyIpv4Multicast<'a>;
            model address_family_ipv6_multicast("address_family_ipv6_multicast", 24) -> item::AddressFamilyIpv6Multicast<'a>;
            model address_family_flow_spec_ipv4("address_family_flow_spec_ipv4", 25) -> item::AddressFamilyFlowSpecIpv4<'a>;
            model address_family_flow_spec_ipv6("address_family_flow_spec_ipv6", 26) -> item::AddressFamilyFlowSpecIpv6<'a>;
            scalar eos_cli("eos_cli", 27) -> &'a str;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Bgp {
                scalar redistribute_internal("redistribute_internal", 0) -> bool;
                model additional_paths("additional_paths", 1) -> bgp::AdditionalPaths<'a>;
            }
        }

        pub mod bgp {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AdditionalPaths {
                    scalar install("install", 0) -> bool;
                    scalar install_ecmp_primary("install_ecmp_primary", 1) -> bool;
                    scalar receive("receive", 2) -> bool;
                    scalar send("send", 3) -> &'a str;
                    scalar send_limit("send_limit", 4) -> i64;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct RdEvpnDomain {
                scalar domain("domain", 0) -> &'a str;
                scalar rd("rd", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct EvpnMulticastAddressFamily {
                model ipv4("ipv4", 0) -> evpn_multicast_address_family::Ipv4<'a>;
            }
        }

        pub mod evpn_multicast_address_family {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Ipv4 {
                    scalar transit("transit", 0) -> bool;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct EvpnMulticastGatewayDrElection {
                scalar algorithm("algorithm", 0) -> &'a str;
                scalar preference_value("preference_value", 1) -> i64;
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct DefaultRouteExports {
                model item (0) -> default_route_exports::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod default_route_exports {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar address_family("address_family", 0) -> &'a str;
                    scalar always("always", 1) -> bool;
                    scalar route_map("route_map", 2) -> &'a str;
                    scalar rcf("rcf", 3) -> &'a str;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct RouteTargets {
                model field_import("import", 0) -> route_targets::Import<'a>;
                model export("export", 1) -> route_targets::Export<'a>;
                model import_evpn_domains("import_evpn_domains", 2) -> route_targets::ImportEvpnDomains<'a>;
                model export_evpn_domains("export_evpn_domains", 3) -> route_targets::ExportEvpnDomains<'a>;
            }
        }

        pub mod route_targets {

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Import {
                    model item (0) -> import::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod import {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar address_family("address_family", 0) -> &'a str;
                        model route_targets("route_targets", 1) -> item::RouteTargets<'a>;
                        scalar route_map("route_map", 2) -> &'a str;
                        scalar rcf("rcf", 3) -> &'a str;
                        scalar vpn_route_filter_rcf("vpn_route_filter_rcf", 4) -> &'a str;
                    }
                }

                pub mod item {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct RouteTargets {
                            scalar item (0) -> &'a str;
                        }
                    }
                }
            }

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Export {
                    model item (0) -> export::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod export {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar address_family("address_family", 0) -> &'a str;
                        model route_targets("route_targets", 1) -> item::RouteTargets<'a>;
                        scalar route_map("route_map", 2) -> &'a str;
                        scalar rcf("rcf", 3) -> &'a str;
                        scalar vrf_route_filter_rcf("vrf_route_filter_rcf", 4) -> &'a str;
                    }
                }

                pub mod item {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct RouteTargets {
                            scalar item (0) -> &'a str;
                        }
                    }
                }
            }

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct ImportEvpnDomains {
                    model item (0) -> import_evpn_domains::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod import_evpn_domains {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar route_target("route_target", 0) -> &'a str;
                        scalar domain("domain", 1) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct ExportEvpnDomains {
                    model item (0) -> export_evpn_domains::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod export_evpn_domains {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar route_target("route_target", 0) -> &'a str;
                        scalar domain("domain", 1) -> &'a str;
                    }
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct GracefulRestart {
                scalar enabled("enabled", 0) -> bool;
                scalar restart_time("restart_time", 1) -> i64;
                scalar stalepath_time("stalepath_time", 2) -> i64;
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Networks {
                model item (0) -> networks::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod networks {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar prefix("prefix", 0) -> &'a str;
                    scalar route_map("route_map", 1) -> &'a str;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct MaximumPaths {
                scalar paths("paths", 0) -> i64;
                scalar ecmp("ecmp", 1) -> i64;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Updates {
                scalar wait_for_convergence("wait_for_convergence", 0) -> bool;
                scalar wait_install("wait_install", 1) -> bool;
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
                    scalar peer_id_include_router_id("peer_id_include_router_id", 1) -> bool;
                    scalar peer_group("peer_group", 2) -> &'a str;
                    scalar peer_filter("peer_filter", 3) -> &'a str;
                    scalar remote_as("remote_as", 4) -> &'a str;
                }
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Neighbors {
                model item (0) -> neighbors::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod neighbors {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar ip_address("ip_address", 0) -> &'a str;
                    scalar peer_group("peer_group", 1) -> &'a str;
                    scalar remote_as("remote_as", 2) -> &'a str;
                    scalar password("password", 3) -> &'a str;
                    scalar password_type("password_type", 4) -> &'a str;
                    scalar passive("passive", 5) -> bool;
                    model remove_private_as("remove_private_as", 6) -> item::RemovePrivateAs<'a>;
                    model remove_private_as_ingress("remove_private_as_ingress", 7) -> item::RemovePrivateAsIngress<'a>;
                    scalar weight("weight", 8) -> i64;
                    scalar local_as("local_as", 9) -> &'a str;
                    model as_path("as_path", 10) -> item::AsPath<'a>;
                    scalar description("description", 11) -> &'a str;
                    scalar route_reflector_client("route_reflector_client", 12) -> bool;
                    scalar ebgp_multihop("ebgp_multihop", 13) -> i64;
                    scalar next_hop_peer("next_hop_peer", 14) -> bool;
                    scalar next_hop_self("next_hop_self", 15) -> bool;
                    scalar shutdown("shutdown", 16) -> bool;
                    scalar bfd("bfd", 17) -> bool;
                    model bfd_timers("bfd_timers", 18) -> item::BfdTimers<'a>;
                    scalar timers("timers", 19) -> &'a str;
                    model rib_in_pre_policy_retain("rib_in_pre_policy_retain", 20) -> item::RibInPrePolicyRetain<'a>;
                    scalar send_community("send_community", 21) -> &'a str;
                    scalar maximum_routes("maximum_routes", 22) -> i64;
                    scalar maximum_routes_warning_limit("maximum_routes_warning_limit", 23) -> &'a str;
                    scalar maximum_routes_warning_only("maximum_routes_warning_only", 24) -> bool;
                    model maximum_accepted_routes("maximum_accepted_routes", 25) -> item::MaximumAcceptedRoutes<'a>;
                    model allowas_in("allowas_in", 26) -> item::AllowasIn<'a>;
                    model default_originate("default_originate", 27) -> item::DefaultOriginate<'a>;
                    scalar enforce_first_as("enforce_first_as", 28) -> bool;
                    scalar update_source("update_source", 29) -> &'a str;
                    scalar route_map_in("route_map_in", 30) -> &'a str;
                    scalar route_map_out("route_map_out", 31) -> &'a str;
                    scalar peer_tag_in("peer_tag_in", 32) -> &'a str;
                    scalar peer_tag_out_discard("peer_tag_out_discard", 33) -> &'a str;
                    model additional_paths("additional_paths", 34) -> item::AdditionalPaths<'a>;
                    model metadata("metadata", 35) -> item::Metadata<'a>;
                }
            }

            pub mod item {

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
                    pub struct AsPath {
                        scalar remote_as_replace_out("remote_as_replace_out", 0) -> bool;
                        scalar prepend_own_disabled("prepend_own_disabled", 1) -> bool;
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
                    pub struct RibInPrePolicyRetain {
                        scalar enabled("enabled", 0) -> bool;
                        scalar all("all", 1) -> bool;
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
                    pub struct AllowasIn {
                        scalar enabled("enabled", 0) -> bool;
                        scalar times("times", 1) -> i64;
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
                    pub struct AdditionalPaths {
                        scalar receive("receive", 0) -> bool;
                        scalar send("send", 1) -> &'a str;
                        scalar send_limit("send_limit", 2) -> i64;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Metadata {
                        scalar validate_state("validate_state", 0) -> bool;
                    }
                }
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct NeighborInterfaces {
                model item (0) -> neighbor_interfaces::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod neighbor_interfaces {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    scalar remote_as("remote_as", 1) -> &'a str;
                    scalar peer_group("peer_group", 2) -> &'a str;
                    scalar peer_filter("peer_filter", 3) -> &'a str;
                    scalar description("description", 4) -> &'a str;
                    model metadata("metadata", 5) -> item::Metadata<'a>;
                }
            }

            pub mod item {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Metadata {
                        scalar validate_state("validate_state", 0) -> bool;
                    }
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Redistribute {
                model attached_host("attached_host", 0) -> redistribute::AttachedHost<'a>;
                model bgp("bgp", 1) -> redistribute::Bgp<'a>;
                model connected("connected", 2) -> redistribute::Connected<'a>;
                model dynamic("dynamic", 3) -> redistribute::Dynamic<'a>;
                model isis("isis", 4) -> redistribute::Isis<'a>;
                model ospf("ospf", 5) -> redistribute::Ospf<'a>;
                model ospfv3("ospfv3", 6) -> redistribute::Ospfv3<'a>;
                model rip("rip", 7) -> redistribute::Rip<'a>;
                model field_static("static", 8) -> redistribute::FieldStatic<'a>;
                model user("user", 9) -> redistribute::User<'a>;
            }
        }

        pub mod redistribute {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AttachedHost {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Bgp {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Connected {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                    scalar rcf("rcf", 2) -> &'a str;
                    scalar include_leaked("include_leaked", 3) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Dynamic {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                    scalar rcf("rcf", 2) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Isis {
                    scalar enabled("enabled", 0) -> bool;
                    scalar isis_level("isis_level", 1) -> &'a str;
                    scalar route_map("route_map", 2) -> &'a str;
                    scalar rcf("rcf", 3) -> &'a str;
                    scalar include_leaked("include_leaked", 4) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Ospf {
                    scalar enabled("enabled", 0) -> bool;
                    model match_external("match_external", 1) -> ospf::MatchExternal<'a>;
                    model match_internal("match_internal", 2) -> ospf::MatchInternal<'a>;
                    model match_nssa_external("match_nssa_external", 3) -> ospf::MatchNssaExternal<'a>;
                    scalar route_map("route_map", 4) -> &'a str;
                    scalar include_leaked("include_leaked", 5) -> bool;
                }
            }

            pub mod ospf {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct MatchExternal {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                        scalar include_leaked("include_leaked", 2) -> bool;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct MatchInternal {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                        scalar include_leaked("include_leaked", 2) -> bool;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct MatchNssaExternal {
                        scalar enabled("enabled", 0) -> bool;
                        scalar nssa_type("nssa_type", 1) -> i64;
                        scalar route_map("route_map", 2) -> &'a str;
                        scalar include_leaked("include_leaked", 3) -> bool;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Ospfv3 {
                    scalar enabled("enabled", 0) -> bool;
                    model match_external("match_external", 1) -> ospfv3::MatchExternal<'a>;
                    model match_internal("match_internal", 2) -> ospfv3::MatchInternal<'a>;
                    model match_nssa_external("match_nssa_external", 3) -> ospfv3::MatchNssaExternal<'a>;
                    scalar route_map("route_map", 4) -> &'a str;
                    scalar include_leaked("include_leaked", 5) -> bool;
                }
            }

            pub mod ospfv3 {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct MatchExternal {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                        scalar include_leaked("include_leaked", 2) -> bool;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct MatchInternal {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                        scalar include_leaked("include_leaked", 2) -> bool;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct MatchNssaExternal {
                        scalar enabled("enabled", 0) -> bool;
                        scalar nssa_type("nssa_type", 1) -> i64;
                        scalar route_map("route_map", 2) -> &'a str;
                        scalar include_leaked("include_leaked", 3) -> bool;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Rip {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct FieldStatic {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                    scalar rcf("rcf", 2) -> &'a str;
                    scalar include_leaked("include_leaked", 3) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct User {
                    scalar enabled("enabled", 0) -> bool;
                    scalar rcf("rcf", 1) -> &'a str;
                }
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AggregateAddresses {
                model item (0) -> aggregate_addresses::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod aggregate_addresses {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar prefix("prefix", 0) -> &'a str;
                    scalar advertise_only("advertise_only", 1) -> bool;
                    scalar as_set("as_set", 2) -> bool;
                    scalar summary_only("summary_only", 3) -> bool;
                    scalar attribute_map("attribute_map", 4) -> &'a str;
                    scalar match_map("match_map", 5) -> &'a str;
                    model attribute("attribute", 6) -> item::Attribute<'a>;
                }
            }

            pub mod item {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Attribute {
                        scalar rcf("rcf", 0) -> &'a str;
                    }
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AddressFamilyIpv4 {
                model bgp("bgp", 0) -> address_family_ipv4::Bgp<'a>;
                model neighbors("neighbors", 1) -> address_family_ipv4::Neighbors<'a>;
                model networks("networks", 2) -> address_family_ipv4::Networks<'a>;
                model redistribute("redistribute", 3) -> address_family_ipv4::Redistribute<'a>;
            }
        }

        pub mod address_family_ipv4 {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Bgp {
                    model missing_policy("missing_policy", 0) -> bgp::MissingPolicy<'a>;
                    model additional_paths("additional_paths", 1) -> bgp::AdditionalPaths<'a>;
                    scalar redistribute_internal("redistribute_internal", 2) -> bool;
                }
            }

            pub mod bgp {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct MissingPolicy {
                        scalar direction_in_action("direction_in_action", 0) -> &'a str;
                        scalar direction_out_action("direction_out_action", 1) -> &'a str;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct AdditionalPaths {
                        scalar install("install", 0) -> bool;
                        scalar install_ecmp_primary("install_ecmp_primary", 1) -> bool;
                        scalar receive("receive", 2) -> bool;
                        scalar send("send", 3) -> &'a str;
                        scalar send_limit("send_limit", 4) -> i64;
                    }
                }
            }

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Neighbors {
                    model item (0) -> neighbors::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod neighbors {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar ip_address("ip_address", 0) -> &'a str;
                        scalar activate("activate", 1) -> bool;
                        scalar route_map_in("route_map_in", 2) -> &'a str;
                        scalar route_map_out("route_map_out", 3) -> &'a str;
                        scalar peer_tag_in("peer_tag_in", 4) -> &'a str;
                        scalar peer_tag_out_discard("peer_tag_out_discard", 5) -> &'a str;
                        scalar rcf_in("rcf_in", 6) -> &'a str;
                        scalar rcf_out("rcf_out", 7) -> &'a str;
                        scalar prefix_list_in("prefix_list_in", 8) -> &'a str;
                        scalar prefix_list_out("prefix_list_out", 9) -> &'a str;
                        model next_hop("next_hop", 10) -> item::NextHop<'a>;
                        model additional_paths("additional_paths", 11) -> item::AdditionalPaths<'a>;
                    }
                }

                pub mod item {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct NextHop {
                            model address_family_ipv6("address_family_ipv6", 0) -> next_hop::AddressFamilyIpv6<'a>;
                        }
                    }

                    pub mod next_hop {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct AddressFamilyIpv6 {
                                scalar enabled("enabled", 0) -> bool;
                                scalar originate("originate", 1) -> bool;
                            }
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct AdditionalPaths {
                            scalar receive("receive", 0) -> bool;
                            scalar send("send", 1) -> &'a str;
                            scalar send_limit("send_limit", 2) -> i64;
                        }
                    }
                }
            }

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Networks {
                    model item (0) -> networks::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod networks {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar prefix("prefix", 0) -> &'a str;
                        scalar route_map("route_map", 1) -> &'a str;
                        scalar rcf("rcf", 2) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Redistribute {
                    model attached_host("attached_host", 0) -> redistribute::AttachedHost<'a>;
                    model bgp("bgp", 1) -> redistribute::Bgp<'a>;
                    model connected("connected", 2) -> redistribute::Connected<'a>;
                    model dynamic("dynamic", 3) -> redistribute::Dynamic<'a>;
                    model isis("isis", 4) -> redistribute::Isis<'a>;
                    model ospf("ospf", 5) -> redistribute::Ospf<'a>;
                    model ospfv3("ospfv3", 6) -> redistribute::Ospfv3<'a>;
                    model rip("rip", 7) -> redistribute::Rip<'a>;
                    model field_static("static", 8) -> redistribute::FieldStatic<'a>;
                    model user("user", 9) -> redistribute::User<'a>;
                }
            }

            pub mod redistribute {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct AttachedHost {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Bgp {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Connected {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                        scalar rcf("rcf", 2) -> &'a str;
                        scalar include_leaked("include_leaked", 3) -> bool;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Dynamic {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                        scalar rcf("rcf", 2) -> &'a str;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Isis {
                        scalar enabled("enabled", 0) -> bool;
                        scalar isis_level("isis_level", 1) -> &'a str;
                        scalar route_map("route_map", 2) -> &'a str;
                        scalar rcf("rcf", 3) -> &'a str;
                        scalar include_leaked("include_leaked", 4) -> bool;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Ospf {
                        scalar enabled("enabled", 0) -> bool;
                        model match_external("match_external", 1) -> ospf::MatchExternal<'a>;
                        model match_internal("match_internal", 2) -> ospf::MatchInternal<'a>;
                        model match_nssa_external("match_nssa_external", 3) -> ospf::MatchNssaExternal<'a>;
                        scalar route_map("route_map", 4) -> &'a str;
                        scalar include_leaked("include_leaked", 5) -> bool;
                    }
                }

                pub mod ospf {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MatchExternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar route_map("route_map", 1) -> &'a str;
                            scalar include_leaked("include_leaked", 2) -> bool;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MatchInternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar route_map("route_map", 1) -> &'a str;
                            scalar include_leaked("include_leaked", 2) -> bool;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MatchNssaExternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar nssa_type("nssa_type", 1) -> i64;
                            scalar route_map("route_map", 2) -> &'a str;
                            scalar include_leaked("include_leaked", 3) -> bool;
                        }
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Ospfv3 {
                        scalar enabled("enabled", 0) -> bool;
                        model match_external("match_external", 1) -> ospfv3::MatchExternal<'a>;
                        model match_internal("match_internal", 2) -> ospfv3::MatchInternal<'a>;
                        model match_nssa_external("match_nssa_external", 3) -> ospfv3::MatchNssaExternal<'a>;
                        scalar route_map("route_map", 4) -> &'a str;
                        scalar include_leaked("include_leaked", 5) -> bool;
                    }
                }

                pub mod ospfv3 {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MatchExternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar route_map("route_map", 1) -> &'a str;
                            scalar include_leaked("include_leaked", 2) -> bool;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MatchInternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar route_map("route_map", 1) -> &'a str;
                            scalar include_leaked("include_leaked", 2) -> bool;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MatchNssaExternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar nssa_type("nssa_type", 1) -> i64;
                            scalar route_map("route_map", 2) -> &'a str;
                            scalar include_leaked("include_leaked", 3) -> bool;
                        }
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Rip {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct FieldStatic {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                        scalar rcf("rcf", 2) -> &'a str;
                        scalar include_leaked("include_leaked", 3) -> bool;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct User {
                        scalar enabled("enabled", 0) -> bool;
                        scalar rcf("rcf", 1) -> &'a str;
                    }
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AddressFamilyIpv6 {
                model bgp("bgp", 0) -> address_family_ipv6::Bgp<'a>;
                model neighbors("neighbors", 1) -> address_family_ipv6::Neighbors<'a>;
                model networks("networks", 2) -> address_family_ipv6::Networks<'a>;
                model redistribute("redistribute", 3) -> address_family_ipv6::Redistribute<'a>;
            }
        }

        pub mod address_family_ipv6 {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Bgp {
                    model missing_policy("missing_policy", 0) -> bgp::MissingPolicy<'a>;
                    model additional_paths("additional_paths", 1) -> bgp::AdditionalPaths<'a>;
                    scalar redistribute_internal("redistribute_internal", 2) -> bool;
                }
            }

            pub mod bgp {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct MissingPolicy {
                        scalar direction_in_action("direction_in_action", 0) -> &'a str;
                        scalar direction_out_action("direction_out_action", 1) -> &'a str;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct AdditionalPaths {
                        scalar install("install", 0) -> bool;
                        scalar install_ecmp_primary("install_ecmp_primary", 1) -> bool;
                        scalar receive("receive", 2) -> bool;
                        scalar send("send", 3) -> &'a str;
                        scalar send_limit("send_limit", 4) -> i64;
                    }
                }
            }

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Neighbors {
                    model item (0) -> neighbors::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod neighbors {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar ip_address("ip_address", 0) -> &'a str;
                        scalar activate("activate", 1) -> bool;
                        scalar route_map_in("route_map_in", 2) -> &'a str;
                        scalar route_map_out("route_map_out", 3) -> &'a str;
                        scalar peer_tag_in("peer_tag_in", 4) -> &'a str;
                        scalar peer_tag_out_discard("peer_tag_out_discard", 5) -> &'a str;
                        scalar rcf_in("rcf_in", 6) -> &'a str;
                        scalar rcf_out("rcf_out", 7) -> &'a str;
                        scalar prefix_list_in("prefix_list_in", 8) -> &'a str;
                        scalar prefix_list_out("prefix_list_out", 9) -> &'a str;
                        model additional_paths("additional_paths", 10) -> item::AdditionalPaths<'a>;
                    }
                }

                pub mod item {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct AdditionalPaths {
                            scalar receive("receive", 0) -> bool;
                            scalar send("send", 1) -> &'a str;
                            scalar send_limit("send_limit", 2) -> i64;
                        }
                    }
                }
            }

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Networks {
                    model item (0) -> networks::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod networks {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar prefix("prefix", 0) -> &'a str;
                        scalar route_map("route_map", 1) -> &'a str;
                        scalar rcf("rcf", 2) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Redistribute {
                    model attached_host("attached_host", 0) -> redistribute::AttachedHost<'a>;
                    model bgp("bgp", 1) -> redistribute::Bgp<'a>;
                    model connected("connected", 2) -> redistribute::Connected<'a>;
                    model dhcp("dhcp", 3) -> redistribute::Dhcp<'a>;
                    model dynamic("dynamic", 4) -> redistribute::Dynamic<'a>;
                    model isis("isis", 5) -> redistribute::Isis<'a>;
                    model ospfv3("ospfv3", 6) -> redistribute::Ospfv3<'a>;
                    model field_static("static", 7) -> redistribute::FieldStatic<'a>;
                    model user("user", 8) -> redistribute::User<'a>;
                }
            }

            pub mod redistribute {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct AttachedHost {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Bgp {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Connected {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                        scalar rcf("rcf", 2) -> &'a str;
                        scalar include_leaked("include_leaked", 3) -> bool;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Dhcp {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Dynamic {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                        scalar rcf("rcf", 2) -> &'a str;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Isis {
                        scalar enabled("enabled", 0) -> bool;
                        scalar isis_level("isis_level", 1) -> &'a str;
                        scalar route_map("route_map", 2) -> &'a str;
                        scalar rcf("rcf", 3) -> &'a str;
                        scalar include_leaked("include_leaked", 4) -> bool;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Ospfv3 {
                        scalar enabled("enabled", 0) -> bool;
                        model match_external("match_external", 1) -> ospfv3::MatchExternal<'a>;
                        model match_internal("match_internal", 2) -> ospfv3::MatchInternal<'a>;
                        model match_nssa_external("match_nssa_external", 3) -> ospfv3::MatchNssaExternal<'a>;
                        scalar route_map("route_map", 4) -> &'a str;
                        scalar include_leaked("include_leaked", 5) -> bool;
                    }
                }

                pub mod ospfv3 {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MatchExternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar route_map("route_map", 1) -> &'a str;
                            scalar include_leaked("include_leaked", 2) -> bool;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MatchInternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar route_map("route_map", 1) -> &'a str;
                            scalar include_leaked("include_leaked", 2) -> bool;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MatchNssaExternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar nssa_type("nssa_type", 1) -> i64;
                            scalar route_map("route_map", 2) -> &'a str;
                            scalar include_leaked("include_leaked", 3) -> bool;
                        }
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct FieldStatic {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                        scalar rcf("rcf", 2) -> &'a str;
                        scalar include_leaked("include_leaked", 3) -> bool;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct User {
                        scalar enabled("enabled", 0) -> bool;
                        scalar rcf("rcf", 1) -> &'a str;
                    }
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AddressFamilyIpv4Multicast {
                model bgp("bgp", 0) -> address_family_ipv4_multicast::Bgp<'a>;
                model neighbors("neighbors", 1) -> address_family_ipv4_multicast::Neighbors<'a>;
                model networks("networks", 2) -> address_family_ipv4_multicast::Networks<'a>;
                model redistribute("redistribute", 3) -> address_family_ipv4_multicast::Redistribute<'a>;
            }
        }

        pub mod address_family_ipv4_multicast {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Bgp {
                    model missing_policy("missing_policy", 0) -> bgp::MissingPolicy<'a>;
                    model additional_paths("additional_paths", 1) -> bgp::AdditionalPaths<'a>;
                }
            }

            pub mod bgp {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct MissingPolicy {
                        scalar direction_in_action("direction_in_action", 0) -> &'a str;
                        scalar direction_out_action("direction_out_action", 1) -> &'a str;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct AdditionalPaths {
                        scalar receive("receive", 0) -> bool;
                    }
                }
            }

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Neighbors {
                    model item (0) -> neighbors::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod neighbors {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar ip_address("ip_address", 0) -> &'a str;
                        scalar activate("activate", 1) -> bool;
                        scalar route_map_in("route_map_in", 2) -> &'a str;
                        scalar route_map_out("route_map_out", 3) -> &'a str;
                        scalar peer_tag_in("peer_tag_in", 4) -> &'a str;
                        scalar peer_tag_out_discard("peer_tag_out_discard", 5) -> &'a str;
                        model additional_paths("additional_paths", 6) -> item::AdditionalPaths<'a>;
                    }
                }

                pub mod item {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct AdditionalPaths {
                            scalar receive("receive", 0) -> bool;
                        }
                    }
                }
            }

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Networks {
                    model item (0) -> networks::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod networks {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar prefix("prefix", 0) -> &'a str;
                        scalar route_map("route_map", 1) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Redistribute {
                    model attached_host("attached_host", 0) -> redistribute::AttachedHost<'a>;
                    model connected("connected", 1) -> redistribute::Connected<'a>;
                    model isis("isis", 2) -> redistribute::Isis<'a>;
                    model ospf("ospf", 3) -> redistribute::Ospf<'a>;
                    model ospfv3("ospfv3", 4) -> redistribute::Ospfv3<'a>;
                    model field_static("static", 5) -> redistribute::FieldStatic<'a>;
                }
            }

            pub mod redistribute {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct AttachedHost {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Connected {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Isis {
                        scalar enabled("enabled", 0) -> bool;
                        scalar isis_level("isis_level", 1) -> &'a str;
                        scalar route_map("route_map", 2) -> &'a str;
                        scalar rcf("rcf", 3) -> &'a str;
                        scalar include_leaked("include_leaked", 4) -> bool;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Ospf {
                        scalar enabled("enabled", 0) -> bool;
                        model match_external("match_external", 1) -> ospf::MatchExternal<'a>;
                        model match_internal("match_internal", 2) -> ospf::MatchInternal<'a>;
                        model match_nssa_external("match_nssa_external", 3) -> ospf::MatchNssaExternal<'a>;
                        scalar route_map("route_map", 4) -> &'a str;
                    }
                }

                pub mod ospf {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MatchExternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar route_map("route_map", 1) -> &'a str;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MatchInternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar route_map("route_map", 1) -> &'a str;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MatchNssaExternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar nssa_type("nssa_type", 1) -> i64;
                            scalar route_map("route_map", 2) -> &'a str;
                        }
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Ospfv3 {
                        scalar enabled("enabled", 0) -> bool;
                        model match_external("match_external", 1) -> ospfv3::MatchExternal<'a>;
                        model match_internal("match_internal", 2) -> ospfv3::MatchInternal<'a>;
                        model match_nssa_external("match_nssa_external", 3) -> ospfv3::MatchNssaExternal<'a>;
                        scalar route_map("route_map", 4) -> &'a str;
                        scalar include_leaked("include_leaked", 5) -> bool;
                    }
                }

                pub mod ospfv3 {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MatchExternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar route_map("route_map", 1) -> &'a str;
                            scalar include_leaked("include_leaked", 2) -> bool;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MatchInternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar route_map("route_map", 1) -> &'a str;
                            scalar include_leaked("include_leaked", 2) -> bool;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MatchNssaExternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar nssa_type("nssa_type", 1) -> i64;
                            scalar route_map("route_map", 2) -> &'a str;
                            scalar include_leaked("include_leaked", 3) -> bool;
                        }
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct FieldStatic {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                    }
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AddressFamilyIpv6Multicast {
                model bgp("bgp", 0) -> address_family_ipv6_multicast::Bgp<'a>;
                model neighbors("neighbors", 1) -> address_family_ipv6_multicast::Neighbors<'a>;
                model networks("networks", 2) -> address_family_ipv6_multicast::Networks<'a>;
                model redistribute("redistribute", 3) -> address_family_ipv6_multicast::Redistribute<'a>;
            }
        }

        pub mod address_family_ipv6_multicast {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Bgp {
                    model missing_policy("missing_policy", 0) -> bgp::MissingPolicy<'a>;
                    model additional_paths("additional_paths", 1) -> bgp::AdditionalPaths<'a>;
                }
            }

            pub mod bgp {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct MissingPolicy {
                        scalar direction_in_action("direction_in_action", 0) -> &'a str;
                        scalar direction_out_action("direction_out_action", 1) -> &'a str;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct AdditionalPaths {
                        scalar receive("receive", 0) -> bool;
                    }
                }
            }

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Neighbors {
                    model item (0) -> neighbors::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod neighbors {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar ip_address("ip_address", 0) -> &'a str;
                        scalar activate("activate", 1) -> bool;
                        scalar route_map_in("route_map_in", 2) -> &'a str;
                        scalar route_map_out("route_map_out", 3) -> &'a str;
                        scalar peer_tag_in("peer_tag_in", 4) -> &'a str;
                        scalar peer_tag_out_discard("peer_tag_out_discard", 5) -> &'a str;
                        model additional_paths("additional_paths", 6) -> item::AdditionalPaths<'a>;
                    }
                }

                pub mod item {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct AdditionalPaths {
                            scalar receive("receive", 0) -> bool;
                        }
                    }
                }
            }

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Networks {
                    model item (0) -> networks::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod networks {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar prefix("prefix", 0) -> &'a str;
                        scalar route_map("route_map", 1) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Redistribute {
                    model connected("connected", 0) -> redistribute::Connected<'a>;
                    model isis("isis", 1) -> redistribute::Isis<'a>;
                    model ospf("ospf", 2) -> redistribute::Ospf<'a>;
                    model ospfv3("ospfv3", 3) -> redistribute::Ospfv3<'a>;
                    model field_static("static", 4) -> redistribute::FieldStatic<'a>;
                }
            }

            pub mod redistribute {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Connected {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Isis {
                        scalar enabled("enabled", 0) -> bool;
                        scalar isis_level("isis_level", 1) -> &'a str;
                        scalar route_map("route_map", 2) -> &'a str;
                        scalar rcf("rcf", 3) -> &'a str;
                        scalar include_leaked("include_leaked", 4) -> bool;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Ospf {
                        scalar enabled("enabled", 0) -> bool;
                        model match_external("match_external", 1) -> ospf::MatchExternal<'a>;
                        model match_internal("match_internal", 2) -> ospf::MatchInternal<'a>;
                        model match_nssa_external("match_nssa_external", 3) -> ospf::MatchNssaExternal<'a>;
                        scalar route_map("route_map", 4) -> &'a str;
                    }
                }

                pub mod ospf {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MatchExternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar route_map("route_map", 1) -> &'a str;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MatchInternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar route_map("route_map", 1) -> &'a str;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MatchNssaExternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar nssa_type("nssa_type", 1) -> i64;
                            scalar route_map("route_map", 2) -> &'a str;
                        }
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Ospfv3 {
                        scalar enabled("enabled", 0) -> bool;
                        model match_external("match_external", 1) -> ospfv3::MatchExternal<'a>;
                        model match_internal("match_internal", 2) -> ospfv3::MatchInternal<'a>;
                        model match_nssa_external("match_nssa_external", 3) -> ospfv3::MatchNssaExternal<'a>;
                        scalar route_map("route_map", 4) -> &'a str;
                    }
                }

                pub mod ospfv3 {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MatchExternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar route_map("route_map", 1) -> &'a str;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MatchInternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar route_map("route_map", 1) -> &'a str;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MatchNssaExternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar nssa_type("nssa_type", 1) -> i64;
                            scalar route_map("route_map", 2) -> &'a str;
                        }
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct FieldStatic {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                    }
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AddressFamilyFlowSpecIpv4 {
                model bgp("bgp", 0) -> address_family_flow_spec_ipv4::Bgp<'a>;
                model neighbors("neighbors", 1) -> address_family_flow_spec_ipv4::Neighbors<'a>;
            }
        }

        pub mod address_family_flow_spec_ipv4 {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Bgp {
                    model missing_policy("missing_policy", 0) -> bgp::MissingPolicy<'a>;
                }
            }

            pub mod bgp {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct MissingPolicy {
                        scalar direction_in_action("direction_in_action", 0) -> &'a str;
                        scalar direction_out_action("direction_out_action", 1) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Neighbors {
                    model item (0) -> neighbors::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod neighbors {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar ip_address("ip_address", 0) -> &'a str;
                        scalar activate("activate", 1) -> bool;
                    }
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AddressFamilyFlowSpecIpv6 {
                model bgp("bgp", 0) -> address_family_flow_spec_ipv6::Bgp<'a>;
                model neighbors("neighbors", 1) -> address_family_flow_spec_ipv6::Neighbors<'a>;
            }
        }

        pub mod address_family_flow_spec_ipv6 {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Bgp {
                    model missing_policy("missing_policy", 0) -> bgp::MissingPolicy<'a>;
                }
            }

            pub mod bgp {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct MissingPolicy {
                        scalar direction_in_action("direction_in_action", 0) -> &'a str;
                        scalar direction_out_action("direction_out_action", 1) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Neighbors {
                    model item (0) -> neighbors::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod neighbors {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar ip_address("ip_address", 0) -> &'a str;
                        scalar activate("activate", 1) -> bool;
                    }
                }
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SessionTrackers {
        model item (0) -> session_trackers::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod session_trackers {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar recovery_delay("recovery_delay", 1) -> i64;
        }
    }
}
