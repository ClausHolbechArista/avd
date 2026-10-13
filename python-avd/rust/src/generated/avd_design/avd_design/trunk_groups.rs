// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesignTrunkGroupsMlag"))]
pub struct Mlag<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesignTrunkGroupsMlagL3"))]
pub struct MlagL3<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesignTrunkGroupsUplink"))]
pub struct Uplink<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
}
