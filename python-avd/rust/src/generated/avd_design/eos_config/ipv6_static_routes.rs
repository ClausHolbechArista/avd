// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub vrf: ::validated_data::Field<&'a str>,
    pub prefix: ::validated_data::RequiredValue<&'a str, Mode>,
    pub interface: ::validated_data::Field<&'a str>,
    pub next_hop: ::validated_data::Field<&'a str>,
    pub track_bfd: ::validated_data::Field<bool>,
    pub distance: ::validated_data::Field<i64>,
    pub tag: ::validated_data::Field<i64>,
    pub name: ::validated_data::Field<&'a str>,
    pub metric: ::validated_data::Field<i64>,
}
