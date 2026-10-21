// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "AVDDesign.FabricSflow", schema = Dict(1572)))]
pub struct FabricSflow<'a, Mode> {
    #[data_view(schema = Bool(526))]
    pub uplinks: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(527))]
    pub downlinks: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(528))]
    pub endpoints: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(529))]
    pub l3_edge: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(530))]
    pub core_interfaces: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(531))]
    pub mlag_interfaces: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(532))]
    pub l3_interfaces: ::validated_data::Field<bool>,
}
