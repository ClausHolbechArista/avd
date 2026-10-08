// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Connector {
        model interface("interface", 0) -> connector::Interface<'a>;
    }
}

pub mod connector {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Interface {
            model patch("patch", 0) -> interface::Patch<'a>;
            model recovery("recovery", 1) -> interface::Recovery<'a>;
        }
    }

    pub mod interface {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Patch {
                scalar bgp_vpws_remote_failure_errdisable("bgp_vpws_remote_failure_errdisable", 0) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Recovery {
                model review_delay("review_delay", 0) -> recovery::ReviewDelay<'a>;
            }
        }

        pub mod recovery {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct ReviewDelay {
                    scalar min("min", 0) -> i64;
                    scalar max("max", 1) -> i64;
                }
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Patches {
        model item (0) -> patches::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod patches {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar enabled("enabled", 1) -> bool;
            model connectors("connectors", 2) -> item::Connectors<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Connectors {
                model item (0) -> connectors::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod connectors {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar id("id", 0) -> &'a str;
                    scalar field_type("type", 1) -> &'a str;
                    scalar endpoint("endpoint", 2) -> &'a str;
                }
            }
        }
    }
}
