// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Lldp<'a, Mode> {
    pub timer: ::data_views::Field<i64>,
    pub timer_reinitialization: ::data_views::Field<i64>,
    pub holdtime: ::data_views::Field<i64>,
    pub management_address: ::data_views::Field<&'a str>,
    pub vrf: ::data_views::Field<&'a str>,
    pub receive_packet_tagged_drop: ::data_views::Field<bool>,
    pub tlvs: ::data_views::Field<Tlvs<'a, Mode>>,
    pub run: ::data_views::Field<bool>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Tlvs<'a, Mode> (::data_views::Field<tlvs::Item<'a, Mode>>);

pub mod tlvs {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub transmit: ::data_views::Field<bool>,
    }
}
