// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Ipv6DhcpRelay<'a, Mode> {
    pub always_on: ::data_views::Field<bool>,
    pub all_subnets: ::data_views::Field<bool>,
    pub option: ::data_views::Field<Option<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Option<'a, Mode> {
    pub link_layer_address: ::data_views::Field<bool>,
    pub remote_id_format: ::data_views::Field<&'a str>,
}
