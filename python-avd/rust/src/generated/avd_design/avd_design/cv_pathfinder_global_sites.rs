// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.CvPathfinderGlobalSites", schema = List(593), item_name = "AVDDesign.CvPathfinderGlobalSitesItem"))]
pub struct CvPathfinderGlobalSites<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.CvPathfinderGlobalSitesItemBase", schema = Dict(1447)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1197))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(3))]
    pub description: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1198))]
    pub location: ::data_views::Field<&'a str>,
}
