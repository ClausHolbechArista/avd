// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub url: ::validated_data::Field<&'a str>,
    pub username: ::validated_data::Field<&'a str>,
    pub password: ::validated_data::Field<&'a str>,
    pub autovlan_disable: ::validated_data::Field<bool>,
    pub vrf: ::validated_data::Field<&'a str>,
    pub source_interface: ::validated_data::Field<&'a str>,
}
