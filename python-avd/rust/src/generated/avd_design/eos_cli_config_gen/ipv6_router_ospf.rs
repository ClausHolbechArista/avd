// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ProcessIds {
        model item (0) -> process_ids::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod process_ids {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar id("id", 0) -> i64;
            scalar vrf("vrf", 1) -> &'a str;
            scalar router_id("router_id", 2) -> &'a str;
            model redistribute("redistribute", 3) -> item::Redistribute<'a>;
            scalar auto_cost_reference_bandwidth("auto_cost_reference_bandwidth", 4) -> i64;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Redistribute {
                model bgp("bgp", 0) -> redistribute::Bgp<'a>;
                model connected("connected", 1) -> redistribute::Connected<'a>;
                model isis("isis", 2) -> redistribute::Isis<'a>;
                model ospfv3("ospfv3", 3) -> redistribute::Ospfv3<'a>;
                model field_static("static", 4) -> redistribute::FieldStatic<'a>;
                model dhcp("dhcp", 5) -> redistribute::Dhcp<'a>;
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
                pub struct Isis {
                    scalar enabled("enabled", 0) -> bool;
                    scalar isis_level("isis_level", 1) -> &'a str;
                    scalar route_map("route_map", 2) -> &'a str;
                    scalar include_leaked("include_leaked", 3) -> bool;
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
                        scalar include_leaked("include_leaked", 2) -> bool;
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
                        scalar route_map("route_map", 1) -> &'a str;
                    }
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
                pub struct Dhcp {
                    scalar enabled("enabled", 0) -> bool;
                    scalar route_map("route_map", 1) -> &'a str;
                }
            }
        }
    }
}
