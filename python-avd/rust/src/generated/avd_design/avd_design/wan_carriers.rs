// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.WanCarriers", schema = List(788), item_name = "AVDDesign.WanCarriersItem"))]
pub struct WanCarriers<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.WanCarriersItemBase", schema = Dict(1777)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1666))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1667))]
    pub description: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1668))]
    pub path_group: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Bool(676))]
    pub trusted: ::data_views::Field<bool>,
}
