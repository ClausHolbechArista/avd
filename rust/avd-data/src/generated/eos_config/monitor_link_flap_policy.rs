// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct MonitorLinkFlapPolicy<'a, Mode> {
    pub damping_profiles: ::data_views::Field<DampingProfiles<'a, Mode>>,
    pub max_flap_profiles: ::data_views::Field<MaxFlapProfiles<'a, Mode>>,
    pub default_profiles: ::data_views::Field<DefaultProfiles<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct DampingProfiles<'a, Mode> (::data_views::Field<damping_profiles::Item<'a, Mode>>);

pub mod damping_profiles {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub penalty_decay: ::data_views::Field<item::PenaltyDecay<'a, Mode>>,
        pub mac_fault_local_penalty: ::data_views::Field<i64>,
        pub mac_fault_remote_penalty: ::data_views::Field<i64>,
        pub penalty_threshold: ::data_views::Field<item::PenaltyThreshold<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct PenaltyDecay<'a, Mode> {
            pub half_life: ::data_views::RequiredValue<i64, Mode>,
            pub units: ::data_views::RequiredValue<&'a str, Mode>,
        }

        #[::data_views::data_view]
        pub struct PenaltyThreshold<'a, Mode> {
            pub maximum: ::data_views::Field<i64>,
            pub reuse: ::data_views::Field<i64>,
            pub suppression: ::data_views::Field<i64>,
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct MaxFlapProfiles<'a, Mode> (::data_views::Field<max_flap_profiles::Item<'a, Mode>>);

pub mod max_flap_profiles {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub max_flaps: ::data_views::RequiredValue<i64, Mode>,
        pub time: ::data_views::RequiredValue<i64, Mode>,
        pub violations: ::data_views::Field<i64>,
        pub intervals: ::data_views::Field<i64>,
    }
}

#[::data_views::data_view(list)]
pub struct DefaultProfiles<'a, Mode> (::data_views::Field<&'a str>);
