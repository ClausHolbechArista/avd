// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Stun<'a, Mode> {
    pub client: ::data_views::Field<Client<'a, Mode>>,
    pub server: ::data_views::Field<Server<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Client<'a, Mode> {
    pub server_profiles: ::data_views::Field<client::ServerProfiles<'a, Mode>>,
}

pub mod client {

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct ServerProfiles<'a, Mode> (::data_views::Field<server_profiles::Item<'a, Mode>>);

    pub mod server_profiles {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub ip_address: ::data_views::Field<&'a str>,
            pub ssl_profile: ::data_views::Field<&'a str>,
            pub port: ::data_views::Field<i64>,
        }
    }
}

#[::data_views::data_view]
pub struct Server<'a, Mode> {
    pub local_interfaces: ::data_views::Field<server::LocalInterfaces<'a, Mode>>,
    pub bindings_timeout: ::data_views::Field<i64>,
    pub ssl_profile: ::data_views::Field<&'a str>,
    pub ssl_connection_lifetime: ::data_views::Field<server::SslConnectionLifetime<'a, Mode>>,
    pub port: ::data_views::Field<i64>,
}

pub mod server {

    #[::data_views::data_view(list)]
    pub struct LocalInterfaces<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view]
    pub struct SslConnectionLifetime<'a, Mode> {
        pub minutes: ::data_views::Field<i64>,
        pub hours: ::data_views::Field<i64>,
    }
}
