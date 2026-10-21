// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct VmtracerSessions<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view]
pub struct Item<'a, Mode> {
    pub name: ::data_views::Field<&'a str>,
    pub url: ::data_views::Field<&'a str>,
    pub username: ::data_views::Field<&'a str>,
    pub password: ::data_views::Field<&'a str>,
    pub autovlan_disable: ::data_views::Field<bool>,
    pub vrf: ::data_views::Field<&'a str>,
    pub source_interface: ::data_views::Field<&'a str>,
}
