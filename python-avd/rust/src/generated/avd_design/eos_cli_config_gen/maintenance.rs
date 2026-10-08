// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct InterfaceProfiles {
        model item (0) -> interface_profiles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod interface_profiles {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model rate_monitoring("rate_monitoring", 1) -> item::RateMonitoring<'a>;
            model shutdown("shutdown", 2) -> item::Shutdown<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct RateMonitoring {
                scalar load_interval("load_interval", 0) -> i64;
                scalar threshold("threshold", 1) -> i64;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Shutdown {
                scalar max_delay("max_delay", 0) -> i64;
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct BgpProfiles {
        model item (0) -> bgp_profiles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod bgp_profiles {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model initiator("initiator", 1) -> item::Initiator<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Initiator {
                scalar route_map_inout("route_map_inout", 0) -> &'a str;
                scalar route_map_in("route_map_in", 1) -> &'a str;
                scalar route_map_out("route_map_out", 2) -> &'a str;
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct UnitProfiles {
        model item (0) -> unit_profiles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod unit_profiles {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model on_boot("on_boot", 1) -> item::OnBoot<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct OnBoot {
                scalar duration("duration", 0) -> i64;
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Units {
        model item (0) -> units::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod units {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar quiesce("quiesce", 1) -> bool;
            scalar profile("profile", 2) -> &'a str;
            model groups("groups", 3) -> item::Groups<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Groups {
                model bgp_groups("bgp_groups", 0) -> groups::BgpGroups<'a>;
                model interface_groups("interface_groups", 1) -> groups::InterfaceGroups<'a>;
            }
        }

        pub mod groups {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct BgpGroups {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct InterfaceGroups {
                    scalar item (0) -> &'a str;
                }
            }
        }
    }
}
