// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RouterId {
        scalar ipv4("ipv4", 0) -> &'a str;
        scalar ipv6("ipv6", 1) -> &'a str;
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
            model leak_routes("leak_routes", 1) -> item::LeakRoutes<'a>;
            model routes("routes", 2) -> item::Routes<'a>;
            scalar software_forwarding_hardware_offload_mtu("software_forwarding_hardware_offload_mtu", 3) -> i64;
        }
    }

    pub mod item {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct LeakRoutes {
                model item (0) -> leak_routes::Item<'a>;
            }
        }

        pub mod leak_routes {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar source_vrf("source_vrf", 0) -> &'a str;
                    scalar subscribe_policy("subscribe_policy", 1) -> &'a str;
                    scalar subscribe_rcf("subscribe_rcf", 2) -> &'a str;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Routes {
                model dynamic_prefix_lists("dynamic_prefix_lists", 0) -> routes::DynamicPrefixLists<'a>;
            }
        }

        pub mod routes {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct DynamicPrefixLists {
                    model item (0) -> dynamic_prefix_lists::Item<'a>;
                }
            }

            pub mod dynamic_prefix_lists {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar name("name", 0) -> &'a str;
                    }
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ControlFunctions {
        model code_units("code_units", 0) -> control_functions::CodeUnits<'a>;
    }
}

pub mod control_functions {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct CodeUnits {
            model item (0) -> code_units::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod code_units {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar content("content", 1) -> &'a str;
            }
        }
    }
}
