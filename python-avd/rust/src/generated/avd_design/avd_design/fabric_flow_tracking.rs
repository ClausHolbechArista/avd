// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Uplinks<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct Downlinks<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct Endpoints<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct L3Edge<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct CoreInterfaces<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct MlagInterfaces<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct L3Interfaces<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct L3PortChannels<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct DpsInterfaces<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct DirectWanHaLinks<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}
