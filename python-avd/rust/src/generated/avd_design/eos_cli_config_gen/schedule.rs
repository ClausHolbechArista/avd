// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Config {
        scalar max_concurrent_jobs("max_concurrent_jobs", 0) -> i64;
        scalar prepend_hostname_logfile("prepend_hostname_logfile", 1) -> bool;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Jobs {
        model item (0) -> jobs::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod jobs {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar interval("interval", 1) -> i64;
            model at("at", 2) -> item::At<'a>;
            scalar timeout("timeout", 3) -> i64;
            scalar max_log_files("max_log_files", 4) -> i64;
            scalar logging_verbose("logging_verbose", 5) -> bool;
            scalar loglocation("loglocation", 6) -> &'a str;
            scalar max_total_size("max_total_size", 7) -> &'a str;
            scalar compression("compression", 8) -> &'a str;
            scalar command("command", 9) -> &'a str;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct At {
                scalar time("time", 0) -> &'a str;
                scalar date("date", 1) -> &'a str;
                scalar once("once", 2) -> bool;
            }
        }
    }
}
