// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct PolicyShowTechSupport {
        model exclude_commands("exclude_commands", 0) -> policy_show_tech_support::ExcludeCommands<'a>;
        model include_commands("include_commands", 1) -> policy_show_tech_support::IncludeCommands<'a>;
    }
}

pub mod policy_show_tech_support {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct ExcludeCommands {
            model item (0) -> exclude_commands::Item<'a>;
        }
    }

    pub mod exclude_commands {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar command("command", 0) -> &'a str;
                scalar field_type("type", 1) -> &'a str;
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct IncludeCommands {
            model item (0) -> include_commands::Item<'a>;
        }
    }

    pub mod include_commands {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar command("command", 0) -> &'a str;
            }
        }
    }
}
