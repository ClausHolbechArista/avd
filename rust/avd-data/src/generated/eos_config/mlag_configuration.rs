// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct MlagConfiguration<'a, Mode> {
    pub domain_id: ::data_views::Field<&'a str>,
    pub heartbeat_interval: ::data_views::Field<i64>,
    pub local_interface: ::data_views::Field<&'a str>,
    pub peer_address: ::data_views::Field<&'a str>,
    pub peer_address_heartbeat: ::data_views::Field<PeerAddressHeartbeat<'a, Mode>>,
    pub dual_primary_detection_delay: ::data_views::Field<i64>,
    pub dual_primary_recovery_delay_mlag: ::data_views::Field<i64>,
    pub dual_primary_recovery_delay_non_mlag: ::data_views::Field<i64>,
    pub peer_link: ::data_views::Field<&'a str>,
    pub reload_delay_mlag: ::data_views::Field<&'a str>,
    pub reload_delay_non_mlag: ::data_views::Field<&'a str>,
}

#[::data_views::data_view]
pub struct PeerAddressHeartbeat<'a, Mode> {
    pub peer_ip: ::data_views::Field<&'a str>,
    pub vrf: ::data_views::Field<&'a str>,
}
