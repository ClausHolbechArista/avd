// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Authentication {
        scalar empty_passwords("empty_passwords", 0) -> &'a str;
        model protocols("protocols", 1) -> authentication::Protocols<'a>;
    }
}

pub mod authentication {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Protocols {
            scalar item (0) -> &'a str;
        }
    }
}

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Cipher {
        scalar item (0) -> &'a str;
    }
}

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct KeyExchange {
        scalar item (0) -> &'a str;
    }
}

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Mac {
        scalar item (0) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Hostkey {
        model server("server", 0) -> hostkey::Server<'a>;
        scalar server_cert("server_cert", 1) -> &'a str;
        scalar client_strict_checking("client_strict_checking", 2) -> bool;
    }
}

pub mod hostkey {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Server {
            scalar item (0) -> &'a str;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Connection {
        scalar limit("limit", 0) -> i64;
        scalar per_host("per_host", 1) -> i64;
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
            scalar enable("enable", 1) -> bool;
            scalar ip_access_group_in("ip_access_group_in", 2) -> &'a str;
            scalar ipv6_access_group_in("ipv6_access_group_in", 3) -> &'a str;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ClientAlive {
        scalar count_max("count_max", 0) -> i64;
        scalar interval("interval", 1) -> i64;
    }
}
