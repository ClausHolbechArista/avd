// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DhcpServersIpv4 {
        scalar item (0) -> &'a str;
    }
}

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DhcpServerInterfaces {
        scalar item (0) -> &'a str;
    }
}

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Leases {
        model item (0) -> leases::Item<'a>;
    }
}

pub mod leases {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar ip("ip", 0) -> &'a str;
            scalar mac("mac", 1) -> &'a str;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct LockedAddress {
        scalar expiration_mac_disabled("expiration_mac_disabled", 0) -> bool;
        scalar ipv4_enforcement_disabled("ipv4_enforcement_disabled", 1) -> bool;
        scalar ipv6_enforcement_disabled("ipv6_enforcement_disabled", 2) -> bool;
    }
}
