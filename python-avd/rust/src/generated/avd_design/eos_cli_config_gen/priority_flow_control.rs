// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Watchdog {
        scalar action("action", 0) -> &'a str;
        scalar timeout("timeout", 1) -> &'a str;
        scalar polling_interval("polling_interval", 2) -> &'a str;
        scalar recovery_time("recovery_time", 3) -> &'a str;
        scalar override_action_drop("override_action_drop", 4) -> bool;
    }
}
