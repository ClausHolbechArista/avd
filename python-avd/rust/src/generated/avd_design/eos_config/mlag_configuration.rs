// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct PeerAddressHeartbeat<'a, Mode> {
    pub peer_ip: ::validated_data::Field<&'a str>,
    pub vrf: ::validated_data::Field<&'a str>,
}
