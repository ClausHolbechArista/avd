// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(list)]
pub struct StaticRoutes<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view]
pub struct Item<'a, Mode> {
    pub vrf: ::data_views::Field<&'a str>,
    pub prefix: ::data_views::RequiredValue<&'a str, Mode>,
    pub interface: ::data_views::Field<&'a str>,
    pub next_hop: ::data_views::Field<&'a str>,
    pub track_bfd: ::data_views::Field<bool>,
    pub distance: ::data_views::Field<i64>,
    pub tag: ::data_views::Field<i64>,
    pub name: ::data_views::Field<&'a str>,
    pub metric: ::data_views::Field<i64>,
}
