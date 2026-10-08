// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Provider {
        model sysdb("sysdb", 0) -> provider::Sysdb<'a>;
        model smash("smash", 1) -> provider::Smash<'a>;
        model macsec("macsec", 2) -> provider::Macsec<'a>;
    }
}

pub mod provider {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Sysdb {
            model disabled_paths("disabled_paths", 0) -> sysdb::DisabledPaths<'a>;
        }
    }

    pub mod sysdb {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct DisabledPaths {
                scalar item (0) -> &'a str;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Smash {
            model paths("paths", 0) -> smash::Paths<'a>;
        }
    }

    pub mod smash {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Paths {
                model item (0) -> paths::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod paths {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar path("path", 0) -> &'a str;
                    scalar disabled("disabled", 1) -> bool;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Macsec {
            scalar interfaces("interfaces", 0) -> bool;
            scalar mka("mka", 1) -> bool;
        }
    }
}
