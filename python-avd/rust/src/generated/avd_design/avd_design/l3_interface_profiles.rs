// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar profile("profile", 0) -> &'a str;
        scalar name("name", 1) -> &'a str;
        scalar description("description", 2) -> &'a str;
        scalar ip_address("ip_address", 3) -> &'a str;
        model ipv6_addresses("ipv6_addresses", 4) -> item::Ipv6Addresses<'a>;
        scalar dhcp_ip("dhcp_ip", 5) -> &'a str;
        scalar public_ip("public_ip", 6) -> &'a str;
        scalar encapsulation_dot1q_vlan("encapsulation_dot1q_vlan", 7) -> i64;
        scalar dhcp_accept_default_route("dhcp_accept_default_route", 8) -> bool;
        scalar enabled("enabled", 9) -> bool;
        scalar speed("speed", 10) -> &'a str;
        scalar receive_bandwidth("receive_bandwidth", 11) -> i64;
        scalar transmit_bandwidth("transmit_bandwidth", 12) -> i64;
        scalar peer("peer", 13) -> &'a str;
        scalar peer_interface("peer_interface", 14) -> &'a str;
        scalar peer_ip("peer_ip", 15) -> &'a str;
        scalar peer_ipv6("peer_ipv6", 16) -> &'a str;
        model bgp("bgp", 17) -> item::Bgp<'a>;
        scalar ipv4_acl_in("ipv4_acl_in", 18) -> &'a str;
        scalar ipv4_acl_out("ipv4_acl_out", 19) -> &'a str;
        scalar ipv6_acl_in("ipv6_acl_in", 20) -> &'a str;
        scalar ipv6_acl_out("ipv6_acl_out", 21) -> &'a str;
        model static_routes("static_routes", 22) -> item::StaticRoutes<'a>;
        scalar qos_profile("qos_profile", 23) -> &'a str;
        scalar wan_carrier("wan_carrier", 24) -> &'a str;
        scalar wan_circuit_id("wan_circuit_id", 25) -> &'a str;
        scalar connected_to_pathfinder("connected_to_pathfinder", 26) -> bool;
        model cv_pathfinder_internet_exit("cv_pathfinder_internet_exit", 27) -> item::CvPathfinderInternetExit<'a>;
        model rx_queue("rx_queue", 28) -> item::RxQueue<'a>;
        scalar raw_eos_cli("raw_eos_cli", 29) -> &'a str;
        model flow_tracking("flow_tracking", 30) -> item::FlowTracking<'a>;
        model structured_config("structured_config", 31) -> super::super::eos_cli_config_gen::ethernet_interfaces::Item<'a>;
    }
}

pub mod item {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipv6Addresses {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Bgp {
            scalar peer_as("peer_as", 0) -> &'a str;
            scalar ipv4_prefix_list_in("ipv4_prefix_list_in", 1) -> &'a str;
            scalar ipv4_prefix_list_out("ipv4_prefix_list_out", 2) -> &'a str;
            scalar ipv6_prefix_list_in("ipv6_prefix_list_in", 3) -> &'a str;
            scalar ipv6_prefix_list_out("ipv6_prefix_list_out", 4) -> &'a str;
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
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct CvPathfinderInternetExit {
            model policies("policies", 0) -> cv_pathfinder_internet_exit::Policies<'a>;
        }
    }

    pub mod cv_pathfinder_internet_exit {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Policies {
                model item (0) -> policies::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod policies {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    scalar tunnel_interface_numbers("tunnel_interface_numbers", 1) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct RxQueue {
            scalar count("count", 0) -> i64;
            model workers("workers", 1) -> rx_queue::Workers<'a>;
            scalar mode("mode", 2) -> &'a str;
        }
    }

    pub mod rx_queue {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Workers {
                scalar item (0) -> &'a str;
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
