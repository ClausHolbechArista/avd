// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "EOSConfig.EventMonitor", schema = Dict(373)))]
pub struct EventMonitor<'a, Mode> {
    #[data_view(schema = Bool(7))]
    pub enabled: ::validated_data::Field<bool>,
}
