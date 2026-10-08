// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar match_map("match_map", 1) -> &'a str;
        model prefix_list("prefix_list", 2) -> item::PrefixList<'a>;
    }
}

pub mod item {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PrefixList {
            scalar ipv4("ipv4", 0) -> &'a str;
            scalar ipv6("ipv6", 1) -> &'a str;
        }
    }
}
