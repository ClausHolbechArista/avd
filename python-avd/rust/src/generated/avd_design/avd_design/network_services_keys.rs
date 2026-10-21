// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.NetworkServicesKeys", schema = List(754), item_name = "AVDDesign.NetworkServicesKeysItem"))]
pub struct NetworkServicesKeys<'a, Mode> (::validated_data::Field<Item<'a, Mode>>);

#[::validated_data::data_view(python(name = "AVDDesign.NetworkServicesKeysItemBase", schema = Dict(4)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(3))]
    pub name: ::validated_data::Field<&'a str>,
}
