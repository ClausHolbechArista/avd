// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "EOSConfig.MonitorLayer1", schema = Dict(652)))]
pub struct MonitorLayer1<'a, Mode> {
    #[data_view(schema = Bool(267))]
    pub enabled: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(268))]
    pub logging_mac_fault: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(651))]
    pub logging_transceiver: ::validated_data::Field<LoggingTransceiver<'a, Mode>>,
}

#[::validated_data::data_view(python(name = "EOSConfig.MonitorLayer1.LoggingTransceiver", schema = Dict(651)))]
pub struct LoggingTransceiver<'a, Mode> {
    #[data_view(schema = Bool(269))]
    pub dom: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(270))]
    pub communication: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(271))]
    pub enabled: ::validated_data::Field<bool>,
}
