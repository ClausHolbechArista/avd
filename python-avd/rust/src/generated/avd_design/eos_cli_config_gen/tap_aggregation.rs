// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Mode {
        model exclusive("exclusive", 0) -> mode::Exclusive<'a>;
    }
}

pub mod mode {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Exclusive {
            scalar enabled("enabled", 0) -> bool;
            scalar profile("profile", 1) -> &'a str;
            model no_errdisable("no_errdisable", 2) -> exclusive::NoErrdisable<'a>;
        }
    }

    pub mod exclusive {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct NoErrdisable {
                scalar item (0) -> &'a str;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Mac {
        model timestamp("timestamp", 0) -> mac::Timestamp<'a>;
        scalar fcs_append("fcs_append", 1) -> bool;
        scalar fcs_error("fcs_error", 2) -> &'a str;
    }
}

pub mod mac {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Timestamp {
            scalar replace_source_mac("replace_source_mac", 0) -> bool;
            model header("header", 1) -> timestamp::Header<'a>;
        }
    }

    pub mod timestamp {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Header {
                scalar format("format", 0) -> &'a str;
                scalar eth_type("eth_type", 1) -> i64;
            }
        }
    }
}
