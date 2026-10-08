// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct PeerHosts {
        scalar item (0) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Services {
        model mcs("mcs", 0) -> services::Mcs<'a>;
        model vxlan("vxlan", 1) -> services::Vxlan<'a>;
        model openstack("openstack", 2) -> services::Openstack<'a>;
    }
}

pub mod services {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Mcs {
            model redis("redis", 0) -> mcs::Redis<'a>;
            scalar shutdown("shutdown", 1) -> bool;
        }
    }

    pub mod mcs {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Redis {
                scalar password("password", 0) -> &'a str;
                scalar password_type("password_type", 1) -> &'a str;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Vxlan {
            scalar shutdown("shutdown", 0) -> bool;
            scalar vtep_mac_learning("vtep_mac_learning", 1) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Openstack {
            model authentication("authentication", 0) -> openstack::Authentication<'a>;
            scalar grace_period("grace_period", 1) -> i64;
            scalar ip_access_group_name("ip_access_group_name", 2) -> &'a str;
            scalar ipv6_access_group_name("ipv6_access_group_name", 3) -> &'a str;
            model name_resolution("name_resolution", 4) -> openstack::NameResolution<'a>;
            model network_type_driver("network_type_driver", 5) -> openstack::NetworkTypeDriver<'a>;
            model regions("regions", 6) -> openstack::Regions<'a>;
            scalar shutdown("shutdown", 7) -> bool;
        }
    }

    pub mod openstack {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Authentication {
                scalar role("role", 0) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct NameResolution {
                scalar force("force", 0) -> bool;
                scalar interval("interval", 1) -> i64;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct NetworkTypeDriver {
                scalar vlan("vlan", 0) -> &'a str;
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Regions {
                model item (0) -> regions::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod regions {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    scalar username("username", 1) -> &'a str;
                    scalar password("password", 2) -> &'a str;
                    scalar password_type("password_type", 3) -> &'a str;
                    scalar tenant("tenant", 4) -> &'a str;
                    model keystone("keystone", 5) -> item::Keystone<'a>;
                }
            }

            pub mod item {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Keystone {
                        scalar auth_url("auth_url", 0) -> &'a str;
                    }
                }
            }
        }
    }
}
