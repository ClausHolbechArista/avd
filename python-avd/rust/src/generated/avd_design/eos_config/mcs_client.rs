// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct CvxSecondary<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub shutdown: ::validated_data::Field<bool>,
    pub server_hosts: ::validated_data::Field<cvx_secondary::ServerHosts<'a, Mode>>,
    pub vrf: ::validated_data::Field<&'a str>,
    pub source_interface: ::validated_data::Field<&'a str>,
}

pub mod cvx_secondary {

    #[::validated_data::data_view(list)]
    pub struct ServerHosts<'a, Mode> (::validated_data::Field<&'a str>);
}
