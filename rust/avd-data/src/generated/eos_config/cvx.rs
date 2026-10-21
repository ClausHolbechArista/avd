// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Cvx<'a, Mode> {
    pub shutdown: ::data_views::Field<bool>,
    pub peer_hosts: ::data_views::Field<PeerHosts<'a, Mode>>,
    pub services: ::data_views::Field<Services<'a, Mode>>,
}

#[::data_views::data_view(list)]
pub struct PeerHosts<'a, Mode> (::data_views::Field<&'a str>);

#[::data_views::data_view]
pub struct Services<'a, Mode> {
    pub mcs: ::data_views::Field<services::Mcs<'a, Mode>>,
    pub vxlan: ::data_views::Field<services::Vxlan<'a, Mode>>,
    pub openstack: ::data_views::Field<services::Openstack<'a, Mode>>,
}

pub mod services {

    #[::data_views::data_view]
    pub struct Mcs<'a, Mode> {
        pub redis: ::data_views::Field<mcs::Redis<'a, Mode>>,
        pub shutdown: ::data_views::Field<bool>,
    }

    pub mod mcs {

        #[::data_views::data_view]
        pub struct Redis<'a, Mode> {
            pub password: ::data_views::Field<&'a str>,
            pub password_type: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view]
    pub struct Vxlan<'a, Mode> {
        pub shutdown: ::data_views::Field<bool>,
        pub vtep_mac_learning: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view]
    pub struct Openstack<'a, Mode> {
        pub authentication: ::data_views::Field<openstack::Authentication<'a, Mode>>,
        pub grace_period: ::data_views::Field<i64>,
        pub ip_access_group_name: ::data_views::Field<&'a str>,
        pub ipv6_access_group_name: ::data_views::Field<&'a str>,
        pub name_resolution: ::data_views::Field<openstack::NameResolution<'a, Mode>>,
        pub network_type_driver: ::data_views::Field<openstack::NetworkTypeDriver<'a, Mode>>,
        pub regions: ::data_views::Field<openstack::Regions<'a, Mode>>,
        pub shutdown: ::data_views::Field<bool>,
    }

    pub mod openstack {

        #[::data_views::data_view]
        pub struct Authentication<'a, Mode> {
            pub role: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct NameResolution<'a, Mode> {
            pub force: ::data_views::Field<bool>,
            pub interval: ::data_views::Field<i64>,
        }

        #[::data_views::data_view]
        pub struct NetworkTypeDriver<'a, Mode> {
            pub vlan: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct Regions<'a, Mode> (::data_views::Field<regions::Item<'a, Mode>>);

        pub mod regions {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
                pub username: ::data_views::Field<&'a str>,
                pub password: ::data_views::Field<&'a str>,
                pub password_type: ::data_views::Field<&'a str>,
                pub tenant: ::data_views::Field<&'a str>,
                pub keystone: ::data_views::Field<item::Keystone<'a, Mode>>,
            }

            pub mod item {

                #[::data_views::data_view]
                pub struct Keystone<'a, Mode> {
                    pub auth_url: ::data_views::Field<&'a str>,
                }
            }
        }
    }
}
