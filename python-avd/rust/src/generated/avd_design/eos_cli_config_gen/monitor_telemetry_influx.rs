// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Destinations {
        model item (0) -> destinations::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod destinations {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar database("database", 1) -> &'a str;
            scalar data_retention_policy("data_retention_policy", 2) -> &'a str;
            scalar url("url", 3) -> &'a str;
            scalar username("username", 4) -> &'a str;
            scalar password("password", 5) -> &'a str;
            scalar password_type("password_type", 6) -> &'a str;
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SourceSockets {
        model item (0) -> source_sockets::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod source_sockets {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar connection_limit("connection_limit", 1) -> i64;
            scalar url("url", 2) -> &'a str;
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Tags {
        model item (0) -> tags::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod tags {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar value("value", 1) -> &'a str;
        }
    }
}
