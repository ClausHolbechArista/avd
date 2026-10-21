// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.NtpSettings", schema = Dict(1713)))]
pub struct NtpSettings<'a, Mode> {
    #[data_view(schema = Str(1592))]
    pub server_vrf: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(608))]
    pub set_first_ntp_server_as_preferred: ::data_views::Field<bool>,
    #[data_view(schema = List(758))]
    pub servers: ::data_views::Field<Servers<'a, Mode>>,
    #[data_view(schema = Bool(7))]
    pub authenticate: ::data_views::Field<bool>,
    #[data_view(schema = Bool(7))]
    pub authenticate_servers_only: ::data_views::Field<bool>,
    #[data_view(schema = List(759))]
    pub authentication_keys: ::data_views::Field<AuthenticationKeys<'a, Mode>>,
    #[data_view(schema = Str(633))]
    pub trusted_keys: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.NtpSettings.Servers", schema = List(758), item_name = "AVDDesign.NtpSettings.ServersItem"))]
pub struct Servers<'a, Mode> (::data_views::Field<servers::Item<'a, Mode>>);

pub mod servers {

    #[::data_views::data_view(python(name = "AVDDesign.NtpSettings.ServersItemBase", schema = Dict(1711)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(625))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = Bool(7))]
        pub burst: ::data_views::Field<bool>,
        #[data_view(schema = Bool(7))]
        pub iburst: ::data_views::Field<bool>,
        #[data_view(schema = Int(108))]
        pub key: ::data_views::Field<i64>,
        #[data_view(schema = Int(298))]
        pub maxpoll: ::data_views::Field<i64>,
        #[data_view(schema = Int(299))]
        pub minpoll: ::data_views::Field<i64>,
        #[data_view(schema = Int(300))]
        pub version: ::data_views::Field<i64>,
        #[data_view(schema = Str(1593))]
        pub source_address: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view(indexed_list, primary_key(id), python(name = "AVDDesign.NtpSettings.AuthenticationKeys", schema = List(759), item_name = "AVDDesign.NtpSettings.AuthenticationKeysItem"))]
pub struct AuthenticationKeys<'a, Mode> (::data_views::Field<authentication_keys::Item<'a, Mode>>);

pub mod authentication_keys {

    #[::data_views::data_view(python(name = "AVDDesign.NtpSettings.AuthenticationKeysItemBase", schema = Dict(1712)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1594))]
        pub key: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1595))]
        pub cleartext_key: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1596))]
        pub key_type: ::data_views::Field<&'a str>,
        #[data_view(schema = Int(301))]
        pub id: ::data_views::Field<i64>,
        #[data_view(schema = Str(630))]
        pub hash_algorithm: ::data_views::RequiredValue<&'a str, Mode>,
    }
}
