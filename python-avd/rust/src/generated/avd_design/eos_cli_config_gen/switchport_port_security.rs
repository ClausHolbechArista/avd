// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MacAddress {
        scalar aging("aging", 0) -> bool;
        scalar moveable("moveable", 1) -> bool;
    }
}
