// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.EvpnHostflapDetection", schema = Dict(1551)))]
pub struct EvpnHostflapDetection<'a, Mode> {
    #[data_view(schema = Bool(519))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = Int(573))]
    pub threshold: ::data_views::Field<i64>,
    #[data_view(schema = Int(574))]
    pub window: ::data_views::Field<i64>,
    #[data_view(schema = Int(575))]
    pub expiry_timeout: ::data_views::Field<i64>,
}
