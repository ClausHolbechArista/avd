// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar vrf("vrf", 1) -> &'a str;
        model neighbors("neighbors", 2) -> item::Neighbors<'a>;
        model bgp_maintenance_profiles("bgp_maintenance_profiles", 3) -> item::BgpMaintenanceProfiles<'a>;
    }
}

pub mod item {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Neighbors {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct BgpMaintenanceProfiles {
            scalar item (0) -> &'a str;
        }
    }
}
