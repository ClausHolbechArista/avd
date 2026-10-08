// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar trust("trust", 1) -> &'a str;
        scalar cos("cos", 2) -> i64;
        scalar dscp("dscp", 3) -> i64;
        model shape("shape", 4) -> item::Shape<'a>;
        model service_policy("service_policy", 5) -> item::ServicePolicy<'a>;
        model tx_queues("tx_queues", 6) -> item::TxQueues<'a>;
        model uc_tx_queues("uc_tx_queues", 7) -> item::UcTxQueues<'a>;
        model mc_tx_queues("mc_tx_queues", 8) -> item::McTxQueues<'a>;
        model priority_flow_control("priority_flow_control", 9) -> item::PriorityFlowControl<'a>;
    }
}

pub mod item {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Shape {
            scalar rate("rate", 0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct ServicePolicy {
            model field_type("type", 0) -> service_policy::FieldType<'a>;
        }
    }

    pub mod service_policy {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct FieldType {
                scalar qos_input("qos_input", 0) -> &'a str;
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct TxQueues {
            model item (0) -> tx_queues::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod tx_queues {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar id("id", 0) -> i64;
                scalar bandwidth_percent("bandwidth_percent", 1) -> i64;
                scalar bandwidth_guaranteed_percent("bandwidth_guaranteed_percent", 2) -> i64;
                scalar priority("priority", 3) -> &'a str;
                model shape("shape", 4) -> item::Shape<'a>;
                scalar comment("comment", 5) -> &'a str;
                model random_detect("random_detect", 6) -> item::RandomDetect<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Shape {
                    scalar rate("rate", 0) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct RandomDetect {
                    model ecn("ecn", 0) -> random_detect::Ecn<'a>;
                    model drop("drop", 1) -> random_detect::Drop<'a>;
                }
            }

            pub mod random_detect {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Ecn {
                        scalar count("count", 0) -> bool;
                        model threshold("threshold", 1) -> ecn::Threshold<'a>;
                    }
                }

                pub mod ecn {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Threshold {
                            scalar units("units", 0) -> &'a str;
                            scalar min("min", 1) -> i64;
                            scalar max("max", 2) -> i64;
                            scalar max_probability("max_probability", 3) -> i64;
                            scalar weight("weight", 4) -> i64;
                        }
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Drop {
                        model threshold("threshold", 0) -> drop::Threshold<'a>;
                    }
                }

                pub mod drop {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Threshold {
                            scalar units("units", 0) -> &'a str;
                            scalar drop_precedence("drop_precedence", 1) -> i64;
                            scalar min("min", 2) -> i64;
                            scalar max("max", 3) -> i64;
                            scalar drop_probability("drop_probability", 4) -> i64;
                            scalar weight("weight", 5) -> i64;
                        }
                    }
                }
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct UcTxQueues {
            model item (0) -> uc_tx_queues::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod uc_tx_queues {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar id("id", 0) -> i64;
                scalar bandwidth_percent("bandwidth_percent", 1) -> i64;
                scalar bandwidth_guaranteed_percent("bandwidth_guaranteed_percent", 2) -> i64;
                scalar priority("priority", 3) -> &'a str;
                model shape("shape", 4) -> item::Shape<'a>;
                scalar comment("comment", 5) -> &'a str;
                model random_detect("random_detect", 6) -> item::RandomDetect<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Shape {
                    scalar rate("rate", 0) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct RandomDetect {
                    model ecn("ecn", 0) -> random_detect::Ecn<'a>;
                    model drop("drop", 1) -> random_detect::Drop<'a>;
                }
            }

            pub mod random_detect {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Ecn {
                        scalar count("count", 0) -> bool;
                        model threshold("threshold", 1) -> ecn::Threshold<'a>;
                    }
                }

                pub mod ecn {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Threshold {
                            scalar units("units", 0) -> &'a str;
                            scalar min("min", 1) -> i64;
                            scalar max("max", 2) -> i64;
                            scalar max_probability("max_probability", 3) -> i64;
                            scalar weight("weight", 4) -> i64;
                        }
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Drop {
                        model threshold("threshold", 0) -> drop::Threshold<'a>;
                    }
                }

                pub mod drop {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Threshold {
                            scalar units("units", 0) -> &'a str;
                            scalar drop_precedence("drop_precedence", 1) -> i64;
                            scalar min("min", 2) -> i64;
                            scalar max("max", 3) -> i64;
                            scalar drop_probability("drop_probability", 4) -> i64;
                            scalar weight("weight", 5) -> i64;
                        }
                    }
                }
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct McTxQueues {
            model item (0) -> mc_tx_queues::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod mc_tx_queues {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar id("id", 0) -> i64;
                scalar bandwidth_percent("bandwidth_percent", 1) -> i64;
                scalar bandwidth_guaranteed_percent("bandwidth_guaranteed_percent", 2) -> i64;
                scalar priority("priority", 3) -> &'a str;
                model shape("shape", 4) -> item::Shape<'a>;
                scalar comment("comment", 5) -> &'a str;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Shape {
                    scalar rate("rate", 0) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PriorityFlowControl {
            scalar enabled("enabled", 0) -> bool;
            model watchdog("watchdog", 1) -> priority_flow_control::Watchdog<'a>;
            model priorities("priorities", 2) -> priority_flow_control::Priorities<'a>;
        }
    }

    pub mod priority_flow_control {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Watchdog {
                scalar enabled("enabled", 0) -> bool;
                scalar action("action", 1) -> &'a str;
                model timer("timer", 2) -> watchdog::Timer<'a>;
            }
        }

        pub mod watchdog {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Timer {
                    scalar timeout("timeout", 0) -> &'a str;
                    scalar polling_interval("polling_interval", 1) -> &'a str;
                    scalar recovery_time("recovery_time", 2) -> &'a str;
                    scalar forced("forced", 3) -> bool;
                }
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Priorities {
                model item (0) -> priorities::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod priorities {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar priority("priority", 0) -> i64;
                    scalar no_drop("no_drop", 1) -> bool;
                }
            }
        }
    }
}
