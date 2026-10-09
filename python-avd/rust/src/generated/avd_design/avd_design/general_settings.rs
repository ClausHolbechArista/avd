// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct InterfaceDefaults<'a, Mode> {
    pub ethernet_shutdown: ::validated_data::Field<bool>,
}

#[::validated_data::data_view]
pub struct Arp<'a, Mode> {
    pub persistent: ::validated_data::Field<super::super::eos_cli_config_gen::arp::Persistent<'a, Mode>>,
    pub aging: ::validated_data::Field<super::super::eos_cli_config_gen::arp::Aging<'a, Mode>>,
}

pub mod arp {
}

#[::validated_data::data_view]
pub struct DhcpRelay<'a, Mode> {
    pub information_option: ::validated_data::Field<bool>,
    pub tunnel_requests_disabled: ::validated_data::Field<bool>,
    pub mlag_peerlink_requests_disabled: ::validated_data::Field<bool>,
}

#[::validated_data::data_view(indexed_list, primary_key(id))]
pub struct SuspendedVlans<'a, Mode> (::validated_data::Field<suspended_vlans::Item<'a, Mode>>);

pub mod suspended_vlans {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub id: ::validated_data::Field<i64>,
        pub name: ::validated_data::Field<&'a str>,
    }
}
