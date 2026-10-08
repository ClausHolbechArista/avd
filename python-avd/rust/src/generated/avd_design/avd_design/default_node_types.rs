// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar node_type("node_type", 0) -> &'a str;
        model match_hostnames("match_hostnames", 1) -> item::MatchHostnames<'a>;
    }
}

pub mod item {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct MatchHostnames {
            scalar item (0) -> &'a str;
        }
    }
}
