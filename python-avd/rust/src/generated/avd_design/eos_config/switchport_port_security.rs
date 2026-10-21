// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct SwitchportPortSecurity<'a, Mode> {
    pub mac_address: ::data_views::Field<MacAddress<'a, Mode>>,
    pub persistence_disabled: ::data_views::Field<bool>,
    pub violation_protect_chip_based: ::data_views::Field<bool>,
}

#[::data_views::data_view]
pub struct MacAddress<'a, Mode> {
    pub aging: ::data_views::Field<bool>,
    pub moveable: ::data_views::Field<bool>,
}
