// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(id), python(name = "AVDDesign.UnderlayOspfAuthentication.MessageDigestKeys", schema = List(786), item_name = "AVDDesign.UnderlayOspfAuthentication.MessageDigestKeysItem"))]
pub struct MessageDigestKeys<'a, Mode> (::validated_data::Field<message_digest_keys::Item<'a, Mode>>);

pub mod message_digest_keys {

    #[::validated_data::data_view(python(name = "AVDDesign.UnderlayOspfAuthentication.MessageDigestKeysItemBase", schema = Dict(1771)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Int(77))]
        pub id: ::validated_data::Field<i64>,
        #[data_view(schema = Str(1502))]
        pub hash_algorithm: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1660))]
        pub cleartext_key: ::validated_data::RequiredValue<&'a str, Mode>,
    }
}
