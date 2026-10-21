// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Lacp<'a, Mode> {
    pub port_id: ::data_views::Field<PortId<'a, Mode>>,
    pub rate_limit: ::data_views::Field<RateLimit<'a, Mode>>,
    pub system_priority: ::data_views::Field<i64>,
}

#[::data_views::data_view]
pub struct PortId<'a, Mode> {
    pub range: ::data_views::Field<port_id::Range<'a, Mode>>,
}

pub mod port_id {

    #[::data_views::data_view]
    pub struct Range<'a, Mode> {
        pub begin: ::data_views::RequiredValue<i64, Mode>,
        pub end: ::data_views::RequiredValue<i64, Mode>,
    }
}

#[::data_views::data_view]
pub struct RateLimit<'a, Mode> {
    pub default: ::data_views::Field<bool>,
}
