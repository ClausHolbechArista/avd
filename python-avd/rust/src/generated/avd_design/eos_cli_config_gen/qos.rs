// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Map {
        model cos("cos", 0) -> map::Cos<'a>;
        model dscp("dscp", 1) -> map::Dscp<'a>;
        model exp("exp", 2) -> map::Exp<'a>;
        model traffic_class("traffic_class", 3) -> map::TrafficClass<'a>;
    }
}

pub mod map {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Cos {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Dscp {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Exp {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct TrafficClass {
            scalar item (0) -> &'a str;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RandomDetect {
        model ecn("ecn", 0) -> random_detect::Ecn<'a>;
    }
}

pub mod random_detect {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ecn {
            model allow_non_ect("allow_non_ect", 0) -> ecn::AllowNonEct<'a>;
            model global_buffer("global_buffer", 1) -> ecn::GlobalBuffer<'a>;
        }
    }

    pub mod ecn {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AllowNonEct {
                scalar enabled("enabled", 0) -> bool;
                scalar chip_based("chip_based", 1) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct GlobalBuffer {
                scalar units("units", 0) -> &'a str;
                scalar min("min", 1) -> i64;
                scalar max("max", 2) -> i64;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct TxQueue {
        scalar shape_rate_percent_adaptive("shape_rate_percent_adaptive", 0) -> bool;
        model queues("queues", 1) -> tx_queue::Queues<'a>;
    }
}

pub mod tx_queue {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Queues {
            model item (0) -> queues::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod queues {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar id("id", 0) -> i64;
                scalar scheduler_profile_responsive("scheduler_profile_responsive", 1) -> bool;
            }
        }
    }
}
