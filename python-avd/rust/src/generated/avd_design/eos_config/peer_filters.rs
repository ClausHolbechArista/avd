// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.PeerFilters", schema = List(306), item_name = "EOSConfig.PeerFiltersItem"))]
pub struct PeerFilters<'a, Mode> (::validated_data::Field<Item<'a, Mode>>);

#[::validated_data::data_view(python(name = "EOSConfig.PeerFiltersItemBase", schema = Dict(732)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(638))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(305))]
    pub sequence_numbers: ::validated_data::RequiredValue<item::SequenceNumbers<'a, Mode>, Mode>,
}

pub mod item {

    #[::validated_data::data_view(indexed_list, primary_key(sequence), python(name = "EOSConfig.PeerFiltersItemBase.SequenceNumbers", schema = List(305), item_name = "EOSConfig.PeerFiltersItemBase.SequenceNumbersItem"))]
    pub struct SequenceNumbers<'a, Mode> (::validated_data::Field<sequence_numbers::Item<'a, Mode>>);

    pub mod sequence_numbers {

        #[::validated_data::data_view(python(name = "EOSConfig.PeerFiltersItemBase.SequenceNumbersItemBase", schema = Dict(731)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Int(5))]
            pub sequence: ::validated_data::Field<i64>,
            #[data_view(schema = Str(639), rename = "match")]
            pub field_match: ::validated_data::RequiredValue<&'a str, Mode>,
        }
    }
}
