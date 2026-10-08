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
            scalar host("host", 0) -> &'a str;
            scalar vrf("vrf", 1) -> &'a str;
            scalar key("key", 2) -> &'a str;
            scalar key_type("key_type", 3) -> &'a str;
            scalar single_connection("single_connection", 4) -> bool;
            scalar timeout("timeout", 5) -> i64;
        }
    }
}
