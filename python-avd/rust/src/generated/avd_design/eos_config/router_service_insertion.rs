// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct RouterServiceInsertion<'a, Mode> {
    pub enabled: ::data_views::Field<bool>,
    pub connections: ::data_views::Field<Connections<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Connections<'a, Mode> (::data_views::Field<connections::Item<'a, Mode>>);

pub mod connections {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub ethernet_interface: ::data_views::Field<item::EthernetInterface<'a, Mode>>,
        pub tunnel_interface: ::data_views::Field<item::TunnelInterface<'a, Mode>>,
        pub monitor_connectivity_host: ::data_views::Field<&'a str>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct EthernetInterface<'a, Mode> {
            pub name: ::data_views::RequiredValue<&'a str, Mode>,
            pub next_hop: ::data_views::RequiredValue<&'a str, Mode>,
        }

        #[::data_views::data_view]
        pub struct TunnelInterface<'a, Mode> {
            pub primary: ::data_views::Field<&'a str>,
            pub secondary: ::data_views::Field<&'a str>,
        }
    }
}
