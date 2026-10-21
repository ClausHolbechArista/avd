// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.Ipv4PrefixListCatalog", schema = List(660), item_name = "AVDDesign.Ipv4PrefixListCatalogItem"))]
pub struct Ipv4PrefixListCatalog<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.Ipv4PrefixListCatalogItemBase", schema = Dict(1598)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(430))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = List(659))]
    pub sequence_numbers: ::data_views::RequiredValue<item::SequenceNumbers<'a, Mode>, Mode>,
}

pub mod item {

    #[::data_views::data_view(indexed_list, primary_key(sequence), python(name = "AVDDesign.Ipv4PrefixListCatalogItemBase.SequenceNumbers", schema = List(659), item_name = "AVDDesign.Ipv4PrefixListCatalogItemBase.SequenceNumbersItem"))]
    pub struct SequenceNumbers<'a, Mode> (::data_views::Field<sequence_numbers::Item<'a, Mode>>);

    pub mod sequence_numbers {

        #[::data_views::data_view(python(name = "AVDDesign.Ipv4PrefixListCatalogItemBase.SequenceNumbersItemBase", schema = Dict(1597)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Int(589))]
            pub sequence: ::data_views::RequiredValue<i64, Mode>,
            #[data_view(schema = Str(1421))]
            pub action: ::data_views::RequiredValue<&'a str, Mode>,
        }
    }
}
