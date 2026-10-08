// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Ipv4 {
        scalar bfd("bfd", 0) -> bool;
        scalar make_before_break("make_before_break", 1) -> bool;
        scalar message_hello_address_secondary_ipv6("message_hello_address_secondary_ipv6", 2) -> bool;
        scalar ssm_range("ssm_range", 3) -> &'a str;
        scalar register_local_interface("register_local_interface", 4) -> &'a str;
        model rp_addresses("rp_addresses", 5) -> ipv4::RpAddresses<'a>;
        model anycast_rps("anycast_rps", 6) -> ipv4::AnycastRps<'a>;
    }
}

pub mod ipv4 {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct RpAddresses {
            model item (0) -> rp_addresses::Item<'a>;
        }
    }

    pub mod rp_addresses {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar address("address", 0) -> &'a str;
                model groups("groups", 1) -> item::Groups<'a>;
                model access_lists("access_lists", 2) -> item::AccessLists<'a>;
                scalar priority("priority", 3) -> i64;
                scalar hashmask("hashmask", 4) -> i64;
                scalar field_override("override", 5) -> bool;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Groups {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AccessLists {
                    scalar item (0) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct AnycastRps {
            model item (0) -> anycast_rps::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod anycast_rps {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar address("address", 0) -> &'a str;
                model other_anycast_rp_addresses("other_anycast_rp_addresses", 1) -> item::OtherAnycastRpAddresses<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct OtherAnycastRpAddresses {
                    model item (0) -> other_anycast_rp_addresses::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod other_anycast_rp_addresses {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar address("address", 0) -> &'a str;
                        scalar register_count("register_count", 1) -> &'a str;
                    }
                }
            }
        }
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
            model ipv4("ipv4", 1) -> item::Ipv4<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ipv4 {
                scalar bfd("bfd", 0) -> bool;
                scalar make_before_break("make_before_break", 1) -> bool;
                scalar register_local_interface("register_local_interface", 2) -> &'a str;
                model rp_addresses("rp_addresses", 3) -> ipv4::RpAddresses<'a>;
                model anycast_rps("anycast_rps", 4) -> ipv4::AnycastRps<'a>;
                scalar ssm_range("ssm_range", 5) -> &'a str;
            }
        }

        pub mod ipv4 {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct RpAddresses {
                    model item (0) -> rp_addresses::Item<'a>;
                }
            }

            pub mod rp_addresses {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar address("address", 0) -> &'a str;
                        model groups("groups", 1) -> item::Groups<'a>;
                        model access_lists("access_lists", 2) -> item::AccessLists<'a>;
                        scalar priority("priority", 3) -> i64;
                        scalar hashmask("hashmask", 4) -> i64;
                        scalar field_override("override", 5) -> bool;
                    }
                }

                pub mod item {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Groups {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct AccessLists {
                            scalar item (0) -> &'a str;
                        }
                    }
                }
            }

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AnycastRps {
                    model item (0) -> anycast_rps::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod anycast_rps {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar address("address", 0) -> &'a str;
                        model other_anycast_rp_addresses("other_anycast_rp_addresses", 1) -> item::OtherAnycastRpAddresses<'a>;
                    }
                }

                pub mod item {

                    ::validation::define_archive_indexed_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct OtherAnycastRpAddresses {
                            model item (0) -> other_anycast_rp_addresses::Item<'a>;
                            primary_key_fields: [0];
                        }
                    }

                    pub mod other_anycast_rp_addresses {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Item {
                                scalar address("address", 0) -> &'a str;
                                scalar register_count("register_count", 1) -> &'a str;
                            }
                        }
                    }
                }
            }
        }
    }
}
