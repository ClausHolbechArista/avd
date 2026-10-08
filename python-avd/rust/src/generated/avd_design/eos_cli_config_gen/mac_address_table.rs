// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct NotificationHostFlap {
        scalar logging("logging", 0) -> bool;
        model detection("detection", 1) -> notification_host_flap::Detection<'a>;
    }
}

pub mod notification_host_flap {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Detection {
            scalar window("window", 0) -> i64;
            scalar moves("moves", 1) -> i64;
        }
    }
}

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct StaticEntries {
        model item (0) -> static_entries::Item<'a>;
    }
}

pub mod static_entries {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar mac_address("mac_address", 0) -> &'a str;
            scalar vlan("vlan", 1) -> i64;
            scalar drop("drop", 2) -> bool;
            scalar interface("interface", 3) -> &'a str;
            scalar eligibility_forwarding("eligibility_forwarding", 4) -> bool;
        }
    }
}
