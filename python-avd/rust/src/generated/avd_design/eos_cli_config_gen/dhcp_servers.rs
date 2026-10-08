// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar disabled("disabled", 0) -> bool;
        scalar vrf("vrf", 1) -> &'a str;
        model lease_time_ipv4("lease_time_ipv4", 2) -> item::LeaseTimeIpv4<'a>;
        model lease_time_ipv6("lease_time_ipv6", 3) -> item::LeaseTimeIpv6<'a>;
        scalar dns_domain_name_ipv4("dns_domain_name_ipv4", 4) -> &'a str;
        scalar dns_domain_name_ipv6("dns_domain_name_ipv6", 5) -> &'a str;
        model dns_servers_ipv4("dns_servers_ipv4", 6) -> item::DnsServersIpv4<'a>;
        model dns_servers_ipv6("dns_servers_ipv6", 7) -> item::DnsServersIpv6<'a>;
        model tftp_server("tftp_server", 8) -> item::TftpServer<'a>;
        model ipv4_vendor_options("ipv4_vendor_options", 9) -> item::Ipv4VendorOptions<'a>;
        model ipv4_subnets("ipv4_subnets", 10) -> item::Ipv4Subnets<'a>;
        model ipv6_subnets("ipv6_subnets", 11) -> item::Ipv6Subnets<'a>;
        scalar eos_cli("eos_cli", 12) -> &'a str;
    }
}

pub mod item {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct LeaseTimeIpv4 {
            scalar days("days", 0) -> i64;
            scalar hours("hours", 1) -> i64;
            scalar minutes("minutes", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct LeaseTimeIpv6 {
            scalar days("days", 0) -> i64;
            scalar hours("hours", 1) -> i64;
            scalar minutes("minutes", 2) -> i64;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct DnsServersIpv4 {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct DnsServersIpv6 {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct TftpServer {
            scalar file_ipv4("file_ipv4", 0) -> &'a str;
            scalar file_ipv6("file_ipv6", 1) -> &'a str;
            scalar option_66_ipv4("option_66_ipv4", 2) -> &'a str;
            model option_150_ipv4("option_150_ipv4", 3) -> tftp_server::Option150Ipv4<'a>;
        }
    }

    pub mod tftp_server {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Option150Ipv4 {
                scalar item (0) -> &'a str;
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipv4VendorOptions {
            model item (0) -> ipv4_vendor_options::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod ipv4_vendor_options {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar vendor_id("vendor_id", 0) -> &'a str;
                model sub_options("sub_options", 1) -> item::SubOptions<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct SubOptions {
                    model item (0) -> sub_options::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod sub_options {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar code("code", 0) -> i64;
                        scalar string("string", 1) -> &'a str;
                        scalar ipv4_address("ipv4_address", 2) -> &'a str;
                        model array_ipv4_address("array_ipv4_address", 3) -> item::ArrayIpv4Address<'a>;
                    }
                }

                pub mod item {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct ArrayIpv4Address {
                            scalar item (0) -> &'a str;
                        }
                    }
                }
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipv4Subnets {
            model item (0) -> ipv4_subnets::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod ipv4_subnets {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar subnet("subnet", 0) -> &'a str;
                scalar default_gateway("default_gateway", 1) -> &'a str;
                model reservations("reservations", 2) -> item::Reservations<'a>;
                model tftp_server("tftp_server", 3) -> item::TftpServer<'a>;
                scalar name("name", 4) -> &'a str;
                model dns_servers("dns_servers", 5) -> item::DnsServers<'a>;
                model ranges("ranges", 6) -> item::Ranges<'a>;
                model lease_time("lease_time", 7) -> item::LeaseTime<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Reservations {
                    model item (0) -> reservations::Item<'a>;
                    primary_key_fields: [1];
                }
            }

            pub mod reservations {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar ipv4_address("ipv4_address", 0) -> &'a str;
                        scalar mac_address("mac_address", 1) -> &'a str;
                        scalar hostname("hostname", 2) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct TftpServer {
                    scalar file("file", 0) -> &'a str;
                    scalar option_66("option_66", 1) -> &'a str;
                    model option_150("option_150", 2) -> tftp_server::Option150<'a>;
                }
            }

            pub mod tftp_server {

                ::validation::define_archive_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Option150 {
                        scalar item (0) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct DnsServers {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Ranges {
                    model item (0) -> ranges::Item<'a>;
                }
            }

            pub mod ranges {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar start("start", 0) -> &'a str;
                        scalar end("end", 1) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct LeaseTime {
                    scalar days("days", 0) -> i64;
                    scalar hours("hours", 1) -> i64;
                    scalar minutes("minutes", 2) -> i64;
                }
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipv6Subnets {
            model item (0) -> ipv6_subnets::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod ipv6_subnets {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar subnet("subnet", 0) -> &'a str;
                model reservations("reservations", 1) -> item::Reservations<'a>;
                model tftp_server("tftp_server", 2) -> item::TftpServer<'a>;
                scalar name("name", 3) -> &'a str;
                model dns_servers("dns_servers", 4) -> item::DnsServers<'a>;
                model ranges("ranges", 5) -> item::Ranges<'a>;
                model lease_time("lease_time", 6) -> item::LeaseTime<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Reservations {
                    model item (0) -> reservations::Item<'a>;
                    primary_key_fields: [1];
                }
            }

            pub mod reservations {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar ipv6_address("ipv6_address", 0) -> &'a str;
                        scalar mac_address("mac_address", 1) -> &'a str;
                        scalar hostname("hostname", 2) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct TftpServer {
                    scalar file("file", 0) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct DnsServers {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Ranges {
                    model item (0) -> ranges::Item<'a>;
                }
            }

            pub mod ranges {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar start("start", 0) -> &'a str;
                        scalar end("end", 1) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct LeaseTime {
                    scalar days("days", 0) -> i64;
                    scalar hours("hours", 1) -> i64;
                    scalar minutes("minutes", 2) -> i64;
                }
            }
        }
    }
}
