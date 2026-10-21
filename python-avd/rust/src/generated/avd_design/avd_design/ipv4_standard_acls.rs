// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.Ipv4StandardAcls", schema = List(661), item_name = "AVDDesign.Ipv4StandardAclsItem"))]
pub struct Ipv4StandardAcls<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.Ipv4StandardAclsItemBase", schema = Dict(1599)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(26))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(7))]
    pub counters_per_entry: ::data_views::Field<bool>,
    #[data_view(schema = List(501))]
    pub entries: ::data_views::Field<item::Entries<'a, Mode>>,
}

pub mod item {

    #[::data_views::data_view(list, python(name = "AVDDesign.Ipv4StandardAclsItemBase.Entries", schema = List(501)))]
    pub struct Entries<'a, Mode> (::data_views::Field<entries::Item<'a, Mode>>);

    pub mod entries {

        #[::data_views::data_view(python(name = "AVDDesign.Ipv4StandardAclsItemBase.EntriesItem", schema = Dict(1255)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Int(5))]
            pub sequence: ::data_views::Field<i64>,
            #[data_view(schema = Str(978))]
            pub action: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(979))]
            pub remark: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(980))]
            pub source: ::data_views::Field<&'a str>,
            #[data_view(schema = Int(469))]
            pub vlan: ::data_views::Field<i64>,
            #[data_view(schema = Str(981))]
            pub vlan_mask: ::data_views::Field<&'a str>,
            #[data_view(schema = Int(470))]
            pub inner_vlan: ::data_views::Field<i64>,
            #[data_view(schema = Str(982))]
            pub inner_vlan_mask: ::data_views::Field<&'a str>,
            #[data_view(schema = Bool(372))]
            pub log: ::data_views::Field<bool>,
            #[data_view(schema = Str(983))]
            pub mirror_session: ::data_views::Field<&'a str>,
        }
    }
}
