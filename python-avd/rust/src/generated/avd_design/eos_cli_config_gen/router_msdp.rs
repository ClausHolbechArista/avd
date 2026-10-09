// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(source_prefix))]
pub struct GroupLimits<'a, Mode> (::validated_data::Field<group_limits::Item<'a, Mode>>);

pub mod group_limits {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub source_prefix: ::validated_data::Field<&'a str>,
        pub limit: ::validated_data::RequiredValue<i64, Mode>,
    }
}

#[::validated_data::data_view(indexed_list, primary_key(ipv4_address))]
pub struct Peers<'a, Mode> (::validated_data::Field<peers::Item<'a, Mode>>);

pub mod peers {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub ipv4_address: ::validated_data::Field<&'a str>,
        pub default_peer: ::validated_data::Field<item::DefaultPeer<'a, Mode>>,
        pub local_interface: ::validated_data::Field<&'a str>,
        pub description: ::validated_data::Field<&'a str>,
        pub disabled: ::validated_data::Field<bool>,
        pub sa_limit: ::validated_data::Field<i64>,
        pub mesh_groups: ::validated_data::Field<item::MeshGroups<'a, Mode>>,
        pub keepalive: ::validated_data::Field<item::Keepalive<'a, Mode>>,
        pub sa_filter: ::validated_data::Field<item::SaFilter<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct DefaultPeer<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub prefix_list: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct MeshGroups<'a, Mode> (::validated_data::Field<mesh_groups::Item<'a, Mode>>);

        pub mod mesh_groups {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
            }
        }

        #[::validated_data::data_view]
        pub struct Keepalive<'a, Mode> {
            pub keepalive_timer: ::validated_data::RequiredValue<i64, Mode>,
            pub hold_timer: ::validated_data::RequiredValue<i64, Mode>,
        }

        #[::validated_data::data_view]
        pub struct SaFilter<'a, Mode> {
            pub in_list: ::validated_data::Field<&'a str>,
            pub out_list: ::validated_data::Field<&'a str>,
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub originator_id_local_interface: ::validated_data::Field<&'a str>,
        pub rejected_limit: ::validated_data::Field<i64>,
        pub forward_register_packets: ::validated_data::Field<bool>,
        pub connection_retry_interval: ::validated_data::Field<i64>,
        pub group_limits: ::validated_data::Field<item::GroupLimits<'a, Mode>>,
        pub peers: ::validated_data::Field<item::Peers<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(source_prefix))]
        pub struct GroupLimits<'a, Mode> (::validated_data::Field<group_limits::Item<'a, Mode>>);

        pub mod group_limits {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub source_prefix: ::validated_data::Field<&'a str>,
                pub limit: ::validated_data::RequiredValue<i64, Mode>,
            }
        }

        #[::validated_data::data_view(indexed_list, primary_key(ipv4_address))]
        pub struct Peers<'a, Mode> (::validated_data::Field<peers::Item<'a, Mode>>);

        pub mod peers {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub ipv4_address: ::validated_data::Field<&'a str>,
                pub default_peer: ::validated_data::Field<item::DefaultPeer<'a, Mode>>,
                pub local_interface: ::validated_data::Field<&'a str>,
                pub description: ::validated_data::Field<&'a str>,
                pub disabled: ::validated_data::Field<bool>,
                pub sa_limit: ::validated_data::Field<i64>,
                pub mesh_groups: ::validated_data::Field<item::MeshGroups<'a, Mode>>,
                pub keepalive: ::validated_data::Field<item::Keepalive<'a, Mode>>,
                pub sa_filter: ::validated_data::Field<item::SaFilter<'a, Mode>>,
            }

            pub mod item {

                #[::validated_data::data_view]
                pub struct DefaultPeer<'a, Mode> {
                    pub enabled: ::validated_data::Field<bool>,
                    pub prefix_list: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view(indexed_list, primary_key(name))]
                pub struct MeshGroups<'a, Mode> (::validated_data::Field<mesh_groups::Item<'a, Mode>>);

                pub mod mesh_groups {

                    #[::validated_data::data_view]
                    pub struct Item<'a, Mode> {
                        pub name: ::validated_data::Field<&'a str>,
                    }
                }

                #[::validated_data::data_view]
                pub struct Keepalive<'a, Mode> {
                    pub keepalive_timer: ::validated_data::RequiredValue<i64, Mode>,
                    pub hold_timer: ::validated_data::RequiredValue<i64, Mode>,
                }

                #[::validated_data::data_view]
                pub struct SaFilter<'a, Mode> {
                    pub in_list: ::validated_data::Field<&'a str>,
                    pub out_list: ::validated_data::Field<&'a str>,
                }
            }
        }
    }
}
