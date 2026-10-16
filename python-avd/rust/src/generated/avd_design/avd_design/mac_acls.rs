// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.MacAclsItemBase", schema = Dict(1619)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1445))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(7))]
    pub counters_per_entry: ::validated_data::Field<bool>,
    #[data_view(schema = List(673))]
    pub entries: ::validated_data::Field<item::Entries<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view(list, python(name = "AVDDesign.MacAclsItemBase.Entries", schema = List(673)))]
    pub struct Entries<'a, Mode> (::validated_data::Field<entries::Item<'a, Mode>>);

    pub mod entries {

        #[::validated_data::data_view(python(name = "AVDDesign.MacAclsItemBase.EntriesItem", schema = Dict(1618)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Int(77))]
            pub sequence: ::validated_data::Field<i64>,
            #[data_view(schema = Str(1446))]
            pub action: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1447))]
            pub remark: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1448))]
            pub source: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1449))]
            pub source_wildcard: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1450))]
            pub destination: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1451))]
            pub destination_wildcard: ::validated_data::Field<&'a str>,
        }
    }
}
