// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Querier {
        scalar enabled("enabled", 0) -> bool;
        scalar address("address", 1) -> &'a str;
        scalar query_interval("query_interval", 2) -> i64;
        scalar max_response_time("max_response_time", 3) -> i64;
        scalar last_member_query_interval("last_member_query_interval", 4) -> i64;
        scalar last_member_query_count("last_member_query_count", 5) -> i64;
        scalar startup_query_interval("startup_query_interval", 6) -> i64;
        scalar startup_query_count("startup_query_count", 7) -> i64;
        scalar version("version", 8) -> i64;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Vlans {
        model item (0) -> vlans::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod vlans {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar id("id", 0) -> i64;
            scalar enabled("enabled", 1) -> bool;
            model querier("querier", 2) -> item::Querier<'a>;
            scalar max_groups("max_groups", 3) -> i64;
            scalar fast_leave("fast_leave", 4) -> bool;
            scalar proxy("proxy", 5) -> bool;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Querier {
                scalar enabled("enabled", 0) -> bool;
                scalar address("address", 1) -> &'a str;
                scalar query_interval("query_interval", 2) -> i64;
                scalar max_response_time("max_response_time", 3) -> i64;
                scalar last_member_query_interval("last_member_query_interval", 4) -> i64;
                scalar last_member_query_count("last_member_query_count", 5) -> i64;
                scalar startup_query_interval("startup_query_interval", 6) -> i64;
                scalar startup_query_count("startup_query_count", 7) -> i64;
                scalar version("version", 8) -> i64;
            }
        }
    }
}
