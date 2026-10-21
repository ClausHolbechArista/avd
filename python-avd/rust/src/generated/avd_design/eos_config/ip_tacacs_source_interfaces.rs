// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(list)]
pub struct IpTacacsSourceInterfaces<'a, Mode> (::validated_data::Field<Item<'a, Mode>>);

#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::RequiredValue<&'a str, Mode>,
    pub vrf: ::validated_data::Field<&'a str>,
}
