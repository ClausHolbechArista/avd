// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Fib<'a, Mode> {
    pub optimize: ::validated_data::Field<fib::Optimize<'a, Mode>>,
    pub load_balance_distribution: ::validated_data::Field<fib::LoadBalanceDistribution<'a, Mode>>,
}

pub mod fib {

    #[::validated_data::data_view]
    pub struct Optimize<'a, Mode> {
        pub prefixes: ::validated_data::Field<optimize::Prefixes<'a, Mode>>,
    }

    pub mod optimize {

        #[::validated_data::data_view]
        pub struct Prefixes<'a, Mode> {
            pub profile: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view]
    pub struct LoadBalanceDistribution<'a, Mode> {
        pub dynamic: ::validated_data::Field<load_balance_distribution::Dynamic<'a, Mode>>,
    }

    pub mod load_balance_distribution {

        #[::validated_data::data_view]
        pub struct Dynamic<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub flow_set_size: ::validated_data::Field<i64>,
        }
    }
}
