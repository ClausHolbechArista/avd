// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.BfdMultihop", schema = Dict(1390)))]
pub struct BfdMultihop<'a, Mode> {
    #[data_view(schema = Int(497))]
    pub interval: ::data_views::RequiredValue<i64, Mode>,
    #[data_view(schema = Int(497))]
    pub min_rx: ::data_views::RequiredValue<i64, Mode>,
    #[data_view(schema = Int(380))]
    pub multiplier: ::data_views::RequiredValue<i64, Mode>,
}
