// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub description: ::validated_data::Field<&'a str>,
    pub shutdown: ::validated_data::Field<bool>,
    pub mtu: ::validated_data::Field<i64>,
    pub ip_address: ::validated_data::Field<&'a str>,
    pub ipv6_address_auto_config: ::validated_data::Field<bool>,
    pub flow_tracker: ::validated_data::Field<item::FlowTracker<'a, Mode>>,
    pub tcp_mss_ceiling: ::validated_data::Field<item::TcpMssCeiling<'a, Mode>>,
    pub eos_cli: ::validated_data::Field<&'a str>,
}

pub mod item {

    #[::validated_data::data_view]
    pub struct FlowTracker<'a, Mode> {
        pub sampled: ::validated_data::Field<&'a str>,
        pub hardware: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct TcpMssCeiling<'a, Mode> {
        pub ipv4: ::validated_data::Field<i64>,
        pub ipv6: ::validated_data::Field<i64>,
        pub direction: ::validated_data::Field<&'a str>,
    }
}
