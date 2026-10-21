// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Qos<'a, Mode> {
    pub map: ::data_views::Field<Map<'a, Mode>>,
    pub rewrite_dscp: ::data_views::Field<bool>,
    pub random_detect: ::data_views::Field<RandomDetect<'a, Mode>>,
    pub tx_queue: ::data_views::Field<TxQueue<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Map<'a, Mode> {
    pub cos: ::data_views::Field<map::Cos<'a, Mode>>,
    pub dscp: ::data_views::Field<map::Dscp<'a, Mode>>,
    pub exp: ::data_views::Field<map::Exp<'a, Mode>>,
    pub traffic_class: ::data_views::Field<map::TrafficClass<'a, Mode>>,
}

pub mod map {

    #[::data_views::data_view(list)]
    pub struct Cos<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(list)]
    pub struct Dscp<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(list)]
    pub struct Exp<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(list)]
    pub struct TrafficClass<'a, Mode> (::data_views::Field<&'a str>);
}

#[::data_views::data_view]
pub struct RandomDetect<'a, Mode> {
    pub ecn: ::data_views::Field<random_detect::Ecn<'a, Mode>>,
}

pub mod random_detect {

    #[::data_views::data_view]
    pub struct Ecn<'a, Mode> {
        pub allow_non_ect: ::data_views::Field<ecn::AllowNonEct<'a, Mode>>,
        pub global_buffer: ::data_views::Field<ecn::GlobalBuffer<'a, Mode>>,
    }

    pub mod ecn {

        #[::data_views::data_view]
        pub struct AllowNonEct<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub chip_based: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct GlobalBuffer<'a, Mode> {
            pub units: ::data_views::RequiredValue<&'a str, Mode>,
            pub min: ::data_views::RequiredValue<i64, Mode>,
            pub max: ::data_views::RequiredValue<i64, Mode>,
        }
    }
}

#[::data_views::data_view]
pub struct TxQueue<'a, Mode> {
    pub shape_rate_percent_adaptive: ::data_views::Field<bool>,
    pub queues: ::data_views::Field<tx_queue::Queues<'a, Mode>>,
}

pub mod tx_queue {

    #[::data_views::data_view(indexed_list, primary_key(id))]
    pub struct Queues<'a, Mode> (::data_views::Field<queues::Item<'a, Mode>>);

    pub mod queues {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub id: ::data_views::Field<i64>,
            pub scheduler_profile_responsive: ::data_views::Field<bool>,
        }
    }
}
