// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct QueueMonitorLength<'a, Mode> {
    pub enabled: ::data_views::RequiredValue<bool, Mode>,
    pub default_thresholds: ::data_views::Field<DefaultThresholds<'a, Mode>>,
    pub log: ::data_views::Field<i64>,
    pub notifying: ::data_views::Field<bool>,
    pub cpu: ::data_views::Field<Cpu<'a, Mode>>,
    pub tx_latency: ::data_views::Field<bool>,
    pub mirror: ::data_views::Field<Mirror<'a, Mode>>,
}

#[::data_views::data_view]
pub struct DefaultThresholds<'a, Mode> {
    pub high: ::data_views::RequiredValue<i64, Mode>,
    pub low: ::data_views::Field<i64>,
}

#[::data_views::data_view]
pub struct Cpu<'a, Mode> {
    pub thresholds: ::data_views::Field<cpu::Thresholds<'a, Mode>>,
}

pub mod cpu {

    #[::data_views::data_view]
    pub struct Thresholds<'a, Mode> {
        pub high: ::data_views::RequiredValue<i64, Mode>,
        pub low: ::data_views::Field<i64>,
    }
}

#[::data_views::data_view]
pub struct Mirror<'a, Mode> {
    pub enabled: ::data_views::Field<bool>,
    pub destination: ::data_views::Field<mirror::Destination<'a, Mode>>,
}

pub mod mirror {

    #[::data_views::data_view]
    pub struct Destination<'a, Mode> {
        pub cpu: ::data_views::Field<bool>,
        pub ethernet_interfaces: ::data_views::Field<destination::EthernetInterfaces<'a, Mode>>,
        pub tunnel_mode_gre: ::data_views::Field<destination::TunnelModeGre<'a, Mode>>,
    }

    pub mod destination {

        #[::data_views::data_view(list)]
        pub struct EthernetInterfaces<'a, Mode> (::data_views::Field<&'a str>);

        #[::data_views::data_view]
        pub struct TunnelModeGre<'a, Mode> {
            pub source: ::data_views::RequiredValue<&'a str, Mode>,
            pub destination: ::data_views::RequiredValue<&'a str, Mode>,
            pub dscp: ::data_views::Field<i64>,
            pub ttl: ::data_views::Field<i64>,
            pub protocol: ::data_views::Field<&'a str>,
            pub vrf: ::data_views::Field<&'a str>,
        }
    }
}
