// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub environment_variables: ::validated_data::Field<item::EnvironmentVariables<'a, Mode>>,
    pub shutdown: ::validated_data::Field<bool>,
    pub shutdown_supervisor_active: ::validated_data::Field<bool>,
    pub shutdown_supervisor_standby: ::validated_data::Field<bool>,
}

pub mod item {

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct EnvironmentVariables<'a, Mode> (::validated_data::Field<environment_variables::Item<'a, Mode>>);

    pub mod environment_variables {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub value: ::validated_data::RequiredValue<&'a str, Mode>,
        }
    }
}
