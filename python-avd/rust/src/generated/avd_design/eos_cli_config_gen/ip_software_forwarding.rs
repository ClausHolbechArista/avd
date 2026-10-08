// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Mtu {
        scalar size("size", 0) -> i64;
        scalar exceed_action_drop("exceed_action_drop", 1) -> bool;
    }
}
