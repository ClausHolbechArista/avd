// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct CvxSecondary {
        scalar name("name", 0) -> &'a str;
        scalar shutdown("shutdown", 1) -> bool;
        model server_hosts("server_hosts", 2) -> cvx_secondary::ServerHosts<'a>;
        scalar vrf("vrf", 3) -> &'a str;
        scalar source_interface("source_interface", 4) -> &'a str;
    }
}

pub mod cvx_secondary {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct ServerHosts {
            scalar item (0) -> &'a str;
        }
    }
}
