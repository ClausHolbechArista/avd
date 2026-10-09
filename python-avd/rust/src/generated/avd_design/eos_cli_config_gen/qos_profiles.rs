// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub trust: ::validated_data::Field<&'a str>,
    pub cos: ::validated_data::Field<i64>,
    pub dscp: ::validated_data::Field<i64>,
    pub shape: ::validated_data::Field<item::Shape<'a, Mode>>,
    pub service_policy: ::validated_data::Field<item::ServicePolicy<'a, Mode>>,
    pub tx_queues: ::validated_data::Field<item::TxQueues<'a, Mode>>,
    pub uc_tx_queues: ::validated_data::Field<item::UcTxQueues<'a, Mode>>,
    pub mc_tx_queues: ::validated_data::Field<item::McTxQueues<'a, Mode>>,
    pub priority_flow_control: ::validated_data::Field<item::PriorityFlowControl<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view]
    pub struct Shape<'a, Mode> {
        pub rate: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct ServicePolicy<'a, Mode> {
        #[data_view(rename = "type")]
        pub field_type: ::validated_data::Field<service_policy::FieldType<'a, Mode>>,
    }

    pub mod service_policy {

        #[::validated_data::data_view]
        pub struct FieldType<'a, Mode> {
            pub qos_input: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(id))]
    pub struct TxQueues<'a, Mode> (::validated_data::Field<tx_queues::Item<'a, Mode>>);

    pub mod tx_queues {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub id: ::validated_data::Field<i64>,
            pub bandwidth_percent: ::validated_data::Field<i64>,
            pub bandwidth_guaranteed_percent: ::validated_data::Field<i64>,
            pub priority: ::validated_data::Field<&'a str>,
            pub shape: ::validated_data::Field<item::Shape<'a, Mode>>,
            pub comment: ::validated_data::Field<&'a str>,
            pub random_detect: ::validated_data::Field<item::RandomDetect<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct Shape<'a, Mode> {
                pub rate: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct RandomDetect<'a, Mode> {
                pub ecn: ::validated_data::Field<random_detect::Ecn<'a, Mode>>,
                pub drop: ::validated_data::Field<random_detect::Drop<'a, Mode>>,
            }

            pub mod random_detect {

                #[::validated_data::data_view]
                pub struct Ecn<'a, Mode> {
                    pub count: ::validated_data::Field<bool>,
                    pub threshold: ::validated_data::Field<ecn::Threshold<'a, Mode>>,
                }

                pub mod ecn {

                    #[::validated_data::data_view]
                    pub struct Threshold<'a, Mode> {
                        pub units: ::validated_data::RequiredValue<&'a str, Mode>,
                        pub min: ::validated_data::RequiredValue<i64, Mode>,
                        pub max: ::validated_data::RequiredValue<i64, Mode>,
                        pub max_probability: ::validated_data::Field<i64>,
                        pub weight: ::validated_data::Field<i64>,
                    }
                }

                #[::validated_data::data_view]
                pub struct Drop<'a, Mode> {
                    pub threshold: ::validated_data::Field<drop::Threshold<'a, Mode>>,
                }

                pub mod drop {

                    #[::validated_data::data_view]
                    pub struct Threshold<'a, Mode> {
                        pub units: ::validated_data::RequiredValue<&'a str, Mode>,
                        pub drop_precedence: ::validated_data::Field<i64>,
                        pub min: ::validated_data::RequiredValue<i64, Mode>,
                        pub max: ::validated_data::RequiredValue<i64, Mode>,
                        pub drop_probability: ::validated_data::RequiredValue<i64, Mode>,
                        pub weight: ::validated_data::Field<i64>,
                    }
                }
            }
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(id))]
    pub struct UcTxQueues<'a, Mode> (::validated_data::Field<uc_tx_queues::Item<'a, Mode>>);

    pub mod uc_tx_queues {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub id: ::validated_data::Field<i64>,
            pub bandwidth_percent: ::validated_data::Field<i64>,
            pub bandwidth_guaranteed_percent: ::validated_data::Field<i64>,
            pub priority: ::validated_data::Field<&'a str>,
            pub shape: ::validated_data::Field<item::Shape<'a, Mode>>,
            pub comment: ::validated_data::Field<&'a str>,
            pub random_detect: ::validated_data::Field<item::RandomDetect<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct Shape<'a, Mode> {
                pub rate: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct RandomDetect<'a, Mode> {
                pub ecn: ::validated_data::Field<random_detect::Ecn<'a, Mode>>,
                pub drop: ::validated_data::Field<random_detect::Drop<'a, Mode>>,
            }

            pub mod random_detect {

                #[::validated_data::data_view]
                pub struct Ecn<'a, Mode> {
                    pub count: ::validated_data::Field<bool>,
                    pub threshold: ::validated_data::Field<ecn::Threshold<'a, Mode>>,
                }

                pub mod ecn {

                    #[::validated_data::data_view]
                    pub struct Threshold<'a, Mode> {
                        pub units: ::validated_data::RequiredValue<&'a str, Mode>,
                        pub min: ::validated_data::RequiredValue<i64, Mode>,
                        pub max: ::validated_data::RequiredValue<i64, Mode>,
                        pub max_probability: ::validated_data::Field<i64>,
                        pub weight: ::validated_data::Field<i64>,
                    }
                }

                #[::validated_data::data_view]
                pub struct Drop<'a, Mode> {
                    pub threshold: ::validated_data::Field<drop::Threshold<'a, Mode>>,
                }

                pub mod drop {

                    #[::validated_data::data_view]
                    pub struct Threshold<'a, Mode> {
                        pub units: ::validated_data::RequiredValue<&'a str, Mode>,
                        pub drop_precedence: ::validated_data::Field<i64>,
                        pub min: ::validated_data::RequiredValue<i64, Mode>,
                        pub max: ::validated_data::RequiredValue<i64, Mode>,
                        pub drop_probability: ::validated_data::RequiredValue<i64, Mode>,
                        pub weight: ::validated_data::Field<i64>,
                    }
                }
            }
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(id))]
    pub struct McTxQueues<'a, Mode> (::validated_data::Field<mc_tx_queues::Item<'a, Mode>>);

    pub mod mc_tx_queues {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub id: ::validated_data::Field<i64>,
            pub bandwidth_percent: ::validated_data::Field<i64>,
            pub bandwidth_guaranteed_percent: ::validated_data::Field<i64>,
            pub priority: ::validated_data::Field<&'a str>,
            pub shape: ::validated_data::Field<item::Shape<'a, Mode>>,
            pub comment: ::validated_data::Field<&'a str>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct Shape<'a, Mode> {
                pub rate: ::validated_data::Field<&'a str>,
            }
        }
    }

    #[::validated_data::data_view]
    pub struct PriorityFlowControl<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub watchdog: ::validated_data::Field<priority_flow_control::Watchdog<'a, Mode>>,
        pub priorities: ::validated_data::Field<priority_flow_control::Priorities<'a, Mode>>,
    }

    pub mod priority_flow_control {

        #[::validated_data::data_view]
        pub struct Watchdog<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub action: ::validated_data::Field<&'a str>,
            pub timer: ::validated_data::Field<watchdog::Timer<'a, Mode>>,
        }

        pub mod watchdog {

            #[::validated_data::data_view]
            pub struct Timer<'a, Mode> {
                pub timeout: ::validated_data::RequiredValue<&'a str, Mode>,
                pub polling_interval: ::validated_data::RequiredValue<&'a str, Mode>,
                pub recovery_time: ::validated_data::RequiredValue<&'a str, Mode>,
                pub forced: ::validated_data::Field<bool>,
            }
        }

        #[::validated_data::data_view(indexed_list, primary_key(priority))]
        pub struct Priorities<'a, Mode> (::validated_data::Field<priorities::Item<'a, Mode>>);

        pub mod priorities {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub priority: ::validated_data::Field<i64>,
                pub no_drop: ::validated_data::RequiredValue<bool, Mode>,
            }
        }
    }
}
