// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "EOSConfig.EventMonitor", schema = Dict(373)))]
pub struct EventMonitor<'a, Mode> {
    #[data_view(schema = Bool(7))]
    pub enabled: ::data_views::Field<bool>,
}
