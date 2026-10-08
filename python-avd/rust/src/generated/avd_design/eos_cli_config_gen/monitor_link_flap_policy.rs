// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DampingProfiles {
        model item (0) -> damping_profiles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod damping_profiles {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model penalty_decay("penalty_decay", 1) -> item::PenaltyDecay<'a>;
            scalar mac_fault_local_penalty("mac_fault_local_penalty", 2) -> i64;
            scalar mac_fault_remote_penalty("mac_fault_remote_penalty", 3) -> i64;
            model penalty_threshold("penalty_threshold", 4) -> item::PenaltyThreshold<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct PenaltyDecay {
                scalar half_life("half_life", 0) -> i64;
                scalar units("units", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct PenaltyThreshold {
                scalar maximum("maximum", 0) -> i64;
                scalar reuse("reuse", 1) -> i64;
                scalar suppression("suppression", 2) -> i64;
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MaxFlapProfiles {
        model item (0) -> max_flap_profiles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod max_flap_profiles {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar max_flaps("max_flaps", 1) -> i64;
            scalar time("time", 2) -> i64;
            scalar violations("violations", 3) -> i64;
            scalar intervals("intervals", 4) -> i64;
        }
    }
}

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DefaultProfiles {
        scalar item (0) -> &'a str;
    }
}
