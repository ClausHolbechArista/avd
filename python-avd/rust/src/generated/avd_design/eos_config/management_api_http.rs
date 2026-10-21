// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct ManagementApiHttp<'a, Mode> {
    pub enable_http: ::data_views::Field<bool>,
    pub enable_https: ::data_views::Field<bool>,
    pub enable_unix: ::data_views::Field<bool>,
    pub https_ssl_profile: ::data_views::Field<&'a str>,
    pub default_services: ::data_views::Field<bool>,
    pub session_timeout: ::data_views::Field<i64>,
    pub enable_vrfs: ::data_views::Field<EnableVrfs<'a, Mode>>,
    pub protocol_https_certificate: ::data_views::Field<ProtocolHttpsCertificate<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct EnableVrfs<'a, Mode> (::data_views::Field<enable_vrfs::Item<'a, Mode>>);

pub mod enable_vrfs {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub access_group: ::data_views::Field<&'a str>,
        pub ipv6_access_group: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view]
pub struct ProtocolHttpsCertificate<'a, Mode> {
    pub certificate: ::data_views::RequiredValue<&'a str, Mode>,
    pub private_key: ::data_views::RequiredValue<&'a str, Mode>,
}
