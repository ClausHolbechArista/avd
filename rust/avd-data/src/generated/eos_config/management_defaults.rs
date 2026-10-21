// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct ManagementDefaults<'a, Mode> {
    pub secret: ::data_views::Field<Secret<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Secret<'a, Mode> {
    pub hash: ::data_views::Field<&'a str>,
}
