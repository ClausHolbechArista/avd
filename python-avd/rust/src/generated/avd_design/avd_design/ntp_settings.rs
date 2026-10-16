// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.NtpSettings.Servers", schema = List(758), item_name = "AVDDesign.NtpSettings.ServersItem"))]
pub struct Servers<'a, Mode> (::validated_data::Field<servers::Item<'a, Mode>>);

pub mod servers {

    #[::validated_data::data_view(python(name = "AVDDesign.NtpSettings.ServersItemBase", schema = Dict(1711)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(625))]
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(schema = Bool(7))]
        pub burst: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(7))]
        pub iburst: ::validated_data::Field<bool>,
        #[data_view(schema = Int(108))]
        pub key: ::validated_data::Field<i64>,
        #[data_view(schema = Int(298))]
        pub maxpoll: ::validated_data::Field<i64>,
        #[data_view(schema = Int(299))]
        pub minpoll: ::validated_data::Field<i64>,
        #[data_view(schema = Int(300))]
        pub version: ::validated_data::Field<i64>,
        #[data_view(schema = Str(1593))]
        pub source_address: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(indexed_list, primary_key(id), python(name = "AVDDesign.NtpSettings.AuthenticationKeys", schema = List(759), item_name = "AVDDesign.NtpSettings.AuthenticationKeysItem"))]
pub struct AuthenticationKeys<'a, Mode> (::validated_data::Field<authentication_keys::Item<'a, Mode>>);

pub mod authentication_keys {

    #[::validated_data::data_view(python(name = "AVDDesign.NtpSettings.AuthenticationKeysItemBase", schema = Dict(1712)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1594))]
        pub key: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1595))]
        pub cleartext_key: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1596))]
        pub key_type: ::validated_data::Field<&'a str>,
        #[data_view(schema = Int(301))]
        pub id: ::validated_data::Field<i64>,
        #[data_view(schema = Str(630))]
        pub hash_algorithm: ::validated_data::RequiredValue<&'a str, Mode>,
    }
}
