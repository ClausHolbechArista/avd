// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(hostname), python(name = "AVDDesign.CvTopology", schema = List(601), item_name = "AVDDesign.CvTopologyItem"))]
pub struct CvTopology<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.CvTopologyItemBase", schema = Dict(1460)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(3))]
    pub hostname: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(2))]
    pub platform: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = List(600))]
    pub interfaces: ::data_views::RequiredValue<item::Interfaces<'a, Mode>, Mode>,
}

pub mod item {

    #[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.CvTopologyItemBase.Interfaces", schema = List(600), item_name = "AVDDesign.CvTopologyItemBase.InterfacesItem"))]
    pub struct Interfaces<'a, Mode> (::data_views::Field<interfaces::Item<'a, Mode>>);

    pub mod interfaces {

        #[::data_views::data_view(python(name = "AVDDesign.CvTopologyItemBase.InterfacesItemBase", schema = Dict(1459)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(3))]
            pub name: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(3))]
            pub neighbor: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(3))]
            pub neighbor_interface: ::data_views::Field<&'a str>,
        }
    }
}
