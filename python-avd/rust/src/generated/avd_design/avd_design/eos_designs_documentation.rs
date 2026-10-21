// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.EosDesignsDocumentation", schema = Dict(1545)))]
pub struct EosDesignsDocumentation<'a, Mode> {
    #[data_view(schema = Bool(511))]
    pub enable: ::data_views::Field<bool>,
    #[data_view(schema = Bool(512))]
    pub connected_endpoints: ::data_views::Field<bool>,
    #[data_view(schema = Bool(513))]
    pub topology_csv: ::data_views::Field<bool>,
    #[data_view(schema = Bool(514))]
    pub p2p_links_csv: ::data_views::Field<bool>,
    #[data_view(schema = Bool(515))]
    pub toc: ::data_views::Field<bool>,
}
