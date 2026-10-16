// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.InternalVlanOrder.Range", schema = Dict(1592)))]
pub struct Range<'a, Mode> {
    #[data_view(schema = Int(494))]
    pub beginning: ::validated_data::RequiredValue<i64, Mode>,
    #[data_view(schema = Int(495))]
    pub ending: ::validated_data::RequiredValue<i64, Mode>,
}
