// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.FabricFlowTracking", schema = Dict(1564)))]
pub struct FabricFlowTracking<'a, Mode> {
    #[data_view(schema = Dict(1554))]
    pub uplinks: ::data_views::Field<Uplinks<'a, Mode>>,
    #[data_view(schema = Dict(1555))]
    pub downlinks: ::data_views::Field<Downlinks<'a, Mode>>,
    #[data_view(schema = Dict(1556))]
    pub endpoints: ::data_views::Field<Endpoints<'a, Mode>>,
    #[data_view(schema = Dict(1557))]
    pub l3_edge: ::data_views::Field<L3Edge<'a, Mode>>,
    #[data_view(schema = Dict(1558))]
    pub core_interfaces: ::data_views::Field<CoreInterfaces<'a, Mode>>,
    #[data_view(schema = Dict(1559))]
    pub mlag_interfaces: ::data_views::Field<MlagInterfaces<'a, Mode>>,
    #[data_view(schema = Dict(1560))]
    pub l3_interfaces: ::data_views::Field<L3Interfaces<'a, Mode>>,
    #[data_view(schema = Dict(1561))]
    pub l3_port_channels: ::data_views::Field<L3PortChannels<'a, Mode>>,
    #[data_view(schema = Dict(1562))]
    pub dps_interfaces: ::data_views::Field<DpsInterfaces<'a, Mode>>,
    #[data_view(schema = Dict(1563))]
    pub direct_wan_ha_links: ::data_views::Field<DirectWanHaLinks<'a, Mode>>,
}

#[::data_views::data_view(python(name = "AVDDesign.FabricFlowTracking.Uplinks", schema = Dict(1554)))]
pub struct Uplinks<'a, Mode> {
    #[data_view(schema = Bool(340))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = Str(1401))]
    pub name: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(python(name = "AVDDesign.FabricFlowTracking.Downlinks", schema = Dict(1555)))]
pub struct Downlinks<'a, Mode> {
    #[data_view(schema = Bool(340))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = Str(1401))]
    pub name: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(python(name = "AVDDesign.FabricFlowTracking.Endpoints", schema = Dict(1556)))]
pub struct Endpoints<'a, Mode> {
    #[data_view(schema = Bool(340))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = Str(1401))]
    pub name: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(python(name = "AVDDesign.FabricFlowTracking.L3Edge", schema = Dict(1557)))]
pub struct L3Edge<'a, Mode> {
    #[data_view(schema = Bool(340))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = Str(1401))]
    pub name: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(python(name = "AVDDesign.FabricFlowTracking.CoreInterfaces", schema = Dict(1558)))]
pub struct CoreInterfaces<'a, Mode> {
    #[data_view(schema = Bool(340))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = Str(1401))]
    pub name: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(python(name = "AVDDesign.FabricFlowTracking.MlagInterfaces", schema = Dict(1559)))]
pub struct MlagInterfaces<'a, Mode> {
    #[data_view(schema = Bool(340))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = Str(1401))]
    pub name: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(python(name = "AVDDesign.FabricFlowTracking.L3Interfaces", schema = Dict(1560)))]
pub struct L3Interfaces<'a, Mode> {
    #[data_view(schema = Bool(340))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = Str(1401))]
    pub name: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(python(name = "AVDDesign.FabricFlowTracking.L3PortChannels", schema = Dict(1561)))]
pub struct L3PortChannels<'a, Mode> {
    #[data_view(schema = Bool(340))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = Str(1401))]
    pub name: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(python(name = "AVDDesign.FabricFlowTracking.DpsInterfaces", schema = Dict(1562)))]
pub struct DpsInterfaces<'a, Mode> {
    #[data_view(schema = Bool(37))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = Str(1401))]
    pub name: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(python(name = "AVDDesign.FabricFlowTracking.DirectWanHaLinks", schema = Dict(1563)))]
pub struct DirectWanHaLinks<'a, Mode> {
    #[data_view(schema = Bool(340))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = Str(1401))]
    pub name: ::data_views::Field<&'a str>,
}
