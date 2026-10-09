// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct EngineIds<'a, Mode> {
    pub local: ::validated_data::Field<&'a str>,
    pub remotes: ::validated_data::Field<engine_ids::Remotes<'a, Mode>>,
}

pub mod engine_ids {

    #[::validated_data::data_view(list)]
    pub struct Remotes<'a, Mode> (::validated_data::Field<remotes::Item<'a, Mode>>);

    pub mod remotes {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub id: ::validated_data::Field<&'a str>,
            pub address: ::validated_data::Field<&'a str>,
            pub udp_port: ::validated_data::Field<i64>,
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(oid))]
pub struct Extensions<'a, Mode> (::validated_data::Field<extensions::Item<'a, Mode>>);

pub mod extensions {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub oid: ::validated_data::Field<&'a str>,
        pub path: ::validated_data::RequiredValue<&'a str, Mode>,
        pub one_shot: ::validated_data::Field<bool>,
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
pub struct Ipv4Acls<'a, Mode> (::validated_data::Field<ipv4_acls::Item<'a, Mode>>);

pub mod ipv4_acls {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub vrf: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(list)]
pub struct Ipv6Acls<'a, Mode> (::validated_data::Field<ipv6_acls::Item<'a, Mode>>);

pub mod ipv6_acls {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub vrf: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct LocalInterfaces<'a, Mode> (::validated_data::Field<local_interfaces::Item<'a, Mode>>);

pub mod local_interfaces {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub vrf: ::validated_data::Field<&'a str>,
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

#[::validated_data::data_view(list)]
pub struct Users<'a, Mode> (::validated_data::Field<users::Item<'a, Mode>>);

pub mod users {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub group: ::validated_data::Field<&'a str>,
        pub remote_address: ::validated_data::Field<&'a str>,
        pub udp_port: ::validated_data::Field<i64>,
        pub version: ::validated_data::Field<&'a str>,
        pub localized: ::validated_data::Field<&'a str>,
        pub auth: ::validated_data::Field<&'a str>,
        pub auth_key_type: ::validated_data::Field<&'a str>,
        pub auth_key: ::validated_data::Field<&'a str>,
        pub auth_passphrase: ::validated_data::Field<&'a str>,
        #[data_view(rename = "priv")]
        pub field_priv: ::validated_data::Field<&'a str>,
        pub priv_key_type: ::validated_data::Field<&'a str>,
        pub priv_key: ::validated_data::Field<&'a str>,
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

#[::validated_data::data_view]
pub struct Traps<'a, Mode> {
    pub enable: ::validated_data::Field<bool>,
    pub snmp_traps: ::validated_data::Field<traps::SnmpTraps<'a, Mode>>,
}

pub mod traps {

    #[::validated_data::data_view(list)]
    pub struct SnmpTraps<'a, Mode> (::validated_data::Field<snmp_traps::Item<'a, Mode>>);

    pub mod snmp_traps {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub enabled: ::validated_data::Field<bool>,
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub enable: ::validated_data::Field<bool>,
    }
}
