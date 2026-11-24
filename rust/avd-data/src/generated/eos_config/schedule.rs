// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Schedule<'a, Mode> {
    pub config: ::data_views::Field<Config<'a, Mode>>,
    pub jobs: ::data_views::Field<Jobs<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Config<'a, Mode> {
    pub max_concurrent_jobs: ::data_views::Field<i64>,
    pub prepend_hostname_logfile: ::data_views::Field<bool>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Jobs<'a, Mode> (::data_views::Field<jobs::Item<'a, Mode>>);

pub mod jobs {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub interval: ::data_views::Field<i64>,
        pub at: ::data_views::Field<item::At<'a, Mode>>,
        pub timeout: ::data_views::Field<i64>,
        pub max_log_files: ::data_views::RequiredValue<i64, Mode>,
        pub logging_verbose: ::data_views::Field<bool>,
        pub loglocation: ::data_views::Field<&'a str>,
        pub max_total_size: ::data_views::Field<&'a str>,
        pub compression: ::data_views::Field<&'a str>,
        pub command: ::data_views::RequiredValue<&'a str, Mode>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct At<'a, Mode> {
            pub time: ::data_views::RequiredValue<&'a str, Mode>,
            pub date: ::data_views::RequiredValue<&'a str, Mode>,
            pub once: ::data_views::Field<bool>,
        }
    }
}
