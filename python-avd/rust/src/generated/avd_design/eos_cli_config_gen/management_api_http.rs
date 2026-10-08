// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct EnableVrfs {
        model item (0) -> enable_vrfs::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod enable_vrfs {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar access_group("access_group", 1) -> &'a str;
            scalar ipv6_access_group("ipv6_access_group", 2) -> &'a str;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ProtocolHttpsCertificate {
        scalar certificate("certificate", 0) -> &'a str;
        scalar private_key("private_key", 1) -> &'a str;
    }
}
