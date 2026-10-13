// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesignFabricFlowTrackingUplinks"))]
pub struct Uplinks<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesignFabricFlowTrackingDownlinks"))]
pub struct Downlinks<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesignFabricFlowTrackingEndpoints"))]
pub struct Endpoints<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesignFabricFlowTrackingL3Edge"))]
pub struct L3Edge<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesignFabricFlowTrackingCoreInterfaces"))]
pub struct CoreInterfaces<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesignFabricFlowTrackingMlagInterfaces"))]
pub struct MlagInterfaces<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesignFabricFlowTrackingL3Interfaces"))]
pub struct L3Interfaces<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesignFabricFlowTrackingL3PortChannels"))]
pub struct L3PortChannels<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesignFabricFlowTrackingDpsInterfaces"))]
pub struct DpsInterfaces<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesignFabricFlowTrackingDirectWanHaLinks"))]
pub struct DirectWanHaLinks<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub name: ::validated_data::Field<&'a str>,
}
