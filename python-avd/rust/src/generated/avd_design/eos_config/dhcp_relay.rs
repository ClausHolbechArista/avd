// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct DhcpRelay<'a, Mode> {
    pub servers: ::data_views::Field<Servers<'a, Mode>>,
    pub tunnel_requests_disabled: ::data_views::Field<bool>,
    pub mlag_peerlink_requests_disabled: ::data_views::Field<bool>,
    pub client_requests: ::data_views::Field<ClientRequests<'a, Mode>>,
    pub reply_source_address_validation: ::data_views::Field<bool>,
}

#[::data_views::data_view(list)]
pub struct Servers<'a, Mode> (::data_views::Field<&'a str>);

#[::data_views::data_view]
pub struct ClientRequests<'a, Mode> {
    pub flooding_suppression_vlans: ::data_views::Field<client_requests::FloodingSuppressionVlans<'a, Mode>>,
}

pub mod client_requests {

    #[::data_views::data_view(list)]
    pub struct FloodingSuppressionVlans<'a, Mode> (::data_views::Field<&'a str>);
}
