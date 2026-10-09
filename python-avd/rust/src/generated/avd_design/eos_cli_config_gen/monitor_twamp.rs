// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct TwampLight<'a, Mode> {
    pub reflector_defaults: ::validated_data::Field<twamp_light::ReflectorDefaults<'a, Mode>>,
    pub sender_defaults: ::validated_data::Field<twamp_light::SenderDefaults<'a, Mode>>,
    pub sender_profiles: ::validated_data::Field<twamp_light::SenderProfiles<'a, Mode>>,
}

pub mod twamp_light {

    #[::validated_data::data_view]
    pub struct ReflectorDefaults<'a, Mode> {
        pub listen_port: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct SenderDefaults<'a, Mode> {
        pub destination_port: ::validated_data::Field<i64>,
        pub source_port: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct SenderProfiles<'a, Mode> (::validated_data::Field<sender_profiles::Item<'a, Mode>>);

    pub mod sender_profiles {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub measurement_interval: ::validated_data::Field<i64>,
            pub measurement_samples: ::validated_data::Field<i64>,
            pub significance: ::validated_data::Field<item::Significance<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct Significance<'a, Mode> {
                pub value: ::validated_data::RequiredValue<i64, Mode>,
                pub offset: ::validated_data::RequiredValue<i64, Mode>,
            }
        }
    }
}
