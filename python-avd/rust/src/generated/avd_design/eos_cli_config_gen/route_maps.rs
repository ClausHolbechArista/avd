// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        model sequence_numbers("sequence_numbers", 1) -> item::SequenceNumbers<'a>;
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
                scalar field_type("type", 1) -> &'a str;
                scalar description("description", 2) -> &'a str;
                model field_match("match", 3) -> item::FieldMatch<'a>;
                model set("set", 4) -> item::Set<'a>;
                scalar sub_route_map("sub_route_map", 5) -> &'a str;
                model field_continue("continue", 6) -> item::FieldContinue<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct FieldMatch {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Set {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct FieldContinue {
                    scalar enabled("enabled", 0) -> bool;
                    scalar sequence_number("sequence_number", 1) -> i64;
                }
            }
        }
    }
}
