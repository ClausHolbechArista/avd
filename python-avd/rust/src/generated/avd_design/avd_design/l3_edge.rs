// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct P2pLinksIpPools {
        model item (0) -> p2p_links_ip_pools::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod p2p_links_ip_pools {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar ipv4_pool("ipv4_pool", 1) -> &'a str;
            scalar prefix_size("prefix_size", 2) -> i64;
            scalar ipv6_pool("ipv6_pool", 3) -> &'a str;
            scalar ipv6_prefix_size("ipv6_prefix_size", 4) -> i64;
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct P2pLinksProfiles {
        model item (0) -> p2p_links_profiles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod p2p_links_profiles {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar id("id", 1) -> i64;
            scalar speed("speed", 2) -> &'a str;
            scalar ip_pool("ip_pool", 3) -> &'a str;
            scalar subnet("subnet", 4) -> &'a str;
            model ip("ip", 5) -> item::Ip<'a>;
            scalar ipv6_enable("ipv6_enable", 6) -> bool;
            scalar ipv6_prefix("ipv6_prefix", 7) -> &'a str;
            model ipv6("ipv6", 8) -> item::Ipv6<'a>;
            model nodes("nodes", 9) -> item::Nodes<'a>;
            model interfaces("interfaces", 10) -> item::Interfaces<'a>;
            model field_as("as", 11) -> item::FieldAs<'a>;
            model descriptions("descriptions", 12) -> item::Descriptions<'a>;
            scalar include_in_underlay_protocol("include_in_underlay_protocol", 13) -> bool;
            scalar use_underlay_ospf_authentication("use_underlay_ospf_authentication", 14) -> bool;
            scalar isis_hello_padding("isis_hello_padding", 15) -> bool;
            scalar isis_metric("isis_metric", 16) -> i64;
            scalar isis_circuit_type("isis_circuit_type", 17) -> &'a str;
            scalar isis_authentication_mode("isis_authentication_mode", 18) -> &'a str;
            scalar isis_authentication_key("isis_authentication_key", 19) -> &'a str;
            scalar isis_authentication_cleartext_key("isis_authentication_cleartext_key", 20) -> &'a str;
            scalar isis_network_type("isis_network_type", 21) -> &'a str;
            scalar mpls_ip("mpls_ip", 22) -> bool;
            scalar mpls_ldp("mpls_ldp", 23) -> bool;
            scalar mtu("mtu", 24) -> i64;
            scalar bfd("bfd", 25) -> bool;
            model ptp("ptp", 26) -> item::Ptp<'a>;
            scalar sflow("sflow", 27) -> bool;
            scalar multicast_pim_sm("multicast_pim_sm", 28) -> bool;
            scalar multicast_static("multicast_static", 29) -> bool;
            model flow_tracking("flow_tracking", 30) -> item::FlowTracking<'a>;
            scalar qos_profile("qos_profile", 31) -> &'a str;
            scalar macsec_profile("macsec_profile", 32) -> &'a str;
            model port_channel("port_channel", 33) -> item::PortChannel<'a>;
            model campus_link_type("campus_link_type", 34) -> item::CampusLinkType<'a>;
            scalar raw_eos_cli("raw_eos_cli", 35) -> &'a str;
            scalar routing_protocol("routing_protocol", 36) -> &'a str;
            model ethernet_structured_config("ethernet_structured_config", 37) -> super::super::super::eos_cli_config_gen::ethernet_interfaces::Item<'a>;
            model port_channel_structured_config("port_channel_structured_config", 38) -> super::super::super::eos_cli_config_gen::port_channel_interfaces::Item<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ip {
                scalar item (0) -> &'a str;
            }
        }

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ipv6 {
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
            pub struct Interfaces {
                scalar item (0) -> &'a str;
            }
        }

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct FieldAs {
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
            pub struct Ptp {
                scalar enabled("enabled", 0) -> bool;
                model roles("roles", 1) -> ptp::Roles<'a>;
                scalar profile("profile", 2) -> &'a str;
            }
        }

        pub mod ptp {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Roles {
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

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct PortChannel {
                scalar description("description", 0) -> &'a str;
                scalar mode("mode", 1) -> &'a str;
                scalar channel_id_algorithm("channel_id_algorithm", 2) -> &'a str;
                scalar channel_id_offset("channel_id_offset", 3) -> i64;
                model nodes_child_interfaces("nodes_child_interfaces", 4) -> port_channel::NodesChildInterfaces<'a>;
            }
        }

        pub mod port_channel {

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct NodesChildInterfaces {
                    model item (0) -> nodes_child_interfaces::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod nodes_child_interfaces {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar node("node", 0) -> &'a str;
                        model interfaces("interfaces", 1) -> item::Interfaces<'a>;
                        scalar channel_id("channel_id", 2) -> i64;
                    }
                }

                pub mod item {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Interfaces {
                            scalar item (0) -> &'a str;
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
    pub struct P2pLinks {
        model item (0) -> p2p_links::Item<'a>;
    }
}

pub mod p2p_links {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            model nodes("nodes", 0) -> item::Nodes<'a>;
            scalar profile("profile", 1) -> &'a str;
            scalar id("id", 2) -> i64;
            scalar speed("speed", 3) -> &'a str;
            scalar ip_pool("ip_pool", 4) -> &'a str;
            scalar subnet("subnet", 5) -> &'a str;
            model ip("ip", 6) -> item::Ip<'a>;
            scalar ipv6_enable("ipv6_enable", 7) -> bool;
            scalar ipv6_prefix("ipv6_prefix", 8) -> &'a str;
            model ipv6("ipv6", 9) -> item::Ipv6<'a>;
            model interfaces("interfaces", 10) -> item::Interfaces<'a>;
            model field_as("as", 11) -> item::FieldAs<'a>;
            model descriptions("descriptions", 12) -> item::Descriptions<'a>;
            scalar include_in_underlay_protocol("include_in_underlay_protocol", 13) -> bool;
            scalar use_underlay_ospf_authentication("use_underlay_ospf_authentication", 14) -> bool;
            scalar isis_hello_padding("isis_hello_padding", 15) -> bool;
            scalar isis_metric("isis_metric", 16) -> i64;
            scalar isis_circuit_type("isis_circuit_type", 17) -> &'a str;
            scalar isis_authentication_mode("isis_authentication_mode", 18) -> &'a str;
            scalar isis_authentication_key("isis_authentication_key", 19) -> &'a str;
            scalar isis_authentication_cleartext_key("isis_authentication_cleartext_key", 20) -> &'a str;
            scalar isis_network_type("isis_network_type", 21) -> &'a str;
            scalar mpls_ip("mpls_ip", 22) -> bool;
            scalar mpls_ldp("mpls_ldp", 23) -> bool;
            scalar mtu("mtu", 24) -> i64;
            scalar bfd("bfd", 25) -> bool;
            model ptp("ptp", 26) -> item::Ptp<'a>;
            scalar sflow("sflow", 27) -> bool;
            scalar multicast_pim_sm("multicast_pim_sm", 28) -> bool;
            scalar multicast_static("multicast_static", 29) -> bool;
            model flow_tracking("flow_tracking", 30) -> item::FlowTracking<'a>;
            scalar qos_profile("qos_profile", 31) -> &'a str;
            scalar macsec_profile("macsec_profile", 32) -> &'a str;
            model port_channel("port_channel", 33) -> item::PortChannel<'a>;
            model campus_link_type("campus_link_type", 34) -> item::CampusLinkType<'a>;
            scalar raw_eos_cli("raw_eos_cli", 35) -> &'a str;
            scalar routing_protocol("routing_protocol", 36) -> &'a str;
            model ethernet_structured_config("ethernet_structured_config", 37) -> super::super::super::eos_cli_config_gen::ethernet_interfaces::Item<'a>;
            model port_channel_structured_config("port_channel_structured_config", 38) -> super::super::super::eos_cli_config_gen::port_channel_interfaces::Item<'a>;
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
            pub struct Ip {
                scalar item (0) -> &'a str;
            }
        }

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ipv6 {
                scalar item (0) -> &'a str;
            }
        }

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Interfaces {
                scalar item (0) -> &'a str;
            }
        }

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct FieldAs {
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
            pub struct Ptp {
                scalar enabled("enabled", 0) -> bool;
                model roles("roles", 1) -> ptp::Roles<'a>;
                scalar profile("profile", 2) -> &'a str;
            }
        }

        pub mod ptp {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Roles {
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

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct PortChannel {
                scalar description("description", 0) -> &'a str;
                scalar mode("mode", 1) -> &'a str;
                scalar channel_id_algorithm("channel_id_algorithm", 2) -> &'a str;
                scalar channel_id_offset("channel_id_offset", 3) -> i64;
                model nodes_child_interfaces("nodes_child_interfaces", 4) -> port_channel::NodesChildInterfaces<'a>;
            }
        }

        pub mod port_channel {

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct NodesChildInterfaces {
                    model item (0) -> nodes_child_interfaces::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod nodes_child_interfaces {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar node("node", 0) -> &'a str;
                        model interfaces("interfaces", 1) -> item::Interfaces<'a>;
                        scalar channel_id("channel_id", 2) -> i64;
                    }
                }

                pub mod item {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Interfaces {
                            scalar item (0) -> &'a str;
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
