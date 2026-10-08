// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar hostname("hostname", 0) -> &'a str;
        scalar vtep_ip("vtep_ip", 1) -> &'a str;
        model path_groups("path_groups", 2) -> item::PathGroups<'a>;
    }
}

pub mod item {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PathGroups {
            model item (0) -> path_groups::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod path_groups {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                model interfaces("interfaces", 1) -> item::Interfaces<'a>;
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
                        scalar public_ip("public_ip", 1) -> &'a str;
                    }
                }
            }
        }
    }
}
