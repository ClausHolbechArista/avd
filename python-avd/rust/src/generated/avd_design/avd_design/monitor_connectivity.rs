// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct InterfaceSets {
        model item (0) -> interface_sets::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod interface_sets {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model interfaces("interfaces", 1) -> item::Interfaces<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Interfaces {
                scalar item (0) -> &'a str;
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Hosts {
        model item (0) -> hosts::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod hosts {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar description("description", 1) -> &'a str;
            scalar single_line_description("single_line_description", 2) -> &'a str;
            scalar ip("ip", 3) -> &'a str;
            scalar icmp_echo_size("icmp_echo_size", 4) -> i64;
            scalar local_interfaces("local_interfaces", 5) -> &'a str;
            scalar address_only("address_only", 6) -> bool;
            scalar url("url", 7) -> &'a str;
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
            scalar description("description", 1) -> &'a str;
            scalar single_line_description("single_line_description", 2) -> &'a str;
            model interface_sets("interface_sets", 3) -> item::InterfaceSets<'a>;
            scalar local_interfaces("local_interfaces", 4) -> &'a str;
            scalar address_only("address_only", 5) -> bool;
            model hosts("hosts", 6) -> item::Hosts<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct InterfaceSets {
                model item (0) -> interface_sets::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod interface_sets {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    model interfaces("interfaces", 1) -> item::Interfaces<'a>;
                }
            }

            pub mod item {

                ::validation::define_archive_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Interfaces {
                        scalar item (0) -> &'a str;
                    }
                }
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Hosts {
                model item (0) -> hosts::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod hosts {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    scalar description("description", 1) -> &'a str;
                    scalar single_line_description("single_line_description", 2) -> &'a str;
                    scalar ip("ip", 3) -> &'a str;
                    scalar icmp_echo_size("icmp_echo_size", 4) -> i64;
                    scalar local_interfaces("local_interfaces", 5) -> &'a str;
                    scalar address_only("address_only", 6) -> bool;
                    scalar url("url", 7) -> &'a str;
                }
            }
        }
    }
}
