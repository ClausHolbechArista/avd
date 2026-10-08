// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct LoggingTransceiver {
        scalar dom("dom", 0) -> bool;
        scalar communication("communication", 1) -> bool;
        scalar enabled("enabled", 2) -> bool;
    }
}
