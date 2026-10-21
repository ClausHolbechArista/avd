// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct Lacp<'a, Mode> {
    pub port_id: ::validated_data::Field<PortId<'a, Mode>>,
    pub rate_limit: ::validated_data::Field<RateLimit<'a, Mode>>,
    pub system_priority: ::validated_data::Field<i64>,
}

#[::validated_data::data_view]
pub struct PortId<'a, Mode> {
    pub range: ::validated_data::Field<port_id::Range<'a, Mode>>,
}

pub mod port_id {

    #[::validated_data::data_view]
    pub struct Range<'a, Mode> {
        pub begin: ::validated_data::RequiredValue<i64, Mode>,
        pub end: ::validated_data::RequiredValue<i64, Mode>,
    }
}

#[::validated_data::data_view]
pub struct RateLimit<'a, Mode> {
    pub default: ::validated_data::Field<bool>,
}
