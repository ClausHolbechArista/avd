// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Attribute32IncludeInAccessReq {
        scalar hostname("hostname", 0) -> bool;
        scalar format("format", 1) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DynamicAuthorization {
        scalar port("port", 0) -> i64;
        scalar tls_ssl_profile("tls_ssl_profile", 1) -> &'a str;
    }
}

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
            scalar host("host", 0) -> &'a str;
            model tls("tls", 1) -> item::Tls<'a>;
            scalar timeout("timeout", 2) -> i64;
            scalar retransmit("retransmit", 3) -> i64;
            scalar key("key", 4) -> &'a str;
            scalar key_type("key_type", 5) -> &'a str;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Tls {
                scalar enabled("enabled", 0) -> bool;
                scalar ssl_profile("ssl_profile", 1) -> &'a str;
                scalar port("port", 2) -> i64;
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
            model servers("servers", 1) -> item::Servers<'a>;
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
                    scalar host("host", 0) -> &'a str;
                    model tls("tls", 1) -> item::Tls<'a>;
                    scalar timeout("timeout", 2) -> i64;
                    scalar retransmit("retransmit", 3) -> i64;
                    scalar key("key", 4) -> &'a str;
                    scalar key_type("key_type", 5) -> &'a str;
                }
            }

            pub mod item {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Tls {
                        scalar enabled("enabled", 0) -> bool;
                        scalar ssl_profile("ssl_profile", 1) -> &'a str;
                        scalar port("port", 2) -> i64;
                    }
                }
            }
        }
    }
}
