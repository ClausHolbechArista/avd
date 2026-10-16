// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase", schema = Dict(1521)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1373))]
    pub profile: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1238), rename = "type")]
    pub field_type: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1239))]
    pub mlag_group: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1374))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(643))]
    pub downlink_pools: ::validated_data::Field<item::DownlinkPools<'a, Mode>>,
    #[data_view(schema = Int(530))]
    pub id: ::validated_data::Field<i64>,
    #[data_view(schema = Str(1240))]
    pub platform: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1241))]
    pub mac_address: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1242))]
    pub system_mac_address: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1243))]
    pub custom_system_mac_address: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1244))]
    pub serial_number: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1245))]
    pub rack: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1246))]
    pub mgmt_ip: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1247))]
    pub mgmt_gateway: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1248))]
    pub ipv6_mgmt_ip: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1249))]
    pub ipv6_mgmt_gateway: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1250))]
    pub mgmt_interface: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1467))]
    pub link_tracking: ::validated_data::Field<item::LinkTracking<'a, Mode>>,
    #[data_view(schema = Dict(1468))]
    pub lacp_port_id_range: ::validated_data::Field<item::LacpPortIdRange<'a, Mode>>,
    #[data_view(schema = Bool(457))]
    pub always_configure_ip_routing: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1253))]
    pub raw_eos_cli: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1469), relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_config::EosConfig<'a, ::validated_data::RelaxedValidated>>,
    #[data_view(schema = Str(1254))]
    pub uplink_type: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1255))]
    pub uplink_ipv4_pool: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1256))]
    pub uplink_ipv6_pool: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(613))]
    pub uplink_interfaces: ::validated_data::Field<item::UplinkInterfaces<'a, Mode>>,
    #[data_view(schema = List(614))]
    pub uplink_switch_interfaces: ::validated_data::Field<item::UplinkSwitchInterfaces<'a, Mode>>,
    #[data_view(schema = List(615))]
    pub uplink_switches: ::validated_data::Field<item::UplinkSwitches<'a, Mode>>,
    #[data_view(schema = Str(1260))]
    pub uplink_interface_speed: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1261))]
    pub uplink_switch_interface_speed: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(535))]
    pub uplink_mtu: ::validated_data::Field<i64>,
    #[data_view(schema = Int(536))]
    pub max_uplink_switches: ::validated_data::Field<i64>,
    #[data_view(schema = Int(537))]
    pub max_parallel_uplinks: ::validated_data::Field<i64>,
    #[data_view(schema = Bool(458))]
    pub uplink_bfd: ::validated_data::Field<bool>,
    #[data_view(schema = Int(538))]
    pub uplink_native_vlan: ::validated_data::Field<i64>,
    #[data_view(schema = Dict(1470))]
    pub uplink_ptp: ::validated_data::Field<item::UplinkPtp<'a, Mode>>,
    #[data_view(schema = Dict(1471))]
    pub uplink_macsec: ::validated_data::Field<item::UplinkMacsec<'a, Mode>>,
    #[data_view(schema = Int(539))]
    pub uplink_port_channel_id: ::validated_data::Field<i64>,
    #[data_view(schema = Int(540))]
    pub uplink_switch_port_channel_id: ::validated_data::Field<i64>,
    #[data_view(schema = Dict(1472), relaxed)]
    pub uplink_ethernet_structured_config: ::validated_data::Field<super::super::eos_config::ethernet_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
    #[data_view(schema = Dict(1473), relaxed)]
    pub uplink_port_channel_structured_config: ::validated_data::Field<super::super::eos_config::port_channel_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
    #[data_view(schema = Dict(1474), relaxed)]
    pub uplink_switch_ethernet_structured_config: ::validated_data::Field<super::super::eos_config::ethernet_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
    #[data_view(schema = Dict(1475), relaxed)]
    pub uplink_switch_port_channel_structured_config: ::validated_data::Field<super::super::eos_config::port_channel_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
    #[data_view(schema = Dict(1476), relaxed)]
    pub mlag_port_channel_structured_config: ::validated_data::Field<super::super::eos_config::port_channel_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
    #[data_view(schema = Dict(1477), relaxed)]
    pub mlag_peer_vlan_structured_config: ::validated_data::Field<super::super::eos_config::vlan_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
    #[data_view(schema = Dict(1478), relaxed)]
    pub mlag_peer_l3_vlan_structured_config: ::validated_data::Field<super::super::eos_config::vlan_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
    #[data_view(schema = Str(1262))]
    pub short_esi: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1263))]
    pub isis_system_id_prefix: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(541))]
    pub isis_maximum_paths: ::validated_data::Field<i64>,
    #[data_view(schema = Str(1264))]
    pub is_type: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(542))]
    pub node_sid_base: ::validated_data::Field<i64>,
    #[data_view(schema = Dict(1479))]
    pub isis_sr: ::validated_data::Field<item::IsisSr<'a, Mode>>,
    #[data_view(schema = Str(1265))]
    pub loopback_ipv4_pool: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1266))]
    pub loopback_ipv4_address: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1267))]
    pub vtep_loopback_ipv4_pool: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1268))]
    pub vtep_loopback_ipv6_pool: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1269))]
    pub vtep_loopback_ipv4_address: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1270))]
    pub vtep_loopback_ipv6_address: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(547))]
    pub loopback_ipv4_offset: ::validated_data::Field<i64>,
    #[data_view(schema = Str(1271))]
    pub router_id_pool: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1272))]
    pub loopback_ipv6_pool: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(548))]
    pub loopback_ipv6_offset: ::validated_data::Field<i64>,
    #[data_view(schema = Bool(459))]
    pub vtep: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1273))]
    pub vtep_loopback: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1274))]
    pub bgp_as: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(616))]
    pub bgp_defaults: ::validated_data::Field<item::BgpDefaults<'a, Mode>>,
    #[data_view(schema = Str(1275))]
    pub evpn_role: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(617))]
    pub evpn_route_servers: ::validated_data::Field<item::EvpnRouteServers<'a, Mode>>,
    #[data_view(schema = Bool(460))]
    pub evpn_services_l2_only: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1480))]
    pub filter: ::validated_data::Field<item::Filter<'a, Mode>>,
    #[data_view(schema = Bool(462))]
    pub igmp_snooping_enabled: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1487))]
    pub evpn_gateway: ::validated_data::Field<item::EvpnGateway<'a, Mode>>,
    #[data_view(schema = Dict(1489))]
    pub ipvpn_gateway: ::validated_data::Field<item::IpvpnGateway<'a, Mode>>,
    #[data_view(schema = Bool(466))]
    pub mlag: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(467))]
    pub mlag_dual_primary_detection: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(468))]
    pub mlag_ibgp_origin_incomplete: ::validated_data::Field<bool>,
    #[data_view(schema = List(626))]
    pub mlag_interfaces: ::validated_data::Field<item::MlagInterfaces<'a, Mode>>,
    #[data_view(schema = Str(1291))]
    pub mlag_interfaces_speed: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(550))]
    pub mlag_peer_l3_vlan: ::validated_data::Field<i64>,
    #[data_view(schema = Str(1292))]
    pub mlag_peer_l3_ipv4_pool: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1293))]
    pub mlag_peer_l3_ipv6_pool: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(551))]
    pub mlag_peer_vlan: ::validated_data::Field<i64>,
    #[data_view(schema = Str(1294))]
    pub mlag_peer_link_allowed_vlans: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1295))]
    pub mlag_peer_address_family: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1296))]
    pub mlag_peer_ipv4_pool: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1297))]
    pub mlag_peer_ipv6_pool: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(552))]
    pub mlag_port_channel_id: ::validated_data::Field<i64>,
    #[data_view(schema = Str(1298))]
    pub mlag_domain_id: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1299))]
    pub spanning_tree_mode: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(553))]
    pub spanning_tree_priority: ::validated_data::Field<i64>,
    #[data_view(schema = Bool(469))]
    pub spanning_tree_root_super: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(470))]
    pub spanning_tree_mst_pvst_boundary: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1490))]
    pub spanning_tree_port_id_allocation_port_channel_range: ::validated_data::Field<super::super::eos_config::spanning_tree::PortIdAllocationPortChannelRange<'a, Mode>>,
    #[data_view(schema = Str(1300))]
    pub virtual_router_mac_address: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1301))]
    pub inband_mgmt_interface: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(554))]
    pub inband_mgmt_vlan: ::validated_data::Field<i64>,
    #[data_view(schema = Str(1302))]
    pub inband_mgmt_subnet: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(555))]
    pub inband_mgmt_subnet_offset: ::validated_data::Field<i64>,
    #[data_view(schema = Str(1303))]
    pub inband_mgmt_ip: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1304))]
    pub inband_mgmt_gateway: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1305))]
    pub inband_mgmt_ipv6_address: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1306))]
    pub inband_mgmt_ipv6_subnet: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1307))]
    pub inband_mgmt_ipv6_gateway: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1308))]
    pub inband_mgmt_description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1309))]
    pub inband_mgmt_vlan_name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1310))]
    pub inband_mgmt_vrf: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(556))]
    pub inband_mgmt_mtu: ::validated_data::Field<i64>,
    #[data_view(schema = Bool(471))]
    pub inband_ztp: ::validated_data::Field<bool>,
    #[data_view(schema = Int(557))]
    pub inband_ztp_lacp_fallback_delay: ::validated_data::Field<i64>,
    #[data_view(schema = Str(1311))]
    pub mpls_overlay_role: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(627))]
    pub overlay_address_families: ::validated_data::Field<item::OverlayAddressFamilies<'a, Mode>>,
    #[data_view(schema = List(628))]
    pub mpls_route_reflectors: ::validated_data::Field<item::MplsRouteReflectors<'a, Mode>>,
    #[data_view(schema = Str(1313))]
    pub bgp_cluster_id: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(472))]
    pub kernel_ecmp_cli: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1496))]
    pub ptp: ::validated_data::Field<item::Ptp<'a, Mode>>,
    #[data_view(schema = Str(1320))]
    pub wan_role: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1321))]
    pub cv_pathfinder_transit_mode: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1322))]
    pub cv_pathfinder_region: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1323))]
    pub cv_pathfinder_site: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1498))]
    pub wan_ha: ::validated_data::Field<item::WanHa<'a, Mode>>,
    #[data_view(schema = Str(1325))]
    pub dps_mss_ipv4: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(635))]
    pub l3_interfaces: ::validated_data::Field<item::L3Interfaces<'a, Mode>>,
    #[data_view(schema = List(638))]
    pub l3_port_channels: ::validated_data::Field<item::L3PortChannels<'a, Mode>>,
    #[data_view(schema = Int(569))]
    pub data_plane_cpu_allocation_max: ::validated_data::Field<i64>,
    #[data_view(schema = Str(1364))]
    pub flow_tracker_type: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1516))]
    pub underlay_multicast: ::validated_data::Field<item::UnderlayMulticast<'a, Mode>>,
    #[data_view(schema = Str(1365))]
    pub campus: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1366))]
    pub campus_pod: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1367))]
    pub campus_access_pod: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1368))]
    pub cv_tags_topology_type: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1517))]
    pub digital_twin: ::validated_data::Field<item::DigitalTwin<'a, Mode>>,
    #[data_view(schema = Str(1372))]
    pub validation_profile: ::validated_data::Field<&'a str>,
}

pub mod item {

    #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.DownlinkPools", schema = List(643)))]
    pub struct DownlinkPools<'a, Mode> (::validated_data::Field<downlink_pools::Item<'a, Mode>>);

    pub mod downlink_pools {

        #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.DownlinkPoolsItem", schema = Dict(1520)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(1375))]
            pub ipv4_pool: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1376))]
            pub ipv6_pool: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(642))]
            pub downlink_interfaces: ::validated_data::Field<item::DownlinkInterfaces<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.DownlinkPoolsItem.DownlinkInterfaces", schema = List(642)))]
            pub struct DownlinkInterfaces<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.LinkTracking", schema = Dict(1467)))]
    pub struct LinkTracking<'a, Mode> {
        #[data_view(schema = Bool(340))]
        pub enabled: ::validated_data::Field<bool>,
        #[data_view(schema = Dict(1465))]
        pub downlinks: ::validated_data::Field<link_tracking::Downlinks<'a, Mode>>,
        #[data_view(schema = List(612))]
        pub groups: ::validated_data::Field<link_tracking::Groups<'a, Mode>>,
    }

    pub mod link_tracking {

        #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.LinkTracking.Downlinks", schema = Dict(1465)))]
        pub struct Downlinks<'a, Mode> {
            #[data_view(schema = Bool(340))]
            pub enabled: ::validated_data::Field<bool>,
            #[data_view(schema = Str(1251))]
            pub group: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.DevicesItemBase.LinkTracking.Groups", schema = List(612), item_name = "AVDDesign.DevicesItemBase.LinkTracking.GroupsItem"))]
        pub struct Groups<'a, Mode> (::validated_data::Field<groups::Item<'a, Mode>>);

        pub mod groups {

            #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.LinkTracking.GroupsItemBase", schema = Dict(1466)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(1252))]
                pub name: ::validated_data::Field<&'a str>,
                #[data_view(schema = Int(531))]
                pub recovery_delay: ::validated_data::Field<i64>,
                #[data_view(schema = Int(532))]
                pub links_minimum: ::validated_data::Field<i64>,
            }
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.LacpPortIdRange", schema = Dict(1468)))]
    pub struct LacpPortIdRange<'a, Mode> {
        #[data_view(schema = Bool(340))]
        pub enabled: ::validated_data::Field<bool>,
        #[data_view(schema = Int(533))]
        pub size: ::validated_data::Field<i64>,
        #[data_view(schema = Int(534))]
        pub offset: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.UplinkInterfaces", schema = List(613)))]
    pub struct UplinkInterfaces<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.UplinkSwitchInterfaces", schema = List(614)))]
    pub struct UplinkSwitchInterfaces<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.UplinkSwitches", schema = List(615)))]
    pub struct UplinkSwitches<'a, Mode> (::validated_data::RequiredValue<&'a str, Mode>);

    #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.UplinkPtp", schema = Dict(1470)))]
    pub struct UplinkPtp<'a, Mode> {
        #[data_view(schema = Bool(340))]
        pub enable: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.UplinkMacsec", schema = Dict(1471)))]
    pub struct UplinkMacsec<'a, Mode> {
        #[data_view(schema = Str(3))]
        pub profile: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.IsisSr", schema = Dict(1479)))]
    pub struct IsisSr<'a, Mode> {
        #[data_view(schema = Int(543))]
        pub ipv4_node_sid_index: ::validated_data::Field<i64>,
        #[data_view(schema = Int(544))]
        pub ipv4_node_sid_index_base: ::validated_data::Field<i64>,
        #[data_view(schema = Int(545))]
        pub ipv6_node_sid_index: ::validated_data::Field<i64>,
        #[data_view(schema = Int(546))]
        pub ipv6_node_sid_index_base: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.BgpDefaults", schema = List(616)))]
    pub struct BgpDefaults<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.EvpnRouteServers", schema = List(617)))]
    pub struct EvpnRouteServers<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.Filter", schema = Dict(1480)))]
    pub struct Filter<'a, Mode> {
        #[data_view(schema = List(618))]
        pub tenants: ::validated_data::Field<filter::Tenants<'a, Mode>>,
        #[data_view(schema = List(619))]
        pub tags: ::validated_data::Field<filter::Tags<'a, Mode>>,
        #[data_view(schema = List(620))]
        pub allow_vrfs: ::validated_data::Field<filter::AllowVrfs<'a, Mode>>,
        #[data_view(schema = List(621))]
        pub deny_vrfs: ::validated_data::Field<filter::DenyVrfs<'a, Mode>>,
        #[data_view(schema = List(622))]
        pub always_include_vrfs_in_tenants: ::validated_data::Field<filter::AlwaysIncludeVrfsInTenants<'a, Mode>>,
        #[data_view(schema = Bool(461))]
        pub only_vlans_in_use: ::validated_data::Field<bool>,
    }

    pub mod filter {

        #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.Filter.Tenants", schema = List(618)))]
        pub struct Tenants<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.Filter.Tags", schema = List(619)))]
        pub struct Tags<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.Filter.AllowVrfs", schema = List(620)))]
        pub struct AllowVrfs<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.Filter.DenyVrfs", schema = List(621)))]
        pub struct DenyVrfs<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.Filter.AlwaysIncludeVrfsInTenants", schema = List(622)))]
        pub struct AlwaysIncludeVrfsInTenants<'a, Mode> (::validated_data::Field<&'a str>);
    }

    #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.EvpnGateway", schema = Dict(1487)))]
    pub struct EvpnGateway<'a, Mode> {
        #[data_view(schema = List(623))]
        pub remote_peers: ::validated_data::Field<evpn_gateway::RemotePeers<'a, Mode>>,
        #[data_view(schema = Dict(1482))]
        pub evpn_l2: ::validated_data::Field<evpn_gateway::EvpnL2<'a, Mode>>,
        #[data_view(schema = Dict(1483))]
        pub evpn_l3: ::validated_data::Field<evpn_gateway::EvpnL3<'a, Mode>>,
        #[data_view(schema = Dict(1484))]
        pub d_path: ::validated_data::Field<evpn_gateway::DPath<'a, Mode>>,
        #[data_view(schema = Dict(1486))]
        pub all_active_multihoming: ::validated_data::Field<evpn_gateway::AllActiveMultihoming<'a, Mode>>,
    }

    pub mod evpn_gateway {

        #[::validated_data::data_view(indexed_list, primary_key(hostname), python(name = "AVDDesign.DevicesItemBase.EvpnGateway.RemotePeers", schema = List(623), item_name = "AVDDesign.DevicesItemBase.EvpnGateway.RemotePeersItem"))]
        pub struct RemotePeers<'a, Mode> (::validated_data::Field<remote_peers::Item<'a, Mode>>);

        pub mod remote_peers {

            #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.EvpnGateway.RemotePeersItemBase", schema = Dict(1481)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(1276))]
                pub hostname: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(1277))]
                pub ip_address: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(1278))]
                pub bgp_as: ::validated_data::Field<&'a str>,
            }
        }

        #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.EvpnGateway.EvpnL2", schema = Dict(1482)))]
        pub struct EvpnL2<'a, Mode> {
            #[data_view(schema = Bool(340))]
            pub enabled: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.EvpnGateway.EvpnL3", schema = Dict(1483)))]
        pub struct EvpnL3<'a, Mode> {
            #[data_view(schema = Bool(340))]
            pub enabled: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(37))]
            pub inter_domain: ::validated_data::Field<bool>,
            #[data_view(schema = Str(1279))]
            pub mode: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.EvpnGateway.DPath", schema = Dict(1484)))]
        pub struct DPath<'a, Mode> {
            #[data_view(schema = Bool(463))]
            pub enabled: ::validated_data::Field<bool>,
            #[data_view(schema = Str(1280))]
            pub local_domain_id: ::validated_data::RequiredValue<&'a str, Mode>,
            #[data_view(schema = Str(1280))]
            pub remote_domain_id: ::validated_data::RequiredValue<&'a str, Mode>,
        }

        #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.EvpnGateway.AllActiveMultihoming", schema = Dict(1486)))]
        pub struct AllActiveMultihoming<'a, Mode> {
            #[data_view(schema = Bool(464))]
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            #[data_view(schema = Bool(465))]
            pub enable_d_path: ::validated_data::Field<bool>,
            #[data_view(schema = Str(1281))]
            pub evpn_domain_id_local: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1282))]
            pub evpn_domain_id_remote: ::validated_data::Field<&'a str>,
            #[data_view(schema = Dict(1485))]
            pub evpn_ethernet_segment: ::validated_data::RequiredValue<all_active_multihoming::EvpnEthernetSegment<'a, Mode>, Mode>,
        }

        pub mod all_active_multihoming {

            #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.EvpnGateway.AllActiveMultihoming.EvpnEthernetSegment", schema = Dict(1485)))]
            pub struct EvpnEthernetSegment<'a, Mode> {
                #[data_view(schema = Str(1283))]
                pub identifier: ::validated_data::RequiredValue<&'a str, Mode>,
                #[data_view(schema = Str(1284))]
                pub rt_import: ::validated_data::RequiredValue<&'a str, Mode>,
            }
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.IpvpnGateway", schema = Dict(1489)))]
    pub struct IpvpnGateway<'a, Mode> {
        #[data_view(schema = Bool(1))]
        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
        #[data_view(schema = Str(1285))]
        pub evpn_domain_id: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1286))]
        pub ipvpn_domain_id: ::validated_data::Field<&'a str>,
        #[data_view(schema = Bool(463))]
        pub enable_d_path: ::validated_data::Field<bool>,
        #[data_view(schema = Int(549))]
        pub maximum_routes: ::validated_data::Field<i64>,
        #[data_view(schema = Str(1287))]
        pub local_as: ::validated_data::Field<&'a str>,
        #[data_view(schema = List(624))]
        pub address_families: ::validated_data::Field<ipvpn_gateway::AddressFamilies<'a, Mode>>,
        #[data_view(schema = List(625))]
        pub remote_peers: ::validated_data::Field<ipvpn_gateway::RemotePeers<'a, Mode>>,
    }

    pub mod ipvpn_gateway {

        #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.IpvpnGateway.AddressFamilies", schema = List(624)))]
        pub struct AddressFamilies<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(indexed_list, primary_key(hostname), python(name = "AVDDesign.DevicesItemBase.IpvpnGateway.RemotePeers", schema = List(625), item_name = "AVDDesign.DevicesItemBase.IpvpnGateway.RemotePeersItem"))]
        pub struct RemotePeers<'a, Mode> (::validated_data::Field<remote_peers::Item<'a, Mode>>);

        pub mod remote_peers {

            #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.IpvpnGateway.RemotePeersItemBase", schema = Dict(1488)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(1288))]
                pub hostname: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(1289))]
                pub ip_address: ::validated_data::RequiredValue<&'a str, Mode>,
                #[data_view(schema = Str(1290))]
                pub bgp_as: ::validated_data::RequiredValue<&'a str, Mode>,
            }
        }
    }

    #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.MlagInterfaces", schema = List(626)))]
    pub struct MlagInterfaces<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.OverlayAddressFamilies", schema = List(627)))]
    pub struct OverlayAddressFamilies<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.MplsRouteReflectors", schema = List(628)))]
    pub struct MplsRouteReflectors<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.Ptp", schema = Dict(1496)))]
    pub struct Ptp<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub enabled: ::validated_data::Field<bool>,
        #[data_view(schema = Str(1314))]
        pub profile: ::validated_data::Field<&'a str>,
        #[data_view(schema = List(629))]
        pub uplinks: ::validated_data::Field<ptp::Uplinks<'a, Mode>>,
        #[data_view(schema = Bool(473))]
        pub mlag: ::validated_data::Field<bool>,
        #[data_view(schema = Int(105))]
        pub domain: ::validated_data::Field<i64>,
        #[data_view(schema = Int(558))]
        pub priority1: ::validated_data::Field<i64>,
        #[data_view(schema = Int(559))]
        pub priority2: ::validated_data::Field<i64>,
        #[data_view(schema = Bool(474))]
        pub auto_clock_identity: ::validated_data::Field<bool>,
        #[data_view(schema = Str(1316))]
        pub clock_identity_prefix: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1317))]
        pub clock_identity: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1318))]
        pub source_ip: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1319))]
        pub mode: ::validated_data::Field<&'a str>,
        #[data_view(schema = Bool(340))]
        pub mode_one_step: ::validated_data::Field<bool>,
        #[data_view(schema = Int(77))]
        pub ttl: ::validated_data::Field<i64>,
        #[data_view(schema = Bool(475))]
        pub forward_unicast: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(294))]
        pub forward_v1: ::validated_data::Field<bool>,
        #[data_view(schema = Dict(818))]
        pub free_running: ::validated_data::Field<super::super::super::eos_config::ptp::FreeRunning<'a, Mode>>,
        #[data_view(schema = Dict(1491))]
        pub dscp: ::validated_data::Field<ptp::Dscp<'a, Mode>>,
        #[data_view(schema = Dict(1495))]
        pub monitor: ::validated_data::Field<ptp::Monitor<'a, Mode>>,
    }

    pub mod ptp {

        #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.Ptp.Uplinks", schema = List(629)))]
        pub struct Uplinks<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.Ptp.Dscp", schema = Dict(1491)))]
        pub struct Dscp<'a, Mode> {
            #[data_view(schema = Int(77))]
            pub general_messages: ::validated_data::Field<i64>,
            #[data_view(schema = Int(77))]
            pub event_messages: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.Ptp.Monitor", schema = Dict(1495)))]
        pub struct Monitor<'a, Mode> {
            #[data_view(schema = Bool(37))]
            pub enabled: ::validated_data::Field<bool>,
            #[data_view(schema = Dict(1492))]
            pub threshold: ::validated_data::Field<monitor::Threshold<'a, Mode>>,
            #[data_view(schema = Dict(1494))]
            pub missing_message: ::validated_data::Field<monitor::MissingMessage<'a, Mode>>,
        }

        pub mod monitor {

            #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.Ptp.Monitor.Threshold", schema = Dict(1492)))]
            pub struct Threshold<'a, Mode> {
                #[data_view(schema = Int(560))]
                pub offset_from_master: ::validated_data::Field<i64>,
                #[data_view(schema = Int(561))]
                pub mean_path_delay: ::validated_data::Field<i64>,
                #[data_view(schema = Dict(812))]
                pub drop: ::validated_data::Field<threshold::Drop<'a, Mode>>,
            }

            pub mod threshold {

                #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.Ptp.Monitor.Threshold.Drop", schema = Dict(812)))]
                pub struct Drop<'a, Mode> {
                    #[data_view(schema = Int(327))]
                    pub offset_from_master: ::validated_data::Field<i64>,
                    #[data_view(schema = Int(327))]
                    pub mean_path_delay: ::validated_data::Field<i64>,
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.Ptp.Monitor.MissingMessage", schema = Dict(1494)))]
            pub struct MissingMessage<'a, Mode> {
                #[data_view(schema = Dict(814))]
                pub intervals: ::validated_data::Field<missing_message::Intervals<'a, Mode>>,
                #[data_view(schema = Dict(1493))]
                pub sequence_ids: ::validated_data::Field<missing_message::SequenceIds<'a, Mode>>,
            }

            pub mod missing_message {

                #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.Ptp.Monitor.MissingMessage.Intervals", schema = Dict(814)))]
                pub struct Intervals<'a, Mode> {
                    #[data_view(schema = Int(328))]
                    pub announce: ::validated_data::Field<i64>,
                    #[data_view(schema = Int(328))]
                    pub follow_up: ::validated_data::Field<i64>,
                    #[data_view(schema = Int(328))]
                    pub sync: ::validated_data::Field<i64>,
                }

                #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.Ptp.Monitor.MissingMessage.SequenceIds", schema = Dict(1493)))]
                pub struct SequenceIds<'a, Mode> {
                    #[data_view(schema = Bool(37))]
                    pub enabled: ::validated_data::Field<bool>,
                    #[data_view(schema = Int(562))]
                    pub announce: ::validated_data::Field<i64>,
                    #[data_view(schema = Int(562))]
                    pub delay_resp: ::validated_data::Field<i64>,
                    #[data_view(schema = Int(562))]
                    pub follow_up: ::validated_data::Field<i64>,
                    #[data_view(schema = Int(562))]
                    pub sync: ::validated_data::Field<i64>,
                }
            }
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.WanHa", schema = Dict(1498)))]
    pub struct WanHa<'a, Mode> {
        #[data_view(schema = Bool(476))]
        pub enabled: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(477))]
        pub ipsec: ::validated_data::Field<bool>,
        #[data_view(schema = Int(563))]
        pub mtu: ::validated_data::Field<i64>,
        #[data_view(schema = List(630))]
        pub ha_interfaces: ::validated_data::Field<wan_ha::HaInterfaces<'a, Mode>>,
        #[data_view(schema = Str(1324))]
        pub ha_ipv4_pool: ::validated_data::Field<&'a str>,
        #[data_view(schema = Int(564))]
        pub port_channel_id: ::validated_data::Field<i64>,
        #[data_view(schema = Bool(478))]
        pub use_port_channel_for_direct_ha: ::validated_data::Field<bool>,
        #[data_view(schema = Dict(1497))]
        pub flow_tracking: ::validated_data::Field<wan_ha::FlowTracking<'a, Mode>>,
    }

    pub mod wan_ha {

        #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.WanHa.HaInterfaces", schema = List(630)))]
        pub struct HaInterfaces<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.WanHa.FlowTracking", schema = Dict(1497)))]
        pub struct FlowTracking<'a, Mode> {
            #[data_view(schema = Bool(7))]
            pub enabled: ::validated_data::Field<bool>,
            #[data_view(schema = Str(1155))]
            pub name: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.DevicesItemBase.L3Interfaces", schema = List(635), item_name = "AVDDesign.DevicesItemBase.L3InterfacesItem"))]
    pub struct L3Interfaces<'a, Mode> (::validated_data::Field<l3_interfaces::Item<'a, Mode>>);

    pub mod l3_interfaces {

        #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.L3InterfacesItemBase", schema = Dict(1506)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(1326))]
            pub profile: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1327))]
            pub name: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1328))]
            pub description: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1329))]
            pub ip_address: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(631))]
            pub ipv6_addresses: ::validated_data::Field<item::Ipv6Addresses<'a, Mode>>,
            #[data_view(schema = Str(1331))]
            pub dhcp_ip: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1332))]
            pub public_ip: ::validated_data::Field<&'a str>,
            #[data_view(schema = Int(565))]
            pub encapsulation_dot1q_vlan: ::validated_data::Field<i64>,
            #[data_view(schema = Bool(479))]
            pub dhcp_accept_default_route: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(480))]
            pub enabled: ::validated_data::Field<bool>,
            #[data_view(schema = Str(178))]
            pub speed: ::validated_data::Field<&'a str>,
            #[data_view(schema = Int(566))]
            pub receive_bandwidth: ::validated_data::Field<i64>,
            #[data_view(schema = Int(567))]
            pub transmit_bandwidth: ::validated_data::Field<i64>,
            #[data_view(schema = Str(1333))]
            pub peer: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1334))]
            pub peer_interface: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1335))]
            pub peer_ip: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1336))]
            pub peer_ipv6: ::validated_data::Field<&'a str>,
            #[data_view(schema = Dict(1499))]
            pub bgp: ::validated_data::Field<item::Bgp<'a, Mode>>,
            #[data_view(schema = Str(1342))]
            pub ipv4_acl_in: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1343))]
            pub ipv4_acl_out: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1344))]
            pub ipv6_acl_in: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1345))]
            pub ipv6_acl_out: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(632))]
            pub static_routes: ::validated_data::Field<item::StaticRoutes<'a, Mode>>,
            #[data_view(schema = Str(1187))]
            pub qos_profile: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1346))]
            pub wan_carrier: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1347))]
            pub wan_circuit_id: ::validated_data::Field<&'a str>,
            #[data_view(schema = Bool(481))]
            pub connected_to_pathfinder: ::validated_data::Field<bool>,
            #[data_view(schema = Dict(1502))]
            pub cv_pathfinder_internet_exit: ::validated_data::Field<item::CvPathfinderInternetExit<'a, Mode>>,
            #[data_view(schema = Dict(1503))]
            pub rx_queue: ::validated_data::Field<item::RxQueue<'a, Mode>>,
            #[data_view(schema = Str(1351))]
            pub raw_eos_cli: ::validated_data::Field<&'a str>,
            #[data_view(schema = Dict(1504))]
            pub flow_tracking: ::validated_data::Field<item::FlowTracking<'a, Mode>>,
            #[data_view(schema = Dict(1505), relaxed)]
            pub structured_config: ::validated_data::Field<super::super::super::super::eos_config::ethernet_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.L3InterfacesItemBase.Ipv6Addresses", schema = List(631)))]
            pub struct Ipv6Addresses<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.L3InterfacesItemBase.Bgp", schema = Dict(1499)))]
            pub struct Bgp<'a, Mode> {
                #[data_view(schema = Str(1337))]
                pub peer_as: ::validated_data::RequiredValue<&'a str, Mode>,
                #[data_view(schema = Str(1338))]
                pub ipv4_prefix_list_in: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(1339))]
                pub ipv4_prefix_list_out: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(1340))]
                pub ipv6_prefix_list_in: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(1341))]
                pub ipv6_prefix_list_out: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.L3InterfacesItemBase.StaticRoutes", schema = List(632)))]
            pub struct StaticRoutes<'a, Mode> (::validated_data::Field<static_routes::Item<'a, Mode>>);

            pub mod static_routes {

                #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.L3InterfacesItemBase.StaticRoutesItem", schema = Dict(1500)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(984))]
                    pub prefix: ::validated_data::RequiredValue<&'a str, Mode>,
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.L3InterfacesItemBase.CvPathfinderInternetExit", schema = Dict(1502)))]
            pub struct CvPathfinderInternetExit<'a, Mode> {
                #[data_view(schema = List(633))]
                pub policies: ::validated_data::Field<cv_pathfinder_internet_exit::Policies<'a, Mode>>,
            }

            pub mod cv_pathfinder_internet_exit {

                #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.DevicesItemBase.L3InterfacesItemBase.CvPathfinderInternetExit.Policies", schema = List(633), item_name = "AVDDesign.DevicesItemBase.L3InterfacesItemBase.CvPathfinderInternetExit.PoliciesItem"))]
                pub struct Policies<'a, Mode> (::validated_data::Field<policies::Item<'a, Mode>>);

                pub mod policies {

                    #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.L3InterfacesItemBase.CvPathfinderInternetExit.PoliciesItemBase", schema = Dict(1501)))]
                    pub struct Item<'a, Mode> {
                        #[data_view(schema = Str(1199))]
                        pub name: ::validated_data::Field<&'a str>,
                        #[data_view(schema = Str(1348))]
                        pub tunnel_interface_numbers: ::validated_data::Field<&'a str>,
                    }
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.L3InterfacesItemBase.RxQueue", schema = Dict(1503)))]
            pub struct RxQueue<'a, Mode> {
                #[data_view(schema = Int(568))]
                pub count: ::validated_data::Field<i64>,
                #[data_view(schema = List(634))]
                pub workers: ::validated_data::Field<rx_queue::Workers<'a, Mode>>,
                #[data_view(schema = Str(1350))]
                pub mode: ::validated_data::Field<&'a str>,
            }

            pub mod rx_queue {

                #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.L3InterfacesItemBase.RxQueue.Workers", schema = List(634)))]
                pub struct Workers<'a, Mode> (::validated_data::Field<&'a str>);
            }

            #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.L3InterfacesItemBase.FlowTracking", schema = Dict(1504)))]
            pub struct FlowTracking<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::validated_data::Field<bool>,
                #[data_view(schema = Str(1155))]
                pub name: ::validated_data::Field<&'a str>,
            }
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.DevicesItemBase.L3PortChannels", schema = List(638), item_name = "AVDDesign.DevicesItemBase.L3PortChannelsItem"))]
    pub struct L3PortChannels<'a, Mode> (::validated_data::Field<l3_port_channels::Item<'a, Mode>>);

    pub mod l3_port_channels {

        #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.L3PortChannelsItemBase", schema = Dict(1513)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(1352))]
            pub name: ::validated_data::RequiredValue<&'a str, Mode>,
            #[data_view(schema = Str(1353))]
            pub description: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1354))]
            pub mode: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(636))]
            pub member_interfaces: ::validated_data::Field<item::MemberInterfaces<'a, Mode>>,
            #[data_view(schema = Str(1329))]
            pub ip_address: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(631))]
            pub ipv6_addresses: ::validated_data::Field<item::Ipv6Addresses<'a, Mode>>,
            #[data_view(schema = Str(1331))]
            pub dhcp_ip: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1358))]
            pub public_ip: ::validated_data::Field<&'a str>,
            #[data_view(schema = Int(565))]
            pub encapsulation_dot1q_vlan: ::validated_data::Field<i64>,
            #[data_view(schema = Bool(479))]
            pub dhcp_accept_default_route: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(480))]
            pub enabled: ::validated_data::Field<bool>,
            #[data_view(schema = Str(1333))]
            pub peer: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1359))]
            pub peer_port_channel: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1335))]
            pub peer_ip: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1336))]
            pub peer_ipv6: ::validated_data::Field<&'a str>,
            #[data_view(schema = Dict(1509))]
            pub bgp: ::validated_data::Field<item::Bgp<'a, Mode>>,
            #[data_view(schema = Str(1342))]
            pub ipv4_acl_in: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1343))]
            pub ipv4_acl_out: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1344))]
            pub ipv6_acl_in: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1345))]
            pub ipv6_acl_out: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(637))]
            pub static_routes: ::validated_data::Field<item::StaticRoutes<'a, Mode>>,
            #[data_view(schema = Str(1187))]
            pub qos_profile: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1346))]
            pub wan_carrier: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1347))]
            pub wan_circuit_id: ::validated_data::Field<&'a str>,
            #[data_view(schema = Bool(481))]
            pub connected_to_pathfinder: ::validated_data::Field<bool>,
            #[data_view(schema = Str(1363))]
            pub raw_eos_cli: ::validated_data::Field<&'a str>,
            #[data_view(schema = Dict(1511))]
            pub flow_tracking: ::validated_data::Field<item::FlowTracking<'a, Mode>>,
            #[data_view(schema = Dict(1512), relaxed)]
            pub structured_config: ::validated_data::Field<super::super::super::super::eos_config::port_channel_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
        }

        pub mod item {

            #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.DevicesItemBase.L3PortChannelsItemBase.MemberInterfaces", schema = List(636), item_name = "AVDDesign.DevicesItemBase.L3PortChannelsItemBase.MemberInterfacesItem"))]
            pub struct MemberInterfaces<'a, Mode> (::validated_data::Field<member_interfaces::Item<'a, Mode>>);

            pub mod member_interfaces {

                #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.L3PortChannelsItemBase.MemberInterfacesItemBase", schema = Dict(1508)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(1355))]
                    pub name: ::validated_data::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Str(1356))]
                    pub description: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1357))]
                    pub peer: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1334))]
                    pub peer_interface: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(178))]
                    pub speed: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Dict(1503))]
                    pub rx_queue: ::validated_data::Field<item::RxQueue<'a, Mode>>,
                    #[data_view(schema = Dict(1507), relaxed)]
                    pub structured_config: ::validated_data::Field<super::super::super::super::super::super::eos_config::ethernet_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
                }

                pub mod item {

                    #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.L3PortChannelsItemBase.MemberInterfacesItemBase.RxQueue", schema = Dict(1503)))]
                    pub struct RxQueue<'a, Mode> {
                        #[data_view(schema = Int(568))]
                        pub count: ::validated_data::Field<i64>,
                        #[data_view(schema = List(634))]
                        pub workers: ::validated_data::Field<rx_queue::Workers<'a, Mode>>,
                        #[data_view(schema = Str(1350))]
                        pub mode: ::validated_data::Field<&'a str>,
                    }

                    pub mod rx_queue {

                        #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.L3PortChannelsItemBase.MemberInterfacesItemBase.RxQueue.Workers", schema = List(634)))]
                        pub struct Workers<'a, Mode> (::validated_data::Field<&'a str>);
                    }
                }
            }

            #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.L3PortChannelsItemBase.Ipv6Addresses", schema = List(631)))]
            pub struct Ipv6Addresses<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.L3PortChannelsItemBase.Bgp", schema = Dict(1509)))]
            pub struct Bgp<'a, Mode> {
                #[data_view(schema = Str(1337))]
                pub peer_as: ::validated_data::RequiredValue<&'a str, Mode>,
                #[data_view(schema = Str(1338))]
                pub ipv4_prefix_list_in: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(1339))]
                pub ipv4_prefix_list_out: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(1360))]
                pub ipv6_prefix_list_in: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(1361))]
                pub ipv6_prefix_list_out: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view(indexed_list, primary_key(prefix), python(name = "AVDDesign.DevicesItemBase.L3PortChannelsItemBase.StaticRoutes", schema = List(637), item_name = "AVDDesign.DevicesItemBase.L3PortChannelsItemBase.StaticRoutesItem"))]
            pub struct StaticRoutes<'a, Mode> (::validated_data::Field<static_routes::Item<'a, Mode>>);

            pub mod static_routes {

                #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.L3PortChannelsItemBase.StaticRoutesItemBase", schema = Dict(1510)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(1362))]
                    pub prefix: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.L3PortChannelsItemBase.FlowTracking", schema = Dict(1511)))]
            pub struct FlowTracking<'a, Mode> {
                #[data_view(schema = Bool(7))]
                pub enabled: ::validated_data::Field<bool>,
                #[data_view(schema = Str(1155))]
                pub name: ::validated_data::Field<&'a str>,
            }
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.UnderlayMulticast", schema = Dict(1516)))]
    pub struct UnderlayMulticast<'a, Mode> {
        #[data_view(schema = Dict(1514))]
        pub pim_sm: ::validated_data::Field<underlay_multicast::PimSm<'a, Mode>>,
        #[data_view(schema = Dict(1515), rename = "static")]
        pub field_static: ::validated_data::Field<underlay_multicast::FieldStatic<'a, Mode>>,
    }

    pub mod underlay_multicast {

        #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.UnderlayMulticast.PimSm", schema = Dict(1514)))]
        pub struct PimSm<'a, Mode> {
            #[data_view(schema = Bool(482))]
            pub enabled: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(483))]
            pub uplinks: ::validated_data::Field<bool>,
            #[data_view(schema = List(639))]
            pub uplink_interfaces: ::validated_data::Field<pim_sm::UplinkInterfaces<'a, Mode>>,
            #[data_view(schema = Bool(484))]
            pub mlag: ::validated_data::Field<bool>,
        }

        pub mod pim_sm {

            #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.UnderlayMulticast.PimSm.UplinkInterfaces", schema = List(639)))]
            pub struct UplinkInterfaces<'a, Mode> (::validated_data::Field<&'a str>);
        }

        #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.UnderlayMulticast.Static", schema = Dict(1515)))]
        pub struct FieldStatic<'a, Mode> {
            #[data_view(schema = Bool(485))]
            pub enabled: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(486))]
            pub uplinks: ::validated_data::Field<bool>,
            #[data_view(schema = List(640))]
            pub uplink_interfaces: ::validated_data::Field<field_static::UplinkInterfaces<'a, Mode>>,
            #[data_view(schema = Bool(487))]
            pub mlag: ::validated_data::Field<bool>,
        }

        pub mod field_static {

            #[::validated_data::data_view(list, python(name = "AVDDesign.DevicesItemBase.UnderlayMulticast.Static.UplinkInterfaces", schema = List(640)))]
            pub struct UplinkInterfaces<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesign.DevicesItemBase.DigitalTwin", schema = Dict(1517)))]
    pub struct DigitalTwin<'a, Mode> {
        #[data_view(schema = Str(1369))]
        pub act_os_version: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1370))]
        pub mgmt_ip: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1371))]
        pub mgmt_gateway: ::validated_data::Field<&'a str>,
        #[data_view(schema = Bool(488))]
        pub act_internet_access: ::validated_data::Field<bool>,
    }
}
