// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct FlowTracking<'a, Mode> {
    pub sampled: ::data_views::Field<Sampled<'a, Mode>>,
    pub hardware: ::data_views::Field<Hardware<'a, Mode>>,
    pub mirror_on_drop: ::data_views::Field<MirrorOnDrop<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Sampled<'a, Mode> {
    pub encapsulation: ::data_views::Field<sampled::Encapsulation<'a, Mode>>,
    pub sample: ::data_views::Field<i64>,
    pub hardware_offload: ::data_views::Field<sampled::HardwareOffload<'a, Mode>>,
    pub trackers: ::data_views::Field<sampled::Trackers<'a, Mode>>,
    pub shutdown: ::data_views::Field<bool>,
}

pub mod sampled {

    #[::data_views::data_view]
    pub struct Encapsulation<'a, Mode> {
        pub ipv4_ipv6: ::data_views::Field<bool>,
        pub mpls: ::data_views::Field<bool>,
    }

    #[::data_views::data_view]
    pub struct HardwareOffload<'a, Mode> {
        pub ipv4: ::data_views::Field<bool>,
        pub ipv6: ::data_views::Field<bool>,
        pub threshold_minimum: ::data_views::Field<i64>,
    }

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct Trackers<'a, Mode> (::data_views::Field<trackers::Item<'a, Mode>>);

    pub mod trackers {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub table_size: ::data_views::Field<i64>,
            pub record_export: ::data_views::Field<item::RecordExport<'a, Mode>>,
            pub name: ::data_views::Field<&'a str>,
            pub exporters: ::data_views::Field<item::Exporters<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct RecordExport<'a, Mode> {
                pub mpls: ::data_views::Field<bool>,
                pub on_inactive_timeout: ::data_views::Field<i64>,
                pub on_interval: ::data_views::Field<i64>,
            }

            #[::data_views::data_view(indexed_list, primary_key(name))]
            pub struct Exporters<'a, Mode> (::data_views::Field<exporters::Item<'a, Mode>>);

            pub mod exporters {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub name: ::data_views::Field<&'a str>,
                    pub collectors: ::data_views::Field<item::Collectors<'a, Mode>>,
                    pub format: ::data_views::Field<item::Format<'a, Mode>>,
                    pub local_interface: ::data_views::Field<&'a str>,
                    pub template_interval: ::data_views::Field<i64>,
                }

                pub mod item {

                    #[::data_views::data_view(indexed_list, primary_key(host))]
                    pub struct Collectors<'a, Mode> (::data_views::Field<collectors::Item<'a, Mode>>);

                    pub mod collectors {

                        #[::data_views::data_view]
                        pub struct Item<'a, Mode> {
                            pub host: ::data_views::Field<&'a str>,
                            pub port: ::data_views::Field<i64>,
                        }
                    }

                    #[::data_views::data_view]
                    pub struct Format<'a, Mode> {
                        pub ipfix_version: ::data_views::Field<i64>,
                    }
                }
            }
        }
    }
}

#[::data_views::data_view]
pub struct Hardware<'a, Mode> {
    pub record: ::data_views::Field<hardware::Record<'a, Mode>>,
    pub trackers: ::data_views::Field<hardware::Trackers<'a, Mode>>,
    pub shutdown: ::data_views::Field<bool>,
}

pub mod hardware {

    #[::data_views::data_view]
    pub struct Record<'a, Mode> {
        pub format_ipfix_standard_timestamps_counters: ::data_views::Field<bool>,
    }

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct Trackers<'a, Mode> (::data_views::Field<trackers::Item<'a, Mode>>);

    pub mod trackers {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub record_export: ::data_views::Field<item::RecordExport<'a, Mode>>,
            pub exporters: ::data_views::Field<item::Exporters<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct RecordExport<'a, Mode> {
                pub on_inactive_timeout: ::data_views::Field<i64>,
                pub on_interval: ::data_views::Field<i64>,
            }

            #[::data_views::data_view(indexed_list, primary_key(name))]
            pub struct Exporters<'a, Mode> (::data_views::Field<exporters::Item<'a, Mode>>);

            pub mod exporters {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub name: ::data_views::Field<&'a str>,
                    pub collectors: ::data_views::Field<item::Collectors<'a, Mode>>,
                    pub format: ::data_views::Field<item::Format<'a, Mode>>,
                    pub local_interface: ::data_views::Field<&'a str>,
                    pub template_interval: ::data_views::Field<i64>,
                }

                pub mod item {

                    #[::data_views::data_view(indexed_list, primary_key(host))]
                    pub struct Collectors<'a, Mode> (::data_views::Field<collectors::Item<'a, Mode>>);

                    pub mod collectors {

                        #[::data_views::data_view]
                        pub struct Item<'a, Mode> {
                            pub host: ::data_views::Field<&'a str>,
                            pub port: ::data_views::Field<i64>,
                        }
                    }

                    #[::data_views::data_view]
                    pub struct Format<'a, Mode> {
                        pub ipfix_version: ::data_views::Field<i64>,
                    }
                }
            }
        }
    }
}

#[::data_views::data_view]
pub struct MirrorOnDrop<'a, Mode> {
    pub encapsulation: ::data_views::Field<mirror_on_drop::Encapsulation<'a, Mode>>,
    pub sample_limit: ::data_views::Field<i64>,
    pub trackers: ::data_views::Field<mirror_on_drop::Trackers<'a, Mode>>,
    pub shutdown: ::data_views::Field<bool>,
}

pub mod mirror_on_drop {

    #[::data_views::data_view]
    pub struct Encapsulation<'a, Mode> {
        pub ipv4_ipv6: ::data_views::Field<bool>,
        pub mpls: ::data_views::Field<bool>,
    }

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct Trackers<'a, Mode> (::data_views::Field<trackers::Item<'a, Mode>>);

    pub mod trackers {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub record_export: ::data_views::Field<item::RecordExport<'a, Mode>>,
            pub exporters: ::data_views::Field<item::Exporters<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct RecordExport<'a, Mode> {
                pub on_inactive_timeout: ::data_views::Field<i64>,
                pub on_interval: ::data_views::Field<i64>,
            }

            #[::data_views::data_view(indexed_list, primary_key(name))]
            pub struct Exporters<'a, Mode> (::data_views::Field<exporters::Item<'a, Mode>>);

            pub mod exporters {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub name: ::data_views::Field<&'a str>,
                    pub collectors: ::data_views::Field<item::Collectors<'a, Mode>>,
                    pub format: ::data_views::Field<&'a str>,
                    pub local_interface: ::data_views::Field<&'a str>,
                    pub template_interval: ::data_views::Field<i64>,
                    pub dscp: ::data_views::Field<i64>,
                }

                pub mod item {

                    #[::data_views::data_view(indexed_list, primary_key(host))]
                    pub struct Collectors<'a, Mode> (::data_views::Field<collectors::Item<'a, Mode>>);

                    pub mod collectors {

                        #[::data_views::data_view]
                        pub struct Item<'a, Mode> {
                            pub host: ::data_views::Field<&'a str>,
                            pub port: ::data_views::Field<i64>,
                        }
                    }
                }
            }
        }
    }
}
