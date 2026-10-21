// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(list)]
pub struct IpRadiusSourceInterfaces<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view]
pub struct Item<'a, Mode> {
    pub name: ::data_views::RequiredValue<&'a str, Mode>,
    pub vrf: ::data_views::Field<&'a str>,
}
