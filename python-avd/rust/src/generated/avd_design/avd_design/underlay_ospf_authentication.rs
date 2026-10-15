// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(id), python(name = "AVDDesign.UnderlayOspfAuthentication.MessageDigestKeys", item_name = "AVDDesign.UnderlayOspfAuthentication.MessageDigestKeysItem"))]
pub struct MessageDigestKeys<'a, Mode> (::validated_data::Field<message_digest_keys::Item<'a, Mode>>);

pub mod message_digest_keys {

    #[::validated_data::data_view(python(name = "AVDDesign.UnderlayOspfAuthentication.MessageDigestKeysItemBase"))]
    pub struct Item<'a, Mode> {
        pub id: ::validated_data::Field<i64>,
        pub hash_algorithm: ::validated_data::Field<&'a str>,
        pub cleartext_key: ::validated_data::RequiredValue<&'a str, Mode>,
    }
}
