// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct VlanInternalOrder<'a, Mode> {
    pub allocation: ::data_views::RequiredValue<&'a str, Mode>,
    pub range: ::data_views::RequiredValue<Range<'a, Mode>, Mode>,
}

#[::data_views::data_view]
pub struct Range<'a, Mode> {
    pub beginning: ::data_views::RequiredValue<i64, Mode>,
    pub ending: ::data_views::RequiredValue<i64, Mode>,
}
