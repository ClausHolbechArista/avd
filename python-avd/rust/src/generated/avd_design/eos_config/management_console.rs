// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "EOSConfig.ManagementConsole", schema = Dict(575)))]
pub struct ManagementConsole<'a, Mode> {
    #[data_view(schema = Int(235))]
    pub idle_timeout: ::validated_data::Field<i64>,
}
