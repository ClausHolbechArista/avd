// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub template: ::validated_data::RequiredValue<&'a str, Mode>,
    pub options: ::validated_data::Field<item::Options<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view]
    pub struct Options<'a, Mode> {
        pub list_merge: ::validated_data::Field<&'a str>,
        pub strip_empty_keys: ::validated_data::Field<bool>,
    }
}
