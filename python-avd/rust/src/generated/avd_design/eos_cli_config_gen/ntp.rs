// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct LocalInterface {
        scalar name("name", 0) -> &'a str;
        scalar vrf("vrf", 1) -> &'a str;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Servers {
        model item (0) -> servers::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod servers {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar burst("burst", 1) -> bool;
            scalar iburst("iburst", 2) -> bool;
            scalar key("key", 3) -> i64;
            scalar local_interface("local_interface", 4) -> &'a str;
            scalar source_address("source_address", 5) -> &'a str;
            scalar maxpoll("maxpoll", 6) -> i64;
            scalar minpoll("minpoll", 7) -> i64;
            scalar preferred("preferred", 8) -> bool;
            scalar version("version", 9) -> i64;
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AuthenticationKeys {
        model item (0) -> authentication_keys::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod authentication_keys {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar id("id", 0) -> i64;
            scalar hash_algorithm("hash_algorithm", 1) -> &'a str;
            scalar key("key", 2) -> &'a str;
            scalar key_type("key_type", 3) -> &'a str;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Serve {
        scalar serve_all("serve_all", 0) -> bool;
        scalar access_group("access_group", 1) -> &'a str;
        scalar ipv6_access_group("ipv6_access_group", 2) -> &'a str;
        model vrfs("vrfs", 3) -> serve::Vrfs<'a>;
    }
}

pub mod serve {

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
                scalar serve_all("serve_all", 1) -> bool;
                scalar access_group("access_group", 2) -> &'a str;
                scalar ipv6_access_group("ipv6_access_group", 3) -> &'a str;
            }
        }
    }
}
