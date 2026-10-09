// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Region<'a, Mode> {
    pub name: ::validated_data::RequiredValue<&'a str, Mode>,
    pub id: ::validated_data::RequiredValue<i64, Mode>,
}

#[::validated_data::data_view]
pub struct Zone<'a, Mode> {
    pub name: ::validated_data::RequiredValue<&'a str, Mode>,
    pub id: ::validated_data::RequiredValue<i64, Mode>,
}

#[::validated_data::data_view]
pub struct Site<'a, Mode> {
    pub name: ::validated_data::RequiredValue<&'a str, Mode>,
    pub id: ::validated_data::RequiredValue<i64, Mode>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Profiles<'a, Mode> (::validated_data::Field<profiles::Item<'a, Mode>>);

pub mod profiles {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub load_balance_policy: ::validated_data::Field<&'a str>,
        pub internet_exit_policy: ::validated_data::Field<&'a str>,
        pub metric_order: ::validated_data::Field<item::MetricOrder<'a, Mode>>,
        pub outlier_elimination: ::validated_data::Field<item::OutlierElimination<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct MetricOrder<'a, Mode> {
            pub preferred_metric: ::validated_data::RequiredValue<&'a str, Mode>,
        }

        #[::validated_data::data_view]
        pub struct OutlierElimination<'a, Mode> {
            pub disabled: ::validated_data::Field<bool>,
            pub threshold: ::validated_data::Field<outlier_elimination::Threshold<'a, Mode>>,
        }

        pub mod outlier_elimination {

            #[::validated_data::data_view]
            pub struct Threshold<'a, Mode> {
                pub jitter: ::validated_data::Field<i64>,
                pub latency: ::validated_data::Field<i64>,
                pub load: ::validated_data::Field<&'a str>,
                pub loss_rate: ::validated_data::Field<&'a str>,
            }
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Policies<'a, Mode> (::validated_data::Field<policies::Item<'a, Mode>>);

pub mod policies {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub matches: ::validated_data::Field<item::Matches<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(list)]
        pub struct Matches<'a, Mode> (::validated_data::Field<matches::Item<'a, Mode>>);

        pub mod matches {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub application_profile: ::validated_data::Field<&'a str>,
                pub avt_profile: ::validated_data::Field<&'a str>,
                pub dscp: ::validated_data::Field<i64>,
                pub traffic_class: ::validated_data::Field<i64>,
            }
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub policy: ::validated_data::Field<&'a str>,
        pub profiles: ::validated_data::Field<item::Profiles<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct Profiles<'a, Mode> (::validated_data::Field<profiles::Item<'a, Mode>>);

        pub mod profiles {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub id: ::validated_data::RequiredValue<i64, Mode>,
            }
        }
    }
}
