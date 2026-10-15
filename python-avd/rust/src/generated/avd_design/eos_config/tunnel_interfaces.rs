// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub description: ::validated_data::Field<&'a str>,
    pub shutdown: ::validated_data::Field<bool>,
    pub mtu: ::validated_data::Field<i64>,
    pub vrf: ::validated_data::Field<&'a str>,
    pub underlay_vrf: ::validated_data::Field<&'a str>,
    pub ip_address: ::validated_data::Field<&'a str>,
    pub ipv6_enable: ::validated_data::Field<bool>,
    pub ipv6_address: ::validated_data::Field<&'a str>,
    pub ipv6_addresses: ::validated_data::Field<item::Ipv6Addresses<'a, Mode>>,
    pub ipv6_address_auto_config: ::validated_data::Field<bool>,
    pub ipv6_nd: ::validated_data::Field<item::Ipv6Nd<'a, Mode>>,
    pub access_group_in: ::validated_data::Field<&'a str>,
    pub access_group_out: ::validated_data::Field<&'a str>,
    pub ipv6_access_group_in: ::validated_data::Field<&'a str>,
    pub ipv6_access_group_out: ::validated_data::Field<&'a str>,
    pub tcp_mss_ceiling: ::validated_data::Field<item::TcpMssCeiling<'a, Mode>>,
    pub tunnel_mode: ::validated_data::Field<&'a str>,
    pub source_interface: ::validated_data::Field<&'a str>,
    pub source: ::validated_data::Field<&'a str>,
    pub destination: ::validated_data::Field<&'a str>,
    pub path_mtu_discovery: ::validated_data::Field<bool>,
    pub ipsec_profile: ::validated_data::Field<&'a str>,
    pub nat_profile: ::validated_data::Field<&'a str>,
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
    pub struct TcpMssCeiling<'a, Mode> {
        pub ipv4: ::validated_data::Field<i64>,
        pub ipv6: ::validated_data::Field<i64>,
        pub direction: ::validated_data::Field<&'a str>,
    }
}
