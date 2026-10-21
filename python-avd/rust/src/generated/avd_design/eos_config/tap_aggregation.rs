// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct TapAggregation<'a, Mode> {
    pub mode: ::data_views::Field<Mode<'a, Mode>>,
    pub encapsulation_dot1br_strip: ::data_views::Field<bool>,
    pub encapsulation_vn_tag_strip: ::data_views::Field<bool>,
    pub protocol_lldp_trap: ::data_views::Field<bool>,
    pub truncation_size: ::data_views::Field<i64>,
    pub mac: ::data_views::Field<Mac<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Mode<'a, Mode> {
    pub exclusive: ::data_views::Field<mode::Exclusive<'a, Mode>>,
}

pub mod mode {

    #[::data_views::data_view]
    pub struct Exclusive<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub profile: ::data_views::Field<&'a str>,
        pub no_errdisable: ::data_views::Field<exclusive::NoErrdisable<'a, Mode>>,
    }

    pub mod exclusive {

        #[::data_views::data_view(list)]
        pub struct NoErrdisable<'a, Mode> (::data_views::Field<&'a str>);
    }
}

#[::data_views::data_view]
pub struct Mac<'a, Mode> {
    pub timestamp: ::data_views::Field<mac::Timestamp<'a, Mode>>,
    pub fcs_append: ::data_views::Field<bool>,
    pub fcs_error: ::data_views::Field<&'a str>,
}

pub mod mac {

    #[::data_views::data_view]
    pub struct Timestamp<'a, Mode> {
        pub replace_source_mac: ::data_views::Field<bool>,
        pub header: ::data_views::Field<timestamp::Header<'a, Mode>>,
    }

    pub mod timestamp {

        #[::data_views::data_view]
        pub struct Header<'a, Mode> {
            pub format: ::data_views::Field<&'a str>,
            pub eth_type: ::data_views::Field<i64>,
        }
    }
}
