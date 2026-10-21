// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct ManagementAccounts<'a, Mode> {
    pub password: ::validated_data::Field<Password<'a, Mode>>,
}

#[::validated_data::data_view]
pub struct Password<'a, Mode> {
    pub policy: ::validated_data::Field<&'a str>,
}
