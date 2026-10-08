// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Sampled {
        model encapsulation("encapsulation", 0) -> sampled::Encapsulation<'a>;
        scalar sample("sample", 1) -> i64;
        model hardware_offload("hardware_offload", 2) -> sampled::HardwareOffload<'a>;
    }
}

pub mod sampled {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Encapsulation {
            scalar ipv4_ipv6("ipv4_ipv6", 0) -> bool;
            scalar mpls("mpls", 1) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct HardwareOffload {
            scalar ipv4("ipv4", 0) -> bool;
            scalar ipv6("ipv6", 1) -> bool;
            scalar threshold_minimum("threshold_minimum", 2) -> i64;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Hardware {
        model record("record", 0) -> hardware::Record<'a>;
    }
}

pub mod hardware {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Record {
            scalar format_ipfix_standard_timestamps_counters("format_ipfix_standard_timestamps_counters", 0) -> bool;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct CloudvisionExporter {
        scalar name("name", 0) -> &'a str;
        scalar vrf("vrf", 1) -> &'a str;
        scalar source_interface("source_interface", 2) -> &'a str;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Trackers {
        model item (0) -> trackers::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod trackers {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model sampled("sampled", 1) -> item::Sampled<'a>;
            model record_export("record_export", 2) -> item::RecordExport<'a>;
            scalar export_to_cloudvision("export_to_cloudvision", 3) -> bool;
            model exporters("exporters", 4) -> item::Exporters<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Sampled {
                scalar table_size("table_size", 0) -> i64;
                model record_export("record_export", 1) -> sampled::RecordExport<'a>;
            }
        }

        pub mod sampled {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct RecordExport {
                    scalar mpls("mpls", 0) -> bool;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct RecordExport {
                scalar on_inactive_timeout("on_inactive_timeout", 0) -> i64;
                scalar on_interval("on_interval", 1) -> i64;
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Exporters {
                model item (0) -> exporters::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod exporters {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    model collectors("collectors", 1) -> item::Collectors<'a>;
                    model format("format", 2) -> item::Format<'a>;
                    scalar local_interface("local_interface", 3) -> &'a str;
                    scalar template_interval("template_interval", 4) -> i64;
                }
            }

            pub mod item {

                ::validation::define_archive_indexed_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Collectors {
                        model item (0) -> collectors::Item<'a>;
                        primary_key_fields: [0];
                    }
                }

                pub mod collectors {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            scalar host("host", 0) -> &'a str;
                            scalar port("port", 1) -> i64;
                        }
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Format {
                        scalar ipfix_version("ipfix_version", 0) -> i64;
                    }
                }
            }
        }
    }
}
