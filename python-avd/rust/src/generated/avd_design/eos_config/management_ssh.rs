// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct ManagementSsh<'a, Mode> {
    pub authentication: ::validated_data::Field<Authentication<'a, Mode>>,
    pub ip_access_group_in: ::validated_data::Field<&'a str>,
    pub ipv6_access_group_in: ::validated_data::Field<&'a str>,
    pub idle_timeout: ::validated_data::Field<i64>,
    pub cipher: ::validated_data::Field<Cipher<'a, Mode>>,
    pub key_exchange: ::validated_data::Field<KeyExchange<'a, Mode>>,
    pub mac: ::validated_data::Field<Mac<'a, Mode>>,
    pub fips_restrictions: ::validated_data::Field<bool>,
    pub hostkey: ::validated_data::Field<Hostkey<'a, Mode>>,
    pub enable: ::validated_data::Field<bool>,
    pub connection: ::validated_data::Field<Connection<'a, Mode>>,
    pub vrfs: ::validated_data::Field<Vrfs<'a, Mode>>,
    pub log_level: ::validated_data::Field<&'a str>,
    pub client_alive: ::validated_data::Field<ClientAlive<'a, Mode>>,
}

#[::validated_data::data_view]
pub struct Authentication<'a, Mode> {
    pub empty_passwords: ::validated_data::Field<&'a str>,
    pub protocols: ::validated_data::Field<authentication::Protocols<'a, Mode>>,
}

pub mod authentication {

    #[::validated_data::data_view(list)]
    pub struct Protocols<'a, Mode> (::validated_data::Field<&'a str>);
}

#[::validated_data::data_view(list)]
pub struct Cipher<'a, Mode> (::validated_data::Field<&'a str>);

#[::validated_data::data_view(list)]
pub struct KeyExchange<'a, Mode> (::validated_data::Field<&'a str>);

#[::validated_data::data_view(list)]
pub struct Mac<'a, Mode> (::validated_data::Field<&'a str>);

#[::validated_data::data_view]
pub struct Hostkey<'a, Mode> {
    pub server: ::validated_data::Field<hostkey::Server<'a, Mode>>,
    pub server_cert: ::validated_data::Field<&'a str>,
    pub client_strict_checking: ::validated_data::Field<bool>,
}

pub mod hostkey {

    #[::validated_data::data_view(list)]
    pub struct Server<'a, Mode> (::validated_data::Field<&'a str>);
}

#[::validated_data::data_view]
pub struct Connection<'a, Mode> {
    pub limit: ::validated_data::Field<i64>,
    pub per_host: ::validated_data::Field<i64>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub enable: ::validated_data::Field<bool>,
        pub ip_access_group_in: ::validated_data::Field<&'a str>,
        pub ipv6_access_group_in: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view]
pub struct ClientAlive<'a, Mode> {
    pub count_max: ::validated_data::Field<i64>,
    pub interval: ::validated_data::Field<i64>,
}
