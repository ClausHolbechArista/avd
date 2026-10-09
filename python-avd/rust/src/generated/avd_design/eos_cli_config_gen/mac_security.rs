// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct License<'a, Mode> {
    pub license_name: ::validated_data::RequiredValue<&'a str, Mode>,
    pub license_key: ::validated_data::RequiredValue<&'a str, Mode>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Profiles<'a, Mode> (::validated_data::Field<profiles::Item<'a, Mode>>);

pub mod profiles {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub cipher: ::validated_data::Field<&'a str>,
        pub connection_keys: ::validated_data::Field<item::ConnectionKeys<'a, Mode>>,
        pub mka: ::validated_data::Field<item::Mka<'a, Mode>>,
        pub sci: ::validated_data::Field<bool>,
        pub l2_protocols: ::validated_data::Field<item::L2Protocols<'a, Mode>>,
        pub traffic_unprotected: ::validated_data::Field<item::TrafficUnprotected<'a, Mode>>,
        pub replay_protection: ::validated_data::Field<item::ReplayProtection<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(id))]
        pub struct ConnectionKeys<'a, Mode> (::validated_data::Field<connection_keys::Item<'a, Mode>>);

        pub mod connection_keys {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub id: ::validated_data::Field<&'a str>,
                pub encrypted_key: ::validated_data::RequiredValue<&'a str, Mode>,
                pub fallback: ::validated_data::Field<bool>,
            }
        }

        #[::validated_data::data_view]
        pub struct Mka<'a, Mode> {
            pub key_server_priority: ::validated_data::Field<i64>,
            pub session: ::validated_data::Field<mka::Session<'a, Mode>>,
        }

        pub mod mka {

            #[::validated_data::data_view]
            pub struct Session<'a, Mode> {
                pub rekey_period: ::validated_data::Field<i64>,
            }
        }

        #[::validated_data::data_view]
        pub struct L2Protocols<'a, Mode> {
            pub ethernet_flow_control: ::validated_data::Field<l2_protocols::EthernetFlowControl<'a, Mode>>,
            pub lldp: ::validated_data::Field<l2_protocols::Lldp<'a, Mode>>,
        }

        pub mod l2_protocols {

            #[::validated_data::data_view]
            pub struct EthernetFlowControl<'a, Mode> {
                pub mode: ::validated_data::RequiredValue<&'a str, Mode>,
            }

            #[::validated_data::data_view]
            pub struct Lldp<'a, Mode> {
                pub mode: ::validated_data::RequiredValue<&'a str, Mode>,
            }
        }

        #[::validated_data::data_view]
        pub struct TrafficUnprotected<'a, Mode> {
            pub action: ::validated_data::RequiredValue<&'a str, Mode>,
            pub allow_active_sak: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct ReplayProtection<'a, Mode> {
            pub disabled: ::validated_data::Field<bool>,
            pub window: ::validated_data::Field<i64>,
        }
    }
}
