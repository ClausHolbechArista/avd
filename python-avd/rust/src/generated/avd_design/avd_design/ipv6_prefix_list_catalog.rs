// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.Ipv6PrefixListCatalogItemBase", schema = Dict(1603)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(430))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(665))]
    pub sequence_numbers: ::validated_data::RequiredValue<item::SequenceNumbers<'a, Mode>, Mode>,
}

pub mod item {

    #[::validated_data::data_view(indexed_list, primary_key(sequence), python(name = "AVDDesign.Ipv6PrefixListCatalogItemBase.SequenceNumbers", schema = List(665), item_name = "AVDDesign.Ipv6PrefixListCatalogItemBase.SequenceNumbersItem"))]
    pub struct SequenceNumbers<'a, Mode> (::validated_data::Field<sequence_numbers::Item<'a, Mode>>);

    pub mod sequence_numbers {

        #[::validated_data::data_view(python(name = "AVDDesign.Ipv6PrefixListCatalogItemBase.SequenceNumbersItemBase", schema = Dict(1602)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Int(589))]
            pub sequence: ::validated_data::RequiredValue<i64, Mode>,
            #[data_view(schema = Str(1425))]
            pub action: ::validated_data::RequiredValue<&'a str, Mode>,
        }
    }
}
