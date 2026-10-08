// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        model sources("sources", 1) -> item::Sources<'a>;
        model destinations("destinations", 2) -> item::Destinations<'a>;
        scalar encapsulation_gre_metadata_tx("encapsulation_gre_metadata_tx", 3) -> bool;
        scalar header_remove_size("header_remove_size", 4) -> i64;
        model access_group("access_group", 5) -> item::AccessGroup<'a>;
        scalar rate_limit_per_ingress_chip("rate_limit_per_ingress_chip", 6) -> &'a str;
        scalar rate_limit_per_egress_chip("rate_limit_per_egress_chip", 7) -> &'a str;
        scalar sample("sample", 8) -> i64;
        model truncate("truncate", 9) -> item::Truncate<'a>;
    }
}

pub mod item {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Sources {
            model item (0) -> sources::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod sources {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar direction("direction", 1) -> &'a str;
                model access_group("access_group", 2) -> item::AccessGroup<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct AccessGroup {
                    scalar field_type("type", 0) -> &'a str;
                    scalar name("name", 1) -> &'a str;
                    scalar priority("priority", 2) -> i64;
                }
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Destinations {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct AccessGroup {
            scalar field_type("type", 0) -> &'a str;
            scalar name("name", 1) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Truncate {
            scalar enabled("enabled", 0) -> bool;
            scalar size("size", 1) -> i64;
        }
    }
}
