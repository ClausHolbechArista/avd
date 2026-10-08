// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Lldp {
        scalar transmit("transmit", 0) -> bool;
        scalar receive("receive", 1) -> bool;
        scalar ztp_vlan("ztp_vlan", 2) -> i64;
    }
}
