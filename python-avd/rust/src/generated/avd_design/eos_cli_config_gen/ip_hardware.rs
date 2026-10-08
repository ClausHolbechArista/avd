// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Fib {
        model optimize("optimize", 0) -> fib::Optimize<'a>;
        model load_balance_distribution("load_balance_distribution", 1) -> fib::LoadBalanceDistribution<'a>;
    }
}

pub mod fib {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Optimize {
            model prefixes("prefixes", 0) -> optimize::Prefixes<'a>;
        }
    }

    pub mod optimize {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Prefixes {
                scalar profile("profile", 0) -> &'a str;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct LoadBalanceDistribution {
            model dynamic("dynamic", 0) -> load_balance_distribution::Dynamic<'a>;
        }
    }

    pub mod load_balance_distribution {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Dynamic {
                scalar enabled("enabled", 0) -> bool;
                scalar flow_set_size("flow_set_size", 1) -> i64;
            }
        }
    }
}
