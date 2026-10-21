// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct VlanInternalOrder<'a, Mode> {
    pub allocation: ::validated_data::RequiredValue<&'a str, Mode>,
    pub range: ::validated_data::RequiredValue<Range<'a, Mode>, Mode>,
}

#[::validated_data::data_view]
pub struct Range<'a, Mode> {
    pub beginning: ::validated_data::RequiredValue<i64, Mode>,
    pub ending: ::validated_data::RequiredValue<i64, Mode>,
}
