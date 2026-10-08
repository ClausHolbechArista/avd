// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Buffered {
        scalar size("size", 0) -> i64;
        scalar level("level", 1) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Synchronous {
        scalar level("level", 0) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Format {
        scalar timestamp("timestamp", 0) -> &'a str;
        scalar hostname("hostname", 1) -> &'a str;
        scalar sequence_numbers("sequence_numbers", 2) -> bool;
        scalar rfc5424("rfc5424", 3) -> bool;
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
            scalar source_interface("source_interface", 1) -> &'a str;
            scalar local_interface("local_interface", 2) -> &'a str;
            model hosts("hosts", 3) -> item::Hosts<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Hosts {
                model item (0) -> hosts::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod hosts {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    scalar protocol("protocol", 1) -> &'a str;
                    model ports("ports", 2) -> item::Ports<'a>;
                    scalar ssl_profile("ssl_profile", 3) -> &'a str;
                }
            }

            pub mod item {

                ::validation::define_archive_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Ports {
                        scalar item (0) -> i64;
                    }
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Policy {
        model field_match("match", 0) -> policy::FieldMatch<'a>;
    }
}

pub mod policy {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct FieldMatch {
            model match_lists("match_lists", 0) -> field_match::MatchLists<'a>;
        }
    }

    pub mod field_match {

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
                    scalar name("name", 0) -> &'a str;
                    scalar action("action", 1) -> &'a str;
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Event {
        scalar congestion_drops_interval("congestion_drops_interval", 0) -> i64;
        scalar global_link_status("global_link_status", 1) -> bool;
        model storm_control("storm_control", 2) -> event::StormControl<'a>;
    }
}

pub mod event {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct StormControl {
            model discards("discards", 0) -> storm_control::Discards<'a>;
        }
    }

    pub mod storm_control {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Discards {
                scalar field_global("global", 0) -> bool;
                scalar interval("interval", 1) -> i64;
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Level {
        model item (0) -> level::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod level {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar facility("facility", 0) -> &'a str;
            scalar severity("severity", 1) -> &'a str;
        }
    }
}
