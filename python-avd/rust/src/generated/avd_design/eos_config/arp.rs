// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Arp<'a, Mode> {
    pub persistent: ::data_views::Field<Persistent<'a, Mode>>,
    pub aging: ::data_views::Field<Aging<'a, Mode>>,
    pub static_entries: ::data_views::Field<StaticEntries<'a, Mode>>,
}

#[::data_views::data_view(python(name = "EOSConfig.Arp.Persistent", schema = Dict(78)))]
pub struct Persistent<'a, Mode> {
    #[data_view(schema = Bool(17))]
    pub enabled: ::data_views::RequiredValue<bool, Mode>,
    #[data_view(schema = Int(6))]
    pub refresh_delay: ::data_views::Field<i64>,
}

#[::data_views::data_view(python(name = "EOSConfig.Arp.Aging", schema = Dict(79)))]
pub struct Aging<'a, Mode> {
    #[data_view(schema = Int(7))]
    pub timeout_default: ::data_views::Field<i64>,
}

#[::data_views::data_view(list)]
pub struct StaticEntries<'a, Mode> (::data_views::Field<static_entries::Item<'a, Mode>>);

pub mod static_entries {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub ipv4_address: ::data_views::RequiredValue<&'a str, Mode>,
        pub vrf: ::data_views::Field<&'a str>,
        pub mac_address: ::data_views::RequiredValue<&'a str, Mode>,
    }
}
