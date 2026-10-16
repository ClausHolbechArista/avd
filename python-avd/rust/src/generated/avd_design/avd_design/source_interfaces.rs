// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.SourceInterfaces.HttpClient", schema = Dict(1750)))]
pub struct HttpClient<'a, Mode> {
    #[data_view(schema = Bool(652))]
    pub mgmt_interface: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(653))]
    pub inband_mgmt_interface: ::validated_data::Field<bool>,
}

#[::validated_data::data_view(python(name = "AVDDesign.SourceInterfaces.SshClient", schema = Dict(1751)))]
pub struct SshClient<'a, Mode> {
    #[data_view(schema = Bool(654))]
    pub mgmt_interface: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(655))]
    pub inband_mgmt_interface: ::validated_data::Field<bool>,
}
