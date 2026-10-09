// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub id: ::validated_data::RequiredValue<i64, Mode>,
    pub rt_override: ::validated_data::Field<&'a str>,
    pub rd_override: ::validated_data::Field<&'a str>,
    pub evpn_l2_multi_domain: ::validated_data::Field<bool>,
    pub bgp: ::validated_data::Field<item::Bgp<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view]
    pub struct Bgp<'a, Mode> {
        pub raw_eos_cli: ::validated_data::Field<&'a str>,
    }
}
