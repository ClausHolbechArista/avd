// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "EOSConfig.QueueMonitorStreaming", schema = Dict(854)))]
pub struct QueueMonitorStreaming<'a, Mode> {
    #[data_view(schema = Bool(7))]
    pub enable: ::validated_data::Field<bool>,
    #[data_view(schema = Str(722))]
    pub ip_access_group: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(723))]
    pub ipv6_access_group: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(184))]
    pub max_connections: ::validated_data::Field<i64>,
    #[data_view(schema = Str(49))]
    pub vrf: ::validated_data::Field<&'a str>,
}
