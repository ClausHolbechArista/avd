// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Poe<'a, Mode> {
    pub reboot: ::data_views::Field<Reboot<'a, Mode>>,
    pub interface_shutdown: ::data_views::Field<InterfaceShutdown<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Reboot<'a, Mode> {
    pub action: ::data_views::Field<&'a str>,
}

#[::data_views::data_view]
pub struct InterfaceShutdown<'a, Mode> {
    pub action: ::data_views::Field<&'a str>,
}
