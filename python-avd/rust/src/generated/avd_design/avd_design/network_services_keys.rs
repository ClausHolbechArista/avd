// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.NetworkServicesKeysItemBase", schema = Dict(4)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(3))]
    pub name: ::validated_data::Field<&'a str>,
}
