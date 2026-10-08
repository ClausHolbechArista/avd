// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AccessLists {
        model item (0) -> access_lists::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod access_lists {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model entries("entries", 1) -> item::Entries<'a>;
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
                    scalar field_type("type", 0) -> &'a str;
                    scalar field_match("match", 1) -> &'a str;
                    scalar origin("origin", 2) -> &'a str;
                }
            }
        }
    }
}
