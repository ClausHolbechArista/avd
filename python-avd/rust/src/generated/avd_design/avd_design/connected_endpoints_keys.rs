// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar key("key", 0) -> &'a str;
        scalar field_type("type", 1) -> &'a str;
        scalar description("description", 2) -> &'a str;
    }
}
