// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar description("description", 1) -> &'a str;
        scalar shutdown("shutdown", 2) -> bool;
        scalar mtu("mtu", 3) -> i64;
        scalar ip_address("ip_address", 4) -> &'a str;
        scalar ipv6_address_auto_config("ipv6_address_auto_config", 5) -> bool;
        model flow_tracker("flow_tracker", 6) -> item::FlowTracker<'a>;
        model tcp_mss_ceiling("tcp_mss_ceiling", 7) -> item::TcpMssCeiling<'a>;
        scalar eos_cli("eos_cli", 8) -> &'a str;
    }
}

pub mod item {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct FlowTracker {
            scalar sampled("sampled", 0) -> &'a str;
            scalar hardware("hardware", 1) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct TcpMssCeiling {
            scalar ipv4("ipv4", 0) -> i64;
            scalar ipv6("ipv6", 1) -> i64;
            scalar direction("direction", 2) -> &'a str;
        }
    }
}
