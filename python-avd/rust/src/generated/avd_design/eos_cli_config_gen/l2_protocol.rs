// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ForwardingProfiles {
        model item (0) -> forwarding_profiles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod forwarding_profiles {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model protocols("protocols", 1) -> item::Protocols<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Protocols {
                model item (0) -> protocols::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod protocols {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    scalar forward("forward", 1) -> bool;
                    scalar tagged_forward("tagged_forward", 2) -> bool;
                    scalar untagged_forward("untagged_forward", 3) -> bool;
                }
            }
        }
    }
}
