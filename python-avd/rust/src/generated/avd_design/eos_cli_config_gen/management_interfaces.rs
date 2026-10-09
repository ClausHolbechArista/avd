// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub description: ::validated_data::Field<&'a str>,
    pub shutdown: ::validated_data::Field<bool>,
    pub speed: ::validated_data::Field<&'a str>,
    pub mtu: ::validated_data::Field<i64>,
    pub vrf: ::validated_data::Field<&'a str>,
    pub ip_address: ::validated_data::Field<&'a str>,
    pub ipv6_enable: ::validated_data::Field<bool>,
    pub ipv6_address: ::validated_data::Field<&'a str>,
    pub ipv6_addresses: ::validated_data::Field<item::Ipv6Addresses<'a, Mode>>,
    pub ipv6_address_auto_config: ::validated_data::Field<bool>,
    pub ipv6_nd: ::validated_data::Field<item::Ipv6Nd<'a, Mode>>,
    #[data_view(rename = "type")]
    pub field_type: ::validated_data::Field<&'a str>,
    pub gateway: ::validated_data::Field<&'a str>,
    pub ipv6_gateway: ::validated_data::Field<&'a str>,
    pub mac_address: ::validated_data::Field<&'a str>,
    pub dhcp_client_accept_default_route: ::validated_data::Field<bool>,
    pub lldp: ::validated_data::Field<item::Lldp<'a, Mode>>,
    pub redundancy: ::validated_data::Field<item::Redundancy<'a, Mode>>,
    pub eos_cli: ::validated_data::Field<&'a str>,
}

pub mod item {

    #[::validated_data::data_view(list)]
    pub struct Ipv6Addresses<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view]
    pub struct Ipv6Nd<'a, Mode> {
        pub cache: ::validated_data::Field<ipv6_nd::Cache<'a, Mode>>,
        pub ra: ::validated_data::Field<ipv6_nd::Ra<'a, Mode>>,
        pub managed_config_flag: ::validated_data::Field<bool>,
        pub prefixes: ::validated_data::Field<ipv6_nd::Prefixes<'a, Mode>>,
        pub other_config_flag: ::validated_data::Field<bool>,
    }

    pub mod ipv6_nd {

        #[::validated_data::data_view]
        pub struct Cache<'a, Mode> {
            pub dynamic_capacity: ::validated_data::Field<i64>,
            pub expire: ::validated_data::Field<i64>,
            pub refresh_always: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct Ra<'a, Mode> {
            pub disabled: ::validated_data::Field<bool>,
            pub rx_accept: ::validated_data::Field<ra::RxAccept<'a, Mode>>,
            pub dns_servers: ::validated_data::Field<ra::DnsServers<'a, Mode>>,
            pub dns_servers_lifetime: ::validated_data::Field<i64>,
        }

        pub mod ra {

            #[::validated_data::data_view]
            pub struct RxAccept<'a, Mode> {
                pub default_route: ::validated_data::Field<bool>,
                pub route_preference: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view(indexed_list, primary_key(address))]
            pub struct DnsServers<'a, Mode> (::validated_data::Field<dns_servers::Item<'a, Mode>>);

            pub mod dns_servers {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub address: ::validated_data::Field<&'a str>,
                    pub lifetime: ::validated_data::Field<i64>,
                }
            }
        }

        #[::validated_data::data_view(indexed_list, primary_key(ipv6_prefix))]
        pub struct Prefixes<'a, Mode> (::validated_data::Field<prefixes::Item<'a, Mode>>);

        pub mod prefixes {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub ipv6_prefix: ::validated_data::Field<&'a str>,
                pub valid_lifetime: ::validated_data::Field<&'a str>,
                pub preferred_lifetime: ::validated_data::Field<&'a str>,
                pub no_autoconfig_flag: ::validated_data::Field<bool>,
            }
        }
    }

    #[::validated_data::data_view]
    pub struct Lldp<'a, Mode> {
        pub transmit: ::validated_data::Field<bool>,
        pub receive: ::validated_data::Field<bool>,
        pub ztp_vlan: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Redundancy<'a, Mode> {
        pub fallback_delay: ::validated_data::Field<&'a str>,
        pub monitor: ::validated_data::Field<redundancy::Monitor<'a, Mode>>,
        pub supervisor_1: ::validated_data::Field<redundancy::Supervisor1<'a, Mode>>,
        pub supervisor_2: ::validated_data::Field<redundancy::Supervisor2<'a, Mode>>,
    }

    pub mod redundancy {

        #[::validated_data::data_view]
        pub struct Monitor<'a, Mode> {
            pub link_state: ::validated_data::Field<bool>,
            pub neighbor: ::validated_data::Field<monitor::Neighbor<'a, Mode>>,
        }

        pub mod monitor {

            #[::validated_data::data_view]
            pub struct Neighbor<'a, Mode> {
                pub ipv6_address: ::validated_data::RequiredValue<&'a str, Mode>,
                pub interval: ::validated_data::Field<i64>,
                pub multiplier: ::validated_data::Field<i64>,
            }
        }

        #[::validated_data::data_view]
        pub struct Supervisor1<'a, Mode> {
            pub primary_management_interface: ::validated_data::RequiredValue<&'a str, Mode>,
            pub backup_management_interfaces: ::validated_data::RequiredValue<supervisor_1::BackupManagementInterfaces<'a, Mode>, Mode>,
        }

        pub mod supervisor_1 {

            #[::validated_data::data_view(list)]
            pub struct BackupManagementInterfaces<'a, Mode> (::validated_data::Field<&'a str>);
        }

        #[::validated_data::data_view]
        pub struct Supervisor2<'a, Mode> {
            pub primary_management_interface: ::validated_data::RequiredValue<&'a str, Mode>,
            pub backup_management_interfaces: ::validated_data::RequiredValue<supervisor_2::BackupManagementInterfaces<'a, Mode>, Mode>,
        }

        pub mod supervisor_2 {

            #[::validated_data::data_view(list)]
            pub struct BackupManagementInterfaces<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }
}
