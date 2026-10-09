// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


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
