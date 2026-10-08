// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Cvaas {
        scalar enabled("enabled", 0) -> bool;
        model clusters("clusters", 1) -> cvaas::Clusters<'a>;
    }
}

pub mod cvaas {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Clusters {
            model item (0) -> clusters::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod clusters {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar region("region", 1) -> &'a str;
                scalar vrf("vrf", 2) -> &'a str;
                scalar token_file("token_file", 3) -> &'a str;
                scalar source_interface("source_interface", 4) -> &'a str;
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct OnpremClusters {
        model item (0) -> onprem_clusters::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod onprem_clusters {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model servers("servers", 1) -> item::Servers<'a>;
            scalar vrf("vrf", 2) -> &'a str;
            scalar token_file("token_file", 3) -> &'a str;
            scalar source_interface("source_interface", 4) -> &'a str;
        }
    }

    pub mod item {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Servers {
                model item (0) -> servers::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod servers {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    scalar port("port", 1) -> i64;
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Terminattr {
        scalar ingestexclude("ingestexclude", 0) -> &'a str;
        scalar smashexcludes("smashexcludes", 1) -> &'a str;
        scalar disable_aaa("disable_aaa", 2) -> bool;
        model cvtargetconfigs("cvtargetconfigs", 3) -> terminattr::Cvtargetconfigs<'a>;
        scalar flowdns("flowdns", 4) -> bool;
        model custom_cv_options("custom_cv_options", 5) -> terminattr::CustomCvOptions<'a>;
    }
}

pub mod terminattr {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Cvtargetconfigs {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct CustomCvOptions {
            model item (0) -> custom_cv_options::Item<'a>;
        }
    }

    pub mod custom_cv_options {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar flag("flag", 0) -> &'a str;
                scalar value("value", 1) -> &'a str;
            }
        }
    }
}
