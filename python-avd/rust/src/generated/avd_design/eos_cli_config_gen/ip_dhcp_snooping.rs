// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct InformationOption {
        scalar enabled("enabled", 0) -> bool;
        scalar circuit_id_type("circuit_id_type", 1) -> &'a str;
        scalar circuit_id_format("circuit_id_format", 2) -> &'a str;
    }
}
