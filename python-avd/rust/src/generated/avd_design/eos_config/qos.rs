// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Map<'a, Mode> {
    pub cos: ::validated_data::Field<map::Cos<'a, Mode>>,
    pub dscp: ::validated_data::Field<map::Dscp<'a, Mode>>,
    pub exp: ::validated_data::Field<map::Exp<'a, Mode>>,
    pub traffic_class: ::validated_data::Field<map::TrafficClass<'a, Mode>>,
}

pub mod map {

    #[::validated_data::data_view(list)]
    pub struct Cos<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list)]
    pub struct Dscp<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list)]
    pub struct Exp<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list)]
    pub struct TrafficClass<'a, Mode> (::validated_data::Field<&'a str>);
}

#[::validated_data::data_view]
pub struct RandomDetect<'a, Mode> {
    pub ecn: ::validated_data::Field<random_detect::Ecn<'a, Mode>>,
}

pub mod random_detect {

    #[::validated_data::data_view]
    pub struct Ecn<'a, Mode> {
        pub allow_non_ect: ::validated_data::Field<ecn::AllowNonEct<'a, Mode>>,
        pub global_buffer: ::validated_data::Field<ecn::GlobalBuffer<'a, Mode>>,
    }

    pub mod ecn {

        #[::validated_data::data_view]
        pub struct AllowNonEct<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub chip_based: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct GlobalBuffer<'a, Mode> {
            pub units: ::validated_data::RequiredValue<&'a str, Mode>,
            pub min: ::validated_data::RequiredValue<i64, Mode>,
            pub max: ::validated_data::RequiredValue<i64, Mode>,
        }
    }
}

#[::validated_data::data_view]
pub struct TxQueue<'a, Mode> {
    pub shape_rate_percent_adaptive: ::validated_data::Field<bool>,
    pub queues: ::validated_data::Field<tx_queue::Queues<'a, Mode>>,
}

pub mod tx_queue {

    #[::validated_data::data_view(indexed_list, primary_key(id))]
    pub struct Queues<'a, Mode> (::validated_data::Field<queues::Item<'a, Mode>>);

    pub mod queues {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub id: ::validated_data::Field<i64>,
            pub scheduler_profile_responsive: ::validated_data::Field<bool>,
        }
    }
}
