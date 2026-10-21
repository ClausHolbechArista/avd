// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "AVDDesign.Redundancy", schema = Dict(1738)))]
pub struct Redundancy<'a, Mode> {
    #[data_view(schema = Str(1620))]
    pub protocol: ::validated_data::Field<&'a str>,
}
