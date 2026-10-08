// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Fib {
        model optimize("optimize", 0) -> fib::Optimize<'a>;
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
}
