// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


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
                scalar action("action", 0) -> &'a str;
                model large_communities("large_communities", 1) -> item::LargeCommunities<'a>;
                scalar regexp("regexp", 2) -> &'a str;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct LargeCommunities {
                    scalar item (0) -> &'a str;
                }
            }
        }
    }
}
