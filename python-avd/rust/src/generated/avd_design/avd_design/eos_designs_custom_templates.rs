// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar template("template", 0) -> &'a str;
        model options("options", 1) -> item::Options<'a>;
    }
}

pub mod item {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Options {
            scalar list_merge("list_merge", 0) -> &'a str;
            scalar strip_empty_keys("strip_empty_keys", 1) -> bool;
        }
    }
}
