// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar vrf("vrf", 0) -> &'a str;
        scalar prefix("prefix", 1) -> &'a str;
        scalar interface("interface", 2) -> &'a str;
        scalar next_hop("next_hop", 3) -> &'a str;
        scalar track_bfd("track_bfd", 4) -> bool;
        scalar distance("distance", 5) -> i64;
        scalar tag("tag", 6) -> i64;
        scalar name("name", 7) -> &'a str;
        scalar metric("metric", 8) -> i64;
    }
}
