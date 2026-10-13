// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesignNtpSettingsServersList", item_name = "AVDDesignNtpSettingsServersListIndexedItem"))]
pub struct Servers<'a, Mode> (::validated_data::Field<servers::Item<'a, Mode>>);

pub mod servers {

    #[::validated_data::data_view(python(name = "AVDDesignNtpSettingsServersItems"))]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub burst: ::validated_data::Field<bool>,
        pub iburst: ::validated_data::Field<bool>,
        pub key: ::validated_data::Field<i64>,
        pub maxpoll: ::validated_data::Field<i64>,
        pub minpoll: ::validated_data::Field<i64>,
        pub version: ::validated_data::Field<i64>,
        pub source_address: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(indexed_list, primary_key(id), python(name = "AVDDesignNtpSettingsAuthenticationKeysList", item_name = "AVDDesignNtpSettingsAuthenticationKeysListIndexedItem"))]
pub struct AuthenticationKeys<'a, Mode> (::validated_data::Field<authentication_keys::Item<'a, Mode>>);

pub mod authentication_keys {

    #[::validated_data::data_view(python(name = "AVDDesignNtpSettingsAuthenticationKeysItems"))]
    pub struct Item<'a, Mode> {
        pub key: ::validated_data::Field<&'a str>,
        pub cleartext_key: ::validated_data::Field<&'a str>,
        pub key_type: ::validated_data::Field<&'a str>,
        pub id: ::validated_data::Field<i64>,
        pub hash_algorithm: ::validated_data::RequiredValue<&'a str, Mode>,
    }
}
