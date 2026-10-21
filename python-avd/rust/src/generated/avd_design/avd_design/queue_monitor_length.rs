// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "AVDDesign.QueueMonitorLength", schema = Dict(1736)))]
pub struct QueueMonitorLength<'a, Mode> {
    #[data_view(schema = Bool(1))]
    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    #[data_view(schema = Bool(640))]
    pub notifying: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(847))]
    pub default_thresholds: ::validated_data::Field<DefaultThresholds<'a, Mode>>,
    #[data_view(schema = Int(345))]
    pub log: ::validated_data::Field<i64>,
    #[data_view(schema = Dict(849))]
    pub cpu: ::validated_data::Field<Cpu<'a, Mode>>,
    #[data_view(schema = Bool(303))]
    pub tx_latency: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(852))]
    pub mirror: ::validated_data::Field<Mirror<'a, Mode>>,
}

#[::validated_data::data_view(python(name = "AVDDesign.QueueMonitorLength.DefaultThresholds", schema = Dict(847)))]
pub struct DefaultThresholds<'a, Mode> {
    #[data_view(schema = Int(343))]
    pub high: ::validated_data::RequiredValue<i64, Mode>,
    #[data_view(schema = Int(344))]
    pub low: ::validated_data::Field<i64>,
}

#[::validated_data::data_view(python(name = "AVDDesign.QueueMonitorLength.Cpu", schema = Dict(849)))]
pub struct Cpu<'a, Mode> {
    #[data_view(schema = Dict(848))]
    pub thresholds: ::validated_data::Field<cpu::Thresholds<'a, Mode>>,
}

pub mod cpu {

    #[::validated_data::data_view(python(name = "AVDDesign.QueueMonitorLength.Cpu.Thresholds", schema = Dict(848)))]
    pub struct Thresholds<'a, Mode> {
        #[data_view(schema = Int(152))]
        pub high: ::validated_data::RequiredValue<i64, Mode>,
        #[data_view(schema = Int(77))]
        pub low: ::validated_data::Field<i64>,
    }
}

#[::validated_data::data_view(python(name = "AVDDesign.QueueMonitorLength.Mirror", schema = Dict(852)))]
pub struct Mirror<'a, Mode> {
    #[data_view(schema = Bool(7))]
    pub enabled: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(851))]
    pub destination: ::validated_data::Field<mirror::Destination<'a, Mode>>,
}

pub mod mirror {

    #[::validated_data::data_view(python(name = "AVDDesign.QueueMonitorLength.Mirror.Destination", schema = Dict(851)))]
    pub struct Destination<'a, Mode> {
        #[data_view(schema = Bool(304))]
        pub cpu: ::validated_data::Field<bool>,
        #[data_view(schema = List(339))]
        pub ethernet_interfaces: ::validated_data::Field<destination::EthernetInterfaces<'a, Mode>>,
        #[data_view(schema = Dict(850))]
        pub tunnel_mode_gre: ::validated_data::Field<destination::TunnelModeGre<'a, Mode>>,
    }

    pub mod destination {

        #[::validated_data::data_view(list, python(name = "AVDDesign.QueueMonitorLength.Mirror.Destination.EthernetInterfaces", schema = List(339)))]
        pub struct EthernetInterfaces<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(python(name = "AVDDesign.QueueMonitorLength.Mirror.Destination.TunnelModeGre", schema = Dict(850)))]
        pub struct TunnelModeGre<'a, Mode> {
            #[data_view(schema = Str(718))]
            pub source: ::validated_data::RequiredValue<&'a str, Mode>,
            #[data_view(schema = Str(719))]
            pub destination: ::validated_data::RequiredValue<&'a str, Mode>,
            #[data_view(schema = Int(346))]
            pub dscp: ::validated_data::Field<i64>,
            #[data_view(schema = Int(347))]
            pub ttl: ::validated_data::Field<i64>,
            #[data_view(schema = Str(720))]
            pub protocol: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(721))]
            pub vrf: ::validated_data::Field<&'a str>,
        }
    }
}
