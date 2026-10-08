// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Servers {
        model item (0) -> servers::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod servers {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar burst("burst", 1) -> bool;
            scalar iburst("iburst", 2) -> bool;
            scalar key("key", 3) -> i64;
            scalar maxpoll("maxpoll", 4) -> i64;
            scalar minpoll("minpoll", 5) -> i64;
            scalar version("version", 6) -> i64;
            scalar source_address("source_address", 7) -> &'a str;
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AuthenticationKeys {
        model item (0) -> authentication_keys::Item<'a>;
        primary_key_fields: [3];
    }
}

pub mod authentication_keys {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar key("key", 0) -> &'a str;
            scalar cleartext_key("cleartext_key", 1) -> &'a str;
            scalar key_type("key_type", 2) -> &'a str;
            scalar id("id", 3) -> i64;
            scalar hash_algorithm("hash_algorithm", 4) -> &'a str;
        }
    }
}
