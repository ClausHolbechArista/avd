// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Profiles {
        model item (0) -> profiles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod profiles {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar vrf("vrf", 1) -> &'a str;
            model destination("destination", 2) -> item::Destination<'a>;
            model source("source", 3) -> item::Source<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Destination {
                model dynamic("dynamic", 0) -> destination::Dynamic<'a>;
                model field_static("static", 1) -> destination::FieldStatic<'a>;
            }
        }

        pub mod destination {

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Dynamic {
                    model item (0) -> dynamic::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod dynamic {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar access_list("access_list", 0) -> &'a str;
                        scalar comment("comment", 1) -> &'a str;
                        scalar pool_name("pool_name", 2) -> &'a str;
                        scalar priority("priority", 3) -> i64;
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct FieldStatic {
                    model item (0) -> field_static::Item<'a>;
                }
            }

            pub mod field_static {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar access_list("access_list", 0) -> &'a str;
                        scalar comment("comment", 1) -> &'a str;
                        scalar direction("direction", 2) -> &'a str;
                        scalar group("group", 3) -> i64;
                        scalar original_ip("original_ip", 4) -> &'a str;
                        scalar original_port("original_port", 5) -> i64;
                        scalar priority("priority", 6) -> i64;
                        scalar protocol("protocol", 7) -> &'a str;
                        scalar translated_ip("translated_ip", 8) -> &'a str;
                        scalar translated_port("translated_port", 9) -> i64;
                    }
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Source {
                model dynamic("dynamic", 0) -> source::Dynamic<'a>;
                model field_static("static", 1) -> source::FieldStatic<'a>;
            }
        }

        pub mod source {

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Dynamic {
                    model item (0) -> dynamic::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod dynamic {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar access_list("access_list", 0) -> &'a str;
                        scalar comment("comment", 1) -> &'a str;
                        scalar nat_type("nat_type", 2) -> &'a str;
                        scalar pool_name("pool_name", 3) -> &'a str;
                        scalar priority("priority", 4) -> i64;
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct FieldStatic {
                    model item (0) -> field_static::Item<'a>;
                }
            }

            pub mod field_static {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar access_list("access_list", 0) -> &'a str;
                        scalar comment("comment", 1) -> &'a str;
                        scalar direction("direction", 2) -> &'a str;
                        scalar group("group", 3) -> i64;
                        scalar original_ip("original_ip", 4) -> &'a str;
                        scalar original_port("original_port", 5) -> i64;
                        scalar priority("priority", 6) -> i64;
                        scalar protocol("protocol", 7) -> &'a str;
                        scalar translated_ip("translated_ip", 8) -> &'a str;
                        scalar translated_port("translated_port", 9) -> i64;
                    }
                }
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Pools {
        model item (0) -> pools::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod pools {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar field_type("type", 1) -> &'a str;
            scalar prefix_length("prefix_length", 2) -> i64;
            model ranges("ranges", 3) -> item::Ranges<'a>;
            scalar utilization_log_threshold("utilization_log_threshold", 4) -> i64;
        }
    }

    pub mod item {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ranges {
                model item (0) -> ranges::Item<'a>;
            }
        }

        pub mod ranges {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar first_ip("first_ip", 0) -> &'a str;
                    scalar last_ip("last_ip", 1) -> &'a str;
                    scalar first_port("first_port", 2) -> i64;
                    scalar last_port("last_port", 3) -> i64;
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Synchronization {
        scalar description("description", 0) -> &'a str;
        scalar expiry_interval("expiry_interval", 1) -> i64;
        scalar local_interface("local_interface", 2) -> &'a str;
        scalar peer_address("peer_address", 3) -> &'a str;
        model port_range("port_range", 4) -> synchronization::PortRange<'a>;
        scalar shutdown("shutdown", 5) -> bool;
    }
}

pub mod synchronization {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PortRange {
            scalar first_port("first_port", 0) -> i64;
            scalar last_port("last_port", 1) -> i64;
            scalar split_disabled("split_disabled", 2) -> bool;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Translation {
        model address_selection("address_selection", 0) -> translation::AddressSelection<'a>;
        scalar counters("counters", 1) -> bool;
        model low_mark("low_mark", 2) -> translation::LowMark<'a>;
        model max_entries("max_entries", 3) -> translation::MaxEntries<'a>;
        model timeouts("timeouts", 4) -> translation::Timeouts<'a>;
    }
}

pub mod translation {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct AddressSelection {
            scalar any("any", 0) -> bool;
            scalar hash_field_source_ip("hash_field_source_ip", 1) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct LowMark {
            scalar percentage("percentage", 0) -> i64;
            scalar host_percentage("host_percentage", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct MaxEntries {
            scalar limit("limit", 0) -> i64;
            scalar host_limit("host_limit", 1) -> i64;
            model ip_limits("ip_limits", 2) -> max_entries::IpLimits<'a>;
        }
    }

    pub mod max_entries {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct IpLimits {
                model item (0) -> ip_limits::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod ip_limits {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar ip("ip", 0) -> &'a str;
                    scalar limit("limit", 1) -> i64;
                }
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Timeouts {
            model item (0) -> timeouts::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod timeouts {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar protocol("protocol", 0) -> &'a str;
                scalar timeout("timeout", 1) -> i64;
            }
        }
    }
}
