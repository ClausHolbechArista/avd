// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.Ipv6PrefixListCatalog", schema = List(666), item_name = "AVDDesign.Ipv6PrefixListCatalogItem"))]
pub struct Ipv6PrefixListCatalog<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.Ipv6PrefixListCatalogItemBase", schema = Dict(1603)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(430))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = List(665))]
    pub sequence_numbers: ::data_views::RequiredValue<item::SequenceNumbers<'a, Mode>, Mode>,
}

pub mod item {

    #[::data_views::data_view(indexed_list, primary_key(sequence), python(name = "AVDDesign.Ipv6PrefixListCatalogItemBase.SequenceNumbers", schema = List(665), item_name = "AVDDesign.Ipv6PrefixListCatalogItemBase.SequenceNumbersItem"))]
    pub struct SequenceNumbers<'a, Mode> (::data_views::Field<sequence_numbers::Item<'a, Mode>>);

    pub mod sequence_numbers {

        #[::data_views::data_view(python(name = "AVDDesign.Ipv6PrefixListCatalogItemBase.SequenceNumbersItemBase", schema = Dict(1602)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Int(589))]
            pub sequence: ::data_views::RequiredValue<i64, Mode>,
            #[data_view(schema = Str(1425))]
            pub action: ::data_views::RequiredValue<&'a str, Mode>,
        }
    }
}
