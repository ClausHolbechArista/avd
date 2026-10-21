// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.CvPathfinderRegions", schema = List(596), item_name = "AVDDesign.CvPathfinderRegionsItem"))]
pub struct CvPathfinderRegions<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.CvPathfinderRegionsItemBase", schema = Dict(1452)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1203))]
    pub name: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(3))]
    pub description: ::data_views::Field<&'a str>,
    #[data_view(schema = Int(525))]
    pub id: ::data_views::RequiredValue<i64, Mode>,
    #[data_view(schema = List(595))]
    pub sites: ::data_views::Field<item::Sites<'a, Mode>>,
}

pub mod item {

    #[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.CvPathfinderRegionsItemBase.Sites", schema = List(595), item_name = "AVDDesign.CvPathfinderRegionsItemBase.SitesItem"))]
    pub struct Sites<'a, Mode> (::data_views::Field<sites::Item<'a, Mode>>);

    pub mod sites {

        #[::data_views::data_view(python(name = "AVDDesign.CvPathfinderRegionsItemBase.SitesItemBase", schema = Dict(1451)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(1204))]
            pub name: ::data_views::RequiredValue<&'a str, Mode>,
            #[data_view(schema = Str(3))]
            pub description: ::data_views::Field<&'a str>,
            #[data_view(schema = Int(526))]
            pub id: ::data_views::RequiredValue<i64, Mode>,
            #[data_view(schema = Str(1198))]
            pub location: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(3))]
            pub site_contact: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(3))]
            pub site_after_hours_contact: ::data_views::Field<&'a str>,
        }
    }
}
