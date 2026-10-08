// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar counters_per_entry("counters_per_entry", 1) -> bool;
        model entries("entries", 2) -> item::Entries<'a>;
        model sequence_numbers("sequence_numbers", 3) -> item::SequenceNumbers<'a>;
    }
}

pub mod item {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Entries {
            model item (0) -> entries::Item<'a>;
        }
    }

    pub mod entries {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar sequence("sequence", 0) -> i64;
                scalar action("action", 1) -> &'a str;
                scalar remark("remark", 2) -> &'a str;
                scalar source("source", 3) -> &'a str;
                scalar vlan("vlan", 4) -> i64;
                scalar vlan_mask("vlan_mask", 5) -> &'a str;
                scalar inner_vlan("inner_vlan", 6) -> i64;
                scalar inner_vlan_mask("inner_vlan_mask", 7) -> &'a str;
                scalar log("log", 8) -> bool;
                scalar mirror_session("mirror_session", 9) -> &'a str;
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct SequenceNumbers {
            model item (0) -> sequence_numbers::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod sequence_numbers {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar sequence("sequence", 0) -> i64;
                scalar action("action", 1) -> &'a str;
            }
        }
    }
}
