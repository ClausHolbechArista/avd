// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ControlPlane {
        scalar ike_policy_name("ike_policy_name", 0) -> &'a str;
        scalar sa_policy_name("sa_policy_name", 1) -> &'a str;
        scalar profile_name("profile_name", 2) -> &'a str;
        scalar shared_key("shared_key", 3) -> &'a str;
        scalar cleartext_shared_key("cleartext_shared_key", 4) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DataPlane {
        scalar ike_policy_name("ike_policy_name", 0) -> &'a str;
        scalar sa_policy_name("sa_policy_name", 1) -> &'a str;
        scalar profile_name("profile_name", 2) -> &'a str;
        scalar shared_key("shared_key", 3) -> &'a str;
        scalar cleartext_shared_key("cleartext_shared_key", 4) -> &'a str;
    }
}
