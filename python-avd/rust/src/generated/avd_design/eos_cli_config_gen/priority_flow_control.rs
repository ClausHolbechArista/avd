// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Watchdog<'a, Mode> {
    pub action: ::validated_data::Field<&'a str>,
    pub timeout: ::validated_data::Field<&'a str>,
    pub polling_interval: ::validated_data::Field<&'a str>,
    pub recovery_time: ::validated_data::Field<&'a str>,
    pub override_action_drop: ::validated_data::Field<bool>,
}
