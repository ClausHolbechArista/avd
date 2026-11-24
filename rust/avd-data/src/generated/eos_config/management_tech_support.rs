// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct ManagementTechSupport<'a, Mode> {
    pub policy_show_tech_support: ::data_views::Field<PolicyShowTechSupport<'a, Mode>>,
}

#[::data_views::data_view]
pub struct PolicyShowTechSupport<'a, Mode> {
    pub exclude_commands: ::data_views::Field<policy_show_tech_support::ExcludeCommands<'a, Mode>>,
    pub include_commands: ::data_views::Field<policy_show_tech_support::IncludeCommands<'a, Mode>>,
}

pub mod policy_show_tech_support {

    #[::data_views::data_view(list)]
    pub struct ExcludeCommands<'a, Mode> (::data_views::Field<exclude_commands::Item<'a, Mode>>);

    pub mod exclude_commands {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub command: ::data_views::Field<&'a str>,
            #[data_view(rename = "type")]
            pub field_type: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view(list)]
    pub struct IncludeCommands<'a, Mode> (::data_views::Field<include_commands::Item<'a, Mode>>);

    pub mod include_commands {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub command: ::data_views::Field<&'a str>,
        }
    }
}
