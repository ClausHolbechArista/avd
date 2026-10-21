// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct MonitorLoopProtection<'a, Mode> {
    pub enabled: ::data_views::Field<bool>,
    pub disabled_time: ::data_views::Field<i64>,
    pub protect_vlan: ::data_views::Field<&'a str>,
    pub rate_limit: ::data_views::Field<i64>,
    pub transmit_interval: ::data_views::Field<i64>,
}
