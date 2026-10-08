// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Fabric {
        scalar act_os_version("act_os_version", 0) -> &'a str;
        scalar act_username("act_username", 1) -> &'a str;
        scalar act_password("act_password", 2) -> &'a str;
        scalar act_internet_access("act_internet_access", 3) -> bool;
        scalar act_ensure_eapi_access("act_ensure_eapi_access", 4) -> bool;
    }
}
