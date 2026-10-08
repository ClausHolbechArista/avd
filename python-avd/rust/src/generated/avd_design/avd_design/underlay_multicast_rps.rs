// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar rp("rp", 0) -> &'a str;
        model nodes("nodes", 1) -> item::Nodes<'a>;
        model groups("groups", 2) -> item::Groups<'a>;
        scalar access_list_name("access_list_name", 3) -> &'a str;
    }
}

pub mod item {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Nodes {
            model item (0) -> nodes::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod nodes {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar loopback_number("loopback_number", 1) -> i64;
                scalar description("description", 2) -> &'a str;
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Groups {
            scalar item (0) -> &'a str;
        }
    }
}
