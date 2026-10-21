// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.CoreInterfaces", schema = Dict(1446)))]
pub struct CoreInterfaces<'a, Mode> {
    #[data_view(schema = List(579))]
    pub p2p_links_ip_pools: ::data_views::Field<P2pLinksIpPools<'a, Mode>>,
    #[data_view(schema = List(589))]
    pub p2p_links_profiles: ::data_views::Field<P2pLinksProfiles<'a, Mode>>,
    #[data_view(schema = List(591))]
    pub p2p_links: ::data_views::Field<P2pLinks<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.CoreInterfaces.P2pLinksIpPools", schema = List(579), item_name = "AVDDesign.CoreInterfaces.P2pLinksIpPoolsItem"))]
pub struct P2pLinksIpPools<'a, Mode> (::data_views::Field<p2p_links_ip_pools::Item<'a, Mode>>);

pub mod p2p_links_ip_pools {

    #[::data_views::data_view(python(name = "AVDDesign.CoreInterfaces.P2pLinksIpPoolsItemBase", schema = Dict(1436)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1169))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1170))]
        pub ipv4_pool: ::data_views::Field<&'a str>,
        #[data_view(schema = Int(517))]
        pub prefix_size: ::data_views::Field<i64>,
        #[data_view(schema = Str(1171))]
        pub ipv6_pool: ::data_views::Field<&'a str>,
        #[data_view(schema = Int(518))]
        pub ipv6_prefix_size: ::data_views::Field<i64>,
    }
}

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.CoreInterfaces.P2pLinksProfiles", schema = List(589), item_name = "AVDDesign.CoreInterfaces.P2pLinksProfilesItem"))]
pub struct P2pLinksProfiles<'a, Mode> (::data_views::Field<p2p_links_profiles::Item<'a, Mode>>);

pub mod p2p_links_profiles {

    #[::data_views::data_view(python(name = "AVDDesign.CoreInterfaces.P2pLinksProfilesItemBase", schema = Dict(1444)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1172))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = Int(519))]
        pub id: ::data_views::Field<i64>,
        #[data_view(schema = Str(178))]
        pub speed: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1173))]
        pub ip_pool: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1174))]
        pub subnet: ::data_views::Field<&'a str>,
        #[data_view(schema = List(580))]
        pub ip: ::data_views::Field<item::Ip<'a, Mode>>,
        #[data_view(schema = Bool(437))]
        pub ipv6_enable: ::data_views::Field<bool>,
        #[data_view(schema = Str(1176))]
        pub ipv6_prefix: ::data_views::Field<&'a str>,
        #[data_view(schema = List(581))]
        pub ipv6: ::data_views::Field<item::Ipv6<'a, Mode>>,
        #[data_view(schema = List(582))]
        pub nodes: ::data_views::Field<item::Nodes<'a, Mode>>,
        #[data_view(schema = List(583))]
        pub interfaces: ::data_views::Field<item::Interfaces<'a, Mode>>,
        #[data_view(schema = List(584), rename = "as")]
        pub field_as: ::data_views::Field<item::FieldAs<'a, Mode>>,
        #[data_view(schema = List(585))]
        pub descriptions: ::data_views::Field<item::Descriptions<'a, Mode>>,
        #[data_view(schema = Bool(438))]
        pub include_in_underlay_protocol: ::data_views::Field<bool>,
        #[data_view(schema = Bool(439))]
        pub use_underlay_ospf_authentication: ::data_views::Field<bool>,
        #[data_view(schema = Bool(37))]
        pub isis_hello_padding: ::data_views::Field<bool>,
        #[data_view(schema = Int(77))]
        pub isis_metric: ::data_views::Field<i64>,
        #[data_view(schema = Str(878))]
        pub isis_circuit_type: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1182))]
        pub isis_authentication_mode: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1183))]
        pub isis_authentication_key: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1184))]
        pub isis_authentication_cleartext_key: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1185))]
        pub isis_network_type: ::data_views::Field<&'a str>,
        #[data_view(schema = Bool(440))]
        pub mpls_ip: ::data_views::Field<bool>,
        #[data_view(schema = Bool(441))]
        pub mpls_ldp: ::data_views::Field<bool>,
        #[data_view(schema = Int(520))]
        pub mtu: ::data_views::Field<i64>,
        #[data_view(schema = Bool(442))]
        pub bfd: ::data_views::Field<bool>,
        #[data_view(schema = Dict(1437))]
        pub ptp: ::data_views::Field<item::Ptp<'a, Mode>>,
        #[data_view(schema = Bool(444))]
        pub sflow: ::data_views::Field<bool>,
        #[data_view(schema = Bool(445))]
        pub multicast_pim_sm: ::data_views::Field<bool>,
        #[data_view(schema = Bool(446))]
        pub multicast_static: ::data_views::Field<bool>,
        #[data_view(schema = Dict(1438))]
        pub flow_tracking: ::data_views::Field<item::FlowTracking<'a, Mode>>,
        #[data_view(schema = Str(1187))]
        pub qos_profile: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1188))]
        pub macsec_profile: ::data_views::Field<&'a str>,
        #[data_view(schema = Dict(1440))]
        pub port_channel: ::data_views::Field<item::PortChannel<'a, Mode>>,
        #[data_view(schema = List(574))]
        pub campus_link_type: ::data_views::Field<item::CampusLinkType<'a, Mode>>,
        #[data_view(schema = Str(1192))]
        pub raw_eos_cli: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1193))]
        pub routing_protocol: ::data_views::Field<&'a str>,
        #[data_view(schema = Dict(1441), relaxed)]
        pub ethernet_structured_config: ::data_views::Field<super::super::super::eos_config::ethernet_interfaces::Item<'a, ::data_views::RelaxedValidated>>,
        #[data_view(schema = Dict(1442), relaxed)]
        pub port_channel_structured_config: ::data_views::Field<super::super::super::eos_config::port_channel_interfaces::Item<'a, ::data_views::RelaxedValidated>>,
    }

    pub mod item {

        #[::data_views::data_view(list, python(name = "AVDDesign.CoreInterfaces.P2pLinksProfilesItemBase.Ip", schema = List(580)))]
        pub struct Ip<'a, Mode> (::data_views::Field<&'a str>);

        #[::data_views::data_view(list, python(name = "AVDDesign.CoreInterfaces.P2pLinksProfilesItemBase.Ipv6", schema = List(581)))]
        pub struct Ipv6<'a, Mode> (::data_views::Field<&'a str>);

        #[::data_views::data_view(list, python(name = "AVDDesign.CoreInterfaces.P2pLinksProfilesItemBase.Nodes", schema = List(582)))]
        pub struct Nodes<'a, Mode> (::data_views::Field<&'a str>);

        #[::data_views::data_view(list, python(name = "AVDDesign.CoreInterfaces.P2pLinksProfilesItemBase.Interfaces", schema = List(583)))]
        pub struct Interfaces<'a, Mode> (::data_views::Field<&'a str>);

        #[::data_views::data_view(list, python(name = "AVDDesign.CoreInterfaces.P2pLinksProfilesItemBase.As", schema = List(584)))]
        pub struct FieldAs<'a, Mode> (::data_views::Field<&'a str>);

        #[::data_views::data_view(list, python(name = "AVDDesign.CoreInterfaces.P2pLinksProfilesItemBase.Descriptions", schema = List(585)))]
        pub struct Descriptions<'a, Mode> (::data_views::Field<&'a str>);

        #[::data_views::data_view(python(name = "AVDDesign.CoreInterfaces.P2pLinksProfilesItemBase.Ptp", schema = Dict(1437)))]
        pub struct Ptp<'a, Mode> {
            #[data_view(schema = Bool(443))]
            pub enabled: ::data_views::Field<bool>,
            #[data_view(schema = List(586))]
            pub roles: ::data_views::Field<ptp::Roles<'a, Mode>>,
            #[data_view(schema = Str(1154))]
            pub profile: ::data_views::Field<&'a str>,
        }

        pub mod ptp {

            #[::data_views::data_view(list, python(name = "AVDDesign.CoreInterfaces.P2pLinksProfilesItemBase.Ptp.Roles", schema = List(586)))]
            pub struct Roles<'a, Mode> (::data_views::Field<&'a str>);
        }

        #[::data_views::data_view(python(name = "AVDDesign.CoreInterfaces.P2pLinksProfilesItemBase.FlowTracking", schema = Dict(1438)))]
        pub struct FlowTracking<'a, Mode> {
            #[data_view(schema = Bool(7))]
            pub enabled: ::data_views::Field<bool>,
            #[data_view(schema = Str(1155))]
            pub name: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view(python(name = "AVDDesign.CoreInterfaces.P2pLinksProfilesItemBase.PortChannel", schema = Dict(1440)))]
        pub struct PortChannel<'a, Mode> {
            #[data_view(schema = Str(1189))]
            pub description: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1190))]
            pub mode: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1191))]
            pub channel_id_algorithm: ::data_views::Field<&'a str>,
            #[data_view(schema = Int(521))]
            pub channel_id_offset: ::data_views::Field<i64>,
            #[data_view(schema = List(588))]
            pub nodes_child_interfaces: ::data_views::Field<port_channel::NodesChildInterfaces<'a, Mode>>,
        }

        pub mod port_channel {

            #[::data_views::data_view(indexed_list, primary_key(node), python(name = "AVDDesign.CoreInterfaces.P2pLinksProfilesItemBase.PortChannel.NodesChildInterfaces", schema = List(588), item_name = "AVDDesign.CoreInterfaces.P2pLinksProfilesItemBase.PortChannel.NodesChildInterfacesItem"))]
            pub struct NodesChildInterfaces<'a, Mode> (::data_views::Field<nodes_child_interfaces::Item<'a, Mode>>);

            pub mod nodes_child_interfaces {

                #[::data_views::data_view(python(name = "AVDDesign.CoreInterfaces.P2pLinksProfilesItemBase.PortChannel.NodesChildInterfacesItemBase", schema = Dict(1439)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(3))]
                    pub node: ::data_views::Field<&'a str>,
                    #[data_view(schema = List(587))]
                    pub interfaces: ::data_views::Field<item::Interfaces<'a, Mode>>,
                    #[data_view(schema = Int(522))]
                    pub channel_id: ::data_views::Field<i64>,
                }

                pub mod item {

                    #[::data_views::data_view(list, python(name = "AVDDesign.CoreInterfaces.P2pLinksProfilesItemBase.PortChannel.NodesChildInterfacesItemBase.Interfaces", schema = List(587)))]
                    pub struct Interfaces<'a, Mode> (::data_views::Field<&'a str>);
                }
            }
        }

        #[::data_views::data_view(list, python(name = "AVDDesign.CoreInterfaces.P2pLinksProfilesItemBase.CampusLinkType", schema = List(574)))]
        pub struct CampusLinkType<'a, Mode> (::data_views::Field<&'a str>);
    }
}

#[::data_views::data_view(list, python(name = "AVDDesign.CoreInterfaces.P2pLinks", schema = List(591)))]
pub struct P2pLinks<'a, Mode> (::data_views::Field<p2p_links::Item<'a, Mode>>);

pub mod p2p_links {

    #[::data_views::data_view(python(name = "AVDDesign.CoreInterfaces.P2pLinksItem", schema = Dict(1445)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = List(590))]
        pub nodes: ::data_views::RequiredValue<item::Nodes<'a, Mode>, Mode>,
        #[data_view(schema = Str(1194))]
        pub profile: ::data_views::Field<&'a str>,
        #[data_view(schema = Int(519))]
        pub id: ::data_views::Field<i64>,
        #[data_view(schema = Str(178))]
        pub speed: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1173))]
        pub ip_pool: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1174))]
        pub subnet: ::data_views::Field<&'a str>,
        #[data_view(schema = List(580))]
        pub ip: ::data_views::Field<item::Ip<'a, Mode>>,
        #[data_view(schema = Bool(437))]
        pub ipv6_enable: ::data_views::Field<bool>,
        #[data_view(schema = Str(1176))]
        pub ipv6_prefix: ::data_views::Field<&'a str>,
        #[data_view(schema = List(581))]
        pub ipv6: ::data_views::Field<item::Ipv6<'a, Mode>>,
        #[data_view(schema = List(583))]
        pub interfaces: ::data_views::Field<item::Interfaces<'a, Mode>>,
        #[data_view(schema = List(584), rename = "as")]
        pub field_as: ::data_views::Field<item::FieldAs<'a, Mode>>,
        #[data_view(schema = List(585))]
        pub descriptions: ::data_views::Field<item::Descriptions<'a, Mode>>,
        #[data_view(schema = Bool(438))]
        pub include_in_underlay_protocol: ::data_views::Field<bool>,
        #[data_view(schema = Bool(439))]
        pub use_underlay_ospf_authentication: ::data_views::Field<bool>,
        #[data_view(schema = Bool(37))]
        pub isis_hello_padding: ::data_views::Field<bool>,
        #[data_view(schema = Int(77))]
        pub isis_metric: ::data_views::Field<i64>,
        #[data_view(schema = Str(878))]
        pub isis_circuit_type: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1182))]
        pub isis_authentication_mode: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1183))]
        pub isis_authentication_key: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1184))]
        pub isis_authentication_cleartext_key: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1185))]
        pub isis_network_type: ::data_views::Field<&'a str>,
        #[data_view(schema = Bool(440))]
        pub mpls_ip: ::data_views::Field<bool>,
        #[data_view(schema = Bool(441))]
        pub mpls_ldp: ::data_views::Field<bool>,
        #[data_view(schema = Int(520))]
        pub mtu: ::data_views::Field<i64>,
        #[data_view(schema = Bool(442))]
        pub bfd: ::data_views::Field<bool>,
        #[data_view(schema = Dict(1437))]
        pub ptp: ::data_views::Field<item::Ptp<'a, Mode>>,
        #[data_view(schema = Bool(444))]
        pub sflow: ::data_views::Field<bool>,
        #[data_view(schema = Bool(445))]
        pub multicast_pim_sm: ::data_views::Field<bool>,
        #[data_view(schema = Bool(446))]
        pub multicast_static: ::data_views::Field<bool>,
        #[data_view(schema = Dict(1438))]
        pub flow_tracking: ::data_views::Field<item::FlowTracking<'a, Mode>>,
        #[data_view(schema = Str(1187))]
        pub qos_profile: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1188))]
        pub macsec_profile: ::data_views::Field<&'a str>,
        #[data_view(schema = Dict(1440))]
        pub port_channel: ::data_views::Field<item::PortChannel<'a, Mode>>,
        #[data_view(schema = List(574))]
        pub campus_link_type: ::data_views::Field<item::CampusLinkType<'a, Mode>>,
        #[data_view(schema = Str(1192))]
        pub raw_eos_cli: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1193))]
        pub routing_protocol: ::data_views::Field<&'a str>,
        #[data_view(schema = Dict(1441), relaxed)]
        pub ethernet_structured_config: ::data_views::Field<super::super::super::eos_config::ethernet_interfaces::Item<'a, ::data_views::RelaxedValidated>>,
        #[data_view(schema = Dict(1442), relaxed)]
        pub port_channel_structured_config: ::data_views::Field<super::super::super::eos_config::port_channel_interfaces::Item<'a, ::data_views::RelaxedValidated>>,
    }

    pub mod item {

        #[::data_views::data_view(list, python(name = "AVDDesign.CoreInterfaces.P2pLinksItem.Nodes", schema = List(590)))]
        pub struct Nodes<'a, Mode> (::data_views::Field<&'a str>);

        #[::data_views::data_view(list, python(name = "AVDDesign.CoreInterfaces.P2pLinksItem.Ip", schema = List(580)))]
        pub struct Ip<'a, Mode> (::data_views::Field<&'a str>);

        #[::data_views::data_view(list, python(name = "AVDDesign.CoreInterfaces.P2pLinksItem.Ipv6", schema = List(581)))]
        pub struct Ipv6<'a, Mode> (::data_views::Field<&'a str>);

        #[::data_views::data_view(list, python(name = "AVDDesign.CoreInterfaces.P2pLinksItem.Interfaces", schema = List(583)))]
        pub struct Interfaces<'a, Mode> (::data_views::Field<&'a str>);

        #[::data_views::data_view(list, python(name = "AVDDesign.CoreInterfaces.P2pLinksItem.As", schema = List(584)))]
        pub struct FieldAs<'a, Mode> (::data_views::Field<&'a str>);

        #[::data_views::data_view(list, python(name = "AVDDesign.CoreInterfaces.P2pLinksItem.Descriptions", schema = List(585)))]
        pub struct Descriptions<'a, Mode> (::data_views::Field<&'a str>);

        #[::data_views::data_view(python(name = "AVDDesign.CoreInterfaces.P2pLinksItem.Ptp", schema = Dict(1437)))]
        pub struct Ptp<'a, Mode> {
            #[data_view(schema = Bool(443))]
            pub enabled: ::data_views::Field<bool>,
            #[data_view(schema = List(586))]
            pub roles: ::data_views::Field<ptp::Roles<'a, Mode>>,
            #[data_view(schema = Str(1154))]
            pub profile: ::data_views::Field<&'a str>,
        }

        pub mod ptp {

            #[::data_views::data_view(list, python(name = "AVDDesign.CoreInterfaces.P2pLinksItem.Ptp.Roles", schema = List(586)))]
            pub struct Roles<'a, Mode> (::data_views::Field<&'a str>);
        }

        #[::data_views::data_view(python(name = "AVDDesign.CoreInterfaces.P2pLinksItem.FlowTracking", schema = Dict(1438)))]
        pub struct FlowTracking<'a, Mode> {
            #[data_view(schema = Bool(7))]
            pub enabled: ::data_views::Field<bool>,
            #[data_view(schema = Str(1155))]
            pub name: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view(python(name = "AVDDesign.CoreInterfaces.P2pLinksItem.PortChannel", schema = Dict(1440)))]
        pub struct PortChannel<'a, Mode> {
            #[data_view(schema = Str(1189))]
            pub description: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1190))]
            pub mode: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1191))]
            pub channel_id_algorithm: ::data_views::Field<&'a str>,
            #[data_view(schema = Int(521))]
            pub channel_id_offset: ::data_views::Field<i64>,
            #[data_view(schema = List(588))]
            pub nodes_child_interfaces: ::data_views::Field<port_channel::NodesChildInterfaces<'a, Mode>>,
        }

        pub mod port_channel {

            #[::data_views::data_view(indexed_list, primary_key(node), python(name = "AVDDesign.CoreInterfaces.P2pLinksItem.PortChannel.NodesChildInterfaces", schema = List(588), item_name = "AVDDesign.CoreInterfaces.P2pLinksItem.PortChannel.NodesChildInterfacesItem"))]
            pub struct NodesChildInterfaces<'a, Mode> (::data_views::Field<nodes_child_interfaces::Item<'a, Mode>>);

            pub mod nodes_child_interfaces {

                #[::data_views::data_view(python(name = "AVDDesign.CoreInterfaces.P2pLinksItem.PortChannel.NodesChildInterfacesItemBase", schema = Dict(1439)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(3))]
                    pub node: ::data_views::Field<&'a str>,
                    #[data_view(schema = List(587))]
                    pub interfaces: ::data_views::Field<item::Interfaces<'a, Mode>>,
                    #[data_view(schema = Int(522))]
                    pub channel_id: ::data_views::Field<i64>,
                }

                pub mod item {

                    #[::data_views::data_view(list, python(name = "AVDDesign.CoreInterfaces.P2pLinksItem.PortChannel.NodesChildInterfacesItemBase.Interfaces", schema = List(587)))]
                    pub struct Interfaces<'a, Mode> (::data_views::Field<&'a str>);
                }
            }
        }

        #[::data_views::data_view(list, python(name = "AVDDesign.CoreInterfaces.P2pLinksItem.CampusLinkType", schema = List(574)))]
        pub struct CampusLinkType<'a, Mode> (::data_views::Field<&'a str>);
    }
}
