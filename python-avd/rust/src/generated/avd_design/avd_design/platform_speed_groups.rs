// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar platform("platform", 0) -> &'a str;
        model speeds("speeds", 1) -> item::Speeds<'a>;
    }
}

pub mod item {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Speeds {
            model item (0) -> speeds::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod speeds {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar speed("speed", 0) -> &'a str;
                model speed_groups("speed_groups", 1) -> item::SpeedGroups<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct SpeedGroups {
                    scalar item (0) -> &'a str;
                }
            }
        }
    }
}
