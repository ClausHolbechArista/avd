// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.IpsecSettings", schema = Dict(1594)))]
pub struct IpsecSettings<'a, Mode> {
    #[data_view(schema = Bool(540))]
    pub bind_connection_to_interface: ::data_views::Field<bool>,
}
