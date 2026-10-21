// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct ManagementCvx<'a, Mode> {
    pub shutdown: ::data_views::Field<bool>,
    pub server_hosts: ::data_views::Field<ServerHosts<'a, Mode>>,
    pub source_interface: ::data_views::Field<&'a str>,
    pub vrf: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(list)]
pub struct ServerHosts<'a, Mode> (::data_views::Field<&'a str>);
