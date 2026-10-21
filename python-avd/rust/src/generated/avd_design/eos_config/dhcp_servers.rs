// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(vrf))]
pub struct DhcpServers<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view]
pub struct Item<'a, Mode> {
    pub disabled: ::data_views::Field<bool>,
    pub vrf: ::data_views::Field<&'a str>,
    pub lease_time_ipv4: ::data_views::Field<item::LeaseTimeIpv4<'a, Mode>>,
    pub lease_time_ipv6: ::data_views::Field<item::LeaseTimeIpv6<'a, Mode>>,
    pub dns_domain_name_ipv4: ::data_views::Field<&'a str>,
    pub dns_domain_name_ipv6: ::data_views::Field<&'a str>,
    pub dns_servers_ipv4: ::data_views::Field<item::DnsServersIpv4<'a, Mode>>,
    pub dns_servers_ipv6: ::data_views::Field<item::DnsServersIpv6<'a, Mode>>,
    pub tftp_server: ::data_views::Field<item::TftpServer<'a, Mode>>,
    pub ipv4_vendor_options: ::data_views::Field<item::Ipv4VendorOptions<'a, Mode>>,
    pub ipv4_subnets: ::data_views::Field<item::Ipv4Subnets<'a, Mode>>,
    pub ipv6_subnets: ::data_views::Field<item::Ipv6Subnets<'a, Mode>>,
    pub eos_cli: ::data_views::Field<&'a str>,
}

pub mod item {

    #[::data_views::data_view]
    pub struct LeaseTimeIpv4<'a, Mode> {
        pub days: ::data_views::RequiredValue<i64, Mode>,
        pub hours: ::data_views::RequiredValue<i64, Mode>,
        pub minutes: ::data_views::RequiredValue<i64, Mode>,
    }

    #[::data_views::data_view]
    pub struct LeaseTimeIpv6<'a, Mode> {
        pub days: ::data_views::RequiredValue<i64, Mode>,
        pub hours: ::data_views::RequiredValue<i64, Mode>,
        pub minutes: ::data_views::RequiredValue<i64, Mode>,
    }

    #[::data_views::data_view(list)]
    pub struct DnsServersIpv4<'a, Mode> (::data_views::RequiredValue<&'a str, Mode>);

    #[::data_views::data_view(list)]
    pub struct DnsServersIpv6<'a, Mode> (::data_views::RequiredValue<&'a str, Mode>);

    #[::data_views::data_view]
    pub struct TftpServer<'a, Mode> {
        pub file_ipv4: ::data_views::Field<&'a str>,
        pub file_ipv6: ::data_views::Field<&'a str>,
        pub option_66_ipv4: ::data_views::Field<&'a str>,
        pub option_150_ipv4: ::data_views::Field<tftp_server::Option150Ipv4<'a, Mode>>,
    }

    pub mod tftp_server {

        #[::data_views::data_view(list)]
        pub struct Option150Ipv4<'a, Mode> (::data_views::Field<&'a str>);
    }

    #[::data_views::data_view(indexed_list, primary_key(vendor_id))]
    pub struct Ipv4VendorOptions<'a, Mode> (::data_views::Field<ipv4_vendor_options::Item<'a, Mode>>);

    pub mod ipv4_vendor_options {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub vendor_id: ::data_views::Field<&'a str>,
            pub sub_options: ::data_views::Field<item::SubOptions<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view(indexed_list, primary_key(code))]
            pub struct SubOptions<'a, Mode> (::data_views::Field<sub_options::Item<'a, Mode>>);

            pub mod sub_options {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub code: ::data_views::RequiredValue<i64, Mode>,
                    pub string: ::data_views::Field<&'a str>,
                    pub ipv4_address: ::data_views::Field<&'a str>,
                    pub array_ipv4_address: ::data_views::Field<item::ArrayIpv4Address<'a, Mode>>,
                }

                pub mod item {

                    #[::data_views::data_view(list)]
                    pub struct ArrayIpv4Address<'a, Mode> (::data_views::Field<&'a str>);
                }
            }
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(subnet))]
    pub struct Ipv4Subnets<'a, Mode> (::data_views::Field<ipv4_subnets::Item<'a, Mode>>);

    pub mod ipv4_subnets {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub subnet: ::data_views::Field<&'a str>,
            pub default_gateway: ::data_views::Field<&'a str>,
            pub reservations: ::data_views::Field<item::Reservations<'a, Mode>>,
            pub tftp_server: ::data_views::Field<item::TftpServer<'a, Mode>>,
            pub name: ::data_views::Field<&'a str>,
            pub dns_servers: ::data_views::Field<item::DnsServers<'a, Mode>>,
            pub ranges: ::data_views::Field<item::Ranges<'a, Mode>>,
            pub lease_time: ::data_views::Field<item::LeaseTime<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view(indexed_list, primary_key(mac_address))]
            pub struct Reservations<'a, Mode> (::data_views::Field<reservations::Item<'a, Mode>>);

            pub mod reservations {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub ipv4_address: ::data_views::Field<&'a str>,
                    pub mac_address: ::data_views::Field<&'a str>,
                    pub hostname: ::data_views::Field<&'a str>,
                }
            }

            #[::data_views::data_view]
            pub struct TftpServer<'a, Mode> {
                pub file: ::data_views::Field<&'a str>,
                pub option_66: ::data_views::Field<&'a str>,
                pub option_150: ::data_views::Field<tftp_server::Option150<'a, Mode>>,
            }

            pub mod tftp_server {

                #[::data_views::data_view(list)]
                pub struct Option150<'a, Mode> (::data_views::Field<&'a str>);
            }

            #[::data_views::data_view(list)]
            pub struct DnsServers<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list)]
            pub struct Ranges<'a, Mode> (::data_views::Field<ranges::Item<'a, Mode>>);

            pub mod ranges {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub start: ::data_views::RequiredValue<&'a str, Mode>,
                    pub end: ::data_views::RequiredValue<&'a str, Mode>,
                }
            }

            #[::data_views::data_view]
            pub struct LeaseTime<'a, Mode> {
                pub days: ::data_views::RequiredValue<i64, Mode>,
                pub hours: ::data_views::RequiredValue<i64, Mode>,
                pub minutes: ::data_views::RequiredValue<i64, Mode>,
            }
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(subnet))]
    pub struct Ipv6Subnets<'a, Mode> (::data_views::Field<ipv6_subnets::Item<'a, Mode>>);

    pub mod ipv6_subnets {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub subnet: ::data_views::Field<&'a str>,
            pub reservations: ::data_views::Field<item::Reservations<'a, Mode>>,
            pub tftp_server: ::data_views::Field<item::TftpServer<'a, Mode>>,
            pub name: ::data_views::Field<&'a str>,
            pub dns_servers: ::data_views::Field<item::DnsServers<'a, Mode>>,
            pub ranges: ::data_views::Field<item::Ranges<'a, Mode>>,
            pub lease_time: ::data_views::Field<item::LeaseTime<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view(indexed_list, primary_key(mac_address))]
            pub struct Reservations<'a, Mode> (::data_views::Field<reservations::Item<'a, Mode>>);

            pub mod reservations {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub ipv6_address: ::data_views::Field<&'a str>,
                    pub mac_address: ::data_views::Field<&'a str>,
                    pub hostname: ::data_views::Field<&'a str>,
                }
            }

            #[::data_views::data_view]
            pub struct TftpServer<'a, Mode> {
                pub file: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view(list)]
            pub struct DnsServers<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list)]
            pub struct Ranges<'a, Mode> (::data_views::Field<ranges::Item<'a, Mode>>);

            pub mod ranges {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub start: ::data_views::RequiredValue<&'a str, Mode>,
                    pub end: ::data_views::RequiredValue<&'a str, Mode>,
                }
            }

            #[::data_views::data_view]
            pub struct LeaseTime<'a, Mode> {
                pub days: ::data_views::RequiredValue<i64, Mode>,
                pub hours: ::data_views::RequiredValue<i64, Mode>,
                pub minutes: ::data_views::RequiredValue<i64, Mode>,
            }
        }
    }
}
