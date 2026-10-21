// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Cfm<'a, Mode> {
    pub continuity_check_loc_state_action_disable_interface_routing: ::data_views::Field<bool>,
    pub domains: ::data_views::Field<Domains<'a, Mode>>,
    pub measurement_loss: ::data_views::Field<MeasurementLoss<'a, Mode>>,
    pub profiles: ::data_views::Field<Profiles<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Domains<'a, Mode> (::data_views::Field<domains::Item<'a, Mode>>);

pub mod domains {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub level: ::data_views::RequiredValue<i64, Mode>,
        pub associations: ::data_views::Field<item::Associations<'a, Mode>>,
        pub intermediate_point: ::data_views::Field<bool>,
    }

    pub mod item {

        #[::data_views::data_view(indexed_list, primary_key(id))]
        pub struct Associations<'a, Mode> (::data_views::Field<associations::Item<'a, Mode>>);

        pub mod associations {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub id: ::data_views::Field<i64>,
                pub direction: ::data_views::Field<&'a str>,
                pub end_points: ::data_views::Field<item::EndPoints<'a, Mode>>,
                pub profile: ::data_views::Field<&'a str>,
                pub remote_end_points: ::data_views::Field<item::RemoteEndPoints<'a, Mode>>,
                pub vlan: ::data_views::Field<i64>,
            }

            pub mod item {

                #[::data_views::data_view(indexed_list, primary_key(id))]
                pub struct EndPoints<'a, Mode> (::data_views::Field<end_points::Item<'a, Mode>>);

                pub mod end_points {

                    #[::data_views::data_view]
                    pub struct Item<'a, Mode> {
                        pub id: ::data_views::Field<i64>,
                        pub remote_end_point: ::data_views::Field<&'a str>,
                        pub interface: ::data_views::Field<&'a str>,
                    }
                }

                #[::data_views::data_view(indexed_list, primary_key(id))]
                pub struct RemoteEndPoints<'a, Mode> (::data_views::Field<remote_end_points::Item<'a, Mode>>);

                pub mod remote_end_points {

                    #[::data_views::data_view]
                    pub struct Item<'a, Mode> {
                        pub id: ::data_views::Field<i64>,
                        pub mac_address: ::data_views::Field<&'a str>,
                    }
                }
            }
        }
    }
}

#[::data_views::data_view]
pub struct MeasurementLoss<'a, Mode> {
    pub inband: ::data_views::Field<bool>,
    pub synthetic: ::data_views::Field<bool>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Profiles<'a, Mode> (::data_views::Field<profiles::Item<'a, Mode>>);

pub mod profiles {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub alarm_indication: ::data_views::Field<item::AlarmIndication<'a, Mode>>,
        pub continuity_check: ::data_views::Field<item::ContinuityCheck<'a, Mode>>,
        pub measurement: ::data_views::Field<item::Measurement<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct AlarmIndication<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub client_domain_level: ::data_views::Field<i64>,
            pub tx_interval: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct ContinuityCheck<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub qos_cos: ::data_views::Field<i64>,
            pub tx_interval: ::data_views::Field<&'a str>,
            pub alarm_defects: ::data_views::Field<continuity_check::AlarmDefects<'a, Mode>>,
        }

        pub mod continuity_check {

            #[::data_views::data_view(list)]
            pub struct AlarmDefects<'a, Mode> (::data_views::Field<&'a str>);
        }

        #[::data_views::data_view]
        pub struct Measurement<'a, Mode> {
            pub delay: ::data_views::Field<measurement::Delay<'a, Mode>>,
            pub loss: ::data_views::Field<measurement::Loss<'a, Mode>>,
        }

        pub mod measurement {

            #[::data_views::data_view]
            pub struct Delay<'a, Mode> {
                pub single_ended: ::data_views::Field<bool>,
                pub qos_cos: ::data_views::Field<i64>,
                pub tx_interval: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct Loss<'a, Mode> {
                pub single_ended: ::data_views::Field<bool>,
                pub qos_cos: ::data_views::Field<i64>,
                pub tx_interval: ::data_views::Field<&'a str>,
                pub synthetic: ::data_views::Field<loss::Synthetic<'a, Mode>>,
            }

            pub mod loss {

                #[::data_views::data_view]
                pub struct Synthetic<'a, Mode> {
                    pub single_ended: ::data_views::Field<bool>,
                    pub qos_cos: ::data_views::Field<&'a str>,
                    pub tx_interval: ::data_views::Field<synthetic::TxInterval<'a, Mode>>,
                }

                pub mod synthetic {

                    #[::data_views::data_view]
                    pub struct TxInterval<'a, Mode> {
                        pub interval: ::data_views::RequiredValue<&'a str, Mode>,
                        pub period_frames: ::data_views::Field<i64>,
                    }
                }
            }
        }
    }
}
