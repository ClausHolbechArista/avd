// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct ManagementLdap<'a, Mode> {
    pub server_defaults: ::data_views::Field<ServerDefaults<'a, Mode>>,
    pub server_hosts: ::data_views::Field<ServerHosts<'a, Mode>>,
    pub group_policies: ::data_views::Field<GroupPolicies<'a, Mode>>,
}

#[::data_views::data_view]
pub struct ServerDefaults<'a, Mode> {
    pub base_dn: ::data_views::Field<&'a str>,
    pub rdn_attribute_user: ::data_views::Field<&'a str>,
    pub ssl_profile: ::data_views::Field<&'a str>,
    pub authorization_group_policy: ::data_views::Field<&'a str>,
    pub timeout: ::data_views::Field<i64>,
    pub search: ::data_views::Field<server_defaults::Search<'a, Mode>>,
}

pub mod server_defaults {

    #[::data_views::data_view]
    pub struct Search<'a, Mode> {
        pub username: ::data_views::RequiredValue<&'a str, Mode>,
        pub password: ::data_views::RequiredValue<&'a str, Mode>,
        pub password_type: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view(list)]
pub struct ServerHosts<'a, Mode> (::data_views::Field<server_hosts::Item<'a, Mode>>);

pub mod server_hosts {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub host: ::data_views::RequiredValue<&'a str, Mode>,
        pub port: ::data_views::Field<i64>,
        pub vrf: ::data_views::Field<&'a str>,
        pub base_dn: ::data_views::Field<&'a str>,
        pub rdn_attribute_user: ::data_views::Field<&'a str>,
        pub ssl_profile: ::data_views::Field<&'a str>,
        pub authorization_group_policy: ::data_views::Field<&'a str>,
        pub timeout: ::data_views::Field<i64>,
        pub search: ::data_views::Field<item::Search<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct Search<'a, Mode> {
            pub username: ::data_views::RequiredValue<&'a str, Mode>,
            pub password: ::data_views::RequiredValue<&'a str, Mode>,
            pub password_type: ::data_views::Field<&'a str>,
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(policy))]
pub struct GroupPolicies<'a, Mode> (::data_views::Field<group_policies::Item<'a, Mode>>);

pub mod group_policies {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub policy: ::data_views::Field<&'a str>,
        pub search_filter: ::data_views::Field<item::SearchFilter<'a, Mode>>,
        pub groups: ::data_views::Field<item::Groups<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct SearchFilter<'a, Mode> {
            pub objectclass: ::data_views::RequiredValue<&'a str, Mode>,
            pub attribute: ::data_views::RequiredValue<&'a str, Mode>,
        }

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct Groups<'a, Mode> (::data_views::Field<groups::Item<'a, Mode>>);

        pub mod groups {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
                pub role: ::data_views::RequiredValue<&'a str, Mode>,
                pub privilege: ::data_views::Field<i64>,
            }
        }
    }
}
