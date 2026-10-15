// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.L3Edge.P2pLinksIpPools", item_name = "AVDDesign.L3Edge.P2pLinksIpPoolsItem"))]
pub struct P2pLinksIpPools<'a, Mode> (::validated_data::Field<p2p_links_ip_pools::Item<'a, Mode>>);

pub mod p2p_links_ip_pools {

    #[::validated_data::data_view(python(name = "AVDDesign.L3Edge.P2pLinksIpPoolsItemBase"))]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub ipv4_pool: ::validated_data::Field<&'a str>,
        pub prefix_size: ::validated_data::Field<i64>,
        pub ipv6_pool: ::validated_data::Field<&'a str>,
        pub ipv6_prefix_size: ::validated_data::Field<i64>,
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.L3Edge.P2pLinksProfiles", item_name = "AVDDesign.L3Edge.P2pLinksProfilesItem"))]
pub struct P2pLinksProfiles<'a, Mode> (::validated_data::Field<p2p_links_profiles::Item<'a, Mode>>);

pub mod p2p_links_profiles {

    #[::validated_data::data_view(python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase"))]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub id: ::validated_data::Field<i64>,
        pub speed: ::validated_data::Field<&'a str>,
        pub ip_pool: ::validated_data::Field<&'a str>,
        pub subnet: ::validated_data::Field<&'a str>,
        pub ip: ::validated_data::Field<item::Ip<'a, Mode>>,
        pub ipv6_enable: ::validated_data::Field<bool>,
        pub ipv6_prefix: ::validated_data::Field<&'a str>,
        pub ipv6: ::validated_data::Field<item::Ipv6<'a, Mode>>,
        pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
        pub interfaces: ::validated_data::Field<item::Interfaces<'a, Mode>>,
        #[data_view(rename = "as")]
        pub field_as: ::validated_data::Field<item::FieldAs<'a, Mode>>,
        pub descriptions: ::validated_data::Field<item::Descriptions<'a, Mode>>,
        pub include_in_underlay_protocol: ::validated_data::Field<bool>,
        pub use_underlay_ospf_authentication: ::validated_data::Field<bool>,
        pub isis_hello_padding: ::validated_data::Field<bool>,
        pub isis_metric: ::validated_data::Field<i64>,
        pub isis_circuit_type: ::validated_data::Field<&'a str>,
        pub isis_authentication_mode: ::validated_data::Field<&'a str>,
        pub isis_authentication_key: ::validated_data::Field<&'a str>,
        pub isis_authentication_cleartext_key: ::validated_data::Field<&'a str>,
        pub isis_network_type: ::validated_data::Field<&'a str>,
        pub mpls_ip: ::validated_data::Field<bool>,
        pub mpls_ldp: ::validated_data::Field<bool>,
        pub mtu: ::validated_data::Field<i64>,
        pub bfd: ::validated_data::Field<bool>,
        pub ptp: ::validated_data::Field<item::Ptp<'a, Mode>>,
        pub sflow: ::validated_data::Field<bool>,
        pub multicast_pim_sm: ::validated_data::Field<bool>,
        pub multicast_static: ::validated_data::Field<bool>,
        pub flow_tracking: ::validated_data::Field<item::FlowTracking<'a, Mode>>,
        pub qos_profile: ::validated_data::Field<&'a str>,
        pub macsec_profile: ::validated_data::Field<&'a str>,
        pub port_channel: ::validated_data::Field<item::PortChannel<'a, Mode>>,
        pub campus_link_type: ::validated_data::Field<item::CampusLinkType<'a, Mode>>,
        pub raw_eos_cli: ::validated_data::Field<&'a str>,
        pub routing_protocol: ::validated_data::Field<&'a str>,
        #[data_view(relaxed)]
        pub ethernet_structured_config: ::validated_data::Field<super::super::super::eos_config::ethernet_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
        #[data_view(relaxed)]
        pub port_channel_structured_config: ::validated_data::Field<super::super::super::eos_config::port_channel_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
    }

    pub mod item {

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.Ip"))]
        pub struct Ip<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.Ipv6"))]
        pub struct Ipv6<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.Nodes"))]
        pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.Interfaces"))]
        pub struct Interfaces<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.As"))]
        pub struct FieldAs<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.Descriptions"))]
        pub struct Descriptions<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.Ptp"))]
        pub struct Ptp<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub roles: ::validated_data::Field<ptp::Roles<'a, Mode>>,
            pub profile: ::validated_data::Field<&'a str>,
        }

        pub mod ptp {

            #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.Ptp.Roles"))]
            pub struct Roles<'a, Mode> (::validated_data::Field<&'a str>);
        }

        #[::validated_data::data_view(python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.FlowTracking"))]
        pub struct FlowTracking<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub name: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view(python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.PortChannel"))]
        pub struct PortChannel<'a, Mode> {
            pub description: ::validated_data::Field<&'a str>,
            pub mode: ::validated_data::Field<&'a str>,
            pub channel_id_algorithm: ::validated_data::Field<&'a str>,
            pub channel_id_offset: ::validated_data::Field<i64>,
            pub nodes_child_interfaces: ::validated_data::Field<port_channel::NodesChildInterfaces<'a, Mode>>,
        }

        pub mod port_channel {

            #[::validated_data::data_view(indexed_list, primary_key(node), python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.PortChannel.NodesChildInterfaces", item_name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.PortChannel.NodesChildInterfacesItem"))]
            pub struct NodesChildInterfaces<'a, Mode> (::validated_data::Field<nodes_child_interfaces::Item<'a, Mode>>);

            pub mod nodes_child_interfaces {

                #[::validated_data::data_view(python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.PortChannel.NodesChildInterfacesItemBase"))]
                pub struct Item<'a, Mode> {
                    pub node: ::validated_data::Field<&'a str>,
                    pub interfaces: ::validated_data::Field<item::Interfaces<'a, Mode>>,
                    pub channel_id: ::validated_data::Field<i64>,
                }

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.PortChannel.NodesChildInterfacesItemBase.Interfaces"))]
                    pub struct Interfaces<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }
        }

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.CampusLinkType"))]
        pub struct CampusLinkType<'a, Mode> (::validated_data::Field<&'a str>);
    }
}

#[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinks"))]
pub struct P2pLinks<'a, Mode> (::validated_data::Field<p2p_links::Item<'a, Mode>>);

pub mod p2p_links {

    #[::validated_data::data_view(python(name = "AVDDesign.L3Edge.P2pLinksItem"))]
    pub struct Item<'a, Mode> {
        pub nodes: ::validated_data::RequiredValue<item::Nodes<'a, Mode>, Mode>,
        pub profile: ::validated_data::Field<&'a str>,
        pub id: ::validated_data::Field<i64>,
        pub speed: ::validated_data::Field<&'a str>,
        pub ip_pool: ::validated_data::Field<&'a str>,
        pub subnet: ::validated_data::Field<&'a str>,
        pub ip: ::validated_data::Field<item::Ip<'a, Mode>>,
        pub ipv6_enable: ::validated_data::Field<bool>,
        pub ipv6_prefix: ::validated_data::Field<&'a str>,
        pub ipv6: ::validated_data::Field<item::Ipv6<'a, Mode>>,
        pub interfaces: ::validated_data::Field<item::Interfaces<'a, Mode>>,
        #[data_view(rename = "as")]
        pub field_as: ::validated_data::Field<item::FieldAs<'a, Mode>>,
        pub descriptions: ::validated_data::Field<item::Descriptions<'a, Mode>>,
        pub include_in_underlay_protocol: ::validated_data::Field<bool>,
        pub use_underlay_ospf_authentication: ::validated_data::Field<bool>,
        pub isis_hello_padding: ::validated_data::Field<bool>,
        pub isis_metric: ::validated_data::Field<i64>,
        pub isis_circuit_type: ::validated_data::Field<&'a str>,
        pub isis_authentication_mode: ::validated_data::Field<&'a str>,
        pub isis_authentication_key: ::validated_data::Field<&'a str>,
        pub isis_authentication_cleartext_key: ::validated_data::Field<&'a str>,
        pub isis_network_type: ::validated_data::Field<&'a str>,
        pub mpls_ip: ::validated_data::Field<bool>,
        pub mpls_ldp: ::validated_data::Field<bool>,
        pub mtu: ::validated_data::Field<i64>,
        pub bfd: ::validated_data::Field<bool>,
        pub ptp: ::validated_data::Field<item::Ptp<'a, Mode>>,
        pub sflow: ::validated_data::Field<bool>,
        pub multicast_pim_sm: ::validated_data::Field<bool>,
        pub multicast_static: ::validated_data::Field<bool>,
        pub flow_tracking: ::validated_data::Field<item::FlowTracking<'a, Mode>>,
        pub qos_profile: ::validated_data::Field<&'a str>,
        pub macsec_profile: ::validated_data::Field<&'a str>,
        pub port_channel: ::validated_data::Field<item::PortChannel<'a, Mode>>,
        pub campus_link_type: ::validated_data::Field<item::CampusLinkType<'a, Mode>>,
        pub raw_eos_cli: ::validated_data::Field<&'a str>,
        pub routing_protocol: ::validated_data::Field<&'a str>,
        #[data_view(relaxed)]
        pub ethernet_structured_config: ::validated_data::Field<super::super::super::eos_config::ethernet_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
        #[data_view(relaxed)]
        pub port_channel_structured_config: ::validated_data::Field<super::super::super::eos_config::port_channel_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
    }

    pub mod item {

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksItem.Nodes"))]
        pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksItem.Ip"))]
        pub struct Ip<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksItem.Ipv6"))]
        pub struct Ipv6<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksItem.Interfaces"))]
        pub struct Interfaces<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksItem.As"))]
        pub struct FieldAs<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksItem.Descriptions"))]
        pub struct Descriptions<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(python(name = "AVDDesign.L3Edge.P2pLinksItem.Ptp"))]
        pub struct Ptp<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub roles: ::validated_data::Field<ptp::Roles<'a, Mode>>,
            pub profile: ::validated_data::Field<&'a str>,
        }

        pub mod ptp {

            #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksItem.Ptp.Roles"))]
            pub struct Roles<'a, Mode> (::validated_data::Field<&'a str>);
        }

        #[::validated_data::data_view(python(name = "AVDDesign.L3Edge.P2pLinksItem.FlowTracking"))]
        pub struct FlowTracking<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub name: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view(python(name = "AVDDesign.L3Edge.P2pLinksItem.PortChannel"))]
        pub struct PortChannel<'a, Mode> {
            pub description: ::validated_data::Field<&'a str>,
            pub mode: ::validated_data::Field<&'a str>,
            pub channel_id_algorithm: ::validated_data::Field<&'a str>,
            pub channel_id_offset: ::validated_data::Field<i64>,
            pub nodes_child_interfaces: ::validated_data::Field<port_channel::NodesChildInterfaces<'a, Mode>>,
        }

        pub mod port_channel {

            #[::validated_data::data_view(indexed_list, primary_key(node), python(name = "AVDDesign.L3Edge.P2pLinksItem.PortChannel.NodesChildInterfaces", item_name = "AVDDesign.L3Edge.P2pLinksItem.PortChannel.NodesChildInterfacesItem"))]
            pub struct NodesChildInterfaces<'a, Mode> (::validated_data::Field<nodes_child_interfaces::Item<'a, Mode>>);

            pub mod nodes_child_interfaces {

                #[::validated_data::data_view(python(name = "AVDDesign.L3Edge.P2pLinksItem.PortChannel.NodesChildInterfacesItemBase"))]
                pub struct Item<'a, Mode> {
                    pub node: ::validated_data::Field<&'a str>,
                    pub interfaces: ::validated_data::Field<item::Interfaces<'a, Mode>>,
                    pub channel_id: ::validated_data::Field<i64>,
                }

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksItem.PortChannel.NodesChildInterfacesItemBase.Interfaces"))]
                    pub struct Interfaces<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }
        }

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksItem.CampusLinkType"))]
        pub struct CampusLinkType<'a, Mode> (::validated_data::Field<&'a str>);
    }
}
