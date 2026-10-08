// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Range {
        scalar beginning("beginning", 0) -> i64;
        scalar ending("ending", 1) -> i64;
    }
}
