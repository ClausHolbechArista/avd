// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Loopback<'a, Mode> {
    pub ipv6_prefix_length: ::validated_data::Field<i64>,
}

#[::validated_data::data_view]
pub struct Mlag<'a, Mode> {
    pub algorithm: ::validated_data::Field<&'a str>,
    pub ipv4_prefix_length: ::validated_data::Field<i64>,
    pub ipv6_prefix_length: ::validated_data::Field<i64>,
}

#[::validated_data::data_view]
pub struct P2pUplinks<'a, Mode> {
    pub ipv4_prefix_length: ::validated_data::Field<i64>,
    pub ipv6_prefix_length: ::validated_data::Field<i64>,
}

#[::validated_data::data_view]
pub struct WanHa<'a, Mode> {
    pub ipv4_prefix_length: ::validated_data::Field<i64>,
}
