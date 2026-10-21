// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct SwitchportDefault<'a, Mode> {
    pub mode: ::data_views::Field<&'a str>,
    pub phone: ::data_views::Field<Phone<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Phone<'a, Mode> {
    pub cos: ::data_views::Field<i64>,
    pub trunk: ::data_views::Field<&'a str>,
    pub vlan: ::data_views::Field<i64>,
    pub access_list_bypass: ::data_views::Field<bool>,
    pub qos_trust: ::data_views::Field<&'a str>,
}
