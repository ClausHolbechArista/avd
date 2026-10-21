// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "AVDDesign.FabricNumbering", schema = Dict(1571)))]
pub struct FabricNumbering<'a, Mode> {
    #[data_view(schema = Dict(1570))]
    pub node_id: ::validated_data::Field<NodeId<'a, Mode>>,
}

#[::validated_data::data_view(python(name = "AVDDesign.FabricNumbering.NodeId", schema = Dict(1570)))]
pub struct NodeId<'a, Mode> {
    #[data_view(schema = Str(1404))]
    pub algorithm: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1405))]
    pub pools_file: ::validated_data::Field<&'a str>,
}
