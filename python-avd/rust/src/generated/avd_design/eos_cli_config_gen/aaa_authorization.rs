// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Policy {
        scalar local_default_role("local_default_role", 0) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Exec {
        scalar default("default", 0) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Dynamic {
        model dot1x_additional_groups("dot1x_additional_groups", 0) -> dynamic::Dot1xAdditionalGroups<'a>;
    }
}

pub mod dynamic {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Dot1xAdditionalGroups {
            scalar item (0) -> &'a str;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Commands {
        scalar all_default("all_default", 0) -> &'a str;
        model privilege("privilege", 1) -> commands::Privilege<'a>;
    }
}

pub mod commands {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Privilege {
            model item (0) -> privilege::Item<'a>;
        }
    }

    pub mod privilege {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar level("level", 0) -> &'a str;
                scalar default("default", 1) -> &'a str;
            }
        }
    }
}
