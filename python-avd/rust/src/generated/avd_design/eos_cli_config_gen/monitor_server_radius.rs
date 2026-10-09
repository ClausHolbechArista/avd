// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Probe<'a, Mode> {
    pub interval: ::validated_data::Field<i64>,
    pub threshold_failure: ::validated_data::Field<i64>,
    pub method: ::validated_data::Field<&'a str>,
    pub access_request: ::validated_data::Field<probe::AccessRequest<'a, Mode>>,
}

pub mod probe {

    #[::validated_data::data_view]
    pub struct AccessRequest<'a, Mode> {
        pub username: ::validated_data::RequiredValue<&'a str, Mode>,
        pub password: ::validated_data::RequiredValue<&'a str, Mode>,
        pub password_type: ::validated_data::Field<&'a str>,
    }
}
