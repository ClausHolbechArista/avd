// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct EdgePort {
        scalar bpdufilter_default("bpdufilter_default", 0) -> bool;
        scalar bpduguard_default("bpduguard_default", 1) -> bool;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct BpduguardRateLimit {
        scalar default("default", 0) -> bool;
        scalar count("count", 1) -> i64;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Mst {
        scalar pvst_border("pvst_border", 0) -> bool;
        model configuration("configuration", 1) -> mst::Configuration<'a>;
    }
}

pub mod mst {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Configuration {
            scalar name("name", 0) -> &'a str;
            scalar revision("revision", 1) -> i64;
            model instances("instances", 2) -> configuration::Instances<'a>;
        }
    }

    pub mod configuration {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Instances {
                model item (0) -> instances::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod instances {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar id("id", 0) -> i64;
                    scalar vlans("vlans", 1) -> &'a str;
                }
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MstInstances {
        model item (0) -> mst_instances::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod mst_instances {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar id("id", 0) -> &'a str;
            scalar priority("priority", 1) -> i64;
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RapidPvstInstances {
        model item (0) -> rapid_pvst_instances::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod rapid_pvst_instances {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar id("id", 0) -> &'a str;
            scalar priority("priority", 1) -> i64;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct PortIdAllocationPortChannelRange {
        scalar minimum("minimum", 0) -> i64;
        scalar maximum("maximum", 1) -> i64;
    }
}
