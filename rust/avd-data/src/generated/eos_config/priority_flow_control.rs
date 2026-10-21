// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct PriorityFlowControl<'a, Mode> {
    pub all_off: ::data_views::Field<bool>,
    pub watchdog: ::data_views::Field<Watchdog<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Watchdog<'a, Mode> {
    pub action: ::data_views::Field<&'a str>,
    pub timeout: ::data_views::Field<&'a str>,
    pub polling_interval: ::data_views::Field<&'a str>,
    pub recovery_time: ::data_views::Field<&'a str>,
    pub override_action_drop: ::data_views::Field<bool>,
}
