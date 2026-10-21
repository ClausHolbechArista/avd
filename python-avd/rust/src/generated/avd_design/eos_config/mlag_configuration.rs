// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct MlagConfiguration<'a, Mode> {
    pub domain_id: ::validated_data::Field<&'a str>,
    pub heartbeat_interval: ::validated_data::Field<i64>,
    pub local_interface: ::validated_data::Field<&'a str>,
    pub peer_address: ::validated_data::Field<&'a str>,
    pub peer_address_heartbeat: ::validated_data::Field<PeerAddressHeartbeat<'a, Mode>>,
    pub dual_primary_detection_delay: ::validated_data::Field<i64>,
    pub dual_primary_recovery_delay_mlag: ::validated_data::Field<i64>,
    pub dual_primary_recovery_delay_non_mlag: ::validated_data::Field<i64>,
    pub peer_link: ::validated_data::Field<&'a str>,
    pub reload_delay_mlag: ::validated_data::Field<&'a str>,
    pub reload_delay_non_mlag: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct PeerAddressHeartbeat<'a, Mode> {
    pub peer_ip: ::validated_data::Field<&'a str>,
    pub vrf: ::validated_data::Field<&'a str>,
}
