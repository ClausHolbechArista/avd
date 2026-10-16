// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.ZscalerEndpoints.Primary", schema = Dict(1798)))]
pub struct Primary<'a, Mode> {
    #[data_view(schema = Str(2))]
    pub ip_address: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub datacenter: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub city: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub country: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub region: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(1698))]
    pub latitude: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(1698))]
    pub longitude: ::validated_data::RequiredValue<&'a str, Mode>,
}

#[::validated_data::data_view(python(name = "AVDDesign.ZscalerEndpoints.Secondary", schema = Dict(1799)))]
pub struct Secondary<'a, Mode> {
    #[data_view(schema = Str(2))]
    pub ip_address: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub datacenter: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub city: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub country: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub region: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(1698))]
    pub latitude: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(1698))]
    pub longitude: ::validated_data::RequiredValue<&'a str, Mode>,
}

#[::validated_data::data_view(python(name = "AVDDesign.ZscalerEndpoints.Tertiary", schema = Dict(1799)))]
pub struct Tertiary<'a, Mode> {
    #[data_view(schema = Str(2))]
    pub ip_address: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub datacenter: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub city: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub country: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub region: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(1698))]
    pub latitude: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(1698))]
    pub longitude: ::validated_data::RequiredValue<&'a str, Mode>,
}

#[::validated_data::data_view(python(name = "AVDDesign.ZscalerEndpoints.DeviceLocation", schema = Dict(1800)))]
pub struct DeviceLocation<'a, Mode> {
    #[data_view(schema = Str(2))]
    pub city: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(2))]
    pub country: ::validated_data::RequiredValue<&'a str, Mode>,
}
