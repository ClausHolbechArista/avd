// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct QosProfiles<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view]
pub struct Item<'a, Mode> {
    pub name: ::data_views::Field<&'a str>,
    pub trust: ::data_views::Field<&'a str>,
    pub cos: ::data_views::Field<i64>,
    pub dscp: ::data_views::Field<i64>,
    pub shape: ::data_views::Field<item::Shape<'a, Mode>>,
    pub service_policy: ::data_views::Field<item::ServicePolicy<'a, Mode>>,
    pub tx_queues: ::data_views::Field<item::TxQueues<'a, Mode>>,
    pub uc_tx_queues: ::data_views::Field<item::UcTxQueues<'a, Mode>>,
    pub mc_tx_queues: ::data_views::Field<item::McTxQueues<'a, Mode>>,
    pub priority_flow_control: ::data_views::Field<item::PriorityFlowControl<'a, Mode>>,
}

pub mod item {

    #[::data_views::data_view]
    pub struct Shape<'a, Mode> {
        pub rate: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view]
    pub struct ServicePolicy<'a, Mode> {
        #[data_view(rename = "type")]
        pub field_type: ::data_views::Field<service_policy::FieldType<'a, Mode>>,
    }

    pub mod service_policy {

        #[::data_views::data_view]
        pub struct FieldType<'a, Mode> {
            pub qos_input: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(id))]
    pub struct TxQueues<'a, Mode> (::data_views::Field<tx_queues::Item<'a, Mode>>);

    pub mod tx_queues {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub id: ::data_views::Field<i64>,
            pub bandwidth_percent: ::data_views::Field<i64>,
            pub bandwidth_guaranteed_percent: ::data_views::Field<i64>,
            pub priority: ::data_views::Field<&'a str>,
            pub shape: ::data_views::Field<item::Shape<'a, Mode>>,
            pub comment: ::data_views::Field<&'a str>,
            pub random_detect: ::data_views::Field<item::RandomDetect<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct Shape<'a, Mode> {
                pub rate: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct RandomDetect<'a, Mode> {
                pub ecn: ::data_views::Field<random_detect::Ecn<'a, Mode>>,
                pub drop: ::data_views::Field<random_detect::Drop<'a, Mode>>,
            }

            pub mod random_detect {

                #[::data_views::data_view]
                pub struct Ecn<'a, Mode> {
                    pub count: ::data_views::Field<bool>,
                    pub threshold: ::data_views::Field<ecn::Threshold<'a, Mode>>,
                }

                pub mod ecn {

                    #[::data_views::data_view]
                    pub struct Threshold<'a, Mode> {
                        pub units: ::data_views::RequiredValue<&'a str, Mode>,
                        pub min: ::data_views::RequiredValue<i64, Mode>,
                        pub max: ::data_views::RequiredValue<i64, Mode>,
                        pub max_probability: ::data_views::Field<i64>,
                        pub weight: ::data_views::Field<i64>,
                    }
                }

                #[::data_views::data_view]
                pub struct Drop<'a, Mode> {
                    pub threshold: ::data_views::Field<drop::Threshold<'a, Mode>>,
                }

                pub mod drop {

                    #[::data_views::data_view]
                    pub struct Threshold<'a, Mode> {
                        pub units: ::data_views::RequiredValue<&'a str, Mode>,
                        pub drop_precedence: ::data_views::Field<i64>,
                        pub min: ::data_views::RequiredValue<i64, Mode>,
                        pub max: ::data_views::RequiredValue<i64, Mode>,
                        pub drop_probability: ::data_views::RequiredValue<i64, Mode>,
                        pub weight: ::data_views::Field<i64>,
                    }
                }
            }
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(id))]
    pub struct UcTxQueues<'a, Mode> (::data_views::Field<uc_tx_queues::Item<'a, Mode>>);

    pub mod uc_tx_queues {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub id: ::data_views::Field<i64>,
            pub bandwidth_percent: ::data_views::Field<i64>,
            pub bandwidth_guaranteed_percent: ::data_views::Field<i64>,
            pub priority: ::data_views::Field<&'a str>,
            pub shape: ::data_views::Field<item::Shape<'a, Mode>>,
            pub comment: ::data_views::Field<&'a str>,
            pub random_detect: ::data_views::Field<item::RandomDetect<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct Shape<'a, Mode> {
                pub rate: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct RandomDetect<'a, Mode> {
                pub ecn: ::data_views::Field<random_detect::Ecn<'a, Mode>>,
                pub drop: ::data_views::Field<random_detect::Drop<'a, Mode>>,
            }

            pub mod random_detect {

                #[::data_views::data_view]
                pub struct Ecn<'a, Mode> {
                    pub count: ::data_views::Field<bool>,
                    pub threshold: ::data_views::Field<ecn::Threshold<'a, Mode>>,
                }

                pub mod ecn {

                    #[::data_views::data_view]
                    pub struct Threshold<'a, Mode> {
                        pub units: ::data_views::RequiredValue<&'a str, Mode>,
                        pub min: ::data_views::RequiredValue<i64, Mode>,
                        pub max: ::data_views::RequiredValue<i64, Mode>,
                        pub max_probability: ::data_views::Field<i64>,
                        pub weight: ::data_views::Field<i64>,
                    }
                }

                #[::data_views::data_view]
                pub struct Drop<'a, Mode> {
                    pub threshold: ::data_views::Field<drop::Threshold<'a, Mode>>,
                }

                pub mod drop {

                    #[::data_views::data_view]
                    pub struct Threshold<'a, Mode> {
                        pub units: ::data_views::RequiredValue<&'a str, Mode>,
                        pub drop_precedence: ::data_views::Field<i64>,
                        pub min: ::data_views::RequiredValue<i64, Mode>,
                        pub max: ::data_views::RequiredValue<i64, Mode>,
                        pub drop_probability: ::data_views::RequiredValue<i64, Mode>,
                        pub weight: ::data_views::Field<i64>,
                    }
                }
            }
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(id))]
    pub struct McTxQueues<'a, Mode> (::data_views::Field<mc_tx_queues::Item<'a, Mode>>);

    pub mod mc_tx_queues {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub id: ::data_views::Field<i64>,
            pub bandwidth_percent: ::data_views::Field<i64>,
            pub bandwidth_guaranteed_percent: ::data_views::Field<i64>,
            pub priority: ::data_views::Field<&'a str>,
            pub shape: ::data_views::Field<item::Shape<'a, Mode>>,
            pub comment: ::data_views::Field<&'a str>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct Shape<'a, Mode> {
                pub rate: ::data_views::Field<&'a str>,
            }
        }
    }

    #[::data_views::data_view]
    pub struct PriorityFlowControl<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub watchdog: ::data_views::Field<priority_flow_control::Watchdog<'a, Mode>>,
        pub priorities: ::data_views::Field<priority_flow_control::Priorities<'a, Mode>>,
    }

    pub mod priority_flow_control {

        #[::data_views::data_view]
        pub struct Watchdog<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub action: ::data_views::Field<&'a str>,
            pub timer: ::data_views::Field<watchdog::Timer<'a, Mode>>,
        }

        pub mod watchdog {

            #[::data_views::data_view]
            pub struct Timer<'a, Mode> {
                pub timeout: ::data_views::RequiredValue<&'a str, Mode>,
                pub polling_interval: ::data_views::RequiredValue<&'a str, Mode>,
                pub recovery_time: ::data_views::RequiredValue<&'a str, Mode>,
                pub forced: ::data_views::Field<bool>,
            }
        }

        #[::data_views::data_view(indexed_list, primary_key(priority))]
        pub struct Priorities<'a, Mode> (::data_views::Field<priorities::Item<'a, Mode>>);

        pub mod priorities {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub priority: ::data_views::Field<i64>,
                pub no_drop: ::data_views::RequiredValue<bool, Mode>,
            }
        }
    }
}
