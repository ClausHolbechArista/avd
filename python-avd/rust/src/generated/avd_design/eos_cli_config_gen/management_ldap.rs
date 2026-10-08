// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ServerDefaults {
        scalar base_dn("base_dn", 0) -> &'a str;
        scalar rdn_attribute_user("rdn_attribute_user", 1) -> &'a str;
        scalar ssl_profile("ssl_profile", 2) -> &'a str;
        scalar authorization_group_policy("authorization_group_policy", 3) -> &'a str;
        scalar timeout("timeout", 4) -> i64;
        model search("search", 5) -> server_defaults::Search<'a>;
    }
}

pub mod server_defaults {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Search {
            scalar username("username", 0) -> &'a str;
            scalar password("password", 1) -> &'a str;
            scalar password_type("password_type", 2) -> &'a str;
        }
    }
}

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ServerHosts {
        model item (0) -> server_hosts::Item<'a>;
    }
}

pub mod server_hosts {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar host("host", 0) -> &'a str;
            scalar port("port", 1) -> i64;
            scalar vrf("vrf", 2) -> &'a str;
            scalar base_dn("base_dn", 3) -> &'a str;
            scalar rdn_attribute_user("rdn_attribute_user", 4) -> &'a str;
            scalar ssl_profile("ssl_profile", 5) -> &'a str;
            scalar authorization_group_policy("authorization_group_policy", 6) -> &'a str;
            scalar timeout("timeout", 7) -> i64;
            model search("search", 8) -> item::Search<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Search {
                scalar username("username", 0) -> &'a str;
                scalar password("password", 1) -> &'a str;
                scalar password_type("password_type", 2) -> &'a str;
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct GroupPolicies {
        model item (0) -> group_policies::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod group_policies {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar policy("policy", 0) -> &'a str;
            model search_filter("search_filter", 1) -> item::SearchFilter<'a>;
            model groups("groups", 2) -> item::Groups<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct SearchFilter {
                scalar objectclass("objectclass", 0) -> &'a str;
                scalar attribute("attribute", 1) -> &'a str;
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Groups {
                model item (0) -> groups::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod groups {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    scalar role("role", 1) -> &'a str;
                    scalar privilege("privilege", 2) -> i64;
                }
            }
        }
    }
}
