// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Daemons<'a, Mode> (::validated_data::Field<Item<'a, Mode>>);

#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub exec: ::validated_data::RequiredValue<&'a str, Mode>,
    pub enabled: ::validated_data::Field<bool>,
}
