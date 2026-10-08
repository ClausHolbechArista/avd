// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IkePolicies {
        model item (0) -> ike_policies::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod ike_policies {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar local_id("local_id", 1) -> &'a str;
            scalar local_id_fqdn("local_id_fqdn", 2) -> &'a str;
            scalar ike_lifetime("ike_lifetime", 3) -> i64;
            scalar encryption("encryption", 4) -> &'a str;
            scalar dh_group("dh_group", 5) -> i64;
            scalar integrity("integrity", 6) -> &'a str;
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SaPolicies {
        model item (0) -> sa_policies::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod sa_policies {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model sa_lifetime("sa_lifetime", 1) -> item::SaLifetime<'a>;
            model esp("esp", 2) -> item::Esp<'a>;
            scalar pfs_dh_group("pfs_dh_group", 3) -> i64;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct SaLifetime {
                scalar value("value", 0) -> i64;
                scalar unit("unit", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Esp {
                scalar integrity("integrity", 0) -> &'a str;
                scalar encryption("encryption", 1) -> &'a str;
            }
        }
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
            scalar ike_policy("ike_policy", 1) -> &'a str;
            scalar sa_policy("sa_policy", 2) -> &'a str;
            scalar connection("connection", 3) -> &'a str;
            scalar shared_key("shared_key", 4) -> &'a str;
            model dpd("dpd", 5) -> item::Dpd<'a>;
            scalar mode("mode", 6) -> &'a str;
            scalar flow_parallelization_encapsulation_udp("flow_parallelization_encapsulation_udp", 7) -> bool;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Dpd {
                scalar interval("interval", 0) -> i64;
                scalar time("time", 1) -> i64;
                scalar action("action", 2) -> &'a str;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct KeyController {
        scalar profile("profile", 0) -> &'a str;
    }
}
