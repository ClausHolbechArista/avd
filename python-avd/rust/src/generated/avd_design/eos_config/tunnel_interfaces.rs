// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct TunnelInterfaces<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view]
pub struct Item<'a, Mode> {
    pub name: ::data_views::Field<&'a str>,
    pub description: ::data_views::Field<&'a str>,
    pub shutdown: ::data_views::Field<bool>,
    pub mtu: ::data_views::Field<i64>,
    pub vrf: ::data_views::Field<&'a str>,
    pub underlay_vrf: ::data_views::Field<&'a str>,
    pub ip_address: ::data_views::Field<&'a str>,
    pub ipv6_enable: ::data_views::Field<bool>,
    pub ipv6_address: ::data_views::Field<&'a str>,
    pub ipv6_addresses: ::data_views::Field<item::Ipv6Addresses<'a, Mode>>,
    pub ipv6_address_auto_config: ::data_views::Field<bool>,
    pub ipv6_nd: ::data_views::Field<item::Ipv6Nd<'a, Mode>>,
    pub access_group_in: ::data_views::Field<&'a str>,
    pub access_group_out: ::data_views::Field<&'a str>,
    pub ipv6_access_group_in: ::data_views::Field<&'a str>,
    pub ipv6_access_group_out: ::data_views::Field<&'a str>,
    pub tcp_mss_ceiling: ::data_views::Field<item::TcpMssCeiling<'a, Mode>>,
    pub tunnel_mode: ::data_views::Field<&'a str>,
    pub source_interface: ::data_views::Field<&'a str>,
    pub source: ::data_views::Field<&'a str>,
    pub destination: ::data_views::Field<&'a str>,
    pub path_mtu_discovery: ::data_views::Field<bool>,
    pub ipsec_profile: ::data_views::Field<&'a str>,
    pub nat_profile: ::data_views::Field<&'a str>,
    pub eos_cli: ::data_views::Field<&'a str>,
}

pub mod item {

    #[::data_views::data_view(list)]
    pub struct Ipv6Addresses<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view]
    pub struct Ipv6Nd<'a, Mode> {
        pub cache: ::data_views::Field<ipv6_nd::Cache<'a, Mode>>,
        pub ra: ::data_views::Field<ipv6_nd::Ra<'a, Mode>>,
        pub managed_config_flag: ::data_views::Field<bool>,
        pub prefixes: ::data_views::Field<ipv6_nd::Prefixes<'a, Mode>>,
        pub other_config_flag: ::data_views::Field<bool>,
    }

    pub mod ipv6_nd {

        #[::data_views::data_view]
        pub struct Cache<'a, Mode> {
            pub dynamic_capacity: ::data_views::Field<i64>,
            pub expire: ::data_views::Field<i64>,
            pub refresh_always: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct Ra<'a, Mode> {
            pub disabled: ::data_views::Field<bool>,
            pub rx_accept: ::data_views::Field<ra::RxAccept<'a, Mode>>,
            pub dns_servers: ::data_views::Field<ra::DnsServers<'a, Mode>>,
            pub dns_servers_lifetime: ::data_views::Field<i64>,
        }

        pub mod ra {

            #[::data_views::data_view]
            pub struct RxAccept<'a, Mode> {
                pub default_route: ::data_views::Field<bool>,
                pub route_preference: ::data_views::Field<bool>,
            }

            #[::data_views::data_view(indexed_list, primary_key(address))]
            pub struct DnsServers<'a, Mode> (::data_views::Field<dns_servers::Item<'a, Mode>>);

            pub mod dns_servers {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub address: ::data_views::Field<&'a str>,
                    pub lifetime: ::data_views::Field<i64>,
                }
            }
        }

        #[::data_views::data_view(indexed_list, primary_key(ipv6_prefix))]
        pub struct Prefixes<'a, Mode> (::data_views::Field<prefixes::Item<'a, Mode>>);

        pub mod prefixes {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub ipv6_prefix: ::data_views::Field<&'a str>,
                pub valid_lifetime: ::data_views::Field<&'a str>,
                pub preferred_lifetime: ::data_views::Field<&'a str>,
                pub no_autoconfig_flag: ::data_views::Field<bool>,
            }
        }
    }

    #[::data_views::data_view]
    pub struct TcpMssCeiling<'a, Mode> {
        pub ipv4: ::data_views::Field<i64>,
        pub ipv6: ::data_views::Field<i64>,
        pub direction: ::data_views::Field<&'a str>,
    }
}
