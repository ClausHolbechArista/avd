// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.MacAcls", schema = List(674), item_name = "AVDDesign.MacAclsItem"))]
pub struct MacAcls<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.MacAclsItemBase", schema = Dict(1619)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1445))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(7))]
    pub counters_per_entry: ::data_views::Field<bool>,
    #[data_view(schema = List(673))]
    pub entries: ::data_views::Field<item::Entries<'a, Mode>>,
}

pub mod item {

    #[::data_views::data_view(list, python(name = "AVDDesign.MacAclsItemBase.Entries", schema = List(673)))]
    pub struct Entries<'a, Mode> (::data_views::Field<entries::Item<'a, Mode>>);

    pub mod entries {

        #[::data_views::data_view(python(name = "AVDDesign.MacAclsItemBase.EntriesItem", schema = Dict(1618)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Int(77))]
            pub sequence: ::data_views::Field<i64>,
            #[data_view(schema = Str(1446))]
            pub action: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1447))]
            pub remark: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1448))]
            pub source: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1449))]
            pub source_wildcard: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1450))]
            pub destination: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1451))]
            pub destination_wildcard: ::data_views::Field<&'a str>,
        }
    }
}
