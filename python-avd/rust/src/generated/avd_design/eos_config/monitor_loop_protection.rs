// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct MonitorLoopProtection<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub disabled_time: ::validated_data::Field<i64>,
    pub protect_vlan: ::validated_data::Field<&'a str>,
    pub rate_limit: ::validated_data::Field<i64>,
    pub transmit_interval: ::validated_data::Field<i64>,
}
