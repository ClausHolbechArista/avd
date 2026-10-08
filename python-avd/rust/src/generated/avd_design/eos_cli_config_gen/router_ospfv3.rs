// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AddressFamilyIpv4 {
        scalar enabled("enabled", 0) -> bool;
        scalar router_id("router_id", 1) -> &'a str;
        scalar passive_interface_default("passive_interface_default", 2) -> bool;
        scalar auto_cost_reference_bandwidth("auto_cost_reference_bandwidth", 3) -> i64;
        model redistribute("redistribute", 4) -> address_family_ipv4::Redistribute<'a>;
    }
}

pub mod address_family_ipv4 {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Redistribute {
            model bgp("bgp", 0) -> redistribute::Bgp<'a>;
            model connected("connected", 1) -> redistribute::Connected<'a>;
            model field_static("static", 2) -> redistribute::FieldStatic<'a>;
            model isis("isis", 3) -> redistribute::Isis<'a>;
            model ospfv3("ospfv3", 4) -> redistribute::Ospfv3<'a>;
        }
    }

    pub mod redistribute {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Bgp {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
                scalar include_leaked("include_leaked", 2) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Connected {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
                scalar include_leaked("include_leaked", 2) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct FieldStatic {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
                scalar include_leaked("include_leaked", 2) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Isis {
                scalar enabled("enabled", 0) -> bool;
                scalar level("level", 1) -> &'a str;
                scalar route_map("route_map", 2) -> &'a str;
                scalar include_leaked("include_leaked", 3) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ospfv3 {
                model leaked("leaked", 0) -> ospfv3::Leaked<'a>;
                model leaked_match_external("leaked_match_external", 1) -> ospfv3::LeakedMatchExternal<'a>;
                model leaked_match_nssa_external("leaked_match_nssa_external", 2) -> ospfv3::LeakedMatchNssaExternal<'a>;
            }
        }

        pub mod ospfv3 {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Leaked {
                    scalar enabled("enabled", 0) -> bool;
                    scalar match_internal("match_internal", 1) -> bool;
                    scalar route_map("route_map", 2) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct LeakedMatchExternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct LeakedMatchNssaExternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AddressFamilyIpv6 {
        model redistribute("redistribute", 0) -> address_family_ipv6::Redistribute<'a>;
        scalar enabled("enabled", 1) -> bool;
        scalar router_id("router_id", 2) -> &'a str;
        scalar passive_interface_default("passive_interface_default", 3) -> bool;
        scalar auto_cost_reference_bandwidth("auto_cost_reference_bandwidth", 4) -> i64;
    }
}

pub mod address_family_ipv6 {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Redistribute {
            model dhcp("dhcp", 0) -> redistribute::Dhcp<'a>;
            model bgp("bgp", 1) -> redistribute::Bgp<'a>;
            model connected("connected", 2) -> redistribute::Connected<'a>;
            model field_static("static", 3) -> redistribute::FieldStatic<'a>;
            model isis("isis", 4) -> redistribute::Isis<'a>;
            model ospfv3("ospfv3", 5) -> redistribute::Ospfv3<'a>;
        }
    }

    pub mod redistribute {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Dhcp {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Bgp {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
                scalar include_leaked("include_leaked", 2) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Connected {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
                scalar include_leaked("include_leaked", 2) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct FieldStatic {
                scalar enabled("enabled", 0) -> bool;
                scalar route_map("route_map", 1) -> &'a str;
                scalar include_leaked("include_leaked", 2) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Isis {
                scalar enabled("enabled", 0) -> bool;
                scalar level("level", 1) -> &'a str;
                scalar route_map("route_map", 2) -> &'a str;
                scalar include_leaked("include_leaked", 3) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ospfv3 {
                model leaked("leaked", 0) -> ospfv3::Leaked<'a>;
                model leaked_match_external("leaked_match_external", 1) -> ospfv3::LeakedMatchExternal<'a>;
                model leaked_match_nssa_external("leaked_match_nssa_external", 2) -> ospfv3::LeakedMatchNssaExternal<'a>;
            }
        }

        pub mod ospfv3 {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Leaked {
                    scalar enabled("enabled", 0) -> bool;
                    scalar match_internal("match_internal", 1) -> bool;
                    scalar route_map("route_map", 2) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct LeakedMatchExternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct LeakedMatchNssaExternal {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                }
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
            scalar router_id("router_id", 1) -> &'a str;
            scalar passive_interface_default("passive_interface_default", 2) -> bool;
            scalar auto_cost_reference_bandwidth("auto_cost_reference_bandwidth", 3) -> i64;
            model address_family_ipv4("address_family_ipv4", 4) -> item::AddressFamilyIpv4<'a>;
            model address_family_ipv6("address_family_ipv6", 5) -> item::AddressFamilyIpv6<'a>;
            scalar eos_cli("eos_cli", 6) -> &'a str;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AddressFamilyIpv4 {
                scalar enabled("enabled", 0) -> bool;
                scalar router_id("router_id", 1) -> &'a str;
                scalar passive_interface_default("passive_interface_default", 2) -> bool;
                scalar auto_cost_reference_bandwidth("auto_cost_reference_bandwidth", 3) -> i64;
                model redistribute("redistribute", 4) -> address_family_ipv4::Redistribute<'a>;
            }
        }

        pub mod address_family_ipv4 {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Redistribute {
                    model bgp("bgp", 0) -> redistribute::Bgp<'a>;
                    model connected("connected", 1) -> redistribute::Connected<'a>;
                    model field_static("static", 2) -> redistribute::FieldStatic<'a>;
                    model isis("isis", 3) -> redistribute::Isis<'a>;
                    model ospfv3("ospfv3", 4) -> redistribute::Ospfv3<'a>;
                }
            }

            pub mod redistribute {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Bgp {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                        scalar include_leaked("include_leaked", 2) -> bool;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Connected {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                        scalar include_leaked("include_leaked", 2) -> bool;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct FieldStatic {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                        scalar include_leaked("include_leaked", 2) -> bool;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Isis {
                        scalar enabled("enabled", 0) -> bool;
                        scalar level("level", 1) -> &'a str;
                        scalar route_map("route_map", 2) -> &'a str;
                        scalar include_leaked("include_leaked", 3) -> bool;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Ospfv3 {
                        model leaked("leaked", 0) -> ospfv3::Leaked<'a>;
                        model leaked_match_external("leaked_match_external", 1) -> ospfv3::LeakedMatchExternal<'a>;
                        model leaked_match_nssa_external("leaked_match_nssa_external", 2) -> ospfv3::LeakedMatchNssaExternal<'a>;
                    }
                }

                pub mod ospfv3 {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Leaked {
                            scalar enabled("enabled", 0) -> bool;
                            scalar match_internal("match_internal", 1) -> bool;
                            scalar route_map("route_map", 2) -> &'a str;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct LeakedMatchExternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar route_map("route_map", 1) -> &'a str;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct LeakedMatchNssaExternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar route_map("route_map", 1) -> &'a str;
                        }
                    }
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AddressFamilyIpv6 {
                model redistribute("redistribute", 0) -> address_family_ipv6::Redistribute<'a>;
                scalar enabled("enabled", 1) -> bool;
                scalar router_id("router_id", 2) -> &'a str;
                scalar passive_interface_default("passive_interface_default", 3) -> bool;
                scalar auto_cost_reference_bandwidth("auto_cost_reference_bandwidth", 4) -> i64;
            }
        }

        pub mod address_family_ipv6 {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Redistribute {
                    model dhcp("dhcp", 0) -> redistribute::Dhcp<'a>;
                    model bgp("bgp", 1) -> redistribute::Bgp<'a>;
                    model connected("connected", 2) -> redistribute::Connected<'a>;
                    model field_static("static", 3) -> redistribute::FieldStatic<'a>;
                    model isis("isis", 4) -> redistribute::Isis<'a>;
                    model ospfv3("ospfv3", 5) -> redistribute::Ospfv3<'a>;
                }
            }

            pub mod redistribute {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Dhcp {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Bgp {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                        scalar include_leaked("include_leaked", 2) -> bool;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Connected {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                        scalar include_leaked("include_leaked", 2) -> bool;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct FieldStatic {
                        scalar enabled("enabled", 0) -> bool;
                        scalar route_map("route_map", 1) -> &'a str;
                        scalar include_leaked("include_leaked", 2) -> bool;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Isis {
                        scalar enabled("enabled", 0) -> bool;
                        scalar level("level", 1) -> &'a str;
                        scalar route_map("route_map", 2) -> &'a str;
                        scalar include_leaked("include_leaked", 3) -> bool;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Ospfv3 {
                        model leaked("leaked", 0) -> ospfv3::Leaked<'a>;
                        model leaked_match_external("leaked_match_external", 1) -> ospfv3::LeakedMatchExternal<'a>;
                        model leaked_match_nssa_external("leaked_match_nssa_external", 2) -> ospfv3::LeakedMatchNssaExternal<'a>;
                    }
                }

                pub mod ospfv3 {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Leaked {
                            scalar enabled("enabled", 0) -> bool;
                            scalar match_internal("match_internal", 1) -> bool;
                            scalar route_map("route_map", 2) -> &'a str;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct LeakedMatchExternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar route_map("route_map", 1) -> &'a str;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct LeakedMatchNssaExternal {
                            scalar enabled("enabled", 0) -> bool;
                            scalar route_map("route_map", 1) -> &'a str;
                        }
                    }
                }
            }
        }
    }
}
