// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "AVDDesign.BfdMultihop", schema = Dict(1390)))]
pub struct BfdMultihop<'a, Mode> {
    #[data_view(schema = Int(497))]
    pub interval: ::validated_data::RequiredValue<i64, Mode>,
    #[data_view(schema = Int(497))]
    pub min_rx: ::validated_data::RequiredValue<i64, Mode>,
    #[data_view(schema = Int(380))]
    pub multiplier: ::validated_data::RequiredValue<i64, Mode>,
}
