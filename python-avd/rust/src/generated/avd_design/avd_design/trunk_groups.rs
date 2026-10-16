// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.TrunkGroups.Mlag", schema = Dict(1764)))]
pub struct Mlag<'a, Mode> {
    #[data_view(schema = Str(1645))]
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesign.TrunkGroups.MlagL3", schema = Dict(1765)))]
pub struct MlagL3<'a, Mode> {
    #[data_view(schema = Str(1645))]
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesign.TrunkGroups.Uplink", schema = Dict(1766)))]
pub struct Uplink<'a, Mode> {
    #[data_view(schema = Str(1646))]
    pub name: ::validated_data::Field<&'a str>,
}
