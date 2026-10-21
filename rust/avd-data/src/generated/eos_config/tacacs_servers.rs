// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct TacacsServers<'a, Mode> {
    pub timeout: ::data_views::Field<i64>,
    pub hosts: ::data_views::Field<Hosts<'a, Mode>>,
    pub policy_unknown_mandatory_attribute_ignore: ::data_views::Field<bool>,
}

#[::data_views::data_view(list)]
pub struct Hosts<'a, Mode> (::data_views::Field<hosts::Item<'a, Mode>>);

pub mod hosts {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub host: ::data_views::Field<&'a str>,
        pub vrf: ::data_views::Field<&'a str>,
        pub key: ::data_views::Field<&'a str>,
        pub key_type: ::data_views::Field<&'a str>,
        pub single_connection: ::data_views::Field<bool>,
        pub timeout: ::data_views::Field<i64>,
    }
}
