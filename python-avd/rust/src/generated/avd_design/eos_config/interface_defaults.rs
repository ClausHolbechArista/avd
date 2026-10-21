// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct InterfaceDefaults<'a, Mode> {
    pub ethernet: ::data_views::Field<Ethernet<'a, Mode>>,
    pub mtu: ::data_views::Field<i64>,
}

#[::data_views::data_view]
pub struct Ethernet<'a, Mode> {
    pub shutdown: ::data_views::Field<bool>,
}
