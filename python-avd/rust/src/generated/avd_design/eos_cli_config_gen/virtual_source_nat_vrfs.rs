// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar ip_address("ip_address", 1) -> &'a str;
        scalar ipv6_address("ipv6_address", 2) -> &'a str;
    }
}
