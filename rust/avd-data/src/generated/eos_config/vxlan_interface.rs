// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct VxlanInterface<'a, Mode> {
    pub vxlan1: ::data_views::Field<Vxlan1<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Vxlan1<'a, Mode> {
    pub description: ::data_views::Field<&'a str>,
    pub vxlan: ::data_views::Field<vxlan1::Vxlan<'a, Mode>>,
    pub eos_cli: ::data_views::Field<&'a str>,
}

pub mod vxlan1 {

    #[::data_views::data_view]
    pub struct Vxlan<'a, Mode> {
        pub source_interface: ::data_views::Field<&'a str>,
        pub shutdown: ::data_views::Field<bool>,
        pub multicast: ::data_views::Field<vxlan::Multicast<'a, Mode>>,
        pub controller_client: ::data_views::Field<vxlan::ControllerClient<'a, Mode>>,
        pub mlag_source_interface: ::data_views::Field<&'a str>,
        pub udp_port: ::data_views::Field<i64>,
        pub encapsulations: ::data_views::Field<vxlan::Encapsulations<'a, Mode>>,
        pub vtep_to_vtep_bridging: ::data_views::Field<bool>,
        pub virtual_router_encapsulation_mac_address: ::data_views::Field<&'a str>,
        pub bfd_vtep_evpn: ::data_views::Field<vxlan::BfdVtepEvpn<'a, Mode>>,
        pub qos: ::data_views::Field<vxlan::Qos<'a, Mode>>,
        pub vlan_range: ::data_views::Field<vxlan::VlanRange<'a, Mode>>,
        pub vlans: ::data_views::Field<vxlan::Vlans<'a, Mode>>,
        pub vrfs: ::data_views::Field<vxlan::Vrfs<'a, Mode>>,
        pub flood_vteps: ::data_views::Field<vxlan::FloodVteps<'a, Mode>>,
        pub flood_vtep_learned_data_plane: ::data_views::Field<bool>,
    }

    pub mod vxlan {

        #[::data_views::data_view]
        pub struct Multicast<'a, Mode> {
            pub headend_replication: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct ControllerClient<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct Encapsulations<'a, Mode> {
            pub ipv4: ::data_views::Field<bool>,
            pub ipv6: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct BfdVtepEvpn<'a, Mode> {
            pub interval: ::data_views::Field<i64>,
            pub min_rx: ::data_views::Field<i64>,
            pub multiplier: ::data_views::Field<i64>,
            pub prefix_list: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct Qos<'a, Mode> {
            pub dscp_propagation_encapsulation: ::data_views::Field<bool>,
            pub ecn_propagation: ::data_views::Field<bool>,
            pub map_dscp_to_traffic_class_decapsulation: ::data_views::Field<bool>,
            pub dscp_ecn: ::data_views::Field<qos::DscpEcn<'a, Mode>>,
        }

        pub mod qos {

            #[::data_views::data_view]
            pub struct DscpEcn<'a, Mode> {
                pub rewrite_bridged_enabled: ::data_views::Field<bool>,
            }
        }

        #[::data_views::data_view]
        pub struct VlanRange<'a, Mode> {
            pub vlans: ::data_views::RequiredValue<&'a str, Mode>,
            pub vnis: ::data_views::RequiredValue<&'a str, Mode>,
        }

        #[::data_views::data_view(indexed_list, primary_key(id))]
        pub struct Vlans<'a, Mode> (::data_views::Field<vlans::Item<'a, Mode>>);

        pub mod vlans {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub id: ::data_views::Field<i64>,
                pub vni: ::data_views::Field<i64>,
                pub multicast_group: ::data_views::Field<&'a str>,
                pub flood_vteps: ::data_views::Field<item::FloodVteps<'a, Mode>>,
                pub flood_group: ::data_views::Field<&'a str>,
            }

            pub mod item {

                #[::data_views::data_view(list)]
                pub struct FloodVteps<'a, Mode> (::data_views::Field<&'a str>);
            }
        }

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

        pub mod vrfs {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
                pub vni: ::data_views::Field<i64>,
                pub multicast_group: ::data_views::Field<&'a str>,
                pub multicast_group_encap_range: ::data_views::Field<&'a str>,
                pub multicast_groups: ::data_views::Field<item::MulticastGroups<'a, Mode>>,
            }

            pub mod item {

                #[::data_views::data_view(indexed_list, primary_key(overlay_group))]
                pub struct MulticastGroups<'a, Mode> (::data_views::Field<multicast_groups::Item<'a, Mode>>);

                pub mod multicast_groups {

                    #[::data_views::data_view]
                    pub struct Item<'a, Mode> {
                        pub overlay_group: ::data_views::Field<&'a str>,
                        pub encap: ::data_views::RequiredValue<&'a str, Mode>,
                    }
                }
            }
        }

        #[::data_views::data_view(list)]
        pub struct FloodVteps<'a, Mode> (::data_views::Field<&'a str>);
    }
}
