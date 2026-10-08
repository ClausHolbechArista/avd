// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Hosts {
        model item (0) -> hosts::Item<'a>;
    }
}

pub mod hosts {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar vrf("vrf", 1) -> &'a str;
            scalar protocol("protocol", 2) -> &'a str;
            model ports("ports", 3) -> item::Ports<'a>;
            scalar ssl_profile("ssl_profile", 4) -> &'a str;
        }
    }

    pub mod item {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ports {
                scalar item (0) -> i64;
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Vrfs {
        model item (0) -> vrfs::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod vrfs {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar source_interface("source_interface", 1) -> &'a str;
        }
    }
}
