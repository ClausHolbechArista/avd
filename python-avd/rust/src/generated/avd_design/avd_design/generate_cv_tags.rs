// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct InterfaceTags {
        model item (0) -> interface_tags::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod interface_tags {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar data_path("data_path", 1) -> &'a str;
            scalar value("value", 2) -> &'a str;
        }
    }
}

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DeviceTags {
        model item (0) -> device_tags::Item<'a>;
    }
}

pub mod device_tags {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar data_path("data_path", 1) -> &'a str;
            scalar value("value", 2) -> &'a str;
        }
    }
}
