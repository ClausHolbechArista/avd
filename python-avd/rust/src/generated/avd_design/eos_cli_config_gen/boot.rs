// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Secret<'a, Mode> {
    pub hash_algorithm: ::validated_data::Field<&'a str>,
    pub key: ::validated_data::Field<&'a str>,
}
