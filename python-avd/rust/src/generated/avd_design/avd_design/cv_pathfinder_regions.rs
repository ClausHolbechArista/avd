// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.CvPathfinderRegionsItemBase", schema = Dict(1452)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1203))]
    pub name: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(3))]
    pub description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(525))]
    pub id: ::validated_data::RequiredValue<i64, Mode>,
    #[data_view(schema = List(595))]
    pub sites: ::validated_data::Field<item::Sites<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.CvPathfinderRegionsItemBase.Sites", schema = List(595), item_name = "AVDDesign.CvPathfinderRegionsItemBase.SitesItem"))]
    pub struct Sites<'a, Mode> (::validated_data::Field<sites::Item<'a, Mode>>);

    pub mod sites {

        #[::validated_data::data_view(python(name = "AVDDesign.CvPathfinderRegionsItemBase.SitesItemBase", schema = Dict(1451)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(1204))]
            pub name: ::validated_data::RequiredValue<&'a str, Mode>,
            #[data_view(schema = Str(3))]
            pub description: ::validated_data::Field<&'a str>,
            #[data_view(schema = Int(526))]
            pub id: ::validated_data::RequiredValue<i64, Mode>,
            #[data_view(schema = Str(1198))]
            pub location: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(3))]
            pub site_contact: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(3))]
            pub site_after_hours_contact: ::validated_data::Field<&'a str>,
        }
    }
}
