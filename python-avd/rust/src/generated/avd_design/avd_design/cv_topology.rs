// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.CvTopologyItemBase", schema = Dict(1460)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(3))]
    pub hostname: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(2))]
    pub platform: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = List(600))]
    pub interfaces: ::validated_data::RequiredValue<item::Interfaces<'a, Mode>, Mode>,
}

pub mod item {

    #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.CvTopologyItemBase.Interfaces", schema = List(600), item_name = "AVDDesign.CvTopologyItemBase.InterfacesItem"))]
    pub struct Interfaces<'a, Mode> (::validated_data::Field<interfaces::Item<'a, Mode>>);

    pub mod interfaces {

        #[::validated_data::data_view(python(name = "AVDDesign.CvTopologyItemBase.InterfacesItemBase", schema = Dict(1459)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(3))]
            pub name: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(3))]
            pub neighbor: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(3))]
            pub neighbor_interface: ::validated_data::Field<&'a str>,
        }
    }
}
