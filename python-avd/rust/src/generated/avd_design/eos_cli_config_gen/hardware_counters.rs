// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Features {
        model item (0) -> features::Item<'a>;
    }
}

pub mod features {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar direction("direction", 1) -> &'a str;
            scalar enabled("enabled", 2) -> bool;
            scalar address_type("address_type", 3) -> &'a str;
            scalar layer3("layer3", 4) -> bool;
            scalar vrf("vrf", 5) -> &'a str;
            scalar prefix("prefix", 6) -> &'a str;
            scalar units_packets("units_packets", 7) -> bool;
        }
    }
}
