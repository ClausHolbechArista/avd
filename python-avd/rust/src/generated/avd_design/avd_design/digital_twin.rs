// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.DigitalTwin", schema = Dict(1523)))]
pub struct DigitalTwin<'a, Mode> {
    #[data_view(schema = Str(1377))]
    pub environment: ::data_views::Field<&'a str>,
    #[data_view(schema = Dict(1522))]
    pub fabric: ::data_views::RequiredValue<Fabric<'a, Mode>, Mode>,
    #[data_view(schema = Bool(491))]
    pub use_default_interfaces_of_digital_twin_platform: ::data_views::Field<bool>,
}

#[::data_views::data_view(python(name = "AVDDesign.DigitalTwin.Fabric", schema = Dict(1522)))]
pub struct Fabric<'a, Mode> {
    #[data_view(schema = Str(1378))]
    pub act_os_version: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1379))]
    pub act_username: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1380))]
    pub act_password: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(489))]
    pub act_internet_access: ::data_views::Field<bool>,
    #[data_view(schema = Bool(490))]
    pub act_ensure_eapi_access: ::data_views::Field<bool>,
}
