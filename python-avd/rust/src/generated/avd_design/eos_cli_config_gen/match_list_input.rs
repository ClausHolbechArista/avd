// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct PrefixIpv4 {
        model item (0) -> prefix_ipv4::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod prefix_ipv4 {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model prefixes("prefixes", 1) -> item::Prefixes<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Prefixes {
                scalar item (0) -> &'a str;
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct PrefixIpv6 {
        model item (0) -> prefix_ipv6::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod prefix_ipv6 {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model prefixes("prefixes", 1) -> item::Prefixes<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Prefixes {
                scalar item (0) -> &'a str;
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct String {
        model item (0) -> string::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod string {

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
                    scalar match_regex("match_regex", 1) -> &'a str;
                }
            }
        }
    }
}
