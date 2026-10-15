// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.UnderlayMulticastRpsItemBase"))]
pub struct Item<'a, Mode> {
    pub rp: ::validated_data::Field<&'a str>,
    pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
    pub groups: ::validated_data::Field<item::Groups<'a, Mode>>,
    pub access_list_name: ::validated_data::Field<&'a str>,
}

pub mod item {

    #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.UnderlayMulticastRpsItemBase.Nodes", item_name = "AVDDesign.UnderlayMulticastRpsItemBase.NodesItem"))]
    pub struct Nodes<'a, Mode> (::validated_data::Field<nodes::Item<'a, Mode>>);

    pub mod nodes {

        #[::validated_data::data_view(python(name = "AVDDesign.UnderlayMulticastRpsItemBase.NodesItemBase"))]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub loopback_number: ::validated_data::RequiredValue<i64, Mode>,
            pub description: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(list, python(name = "AVDDesign.UnderlayMulticastRpsItemBase.Groups"))]
    pub struct Groups<'a, Mode> (::validated_data::Field<&'a str>);
}
