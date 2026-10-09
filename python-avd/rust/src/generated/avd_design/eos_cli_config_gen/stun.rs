// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Client<'a, Mode> {
    pub server_profiles: ::validated_data::Field<client::ServerProfiles<'a, Mode>>,
}

pub mod client {

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct ServerProfiles<'a, Mode> (::validated_data::Field<server_profiles::Item<'a, Mode>>);

    pub mod server_profiles {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub ip_address: ::validated_data::Field<&'a str>,
            pub ssl_profile: ::validated_data::Field<&'a str>,
            pub port: ::validated_data::Field<i64>,
        }
    }
}

#[::validated_data::data_view]
pub struct Server<'a, Mode> {
    pub local_interfaces: ::validated_data::Field<server::LocalInterfaces<'a, Mode>>,
    pub bindings_timeout: ::validated_data::Field<i64>,
    pub ssl_profile: ::validated_data::Field<&'a str>,
    pub ssl_connection_lifetime: ::validated_data::Field<server::SslConnectionLifetime<'a, Mode>>,
    pub port: ::validated_data::Field<i64>,
}

pub mod server {

    #[::validated_data::data_view(list)]
    pub struct LocalInterfaces<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view]
    pub struct SslConnectionLifetime<'a, Mode> {
        pub minutes: ::validated_data::Field<i64>,
        pub hours: ::validated_data::Field<i64>,
    }
}
