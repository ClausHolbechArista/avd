// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct TwampLight {
        model reflector_defaults("reflector_defaults", 0) -> twamp_light::ReflectorDefaults<'a>;
        model sender_defaults("sender_defaults", 1) -> twamp_light::SenderDefaults<'a>;
        model sender_profiles("sender_profiles", 2) -> twamp_light::SenderProfiles<'a>;
    }
}

pub mod twamp_light {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct ReflectorDefaults {
            scalar listen_port("listen_port", 0) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct SenderDefaults {
            scalar destination_port("destination_port", 0) -> i64;
            scalar source_port("source_port", 1) -> i64;
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct SenderProfiles {
            model item (0) -> sender_profiles::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod sender_profiles {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar measurement_interval("measurement_interval", 1) -> i64;
                scalar measurement_samples("measurement_samples", 2) -> i64;
                model significance("significance", 3) -> item::Significance<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Significance {
                    scalar value("value", 0) -> i64;
                    scalar offset("offset", 1) -> i64;
                }
            }
        }
    }
}
