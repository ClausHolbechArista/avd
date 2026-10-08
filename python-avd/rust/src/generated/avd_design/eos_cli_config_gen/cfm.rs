// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Domains {
        model item (0) -> domains::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod domains {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar level("level", 1) -> i64;
            model associations("associations", 2) -> item::Associations<'a>;
            scalar intermediate_point("intermediate_point", 3) -> bool;
        }
    }

    pub mod item {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Associations {
                model item (0) -> associations::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod associations {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar id("id", 0) -> i64;
                    scalar direction("direction", 1) -> &'a str;
                    model end_points("end_points", 2) -> item::EndPoints<'a>;
                    scalar profile("profile", 3) -> &'a str;
                    model remote_end_points("remote_end_points", 4) -> item::RemoteEndPoints<'a>;
                    scalar vlan("vlan", 5) -> i64;
                }
            }

            pub mod item {

                ::validation::define_archive_indexed_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct EndPoints {
                        model item (0) -> end_points::Item<'a>;
                        primary_key_fields: [0];
                    }
                }

                pub mod end_points {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            scalar id("id", 0) -> i64;
                            scalar remote_end_point("remote_end_point", 1) -> &'a str;
                            scalar interface("interface", 2) -> &'a str;
                        }
                    }
                }

                ::validation::define_archive_indexed_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct RemoteEndPoints {
                        model item (0) -> remote_end_points::Item<'a>;
                        primary_key_fields: [0];
                    }
                }

                pub mod remote_end_points {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            scalar id("id", 0) -> i64;
                            scalar mac_address("mac_address", 1) -> &'a str;
                        }
                    }
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MeasurementLoss {
        scalar inband("inband", 0) -> bool;
        scalar synthetic("synthetic", 1) -> bool;
    }
}

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
            model alarm_indication("alarm_indication", 1) -> item::AlarmIndication<'a>;
            model continuity_check("continuity_check", 2) -> item::ContinuityCheck<'a>;
            model measurement("measurement", 3) -> item::Measurement<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AlarmIndication {
                scalar enabled("enabled", 0) -> bool;
                scalar client_domain_level("client_domain_level", 1) -> i64;
                scalar tx_interval("tx_interval", 2) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct ContinuityCheck {
                scalar enabled("enabled", 0) -> bool;
                scalar qos_cos("qos_cos", 1) -> i64;
                scalar tx_interval("tx_interval", 2) -> &'a str;
                model alarm_defects("alarm_defects", 3) -> continuity_check::AlarmDefects<'a>;
            }
        }

        pub mod continuity_check {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AlarmDefects {
                    scalar item (0) -> &'a str;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Measurement {
                model delay("delay", 0) -> measurement::Delay<'a>;
                model loss("loss", 1) -> measurement::Loss<'a>;
            }
        }

        pub mod measurement {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Delay {
                    scalar single_ended("single_ended", 0) -> bool;
                    scalar qos_cos("qos_cos", 1) -> i64;
                    scalar tx_interval("tx_interval", 2) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Loss {
                    scalar single_ended("single_ended", 0) -> bool;
                    scalar qos_cos("qos_cos", 1) -> i64;
                    scalar tx_interval("tx_interval", 2) -> &'a str;
                    model synthetic("synthetic", 3) -> loss::Synthetic<'a>;
                }
            }

            pub mod loss {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Synthetic {
                        scalar single_ended("single_ended", 0) -> bool;
                        scalar qos_cos("qos_cos", 1) -> &'a str;
                        model tx_interval("tx_interval", 2) -> synthetic::TxInterval<'a>;
                    }
                }

                pub mod synthetic {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct TxInterval {
                            scalar interval("interval", 0) -> &'a str;
                            scalar period_frames("period_frames", 1) -> i64;
                        }
                    }
                }
            }
        }
    }
}
