// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct LoadBalanceTridentUdf {
        model item (0) -> load_balance_trident_udf::Item<'a>;
    }
}

pub mod load_balance_trident_udf {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar eth_type("eth_type", 0) -> &'a str;
            scalar ip_protocol("ip_protocol", 1) -> &'a str;
            scalar header("header", 2) -> &'a str;
            scalar offset("offset", 3) -> i64;
            scalar mask("mask", 4) -> &'a str;
        }
    }
}
