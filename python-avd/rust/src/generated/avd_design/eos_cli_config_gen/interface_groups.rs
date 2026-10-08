// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        model interfaces("interfaces", 1) -> item::Interfaces<'a>;
        model bgp_maintenance_profiles("bgp_maintenance_profiles", 2) -> item::BgpMaintenanceProfiles<'a>;
        model interface_maintenance_profiles("interface_maintenance_profiles", 3) -> item::InterfaceMaintenanceProfiles<'a>;
    }
}

pub mod item {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Interfaces {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct BgpMaintenanceProfiles {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct InterfaceMaintenanceProfiles {
            scalar item (0) -> &'a str;
        }
    }
}
