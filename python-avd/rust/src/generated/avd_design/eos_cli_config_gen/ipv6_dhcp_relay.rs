// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Option {
        scalar link_layer_address("link_layer_address", 0) -> bool;
        scalar remote_id_format("remote_id_format", 1) -> &'a str;
    }
}
