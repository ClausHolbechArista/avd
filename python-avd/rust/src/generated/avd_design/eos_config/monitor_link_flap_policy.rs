// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct DampingProfiles<'a, Mode> (::validated_data::Field<damping_profiles::Item<'a, Mode>>);

pub mod damping_profiles {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub penalty_decay: ::validated_data::Field<item::PenaltyDecay<'a, Mode>>,
        pub mac_fault_local_penalty: ::validated_data::Field<i64>,
        pub mac_fault_remote_penalty: ::validated_data::Field<i64>,
        pub penalty_threshold: ::validated_data::Field<item::PenaltyThreshold<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct PenaltyDecay<'a, Mode> {
            pub half_life: ::validated_data::RequiredValue<i64, Mode>,
            pub units: ::validated_data::RequiredValue<&'a str, Mode>,
        }

        #[::validated_data::data_view]
        pub struct PenaltyThreshold<'a, Mode> {
            pub maximum: ::validated_data::Field<i64>,
            pub reuse: ::validated_data::Field<i64>,
            pub suppression: ::validated_data::Field<i64>,
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct MaxFlapProfiles<'a, Mode> (::validated_data::Field<max_flap_profiles::Item<'a, Mode>>);

pub mod max_flap_profiles {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub max_flaps: ::validated_data::RequiredValue<i64, Mode>,
        pub time: ::validated_data::RequiredValue<i64, Mode>,
        pub violations: ::validated_data::Field<i64>,
        pub intervals: ::validated_data::Field<i64>,
    }
}

#[::validated_data::data_view(list)]
pub struct DefaultProfiles<'a, Mode> (::validated_data::Field<&'a str>);
