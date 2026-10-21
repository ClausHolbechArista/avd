// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(node_type), python(name = "AVDDesign.DefaultNodeTypes", schema = List(611), item_name = "AVDDesign.DefaultNodeTypesItem"))]
pub struct DefaultNodeTypes<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.DefaultNodeTypesItemBase", schema = Dict(1463)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1230))]
    pub node_type: ::data_views::Field<&'a str>,
    #[data_view(schema = List(610))]
    pub match_hostnames: ::data_views::RequiredValue<item::MatchHostnames<'a, Mode>, Mode>,
}

pub mod item {

    #[::data_views::data_view(list, python(name = "AVDDesign.DefaultNodeTypesItemBase.MatchHostnames", schema = List(610)))]
    pub struct MatchHostnames<'a, Mode> (::data_views::RequiredValue<&'a str, Mode>);
}
