// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Profiles {
        model item (0) -> profiles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod profiles {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar config("config", 1) -> &'a str;
            scalar source("source", 2) -> &'a str;
        }
    }
}
