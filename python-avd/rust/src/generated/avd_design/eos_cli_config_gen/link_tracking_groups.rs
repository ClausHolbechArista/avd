// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar links_minimum("links_minimum", 1) -> i64;
        scalar recovery_delay("recovery_delay", 2) -> i64;
    }
}
