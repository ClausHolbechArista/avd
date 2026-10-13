// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct ServerDefaults<'a, Mode> {
    pub base_dn: ::validated_data::Field<&'a str>,
    pub rdn_attribute_user: ::validated_data::Field<&'a str>,
    pub ssl_profile: ::validated_data::Field<&'a str>,
    pub authorization_group_policy: ::validated_data::Field<&'a str>,
    pub timeout: ::validated_data::Field<i64>,
    pub search: ::validated_data::Field<server_defaults::Search<'a, Mode>>,
}

pub mod server_defaults {

    #[::validated_data::data_view]
    pub struct Search<'a, Mode> {
        pub username: ::validated_data::RequiredValue<&'a str, Mode>,
        pub password: ::validated_data::RequiredValue<&'a str, Mode>,
        pub password_type: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(list)]
pub struct ServerHosts<'a, Mode> (::validated_data::Field<server_hosts::Item<'a, Mode>>);

pub mod server_hosts {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub host: ::validated_data::RequiredValue<&'a str, Mode>,
        pub port: ::validated_data::Field<i64>,
        pub vrf: ::validated_data::Field<&'a str>,
        pub base_dn: ::validated_data::Field<&'a str>,
        pub rdn_attribute_user: ::validated_data::Field<&'a str>,
        pub ssl_profile: ::validated_data::Field<&'a str>,
        pub authorization_group_policy: ::validated_data::Field<&'a str>,
        pub timeout: ::validated_data::Field<i64>,
        pub search: ::validated_data::Field<item::Search<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct Search<'a, Mode> {
            pub username: ::validated_data::RequiredValue<&'a str, Mode>,
            pub password: ::validated_data::RequiredValue<&'a str, Mode>,
            pub password_type: ::validated_data::Field<&'a str>,
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(policy))]
pub struct GroupPolicies<'a, Mode> (::validated_data::Field<group_policies::Item<'a, Mode>>);

pub mod group_policies {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub policy: ::validated_data::Field<&'a str>,
        pub search_filter: ::validated_data::Field<item::SearchFilter<'a, Mode>>,
        pub groups: ::validated_data::Field<item::Groups<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct SearchFilter<'a, Mode> {
            pub objectclass: ::validated_data::RequiredValue<&'a str, Mode>,
            pub attribute: ::validated_data::RequiredValue<&'a str, Mode>,
        }

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct Groups<'a, Mode> (::validated_data::Field<groups::Item<'a, Mode>>);

        pub mod groups {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub role: ::validated_data::RequiredValue<&'a str, Mode>,
                pub privilege: ::validated_data::Field<i64>,
            }
        }
    }
}
