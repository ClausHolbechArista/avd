// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Loopback {
        scalar ipv6_prefix_length("ipv6_prefix_length", 0) -> i64;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Mlag {
        scalar algorithm("algorithm", 0) -> &'a str;
        scalar ipv4_prefix_length("ipv4_prefix_length", 1) -> i64;
        scalar ipv6_prefix_length("ipv6_prefix_length", 2) -> i64;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct P2pUplinks {
        scalar ipv4_prefix_length("ipv4_prefix_length", 0) -> i64;
        scalar ipv6_prefix_length("ipv6_prefix_length", 1) -> i64;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct WanHa {
        scalar ipv4_prefix_length("ipv4_prefix_length", 0) -> i64;
    }
}
