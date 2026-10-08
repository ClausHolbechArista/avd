// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Secret {
        scalar hash_algorithm("hash_algorithm", 0) -> &'a str;
        scalar key("key", 1) -> &'a str;
    }
}
