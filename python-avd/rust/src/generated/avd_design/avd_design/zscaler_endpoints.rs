// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.ZscalerEndpoints", schema = Dict(1801)))]
pub struct ZscalerEndpoints<'a, Mode> {
    #[data_view(schema = Dict(1798))]
    pub primary: ::data_views::RequiredValue<Primary<'a, Mode>, Mode>,
    #[data_view(schema = Dict(1799))]
    pub secondary: ::data_views::Field<Secondary<'a, Mode>>,
    #[data_view(schema = Dict(1799))]
    pub tertiary: ::data_views::Field<Tertiary<'a, Mode>>,
    #[data_view(schema = Str(1699))]
    pub cloud_name: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Dict(1800))]
    pub device_location: ::data_views::RequiredValue<DeviceLocation<'a, Mode>, Mode>,
}

#[::data_views::data_view(python(name = "AVDDesign.ZscalerEndpoints.Primary", schema = Dict(1798)))]
pub struct Primary<'a, Mode> {
    #[data_view(schema = Str(2))]
    pub ip_address: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub datacenter: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub city: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub country: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub region: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(1698))]
    pub latitude: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(1698))]
    pub longitude: ::data_views::RequiredValue<&'a str, Mode>,
}

#[::data_views::data_view(python(name = "AVDDesign.ZscalerEndpoints.Secondary", schema = Dict(1799)))]
pub struct Secondary<'a, Mode> {
    #[data_view(schema = Str(2))]
    pub ip_address: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub datacenter: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub city: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub country: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub region: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(1698))]
    pub latitude: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(1698))]
    pub longitude: ::data_views::RequiredValue<&'a str, Mode>,
}

#[::data_views::data_view(python(name = "AVDDesign.ZscalerEndpoints.Tertiary", schema = Dict(1799)))]
pub struct Tertiary<'a, Mode> {
    #[data_view(schema = Str(2))]
    pub ip_address: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub datacenter: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub city: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub country: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub region: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(1698))]
    pub latitude: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(1698))]
    pub longitude: ::data_views::RequiredValue<&'a str, Mode>,
}

#[::data_views::data_view(python(name = "AVDDesign.ZscalerEndpoints.DeviceLocation", schema = Dict(1800)))]
pub struct DeviceLocation<'a, Mode> {
    #[data_view(schema = Str(2))]
    pub city: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub country: ::data_views::RequiredValue<&'a str, Mode>,
}
