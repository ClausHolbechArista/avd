// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Probe {
        scalar interval("interval", 0) -> i64;
        scalar threshold_failure("threshold_failure", 1) -> i64;
        scalar method("method", 2) -> &'a str;
        model access_request("access_request", 3) -> probe::AccessRequest<'a>;
    }
}

pub mod probe {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct AccessRequest {
            scalar username("username", 0) -> &'a str;
            scalar password("password", 1) -> &'a str;
            scalar password_type("password_type", 2) -> &'a str;
        }
    }
}
