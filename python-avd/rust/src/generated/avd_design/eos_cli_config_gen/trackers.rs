// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub interface: ::validated_data::RequiredValue<&'a str, Mode>,
    pub tracked_property: ::validated_data::Field<&'a str>,
}
