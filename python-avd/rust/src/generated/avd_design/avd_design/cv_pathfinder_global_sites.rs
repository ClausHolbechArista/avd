// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.CvPathfinderGlobalSites", schema = List(593), item_name = "AVDDesign.CvPathfinderGlobalSitesItem"))]
pub struct CvPathfinderGlobalSites<'a, Mode> (::validated_data::Field<Item<'a, Mode>>);

#[::validated_data::data_view(python(name = "AVDDesign.CvPathfinderGlobalSitesItemBase", schema = Dict(1447)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1197))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(3))]
    pub description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1198))]
    pub location: ::validated_data::Field<&'a str>,
}
