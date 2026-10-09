// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(list)]
pub struct StaticEntries<'a, Mode> (::validated_data::Field<static_entries::Item<'a, Mode>>);

pub mod static_entries {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub ipv6_address: ::validated_data::RequiredValue<&'a str, Mode>,
        pub vrf: ::validated_data::Field<&'a str>,
        pub interface: ::validated_data::RequiredValue<&'a str, Mode>,
        pub mac_address: ::validated_data::RequiredValue<&'a str, Mode>,
    }
}

#[::validated_data::data_view]
pub struct Persistent<'a, Mode> {
    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    pub refresh_delay: ::validated_data::Field<i64>,
}
