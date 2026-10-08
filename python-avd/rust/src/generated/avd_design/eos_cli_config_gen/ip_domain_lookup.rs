// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SourceInterfaces {
        model item (0) -> source_interfaces::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod source_interfaces {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar vrf("vrf", 1) -> &'a str;
        }
    }
}
