// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Ntp<'a, Mode> {
    pub local_interface: ::data_views::Field<LocalInterface<'a, Mode>>,
    pub servers: ::data_views::Field<Servers<'a, Mode>>,
    pub vrf: ::data_views::Field<&'a str>,
    pub authenticate: ::data_views::Field<bool>,
    pub authenticate_servers_only: ::data_views::Field<bool>,
    pub authentication_keys: ::data_views::Field<AuthenticationKeys<'a, Mode>>,
    pub trusted_keys: ::data_views::Field<&'a str>,
    pub serve: ::data_views::Field<Serve<'a, Mode>>,
}

#[::data_views::data_view]
pub struct LocalInterface<'a, Mode> {
    pub name: ::data_views::Field<&'a str>,
    pub vrf: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Servers<'a, Mode> (::data_views::Field<servers::Item<'a, Mode>>);

pub mod servers {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub burst: ::data_views::Field<bool>,
        pub iburst: ::data_views::Field<bool>,
        pub key: ::data_views::Field<i64>,
        pub local_interface: ::data_views::Field<&'a str>,
        pub source_address: ::data_views::Field<&'a str>,
        pub maxpoll: ::data_views::Field<i64>,
        pub minpoll: ::data_views::Field<i64>,
        pub preferred: ::data_views::Field<bool>,
        pub version: ::data_views::Field<i64>,
    }
}

#[::data_views::data_view(indexed_list, primary_key(id))]
pub struct AuthenticationKeys<'a, Mode> (::data_views::Field<authentication_keys::Item<'a, Mode>>);

pub mod authentication_keys {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub id: ::data_views::Field<i64>,
        pub hash_algorithm: ::data_views::RequiredValue<&'a str, Mode>,
        pub key: ::data_views::RequiredValue<&'a str, Mode>,
        pub key_type: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view]
pub struct Serve<'a, Mode> {
    pub serve_all: ::data_views::Field<bool>,
    pub access_group: ::data_views::Field<&'a str>,
    pub ipv6_access_group: ::data_views::Field<&'a str>,
    pub vrfs: ::data_views::Field<serve::Vrfs<'a, Mode>>,
}

pub mod serve {

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

    pub mod vrfs {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub serve_all: ::data_views::Field<bool>,
            pub access_group: ::data_views::Field<&'a str>,
            pub ipv6_access_group: ::data_views::Field<&'a str>,
        }
    }
}
