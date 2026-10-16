// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.L3Edge.P2pLinksIpPools", schema = List(579), item_name = "AVDDesign.L3Edge.P2pLinksIpPoolsItem"))]
pub struct P2pLinksIpPools<'a, Mode> (::validated_data::Field<p2p_links_ip_pools::Item<'a, Mode>>);

pub mod p2p_links_ip_pools {

    #[::validated_data::data_view(python(name = "AVDDesign.L3Edge.P2pLinksIpPoolsItemBase", schema = Dict(1436)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1169))]
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1170))]
        pub ipv4_pool: ::validated_data::Field<&'a str>,
        #[data_view(schema = Int(517))]
        pub prefix_size: ::validated_data::Field<i64>,
        #[data_view(schema = Str(1171))]
        pub ipv6_pool: ::validated_data::Field<&'a str>,
        #[data_view(schema = Int(518))]
        pub ipv6_prefix_size: ::validated_data::Field<i64>,
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.L3Edge.P2pLinksProfiles", schema = List(589), item_name = "AVDDesign.L3Edge.P2pLinksProfilesItem"))]
pub struct P2pLinksProfiles<'a, Mode> (::validated_data::Field<p2p_links_profiles::Item<'a, Mode>>);

pub mod p2p_links_profiles {

    #[::validated_data::data_view(python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase", schema = Dict(1444)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1172))]
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(schema = Int(519))]
        pub id: ::validated_data::Field<i64>,
        #[data_view(schema = Str(178))]
        pub speed: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1173))]
        pub ip_pool: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1174))]
        pub subnet: ::validated_data::Field<&'a str>,
        #[data_view(schema = List(580))]
        pub ip: ::validated_data::Field<item::Ip<'a, Mode>>,
        #[data_view(schema = Bool(437))]
        pub ipv6_enable: ::validated_data::Field<bool>,
        #[data_view(schema = Str(1176))]
        pub ipv6_prefix: ::validated_data::Field<&'a str>,
        #[data_view(schema = List(581))]
        pub ipv6: ::validated_data::Field<item::Ipv6<'a, Mode>>,
        #[data_view(schema = List(582))]
        pub nodes: ::validated_data::Field<item::Nodes<'a, Mode>>,
        #[data_view(schema = List(583))]
        pub interfaces: ::validated_data::Field<item::Interfaces<'a, Mode>>,
        #[data_view(schema = List(584), rename = "as")]
        pub field_as: ::validated_data::Field<item::FieldAs<'a, Mode>>,
        #[data_view(schema = List(585))]
        pub descriptions: ::validated_data::Field<item::Descriptions<'a, Mode>>,
        #[data_view(schema = Bool(438))]
        pub include_in_underlay_protocol: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(439))]
        pub use_underlay_ospf_authentication: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(37))]
        pub isis_hello_padding: ::validated_data::Field<bool>,
        #[data_view(schema = Int(77))]
        pub isis_metric: ::validated_data::Field<i64>,
        #[data_view(schema = Str(878))]
        pub isis_circuit_type: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1182))]
        pub isis_authentication_mode: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1183))]
        pub isis_authentication_key: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1184))]
        pub isis_authentication_cleartext_key: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1185))]
        pub isis_network_type: ::validated_data::Field<&'a str>,
        #[data_view(schema = Bool(440))]
        pub mpls_ip: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(441))]
        pub mpls_ldp: ::validated_data::Field<bool>,
        #[data_view(schema = Int(520))]
        pub mtu: ::validated_data::Field<i64>,
        #[data_view(schema = Bool(442))]
        pub bfd: ::validated_data::Field<bool>,
        #[data_view(schema = Dict(1437))]
        pub ptp: ::validated_data::Field<item::Ptp<'a, Mode>>,
        #[data_view(schema = Bool(444))]
        pub sflow: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(445))]
        pub multicast_pim_sm: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(446))]
        pub multicast_static: ::validated_data::Field<bool>,
        #[data_view(schema = Dict(1438))]
        pub flow_tracking: ::validated_data::Field<item::FlowTracking<'a, Mode>>,
        #[data_view(schema = Str(1187))]
        pub qos_profile: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1188))]
        pub macsec_profile: ::validated_data::Field<&'a str>,
        #[data_view(schema = Dict(1440))]
        pub port_channel: ::validated_data::Field<item::PortChannel<'a, Mode>>,
        #[data_view(schema = List(574))]
        pub campus_link_type: ::validated_data::Field<item::CampusLinkType<'a, Mode>>,
        #[data_view(schema = Str(1192))]
        pub raw_eos_cli: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1193))]
        pub routing_protocol: ::validated_data::Field<&'a str>,
        #[data_view(schema = Dict(1441), relaxed)]
        pub ethernet_structured_config: ::validated_data::Field<super::super::super::eos_config::ethernet_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
        #[data_view(schema = Dict(1442), relaxed)]
        pub port_channel_structured_config: ::validated_data::Field<super::super::super::eos_config::port_channel_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
    }

    pub mod item {

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.Ip", schema = List(580)))]
        pub struct Ip<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.Ipv6", schema = List(581)))]
        pub struct Ipv6<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.Nodes", schema = List(582)))]
        pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.Interfaces", schema = List(583)))]
        pub struct Interfaces<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.As", schema = List(584)))]
        pub struct FieldAs<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.Descriptions", schema = List(585)))]
        pub struct Descriptions<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.Ptp", schema = Dict(1437)))]
        pub struct Ptp<'a, Mode> {
            #[data_view(schema = Bool(443))]
            pub enabled: ::validated_data::Field<bool>,
            #[data_view(schema = List(586))]
            pub roles: ::validated_data::Field<ptp::Roles<'a, Mode>>,
            #[data_view(schema = Str(1154))]
            pub profile: ::validated_data::Field<&'a str>,
        }

        pub mod ptp {

            #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.Ptp.Roles", schema = List(586)))]
            pub struct Roles<'a, Mode> (::validated_data::Field<&'a str>);
        }

        #[::validated_data::data_view(python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.FlowTracking", schema = Dict(1438)))]
        pub struct FlowTracking<'a, Mode> {
            #[data_view(schema = Bool(7))]
            pub enabled: ::validated_data::Field<bool>,
            #[data_view(schema = Str(1155))]
            pub name: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view(python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.PortChannel", schema = Dict(1440)))]
        pub struct PortChannel<'a, Mode> {
            #[data_view(schema = Str(1189))]
            pub description: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1190))]
            pub mode: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1191))]
            pub channel_id_algorithm: ::validated_data::Field<&'a str>,
            #[data_view(schema = Int(521))]
            pub channel_id_offset: ::validated_data::Field<i64>,
            #[data_view(schema = List(588))]
            pub nodes_child_interfaces: ::validated_data::Field<port_channel::NodesChildInterfaces<'a, Mode>>,
        }

        pub mod port_channel {

            #[::validated_data::data_view(indexed_list, primary_key(node), python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.PortChannel.NodesChildInterfaces", schema = List(588), item_name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.PortChannel.NodesChildInterfacesItem"))]
            pub struct NodesChildInterfaces<'a, Mode> (::validated_data::Field<nodes_child_interfaces::Item<'a, Mode>>);

            pub mod nodes_child_interfaces {

                #[::validated_data::data_view(python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.PortChannel.NodesChildInterfacesItemBase", schema = Dict(1439)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(3))]
                    pub node: ::validated_data::Field<&'a str>,
                    #[data_view(schema = List(587))]
                    pub interfaces: ::validated_data::Field<item::Interfaces<'a, Mode>>,
                    #[data_view(schema = Int(522))]
                    pub channel_id: ::validated_data::Field<i64>,
                }

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.PortChannel.NodesChildInterfacesItemBase.Interfaces", schema = List(587)))]
                    pub struct Interfaces<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }
        }

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksProfilesItemBase.CampusLinkType", schema = List(574)))]
        pub struct CampusLinkType<'a, Mode> (::validated_data::Field<&'a str>);
    }
}

#[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinks", schema = List(591)))]
pub struct P2pLinks<'a, Mode> (::validated_data::Field<p2p_links::Item<'a, Mode>>);

pub mod p2p_links {

    #[::validated_data::data_view(python(name = "AVDDesign.L3Edge.P2pLinksItem", schema = Dict(1445)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = List(590))]
        pub nodes: ::validated_data::RequiredValue<item::Nodes<'a, Mode>, Mode>,
        #[data_view(schema = Str(1194))]
        pub profile: ::validated_data::Field<&'a str>,
        #[data_view(schema = Int(519))]
        pub id: ::validated_data::Field<i64>,
        #[data_view(schema = Str(178))]
        pub speed: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1173))]
        pub ip_pool: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1174))]
        pub subnet: ::validated_data::Field<&'a str>,
        #[data_view(schema = List(580))]
        pub ip: ::validated_data::Field<item::Ip<'a, Mode>>,
        #[data_view(schema = Bool(437))]
        pub ipv6_enable: ::validated_data::Field<bool>,
        #[data_view(schema = Str(1176))]
        pub ipv6_prefix: ::validated_data::Field<&'a str>,
        #[data_view(schema = List(581))]
        pub ipv6: ::validated_data::Field<item::Ipv6<'a, Mode>>,
        #[data_view(schema = List(583))]
        pub interfaces: ::validated_data::Field<item::Interfaces<'a, Mode>>,
        #[data_view(schema = List(584), rename = "as")]
        pub field_as: ::validated_data::Field<item::FieldAs<'a, Mode>>,
        #[data_view(schema = List(585))]
        pub descriptions: ::validated_data::Field<item::Descriptions<'a, Mode>>,
        #[data_view(schema = Bool(438))]
        pub include_in_underlay_protocol: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(439))]
        pub use_underlay_ospf_authentication: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(37))]
        pub isis_hello_padding: ::validated_data::Field<bool>,
        #[data_view(schema = Int(77))]
        pub isis_metric: ::validated_data::Field<i64>,
        #[data_view(schema = Str(878))]
        pub isis_circuit_type: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1182))]
        pub isis_authentication_mode: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1183))]
        pub isis_authentication_key: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1184))]
        pub isis_authentication_cleartext_key: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1185))]
        pub isis_network_type: ::validated_data::Field<&'a str>,
        #[data_view(schema = Bool(440))]
        pub mpls_ip: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(441))]
        pub mpls_ldp: ::validated_data::Field<bool>,
        #[data_view(schema = Int(520))]
        pub mtu: ::validated_data::Field<i64>,
        #[data_view(schema = Bool(442))]
        pub bfd: ::validated_data::Field<bool>,
        #[data_view(schema = Dict(1437))]
        pub ptp: ::validated_data::Field<item::Ptp<'a, Mode>>,
        #[data_view(schema = Bool(444))]
        pub sflow: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(445))]
        pub multicast_pim_sm: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(446))]
        pub multicast_static: ::validated_data::Field<bool>,
        #[data_view(schema = Dict(1438))]
        pub flow_tracking: ::validated_data::Field<item::FlowTracking<'a, Mode>>,
        #[data_view(schema = Str(1187))]
        pub qos_profile: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1188))]
        pub macsec_profile: ::validated_data::Field<&'a str>,
        #[data_view(schema = Dict(1440))]
        pub port_channel: ::validated_data::Field<item::PortChannel<'a, Mode>>,
        #[data_view(schema = List(574))]
        pub campus_link_type: ::validated_data::Field<item::CampusLinkType<'a, Mode>>,
        #[data_view(schema = Str(1192))]
        pub raw_eos_cli: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1193))]
        pub routing_protocol: ::validated_data::Field<&'a str>,
        #[data_view(schema = Dict(1441), relaxed)]
        pub ethernet_structured_config: ::validated_data::Field<super::super::super::eos_config::ethernet_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
        #[data_view(schema = Dict(1442), relaxed)]
        pub port_channel_structured_config: ::validated_data::Field<super::super::super::eos_config::port_channel_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
    }

    pub mod item {

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksItem.Nodes", schema = List(590)))]
        pub struct Nodes<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksItem.Ip", schema = List(580)))]
        pub struct Ip<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksItem.Ipv6", schema = List(581)))]
        pub struct Ipv6<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksItem.Interfaces", schema = List(583)))]
        pub struct Interfaces<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksItem.As", schema = List(584)))]
        pub struct FieldAs<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksItem.Descriptions", schema = List(585)))]
        pub struct Descriptions<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(python(name = "AVDDesign.L3Edge.P2pLinksItem.Ptp", schema = Dict(1437)))]
        pub struct Ptp<'a, Mode> {
            #[data_view(schema = Bool(443))]
            pub enabled: ::validated_data::Field<bool>,
            #[data_view(schema = List(586))]
            pub roles: ::validated_data::Field<ptp::Roles<'a, Mode>>,
            #[data_view(schema = Str(1154))]
            pub profile: ::validated_data::Field<&'a str>,
        }

        pub mod ptp {

            #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksItem.Ptp.Roles", schema = List(586)))]
            pub struct Roles<'a, Mode> (::validated_data::Field<&'a str>);
        }

        #[::validated_data::data_view(python(name = "AVDDesign.L3Edge.P2pLinksItem.FlowTracking", schema = Dict(1438)))]
        pub struct FlowTracking<'a, Mode> {
            #[data_view(schema = Bool(7))]
            pub enabled: ::validated_data::Field<bool>,
            #[data_view(schema = Str(1155))]
            pub name: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view(python(name = "AVDDesign.L3Edge.P2pLinksItem.PortChannel", schema = Dict(1440)))]
        pub struct PortChannel<'a, Mode> {
            #[data_view(schema = Str(1189))]
            pub description: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1190))]
            pub mode: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1191))]
            pub channel_id_algorithm: ::validated_data::Field<&'a str>,
            #[data_view(schema = Int(521))]
            pub channel_id_offset: ::validated_data::Field<i64>,
            #[data_view(schema = List(588))]
            pub nodes_child_interfaces: ::validated_data::Field<port_channel::NodesChildInterfaces<'a, Mode>>,
        }

        pub mod port_channel {

            #[::validated_data::data_view(indexed_list, primary_key(node), python(name = "AVDDesign.L3Edge.P2pLinksItem.PortChannel.NodesChildInterfaces", schema = List(588), item_name = "AVDDesign.L3Edge.P2pLinksItem.PortChannel.NodesChildInterfacesItem"))]
            pub struct NodesChildInterfaces<'a, Mode> (::validated_data::Field<nodes_child_interfaces::Item<'a, Mode>>);

            pub mod nodes_child_interfaces {

                #[::validated_data::data_view(python(name = "AVDDesign.L3Edge.P2pLinksItem.PortChannel.NodesChildInterfacesItemBase", schema = Dict(1439)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(3))]
                    pub node: ::validated_data::Field<&'a str>,
                    #[data_view(schema = List(587))]
                    pub interfaces: ::validated_data::Field<item::Interfaces<'a, Mode>>,
                    #[data_view(schema = Int(522))]
                    pub channel_id: ::validated_data::Field<i64>,
                }

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksItem.PortChannel.NodesChildInterfacesItemBase.Interfaces", schema = List(587)))]
                    pub struct Interfaces<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }
        }

        #[::validated_data::data_view(list, python(name = "AVDDesign.L3Edge.P2pLinksItem.CampusLinkType", schema = List(574)))]
        pub struct CampusLinkType<'a, Mode> (::validated_data::Field<&'a str>);
    }
}
