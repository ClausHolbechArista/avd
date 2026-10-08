// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Persistent {
        scalar enabled("enabled", 0) -> bool;
        scalar refresh_delay("refresh_delay", 1) -> i64;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Aging {
        scalar timeout_default("timeout_default", 0) -> i64;
    }
}

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct StaticEntries {
        model item (0) -> static_entries::Item<'a>;
    }
}

pub mod static_entries {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar ipv4_address("ipv4_address", 0) -> &'a str;
            scalar vrf("vrf", 1) -> &'a str;
            scalar mac_address("mac_address", 2) -> &'a str;
        }
    }
}
