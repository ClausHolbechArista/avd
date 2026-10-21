// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Agents<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view]
pub struct Item<'a, Mode> {
    pub name: ::data_views::Field<&'a str>,
    pub environment_variables: ::data_views::Field<item::EnvironmentVariables<'a, Mode>>,
    pub shutdown: ::data_views::Field<bool>,
    pub shutdown_supervisor_active: ::data_views::Field<bool>,
    pub shutdown_supervisor_standby: ::data_views::Field<bool>,
}

pub mod item {

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct EnvironmentVariables<'a, Mode> (::data_views::Field<environment_variables::Item<'a, Mode>>);

    pub mod environment_variables {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub value: ::data_views::RequiredValue<&'a str, Mode>,
        }
    }
}
