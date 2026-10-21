// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "AVDDesign.EosDesignsDocumentation", schema = Dict(1545)))]
pub struct EosDesignsDocumentation<'a, Mode> {
    #[data_view(schema = Bool(511))]
    pub enable: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(512))]
    pub connected_endpoints: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(513))]
    pub topology_csv: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(514))]
    pub p2p_links_csv: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(515))]
    pub toc: ::validated_data::Field<bool>,
}
