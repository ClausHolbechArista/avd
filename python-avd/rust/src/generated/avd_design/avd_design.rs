// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "AVDDesign", schema = Dict(1808)))]
pub struct AvdDesign<'a, Mode> {
    #[data_view(schema = Dict(1385))]
    pub aaa_settings: ::validated_data::Field<AaaSettings<'a, Mode>>,
    #[data_view(schema = Dict(1386))]
    pub address_locking_settings: ::validated_data::Field<AddressLockingSettings<'a, Mode>>,
    #[data_view(schema = Dict(77))]
    pub application_classification: ::validated_data::Field<super::eos_config::ApplicationTrafficRecognition<'a, Mode>>,
    #[data_view(schema = Dict(1388))]
    pub avd_design_future: ::validated_data::Field<AvdDesignFuture<'a, Mode>>,
    #[data_view(schema = Bool(419))]
    pub avd_digital_twin_mode: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(422))]
    pub avd_eos_designs_structured_config: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1104))]
    pub avd_structured_config_file_format: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1389))]
    pub eos_designs_validation_configuration: ::validated_data::Field<EosDesignsValidationConfiguration<'a, Mode>>,
    #[data_view(schema = Str(68))]
    pub avd_vault_id: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1390))]
    pub bfd_multihop: ::validated_data::Field<BfdMultihop<'a, Mode>>,
    #[data_view(schema = Str(1105))]
    pub bgp_as: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1106))]
    pub bgp_as_notation: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(424))]
    pub bgp_default_ipv4_unicast: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1391))]
    pub bgp_distance: ::validated_data::Field<super::eos_config::router_bgp::Distance<'a, Mode>>,
    #[data_view(schema = Int(498))]
    pub bgp_ecmp: ::validated_data::Field<i64>,
    #[data_view(schema = Dict(1392))]
    pub bgp_graceful_restart: ::validated_data::Field<BgpGracefulRestart<'a, Mode>>,
    #[data_view(schema = Int(500))]
    pub bgp_maximum_paths: ::validated_data::Field<i64>,
    #[data_view(schema = Bool(426))]
    pub bgp_mesh_pes: ::validated_data::Field<bool>,
    #[data_view(schema = List(563))]
    pub bgp_peer_filters_catalog: ::validated_data::Field<super::eos_config::PeerFilters<'a, Mode>>,
    #[data_view(schema = Dict(1405))]
    pub bgp_peer_groups: ::validated_data::Field<BgpPeerGroups<'a, Mode>>,
    #[data_view(schema = Bool(427))]
    pub bgp_update_wait_install: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(428))]
    pub bgp_update_wait_for_convergence: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1122))]
    pub campus: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1123))]
    pub campus_access_pod: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1124))]
    pub campus_pod: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(576))]
    pub connected_endpoints: ::validated_data::Field<ConnectedEndpoints<'a, Mode>>,
    #[data_view(schema = List(577))]
    pub custom_connected_endpoints_keys: ::validated_data::Field<CustomConnectedEndpointsKeys<'a, Mode>>,
    #[data_view(schema = List(578))]
    pub connected_endpoints_keys: ::validated_data::Field<ConnectedEndpointsKeys<'a, Mode>>,
    #[data_view(schema = Dict(1446))]
    pub core_interfaces: ::validated_data::Field<CoreInterfaces<'a, Mode>>,
    #[data_view(schema = Str(1195))]
    pub custom_structured_configuration_list_merge: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(592))]
    pub custom_structured_configuration_prefix: ::validated_data::Field<CustomStructuredConfigurationPrefix<'a, Mode>>,
    #[data_view(schema = Str(1196))]
    pub custom_system_mac_address: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(593))]
    pub cv_pathfinder_global_sites: ::validated_data::Field<CvPathfinderGlobalSites<'a, Mode>>,
    #[data_view(schema = List(594))]
    pub cv_pathfinder_internet_exit_policies: ::validated_data::Field<CvPathfinderInternetExitPolicies<'a, Mode>>,
    #[data_view(schema = List(596))]
    pub cv_pathfinder_regions: ::validated_data::Field<CvPathfinderRegions<'a, Mode>>,
    #[data_view(schema = Str(1205))]
    pub cv_server: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1458))]
    pub cv_settings: ::validated_data::Field<CvSettings<'a, Mode>>,
    #[data_view(schema = Str(1215))]
    pub cv_tags_topology_type: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1216))]
    pub cv_token: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(601))]
    pub cv_topology: ::validated_data::Field<CvTopology<'a, Mode>>,
    #[data_view(schema = List(602))]
    pub cv_topology_levels: ::validated_data::Field<CvTopologyLevels<'a, Mode>>,
    #[data_view(schema = Str(1220))]
    pub dc_name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1221))]
    pub default_connected_endpoints_description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1222))]
    pub default_connected_endpoints_port_channel_description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(456))]
    pub default_igmp_snooping_enabled: ::validated_data::Field<bool>,
    #[data_view(schema = Int(529))]
    pub default_interface_mtu: ::validated_data::Field<i64>,
    #[data_view(schema = List(609))]
    pub default_interfaces: ::validated_data::Field<DefaultInterfaces<'a, Mode>>,
    #[data_view(schema = Str(1227))]
    pub default_mgmt_method: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1228))]
    pub default_network_ports_description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1229))]
    pub default_network_ports_port_channel_description: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(611))]
    pub default_node_types: ::validated_data::Field<DefaultNodeTypes<'a, Mode>>,
    #[data_view(schema = Str(1232))]
    pub default_underlay_p2p_ethernet_description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1233))]
    pub default_underlay_p2p_port_channel_description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1234))]
    pub default_vrf_diag_loopback_description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1235))]
    pub device_profile: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(641))]
    pub device_profiles: ::validated_data::Field<DeviceProfiles<'a, Mode>>,
    #[data_view(schema = List(644))]
    pub devices: ::validated_data::Field<Devices<'a, Mode>>,
    #[data_view(schema = Dict(1523))]
    pub digital_twin: ::validated_data::Field<DigitalTwin<'a, Mode>>,
    #[data_view(schema = Dict(1526))]
    pub dns_settings: ::validated_data::Field<DnsSettings<'a, Mode>>,
    #[data_view(schema = Dict(1542))]
    pub dot1x_settings: ::validated_data::Field<Dot1xSettings<'a, Mode>>,
    #[data_view(schema = Bool(509))]
    pub enable_trunk_groups: ::validated_data::Field<bool>,
    #[data_view(schema = List(650))]
    pub eos_designs_custom_templates: ::validated_data::Field<EosDesignsCustomTemplates<'a, Mode>>,
    #[data_view(schema = Dict(1545))]
    pub eos_designs_documentation: ::validated_data::Field<EosDesignsDocumentation<'a, Mode>>,
    #[data_view(schema = Bool(516))]
    pub eos_designs_keep_tmp_files: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(517))]
    pub eos_designs_return_structured_config: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1393))]
    pub eos_designs_tmp_dir: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(35))]
    pub eos_designs_validate_inputs_batch_size: ::validated_data::Field<i64>,
    #[data_view(schema = Bool(518))]
    pub eos_designs_validate_inputs_template_with_multiprocessing: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1550))]
    pub errdisable_settings: ::validated_data::Field<ErrdisableSettings<'a, Mode>>,
    #[data_view(schema = List(140))]
    pub event_handlers: ::validated_data::Field<super::eos_config::EventHandlers<'a, Mode>>,
    #[data_view(schema = Dict(373))]
    pub event_monitor: ::validated_data::Field<super::eos_config::EventMonitor<'a, Mode>>,
    #[data_view(schema = Int(571))]
    pub evpn_ebgp_gateway_multihop: ::validated_data::Field<i64>,
    #[data_view(schema = Int(572))]
    pub evpn_ebgp_multihop: ::validated_data::Field<i64>,
    #[data_view(schema = Dict(1551))]
    pub evpn_hostflap_detection: ::validated_data::Field<EvpnHostflapDetection<'a, Mode>>,
    #[data_view(schema = Bool(520))]
    pub evpn_import_pruning: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(521))]
    pub evpn_multicast: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(522))]
    pub evpn_overlay_bgp_rtc: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(523))]
    pub evpn_prevent_readvertise_to_server: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1394))]
    pub evpn_prevent_readvertise_to_server_mode: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1395))]
    pub evpn_short_esi_prefix: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(524))]
    pub evpn_vlan_aware_bundles: ::validated_data::Field<bool>,
    #[data_view(schema = List(651))]
    pub evpn_vlan_bundles: ::validated_data::Field<EvpnVlanBundles<'a, Mode>>,
    #[data_view(schema = Str(1400))]
    pub fabric_evpn_encapsulation: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1564))]
    pub fabric_flow_tracking: ::validated_data::Field<FabricFlowTracking<'a, Mode>>,
    #[data_view(schema = Dict(1569))]
    pub fabric_ip_addressing: ::validated_data::Field<FabricIpAddressing<'a, Mode>>,
    #[data_view(schema = Str(1403))]
    pub fabric_name: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Dict(1571))]
    pub fabric_numbering: ::validated_data::Field<FabricNumbering<'a, Mode>>,
    #[data_view(schema = Str(1406))]
    pub fabric_numbering_node_id_pool: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1572))]
    pub fabric_sflow: ::validated_data::Field<FabricSflow<'a, Mode>>,
    #[data_view(schema = Dict(1581))]
    pub flow_tracking_settings: ::validated_data::Field<FlowTrackingSettings<'a, Mode>>,
    #[data_view(schema = Dict(1587))]
    pub general_settings: ::validated_data::Field<GeneralSettings<'a, Mode>>,
    #[data_view(schema = Dict(1590))]
    pub generate_cv_tags: ::validated_data::Field<GenerateCvTags<'a, Mode>>,
    #[data_view(schema = Dict(1591))]
    pub hardware_counters: ::validated_data::Field<super::eos_config::HardwareCounters<'a, Mode>>,
    #[data_view(schema = Str(1417))]
    pub inband_ztp_bootstrap_file: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1593))]
    pub internal_vlan_order: ::validated_data::Field<InternalVlanOrder<'a, Mode>>,
    #[data_view(schema = Dict(1594))]
    pub ipsec_settings: ::validated_data::Field<IpsecSettings<'a, Mode>>,
    #[data_view(schema = List(658))]
    pub ipv4_acls: ::validated_data::Field<Ipv4Acls<'a, Mode>>,
    #[data_view(schema = List(660))]
    pub ipv4_prefix_list_catalog: ::validated_data::Field<Ipv4PrefixListCatalog<'a, Mode>>,
    #[data_view(schema = List(661))]
    pub ipv4_standard_acls: ::validated_data::Field<Ipv4StandardAcls<'a, Mode>>,
    #[data_view(schema = List(663))]
    pub ipv6_acls: ::validated_data::Field<Ipv6Acls<'a, Mode>>,
    #[data_view(schema = List(664))]
    pub ipv6_mgmt_destination_networks: ::validated_data::Field<Ipv6MgmtDestinationNetworks<'a, Mode>>,
    #[data_view(schema = Str(1424))]
    pub ipv6_mgmt_gateway: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(666))]
    pub ipv6_prefix_list_catalog: ::validated_data::Field<Ipv6PrefixListCatalog<'a, Mode>>,
    #[data_view(schema = Bool(541))]
    pub is_deployed: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(542))]
    pub isis_advertise_passive_only: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1426))]
    pub isis_area_id: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1427))]
    pub isis_default_circuit_type: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1428))]
    pub isis_default_is_type: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(590))]
    pub isis_default_metric: ::validated_data::Field<i64>,
    #[data_view(schema = Int(591))]
    pub isis_maximum_paths: ::validated_data::Field<i64>,
    #[data_view(schema = Str(1429))]
    pub isis_system_id_format: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1604))]
    pub isis_ti_lfa: ::validated_data::Field<IsisTiLfa<'a, Mode>>,
    #[data_view(schema = List(668))]
    pub l2vlan_profiles: ::validated_data::Field<L2vlanProfiles<'a, Mode>>,
    #[data_view(schema = Dict(1446))]
    pub l3_edge: ::validated_data::Field<L3Edge<'a, Mode>>,
    #[data_view(schema = List(669))]
    pub l3_interface_profiles: ::validated_data::Field<L3InterfaceProfiles<'a, Mode>>,
    #[data_view(schema = Dict(511))]
    pub load_interval: ::validated_data::Field<super::eos_config::LoadInterval<'a, Mode>>,
    #[data_view(schema = Dict(1617))]
    pub logging_settings: ::validated_data::Field<LoggingSettings<'a, Mode>>,
    #[data_view(schema = List(674))]
    pub mac_acls: ::validated_data::Field<MacAcls<'a, Mode>>,
    #[data_view(schema = Dict(1620))]
    pub mac_address_table: ::validated_data::Field<super::eos_config::MacAddressTable<'a, Mode>>,
    #[data_view(schema = Dict(1622))]
    pub management_eapi: ::validated_data::Field<ManagementEapi<'a, Mode>>,
    #[data_view(schema = Dict(1623))]
    pub management_settings: ::validated_data::Field<ManagementSettings<'a, Mode>>,
    #[data_view(schema = List(676))]
    pub mgmt_destination_networks: ::validated_data::Field<MgmtDestinationNetworks<'a, Mode>>,
    #[data_view(schema = Str(1453))]
    pub mgmt_gateway: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1454))]
    pub mgmt_interface: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1455))]
    pub mgmt_interface_description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1625))]
    pub mgmt_interface_settings: ::validated_data::Field<MgmtInterfaceSettings<'a, Mode>>,
    #[data_view(schema = Str(1459))]
    pub mgmt_interface_vrf: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(557))]
    pub mgmt_vrf_routing: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1460))]
    pub mlag_bgp_peer_description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1461))]
    pub mlag_bgp_peer_group_description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1626))]
    pub mlag_ibgp_peering_vrfs: ::validated_data::Field<MlagIbgpPeeringVrfs<'a, Mode>>,
    #[data_view(schema = Str(1462))]
    pub mlag_member_description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(558))]
    pub mlag_on_orphan_port_channel_downlink: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1463))]
    pub mlag_peer_l3_svi_description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1464))]
    pub mlag_peer_l3_vlan_name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1465))]
    pub mlag_peer_l3_vrf_svi_description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1466))]
    pub mlag_peer_l3_vrf_vlan_name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1467))]
    pub mlag_peer_svi_description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1468))]
    pub mlag_peer_vlan_name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1469))]
    pub mlag_port_channel_description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1631))]
    pub monitor_connectivity: ::validated_data::Field<MonitorConnectivity<'a, Mode>>,
    #[data_view(schema = List(686))]
    pub network_ports: ::validated_data::Field<NetworkPorts<'a, Mode>>,
    #[data_view(schema = List(753))]
    pub network_services: ::validated_data::Field<NetworkServices<'a, Mode>>,
    #[data_view(schema = List(754))]
    pub network_services_keys: ::validated_data::Field<NetworkServicesKeys<'a, Mode>>,
    #[data_view(schema = List(756))]
    pub custom_node_type_keys: ::validated_data::Field<CustomNodeTypeKeys<'a, Mode>>,
    #[data_view(schema = List(757))]
    pub node_type_keys: ::validated_data::Field<NodeTypeKeys<'a, Mode>>,
    #[data_view(schema = Dict(1713))]
    pub ntp_settings: ::validated_data::Field<NtpSettings<'a, Mode>>,
    #[data_view(schema = Bool(609))]
    pub only_local_vlan_trunk_groups: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1597))]
    pub overlay_bgp_peer_description: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(760))]
    pub overlay_cvx_servers: ::validated_data::Field<OverlayCvxServers<'a, Mode>>,
    #[data_view(schema = Bool(610))]
    pub overlay_her_flood_list_per_vni: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1599))]
    pub overlay_her_flood_list_scope: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(611))]
    pub overlay_mlag_rfc5549: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1714))]
    pub overlay_rd_type: ::validated_data::Field<OverlayRdType<'a, Mode>>,
    #[data_view(schema = Str(1604))]
    pub overlay_routing_protocol: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1605))]
    pub overlay_routing_protocol_address_family: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1715))]
    pub overlay_rt_type: ::validated_data::Field<OverlayRtType<'a, Mode>>,
    #[data_view(schema = Int(621))]
    pub p2p_uplinks_mtu: ::validated_data::Field<i64>,
    #[data_view(schema = Str(1609))]
    pub p2p_uplinks_qos_profile: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(763))]
    pub custom_platform_settings: ::validated_data::Field<CustomPlatformSettings<'a, Mode>>,
    #[data_view(schema = List(764))]
    pub platform_settings: ::validated_data::Field<PlatformSettings<'a, Mode>>,
    #[data_view(schema = List(766))]
    pub platform_speed_groups: ::validated_data::Field<PlatformSpeedGroups<'a, Mode>>,
    #[data_view(schema = Str(1615))]
    pub pod_name: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(768))]
    pub port_profiles: ::validated_data::Field<PortProfiles<'a, Mode>>,
    #[data_view(schema = List(769))]
    pub ptp_profiles: ::validated_data::Field<PtpProfiles<'a, Mode>>,
    #[data_view(schema = Dict(1735))]
    pub ptp_settings: ::validated_data::Field<PtpSettings<'a, Mode>>,
    #[data_view(schema = Dict(1736))]
    pub queue_monitor_length: ::validated_data::Field<QueueMonitorLength<'a, Mode>>,
    #[data_view(schema = Dict(1737))]
    pub queue_monitor_streaming: ::validated_data::Field<super::eos_config::QueueMonitorStreaming<'a, Mode>>,
    #[data_view(schema = Dict(1738))]
    pub redundancy: ::validated_data::Field<Redundancy<'a, Mode>>,
    #[data_view(schema = Str(1621))]
    pub router_id_loopback_description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1622))]
    pub serial_number: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1743))]
    pub sflow_settings: ::validated_data::Field<SflowSettings<'a, Mode>>,
    #[data_view(schema = Bool(642))]
    pub shutdown_bgp_towards_undeployed_peers: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(643))]
    pub shutdown_interfaces_towards_undeployed_peers: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1749))]
    pub snmp_settings: ::validated_data::Field<SnmpSettings<'a, Mode>>,
    #[data_view(schema = Dict(1756))]
    pub source_interfaces: ::validated_data::Field<SourceInterfaces<'a, Mode>>,
    #[data_view(schema = Dict(1758))]
    pub spanning_tree_settings: ::validated_data::Field<SpanningTreeSettings<'a, Mode>>,
    #[data_view(schema = Dict(1761))]
    pub ssh_settings: ::validated_data::Field<SshSettings<'a, Mode>>,
    #[data_view(schema = List(781))]
    pub svi_profiles: ::validated_data::Field<SviProfiles<'a, Mode>>,
    #[data_view(schema = Str(1643))]
    pub system_mac_address: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(782))]
    pub tcam_profiles: ::validated_data::Field<super::eos_config::tcam_profile::Profiles<'a, Mode>>,
    #[data_view(schema = Str(1644))]
    pub timezone: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1767))]
    pub trunk_groups: ::validated_data::Field<TrunkGroups<'a, Mode>>,
    #[data_view(schema = Str(1647), rename = "type")]
    pub field_type: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(661))]
    pub underlay_filter_peer_as: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(662))]
    pub underlay_filter_redistribute_connected: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(663))]
    pub underlay_ipv6: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(664))]
    pub underlay_ipv6_numbered: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1648))]
    pub underlay_isis_authentication_cleartext_key: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1649))]
    pub underlay_isis_authentication_key: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1650))]
    pub underlay_isis_authentication_mode: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(665))]
    pub underlay_isis_bfd: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1651))]
    pub underlay_isis_instance_name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1652))]
    pub underlay_l2_ethernet_description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1653))]
    pub underlay_l2_port_channel_description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1768))]
    pub underlay_multicast_anycast_rp: ::validated_data::Field<UnderlayMulticastAnycastRp<'a, Mode>>,
    #[data_view(schema = Bool(667))]
    pub underlay_multicast_pim_sm: ::validated_data::Field<bool>,
    #[data_view(schema = List(785))]
    pub underlay_multicast_rps: ::validated_data::Field<UnderlayMulticastRps<'a, Mode>>,
    #[data_view(schema = Bool(668))]
    pub underlay_multicast_static: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1659))]
    pub underlay_ospf_area: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1772))]
    pub underlay_ospf_authentication: ::validated_data::Field<UnderlayOspfAuthentication<'a, Mode>>,
    #[data_view(schema = Bool(670))]
    pub underlay_ospf_bfd_enable: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(671))]
    pub underlay_ospf_graceful_restart: ::validated_data::Field<bool>,
    #[data_view(schema = Int(635))]
    pub underlay_ospf_max_lsa: ::validated_data::Field<i64>,
    #[data_view(schema = Int(636))]
    pub underlay_ospf_maximum_paths: ::validated_data::Field<i64>,
    #[data_view(schema = Int(637))]
    pub underlay_ospf_process_id: ::validated_data::Field<i64>,
    #[data_view(schema = Bool(672))]
    pub underlay_rfc5549: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1662))]
    pub underlay_routing_protocol: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1217))]
    pub unsupported_transceiver: ::validated_data::Field<super::eos_config::ServiceUnsupportedTransceiver<'a, Mode>>,
    #[data_view(schema = Dict(1773))]
    pub uplink_ptp: ::validated_data::Field<UplinkPtp<'a, Mode>>,
    #[data_view(schema = Bool(673))]
    pub use_cv_topology: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(674))]
    pub use_router_general_for_router_id: ::validated_data::Field<bool>,
    #[data_view(schema = List(787))]
    pub validation_profiles: ::validated_data::Field<ValidationProfiles<'a, Mode>>,
    #[data_view(schema = Str(1664))]
    pub vtep_loopback_description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1665))]
    pub vtep_vvtep_ip: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(788))]
    pub wan_carriers: ::validated_data::Field<WanCarriers<'a, Mode>>,
    #[data_view(schema = Str(1669))]
    pub wan_encapsulation: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1778))]
    pub wan_ha: ::validated_data::Field<WanHa<'a, Mode>>,
    #[data_view(schema = Dict(1781))]
    pub wan_ipsec_profiles: ::validated_data::Field<WanIpsecProfiles<'a, Mode>>,
    #[data_view(schema = Str(1679))]
    pub wan_mode: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(790))]
    pub wan_path_groups: ::validated_data::Field<WanPathGroups<'a, Mode>>,
    #[data_view(schema = List(793))]
    pub wan_route_servers: ::validated_data::Field<WanRouteServers<'a, Mode>>,
    #[data_view(schema = Bool(680))]
    pub wan_stun_dtls_disable: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1688))]
    pub wan_stun_dtls_profile_name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1797))]
    pub wan_virtual_topologies: ::validated_data::Field<WanVirtualTopologies<'a, Mode>>,
    #[data_view(schema = Dict(1801))]
    pub zscaler_endpoints: ::validated_data::Field<ZscalerEndpoints<'a, Mode>>,
    #[data_view(schema = Dict(1805), python_name = "custom_node_types", dynamic = "custom_node_type_keys.key")]
    pub custom_node_type_keys_key: ::validated_data::Field<DynamicSlot12232<'a, Mode>>,
    #[data_view(schema = List(801), python_name = "connected_endpoints", dynamic = "connected_endpoints_keys.key")]
    pub connected_endpoints_keys_key: ::validated_data::Field<DynamicSlot13731<'a, Mode>>,
    #[data_view(schema = List(802), python_name = "custom_connected_endpoints", dynamic = "custom_connected_endpoints_keys.key")]
    pub custom_connected_endpoints_keys_key: ::validated_data::Field<DynamicSlot13919<'a, Mode>>,
    #[data_view(schema = List(803), python_name = "network_services", dynamic = "network_services_keys.name")]
    pub network_services_keys_name: ::validated_data::Field<DynamicSlot14107<'a, Mode>>,
    #[data_view(schema = Dict(1807), python_name = "node_types", dynamic = "node_type_keys.key")]
    pub node_type_keys_key: ::validated_data::Field<DynamicSlot15029<'a, Mode>>,
}

pub mod aaa_settings;
pub use aaa_settings::AaaSettings;

pub mod address_locking_settings;
pub use address_locking_settings::AddressLockingSettings;

pub mod avd_design_future;
pub use avd_design_future::AvdDesignFuture;

pub mod eos_designs_validation_configuration;
pub use eos_designs_validation_configuration::EosDesignsValidationConfiguration;

pub mod bfd_multihop;
pub use bfd_multihop::BfdMultihop;

pub mod bgp_graceful_restart;
pub use bgp_graceful_restart::BgpGracefulRestart;

pub mod bgp_peer_groups;
pub use bgp_peer_groups::BgpPeerGroups;

pub mod connected_endpoints;
pub use connected_endpoints::ConnectedEndpoints;

pub mod custom_connected_endpoints_keys;
pub use custom_connected_endpoints_keys::CustomConnectedEndpointsKeys;

pub mod connected_endpoints_keys;
pub use connected_endpoints_keys::ConnectedEndpointsKeys;

pub mod core_interfaces;
pub use core_interfaces::CoreInterfaces;

pub mod custom_structured_configuration_prefix;
pub use custom_structured_configuration_prefix::CustomStructuredConfigurationPrefix;

pub mod cv_pathfinder_global_sites;
pub use cv_pathfinder_global_sites::CvPathfinderGlobalSites;

pub mod cv_pathfinder_internet_exit_policies;
pub use cv_pathfinder_internet_exit_policies::CvPathfinderInternetExitPolicies;

pub mod cv_pathfinder_regions;
pub use cv_pathfinder_regions::CvPathfinderRegions;

pub mod cv_settings;
pub use cv_settings::CvSettings;

pub mod cv_topology;
pub use cv_topology::CvTopology;

pub mod cv_topology_levels;
pub use cv_topology_levels::CvTopologyLevels;

pub mod default_interfaces;
pub use default_interfaces::DefaultInterfaces;

pub mod default_node_types;
pub use default_node_types::DefaultNodeTypes;

pub mod device_profiles;
pub use device_profiles::DeviceProfiles;

pub mod devices;
pub use devices::Devices;

pub mod digital_twin;
pub use digital_twin::DigitalTwin;

pub mod dns_settings;
pub use dns_settings::DnsSettings;

pub mod dot1x_settings;
pub use dot1x_settings::Dot1xSettings;

pub mod eos_designs_custom_templates;
pub use eos_designs_custom_templates::EosDesignsCustomTemplates;

pub mod eos_designs_documentation;
pub use eos_designs_documentation::EosDesignsDocumentation;

pub mod errdisable_settings;
pub use errdisable_settings::ErrdisableSettings;

pub mod evpn_hostflap_detection;
pub use evpn_hostflap_detection::EvpnHostflapDetection;

pub mod evpn_vlan_bundles;
pub use evpn_vlan_bundles::EvpnVlanBundles;

pub mod fabric_flow_tracking;
pub use fabric_flow_tracking::FabricFlowTracking;

pub mod fabric_ip_addressing;
pub use fabric_ip_addressing::FabricIpAddressing;

pub mod fabric_numbering;
pub use fabric_numbering::FabricNumbering;

pub mod fabric_sflow;
pub use fabric_sflow::FabricSflow;

pub mod flow_tracking_settings;
pub use flow_tracking_settings::FlowTrackingSettings;

pub mod general_settings;
pub use general_settings::GeneralSettings;

pub mod generate_cv_tags;
pub use generate_cv_tags::GenerateCvTags;

pub mod internal_vlan_order;
pub use internal_vlan_order::InternalVlanOrder;

pub mod ipsec_settings;
pub use ipsec_settings::IpsecSettings;

pub mod ipv4_acls;
pub use ipv4_acls::Ipv4Acls;

pub mod ipv4_prefix_list_catalog;
pub use ipv4_prefix_list_catalog::Ipv4PrefixListCatalog;

pub mod ipv4_standard_acls;
pub use ipv4_standard_acls::Ipv4StandardAcls;

pub mod ipv6_acls;
pub use ipv6_acls::Ipv6Acls;

pub mod ipv6_mgmt_destination_networks;
pub use ipv6_mgmt_destination_networks::Ipv6MgmtDestinationNetworks;

pub mod ipv6_prefix_list_catalog;
pub use ipv6_prefix_list_catalog::Ipv6PrefixListCatalog;

pub mod isis_ti_lfa;
pub use isis_ti_lfa::IsisTiLfa;

pub mod l2vlan_profiles;
pub use l2vlan_profiles::L2vlanProfiles;

pub mod l3_edge;
pub use l3_edge::L3Edge;

pub mod l3_interface_profiles;
pub use l3_interface_profiles::L3InterfaceProfiles;

pub mod logging_settings;
pub use logging_settings::LoggingSettings;

pub mod mac_acls;
pub use mac_acls::MacAcls;

pub mod management_eapi;
pub use management_eapi::ManagementEapi;

pub mod management_settings;
pub use management_settings::ManagementSettings;

pub mod mgmt_destination_networks;
pub use mgmt_destination_networks::MgmtDestinationNetworks;

pub mod mgmt_interface_settings;
pub use mgmt_interface_settings::MgmtInterfaceSettings;

pub mod mlag_ibgp_peering_vrfs;
pub use mlag_ibgp_peering_vrfs::MlagIbgpPeeringVrfs;

pub mod monitor_connectivity;
pub use monitor_connectivity::MonitorConnectivity;

pub mod network_ports;
pub use network_ports::NetworkPorts;

pub mod network_services;
pub use network_services::NetworkServices;

pub mod network_services_keys;
pub use network_services_keys::NetworkServicesKeys;

pub mod custom_node_type_keys;
pub use custom_node_type_keys::CustomNodeTypeKeys;

pub mod node_type_keys;
pub use node_type_keys::NodeTypeKeys;

pub mod ntp_settings;
pub use ntp_settings::NtpSettings;

pub mod overlay_cvx_servers;
pub use overlay_cvx_servers::OverlayCvxServers;

pub mod overlay_rd_type;
pub use overlay_rd_type::OverlayRdType;

pub mod overlay_rt_type;
pub use overlay_rt_type::OverlayRtType;

pub mod custom_platform_settings;
pub use custom_platform_settings::CustomPlatformSettings;

pub mod platform_settings;
pub use platform_settings::PlatformSettings;

pub mod platform_speed_groups;
pub use platform_speed_groups::PlatformSpeedGroups;

pub mod port_profiles;
pub use port_profiles::PortProfiles;

pub mod ptp_profiles;
pub use ptp_profiles::PtpProfiles;

pub mod ptp_settings;
pub use ptp_settings::PtpSettings;

pub mod queue_monitor_length;
pub use queue_monitor_length::QueueMonitorLength;

pub mod redundancy;
pub use redundancy::Redundancy;

pub mod sflow_settings;
pub use sflow_settings::SflowSettings;

pub mod snmp_settings;
pub use snmp_settings::SnmpSettings;

pub mod source_interfaces;
pub use source_interfaces::SourceInterfaces;

pub mod spanning_tree_settings;
pub use spanning_tree_settings::SpanningTreeSettings;

pub mod ssh_settings;
pub use ssh_settings::SshSettings;

pub mod svi_profiles;
pub use svi_profiles::SviProfiles;

pub mod trunk_groups;
pub use trunk_groups::TrunkGroups;

pub mod underlay_multicast_anycast_rp;
pub use underlay_multicast_anycast_rp::UnderlayMulticastAnycastRp;

pub mod underlay_multicast_rps;
pub use underlay_multicast_rps::UnderlayMulticastRps;

pub mod underlay_ospf_authentication;
pub use underlay_ospf_authentication::UnderlayOspfAuthentication;

pub mod uplink_ptp;
pub use uplink_ptp::UplinkPtp;

pub mod validation_profiles;
pub use validation_profiles::ValidationProfiles;

pub mod wan_carriers;
pub use wan_carriers::WanCarriers;

pub mod wan_ha;
pub use wan_ha::WanHa;

pub mod wan_ipsec_profiles;
pub use wan_ipsec_profiles::WanIpsecProfiles;

pub mod wan_path_groups;
pub use wan_path_groups::WanPathGroups;

pub mod wan_route_servers;
pub use wan_route_servers::WanRouteServers;

pub mod wan_virtual_topologies;
pub use wan_virtual_topologies::WanVirtualTopologies;

pub mod zscaler_endpoints;
pub use zscaler_endpoints::ZscalerEndpoints;

pub mod dynamic_slot_12232;
pub use dynamic_slot_12232::DynamicSlot12232;

pub mod dynamic_slot_13731;
pub use dynamic_slot_13731::DynamicSlot13731;

pub mod dynamic_slot_13919;
pub use dynamic_slot_13919::DynamicSlot13919;

pub mod dynamic_slot_14107;
pub use dynamic_slot_14107::DynamicSlot14107;

pub mod dynamic_slot_15029;
pub use dynamic_slot_15029::DynamicSlot15029;
