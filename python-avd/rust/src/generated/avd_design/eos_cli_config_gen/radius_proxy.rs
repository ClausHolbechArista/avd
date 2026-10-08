// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ClientGroups {
        model item (0) -> client_groups::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod client_groups {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model server_groups("server_groups", 1) -> item::ServerGroups<'a>;
            model vrfs("vrfs", 2) -> item::Vrfs<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct ServerGroups {
                scalar item (0) -> &'a str;
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Vrfs {
                model item (0) -> vrfs::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod vrfs {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    model ipv4_clients("ipv4_clients", 1) -> item::Ipv4Clients<'a>;
                    model ipv6_clients("ipv6_clients", 2) -> item::Ipv6Clients<'a>;
                    model host_clients("host_clients", 3) -> item::HostClients<'a>;
                }
            }

            pub mod item {

                ::validation::define_archive_indexed_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Ipv4Clients {
                        model item (0) -> ipv4_clients::Item<'a>;
                        primary_key_fields: [0];
                    }
                }

                pub mod ipv4_clients {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            scalar address("address", 0) -> &'a str;
                            scalar key("key", 1) -> &'a str;
                        }
                    }
                }

                ::validation::define_archive_indexed_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Ipv6Clients {
                        model item (0) -> ipv6_clients::Item<'a>;
                        primary_key_fields: [0];
                    }
                }

                pub mod ipv6_clients {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            scalar address("address", 0) -> &'a str;
                            scalar key("key", 1) -> &'a str;
                        }
                    }
                }

                ::validation::define_archive_indexed_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct HostClients {
                        model item (0) -> host_clients::Item<'a>;
                        primary_key_fields: [0];
                    }
                }

                pub mod host_clients {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            scalar name("name", 0) -> &'a str;
                            scalar key("key", 1) -> &'a str;
                        }
                    }
                }
            }
        }
    }
}
