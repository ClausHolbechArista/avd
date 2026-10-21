// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct ManagementApiHttp<'a, Mode> {
    pub enable_http: ::validated_data::Field<bool>,
    pub enable_https: ::validated_data::Field<bool>,
    pub enable_unix: ::validated_data::Field<bool>,
    pub https_ssl_profile: ::validated_data::Field<&'a str>,
    pub default_services: ::validated_data::Field<bool>,
    pub session_timeout: ::validated_data::Field<i64>,
    pub enable_vrfs: ::validated_data::Field<EnableVrfs<'a, Mode>>,
    pub protocol_https_certificate: ::validated_data::Field<ProtocolHttpsCertificate<'a, Mode>>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct EnableVrfs<'a, Mode> (::validated_data::Field<enable_vrfs::Item<'a, Mode>>);

pub mod enable_vrfs {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub access_group: ::validated_data::Field<&'a str>,
        pub ipv6_access_group: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view]
pub struct ProtocolHttpsCertificate<'a, Mode> {
    pub certificate: ::validated_data::RequiredValue<&'a str, Mode>,
    pub private_key: ::validated_data::RequiredValue<&'a str, Mode>,
}
