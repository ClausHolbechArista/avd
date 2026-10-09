// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub disabled: ::validated_data::Field<bool>,
    pub vrf: ::validated_data::Field<&'a str>,
    pub lease_time_ipv4: ::validated_data::Field<item::LeaseTimeIpv4<'a, Mode>>,
    pub lease_time_ipv6: ::validated_data::Field<item::LeaseTimeIpv6<'a, Mode>>,
    pub dns_domain_name_ipv4: ::validated_data::Field<&'a str>,
    pub dns_domain_name_ipv6: ::validated_data::Field<&'a str>,
    pub dns_servers_ipv4: ::validated_data::Field<item::DnsServersIpv4<'a, Mode>>,
    pub dns_servers_ipv6: ::validated_data::Field<item::DnsServersIpv6<'a, Mode>>,
    pub tftp_server: ::validated_data::Field<item::TftpServer<'a, Mode>>,
    pub ipv4_vendor_options: ::validated_data::Field<item::Ipv4VendorOptions<'a, Mode>>,
    pub ipv4_subnets: ::validated_data::Field<item::Ipv4Subnets<'a, Mode>>,
    pub ipv6_subnets: ::validated_data::Field<item::Ipv6Subnets<'a, Mode>>,
    pub eos_cli: ::validated_data::Field<&'a str>,
}

pub mod item {

    #[::validated_data::data_view]
    pub struct LeaseTimeIpv4<'a, Mode> {
        pub days: ::validated_data::RequiredValue<i64, Mode>,
        pub hours: ::validated_data::RequiredValue<i64, Mode>,
        pub minutes: ::validated_data::RequiredValue<i64, Mode>,
    }

    #[::validated_data::data_view]
    pub struct LeaseTimeIpv6<'a, Mode> {
        pub days: ::validated_data::RequiredValue<i64, Mode>,
        pub hours: ::validated_data::RequiredValue<i64, Mode>,
        pub minutes: ::validated_data::RequiredValue<i64, Mode>,
    }

    #[::validated_data::data_view(list)]
    pub struct DnsServersIpv4<'a, Mode> (::validated_data::RequiredValue<&'a str, Mode>);

    #[::validated_data::data_view(list)]
    pub struct DnsServersIpv6<'a, Mode> (::validated_data::RequiredValue<&'a str, Mode>);

    #[::validated_data::data_view]
    pub struct TftpServer<'a, Mode> {
        pub file_ipv4: ::validated_data::Field<&'a str>,
        pub file_ipv6: ::validated_data::Field<&'a str>,
        pub option_66_ipv4: ::validated_data::Field<&'a str>,
        pub option_150_ipv4: ::validated_data::Field<tftp_server::Option150Ipv4<'a, Mode>>,
    }

    pub mod tftp_server {

        #[::validated_data::data_view(list)]
        pub struct Option150Ipv4<'a, Mode> (::validated_data::Field<&'a str>);
    }

    #[::validated_data::data_view(indexed_list, primary_key(vendor_id))]
    pub struct Ipv4VendorOptions<'a, Mode> (::validated_data::Field<ipv4_vendor_options::Item<'a, Mode>>);

    pub mod ipv4_vendor_options {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub vendor_id: ::validated_data::Field<&'a str>,
            pub sub_options: ::validated_data::Field<item::SubOptions<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(indexed_list, primary_key(code))]
            pub struct SubOptions<'a, Mode> (::validated_data::Field<sub_options::Item<'a, Mode>>);

            pub mod sub_options {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub code: ::validated_data::RequiredValue<i64, Mode>,
                    pub string: ::validated_data::Field<&'a str>,
                    pub ipv4_address: ::validated_data::Field<&'a str>,
                    pub array_ipv4_address: ::validated_data::Field<item::ArrayIpv4Address<'a, Mode>>,
                }

                pub mod item {

                    #[::validated_data::data_view(list)]
                    pub struct ArrayIpv4Address<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(subnet))]
    pub struct Ipv4Subnets<'a, Mode> (::validated_data::Field<ipv4_subnets::Item<'a, Mode>>);

    pub mod ipv4_subnets {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub subnet: ::validated_data::Field<&'a str>,
            pub default_gateway: ::validated_data::Field<&'a str>,
            pub reservations: ::validated_data::Field<item::Reservations<'a, Mode>>,
            pub tftp_server: ::validated_data::Field<item::TftpServer<'a, Mode>>,
            pub name: ::validated_data::Field<&'a str>,
            pub dns_servers: ::validated_data::Field<item::DnsServers<'a, Mode>>,
            pub ranges: ::validated_data::Field<item::Ranges<'a, Mode>>,
            pub lease_time: ::validated_data::Field<item::LeaseTime<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(indexed_list, primary_key(mac_address))]
            pub struct Reservations<'a, Mode> (::validated_data::Field<reservations::Item<'a, Mode>>);

            pub mod reservations {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub ipv4_address: ::validated_data::Field<&'a str>,
                    pub mac_address: ::validated_data::Field<&'a str>,
                    pub hostname: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view]
            pub struct TftpServer<'a, Mode> {
                pub file: ::validated_data::Field<&'a str>,
                pub option_66: ::validated_data::Field<&'a str>,
                pub option_150: ::validated_data::Field<tftp_server::Option150<'a, Mode>>,
            }

            pub mod tftp_server {

                #[::validated_data::data_view(list)]
                pub struct Option150<'a, Mode> (::validated_data::Field<&'a str>);
            }

            #[::validated_data::data_view(list)]
            pub struct DnsServers<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list)]
            pub struct Ranges<'a, Mode> (::validated_data::Field<ranges::Item<'a, Mode>>);

            pub mod ranges {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub start: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub end: ::validated_data::RequiredValue<&'a str, Mode>,
                }
            }

            #[::validated_data::data_view]
            pub struct LeaseTime<'a, Mode> {
                pub days: ::validated_data::RequiredValue<i64, Mode>,
                pub hours: ::validated_data::RequiredValue<i64, Mode>,
                pub minutes: ::validated_data::RequiredValue<i64, Mode>,
            }
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(subnet))]
    pub struct Ipv6Subnets<'a, Mode> (::validated_data::Field<ipv6_subnets::Item<'a, Mode>>);

    pub mod ipv6_subnets {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub subnet: ::validated_data::Field<&'a str>,
            pub reservations: ::validated_data::Field<item::Reservations<'a, Mode>>,
            pub tftp_server: ::validated_data::Field<item::TftpServer<'a, Mode>>,
            pub name: ::validated_data::Field<&'a str>,
            pub dns_servers: ::validated_data::Field<item::DnsServers<'a, Mode>>,
            pub ranges: ::validated_data::Field<item::Ranges<'a, Mode>>,
            pub lease_time: ::validated_data::Field<item::LeaseTime<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(indexed_list, primary_key(mac_address))]
            pub struct Reservations<'a, Mode> (::validated_data::Field<reservations::Item<'a, Mode>>);

            pub mod reservations {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub ipv6_address: ::validated_data::Field<&'a str>,
                    pub mac_address: ::validated_data::Field<&'a str>,
                    pub hostname: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view]
            pub struct TftpServer<'a, Mode> {
                pub file: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view(list)]
            pub struct DnsServers<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list)]
            pub struct Ranges<'a, Mode> (::validated_data::Field<ranges::Item<'a, Mode>>);

            pub mod ranges {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub start: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub end: ::validated_data::RequiredValue<&'a str, Mode>,
                }
            }

            #[::validated_data::data_view]
            pub struct LeaseTime<'a, Mode> {
                pub days: ::validated_data::RequiredValue<i64, Mode>,
                pub hours: ::validated_data::RequiredValue<i64, Mode>,
                pub minutes: ::validated_data::RequiredValue<i64, Mode>,
            }
        }
    }
}
