// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(list)]
pub struct PeerHosts<'a, Mode> (::validated_data::Field<&'a str>);

#[::validated_data::data_view]
pub struct Services<'a, Mode> {
    pub mcs: ::validated_data::Field<services::Mcs<'a, Mode>>,
    pub vxlan: ::validated_data::Field<services::Vxlan<'a, Mode>>,
    pub openstack: ::validated_data::Field<services::Openstack<'a, Mode>>,
}

pub mod services {

    #[::validated_data::data_view]
    pub struct Mcs<'a, Mode> {
        pub redis: ::validated_data::Field<mcs::Redis<'a, Mode>>,
        pub shutdown: ::validated_data::Field<bool>,
    }

    pub mod mcs {

        #[::validated_data::data_view]
        pub struct Redis<'a, Mode> {
            pub password: ::validated_data::Field<&'a str>,
            pub password_type: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view]
    pub struct Vxlan<'a, Mode> {
        pub shutdown: ::validated_data::Field<bool>,
        pub vtep_mac_learning: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct Openstack<'a, Mode> {
        pub authentication: ::validated_data::Field<openstack::Authentication<'a, Mode>>,
        pub grace_period: ::validated_data::Field<i64>,
        pub ip_access_group_name: ::validated_data::Field<&'a str>,
        pub ipv6_access_group_name: ::validated_data::Field<&'a str>,
        pub name_resolution: ::validated_data::Field<openstack::NameResolution<'a, Mode>>,
        pub network_type_driver: ::validated_data::Field<openstack::NetworkTypeDriver<'a, Mode>>,
        pub regions: ::validated_data::Field<openstack::Regions<'a, Mode>>,
        pub shutdown: ::validated_data::Field<bool>,
    }

    pub mod openstack {

        #[::validated_data::data_view]
        pub struct Authentication<'a, Mode> {
            pub role: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct NameResolution<'a, Mode> {
            pub force: ::validated_data::Field<bool>,
            pub interval: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view]
        pub struct NetworkTypeDriver<'a, Mode> {
            pub vlan: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct Regions<'a, Mode> (::validated_data::Field<regions::Item<'a, Mode>>);

        pub mod regions {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub username: ::validated_data::Field<&'a str>,
                pub password: ::validated_data::Field<&'a str>,
                pub password_type: ::validated_data::Field<&'a str>,
                pub tenant: ::validated_data::Field<&'a str>,
                pub keystone: ::validated_data::Field<item::Keystone<'a, Mode>>,
            }

            pub mod item {

                #[::validated_data::data_view]
                pub struct Keystone<'a, Mode> {
                    pub auth_url: ::validated_data::Field<&'a str>,
                }
            }
        }
    }
}
