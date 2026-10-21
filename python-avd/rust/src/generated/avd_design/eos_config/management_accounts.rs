// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct ManagementAccounts<'a, Mode> {
    pub password: ::data_views::Field<Password<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Password<'a, Mode> {
    pub policy: ::data_views::Field<&'a str>,
}
