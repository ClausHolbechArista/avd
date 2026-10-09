// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Mtu<'a, Mode> {
    pub size: ::validated_data::Field<i64>,
    pub exceed_action_drop: ::validated_data::Field<bool>,
}
