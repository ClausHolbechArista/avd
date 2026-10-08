// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Policies {
        model item (0) -> policies::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod policies {

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
                    scalar application("application", 1) -> &'a str;
                    scalar action("action", 2) -> &'a str;
                    scalar log("log", 3) -> bool;
                    scalar stateless("stateless", 4) -> bool;
                    scalar next_hop("next_hop", 5) -> &'a str;
                }
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Vrfs {
        model item (0) -> vrfs::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod vrfs {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model segments("segments", 1) -> item::Segments<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Segments {
                model item (0) -> segments::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod segments {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    model definition("definition", 1) -> item::Definition<'a>;
                    model policies("policies", 2) -> item::Policies<'a>;
                    scalar fallback_policy("fallback_policy", 3) -> &'a str;
                }
            }

            pub mod item {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Definition {
                        model interfaces("interfaces", 0) -> definition::Interfaces<'a>;
                        model match_lists("match_lists", 1) -> definition::MatchLists<'a>;
                    }
                }

                pub mod definition {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Interfaces {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_indexed_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct MatchLists {
                            model item (0) -> match_lists::Item<'a>;
                            primary_key_fields: [0];
                        }
                    }

                    pub mod match_lists {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Item {
                                scalar address_family("address_family", 0) -> &'a str;
                                scalar covered_prefix_list("covered_prefix_list", 1) -> &'a str;
                                scalar prefix("prefix", 2) -> &'a str;
                            }
                        }
                    }
                }

                ::validation::define_archive_indexed_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Policies {
                        model item (0) -> policies::Item<'a>;
                        primary_key_fields: [0];
                    }
                }

                pub mod policies {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            scalar field_from("from", 0) -> &'a str;
                            scalar policy("policy", 1) -> &'a str;
                        }
                    }
                }
            }
        }
    }
}
