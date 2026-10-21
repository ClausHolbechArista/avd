// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct Lldp<'a, Mode> {
    pub timer: ::validated_data::Field<i64>,
    pub timer_reinitialization: ::validated_data::Field<i64>,
    pub holdtime: ::validated_data::Field<i64>,
    pub management_address: ::validated_data::Field<&'a str>,
    pub vrf: ::validated_data::Field<&'a str>,
    pub receive_packet_tagged_drop: ::validated_data::Field<bool>,
    pub tlvs: ::validated_data::Field<Tlvs<'a, Mode>>,
    pub run: ::validated_data::Field<bool>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Tlvs<'a, Mode> (::validated_data::Field<tlvs::Item<'a, Mode>>);

pub mod tlvs {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub transmit: ::validated_data::Field<bool>,
    }
}
