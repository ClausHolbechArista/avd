// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct ManagementCvx<'a, Mode> {
    pub shutdown: ::validated_data::Field<bool>,
    pub server_hosts: ::validated_data::Field<ServerHosts<'a, Mode>>,
    pub source_interface: ::validated_data::Field<&'a str>,
    pub vrf: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(list)]
pub struct ServerHosts<'a, Mode> (::validated_data::Field<&'a str>);
