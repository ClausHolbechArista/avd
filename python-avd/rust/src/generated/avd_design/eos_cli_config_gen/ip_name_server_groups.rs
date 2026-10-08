// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        model vrfs("vrfs", 1) -> item::Vrfs<'a>;
        model name_servers("name_servers", 2) -> item::NameServers<'a>;
        scalar dns_domain("dns_domain", 3) -> &'a str;
        model ip_domain_lists("ip_domain_lists", 4) -> item::IpDomainLists<'a>;
    }
}

pub mod item {

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
                model name_servers("name_servers", 1) -> item::NameServers<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct NameServers {
                    model item (0) -> name_servers::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod name_servers {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar ip_address("ip_address", 0) -> &'a str;
                        scalar priority("priority", 1) -> i64;
                    }
                }
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct NameServers {
            model item (0) -> name_servers::Item<'a>;
        }
    }

    pub mod name_servers {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar ip_address("ip_address", 0) -> &'a str;
                scalar vrf("vrf", 1) -> &'a str;
                scalar priority("priority", 2) -> i64;
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct IpDomainLists {
            scalar item (0) -> &'a str;
        }
    }
}
