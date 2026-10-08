// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar url("url", 1) -> &'a str;
        scalar username("username", 2) -> &'a str;
        scalar password("password", 3) -> &'a str;
        scalar autovlan_disable("autovlan_disable", 4) -> bool;
        scalar vrf("vrf", 5) -> &'a str;
        scalar source_interface("source_interface", 6) -> &'a str;
    }
}
