// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct RouterMsdp<'a, Mode> {
    pub originator_id_local_interface: ::data_views::Field<&'a str>,
    pub rejected_limit: ::data_views::Field<i64>,
    pub forward_register_packets: ::data_views::Field<bool>,
    pub connection_retry_interval: ::data_views::Field<i64>,
    pub group_limits: ::data_views::Field<GroupLimits<'a, Mode>>,
    pub peers: ::data_views::Field<Peers<'a, Mode>>,
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(source_prefix))]
pub struct GroupLimits<'a, Mode> (::data_views::Field<group_limits::Item<'a, Mode>>);

pub mod group_limits {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub source_prefix: ::data_views::Field<&'a str>,
        pub limit: ::data_views::RequiredValue<i64, Mode>,
    }
}

#[::data_views::data_view(indexed_list, primary_key(ipv4_address))]
pub struct Peers<'a, Mode> (::data_views::Field<peers::Item<'a, Mode>>);

pub mod peers {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub ipv4_address: ::data_views::Field<&'a str>,
        pub default_peer: ::data_views::Field<item::DefaultPeer<'a, Mode>>,
        pub local_interface: ::data_views::Field<&'a str>,
        pub description: ::data_views::Field<&'a str>,
        pub disabled: ::data_views::Field<bool>,
        pub sa_limit: ::data_views::Field<i64>,
        pub mesh_groups: ::data_views::Field<item::MeshGroups<'a, Mode>>,
        pub keepalive: ::data_views::Field<item::Keepalive<'a, Mode>>,
        pub sa_filter: ::data_views::Field<item::SaFilter<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct DefaultPeer<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub prefix_list: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct MeshGroups<'a, Mode> (::data_views::Field<mesh_groups::Item<'a, Mode>>);

        pub mod mesh_groups {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
            }
        }

        #[::data_views::data_view]
        pub struct Keepalive<'a, Mode> {
            pub keepalive_timer: ::data_views::RequiredValue<i64, Mode>,
            pub hold_timer: ::data_views::RequiredValue<i64, Mode>,
        }

        #[::data_views::data_view]
        pub struct SaFilter<'a, Mode> {
            pub in_list: ::data_views::Field<&'a str>,
            pub out_list: ::data_views::Field<&'a str>,
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub originator_id_local_interface: ::data_views::Field<&'a str>,
        pub rejected_limit: ::data_views::Field<i64>,
        pub forward_register_packets: ::data_views::Field<bool>,
        pub connection_retry_interval: ::data_views::Field<i64>,
        pub group_limits: ::data_views::Field<item::GroupLimits<'a, Mode>>,
        pub peers: ::data_views::Field<item::Peers<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(indexed_list, primary_key(source_prefix))]
        pub struct GroupLimits<'a, Mode> (::data_views::Field<group_limits::Item<'a, Mode>>);

        pub mod group_limits {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub source_prefix: ::data_views::Field<&'a str>,
                pub limit: ::data_views::RequiredValue<i64, Mode>,
            }
        }

        #[::data_views::data_view(indexed_list, primary_key(ipv4_address))]
        pub struct Peers<'a, Mode> (::data_views::Field<peers::Item<'a, Mode>>);

        pub mod peers {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub ipv4_address: ::data_views::Field<&'a str>,
                pub default_peer: ::data_views::Field<item::DefaultPeer<'a, Mode>>,
                pub local_interface: ::data_views::Field<&'a str>,
                pub description: ::data_views::Field<&'a str>,
                pub disabled: ::data_views::Field<bool>,
                pub sa_limit: ::data_views::Field<i64>,
                pub mesh_groups: ::data_views::Field<item::MeshGroups<'a, Mode>>,
                pub keepalive: ::data_views::Field<item::Keepalive<'a, Mode>>,
                pub sa_filter: ::data_views::Field<item::SaFilter<'a, Mode>>,
            }

            pub mod item {

                #[::data_views::data_view]
                pub struct DefaultPeer<'a, Mode> {
                    pub enabled: ::data_views::Field<bool>,
                    pub prefix_list: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view(indexed_list, primary_key(name))]
                pub struct MeshGroups<'a, Mode> (::data_views::Field<mesh_groups::Item<'a, Mode>>);

                pub mod mesh_groups {

                    #[::data_views::data_view]
                    pub struct Item<'a, Mode> {
                        pub name: ::data_views::Field<&'a str>,
                    }
                }

                #[::data_views::data_view]
                pub struct Keepalive<'a, Mode> {
                    pub keepalive_timer: ::data_views::RequiredValue<i64, Mode>,
                    pub hold_timer: ::data_views::RequiredValue<i64, Mode>,
                }

                #[::data_views::data_view]
                pub struct SaFilter<'a, Mode> {
                    pub in_list: ::data_views::Field<&'a str>,
                    pub out_list: ::data_views::Field<&'a str>,
                }
            }
        }
    }
}
