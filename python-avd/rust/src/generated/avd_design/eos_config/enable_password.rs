// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct EnablePassword<'a, Mode> {
    pub disabled: ::validated_data::Field<bool>,
    pub hash_algorithm: ::validated_data::Field<&'a str>,
    pub key: ::validated_data::Field<&'a str>,
}
