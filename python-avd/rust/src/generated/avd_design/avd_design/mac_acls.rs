// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar counters_per_entry("counters_per_entry", 1) -> bool;
        model entries("entries", 2) -> item::Entries<'a>;
    }
}

pub mod item {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Entries {
            model item (0) -> entries::Item<'a>;
        }
    }

    pub mod entries {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar sequence("sequence", 0) -> i64;
                scalar action("action", 1) -> &'a str;
                scalar remark("remark", 2) -> &'a str;
                scalar source("source", 3) -> &'a str;
                scalar source_wildcard("source_wildcard", 4) -> &'a str;
                scalar destination("destination", 5) -> &'a str;
                scalar destination_wildcard("destination_wildcard", 6) -> &'a str;
            }
        }
    }
}
