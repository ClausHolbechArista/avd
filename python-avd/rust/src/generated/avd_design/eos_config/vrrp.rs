// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct Vrrp<'a, Mode> {
    pub ipv4: ::validated_data::Field<Ipv4<'a, Mode>>,
}

#[::validated_data::data_view]
pub struct Ipv4<'a, Mode> {
    pub authentication_anti_replay: ::validated_data::Field<bool>,
}
