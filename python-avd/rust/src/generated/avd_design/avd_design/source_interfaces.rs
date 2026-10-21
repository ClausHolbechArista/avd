// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.SourceInterfaces", schema = Dict(1756)))]
pub struct SourceInterfaces<'a, Mode> {
    #[data_view(schema = Dict(1750))]
    pub http_client: ::data_views::Field<HttpClient<'a, Mode>>,
    #[data_view(schema = Dict(1751))]
    pub ssh_client: ::data_views::Field<SshClient<'a, Mode>>,
}

#[::data_views::data_view(python(name = "AVDDesign.SourceInterfaces.HttpClient", schema = Dict(1750)))]
pub struct HttpClient<'a, Mode> {
    #[data_view(schema = Bool(652))]
    pub mgmt_interface: ::data_views::Field<bool>,
    #[data_view(schema = Bool(653))]
    pub inband_mgmt_interface: ::data_views::Field<bool>,
}

#[::data_views::data_view(python(name = "AVDDesign.SourceInterfaces.SshClient", schema = Dict(1751)))]
pub struct SshClient<'a, Mode> {
    #[data_view(schema = Bool(654))]
    pub mgmt_interface: ::data_views::Field<bool>,
    #[data_view(schema = Bool(655))]
    pub inband_mgmt_interface: ::data_views::Field<bool>,
}
