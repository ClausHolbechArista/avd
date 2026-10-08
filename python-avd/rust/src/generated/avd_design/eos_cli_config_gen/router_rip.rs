// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Vrfs {
        model item (0) -> vrfs::Item<'a>;
        primary_key_fields: [1];
    }
}

pub mod vrfs {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar enabled("enabled", 0) -> bool;
            scalar vrf("vrf", 1) -> &'a str;
            scalar metric_default("metric_default", 2) -> i64;
            model networks("networks", 3) -> item::Networks<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Networks {
                scalar item (0) -> &'a str;
            }
        }
    }
}
