// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Mode<'a, Mode> {
    pub exclusive: ::validated_data::Field<mode::Exclusive<'a, Mode>>,
}

pub mod mode {

    #[::validated_data::data_view]
    pub struct Exclusive<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub profile: ::validated_data::Field<&'a str>,
        pub no_errdisable: ::validated_data::Field<exclusive::NoErrdisable<'a, Mode>>,
    }

    pub mod exclusive {

        #[::validated_data::data_view(list)]
        pub struct NoErrdisable<'a, Mode> (::validated_data::Field<&'a str>);
    }
}

#[::validated_data::data_view]
pub struct Mac<'a, Mode> {
    pub timestamp: ::validated_data::Field<mac::Timestamp<'a, Mode>>,
    pub fcs_append: ::validated_data::Field<bool>,
    pub fcs_error: ::validated_data::Field<&'a str>,
}

pub mod mac {

    #[::validated_data::data_view]
    pub struct Timestamp<'a, Mode> {
        pub replace_source_mac: ::validated_data::Field<bool>,
        pub header: ::validated_data::Field<timestamp::Header<'a, Mode>>,
    }

    pub mod timestamp {

        #[::validated_data::data_view]
        pub struct Header<'a, Mode> {
            pub format: ::validated_data::Field<&'a str>,
            pub eth_type: ::validated_data::Field<i64>,
        }
    }
}
