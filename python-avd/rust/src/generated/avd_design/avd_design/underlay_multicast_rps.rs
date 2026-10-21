// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(rp), python(name = "AVDDesign.UnderlayMulticastRps", schema = List(785), item_name = "AVDDesign.UnderlayMulticastRpsItem"))]
pub struct UnderlayMulticastRps<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.UnderlayMulticastRpsItemBase", schema = Dict(1770)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1655))]
    pub rp: ::data_views::Field<&'a str>,
    #[data_view(schema = List(783))]
    pub nodes: ::data_views::Field<item::Nodes<'a, Mode>>,
    #[data_view(schema = List(784))]
    pub groups: ::data_views::Field<item::Groups<'a, Mode>>,
    #[data_view(schema = Str(1658))]
    pub access_list_name: ::data_views::Field<&'a str>,
}

pub mod item {

    #[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.UnderlayMulticastRpsItemBase.Nodes", schema = List(783), item_name = "AVDDesign.UnderlayMulticastRpsItemBase.NodesItem"))]
    pub struct Nodes<'a, Mode> (::data_views::Field<nodes::Item<'a, Mode>>);

    pub mod nodes {

        #[::data_views::data_view(python(name = "AVDDesign.UnderlayMulticastRpsItemBase.NodesItemBase", schema = Dict(1769)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(729))]
            pub name: ::data_views::Field<&'a str>,
            #[data_view(schema = Int(152))]
            pub loopback_number: ::data_views::RequiredValue<i64, Mode>,
            #[data_view(schema = Str(1656))]
            pub description: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view(list, python(name = "AVDDesign.UnderlayMulticastRpsItemBase.Groups", schema = List(784)))]
    pub struct Groups<'a, Mode> (::data_views::Field<&'a str>);
}
