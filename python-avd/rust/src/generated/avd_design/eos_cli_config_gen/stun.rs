// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Client {
        model server_profiles("server_profiles", 0) -> client::ServerProfiles<'a>;
    }
}

pub mod client {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct ServerProfiles {
            model item (0) -> server_profiles::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod server_profiles {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar ip_address("ip_address", 1) -> &'a str;
                scalar ssl_profile("ssl_profile", 2) -> &'a str;
                scalar port("port", 3) -> i64;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Server {
        model local_interfaces("local_interfaces", 0) -> server::LocalInterfaces<'a>;
        scalar bindings_timeout("bindings_timeout", 1) -> i64;
        scalar ssl_profile("ssl_profile", 2) -> &'a str;
        model ssl_connection_lifetime("ssl_connection_lifetime", 3) -> server::SslConnectionLifetime<'a>;
        scalar port("port", 4) -> i64;
    }
}

pub mod server {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct LocalInterfaces {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct SslConnectionLifetime {
            scalar minutes("minutes", 0) -> i64;
            scalar hours("hours", 1) -> i64;
        }
    }
}
