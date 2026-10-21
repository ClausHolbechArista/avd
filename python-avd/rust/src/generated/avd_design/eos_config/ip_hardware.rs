// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct IpHardware<'a, Mode> {
    pub fib: ::data_views::Field<Fib<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Fib<'a, Mode> {
    pub optimize: ::data_views::Field<fib::Optimize<'a, Mode>>,
    pub load_balance_distribution: ::data_views::Field<fib::LoadBalanceDistribution<'a, Mode>>,
}

pub mod fib {

    #[::data_views::data_view]
    pub struct Optimize<'a, Mode> {
        pub prefixes: ::data_views::Field<optimize::Prefixes<'a, Mode>>,
    }

    pub mod optimize {

        #[::data_views::data_view]
        pub struct Prefixes<'a, Mode> {
            pub profile: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view]
    pub struct LoadBalanceDistribution<'a, Mode> {
        pub dynamic: ::data_views::Field<load_balance_distribution::Dynamic<'a, Mode>>,
    }

    pub mod load_balance_distribution {

        #[::data_views::data_view]
        pub struct Dynamic<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub flow_set_size: ::data_views::Field<i64>,
        }
    }
}
