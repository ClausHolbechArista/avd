// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct PeerAddressHeartbeat {
        scalar peer_ip("peer_ip", 0) -> &'a str;
        scalar vrf("vrf", 1) -> &'a str;
    }
}
