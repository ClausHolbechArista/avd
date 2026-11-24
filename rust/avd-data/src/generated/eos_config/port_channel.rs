// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct PortChannel<'a, Mode> {
    pub load_balance_trident_udf: ::data_views::Field<LoadBalanceTridentUdf<'a, Mode>>,
    pub load_balance_sand_profile: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(list)]
pub struct LoadBalanceTridentUdf<'a, Mode> (::data_views::Field<load_balance_trident_udf::Item<'a, Mode>>);

pub mod load_balance_trident_udf {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub eth_type: ::data_views::RequiredValue<&'a str, Mode>,
        pub ip_protocol: ::data_views::Field<&'a str>,
        pub header: ::data_views::RequiredValue<&'a str, Mode>,
        pub offset: ::data_views::RequiredValue<i64, Mode>,
        pub mask: ::data_views::Field<&'a str>,
    }
}
