// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.FabricSflow", schema = Dict(1572)))]
pub struct FabricSflow<'a, Mode> {
    #[data_view(schema = Bool(526))]
    pub uplinks: ::data_views::Field<bool>,
    #[data_view(schema = Bool(527))]
    pub downlinks: ::data_views::Field<bool>,
    #[data_view(schema = Bool(528))]
    pub endpoints: ::data_views::Field<bool>,
    #[data_view(schema = Bool(529))]
    pub l3_edge: ::data_views::Field<bool>,
    #[data_view(schema = Bool(530))]
    pub core_interfaces: ::data_views::Field<bool>,
    #[data_view(schema = Bool(531))]
    pub mlag_interfaces: ::data_views::Field<bool>,
    #[data_view(schema = Bool(532))]
    pub l3_interfaces: ::data_views::Field<bool>,
}
