// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Vrrp<'a, Mode> {
    pub ipv4: ::data_views::Field<Ipv4<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Ipv4<'a, Mode> {
    pub authentication_anti_replay: ::data_views::Field<bool>,
}
