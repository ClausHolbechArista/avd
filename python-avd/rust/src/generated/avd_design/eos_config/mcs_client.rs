// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct McsClient<'a, Mode> {
    pub shutdown: ::data_views::Field<bool>,
    pub cvx_secondary: ::data_views::Field<CvxSecondary<'a, Mode>>,
}

#[::data_views::data_view]
pub struct CvxSecondary<'a, Mode> {
    pub name: ::data_views::Field<&'a str>,
    pub shutdown: ::data_views::Field<bool>,
    pub server_hosts: ::data_views::Field<cvx_secondary::ServerHosts<'a, Mode>>,
    pub vrf: ::data_views::Field<&'a str>,
    pub source_interface: ::data_views::Field<&'a str>,
}

pub mod cvx_secondary {

    #[::data_views::data_view(list)]
    pub struct ServerHosts<'a, Mode> (::data_views::Field<&'a str>);
}
