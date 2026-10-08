// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Region {
        scalar name("name", 0) -> &'a str;
        scalar id("id", 1) -> i64;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Zone {
        scalar name("name", 0) -> &'a str;
        scalar id("id", 1) -> i64;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Site {
        scalar name("name", 0) -> &'a str;
        scalar id("id", 1) -> i64;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Profiles {
        model item (0) -> profiles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod profiles {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar load_balance_policy("load_balance_policy", 1) -> &'a str;
            scalar internet_exit_policy("internet_exit_policy", 2) -> &'a str;
            model metric_order("metric_order", 3) -> item::MetricOrder<'a>;
            model outlier_elimination("outlier_elimination", 4) -> item::OutlierElimination<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct MetricOrder {
                scalar preferred_metric("preferred_metric", 0) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct OutlierElimination {
                scalar disabled("disabled", 0) -> bool;
                model threshold("threshold", 1) -> outlier_elimination::Threshold<'a>;
            }
        }

        pub mod outlier_elimination {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Threshold {
                    scalar jitter("jitter", 0) -> i64;
                    scalar latency("latency", 1) -> i64;
                    scalar load("load", 2) -> &'a str;
                    scalar loss_rate("loss_rate", 3) -> &'a str;
                }
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Policies {
        model item (0) -> policies::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod policies {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model matches("matches", 1) -> item::Matches<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Matches {
                model item (0) -> matches::Item<'a>;
            }
        }

        pub mod matches {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar application_profile("application_profile", 0) -> &'a str;
                    scalar avt_profile("avt_profile", 1) -> &'a str;
                    scalar dscp("dscp", 2) -> i64;
                    scalar traffic_class("traffic_class", 3) -> i64;
                }
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Vrfs {
        model item (0) -> vrfs::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod vrfs {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar policy("policy", 1) -> &'a str;
            model profiles("profiles", 2) -> item::Profiles<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Profiles {
                model item (0) -> profiles::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod profiles {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    scalar id("id", 1) -> i64;
                }
            }
        }
    }
}
