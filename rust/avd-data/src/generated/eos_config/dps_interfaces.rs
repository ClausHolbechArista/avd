// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct DpsInterfaces<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view]
pub struct Item<'a, Mode> {
    pub name: ::data_views::Field<&'a str>,
    pub description: ::data_views::Field<&'a str>,
    pub shutdown: ::data_views::Field<bool>,
    pub mtu: ::data_views::Field<i64>,
    pub ip_address: ::data_views::Field<&'a str>,
    pub ipv6_address_auto_config: ::data_views::Field<bool>,
    pub flow_tracker: ::data_views::Field<item::FlowTracker<'a, Mode>>,
    pub tcp_mss_ceiling: ::data_views::Field<item::TcpMssCeiling<'a, Mode>>,
    pub eos_cli: ::data_views::Field<&'a str>,
}

pub mod item {

    #[::data_views::data_view]
    pub struct FlowTracker<'a, Mode> {
        pub sampled: ::data_views::Field<&'a str>,
        pub hardware: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view]
    pub struct TcpMssCeiling<'a, Mode> {
        pub ipv4: ::data_views::Field<i64>,
        pub ipv6: ::data_views::Field<i64>,
        pub direction: ::data_views::Field<&'a str>,
    }
}
