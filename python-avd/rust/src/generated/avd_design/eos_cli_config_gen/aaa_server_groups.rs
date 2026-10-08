// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar field_type("type", 1) -> &'a str;
        model servers("servers", 2) -> item::Servers<'a>;
    }
}

pub mod item {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Servers {
            model item (0) -> servers::Item<'a>;
        }
    }

    pub mod servers {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar server("server", 0) -> &'a str;
                scalar vrf("vrf", 1) -> &'a str;
                model tls("tls", 2) -> item::Tls<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Tls {
                    scalar enabled("enabled", 0) -> bool;
                    scalar port("port", 1) -> i64;
                }
            }
        }
    }
}
