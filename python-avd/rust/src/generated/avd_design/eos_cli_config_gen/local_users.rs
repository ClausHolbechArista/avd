// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub disabled: ::validated_data::Field<bool>,
    pub privilege: ::validated_data::Field<i64>,
    pub role: ::validated_data::Field<&'a str>,
    pub sha512_password: ::validated_data::Field<&'a str>,
    pub no_password: ::validated_data::Field<bool>,
    pub ssh_key: ::validated_data::Field<&'a str>,
    pub secondary_ssh_key: ::validated_data::Field<&'a str>,
    pub shell: ::validated_data::Field<&'a str>,
}
