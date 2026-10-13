// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesignZscalerEndpointsPrimary"))]
pub struct Primary<'a, Mode> {
    pub ip_address: ::validated_data::RequiredValue<&'a str, Mode>,
    pub datacenter: ::validated_data::RequiredValue<&'a str, Mode>,
    pub city: ::validated_data::RequiredValue<&'a str, Mode>,
    pub country: ::validated_data::RequiredValue<&'a str, Mode>,
    pub region: ::validated_data::RequiredValue<&'a str, Mode>,
    pub latitude: ::validated_data::RequiredValue<&'a str, Mode>,
    pub longitude: ::validated_data::RequiredValue<&'a str, Mode>,
}

#[::validated_data::data_view(python(name = "AVDDesignZscalerEndpointsSecondary"))]
pub struct Secondary<'a, Mode> {
    pub ip_address: ::validated_data::RequiredValue<&'a str, Mode>,
    pub datacenter: ::validated_data::RequiredValue<&'a str, Mode>,
    pub city: ::validated_data::RequiredValue<&'a str, Mode>,
    pub country: ::validated_data::RequiredValue<&'a str, Mode>,
    pub region: ::validated_data::RequiredValue<&'a str, Mode>,
    pub latitude: ::validated_data::RequiredValue<&'a str, Mode>,
    pub longitude: ::validated_data::RequiredValue<&'a str, Mode>,
}

#[::validated_data::data_view(python(name = "AVDDesignZscalerEndpointsTertiary"))]
pub struct Tertiary<'a, Mode> {
    pub ip_address: ::validated_data::RequiredValue<&'a str, Mode>,
    pub datacenter: ::validated_data::RequiredValue<&'a str, Mode>,
    pub city: ::validated_data::RequiredValue<&'a str, Mode>,
    pub country: ::validated_data::RequiredValue<&'a str, Mode>,
    pub region: ::validated_data::RequiredValue<&'a str, Mode>,
    pub latitude: ::validated_data::RequiredValue<&'a str, Mode>,
    pub longitude: ::validated_data::RequiredValue<&'a str, Mode>,
}

#[::validated_data::data_view(python(name = "AVDDesignZscalerEndpointsDeviceLocation"))]
pub struct DeviceLocation<'a, Mode> {
    pub city: ::validated_data::RequiredValue<&'a str, Mode>,
    pub country: ::validated_data::RequiredValue<&'a str, Mode>,
}
