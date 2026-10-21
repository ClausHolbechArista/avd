// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct MonitorTwamp<'a, Mode> {
    pub twamp_light: ::data_views::Field<TwampLight<'a, Mode>>,
}

#[::data_views::data_view]
pub struct TwampLight<'a, Mode> {
    pub reflector_defaults: ::data_views::Field<twamp_light::ReflectorDefaults<'a, Mode>>,
    pub sender_defaults: ::data_views::Field<twamp_light::SenderDefaults<'a, Mode>>,
    pub sender_profiles: ::data_views::Field<twamp_light::SenderProfiles<'a, Mode>>,
}

pub mod twamp_light {

    #[::data_views::data_view]
    pub struct ReflectorDefaults<'a, Mode> {
        pub listen_port: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct SenderDefaults<'a, Mode> {
        pub destination_port: ::data_views::Field<i64>,
        pub source_port: ::data_views::Field<i64>,
    }

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct SenderProfiles<'a, Mode> (::data_views::Field<sender_profiles::Item<'a, Mode>>);

    pub mod sender_profiles {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub measurement_interval: ::data_views::Field<i64>,
            pub measurement_samples: ::data_views::Field<i64>,
            pub significance: ::data_views::Field<item::Significance<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct Significance<'a, Mode> {
                pub value: ::data_views::RequiredValue<i64, Mode>,
                pub offset: ::data_views::RequiredValue<i64, Mode>,
            }
        }
    }
}
