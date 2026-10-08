// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Categories {
        model item (0) -> categories::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod categories {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model applications("applications", 1) -> item::Applications<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Applications {
                model item (0) -> applications::Item<'a>;
            }
        }

        pub mod applications {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    scalar service("service", 1) -> &'a str;
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct FieldSets {
        model l4_ports("l4_ports", 0) -> field_sets::L4Ports<'a>;
        model ipv4_prefixes("ipv4_prefixes", 1) -> field_sets::Ipv4Prefixes<'a>;
    }
}

pub mod field_sets {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct L4Ports {
            model item (0) -> l4_ports::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod l4_ports {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                model port_values("port_values", 1) -> item::PortValues<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct PortValues {
                    scalar item (0) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipv4Prefixes {
            model item (0) -> ipv4_prefixes::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod ipv4_prefixes {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                model prefix_values("prefix_values", 1) -> item::PrefixValues<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct PrefixValues {
                    scalar item (0) -> &'a str;
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Applications {
        model ipv4_applications("ipv4_applications", 0) -> applications::Ipv4Applications<'a>;
        model l4_applications("l4_applications", 1) -> applications::L4Applications<'a>;
    }
}

pub mod applications {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipv4Applications {
            model item (0) -> ipv4_applications::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod ipv4_applications {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar src_prefix_set_name("src_prefix_set_name", 1) -> &'a str;
                scalar dest_prefix_set_name("dest_prefix_set_name", 2) -> &'a str;
                model dscp_ranges("dscp_ranges", 3) -> item::DscpRanges<'a>;
                model protocols("protocols", 4) -> item::Protocols<'a>;
                model protocol_ranges("protocol_ranges", 5) -> item::ProtocolRanges<'a>;
                scalar udp_src_port_set_name("udp_src_port_set_name", 6) -> &'a str;
                scalar tcp_src_port_set_name("tcp_src_port_set_name", 7) -> &'a str;
                scalar udp_dest_port_set_name("udp_dest_port_set_name", 8) -> &'a str;
                scalar tcp_dest_port_set_name("tcp_dest_port_set_name", 9) -> &'a str;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct DscpRanges {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Protocols {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct ProtocolRanges {
                    scalar item (0) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct L4Applications {
            model item (0) -> l4_applications::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod l4_applications {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                model protocols("protocols", 1) -> item::Protocols<'a>;
                model protocol_ranges("protocol_ranges", 2) -> item::ProtocolRanges<'a>;
                scalar udp_src_port_set_name("udp_src_port_set_name", 3) -> &'a str;
                scalar tcp_src_port_set_name("tcp_src_port_set_name", 4) -> &'a str;
                scalar udp_dest_port_set_name("udp_dest_port_set_name", 5) -> &'a str;
                scalar tcp_dest_port_set_name("tcp_dest_port_set_name", 6) -> &'a str;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Protocols {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct ProtocolRanges {
                    scalar item (0) -> &'a str;
                }
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ApplicationProfiles {
        model item (0) -> application_profiles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod application_profiles {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model applications("applications", 1) -> item::Applications<'a>;
            model application_transports("application_transports", 2) -> item::ApplicationTransports<'a>;
            model categories("categories", 3) -> item::Categories<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Applications {
                model item (0) -> applications::Item<'a>;
            }
        }

        pub mod applications {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    scalar service("service", 1) -> &'a str;
                }
            }
        }

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct ApplicationTransports {
                scalar item (0) -> &'a str;
            }
        }

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Categories {
                model item (0) -> categories::Item<'a>;
            }
        }

        pub mod categories {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    scalar service("service", 1) -> &'a str;
                }
            }
        }
    }
}
