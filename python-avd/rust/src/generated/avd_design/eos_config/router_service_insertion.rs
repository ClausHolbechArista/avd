// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Connections<'a, Mode> (::validated_data::Field<connections::Item<'a, Mode>>);

pub mod connections {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub ethernet_interface: ::validated_data::Field<item::EthernetInterface<'a, Mode>>,
        pub tunnel_interface: ::validated_data::Field<item::TunnelInterface<'a, Mode>>,
        pub monitor_connectivity_host: ::validated_data::Field<&'a str>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct EthernetInterface<'a, Mode> {
            pub name: ::validated_data::RequiredValue<&'a str, Mode>,
            pub next_hop: ::validated_data::RequiredValue<&'a str, Mode>,
        }

        #[::validated_data::data_view]
        pub struct TunnelInterface<'a, Mode> {
            pub primary: ::validated_data::Field<&'a str>,
            pub secondary: ::validated_data::Field<&'a str>,
        }
    }
}
