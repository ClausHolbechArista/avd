// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct InterfaceDefaults<'a, Mode> {
    pub ethernet: ::validated_data::Field<Ethernet<'a, Mode>>,
    pub mtu: ::validated_data::Field<i64>,
}

#[::validated_data::data_view]
pub struct Ethernet<'a, Mode> {
    pub shutdown: ::validated_data::Field<bool>,
}
