// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Lldp<'a, Mode> {
    pub transmit: ::validated_data::Field<bool>,
    pub receive: ::validated_data::Field<bool>,
    pub ztp_vlan: ::validated_data::Field<i64>,
}
