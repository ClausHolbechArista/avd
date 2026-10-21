// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "AVDDesign.SpanningTreeSettings", schema = Dict(1758)))]
pub struct SpanningTreeSettings<'a, Mode> {
    #[data_view(schema = Str(1638))]
    pub mode: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(633))]
    pub priority: ::validated_data::Field<i64>,
    #[data_view(schema = Dict(1757))]
    pub port_id_allocation_port_channel_range: ::validated_data::Field<super::super::eos_config::spanning_tree::PortIdAllocationPortChannelRange<'a, Mode>>,
    #[data_view(schema = Bool(656))]
    pub loop_guard_default: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(657))]
    pub edge_port_bpduguard_default: ::validated_data::Field<bool>,
}
