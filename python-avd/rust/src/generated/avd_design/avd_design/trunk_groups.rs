// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.TrunkGroups.Mlag"))]
pub struct Mlag<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesign.TrunkGroups.MlagL3"))]
pub struct MlagL3<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesign.TrunkGroups.Uplink"))]
pub struct Uplink<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
}
