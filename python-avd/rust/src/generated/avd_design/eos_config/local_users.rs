// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct LocalUsers<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view]
pub struct Item<'a, Mode> {
    pub name: ::data_views::Field<&'a str>,
    pub disabled: ::data_views::Field<bool>,
    pub privilege: ::data_views::Field<i64>,
    pub role: ::data_views::Field<&'a str>,
    pub sha512_password: ::data_views::Field<&'a str>,
    pub no_password: ::data_views::Field<bool>,
    pub ssh_key: ::data_views::Field<&'a str>,
    pub secondary_ssh_key: ::data_views::Field<&'a str>,
    pub shell: ::data_views::Field<&'a str>,
}
