// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct ManagementSsh<'a, Mode> {
    pub authentication: ::data_views::Field<Authentication<'a, Mode>>,
    pub ip_access_group_in: ::data_views::Field<&'a str>,
    pub ipv6_access_group_in: ::data_views::Field<&'a str>,
    pub idle_timeout: ::data_views::Field<i64>,
    pub cipher: ::data_views::Field<Cipher<'a, Mode>>,
    pub key_exchange: ::data_views::Field<KeyExchange<'a, Mode>>,
    pub mac: ::data_views::Field<Mac<'a, Mode>>,
    pub fips_restrictions: ::data_views::Field<bool>,
    pub hostkey: ::data_views::Field<Hostkey<'a, Mode>>,
    pub enable: ::data_views::Field<bool>,
    pub connection: ::data_views::Field<Connection<'a, Mode>>,
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
    pub log_level: ::data_views::Field<&'a str>,
    pub client_alive: ::data_views::Field<ClientAlive<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Authentication<'a, Mode> {
    pub empty_passwords: ::data_views::Field<&'a str>,
    pub protocols: ::data_views::Field<authentication::Protocols<'a, Mode>>,
}

pub mod authentication {

    #[::data_views::data_view(list)]
    pub struct Protocols<'a, Mode> (::data_views::Field<&'a str>);
}

#[::data_views::data_view(list)]
pub struct Cipher<'a, Mode> (::data_views::Field<&'a str>);

#[::data_views::data_view(list)]
pub struct KeyExchange<'a, Mode> (::data_views::Field<&'a str>);

#[::data_views::data_view(list)]
pub struct Mac<'a, Mode> (::data_views::Field<&'a str>);

#[::data_views::data_view]
pub struct Hostkey<'a, Mode> {
    pub server: ::data_views::Field<hostkey::Server<'a, Mode>>,
    pub server_cert: ::data_views::Field<&'a str>,
    pub client_strict_checking: ::data_views::Field<bool>,
}

pub mod hostkey {

    #[::data_views::data_view(list)]
    pub struct Server<'a, Mode> (::data_views::Field<&'a str>);
}

#[::data_views::data_view]
pub struct Connection<'a, Mode> {
    pub limit: ::data_views::Field<i64>,
    pub per_host: ::data_views::Field<i64>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub enable: ::data_views::Field<bool>,
        pub ip_access_group_in: ::data_views::Field<&'a str>,
        pub ipv6_access_group_in: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view]
pub struct ClientAlive<'a, Mode> {
    pub count_max: ::data_views::Field<i64>,
    pub interval: ::data_views::Field<i64>,
}
