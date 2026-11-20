// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct MonitorTelemetryInflux<'a, Mode> {
    pub vrf: ::data_views::Field<&'a str>,
    pub destinations: ::data_views::Field<Destinations<'a, Mode>>,
    pub source_group_standard_disabled: ::data_views::Field<bool>,
    pub source_sockets: ::data_views::Field<SourceSockets<'a, Mode>>,
    pub tags: ::data_views::Field<Tags<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Destinations<'a, Mode> (::data_views::Field<destinations::Item<'a, Mode>>);

pub mod destinations {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub database: ::data_views::Field<&'a str>,
        pub data_retention_policy: ::data_views::Field<&'a str>,
        pub url: ::data_views::Field<&'a str>,
        pub username: ::data_views::Field<&'a str>,
        pub password: ::data_views::Field<&'a str>,
        pub password_type: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct SourceSockets<'a, Mode> (::data_views::Field<source_sockets::Item<'a, Mode>>);

pub mod source_sockets {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub connection_limit: ::data_views::Field<i64>,
        pub url: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Tags<'a, Mode> (::data_views::Field<tags::Item<'a, Mode>>);

pub mod tags {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub value: ::data_views::RequiredValue<&'a str, Mode>,
    }
}
