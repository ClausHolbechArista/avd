// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Sampled<'a, Mode> {
    pub encapsulation: ::validated_data::Field<sampled::Encapsulation<'a, Mode>>,
    pub sample: ::validated_data::Field<i64>,
    pub hardware_offload: ::validated_data::Field<sampled::HardwareOffload<'a, Mode>>,
    pub trackers: ::validated_data::Field<sampled::Trackers<'a, Mode>>,
    pub shutdown: ::validated_data::Field<bool>,
}

pub mod sampled {

    #[::validated_data::data_view]
    pub struct Encapsulation<'a, Mode> {
        pub ipv4_ipv6: ::validated_data::Field<bool>,
        pub mpls: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view]
    pub struct HardwareOffload<'a, Mode> {
        pub ipv4: ::validated_data::Field<bool>,
        pub ipv6: ::validated_data::Field<bool>,
        pub threshold_minimum: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct Trackers<'a, Mode> (::validated_data::Field<trackers::Item<'a, Mode>>);

    pub mod trackers {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub table_size: ::validated_data::Field<i64>,
            pub record_export: ::validated_data::Field<item::RecordExport<'a, Mode>>,
            pub name: ::validated_data::Field<&'a str>,
            pub exporters: ::validated_data::Field<item::Exporters<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct RecordExport<'a, Mode> {
                pub mpls: ::validated_data::Field<bool>,
                pub on_inactive_timeout: ::validated_data::Field<i64>,
                pub on_interval: ::validated_data::Field<i64>,
            }

            #[::validated_data::data_view(indexed_list, primary_key(name))]
            pub struct Exporters<'a, Mode> (::validated_data::Field<exporters::Item<'a, Mode>>);

            pub mod exporters {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub name: ::validated_data::Field<&'a str>,
                    pub collectors: ::validated_data::Field<item::Collectors<'a, Mode>>,
                    pub format: ::validated_data::Field<item::Format<'a, Mode>>,
                    pub local_interface: ::validated_data::Field<&'a str>,
                    pub template_interval: ::validated_data::Field<i64>,
                }

                pub mod item {

                    #[::validated_data::data_view(indexed_list, primary_key(host))]
                    pub struct Collectors<'a, Mode> (::validated_data::Field<collectors::Item<'a, Mode>>);

                    pub mod collectors {

                        #[::validated_data::data_view]
                        pub struct Item<'a, Mode> {
                            pub host: ::validated_data::Field<&'a str>,
                            pub port: ::validated_data::Field<i64>,
                        }
                    }

                    #[::validated_data::data_view]
                    pub struct Format<'a, Mode> {
                        pub ipfix_version: ::validated_data::Field<i64>,
                    }
                }
            }
        }
    }
}

#[::validated_data::data_view]
pub struct Hardware<'a, Mode> {
    pub record: ::validated_data::Field<hardware::Record<'a, Mode>>,
    pub trackers: ::validated_data::Field<hardware::Trackers<'a, Mode>>,
    pub shutdown: ::validated_data::Field<bool>,
}

pub mod hardware {

    #[::validated_data::data_view]
    pub struct Record<'a, Mode> {
        pub format_ipfix_standard_timestamps_counters: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct Trackers<'a, Mode> (::validated_data::Field<trackers::Item<'a, Mode>>);

    pub mod trackers {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub record_export: ::validated_data::Field<item::RecordExport<'a, Mode>>,
            pub exporters: ::validated_data::Field<item::Exporters<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct RecordExport<'a, Mode> {
                pub on_inactive_timeout: ::validated_data::Field<i64>,
                pub on_interval: ::validated_data::Field<i64>,
            }

            #[::validated_data::data_view(indexed_list, primary_key(name))]
            pub struct Exporters<'a, Mode> (::validated_data::Field<exporters::Item<'a, Mode>>);

            pub mod exporters {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub name: ::validated_data::Field<&'a str>,
                    pub collectors: ::validated_data::Field<item::Collectors<'a, Mode>>,
                    pub format: ::validated_data::Field<item::Format<'a, Mode>>,
                    pub local_interface: ::validated_data::Field<&'a str>,
                    pub template_interval: ::validated_data::Field<i64>,
                }

                pub mod item {

                    #[::validated_data::data_view(indexed_list, primary_key(host))]
                    pub struct Collectors<'a, Mode> (::validated_data::Field<collectors::Item<'a, Mode>>);

                    pub mod collectors {

                        #[::validated_data::data_view]
                        pub struct Item<'a, Mode> {
                            pub host: ::validated_data::Field<&'a str>,
                            pub port: ::validated_data::Field<i64>,
                        }
                    }

                    #[::validated_data::data_view]
                    pub struct Format<'a, Mode> {
                        pub ipfix_version: ::validated_data::Field<i64>,
                    }
                }
            }
        }
    }
}

#[::validated_data::data_view]
pub struct MirrorOnDrop<'a, Mode> {
    pub encapsulation: ::validated_data::Field<mirror_on_drop::Encapsulation<'a, Mode>>,
    pub sample_limit: ::validated_data::Field<i64>,
    pub trackers: ::validated_data::Field<mirror_on_drop::Trackers<'a, Mode>>,
    pub shutdown: ::validated_data::Field<bool>,
}

pub mod mirror_on_drop {

    #[::validated_data::data_view]
    pub struct Encapsulation<'a, Mode> {
        pub ipv4_ipv6: ::validated_data::Field<bool>,
        pub mpls: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct Trackers<'a, Mode> (::validated_data::Field<trackers::Item<'a, Mode>>);

    pub mod trackers {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub record_export: ::validated_data::Field<item::RecordExport<'a, Mode>>,
            pub exporters: ::validated_data::Field<item::Exporters<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct RecordExport<'a, Mode> {
                pub on_inactive_timeout: ::validated_data::Field<i64>,
                pub on_interval: ::validated_data::Field<i64>,
            }

            #[::validated_data::data_view(indexed_list, primary_key(name))]
            pub struct Exporters<'a, Mode> (::validated_data::Field<exporters::Item<'a, Mode>>);

            pub mod exporters {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub name: ::validated_data::Field<&'a str>,
                    pub collectors: ::validated_data::Field<item::Collectors<'a, Mode>>,
                    pub format: ::validated_data::Field<&'a str>,
                    pub local_interface: ::validated_data::Field<&'a str>,
                    pub template_interval: ::validated_data::Field<i64>,
                    pub dscp: ::validated_data::Field<i64>,
                }

                pub mod item {

                    #[::validated_data::data_view(indexed_list, primary_key(host))]
                    pub struct Collectors<'a, Mode> (::validated_data::Field<collectors::Item<'a, Mode>>);

                    pub mod collectors {

                        #[::validated_data::data_view]
                        pub struct Item<'a, Mode> {
                            pub host: ::validated_data::Field<&'a str>,
                            pub port: ::validated_data::Field<i64>,
                        }
                    }
                }
            }
        }
    }
}
