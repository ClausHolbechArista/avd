// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct EnablePassword<'a, Mode> {
    pub disabled: ::data_views::Field<bool>,
    pub hash_algorithm: ::data_views::Field<&'a str>,
    pub key: ::data_views::Field<&'a str>,
}
