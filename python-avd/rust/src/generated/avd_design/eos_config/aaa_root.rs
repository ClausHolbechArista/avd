// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct AaaRoot<'a, Mode> {
    pub disabled: ::validated_data::Field<bool>,
    pub secret: ::validated_data::Field<Secret<'a, Mode>>,
}

#[::validated_data::data_view]
pub struct Secret<'a, Mode> {
    pub sha512_password: ::validated_data::Field<&'a str>,
}
