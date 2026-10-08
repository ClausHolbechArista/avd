// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar profile("profile", 0) -> &'a str;
        model announce("announce", 1) -> item::Announce<'a>;
        scalar delay_req("delay_req", 2) -> i64;
        model sync_message("sync_message", 3) -> item::SyncMessage<'a>;
        scalar transport("transport", 4) -> &'a str;
        model management("management", 5) -> item::Management<'a>;
    }
}

pub mod item {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Announce {
            scalar interval("interval", 0) -> i64;
            scalar timeout("timeout", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct SyncMessage {
            scalar interval("interval", 0) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Management {
            scalar drop("drop", 0) -> bool;
        }
    }
}
