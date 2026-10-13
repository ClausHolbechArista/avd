// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Vxlan1<'a, Mode> {
    pub description: ::validated_data::Field<&'a str>,
    pub vxlan: ::validated_data::Field<vxlan1::Vxlan<'a, Mode>>,
    pub eos_cli: ::validated_data::Field<&'a str>,
}

pub mod vxlan1 {

    #[::validated_data::data_view]
    pub struct Vxlan<'a, Mode> {
        pub source_interface: ::validated_data::Field<&'a str>,
        pub shutdown: ::validated_data::Field<bool>,
        pub multicast: ::validated_data::Field<vxlan::Multicast<'a, Mode>>,
        pub controller_client: ::validated_data::Field<vxlan::ControllerClient<'a, Mode>>,
        pub mlag_source_interface: ::validated_data::Field<&'a str>,
        pub udp_port: ::validated_data::Field<i64>,
        pub encapsulations: ::validated_data::Field<vxlan::Encapsulations<'a, Mode>>,
        pub vtep_to_vtep_bridging: ::validated_data::Field<bool>,
        pub virtual_router_encapsulation_mac_address: ::validated_data::Field<&'a str>,
        pub bfd_vtep_evpn: ::validated_data::Field<vxlan::BfdVtepEvpn<'a, Mode>>,
        pub qos: ::validated_data::Field<vxlan::Qos<'a, Mode>>,
        pub vlan_range: ::validated_data::Field<vxlan::VlanRange<'a, Mode>>,
        pub vlans: ::validated_data::Field<vxlan::Vlans<'a, Mode>>,
        pub vrfs: ::validated_data::Field<vxlan::Vrfs<'a, Mode>>,
        pub flood_vteps: ::validated_data::Field<vxlan::FloodVteps<'a, Mode>>,
        pub flood_vtep_learned_data_plane: ::validated_data::Field<bool>,
    }

    pub mod vxlan {

        #[::validated_data::data_view]
        pub struct Multicast<'a, Mode> {
            pub headend_replication: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct ControllerClient<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct Encapsulations<'a, Mode> {
            pub ipv4: ::validated_data::Field<bool>,
            pub ipv6: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct BfdVtepEvpn<'a, Mode> {
            pub interval: ::validated_data::Field<i64>,
            pub min_rx: ::validated_data::Field<i64>,
            pub multiplier: ::validated_data::Field<i64>,
            pub prefix_list: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct Qos<'a, Mode> {
            pub dscp_propagation_encapsulation: ::validated_data::Field<bool>,
            pub ecn_propagation: ::validated_data::Field<bool>,
            pub map_dscp_to_traffic_class_decapsulation: ::validated_data::Field<bool>,
            pub dscp_ecn: ::validated_data::Field<qos::DscpEcn<'a, Mode>>,
        }

        pub mod qos {

            #[::validated_data::data_view]
            pub struct DscpEcn<'a, Mode> {
                pub rewrite_bridged_enabled: ::validated_data::Field<bool>,
            }
        }

        #[::validated_data::data_view]
        pub struct VlanRange<'a, Mode> {
            pub vlans: ::validated_data::RequiredValue<&'a str, Mode>,
            pub vnis: ::validated_data::RequiredValue<&'a str, Mode>,
        }

        #[::validated_data::data_view(indexed_list, primary_key(id))]
        pub struct Vlans<'a, Mode> (::validated_data::Field<vlans::Item<'a, Mode>>);

        pub mod vlans {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub id: ::validated_data::Field<i64>,
                pub vni: ::validated_data::Field<i64>,
                pub multicast_group: ::validated_data::Field<&'a str>,
                pub flood_vteps: ::validated_data::Field<item::FloodVteps<'a, Mode>>,
                pub flood_group: ::validated_data::Field<&'a str>,
            }

            pub mod item {

                #[::validated_data::data_view(list)]
                pub struct FloodVteps<'a, Mode> (::validated_data::Field<&'a str>);
            }
        }

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

        pub mod vrfs {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub vni: ::validated_data::Field<i64>,
                pub multicast_group: ::validated_data::Field<&'a str>,
                pub multicast_group_encap_range: ::validated_data::Field<&'a str>,
                pub multicast_groups: ::validated_data::Field<item::MulticastGroups<'a, Mode>>,
            }

            pub mod item {

                #[::validated_data::data_view(indexed_list, primary_key(overlay_group))]
                pub struct MulticastGroups<'a, Mode> (::validated_data::Field<multicast_groups::Item<'a, Mode>>);

                pub mod multicast_groups {

                    #[::validated_data::data_view]
                    pub struct Item<'a, Mode> {
                        pub overlay_group: ::validated_data::Field<&'a str>,
                        pub encap: ::validated_data::RequiredValue<&'a str, Mode>,
                    }
                }
            }
        }

        #[::validated_data::data_view(list)]
        pub struct FloodVteps<'a, Mode> (::validated_data::Field<&'a str>);
    }
}
