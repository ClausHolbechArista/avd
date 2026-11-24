// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct IpDhcpSnooping<'a, Mode> {
    pub enabled: ::data_views::Field<bool>,
    pub bridging: ::data_views::Field<bool>,
    pub information_option: ::data_views::Field<InformationOption<'a, Mode>>,
    pub vlan: ::data_views::Field<&'a str>,
}

#[::data_views::data_view]
pub struct InformationOption<'a, Mode> {
    pub enabled: ::data_views::Field<bool>,
    pub circuit_id_type: ::data_views::Field<&'a str>,
    pub circuit_id_format: ::data_views::Field<&'a str>,
}
