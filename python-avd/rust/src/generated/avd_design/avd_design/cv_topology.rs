// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar hostname("hostname", 0) -> &'a str;
        scalar platform("platform", 1) -> &'a str;
        model interfaces("interfaces", 2) -> item::Interfaces<'a>;
    }
}

pub mod item {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Interfaces {
            model item (0) -> interfaces::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod interfaces {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar neighbor("neighbor", 1) -> &'a str;
                scalar neighbor_interface("neighbor_interface", 2) -> &'a str;
            }
        }
    }
}
