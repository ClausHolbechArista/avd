// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct InterfaceDefaults {
        scalar ethernet_shutdown("ethernet_shutdown", 0) -> bool;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Arp {
        model persistent("persistent", 0) -> super::super::eos_cli_config_gen::arp::Persistent<'a>;
        model aging("aging", 1) -> super::super::eos_cli_config_gen::arp::Aging<'a>;
    }
}

pub mod arp {
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DhcpRelay {
        scalar information_option("information_option", 0) -> bool;
        scalar tunnel_requests_disabled("tunnel_requests_disabled", 1) -> bool;
        scalar mlag_peerlink_requests_disabled("mlag_peerlink_requests_disabled", 2) -> bool;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SuspendedVlans {
        model item (0) -> suspended_vlans::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod suspended_vlans {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar id("id", 0) -> i64;
            scalar name("name", 1) -> &'a str;
        }
    }
}
