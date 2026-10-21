// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(indexed_list, primary_key(node_type), python(name = "AVDDesign.DefaultNodeTypes", schema = List(611), item_name = "AVDDesign.DefaultNodeTypesItem"))]
pub struct DefaultNodeTypes<'a, Mode> (::validated_data::Field<Item<'a, Mode>>);

#[::validated_data::data_view(python(name = "AVDDesign.DefaultNodeTypesItemBase", schema = Dict(1463)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1230))]
    pub node_type: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(610))]
    pub match_hostnames: ::validated_data::RequiredValue<item::MatchHostnames<'a, Mode>, Mode>,
}

pub mod item {

    #[::validated_data::data_view(list, python(name = "AVDDesign.DefaultNodeTypesItemBase.MatchHostnames", schema = List(610)))]
    pub struct MatchHostnames<'a, Mode> (::validated_data::RequiredValue<&'a str, Mode>);
}
