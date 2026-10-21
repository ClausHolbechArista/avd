// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct AaaRoot<'a, Mode> {
    pub disabled: ::data_views::Field<bool>,
    pub secret: ::data_views::Field<Secret<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Secret<'a, Mode> {
    pub sha512_password: ::data_views::Field<&'a str>,
}
