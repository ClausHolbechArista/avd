// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct PolicyShowTechSupport<'a, Mode> {
    pub exclude_commands: ::validated_data::Field<policy_show_tech_support::ExcludeCommands<'a, Mode>>,
    pub include_commands: ::validated_data::Field<policy_show_tech_support::IncludeCommands<'a, Mode>>,
}

pub mod policy_show_tech_support {

    #[::validated_data::data_view(list)]
    pub struct ExcludeCommands<'a, Mode> (::validated_data::Field<exclude_commands::Item<'a, Mode>>);

    pub mod exclude_commands {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub command: ::validated_data::Field<&'a str>,
            #[data_view(rename = "type")]
            pub field_type: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(list)]
    pub struct IncludeCommands<'a, Mode> (::validated_data::Field<include_commands::Item<'a, Mode>>);

    pub mod include_commands {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub command: ::validated_data::Field<&'a str>,
        }
    }
}
