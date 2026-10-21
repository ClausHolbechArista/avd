// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Boot<'a, Mode> {
    pub secret: ::data_views::Field<Secret<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Secret<'a, Mode> {
    pub hash_algorithm: ::data_views::Field<&'a str>,
    pub key: ::data_views::Field<&'a str>,
}
