// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub enable: ::validated_data::Field<bool>,
        pub source_interface: ::validated_data::Field<&'a str>,
        pub ipv4_acl: ::validated_data::Field<&'a str>,
        pub ipv6_acl: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(list)]
pub struct Users<'a, Mode> (::validated_data::Field<users::Item<'a, Mode>>);

pub mod users {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub group: ::validated_data::Field<&'a str>,
        pub version: ::validated_data::Field<&'a str>,
        pub auth: ::validated_data::Field<&'a str>,
        pub auth_passphrase: ::validated_data::Field<&'a str>,
        #[data_view(rename = "priv")]
        pub field_priv: ::validated_data::Field<&'a str>,
        pub priv_passphrase: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(list)]
pub struct Hosts<'a, Mode> (::validated_data::Field<hosts::Item<'a, Mode>>);

pub mod hosts {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub host: ::validated_data::Field<&'a str>,
        pub vrf: ::validated_data::Field<&'a str>,
        pub version: ::validated_data::Field<&'a str>,
        pub community: ::validated_data::Field<&'a str>,
        pub users: ::validated_data::Field<item::Users<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(list)]
        pub struct Users<'a, Mode> (::validated_data::Field<users::Item<'a, Mode>>);

        pub mod users {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub username: ::validated_data::Field<&'a str>,
                pub authentication_level: ::validated_data::Field<&'a str>,
            }
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Communities<'a, Mode> (::validated_data::Field<communities::Item<'a, Mode>>);

pub mod communities {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub access: ::validated_data::Field<&'a str>,
        pub access_list_ipv4: ::validated_data::Field<item::AccessListIpv4<'a, Mode>>,
        pub ipv4_standard_acl: ::validated_data::Field<&'a str>,
        pub access_list_ipv6: ::validated_data::Field<item::AccessListIpv6<'a, Mode>>,
        pub view: ::validated_data::Field<&'a str>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct AccessListIpv4<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct AccessListIpv6<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
        }
    }
}

#[::validated_data::data_view(list)]
pub struct Views<'a, Mode> (::validated_data::Field<views::Item<'a, Mode>>);

pub mod views {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub mib_family_name: ::validated_data::Field<&'a str>,
        pub included: ::validated_data::Field<bool>,
    }
}

#[::validated_data::data_view(list)]
pub struct Groups<'a, Mode> (::validated_data::Field<groups::Item<'a, Mode>>);

pub mod groups {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub version: ::validated_data::Field<&'a str>,
        pub authentication: ::validated_data::Field<&'a str>,
        pub read: ::validated_data::Field<&'a str>,
        pub write: ::validated_data::Field<&'a str>,
        pub notify: ::validated_data::Field<&'a str>,
    }
}
