// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Config<'a, Mode> {
    pub max_concurrent_jobs: ::validated_data::Field<i64>,
    pub prepend_hostname_logfile: ::validated_data::Field<bool>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Jobs<'a, Mode> (::validated_data::Field<jobs::Item<'a, Mode>>);

pub mod jobs {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub interval: ::validated_data::Field<i64>,
        pub at: ::validated_data::Field<item::At<'a, Mode>>,
        pub timeout: ::validated_data::Field<i64>,
        pub max_log_files: ::validated_data::RequiredValue<i64, Mode>,
        pub logging_verbose: ::validated_data::Field<bool>,
        pub loglocation: ::validated_data::Field<&'a str>,
        pub max_total_size: ::validated_data::Field<&'a str>,
        pub compression: ::validated_data::Field<&'a str>,
        pub command: ::validated_data::RequiredValue<&'a str, Mode>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct At<'a, Mode> {
            pub time: ::validated_data::RequiredValue<&'a str, Mode>,
            pub date: ::validated_data::RequiredValue<&'a str, Mode>,
            pub once: ::validated_data::Field<bool>,
        }
    }
}
