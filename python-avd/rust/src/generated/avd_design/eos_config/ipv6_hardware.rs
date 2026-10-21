// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Ipv6Hardware<'a, Mode> {
    pub fib: ::data_views::Field<Fib<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Fib<'a, Mode> {
    pub optimize: ::data_views::Field<fib::Optimize<'a, Mode>>,
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
}
