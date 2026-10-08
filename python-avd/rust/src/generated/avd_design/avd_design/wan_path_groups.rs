// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar id("id", 1) -> i64;
        scalar description("description", 2) -> &'a str;
        model ipsec("ipsec", 3) -> item::Ipsec<'a>;
        model import_path_groups("import_path_groups", 4) -> item::ImportPathGroups<'a>;
        scalar default_preference("default_preference", 5) -> &'a str;
        scalar excluded_from_default_policy("excluded_from_default_policy", 6) -> bool;
        model dps_keepalive("dps_keepalive", 7) -> item::DpsKeepalive<'a>;
    }
}

pub mod item {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipsec {
            scalar dynamic_peers("dynamic_peers", 0) -> bool;
            scalar static_peers("static_peers", 1) -> bool;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct ImportPathGroups {
            model item (0) -> import_path_groups::Item<'a>;
        }
    }

    pub mod import_path_groups {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar remote("remote", 0) -> &'a str;
                scalar local("local", 1) -> &'a str;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct DpsKeepalive {
            scalar interval("interval", 0) -> &'a str;
            scalar failure_threshold("failure_threshold", 1) -> i64;
        }
    }
}
