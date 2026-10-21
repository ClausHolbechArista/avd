// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct SnmpServer<'a, Mode> {
    pub engine_ids: ::data_views::Field<EngineIds<'a, Mode>>,
    pub extensions: ::data_views::Field<Extensions<'a, Mode>>,
    pub contact: ::data_views::Field<&'a str>,
    pub location: ::data_views::Field<&'a str>,
    pub communities: ::data_views::Field<Communities<'a, Mode>>,
    pub ipv4_acls: ::data_views::Field<Ipv4Acls<'a, Mode>>,
    pub ipv6_acls: ::data_views::Field<Ipv6Acls<'a, Mode>>,
    pub local_interfaces: ::data_views::Field<LocalInterfaces<'a, Mode>>,
    pub views: ::data_views::Field<Views<'a, Mode>>,
    pub groups: ::data_views::Field<Groups<'a, Mode>>,
    pub users: ::data_views::Field<Users<'a, Mode>>,
    pub hosts: ::data_views::Field<Hosts<'a, Mode>>,
    pub traps: ::data_views::Field<Traps<'a, Mode>>,
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
    pub ifmib_ifspeed_shape_rate: ::data_views::Field<bool>,
}

#[::data_views::data_view]
pub struct EngineIds<'a, Mode> {
    pub local: ::data_views::Field<&'a str>,
    pub remotes: ::data_views::Field<engine_ids::Remotes<'a, Mode>>,
}

pub mod engine_ids {

    #[::data_views::data_view(list)]
    pub struct Remotes<'a, Mode> (::data_views::Field<remotes::Item<'a, Mode>>);

    pub mod remotes {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub id: ::data_views::Field<&'a str>,
            pub address: ::data_views::Field<&'a str>,
            pub udp_port: ::data_views::Field<i64>,
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(oid))]
pub struct Extensions<'a, Mode> (::data_views::Field<extensions::Item<'a, Mode>>);

pub mod extensions {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub oid: ::data_views::Field<&'a str>,
        pub path: ::data_views::RequiredValue<&'a str, Mode>,
        pub one_shot: ::data_views::Field<bool>,
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Communities<'a, Mode> (::data_views::Field<communities::Item<'a, Mode>>);

pub mod communities {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub access: ::data_views::Field<&'a str>,
        pub access_list_ipv4: ::data_views::Field<item::AccessListIpv4<'a, Mode>>,
        pub access_list_ipv6: ::data_views::Field<item::AccessListIpv6<'a, Mode>>,
        pub view: ::data_views::Field<&'a str>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct AccessListIpv4<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct AccessListIpv6<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
        }
    }
}

#[::data_views::data_view(list)]
pub struct Ipv4Acls<'a, Mode> (::data_views::Field<ipv4_acls::Item<'a, Mode>>);

pub mod ipv4_acls {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub vrf: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view(list)]
pub struct Ipv6Acls<'a, Mode> (::data_views::Field<ipv6_acls::Item<'a, Mode>>);

pub mod ipv6_acls {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub vrf: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct LocalInterfaces<'a, Mode> (::data_views::Field<local_interfaces::Item<'a, Mode>>);

pub mod local_interfaces {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub vrf: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view(list)]
pub struct Views<'a, Mode> (::data_views::Field<views::Item<'a, Mode>>);

pub mod views {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub mib_family_name: ::data_views::Field<&'a str>,
        pub included: ::data_views::Field<bool>,
    }
}

#[::data_views::data_view(list)]
pub struct Groups<'a, Mode> (::data_views::Field<groups::Item<'a, Mode>>);

pub mod groups {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub version: ::data_views::Field<&'a str>,
        pub authentication: ::data_views::Field<&'a str>,
        pub read: ::data_views::Field<&'a str>,
        pub write: ::data_views::Field<&'a str>,
        pub notify: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view(list)]
pub struct Users<'a, Mode> (::data_views::Field<users::Item<'a, Mode>>);

pub mod users {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub group: ::data_views::Field<&'a str>,
        pub remote_address: ::data_views::Field<&'a str>,
        pub udp_port: ::data_views::Field<i64>,
        pub version: ::data_views::Field<&'a str>,
        pub localized: ::data_views::Field<&'a str>,
        pub auth: ::data_views::Field<&'a str>,
        pub auth_key_type: ::data_views::Field<&'a str>,
        pub auth_key: ::data_views::Field<&'a str>,
        pub auth_passphrase: ::data_views::Field<&'a str>,
        #[data_view(rename = "priv")]
        pub field_priv: ::data_views::Field<&'a str>,
        pub priv_key_type: ::data_views::Field<&'a str>,
        pub priv_key: ::data_views::Field<&'a str>,
        pub priv_passphrase: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view(list)]
pub struct Hosts<'a, Mode> (::data_views::Field<hosts::Item<'a, Mode>>);

pub mod hosts {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub host: ::data_views::Field<&'a str>,
        pub vrf: ::data_views::Field<&'a str>,
        pub version: ::data_views::Field<&'a str>,
        pub community: ::data_views::Field<&'a str>,
        pub users: ::data_views::Field<item::Users<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(list)]
        pub struct Users<'a, Mode> (::data_views::Field<users::Item<'a, Mode>>);

        pub mod users {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub username: ::data_views::Field<&'a str>,
                pub authentication_level: ::data_views::Field<&'a str>,
            }
        }
    }
}

#[::data_views::data_view(python(name = "EOSConfig.SnmpServer.Traps", schema = Dict(1243)))]
pub struct Traps<'a, Mode> {
    #[data_view(schema = Bool(368))]
    pub enable: ::data_views::Field<bool>,
    #[data_view(schema = List(496))]
    pub snmp_traps: ::data_views::Field<traps::SnmpTraps<'a, Mode>>,
}

pub mod traps {

    #[::data_views::data_view(list, python(name = "EOSConfig.SnmpServer.Traps.SnmpTraps", schema = List(496)))]
    pub struct SnmpTraps<'a, Mode> (::data_views::Field<snmp_traps::Item<'a, Mode>>);

    pub mod snmp_traps {

        #[::data_views::data_view(python(name = "EOSConfig.SnmpServer.Traps.SnmpTrapsItem", schema = Dict(1242)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(973))]
            pub name: ::data_views::Field<&'a str>,
            #[data_view(schema = Bool(369))]
            pub enabled: ::data_views::Field<bool>,
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub enable: ::data_views::Field<bool>,
    }
}
