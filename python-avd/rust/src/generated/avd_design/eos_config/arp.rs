// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct Arp<'a, Mode> {
    pub persistent: ::validated_data::Field<Persistent<'a, Mode>>,
    pub aging: ::validated_data::Field<Aging<'a, Mode>>,
    pub static_entries: ::validated_data::Field<StaticEntries<'a, Mode>>,
}

#[::validated_data::data_view(python(name = "EOSConfig.Arp.Persistent", schema = Dict(78)))]
pub struct Persistent<'a, Mode> {
    #[data_view(schema = Bool(17))]
    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    #[data_view(schema = Int(6))]
    pub refresh_delay: ::validated_data::Field<i64>,
}

#[::validated_data::data_view(python(name = "EOSConfig.Arp.Aging", schema = Dict(79)))]
pub struct Aging<'a, Mode> {
    #[data_view(schema = Int(7))]
    pub timeout_default: ::validated_data::Field<i64>,
}

#[::validated_data::data_view(list)]
pub struct StaticEntries<'a, Mode> (::validated_data::Field<static_entries::Item<'a, Mode>>);

pub mod static_entries {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub ipv4_address: ::validated_data::RequiredValue<&'a str, Mode>,
        pub vrf: ::validated_data::Field<&'a str>,
        pub mac_address: ::validated_data::RequiredValue<&'a str, Mode>,
    }
}
