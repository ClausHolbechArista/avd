// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "AVDDesign.EvpnHostflapDetection", schema = Dict(1551)))]
pub struct EvpnHostflapDetection<'a, Mode> {
    #[data_view(schema = Bool(519))]
    pub enabled: ::validated_data::Field<bool>,
    #[data_view(schema = Int(573))]
    pub threshold: ::validated_data::Field<i64>,
    #[data_view(schema = Int(574))]
    pub window: ::validated_data::Field<i64>,
    #[data_view(schema = Int(575))]
    pub expiry_timeout: ::validated_data::Field<i64>,
}
