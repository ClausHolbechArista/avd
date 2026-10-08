// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Connections {
        model item (0) -> connections::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod connections {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model ethernet_interface("ethernet_interface", 1) -> item::EthernetInterface<'a>;
            model tunnel_interface("tunnel_interface", 2) -> item::TunnelInterface<'a>;
            scalar monitor_connectivity_host("monitor_connectivity_host", 3) -> &'a str;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct EthernetInterface {
                scalar name("name", 0) -> &'a str;
                scalar next_hop("next_hop", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct TunnelInterface {
                scalar primary("primary", 0) -> &'a str;
                scalar secondary("secondary", 1) -> &'a str;
            }
        }
    }
}
