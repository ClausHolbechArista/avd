// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub key: ::validated_data::Field<&'a str>,
    #[data_view(rename = "type")]
    pub field_type: ::validated_data::Field<&'a str>,
    pub description: ::validated_data::Field<&'a str>,
}
