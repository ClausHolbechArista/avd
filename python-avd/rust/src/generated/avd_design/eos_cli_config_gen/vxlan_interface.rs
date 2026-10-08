// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Vxlan1 {
        scalar description("description", 0) -> &'a str;
        model vxlan("vxlan", 1) -> vxlan1::Vxlan<'a>;
        scalar eos_cli("eos_cli", 2) -> &'a str;
    }
}

pub mod vxlan1 {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Vxlan {
            scalar source_interface("source_interface", 0) -> &'a str;
            scalar shutdown("shutdown", 1) -> bool;
            model multicast("multicast", 2) -> vxlan::Multicast<'a>;
            model controller_client("controller_client", 3) -> vxlan::ControllerClient<'a>;
            scalar mlag_source_interface("mlag_source_interface", 4) -> &'a str;
            scalar udp_port("udp_port", 5) -> i64;
            model encapsulations("encapsulations", 6) -> vxlan::Encapsulations<'a>;
            scalar vtep_to_vtep_bridging("vtep_to_vtep_bridging", 7) -> bool;
            scalar virtual_router_encapsulation_mac_address("virtual_router_encapsulation_mac_address", 8) -> &'a str;
            model bfd_vtep_evpn("bfd_vtep_evpn", 9) -> vxlan::BfdVtepEvpn<'a>;
            model qos("qos", 10) -> vxlan::Qos<'a>;
            model vlan_range("vlan_range", 11) -> vxlan::VlanRange<'a>;
            model vlans("vlans", 12) -> vxlan::Vlans<'a>;
            model vrfs("vrfs", 13) -> vxlan::Vrfs<'a>;
            model flood_vteps("flood_vteps", 14) -> vxlan::FloodVteps<'a>;
            scalar flood_vtep_learned_data_plane("flood_vtep_learned_data_plane", 15) -> bool;
        }
    }

    pub mod vxlan {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Multicast {
                scalar headend_replication("headend_replication", 0) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct ControllerClient {
                scalar enabled("enabled", 0) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Encapsulations {
                scalar ipv4("ipv4", 0) -> bool;
                scalar ipv6("ipv6", 1) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct BfdVtepEvpn {
                scalar interval("interval", 0) -> i64;
                scalar min_rx("min_rx", 1) -> i64;
                scalar multiplier("multiplier", 2) -> i64;
                scalar prefix_list("prefix_list", 3) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Qos {
                scalar dscp_propagation_encapsulation("dscp_propagation_encapsulation", 0) -> bool;
                scalar ecn_propagation("ecn_propagation", 1) -> bool;
                scalar map_dscp_to_traffic_class_decapsulation("map_dscp_to_traffic_class_decapsulation", 2) -> bool;
                model dscp_ecn("dscp_ecn", 3) -> qos::DscpEcn<'a>;
            }
        }

        pub mod qos {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct DscpEcn {
                    scalar rewrite_bridged_enabled("rewrite_bridged_enabled", 0) -> bool;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct VlanRange {
                scalar vlans("vlans", 0) -> &'a str;
                scalar vnis("vnis", 1) -> &'a str;
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
                    scalar vni("vni", 1) -> i64;
                    scalar multicast_group("multicast_group", 2) -> &'a str;
                    model flood_vteps("flood_vteps", 3) -> item::FloodVteps<'a>;
                    scalar flood_group("flood_group", 4) -> &'a str;
                }
            }

            pub mod item {

                ::validation::define_archive_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct FloodVteps {
                        scalar item (0) -> &'a str;
                    }
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
                    scalar vni("vni", 1) -> i64;
                    scalar multicast_group("multicast_group", 2) -> &'a str;
                    scalar multicast_group_encap_range("multicast_group_encap_range", 3) -> &'a str;
                    model multicast_groups("multicast_groups", 4) -> item::MulticastGroups<'a>;
                }
            }

            pub mod item {

                ::validation::define_archive_indexed_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct MulticastGroups {
                        model item (0) -> multicast_groups::Item<'a>;
                        primary_key_fields: [0];
                    }
                }

                pub mod multicast_groups {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            scalar overlay_group("overlay_group", 0) -> &'a str;
                            scalar encap("encap", 1) -> &'a str;
                        }
                    }
                }
            }
        }

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct FloodVteps {
                scalar item (0) -> &'a str;
            }
        }
    }
}
