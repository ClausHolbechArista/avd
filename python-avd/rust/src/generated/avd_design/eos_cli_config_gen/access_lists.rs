// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar counters_per_entry("counters_per_entry", 1) -> bool;
        scalar permit_response_traffic("permit_response_traffic", 2) -> &'a str;
        model sequence_numbers("sequence_numbers", 3) -> item::SequenceNumbers<'a>;
    }
}

pub mod item {

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
