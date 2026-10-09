// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(list)]
pub struct Servers<'a, Mode> (::validated_data::Field<&'a str>);

#[::validated_data::data_view]
pub struct ClientRequests<'a, Mode> {
    pub flooding_suppression_vlans: ::validated_data::Field<client_requests::FloodingSuppressionVlans<'a, Mode>>,
}

pub mod client_requests {

    #[::validated_data::data_view(list)]
    pub struct FloodingSuppressionVlans<'a, Mode> (::validated_data::Field<&'a str>);
}
