// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar description("description", 1) -> &'a str;
        scalar id("id", 2) -> i64;
        model sites("sites", 3) -> item::Sites<'a>;
    }
}

pub mod item {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Sites {
            model item (0) -> sites::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod sites {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar description("description", 1) -> &'a str;
                scalar id("id", 2) -> i64;
                scalar location("location", 3) -> &'a str;
                scalar site_contact("site_contact", 4) -> &'a str;
                scalar site_after_hours_contact("site_after_hours_contact", 5) -> &'a str;
            }
        }
    }
}
