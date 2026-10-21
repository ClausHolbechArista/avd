// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.NetworkServicesKeys", schema = List(754), item_name = "AVDDesign.NetworkServicesKeysItem"))]
pub struct NetworkServicesKeys<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.NetworkServicesKeysItemBase", schema = Dict(4)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(3))]
    pub name: ::data_views::Field<&'a str>,
}
