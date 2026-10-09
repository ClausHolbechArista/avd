// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Fib<'a, Mode> {
    pub optimize: ::validated_data::Field<fib::Optimize<'a, Mode>>,
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
}
