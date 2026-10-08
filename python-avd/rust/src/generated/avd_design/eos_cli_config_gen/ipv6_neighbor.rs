// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


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
            scalar ipv6_address("ipv6_address", 0) -> &'a str;
            scalar vrf("vrf", 1) -> &'a str;
            scalar interface("interface", 2) -> &'a str;
            scalar mac_address("mac_address", 3) -> &'a str;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Persistent {
        scalar enabled("enabled", 0) -> bool;
        scalar refresh_delay("refresh_delay", 1) -> i64;
    }
}
