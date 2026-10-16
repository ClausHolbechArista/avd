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

#[::validated_data::data_view(python(name = "AVDDesign.AaaSettings", schema = Dict(1385)))]
pub struct AaaSettings<'a, Mode> {
    #[data_view(schema = Dict(1366))]
    pub enable_password: ::validated_data::Field<aaa_settings::EnablePassword<'a, Mode>>,
    #[data_view(schema = Dict(1370))]
    pub tacacs: ::validated_data::Field<aaa_settings::Tacacs<'a, Mode>>,
    #[data_view(schema = Dict(1374))]
    pub radius: ::validated_data::Field<aaa_settings::Radius<'a, Mode>>,
    #[data_view(schema = Dict(1376))]
    pub authentication: ::validated_data::Field<aaa_settings::Authentication<'a, Mode>>,
    #[data_view(schema = Dict(1378))]
    pub authorization: ::validated_data::Field<aaa_settings::Authorization<'a, Mode>>,
    #[data_view(schema = Dict(1382))]
    pub accounting: ::validated_data::Field<aaa_settings::Accounting<'a, Mode>>,
    #[data_view(schema = Dict(1383))]
    pub root_login: ::validated_data::Field<aaa_settings::RootLogin<'a, Mode>>,
    #[data_view(schema = List(561))]
    pub local_users: ::validated_data::Field<aaa_settings::LocalUsers<'a, Mode>>,
}

pub mod aaa_settings;

#[::validated_data::data_view(python(name = "AVDDesign.AddressLockingSettings", schema = Dict(1386)))]
pub struct AddressLockingSettings<'a, Mode> {
    #[data_view(schema = Str(1102))]
    pub local_interface: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(562))]
    pub dhcp_server_interfaces: ::validated_data::Field<address_locking_settings::DhcpServerInterfaces<'a, Mode>>,
    #[data_view(schema = List(32))]
    pub dhcp_servers_ipv4: ::validated_data::Field<address_locking_settings::DhcpServersIpv4<'a, Mode>>,
    #[data_view(schema = Bool(10))]
    pub disabled: ::validated_data::Field<bool>,
    #[data_view(schema = List(34))]
    pub leases: ::validated_data::Field<address_locking_settings::Leases<'a, Mode>>,
    #[data_view(schema = Dict(62))]
    pub locked_address: ::validated_data::Field<address_locking_settings::LockedAddress<'a, Mode>>,
}

pub mod address_locking_settings;

#[::validated_data::data_view(python(name = "AVDDesign.AvdDesignFuture", schema = Dict(1388)))]
pub struct AvdDesignFuture<'a, Mode> {
    #[data_view(schema = Bool(402))]
    pub accept_dhcp_default_route_for_mgmt_ip_dhcp: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(403))]
    pub accept_ra_default_route_for_ipv6_mgmt_ip_auto_config: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(404))]
    pub accept_dhcp_default_route_for_inband_mgmt_ip_dhcp: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(405))]
    pub allow_recursive_profile_inheritance: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(406))]
    pub configure_inband_mgmt_ipv6_vrf: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(407))]
    pub consistent_uplink_vlans: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(408))]
    pub fix_address_locking_dhcp_server_interfaces: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(409))]
    pub fix_match_ipv6_prefix_list_on_mlag_route_map: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(410))]
    pub fix_mlag_ibgp_peering_ipv6_pool: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(411))]
    pub fix_radius_server_group_tls: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(412))]
    pub only_configure_ipv6_inband_mgmt_prefix_list_when_used: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(413))]
    pub only_configure_mlag_vrfs_peer_group_when_used: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(414))]
    pub only_configure_pvst_border_when_mode_is_mstp: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(415))]
    pub only_configure_route_map_connected_to_bgp_vrfs_when_used: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(416))]
    pub raise_for_port_channels_without_members: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(417))]
    pub raise_for_underlay_router_with_uplink_type_port_channel: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(418))]
    pub remove_redundant_ipv4_unicast_for_peer_groups: ::validated_data::Field<bool>,
}

#[::validated_data::data_view(python(name = "AVDDesign.EosDesignsValidationConfiguration", schema = Dict(1389)))]
pub struct EosDesignsValidationConfiguration<'a, Mode> {
    #[data_view(schema = Bool(423))]
    pub warn_eos_config_keys: ::validated_data::Field<bool>,
}

#[::validated_data::data_view(python(name = "AVDDesign.BfdMultihop", schema = Dict(1390)))]
pub struct BfdMultihop<'a, Mode> {
    #[data_view(schema = Int(497))]
    pub interval: ::validated_data::RequiredValue<i64, Mode>,
    #[data_view(schema = Int(497))]
    pub min_rx: ::validated_data::RequiredValue<i64, Mode>,
    #[data_view(schema = Int(380))]
    pub multiplier: ::validated_data::RequiredValue<i64, Mode>,
}

#[::validated_data::data_view(python(name = "AVDDesign.BgpGracefulRestart", schema = Dict(1392)))]
pub struct BgpGracefulRestart<'a, Mode> {
    #[data_view(schema = Bool(425))]
    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    #[data_view(schema = Int(499))]
    pub restart_time: ::validated_data::Field<i64>,
}

#[::validated_data::data_view(python(name = "AVDDesign.BgpPeerGroups", schema = Dict(1405)))]
pub struct BgpPeerGroups<'a, Mode> {
    #[data_view(schema = Dict(1394))]
    pub ipv4_underlay_peers: ::validated_data::Field<bgp_peer_groups::Ipv4UnderlayPeers<'a, Mode>>,
    #[data_view(schema = Dict(1395))]
    pub mlag_ipv4_vrfs_peer: ::validated_data::Field<bgp_peer_groups::MlagIpv4VrfsPeer<'a, Mode>>,
    #[data_view(schema = Dict(1396))]
    pub mlag_ipv4_underlay_peer: ::validated_data::Field<bgp_peer_groups::MlagIpv4UnderlayPeer<'a, Mode>>,
    #[data_view(schema = Dict(1397))]
    pub evpn_overlay_peers: ::validated_data::Field<bgp_peer_groups::EvpnOverlayPeers<'a, Mode>>,
    #[data_view(schema = Dict(1398))]
    pub evpn_overlay_core: ::validated_data::Field<bgp_peer_groups::EvpnOverlayCore<'a, Mode>>,
    #[data_view(schema = Dict(1399))]
    pub mpls_overlay_peers: ::validated_data::Field<bgp_peer_groups::MplsOverlayPeers<'a, Mode>>,
    #[data_view(schema = Dict(1400))]
    pub rr_overlay_peers: ::validated_data::Field<bgp_peer_groups::RrOverlayPeers<'a, Mode>>,
    #[data_view(schema = Dict(1401))]
    pub ipvpn_gateway_peers: ::validated_data::Field<bgp_peer_groups::IpvpnGatewayPeers<'a, Mode>>,
    #[data_view(schema = Dict(1403))]
    pub wan_overlay_peers: ::validated_data::Field<bgp_peer_groups::WanOverlayPeers<'a, Mode>>,
    #[data_view(schema = Dict(1404))]
    pub wan_rr_overlay_peers: ::validated_data::Field<bgp_peer_groups::WanRrOverlayPeers<'a, Mode>>,
}

pub mod bgp_peer_groups;

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.ConnectedEndpoints", schema = List(576), item_name = "AVDDesign.ConnectedEndpointsItem"))]
pub struct ConnectedEndpoints<'a, Mode> (::validated_data::Field<connected_endpoints::Item<'a, Mode>>);

pub mod connected_endpoints;

#[::validated_data::data_view(indexed_list, primary_key(key), python(name = "AVDDesign.CustomConnectedEndpointsKeys", schema = List(577), item_name = "AVDDesign.CustomConnectedEndpointsKeysItem"))]
pub struct CustomConnectedEndpointsKeys<'a, Mode> (::validated_data::Field<custom_connected_endpoints_keys::Item<'a, Mode>>);

pub mod custom_connected_endpoints_keys;

#[::validated_data::data_view(indexed_list, primary_key(key), python(name = "AVDDesign.ConnectedEndpointsKeys", schema = List(578), item_name = "AVDDesign.ConnectedEndpointsKeysItem"))]
pub struct ConnectedEndpointsKeys<'a, Mode> (::validated_data::Field<connected_endpoints_keys::Item<'a, Mode>>);

pub mod connected_endpoints_keys;

#[::validated_data::data_view(python(name = "AVDDesign.CoreInterfaces", schema = Dict(1446)))]
pub struct CoreInterfaces<'a, Mode> {
    #[data_view(schema = List(579))]
    pub p2p_links_ip_pools: ::validated_data::Field<core_interfaces::P2pLinksIpPools<'a, Mode>>,
    #[data_view(schema = List(589))]
    pub p2p_links_profiles: ::validated_data::Field<core_interfaces::P2pLinksProfiles<'a, Mode>>,
    #[data_view(schema = List(591))]
    pub p2p_links: ::validated_data::Field<core_interfaces::P2pLinks<'a, Mode>>,
}

pub mod core_interfaces;

#[::validated_data::data_view(list, python(name = "AVDDesign.CustomStructuredConfigurationPrefix", schema = List(592)))]
pub struct CustomStructuredConfigurationPrefix<'a, Mode> (::validated_data::Field<&'a str>);

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.CvPathfinderGlobalSites", schema = List(593), item_name = "AVDDesign.CvPathfinderGlobalSitesItem"))]
pub struct CvPathfinderGlobalSites<'a, Mode> (::validated_data::Field<cv_pathfinder_global_sites::Item<'a, Mode>>);

pub mod cv_pathfinder_global_sites;

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.CvPathfinderInternetExitPolicies", schema = List(594), item_name = "AVDDesign.CvPathfinderInternetExitPoliciesItem"))]
pub struct CvPathfinderInternetExitPolicies<'a, Mode> (::validated_data::Field<cv_pathfinder_internet_exit_policies::Item<'a, Mode>>);

pub mod cv_pathfinder_internet_exit_policies;

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.CvPathfinderRegions", schema = List(596), item_name = "AVDDesign.CvPathfinderRegionsItem"))]
pub struct CvPathfinderRegions<'a, Mode> (::validated_data::Field<cv_pathfinder_regions::Item<'a, Mode>>);

pub mod cv_pathfinder_regions;

#[::validated_data::data_view(python(name = "AVDDesign.CvSettings", schema = Dict(1458)))]
pub struct CvSettings<'a, Mode> {
    #[data_view(schema = Dict(1454))]
    pub cvaas: ::validated_data::Field<cv_settings::Cvaas<'a, Mode>>,
    #[data_view(schema = List(599))]
    pub onprem_clusters: ::validated_data::Field<cv_settings::OnpremClusters<'a, Mode>>,
    #[data_view(schema = Dict(1457))]
    pub terminattr: ::validated_data::Field<cv_settings::Terminattr<'a, Mode>>,
    #[data_view(schema = Bool(455))]
    pub set_source_interfaces: ::validated_data::Field<bool>,
}

pub mod cv_settings;

#[::validated_data::data_view(indexed_list, primary_key(hostname), python(name = "AVDDesign.CvTopology", schema = List(601), item_name = "AVDDesign.CvTopologyItem"))]
pub struct CvTopology<'a, Mode> (::validated_data::Field<cv_topology::Item<'a, Mode>>);

pub mod cv_topology;

#[::validated_data::data_view(indexed_list, primary_key(field_type), python(name = "AVDDesign.CvTopologyLevels", schema = List(602), item_name = "AVDDesign.CvTopologyLevelsItem"))]
pub struct CvTopologyLevels<'a, Mode> (::validated_data::Field<cv_topology_levels::Item<'a, Mode>>);

pub mod cv_topology_levels;

#[::validated_data::data_view(list, python(name = "AVDDesign.DefaultInterfaces", schema = List(609)))]
pub struct DefaultInterfaces<'a, Mode> (::validated_data::Field<default_interfaces::Item<'a, Mode>>);

pub mod default_interfaces;

#[::validated_data::data_view(indexed_list, primary_key(node_type), python(name = "AVDDesign.DefaultNodeTypes", schema = List(611), item_name = "AVDDesign.DefaultNodeTypesItem"))]
pub struct DefaultNodeTypes<'a, Mode> (::validated_data::Field<default_node_types::Item<'a, Mode>>);

pub mod default_node_types;

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.DeviceProfiles", schema = List(641), item_name = "AVDDesign.DeviceProfilesItem"))]
pub struct DeviceProfiles<'a, Mode> (::validated_data::Field<device_profiles::Item<'a, Mode>>);

pub mod device_profiles;

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.Devices", schema = List(644), item_name = "AVDDesign.DevicesItem"))]
pub struct Devices<'a, Mode> (::validated_data::Field<devices::Item<'a, Mode>>);

pub mod devices;

#[::validated_data::data_view(python(name = "AVDDesign.DigitalTwin", schema = Dict(1523)))]
pub struct DigitalTwin<'a, Mode> {
    #[data_view(schema = Str(1377))]
    pub environment: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1522))]
    pub fabric: ::validated_data::RequiredValue<digital_twin::Fabric<'a, Mode>, Mode>,
    #[data_view(schema = Bool(491))]
    pub use_default_interfaces_of_digital_twin_platform: ::validated_data::Field<bool>,
}

pub mod digital_twin;

#[::validated_data::data_view(python(name = "AVDDesign.DnsSettings", schema = Dict(1526)))]
pub struct DnsSettings<'a, Mode> {
    #[data_view(schema = Str(1381))]
    pub domain: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(90))]
    pub domain_list: ::validated_data::Field<dns_settings::DomainList<'a, Mode>>,
    #[data_view(schema = List(645))]
    pub servers: ::validated_data::RequiredValue<dns_settings::Servers<'a, Mode>, Mode>,
    #[data_view(schema = List(646))]
    pub vrfs: ::validated_data::Field<dns_settings::Vrfs<'a, Mode>>,
    #[data_view(schema = Bool(492))]
    pub set_source_interfaces: ::validated_data::Field<bool>,
    #[data_view(schema = List(173))]
    pub ip_hosts: ::validated_data::Field<super::eos_config::IpHosts<'a, Mode>>,
}

pub mod dns_settings;

#[::validated_data::data_view(python(name = "AVDDesign.Dot1xSettings", schema = Dict(1542)))]
pub struct Dot1xSettings<'a, Mode> {
    #[data_view(schema = Bool(493))]
    pub enabled: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1527))]
    pub authentication: ::validated_data::Field<dot1x_settings::Authentication<'a, Mode>>,
    #[data_view(schema = Dict(1528))]
    pub accounting: ::validated_data::Field<dot1x_settings::Accounting<'a, Mode>>,
    #[data_view(schema = Bool(497))]
    pub bypass_bpdu: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(498))]
    pub bypass_lldp: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1529))]
    pub dynamic_authorization: ::validated_data::Field<dot1x_settings::DynamicAuthorization<'a, Mode>>,
    #[data_view(schema = Dict(1531))]
    pub mac_based_authentication: ::validated_data::Field<dot1x_settings::MacBasedAuthentication<'a, Mode>>,
    #[data_view(schema = Dict(1532))]
    pub radius_av_pairs: ::validated_data::Field<dot1x_settings::RadiusAvPairs<'a, Mode>>,
    #[data_view(schema = Dict(1540))]
    pub device_profiling: ::validated_data::Field<dot1x_settings::DeviceProfiling<'a, Mode>>,
    #[data_view(schema = Bool(506))]
    pub redistribute_in_evpn: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1541))]
    pub web_authentication: ::validated_data::Field<dot1x_settings::WebAuthentication<'a, Mode>>,
}

pub mod dot1x_settings;

#[::validated_data::data_view(list, python(name = "AVDDesign.EosDesignsCustomTemplates", schema = List(650)))]
pub struct EosDesignsCustomTemplates<'a, Mode> (::validated_data::Field<eos_designs_custom_templates::Item<'a, Mode>>);

pub mod eos_designs_custom_templates;

#[::validated_data::data_view(python(name = "AVDDesign.EosDesignsDocumentation", schema = Dict(1545)))]
pub struct EosDesignsDocumentation<'a, Mode> {
    #[data_view(schema = Bool(511))]
    pub enable: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(512))]
    pub connected_endpoints: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(513))]
    pub topology_csv: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(514))]
    pub p2p_links_csv: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(515))]
    pub toc: ::validated_data::Field<bool>,
}

#[::validated_data::data_view(python(name = "AVDDesign.ErrdisableSettings", schema = Dict(1550)))]
pub struct ErrdisableSettings<'a, Mode> {
    #[data_view(schema = Int(570))]
    pub recovery_interval: ::validated_data::Field<i64>,
    #[data_view(schema = Dict(1549))]
    pub causes: ::validated_data::Field<errdisable_settings::Causes<'a, Mode>>,
}

pub mod errdisable_settings;

#[::validated_data::data_view(python(name = "AVDDesign.EvpnHostflapDetection", schema = Dict(1551)))]
pub struct EvpnHostflapDetection<'a, Mode> {
    #[data_view(schema = Bool(519))]
    pub enabled: ::validated_data::Field<bool>,
    #[data_view(schema = Int(573))]
    pub threshold: ::validated_data::Field<i64>,
    #[data_view(schema = Int(574))]
    pub window: ::validated_data::Field<i64>,
    #[data_view(schema = Int(575))]
    pub expiry_timeout: ::validated_data::Field<i64>,
}

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.EvpnVlanBundles", schema = List(651), item_name = "AVDDesign.EvpnVlanBundlesItem"))]
pub struct EvpnVlanBundles<'a, Mode> (::validated_data::Field<evpn_vlan_bundles::Item<'a, Mode>>);

pub mod evpn_vlan_bundles;

#[::validated_data::data_view(python(name = "AVDDesign.FabricFlowTracking", schema = Dict(1564)))]
pub struct FabricFlowTracking<'a, Mode> {
    #[data_view(schema = Dict(1554))]
    pub uplinks: ::validated_data::Field<fabric_flow_tracking::Uplinks<'a, Mode>>,
    #[data_view(schema = Dict(1555))]
    pub downlinks: ::validated_data::Field<fabric_flow_tracking::Downlinks<'a, Mode>>,
    #[data_view(schema = Dict(1556))]
    pub endpoints: ::validated_data::Field<fabric_flow_tracking::Endpoints<'a, Mode>>,
    #[data_view(schema = Dict(1557))]
    pub l3_edge: ::validated_data::Field<fabric_flow_tracking::L3Edge<'a, Mode>>,
    #[data_view(schema = Dict(1558))]
    pub core_interfaces: ::validated_data::Field<fabric_flow_tracking::CoreInterfaces<'a, Mode>>,
    #[data_view(schema = Dict(1559))]
    pub mlag_interfaces: ::validated_data::Field<fabric_flow_tracking::MlagInterfaces<'a, Mode>>,
    #[data_view(schema = Dict(1560))]
    pub l3_interfaces: ::validated_data::Field<fabric_flow_tracking::L3Interfaces<'a, Mode>>,
    #[data_view(schema = Dict(1561))]
    pub l3_port_channels: ::validated_data::Field<fabric_flow_tracking::L3PortChannels<'a, Mode>>,
    #[data_view(schema = Dict(1562))]
    pub dps_interfaces: ::validated_data::Field<fabric_flow_tracking::DpsInterfaces<'a, Mode>>,
    #[data_view(schema = Dict(1563))]
    pub direct_wan_ha_links: ::validated_data::Field<fabric_flow_tracking::DirectWanHaLinks<'a, Mode>>,
}

pub mod fabric_flow_tracking;

#[::validated_data::data_view(python(name = "AVDDesign.FabricIpAddressing", schema = Dict(1569)))]
pub struct FabricIpAddressing<'a, Mode> {
    #[data_view(schema = Dict(1565))]
    pub loopback: ::validated_data::Field<fabric_ip_addressing::Loopback<'a, Mode>>,
    #[data_view(schema = Dict(1566))]
    pub mlag: ::validated_data::Field<fabric_ip_addressing::Mlag<'a, Mode>>,
    #[data_view(schema = Dict(1567))]
    pub p2p_uplinks: ::validated_data::Field<fabric_ip_addressing::P2pUplinks<'a, Mode>>,
    #[data_view(schema = Dict(1568))]
    pub wan_ha: ::validated_data::Field<fabric_ip_addressing::WanHa<'a, Mode>>,
}

pub mod fabric_ip_addressing;

#[::validated_data::data_view(python(name = "AVDDesign.FabricNumbering", schema = Dict(1571)))]
pub struct FabricNumbering<'a, Mode> {
    #[data_view(schema = Dict(1570))]
    pub node_id: ::validated_data::Field<fabric_numbering::NodeId<'a, Mode>>,
}

pub mod fabric_numbering;

#[::validated_data::data_view(python(name = "AVDDesign.FabricSflow", schema = Dict(1572)))]
pub struct FabricSflow<'a, Mode> {
    #[data_view(schema = Bool(526))]
    pub uplinks: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(527))]
    pub downlinks: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(528))]
    pub endpoints: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(529))]
    pub l3_edge: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(530))]
    pub core_interfaces: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(531))]
    pub mlag_interfaces: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(532))]
    pub l3_interfaces: ::validated_data::Field<bool>,
}

#[::validated_data::data_view(python(name = "AVDDesign.FlowTrackingSettings", schema = Dict(1581)))]
pub struct FlowTrackingSettings<'a, Mode> {
    #[data_view(schema = Dict(1573))]
    pub sampled: ::validated_data::Field<flow_tracking_settings::Sampled<'a, Mode>>,
    #[data_view(schema = Dict(1574))]
    pub hardware: ::validated_data::Field<flow_tracking_settings::Hardware<'a, Mode>>,
    #[data_view(schema = Dict(1575))]
    pub cloudvision_exporter: ::validated_data::Field<flow_tracking_settings::CloudvisionExporter<'a, Mode>>,
    #[data_view(schema = List(653))]
    pub trackers: ::validated_data::Field<flow_tracking_settings::Trackers<'a, Mode>>,
}

pub mod flow_tracking_settings;

#[::validated_data::data_view(python(name = "AVDDesign.GeneralSettings", schema = Dict(1587)))]
pub struct GeneralSettings<'a, Mode> {
    #[data_view(schema = Dict(1582))]
    pub interface_defaults: ::validated_data::Field<general_settings::InterfaceDefaults<'a, Mode>>,
    #[data_view(schema = Dict(1583))]
    pub arp: ::validated_data::Field<general_settings::Arp<'a, Mode>>,
    #[data_view(schema = Bool(7))]
    pub ip_icmp_redirect: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1584))]
    pub dhcp_relay: ::validated_data::Field<general_settings::DhcpRelay<'a, Mode>>,
    #[data_view(schema = List(654))]
    pub suspended_vlans: ::validated_data::Field<general_settings::SuspendedVlans<'a, Mode>>,
    #[data_view(schema = Dict(1586))]
    pub ip_software_forwarding_exceed_action_drop: ::validated_data::Field<general_settings::IpSoftwareForwardingExceedActionDrop<'a, Mode>>,
}

pub mod general_settings;

#[::validated_data::data_view(python(name = "AVDDesign.GenerateCvTags", schema = Dict(1590)))]
pub struct GenerateCvTags<'a, Mode> {
    #[data_view(schema = Bool(538))]
    pub topology_hints: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(539))]
    pub campus_fabric: ::validated_data::Field<bool>,
    #[data_view(schema = List(655))]
    pub interface_tags: ::validated_data::Field<generate_cv_tags::InterfaceTags<'a, Mode>>,
    #[data_view(schema = List(656))]
    pub device_tags: ::validated_data::Field<generate_cv_tags::DeviceTags<'a, Mode>>,
}

pub mod generate_cv_tags;

#[::validated_data::data_view(python(name = "AVDDesign.InternalVlanOrder", schema = Dict(1593)))]
pub struct InternalVlanOrder<'a, Mode> {
    #[data_view(schema = Str(1063))]
    pub allocation: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Dict(1592))]
    pub range: ::validated_data::Field<internal_vlan_order::Range<'a, Mode>>,
}

pub mod internal_vlan_order;

#[::validated_data::data_view(python(name = "AVDDesign.IpsecSettings", schema = Dict(1594)))]
pub struct IpsecSettings<'a, Mode> {
    #[data_view(schema = Bool(540))]
    pub bind_connection_to_interface: ::validated_data::Field<bool>,
}

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.Ipv4Acls", schema = List(658), item_name = "AVDDesign.Ipv4AclsItem"))]
pub struct Ipv4Acls<'a, Mode> (::validated_data::Field<ipv4_acls::Item<'a, Mode>>);

pub mod ipv4_acls;

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.Ipv4PrefixListCatalog", schema = List(660), item_name = "AVDDesign.Ipv4PrefixListCatalogItem"))]
pub struct Ipv4PrefixListCatalog<'a, Mode> (::validated_data::Field<ipv4_prefix_list_catalog::Item<'a, Mode>>);

pub mod ipv4_prefix_list_catalog;

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.Ipv4StandardAcls", schema = List(661), item_name = "AVDDesign.Ipv4StandardAclsItem"))]
pub struct Ipv4StandardAcls<'a, Mode> (::validated_data::Field<ipv4_standard_acls::Item<'a, Mode>>);

pub mod ipv4_standard_acls;

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.Ipv6Acls", schema = List(663), item_name = "AVDDesign.Ipv6AclsItem"))]
pub struct Ipv6Acls<'a, Mode> (::validated_data::Field<ipv6_acls::Item<'a, Mode>>);

pub mod ipv6_acls;

#[::validated_data::data_view(list, python(name = "AVDDesign.Ipv6MgmtDestinationNetworks", schema = List(664)))]
pub struct Ipv6MgmtDestinationNetworks<'a, Mode> (::validated_data::Field<&'a str>);

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.Ipv6PrefixListCatalog", schema = List(666), item_name = "AVDDesign.Ipv6PrefixListCatalogItem"))]
pub struct Ipv6PrefixListCatalog<'a, Mode> (::validated_data::Field<ipv6_prefix_list_catalog::Item<'a, Mode>>);

pub mod ipv6_prefix_list_catalog;

#[::validated_data::data_view(python(name = "AVDDesign.IsisTiLfa", schema = Dict(1604)))]
pub struct IsisTiLfa<'a, Mode> {
    #[data_view(schema = Bool(340))]
    pub enabled: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1430))]
    pub protection: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(592))]
    pub local_convergence_delay: ::validated_data::Field<i64>,
}

#[::validated_data::data_view(indexed_list, primary_key(profile), python(name = "AVDDesign.L2vlanProfiles", schema = List(668), item_name = "AVDDesign.L2vlanProfilesItem"))]
pub struct L2vlanProfiles<'a, Mode> (::validated_data::Field<l2vlan_profiles::Item<'a, Mode>>);

pub mod l2vlan_profiles;

#[::validated_data::data_view(python(name = "AVDDesign.L3Edge", schema = Dict(1446)))]
pub struct L3Edge<'a, Mode> {
    #[data_view(schema = List(579))]
    pub p2p_links_ip_pools: ::validated_data::Field<l3_edge::P2pLinksIpPools<'a, Mode>>,
    #[data_view(schema = List(589))]
    pub p2p_links_profiles: ::validated_data::Field<l3_edge::P2pLinksProfiles<'a, Mode>>,
    #[data_view(schema = List(591))]
    pub p2p_links: ::validated_data::Field<l3_edge::P2pLinks<'a, Mode>>,
}

pub mod l3_edge;

#[::validated_data::data_view(indexed_list, primary_key(profile), python(name = "AVDDesign.L3InterfaceProfiles", schema = List(669), item_name = "AVDDesign.L3InterfaceProfilesItem"))]
pub struct L3InterfaceProfiles<'a, Mode> (::validated_data::Field<l3_interface_profiles::Item<'a, Mode>>);

pub mod l3_interface_profiles;

#[::validated_data::data_view(python(name = "AVDDesign.LoggingSettings", schema = Dict(1617)))]
pub struct LoggingSettings<'a, Mode> {
    #[data_view(schema = Bool(551))]
    pub use_local_interface_cli: ::validated_data::Field<bool>,
    #[data_view(schema = List(671))]
    pub hosts: ::validated_data::Field<logging_settings::Hosts<'a, Mode>>,
    #[data_view(schema = List(672))]
    pub vrfs: ::validated_data::Field<logging_settings::Vrfs<'a, Mode>>,
    #[data_view(schema = Str(455))]
    pub console: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(456))]
    pub monitor: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(513))]
    pub buffered: ::validated_data::Field<super::eos_config::logging::Buffered<'a, Mode>>,
    #[data_view(schema = Bool(230))]
    pub repeat_messages: ::validated_data::Field<bool>,
    #[data_view(schema = Str(458))]
    pub trap: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(514))]
    pub synchronous: ::validated_data::Field<super::eos_config::logging::Synchronous<'a, Mode>>,
    #[data_view(schema = Dict(515))]
    pub format: ::validated_data::Field<super::eos_config::logging::Format<'a, Mode>>,
    #[data_view(schema = Str(462))]
    pub facility: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(520))]
    pub policy: ::validated_data::Field<super::eos_config::logging::Policy<'a, Mode>>,
    #[data_view(schema = Dict(523))]
    pub event: ::validated_data::Field<super::eos_config::logging::Event<'a, Mode>>,
    #[data_view(schema = List(221))]
    pub level: ::validated_data::Field<super::eos_config::logging::Level<'a, Mode>>,
    #[data_view(schema = Dict(652))]
    pub monitor_layer1: ::validated_data::Field<super::eos_config::MonitorLayer1<'a, Mode>>,
}

pub mod logging_settings;

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.MacAcls", schema = List(674), item_name = "AVDDesign.MacAclsItem"))]
pub struct MacAcls<'a, Mode> (::validated_data::Field<mac_acls::Item<'a, Mode>>);

pub mod mac_acls;

#[::validated_data::data_view(python(name = "AVDDesign.ManagementEapi", schema = Dict(1622)))]
pub struct ManagementEapi<'a, Mode> {
    #[data_view(schema = Bool(552))]
    pub enabled: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(7))]
    pub enable_http: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(37))]
    pub enable_https: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(7))]
    pub default_services: ::validated_data::Field<bool>,
    #[data_view(schema = List(675))]
    pub vrfs: ::validated_data::Field<management_eapi::Vrfs<'a, Mode>>,
}

pub mod management_eapi;

#[::validated_data::data_view(python(name = "AVDDesign.ManagementSettings", schema = Dict(1623)))]
pub struct ManagementSettings<'a, Mode> {
    #[data_view(schema = Dict(575))]
    pub console: ::validated_data::Field<super::eos_config::ManagementConsole<'a, Mode>>,
    #[data_view(schema = Dict(85))]
    pub banners: ::validated_data::Field<super::eos_config::Banners<'a, Mode>>,
}

pub mod management_settings;

#[::validated_data::data_view(list, python(name = "AVDDesign.MgmtDestinationNetworks", schema = List(676)))]
pub struct MgmtDestinationNetworks<'a, Mode> (::validated_data::Field<&'a str>);

#[::validated_data::data_view(python(name = "AVDDesign.MgmtInterfaceSettings", schema = Dict(1625)))]
pub struct MgmtInterfaceSettings<'a, Mode> {
    #[data_view(schema = Str(1456))]
    pub description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1457))]
    pub vrf: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(554))]
    pub vrf_routing: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1458))]
    pub interface: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1624))]
    pub lldp: ::validated_data::Field<mgmt_interface_settings::Lldp<'a, Mode>>,
}

pub mod mgmt_interface_settings;

#[::validated_data::data_view(python(name = "AVDDesign.MlagIbgpPeeringVrfs", schema = Dict(1626)))]
pub struct MlagIbgpPeeringVrfs<'a, Mode> {
    #[data_view(schema = Int(598))]
    pub base_vlan: ::validated_data::Field<i64>,
}

#[::validated_data::data_view(python(name = "AVDDesign.MonitorConnectivity", schema = Dict(1631)))]
pub struct MonitorConnectivity<'a, Mode> {
    #[data_view(schema = Bool(7))]
    pub shutdown: ::validated_data::Field<bool>,
    #[data_view(schema = Int(77))]
    pub interval: ::validated_data::Field<i64>,
    #[data_view(schema = List(678))]
    pub interface_sets: ::validated_data::Field<monitor_connectivity::InterfaceSets<'a, Mode>>,
    #[data_view(schema = Str(3))]
    pub local_interfaces: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(559))]
    pub address_only: ::validated_data::Field<bool>,
    #[data_view(schema = List(679))]
    pub hosts: ::validated_data::Field<monitor_connectivity::Hosts<'a, Mode>>,
    #[data_view(schema = Str(584))]
    pub name_server_group: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(681))]
    pub vrfs: ::validated_data::Field<monitor_connectivity::Vrfs<'a, Mode>>,
}

pub mod monitor_connectivity;

#[::validated_data::data_view(list, python(name = "AVDDesign.NetworkPorts", schema = List(686)))]
pub struct NetworkPorts<'a, Mode> (::validated_data::Field<network_ports::Item<'a, Mode>>);

pub mod network_ports;

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.NetworkServices", schema = List(753), item_name = "AVDDesign.NetworkServicesItem"))]
pub struct NetworkServices<'a, Mode> (::validated_data::Field<network_services::Item<'a, Mode>>);

pub mod network_services;

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.NetworkServicesKeys", schema = List(754), item_name = "AVDDesign.NetworkServicesKeysItem"))]
pub struct NetworkServicesKeys<'a, Mode> (::validated_data::Field<network_services_keys::Item<'a, Mode>>);

pub mod network_services_keys;

#[::validated_data::data_view(indexed_list, primary_key(key), python(name = "AVDDesign.CustomNodeTypeKeys", schema = List(756), item_name = "AVDDesign.CustomNodeTypeKeysItem"))]
pub struct CustomNodeTypeKeys<'a, Mode> (::validated_data::Field<custom_node_type_keys::Item<'a, Mode>>);

pub mod custom_node_type_keys;

#[::validated_data::data_view(indexed_list, primary_key(key), python(name = "AVDDesign.NodeTypeKeys", schema = List(757), item_name = "AVDDesign.NodeTypeKeysItem"))]
pub struct NodeTypeKeys<'a, Mode> (::validated_data::Field<node_type_keys::Item<'a, Mode>>);

pub mod node_type_keys;

#[::validated_data::data_view(python(name = "AVDDesign.NtpSettings", schema = Dict(1713)))]
pub struct NtpSettings<'a, Mode> {
    #[data_view(schema = Str(1592))]
    pub server_vrf: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(608))]
    pub set_first_ntp_server_as_preferred: ::validated_data::Field<bool>,
    #[data_view(schema = List(758))]
    pub servers: ::validated_data::Field<ntp_settings::Servers<'a, Mode>>,
    #[data_view(schema = Bool(7))]
    pub authenticate: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(7))]
    pub authenticate_servers_only: ::validated_data::Field<bool>,
    #[data_view(schema = List(759))]
    pub authentication_keys: ::validated_data::Field<ntp_settings::AuthenticationKeys<'a, Mode>>,
    #[data_view(schema = Str(633))]
    pub trusted_keys: ::validated_data::Field<&'a str>,
}

pub mod ntp_settings;

#[::validated_data::data_view(list, python(name = "AVDDesign.OverlayCvxServers", schema = List(760)))]
pub struct OverlayCvxServers<'a, Mode> (::validated_data::Field<&'a str>);

#[::validated_data::data_view(python(name = "AVDDesign.OverlayRdType", schema = Dict(1714)))]
pub struct OverlayRdType<'a, Mode> {
    #[data_view(schema = Str(1601))]
    pub admin_subfield: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(619))]
    pub admin_subfield_offset: ::validated_data::Field<i64>,
    #[data_view(schema = Str(1602))]
    pub vrf_admin_subfield: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(620))]
    pub vrf_admin_subfield_offset: ::validated_data::Field<i64>,
    #[data_view(schema = Str(1603))]
    pub vlan_assigned_number_subfield: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesign.OverlayRtType", schema = Dict(1715)))]
pub struct OverlayRtType<'a, Mode> {
    #[data_view(schema = Str(1606))]
    pub admin_subfield: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1607))]
    pub vrf_admin_subfield: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1608))]
    pub vlan_assigned_number_subfield: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(list, python(name = "AVDDesign.CustomPlatformSettings", schema = List(763)))]
pub struct CustomPlatformSettings<'a, Mode> (::validated_data::Field<custom_platform_settings::Item<'a, Mode>>);

pub mod custom_platform_settings;

#[::validated_data::data_view(list, python(name = "AVDDesign.PlatformSettings", schema = List(764)))]
pub struct PlatformSettings<'a, Mode> (::validated_data::Field<platform_settings::Item<'a, Mode>>);

pub mod platform_settings;

#[::validated_data::data_view(indexed_list, primary_key(platform), python(name = "AVDDesign.PlatformSpeedGroups", schema = List(766), item_name = "AVDDesign.PlatformSpeedGroupsItem"))]
pub struct PlatformSpeedGroups<'a, Mode> (::validated_data::Field<platform_speed_groups::Item<'a, Mode>>);

pub mod platform_speed_groups;

#[::validated_data::data_view(indexed_list, primary_key(profile), python(name = "AVDDesign.PortProfiles", schema = List(768), item_name = "AVDDesign.PortProfilesItem"))]
pub struct PortProfiles<'a, Mode> (::validated_data::Field<port_profiles::Item<'a, Mode>>);

pub mod port_profiles;

#[::validated_data::data_view(indexed_list, primary_key(profile), python(name = "AVDDesign.PtpProfiles", schema = List(769), item_name = "AVDDesign.PtpProfilesItem"))]
pub struct PtpProfiles<'a, Mode> (::validated_data::Field<ptp_profiles::Item<'a, Mode>>);

pub mod ptp_profiles;

#[::validated_data::data_view(python(name = "AVDDesign.PtpSettings", schema = Dict(1735)))]
pub struct PtpSettings<'a, Mode> {
    #[data_view(schema = Bool(7))]
    pub enabled: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1154))]
    pub profile: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(629))]
    pub domain: ::validated_data::Field<i64>,
    #[data_view(schema = Bool(37))]
    pub auto_clock_identity: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(639))]
    pub forward_v1: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(818))]
    pub free_running: ::validated_data::Field<super::eos_config::ptp::FreeRunning<'a, Mode>>,
}

pub mod ptp_settings;

#[::validated_data::data_view(python(name = "AVDDesign.QueueMonitorLength", schema = Dict(1736)))]
pub struct QueueMonitorLength<'a, Mode> {
    #[data_view(schema = Bool(1))]
    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    #[data_view(schema = Bool(640))]
    pub notifying: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(847))]
    pub default_thresholds: ::validated_data::Field<queue_monitor_length::DefaultThresholds<'a, Mode>>,
    #[data_view(schema = Int(345))]
    pub log: ::validated_data::Field<i64>,
    #[data_view(schema = Dict(849))]
    pub cpu: ::validated_data::Field<queue_monitor_length::Cpu<'a, Mode>>,
    #[data_view(schema = Bool(303))]
    pub tx_latency: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(852))]
    pub mirror: ::validated_data::Field<queue_monitor_length::Mirror<'a, Mode>>,
}

pub mod queue_monitor_length;

#[::validated_data::data_view(python(name = "AVDDesign.Redundancy", schema = Dict(1738)))]
pub struct Redundancy<'a, Mode> {
    #[data_view(schema = Str(1620))]
    pub protocol: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesign.SflowSettings", schema = Dict(1743)))]
pub struct SflowSettings<'a, Mode> {
    #[data_view(schema = Int(630))]
    pub polling_interval: ::validated_data::Field<i64>,
    #[data_view(schema = Dict(1739))]
    pub sample: ::validated_data::Field<sflow_settings::Sample<'a, Mode>>,
    #[data_view(schema = List(770))]
    pub destinations: ::validated_data::Field<sflow_settings::Destinations<'a, Mode>>,
    #[data_view(schema = Dict(1741))]
    pub export_to_cloudvision: ::validated_data::Field<sflow_settings::ExportToCloudvision<'a, Mode>>,
    #[data_view(schema = List(771))]
    pub vrfs: ::validated_data::Field<sflow_settings::Vrfs<'a, Mode>>,
}

pub mod sflow_settings;

#[::validated_data::data_view(python(name = "AVDDesign.SnmpSettings", schema = Dict(1749)))]
pub struct SnmpSettings<'a, Mode> {
    #[data_view(schema = Str(948))]
    pub contact: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(644))]
    pub location: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1627))]
    pub location_template: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(772))]
    pub vrfs: ::validated_data::Field<snmp_settings::Vrfs<'a, Mode>>,
    #[data_view(schema = Bool(646))]
    pub compute_local_engineid: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1630))]
    pub compute_local_engineid_source: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1631))]
    pub local_engineid_ip: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(647))]
    pub compute_v3_user_localized_key: ::validated_data::Field<bool>,
    #[data_view(schema = List(773))]
    pub users: ::validated_data::Field<snmp_settings::Users<'a, Mode>>,
    #[data_view(schema = List(774))]
    pub hosts: ::validated_data::Field<snmp_settings::Hosts<'a, Mode>>,
    #[data_view(schema = List(775))]
    pub communities: ::validated_data::Field<snmp_settings::Communities<'a, Mode>>,
    #[data_view(schema = List(491))]
    pub views: ::validated_data::Field<snmp_settings::Views<'a, Mode>>,
    #[data_view(schema = List(492))]
    pub groups: ::validated_data::Field<snmp_settings::Groups<'a, Mode>>,
    #[data_view(schema = Dict(1243))]
    pub traps: ::validated_data::Field<super::eos_config::snmp_server::Traps<'a, Mode>>,
}

pub mod snmp_settings;

#[::validated_data::data_view(python(name = "AVDDesign.SourceInterfaces", schema = Dict(1756)))]
pub struct SourceInterfaces<'a, Mode> {
    #[data_view(schema = Dict(1750))]
    pub http_client: ::validated_data::Field<source_interfaces::HttpClient<'a, Mode>>,
    #[data_view(schema = Dict(1751))]
    pub ssh_client: ::validated_data::Field<source_interfaces::SshClient<'a, Mode>>,
}

pub mod source_interfaces;

#[::validated_data::data_view(python(name = "AVDDesign.SpanningTreeSettings", schema = Dict(1758)))]
pub struct SpanningTreeSettings<'a, Mode> {
    #[data_view(schema = Str(1638))]
    pub mode: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(633))]
    pub priority: ::validated_data::Field<i64>,
    #[data_view(schema = Dict(1757))]
    pub port_id_allocation_port_channel_range: ::validated_data::Field<super::eos_config::spanning_tree::PortIdAllocationPortChannelRange<'a, Mode>>,
    #[data_view(schema = Bool(656))]
    pub loop_guard_default: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(657))]
    pub edge_port_bpduguard_default: ::validated_data::Field<bool>,
}

pub mod spanning_tree_settings;

#[::validated_data::data_view(python(name = "AVDDesign.SshSettings", schema = Dict(1761)))]
pub struct SshSettings<'a, Mode> {
    #[data_view(schema = Bool(658))]
    pub enabled: ::validated_data::Field<bool>,
    #[data_view(schema = List(778))]
    pub vrfs: ::validated_data::Field<ssh_settings::Vrfs<'a, Mode>>,
    #[data_view(schema = Int(235))]
    pub idle_timeout: ::validated_data::Field<i64>,
    #[data_view(schema = List(779))]
    pub client_vrfs: ::validated_data::Field<ssh_settings::ClientVrfs<'a, Mode>>,
}

pub mod ssh_settings;

#[::validated_data::data_view(indexed_list, primary_key(profile), python(name = "AVDDesign.SviProfiles", schema = List(781), item_name = "AVDDesign.SviProfilesItem"))]
pub struct SviProfiles<'a, Mode> (::validated_data::Field<svi_profiles::Item<'a, Mode>>);

pub mod svi_profiles;

#[::validated_data::data_view(python(name = "AVDDesign.TrunkGroups", schema = Dict(1767)))]
pub struct TrunkGroups<'a, Mode> {
    #[data_view(schema = Dict(1764))]
    pub mlag: ::validated_data::Field<trunk_groups::Mlag<'a, Mode>>,
    #[data_view(schema = Dict(1765))]
    pub mlag_l3: ::validated_data::Field<trunk_groups::MlagL3<'a, Mode>>,
    #[data_view(schema = Dict(1766))]
    pub uplink: ::validated_data::Field<trunk_groups::Uplink<'a, Mode>>,
}

pub mod trunk_groups;

#[::validated_data::data_view(python(name = "AVDDesign.UnderlayMulticastAnycastRp", schema = Dict(1768)))]
pub struct UnderlayMulticastAnycastRp<'a, Mode> {
    #[data_view(schema = Str(1654))]
    pub mode: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(indexed_list, primary_key(rp), python(name = "AVDDesign.UnderlayMulticastRps", schema = List(785), item_name = "AVDDesign.UnderlayMulticastRpsItem"))]
pub struct UnderlayMulticastRps<'a, Mode> (::validated_data::Field<underlay_multicast_rps::Item<'a, Mode>>);

pub mod underlay_multicast_rps;

#[::validated_data::data_view(python(name = "AVDDesign.UnderlayOspfAuthentication", schema = Dict(1772)))]
pub struct UnderlayOspfAuthentication<'a, Mode> {
    #[data_view(schema = Bool(669))]
    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    #[data_view(schema = List(786))]
    pub message_digest_keys: ::validated_data::RequiredValue<underlay_ospf_authentication::MessageDigestKeys<'a, Mode>, Mode>,
}

pub mod underlay_ospf_authentication;

#[::validated_data::data_view(python(name = "AVDDesign.UplinkPtp", schema = Dict(1773)))]
pub struct UplinkPtp<'a, Mode> {
    #[data_view(schema = Bool(340))]
    pub enable: ::validated_data::Field<bool>,
}

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.ValidationProfiles", schema = List(787), item_name = "AVDDesign.ValidationProfilesItem"))]
pub struct ValidationProfiles<'a, Mode> (::validated_data::Field<validation_profiles::Item<'a, Mode>>);

pub mod validation_profiles;

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.WanCarriers", schema = List(788), item_name = "AVDDesign.WanCarriersItem"))]
pub struct WanCarriers<'a, Mode> (::validated_data::Field<wan_carriers::Item<'a, Mode>>);

pub mod wan_carriers;

#[::validated_data::data_view(python(name = "AVDDesign.WanHa", schema = Dict(1778)))]
pub struct WanHa<'a, Mode> {
    #[data_view(schema = Str(1670))]
    pub lan_ha_path_group_name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "AVDDesign.WanIpsecProfiles", schema = Dict(1781)))]
pub struct WanIpsecProfiles<'a, Mode> {
    #[data_view(schema = Dict(1779))]
    pub control_plane: ::validated_data::RequiredValue<wan_ipsec_profiles::ControlPlane<'a, Mode>, Mode>,
    #[data_view(schema = Dict(1780))]
    pub data_plane: ::validated_data::Field<wan_ipsec_profiles::DataPlane<'a, Mode>>,
}

pub mod wan_ipsec_profiles;

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.WanPathGroups", schema = List(790), item_name = "AVDDesign.WanPathGroupsItem"))]
pub struct WanPathGroups<'a, Mode> (::validated_data::Field<wan_path_groups::Item<'a, Mode>>);

pub mod wan_path_groups;

#[::validated_data::data_view(indexed_list, primary_key(hostname), python(name = "AVDDesign.WanRouteServers", schema = List(793), item_name = "AVDDesign.WanRouteServersItem"))]
pub struct WanRouteServers<'a, Mode> (::validated_data::Field<wan_route_servers::Item<'a, Mode>>);

pub mod wan_route_servers;

#[::validated_data::data_view(python(name = "AVDDesign.WanVirtualTopologies", schema = Dict(1797)))]
pub struct WanVirtualTopologies<'a, Mode> {
    #[data_view(schema = List(794))]
    pub vrfs: ::validated_data::Field<wan_virtual_topologies::Vrfs<'a, Mode>>,
    #[data_view(schema = Dict(1793))]
    pub control_plane_virtual_topology: ::validated_data::Field<wan_virtual_topologies::ControlPlaneVirtualTopology<'a, Mode>>,
    #[data_view(schema = List(798))]
    pub policies: ::validated_data::Field<wan_virtual_topologies::Policies<'a, Mode>>,
}

pub mod wan_virtual_topologies;

#[::validated_data::data_view(python(name = "AVDDesign.ZscalerEndpoints", schema = Dict(1801)))]
pub struct ZscalerEndpoints<'a, Mode> {
    #[data_view(schema = Dict(1798))]
    pub primary: ::validated_data::RequiredValue<zscaler_endpoints::Primary<'a, Mode>, Mode>,
    #[data_view(schema = Dict(1799))]
    pub secondary: ::validated_data::Field<zscaler_endpoints::Secondary<'a, Mode>>,
    #[data_view(schema = Dict(1799))]
    pub tertiary: ::validated_data::Field<zscaler_endpoints::Tertiary<'a, Mode>>,
    #[data_view(schema = Str(1699))]
    pub cloud_name: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Dict(1800))]
    pub device_location: ::validated_data::RequiredValue<zscaler_endpoints::DeviceLocation<'a, Mode>, Mode>,
}

pub mod zscaler_endpoints;

#[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.CustomNodeTypes", schema = Dict(1805)))]
pub struct DynamicSlot12232<'a, Mode> {
    #[data_view(schema = Dict(1802))]
    pub defaults: ::validated_data::Field<dynamic_slot_12232::Defaults<'a, Mode>>,
    #[data_view(schema = List(800))]
    pub node_groups: ::validated_data::Field<dynamic_slot_12232::NodeGroups<'a, Mode>>,
    #[data_view(schema = List(799))]
    pub nodes: ::validated_data::Field<dynamic_slot_12232::Nodes<'a, Mode>>,
}

pub mod dynamic_slot_12232;

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign._DynamicKeys.ConnectedEndpoints", schema = List(801), item_name = "AVDDesign._DynamicKeys.ConnectedEndpointsItem"))]
pub struct DynamicSlot13731<'a, Mode> (::validated_data::Field<dynamic_slot_13731::Item<'a, Mode>>);

pub mod dynamic_slot_13731;

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign._DynamicKeys.CustomConnectedEndpoints", schema = List(802), item_name = "AVDDesign._DynamicKeys.CustomConnectedEndpointsItem"))]
pub struct DynamicSlot13919<'a, Mode> (::validated_data::Field<dynamic_slot_13919::Item<'a, Mode>>);

pub mod dynamic_slot_13919;

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign._DynamicKeys.NetworkServices", schema = List(803), item_name = "AVDDesign._DynamicKeys.NetworkServicesItem"))]
pub struct DynamicSlot14107<'a, Mode> (::validated_data::Field<dynamic_slot_14107::Item<'a, Mode>>);

pub mod dynamic_slot_14107;

#[::validated_data::data_view(python(name = "AVDDesign._DynamicKeys.NodeTypes", schema = Dict(1807)))]
pub struct DynamicSlot15029<'a, Mode> {
    #[data_view(schema = Dict(1802))]
    pub defaults: ::validated_data::Field<dynamic_slot_15029::Defaults<'a, Mode>>,
    #[data_view(schema = List(800))]
    pub node_groups: ::validated_data::Field<dynamic_slot_15029::NodeGroups<'a, Mode>>,
    #[data_view(schema = List(799))]
    pub nodes: ::validated_data::Field<dynamic_slot_15029::Nodes<'a, Mode>>,
}

pub mod dynamic_slot_15029;
