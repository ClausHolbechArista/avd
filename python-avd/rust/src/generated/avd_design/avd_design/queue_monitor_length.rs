// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.QueueMonitorLength", schema = Dict(1736)))]
pub struct QueueMonitorLength<'a, Mode> {
    #[data_view(schema = Bool(1))]
    pub enabled: ::data_views::RequiredValue<bool, Mode>,
    #[data_view(schema = Bool(640))]
    pub notifying: ::data_views::Field<bool>,
    #[data_view(schema = Dict(847))]
    pub default_thresholds: ::data_views::Field<DefaultThresholds<'a, Mode>>,
    #[data_view(schema = Int(345))]
    pub log: ::data_views::Field<i64>,
    #[data_view(schema = Dict(849))]
    pub cpu: ::data_views::Field<Cpu<'a, Mode>>,
    #[data_view(schema = Bool(303))]
    pub tx_latency: ::data_views::Field<bool>,
    #[data_view(schema = Dict(852))]
    pub mirror: ::data_views::Field<Mirror<'a, Mode>>,
}

#[::data_views::data_view(python(name = "AVDDesign.QueueMonitorLength.DefaultThresholds", schema = Dict(847)))]
pub struct DefaultThresholds<'a, Mode> {
    #[data_view(schema = Int(343))]
    pub high: ::data_views::RequiredValue<i64, Mode>,
    #[data_view(schema = Int(344))]
    pub low: ::data_views::Field<i64>,
}

#[::data_views::data_view(python(name = "AVDDesign.QueueMonitorLength.Cpu", schema = Dict(849)))]
pub struct Cpu<'a, Mode> {
    #[data_view(schema = Dict(848))]
    pub thresholds: ::data_views::Field<cpu::Thresholds<'a, Mode>>,
}

pub mod cpu {

    #[::data_views::data_view(python(name = "AVDDesign.QueueMonitorLength.Cpu.Thresholds", schema = Dict(848)))]
    pub struct Thresholds<'a, Mode> {
        #[data_view(schema = Int(152))]
        pub high: ::data_views::RequiredValue<i64, Mode>,
        #[data_view(schema = Int(77))]
        pub low: ::data_views::Field<i64>,
    }
}

#[::data_views::data_view(python(name = "AVDDesign.QueueMonitorLength.Mirror", schema = Dict(852)))]
pub struct Mirror<'a, Mode> {
    #[data_view(schema = Bool(7))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = Dict(851))]
    pub destination: ::data_views::Field<mirror::Destination<'a, Mode>>,
}

pub mod mirror {

    #[::data_views::data_view(python(name = "AVDDesign.QueueMonitorLength.Mirror.Destination", schema = Dict(851)))]
    pub struct Destination<'a, Mode> {
        #[data_view(schema = Bool(304))]
        pub cpu: ::data_views::Field<bool>,
        #[data_view(schema = List(339))]
        pub ethernet_interfaces: ::data_views::Field<destination::EthernetInterfaces<'a, Mode>>,
        #[data_view(schema = Dict(850))]
        pub tunnel_mode_gre: ::data_views::Field<destination::TunnelModeGre<'a, Mode>>,
    }

    pub mod destination {

        #[::data_views::data_view(list, python(name = "AVDDesign.QueueMonitorLength.Mirror.Destination.EthernetInterfaces", schema = List(339)))]
        pub struct EthernetInterfaces<'a, Mode> (::data_views::Field<&'a str>);

        #[::data_views::data_view(python(name = "AVDDesign.QueueMonitorLength.Mirror.Destination.TunnelModeGre", schema = Dict(850)))]
        pub struct TunnelModeGre<'a, Mode> {
            #[data_view(schema = Str(718))]
            pub source: ::data_views::RequiredValue<&'a str, Mode>,
            #[data_view(schema = Str(719))]
            pub destination: ::data_views::RequiredValue<&'a str, Mode>,
            #[data_view(schema = Int(346))]
            pub dscp: ::data_views::Field<i64>,
            #[data_view(schema = Int(347))]
            pub ttl: ::data_views::Field<i64>,
            #[data_view(schema = Str(720))]
            pub protocol: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(721))]
            pub vrf: ::data_views::Field<&'a str>,
        }
    }
}
