// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(list)]
pub struct LoadBalanceTridentUdf<'a, Mode> (::validated_data::Field<load_balance_trident_udf::Item<'a, Mode>>);

pub mod load_balance_trident_udf {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub eth_type: ::validated_data::RequiredValue<&'a str, Mode>,
        pub ip_protocol: ::validated_data::Field<&'a str>,
        pub header: ::validated_data::RequiredValue<&'a str, Mode>,
        pub offset: ::validated_data::RequiredValue<i64, Mode>,
        pub mask: ::validated_data::Field<&'a str>,
    }
}
