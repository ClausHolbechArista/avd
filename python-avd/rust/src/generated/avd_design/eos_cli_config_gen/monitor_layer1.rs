// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "EosCliConfigGenMonitorLayer1LoggingTransceiver"))]
pub struct LoggingTransceiver<'a, Mode> {
    pub dom: ::validated_data::Field<bool>,
    pub communication: ::validated_data::Field<bool>,
    pub enabled: ::validated_data::Field<bool>,
}
