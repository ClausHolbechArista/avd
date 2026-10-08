// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Phone {
        scalar cos("cos", 0) -> i64;
        scalar trunk("trunk", 1) -> &'a str;
        scalar vlan("vlan", 2) -> i64;
        scalar access_list_bypass("access_list_bypass", 3) -> bool;
        scalar qos_trust("qos_trust", 4) -> &'a str;
    }
}
