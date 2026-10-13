// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(list)]
pub struct Hosts<'a, Mode> (::validated_data::Field<hosts::Item<'a, Mode>>);

pub mod hosts {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub host: ::validated_data::Field<&'a str>,
        pub vrf: ::validated_data::Field<&'a str>,
        pub key: ::validated_data::Field<&'a str>,
        pub key_type: ::validated_data::Field<&'a str>,
        pub single_connection: ::validated_data::Field<bool>,
        pub timeout: ::validated_data::Field<i64>,
    }
}
