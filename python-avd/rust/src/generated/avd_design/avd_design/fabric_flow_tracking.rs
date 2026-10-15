// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.FabricFlowTracking.Uplinks"))]
pub struct Uplinks<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesign.FabricFlowTracking.Downlinks"))]
pub struct Downlinks<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesign.FabricFlowTracking.Endpoints"))]
pub struct Endpoints<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesign.FabricFlowTracking.L3Edge"))]
pub struct L3Edge<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesign.FabricFlowTracking.CoreInterfaces"))]
pub struct CoreInterfaces<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesign.FabricFlowTracking.MlagInterfaces"))]
pub struct MlagInterfaces<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesign.FabricFlowTracking.L3Interfaces"))]
pub struct L3Interfaces<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesign.FabricFlowTracking.L3PortChannels"))]
pub struct L3PortChannels<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesign.FabricFlowTracking.DpsInterfaces"))]
pub struct DpsInterfaces<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesign.FabricFlowTracking.DirectWanHaLinks"))]
pub struct DirectWanHaLinks<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}
