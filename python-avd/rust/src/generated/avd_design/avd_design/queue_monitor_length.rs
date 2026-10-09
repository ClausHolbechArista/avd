// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct DefaultThresholds<'a, Mode> {
    pub high: ::validated_data::RequiredValue<i64, Mode>,
    pub low: ::validated_data::Field<i64>,
}

#[::validated_data::data_view]
pub struct Cpu<'a, Mode> {
    pub thresholds: ::validated_data::Field<cpu::Thresholds<'a, Mode>>,
}

pub mod cpu {

    #[::validated_data::data_view]
    pub struct Thresholds<'a, Mode> {
        pub high: ::validated_data::RequiredValue<i64, Mode>,
        pub low: ::validated_data::Field<i64>,
    }
}

#[::validated_data::data_view]
pub struct Mirror<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub destination: ::validated_data::Field<mirror::Destination<'a, Mode>>,
}

pub mod mirror {

    #[::validated_data::data_view]
    pub struct Destination<'a, Mode> {
        pub cpu: ::validated_data::Field<bool>,
        pub ethernet_interfaces: ::validated_data::Field<destination::EthernetInterfaces<'a, Mode>>,
        pub tunnel_mode_gre: ::validated_data::Field<destination::TunnelModeGre<'a, Mode>>,
    }

    pub mod destination {

        #[::validated_data::data_view(list)]
        pub struct EthernetInterfaces<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view]
        pub struct TunnelModeGre<'a, Mode> {
            pub source: ::validated_data::RequiredValue<&'a str, Mode>,
            pub destination: ::validated_data::RequiredValue<&'a str, Mode>,
            pub dscp: ::validated_data::Field<i64>,
            pub ttl: ::validated_data::Field<i64>,
            pub protocol: ::validated_data::Field<&'a str>,
            pub vrf: ::validated_data::Field<&'a str>,
        }
    }
}
