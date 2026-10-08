// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        model environment_variables("environment_variables", 1) -> item::EnvironmentVariables<'a>;
        scalar shutdown("shutdown", 2) -> bool;
        scalar shutdown_supervisor_active("shutdown_supervisor_active", 3) -> bool;
        scalar shutdown_supervisor_standby("shutdown_supervisor_standby", 4) -> bool;
    }
}

pub mod item {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct EnvironmentVariables {
            model item (0) -> environment_variables::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod environment_variables {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar value("value", 1) -> &'a str;
            }
        }
    }
}
