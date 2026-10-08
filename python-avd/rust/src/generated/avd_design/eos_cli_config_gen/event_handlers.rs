// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        model actions("actions", 1) -> item::Actions<'a>;
        scalar delay("delay", 2) -> i64;
        scalar trigger("trigger", 3) -> &'a str;
        model trigger_on_counters("trigger_on_counters", 4) -> item::TriggerOnCounters<'a>;
        model trigger_on_logging("trigger_on_logging", 5) -> item::TriggerOnLogging<'a>;
        model trigger_on_intf("trigger_on_intf", 6) -> item::TriggerOnIntf<'a>;
        model trigger_on_maintenance("trigger_on_maintenance", 7) -> item::TriggerOnMaintenance<'a>;
        scalar asynchronous("asynchronous", 8) -> bool;
    }
}

pub mod item {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Actions {
            scalar bash_command("bash_command", 0) -> &'a str;
            scalar log("log", 1) -> bool;
            scalar increment_device_health_metric("increment_device_health_metric", 2) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct TriggerOnCounters {
            scalar condition("condition", 0) -> &'a str;
            scalar granularity_per_source("granularity_per_source", 1) -> bool;
            scalar poll_interval("poll_interval", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct TriggerOnLogging {
            scalar poll_interval("poll_interval", 0) -> i64;
            scalar regex("regex", 1) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct TriggerOnIntf {
            scalar interface("interface", 0) -> &'a str;
            scalar ip("ip", 1) -> bool;
            scalar ipv6("ipv6", 2) -> bool;
            scalar operstatus("operstatus", 3) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct TriggerOnMaintenance {
            scalar operation("operation", 0) -> &'a str;
            scalar bgp_peer("bgp_peer", 1) -> &'a str;
            scalar action("action", 2) -> &'a str;
            scalar stage("stage", 3) -> &'a str;
            scalar vrf("vrf", 4) -> &'a str;
            scalar interface("interface", 5) -> &'a str;
            scalar unit("unit", 6) -> &'a str;
        }
    }
}
