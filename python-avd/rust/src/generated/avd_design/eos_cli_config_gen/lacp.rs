// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct PortId {
        model range("range", 0) -> port_id::Range<'a>;
    }
}

pub mod port_id {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Range {
            scalar begin("begin", 0) -> i64;
            scalar end("end", 1) -> i64;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RateLimit {
        scalar default("default", 0) -> bool;
    }
}
