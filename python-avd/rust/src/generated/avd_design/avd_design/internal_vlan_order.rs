// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.InternalVlanOrder", schema = Dict(1593)))]
pub struct InternalVlanOrder<'a, Mode> {
    #[data_view(schema = Str(1063))]
    pub allocation: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Dict(1592))]
    pub range: ::data_views::Field<Range<'a, Mode>>,
}

#[::data_views::data_view(python(name = "AVDDesign.InternalVlanOrder.Range", schema = Dict(1592)))]
pub struct Range<'a, Mode> {
    #[data_view(schema = Int(494))]
    pub beginning: ::data_views::RequiredValue<i64, Mode>,
    #[data_view(schema = Int(495))]
    pub ending: ::data_views::RequiredValue<i64, Mode>,
}
