// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.Ipv4StandardAclsItemBase", schema = Dict(1599)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(26))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(7))]
    pub counters_per_entry: ::validated_data::Field<bool>,
    #[data_view(schema = List(501))]
    pub entries: ::validated_data::Field<item::Entries<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view(list, python(name = "AVDDesign.Ipv4StandardAclsItemBase.Entries", schema = List(501)))]
    pub struct Entries<'a, Mode> (::validated_data::Field<entries::Item<'a, Mode>>);

    pub mod entries {

        #[::validated_data::data_view(python(name = "AVDDesign.Ipv4StandardAclsItemBase.EntriesItem", schema = Dict(1255)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Int(5))]
            pub sequence: ::validated_data::Field<i64>,
            #[data_view(schema = Str(978))]
            pub action: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(979))]
            pub remark: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(980))]
            pub source: ::validated_data::Field<&'a str>,
            #[data_view(schema = Int(469))]
            pub vlan: ::validated_data::Field<i64>,
            #[data_view(schema = Str(981))]
            pub vlan_mask: ::validated_data::Field<&'a str>,
            #[data_view(schema = Int(470))]
            pub inner_vlan: ::validated_data::Field<i64>,
            #[data_view(schema = Str(982))]
            pub inner_vlan_mask: ::validated_data::Field<&'a str>,
            #[data_view(schema = Bool(372))]
            pub log: ::validated_data::Field<bool>,
            #[data_view(schema = Str(983))]
            pub mirror_session: ::validated_data::Field<&'a str>,
        }
    }
}
