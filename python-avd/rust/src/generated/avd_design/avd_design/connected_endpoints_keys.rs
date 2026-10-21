// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(indexed_list, primary_key(key), python(name = "AVDDesign.ConnectedEndpointsKeys", schema = List(578), item_name = "AVDDesign.ConnectedEndpointsKeysItem"))]
pub struct ConnectedEndpointsKeys<'a, Mode> (::validated_data::Field<Item<'a, Mode>>);

#[::validated_data::data_view(python(name = "AVDDesign.ConnectedEndpointsKeysItemBase", schema = Dict(1435)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(3))]
    pub key: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1167), rename = "type")]
    pub field_type: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1168))]
    pub description: ::validated_data::Field<&'a str>,
}
