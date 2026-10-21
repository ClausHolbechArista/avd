// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct MacSecurity<'a, Mode> {
    pub license: ::data_views::Field<License<'a, Mode>>,
    pub fips_restrictions: ::data_views::Field<bool>,
    pub profiles: ::data_views::Field<Profiles<'a, Mode>>,
}

#[::data_views::data_view]
pub struct License<'a, Mode> {
    pub license_name: ::data_views::RequiredValue<&'a str, Mode>,
    pub license_key: ::data_views::RequiredValue<&'a str, Mode>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Profiles<'a, Mode> (::data_views::Field<profiles::Item<'a, Mode>>);

pub mod profiles {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub cipher: ::data_views::Field<&'a str>,
        pub connection_keys: ::data_views::Field<item::ConnectionKeys<'a, Mode>>,
        pub mka: ::data_views::Field<item::Mka<'a, Mode>>,
        pub sci: ::data_views::Field<bool>,
        pub l2_protocols: ::data_views::Field<item::L2Protocols<'a, Mode>>,
        pub traffic_unprotected: ::data_views::Field<item::TrafficUnprotected<'a, Mode>>,
        pub replay_protection: ::data_views::Field<item::ReplayProtection<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(indexed_list, primary_key(id))]
        pub struct ConnectionKeys<'a, Mode> (::data_views::Field<connection_keys::Item<'a, Mode>>);

        pub mod connection_keys {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub id: ::data_views::Field<&'a str>,
                pub encrypted_key: ::data_views::RequiredValue<&'a str, Mode>,
                pub fallback: ::data_views::Field<bool>,
            }
        }

        #[::data_views::data_view]
        pub struct Mka<'a, Mode> {
            pub key_server_priority: ::data_views::Field<i64>,
            pub session: ::data_views::Field<mka::Session<'a, Mode>>,
        }

        pub mod mka {

            #[::data_views::data_view]
            pub struct Session<'a, Mode> {
                pub rekey_period: ::data_views::Field<i64>,
            }
        }

        #[::data_views::data_view]
        pub struct L2Protocols<'a, Mode> {
            pub ethernet_flow_control: ::data_views::Field<l2_protocols::EthernetFlowControl<'a, Mode>>,
            pub lldp: ::data_views::Field<l2_protocols::Lldp<'a, Mode>>,
        }

        pub mod l2_protocols {

            #[::data_views::data_view]
            pub struct EthernetFlowControl<'a, Mode> {
                pub mode: ::data_views::RequiredValue<&'a str, Mode>,
            }

            #[::data_views::data_view]
            pub struct Lldp<'a, Mode> {
                pub mode: ::data_views::RequiredValue<&'a str, Mode>,
            }
        }

        #[::data_views::data_view]
        pub struct TrafficUnprotected<'a, Mode> {
            pub action: ::data_views::RequiredValue<&'a str, Mode>,
            pub allow_active_sak: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct ReplayProtection<'a, Mode> {
            pub disabled: ::data_views::Field<bool>,
            pub window: ::data_views::Field<i64>,
        }
    }
}
