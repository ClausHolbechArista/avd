// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Phone<'a, Mode> {
    pub cos: ::validated_data::Field<i64>,
    pub trunk: ::validated_data::Field<&'a str>,
    pub vlan: ::validated_data::Field<i64>,
    pub access_list_bypass: ::validated_data::Field<bool>,
    pub qos_trust: ::validated_data::Field<&'a str>,
}
