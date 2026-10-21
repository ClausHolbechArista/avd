// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct MonitorServerRadius<'a, Mode> {
    pub service_dot1x: ::data_views::Field<bool>,
    pub probe: ::data_views::Field<Probe<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Probe<'a, Mode> {
    pub interval: ::data_views::Field<i64>,
    pub threshold_failure: ::data_views::Field<i64>,
    pub method: ::data_views::Field<&'a str>,
    pub access_request: ::data_views::Field<probe::AccessRequest<'a, Mode>>,
}

pub mod probe {

    #[::data_views::data_view]
    pub struct AccessRequest<'a, Mode> {
        pub username: ::data_views::RequiredValue<&'a str, Mode>,
        pub password: ::data_views::RequiredValue<&'a str, Mode>,
        pub password_type: ::data_views::Field<&'a str>,
    }
}
