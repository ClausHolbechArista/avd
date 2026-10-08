// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct License {
        scalar license_name("license_name", 0) -> &'a str;
        scalar license_key("license_key", 1) -> &'a str;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Profiles {
        model item (0) -> profiles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod profiles {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar cipher("cipher", 1) -> &'a str;
            model connection_keys("connection_keys", 2) -> item::ConnectionKeys<'a>;
            model mka("mka", 3) -> item::Mka<'a>;
            scalar sci("sci", 4) -> bool;
            model l2_protocols("l2_protocols", 5) -> item::L2Protocols<'a>;
            model traffic_unprotected("traffic_unprotected", 6) -> item::TrafficUnprotected<'a>;
            model replay_protection("replay_protection", 7) -> item::ReplayProtection<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct ConnectionKeys {
                model item (0) -> connection_keys::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod connection_keys {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar id("id", 0) -> &'a str;
                    scalar encrypted_key("encrypted_key", 1) -> &'a str;
                    scalar fallback("fallback", 2) -> bool;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Mka {
                scalar key_server_priority("key_server_priority", 0) -> i64;
                model session("session", 1) -> mka::Session<'a>;
            }
        }

        pub mod mka {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Session {
                    scalar rekey_period("rekey_period", 0) -> i64;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct L2Protocols {
                model ethernet_flow_control("ethernet_flow_control", 0) -> l2_protocols::EthernetFlowControl<'a>;
                model lldp("lldp", 1) -> l2_protocols::Lldp<'a>;
            }
        }

        pub mod l2_protocols {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct EthernetFlowControl {
                    scalar mode("mode", 0) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Lldp {
                    scalar mode("mode", 0) -> &'a str;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct TrafficUnprotected {
                scalar action("action", 0) -> &'a str;
                scalar allow_active_sak("allow_active_sak", 1) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct ReplayProtection {
                scalar disabled("disabled", 0) -> bool;
                scalar window("window", 1) -> i64;
            }
        }
    }
}
