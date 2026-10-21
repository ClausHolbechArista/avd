// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.PeerFilters", schema = List(306), item_name = "EOSConfig.PeerFiltersItem"))]
pub struct PeerFilters<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "EOSConfig.PeerFiltersItemBase", schema = Dict(732)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(638))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = List(305))]
    pub sequence_numbers: ::data_views::RequiredValue<item::SequenceNumbers<'a, Mode>, Mode>,
}

pub mod item {

    #[::data_views::data_view(indexed_list, primary_key(sequence), python(name = "EOSConfig.PeerFiltersItemBase.SequenceNumbers", schema = List(305), item_name = "EOSConfig.PeerFiltersItemBase.SequenceNumbersItem"))]
    pub struct SequenceNumbers<'a, Mode> (::data_views::Field<sequence_numbers::Item<'a, Mode>>);

    pub mod sequence_numbers {

        #[::data_views::data_view(python(name = "EOSConfig.PeerFiltersItemBase.SequenceNumbersItemBase", schema = Dict(731)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Int(5))]
            pub sequence: ::data_views::Field<i64>,
            #[data_view(schema = Str(639), rename = "match")]
            pub field_match: ::data_views::RequiredValue<&'a str, Mode>,
        }
    }
}
