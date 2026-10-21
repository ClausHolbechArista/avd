// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Ipv6Neighbor<'a, Mode> {
    pub static_entries: ::data_views::Field<StaticEntries<'a, Mode>>,
    pub persistent: ::data_views::Field<Persistent<'a, Mode>>,
}

#[::data_views::data_view(list)]
pub struct StaticEntries<'a, Mode> (::data_views::Field<static_entries::Item<'a, Mode>>);

pub mod static_entries {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub ipv6_address: ::data_views::RequiredValue<&'a str, Mode>,
        pub vrf: ::data_views::Field<&'a str>,
        pub interface: ::data_views::RequiredValue<&'a str, Mode>,
        pub mac_address: ::data_views::RequiredValue<&'a str, Mode>,
    }
}

#[::data_views::data_view]
pub struct Persistent<'a, Mode> {
    pub enabled: ::data_views::RequiredValue<bool, Mode>,
    pub refresh_delay: ::data_views::Field<i64>,
}
