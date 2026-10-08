// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar hostname("hostname", 0) -> &'a str;
        model ipv4_addresses("ipv4_addresses", 1) -> item::Ipv4Addresses<'a>;
    }
}

pub mod item {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipv4Addresses {
            scalar item (0) -> &'a str;
        }
    }
}
