// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.UnderlayOspfAuthentication", schema = Dict(1772)))]
pub struct UnderlayOspfAuthentication<'a, Mode> {
    #[data_view(schema = Bool(669))]
    pub enabled: ::data_views::RequiredValue<bool, Mode>,
    #[data_view(schema = List(786))]
    pub message_digest_keys: ::data_views::RequiredValue<MessageDigestKeys<'a, Mode>, Mode>,
}

#[::data_views::data_view(indexed_list, primary_key(id), python(name = "AVDDesign.UnderlayOspfAuthentication.MessageDigestKeys", schema = List(786), item_name = "AVDDesign.UnderlayOspfAuthentication.MessageDigestKeysItem"))]
pub struct MessageDigestKeys<'a, Mode> (::data_views::Field<message_digest_keys::Item<'a, Mode>>);

pub mod message_digest_keys {

    #[::data_views::data_view(python(name = "AVDDesign.UnderlayOspfAuthentication.MessageDigestKeysItemBase", schema = Dict(1771)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Int(77))]
        pub id: ::data_views::Field<i64>,
        #[data_view(schema = Str(1502))]
        pub hash_algorithm: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1660))]
        pub cleartext_key: ::data_views::RequiredValue<&'a str, Mode>,
    }
}
