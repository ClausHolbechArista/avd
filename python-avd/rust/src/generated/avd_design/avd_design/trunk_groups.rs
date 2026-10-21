// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.TrunkGroups", schema = Dict(1767)))]
pub struct TrunkGroups<'a, Mode> {
    #[data_view(schema = Dict(1764))]
    pub mlag: ::data_views::Field<Mlag<'a, Mode>>,
    #[data_view(schema = Dict(1765))]
    pub mlag_l3: ::data_views::Field<MlagL3<'a, Mode>>,
    #[data_view(schema = Dict(1766))]
    pub uplink: ::data_views::Field<Uplink<'a, Mode>>,
}

#[::data_views::data_view(python(name = "AVDDesign.TrunkGroups.Mlag", schema = Dict(1764)))]
pub struct Mlag<'a, Mode> {
    #[data_view(schema = Str(1645))]
    pub name: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(python(name = "AVDDesign.TrunkGroups.MlagL3", schema = Dict(1765)))]
pub struct MlagL3<'a, Mode> {
    #[data_view(schema = Str(1645))]
    pub name: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(python(name = "AVDDesign.TrunkGroups.Uplink", schema = Dict(1766)))]
pub struct Uplink<'a, Mode> {
    #[data_view(schema = Str(1646))]
    pub name: ::data_views::Field<&'a str>,
}
