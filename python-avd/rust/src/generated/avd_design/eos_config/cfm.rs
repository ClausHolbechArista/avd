// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Domains<'a, Mode> (::validated_data::Field<domains::Item<'a, Mode>>);

pub mod domains {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub level: ::validated_data::RequiredValue<i64, Mode>,
        pub associations: ::validated_data::Field<item::Associations<'a, Mode>>,
        pub intermediate_point: ::validated_data::Field<bool>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(id))]
        pub struct Associations<'a, Mode> (::validated_data::Field<associations::Item<'a, Mode>>);

        pub mod associations {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub id: ::validated_data::Field<i64>,
                pub direction: ::validated_data::Field<&'a str>,
                pub end_points: ::validated_data::Field<item::EndPoints<'a, Mode>>,
                pub profile: ::validated_data::Field<&'a str>,
                pub remote_end_points: ::validated_data::Field<item::RemoteEndPoints<'a, Mode>>,
                pub vlan: ::validated_data::Field<i64>,
            }

            pub mod item {

                #[::validated_data::data_view(indexed_list, primary_key(id))]
                pub struct EndPoints<'a, Mode> (::validated_data::Field<end_points::Item<'a, Mode>>);

                pub mod end_points {

                    #[::validated_data::data_view]
                    pub struct Item<'a, Mode> {
                        pub id: ::validated_data::Field<i64>,
                        pub remote_end_point: ::validated_data::Field<&'a str>,
                        pub interface: ::validated_data::Field<&'a str>,
                    }
                }

                #[::validated_data::data_view(indexed_list, primary_key(id))]
                pub struct RemoteEndPoints<'a, Mode> (::validated_data::Field<remote_end_points::Item<'a, Mode>>);

                pub mod remote_end_points {

                    #[::validated_data::data_view]
                    pub struct Item<'a, Mode> {
                        pub id: ::validated_data::Field<i64>,
                        pub mac_address: ::validated_data::Field<&'a str>,
                    }
                }
            }
        }
    }
}

#[::validated_data::data_view]
pub struct MeasurementLoss<'a, Mode> {
    pub inband: ::validated_data::Field<bool>,
    pub synthetic: ::validated_data::Field<bool>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Profiles<'a, Mode> (::validated_data::Field<profiles::Item<'a, Mode>>);

pub mod profiles {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub alarm_indication: ::validated_data::Field<item::AlarmIndication<'a, Mode>>,
        pub continuity_check: ::validated_data::Field<item::ContinuityCheck<'a, Mode>>,
        pub measurement: ::validated_data::Field<item::Measurement<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct AlarmIndication<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub client_domain_level: ::validated_data::Field<i64>,
            pub tx_interval: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct ContinuityCheck<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub qos_cos: ::validated_data::Field<i64>,
            pub tx_interval: ::validated_data::Field<&'a str>,
            pub alarm_defects: ::validated_data::Field<continuity_check::AlarmDefects<'a, Mode>>,
        }

        pub mod continuity_check {

            #[::validated_data::data_view(list)]
            pub struct AlarmDefects<'a, Mode> (::validated_data::Field<&'a str>);
        }

        #[::validated_data::data_view]
        pub struct Measurement<'a, Mode> {
            pub delay: ::validated_data::Field<measurement::Delay<'a, Mode>>,
            pub loss: ::validated_data::Field<measurement::Loss<'a, Mode>>,
        }

        pub mod measurement {

            #[::validated_data::data_view]
            pub struct Delay<'a, Mode> {
                pub single_ended: ::validated_data::Field<bool>,
                pub qos_cos: ::validated_data::Field<i64>,
                pub tx_interval: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct Loss<'a, Mode> {
                pub single_ended: ::validated_data::Field<bool>,
                pub qos_cos: ::validated_data::Field<i64>,
                pub tx_interval: ::validated_data::Field<&'a str>,
                pub synthetic: ::validated_data::Field<loss::Synthetic<'a, Mode>>,
            }

            pub mod loss {

                #[::validated_data::data_view]
                pub struct Synthetic<'a, Mode> {
                    pub single_ended: ::validated_data::Field<bool>,
                    pub qos_cos: ::validated_data::Field<&'a str>,
                    pub tx_interval: ::validated_data::Field<synthetic::TxInterval<'a, Mode>>,
                }

                pub mod synthetic {

                    #[::validated_data::data_view]
                    pub struct TxInterval<'a, Mode> {
                        pub interval: ::validated_data::RequiredValue<&'a str, Mode>,
                        pub period_frames: ::validated_data::Field<i64>,
                    }
                }
            }
        }
    }
}
