// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct IpSoftwareForwarding<'a, Mode> {
    pub mtu: ::data_views::Field<Mtu<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Mtu<'a, Mode> {
    pub size: ::data_views::Field<i64>,
    pub exceed_action_drop: ::data_views::Field<bool>,
}
