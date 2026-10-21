// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "AVDDesign.IpsecSettings", schema = Dict(1594)))]
pub struct IpsecSettings<'a, Mode> {
    #[data_view(schema = Bool(540))]
    pub bind_connection_to_interface: ::validated_data::Field<bool>,
}
