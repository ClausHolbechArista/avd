// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.DigitalTwin.Fabric", schema = Dict(1522)))]
pub struct Fabric<'a, Mode> {
    #[data_view(schema = Str(1378))]
    pub act_os_version: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1379))]
    pub act_username: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1380))]
    pub act_password: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(489))]
    pub act_internet_access: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(490))]
    pub act_ensure_eapi_access: ::validated_data::Field<bool>,
}
