// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(key), python(name = "AVDDesign.ConnectedEndpointsKeys", schema = List(578), item_name = "AVDDesign.ConnectedEndpointsKeysItem"))]
pub struct ConnectedEndpointsKeys<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.ConnectedEndpointsKeysItemBase", schema = Dict(1435)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(3))]
    pub key: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1167), rename = "type")]
    pub field_type: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1168))]
    pub description: ::data_views::Field<&'a str>,
}
