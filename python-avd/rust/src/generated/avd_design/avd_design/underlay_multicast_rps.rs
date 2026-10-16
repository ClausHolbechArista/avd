// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.UnderlayMulticastRpsItemBase", schema = Dict(1770)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1655))]
    pub rp: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(783))]
    pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
    #[data_view(schema = List(784))]
    pub groups: ::validated_data::Field<item::Groups<'a, Mode>>,
    #[data_view(schema = Str(1658))]
    pub access_list_name: ::validated_data::Field<&'a str>,
}

pub mod item {

    #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.UnderlayMulticastRpsItemBase.Nodes", schema = List(783), item_name = "AVDDesign.UnderlayMulticastRpsItemBase.NodesItem"))]
    pub struct Nodes<'a, Mode> (::validated_data::Field<nodes::Item<'a, Mode>>);

    pub mod nodes {

        #[::validated_data::data_view(python(name = "AVDDesign.UnderlayMulticastRpsItemBase.NodesItemBase", schema = Dict(1769)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(729))]
            pub name: ::validated_data::Field<&'a str>,
            #[data_view(schema = Int(152))]
            pub loopback_number: ::validated_data::RequiredValue<i64, Mode>,
            #[data_view(schema = Str(1656))]
            pub description: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(list, python(name = "AVDDesign.UnderlayMulticastRpsItemBase.Groups", schema = List(784)))]
    pub struct Groups<'a, Mode> (::validated_data::Field<&'a str>);
}
