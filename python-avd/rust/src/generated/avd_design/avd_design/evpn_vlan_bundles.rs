// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar id("id", 1) -> i64;
        scalar rt_override("rt_override", 2) -> &'a str;
        scalar rd_override("rd_override", 3) -> &'a str;
        scalar evpn_l2_multi_domain("evpn_l2_multi_domain", 4) -> bool;
        model bgp("bgp", 5) -> item::Bgp<'a>;
    }
}

pub mod item {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Bgp {
            scalar raw_eos_cli("raw_eos_cli", 0) -> &'a str;
        }
    }
}
