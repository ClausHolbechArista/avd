// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct AvdDesign<'a, Mode> {
    pub aaa_settings: ::validated_data::Field<AaaSettings<'a, Mode>>,
    pub address_locking_settings: ::validated_data::Field<AddressLockingSettings<'a, Mode>>,
    pub application_classification: ::validated_data::Field<super::eos_cli_config_gen::ApplicationTrafficRecognition<'a, Mode>>,
    pub avd_design_future: ::validated_data::Field<AvdDesignFuture<'a, Mode>>,
    pub avd_digital_twin_mode: ::validated_data::Field<bool>,
    pub avd_eos_designs_structured_config: ::validated_data::Field<bool>,
    pub avd_structured_config_file_format: ::validated_data::Field<&'a str>,
    pub eos_designs_validation_configuration: ::validated_data::Field<EosDesignsValidationConfiguration<'a, Mode>>,
    pub avd_vault_id: ::validated_data::Field<&'a str>,
    pub bfd_multihop: ::validated_data::Field<BfdMultihop<'a, Mode>>,
    pub bgp_as: ::validated_data::Field<&'a str>,
    pub bgp_as_notation: ::validated_data::Field<&'a str>,
    pub bgp_default_ipv4_unicast: ::validated_data::Field<bool>,
    pub bgp_distance: ::validated_data::Field<super::eos_cli_config_gen::router_bgp::Distance<'a, Mode>>,
    pub bgp_ecmp: ::validated_data::Field<i64>,
    pub bgp_graceful_restart: ::validated_data::Field<BgpGracefulRestart<'a, Mode>>,
    pub bgp_maximum_paths: ::validated_data::Field<i64>,
    pub bgp_mesh_pes: ::validated_data::Field<bool>,
    pub bgp_peer_filters_catalog: ::validated_data::Field<super::eos_cli_config_gen::PeerFilters<'a, Mode>>,
    pub bgp_peer_groups: ::validated_data::Field<BgpPeerGroups<'a, Mode>>,
    pub bgp_update_wait_install: ::validated_data::Field<bool>,
    pub bgp_update_wait_for_convergence: ::validated_data::Field<bool>,
    pub campus: ::validated_data::Field<&'a str>,
    pub campus_access_pod: ::validated_data::Field<&'a str>,
    pub campus_pod: ::validated_data::Field<&'a str>,
    pub connected_endpoints: ::validated_data::Field<ConnectedEndpoints<'a, Mode>>,
    pub custom_connected_endpoints_keys: ::validated_data::Field<CustomConnectedEndpointsKeys<'a, Mode>>,
    pub connected_endpoints_keys: ::validated_data::Field<ConnectedEndpointsKeys<'a, Mode>>,
    pub core_interfaces: ::validated_data::Field<CoreInterfaces<'a, Mode>>,
    pub custom_structured_configuration_list_merge: ::validated_data::Field<&'a str>,
    pub custom_structured_configuration_prefix: ::validated_data::Field<CustomStructuredConfigurationPrefix<'a, Mode>>,
    pub custom_system_mac_address: ::validated_data::Field<&'a str>,
    pub cv_pathfinder_global_sites: ::validated_data::Field<CvPathfinderGlobalSites<'a, Mode>>,
    pub cv_pathfinder_internet_exit_policies: ::validated_data::Field<CvPathfinderInternetExitPolicies<'a, Mode>>,
    pub cv_pathfinder_regions: ::validated_data::Field<CvPathfinderRegions<'a, Mode>>,
    pub cv_server: ::validated_data::Field<&'a str>,
    pub cv_settings: ::validated_data::Field<CvSettings<'a, Mode>>,
    pub cv_tags_topology_type: ::validated_data::Field<&'a str>,
    pub cv_token: ::validated_data::Field<&'a str>,
    pub cv_topology: ::validated_data::Field<CvTopology<'a, Mode>>,
    pub cv_topology_levels: ::validated_data::Field<CvTopologyLevels<'a, Mode>>,
    pub dc_name: ::validated_data::Field<&'a str>,
    pub default_connected_endpoints_description: ::validated_data::Field<&'a str>,
    pub default_connected_endpoints_port_channel_description: ::validated_data::Field<&'a str>,
    pub default_igmp_snooping_enabled: ::validated_data::Field<bool>,
    pub default_interface_mtu: ::validated_data::Field<i64>,
    pub default_interfaces: ::validated_data::Field<DefaultInterfaces<'a, Mode>>,
    pub default_mgmt_method: ::validated_data::Field<&'a str>,
    pub default_network_ports_description: ::validated_data::Field<&'a str>,
    pub default_network_ports_port_channel_description: ::validated_data::Field<&'a str>,
    pub default_node_types: ::validated_data::Field<DefaultNodeTypes<'a, Mode>>,
    pub default_underlay_p2p_ethernet_description: ::validated_data::Field<&'a str>,
    pub default_underlay_p2p_port_channel_description: ::validated_data::Field<&'a str>,
    pub default_vrf_diag_loopback_description: ::validated_data::Field<&'a str>,
    pub device_profile: ::validated_data::Field<&'a str>,
    pub device_profiles: ::validated_data::Field<DeviceProfiles<'a, Mode>>,
    pub devices: ::validated_data::Field<Devices<'a, Mode>>,
    pub digital_twin: ::validated_data::Field<DigitalTwin<'a, Mode>>,
    pub dns_settings: ::validated_data::Field<DnsSettings<'a, Mode>>,
    pub dot1x_settings: ::validated_data::Field<Dot1xSettings<'a, Mode>>,
    pub enable_trunk_groups: ::validated_data::Field<bool>,
    pub eos_designs_custom_templates: ::validated_data::Field<EosDesignsCustomTemplates<'a, Mode>>,
    pub eos_designs_documentation: ::validated_data::Field<EosDesignsDocumentation<'a, Mode>>,
    pub eos_designs_keep_tmp_files: ::validated_data::Field<bool>,
    pub eos_designs_return_structured_config: ::validated_data::Field<bool>,
    pub eos_designs_tmp_dir: ::validated_data::Field<&'a str>,
    pub eos_designs_validate_inputs_batch_size: ::validated_data::Field<i64>,
    pub eos_designs_validate_inputs_template_with_multiprocessing: ::validated_data::Field<bool>,
    pub errdisable_settings: ::validated_data::Field<ErrdisableSettings<'a, Mode>>,
    pub event_handlers: ::validated_data::Field<super::eos_cli_config_gen::EventHandlers<'a, Mode>>,
    pub event_monitor: ::validated_data::Field<super::eos_cli_config_gen::EventMonitor<'a, Mode>>,
    pub evpn_ebgp_gateway_multihop: ::validated_data::Field<i64>,
    pub evpn_ebgp_multihop: ::validated_data::Field<i64>,
    pub evpn_hostflap_detection: ::validated_data::Field<EvpnHostflapDetection<'a, Mode>>,
    pub evpn_import_pruning: ::validated_data::Field<bool>,
    pub evpn_multicast: ::validated_data::Field<bool>,
    pub evpn_overlay_bgp_rtc: ::validated_data::Field<bool>,
    pub evpn_prevent_readvertise_to_server: ::validated_data::Field<bool>,
    pub evpn_prevent_readvertise_to_server_mode: ::validated_data::Field<&'a str>,
    pub evpn_short_esi_prefix: ::validated_data::Field<&'a str>,
    pub evpn_vlan_aware_bundles: ::validated_data::Field<bool>,
    pub evpn_vlan_bundles: ::validated_data::Field<EvpnVlanBundles<'a, Mode>>,
    pub fabric_evpn_encapsulation: ::validated_data::Field<&'a str>,
    pub fabric_flow_tracking: ::validated_data::Field<FabricFlowTracking<'a, Mode>>,
    pub fabric_ip_addressing: ::validated_data::Field<FabricIpAddressing<'a, Mode>>,
    pub fabric_name: ::validated_data::RequiredValue<&'a str, Mode>,
    pub fabric_numbering: ::validated_data::Field<FabricNumbering<'a, Mode>>,
    pub fabric_numbering_node_id_pool: ::validated_data::Field<&'a str>,
    pub fabric_sflow: ::validated_data::Field<FabricSflow<'a, Mode>>,
    pub flow_tracking_settings: ::validated_data::Field<FlowTrackingSettings<'a, Mode>>,
    pub general_settings: ::validated_data::Field<GeneralSettings<'a, Mode>>,
    pub generate_cv_tags: ::validated_data::Field<GenerateCvTags<'a, Mode>>,
    pub hardware_counters: ::validated_data::Field<super::eos_cli_config_gen::HardwareCounters<'a, Mode>>,
    pub inband_ztp_bootstrap_file: ::validated_data::Field<&'a str>,
    pub internal_vlan_order: ::validated_data::Field<InternalVlanOrder<'a, Mode>>,
    pub ipsec_settings: ::validated_data::Field<IpsecSettings<'a, Mode>>,
    pub ipv4_acls: ::validated_data::Field<Ipv4Acls<'a, Mode>>,
    pub ipv4_prefix_list_catalog: ::validated_data::Field<Ipv4PrefixListCatalog<'a, Mode>>,
    pub ipv4_standard_acls: ::validated_data::Field<Ipv4StandardAcls<'a, Mode>>,
    pub ipv6_acls: ::validated_data::Field<Ipv6Acls<'a, Mode>>,
    pub ipv6_mgmt_destination_networks: ::validated_data::Field<Ipv6MgmtDestinationNetworks<'a, Mode>>,
    pub ipv6_mgmt_gateway: ::validated_data::Field<&'a str>,
    pub ipv6_prefix_list_catalog: ::validated_data::Field<Ipv6PrefixListCatalog<'a, Mode>>,
    pub is_deployed: ::validated_data::Field<bool>,
    pub isis_advertise_passive_only: ::validated_data::Field<bool>,
    pub isis_area_id: ::validated_data::Field<&'a str>,
    pub isis_default_circuit_type: ::validated_data::Field<&'a str>,
    pub isis_default_is_type: ::validated_data::Field<&'a str>,
    pub isis_default_metric: ::validated_data::Field<i64>,
    pub isis_maximum_paths: ::validated_data::Field<i64>,
    pub isis_system_id_format: ::validated_data::Field<&'a str>,
    pub isis_ti_lfa: ::validated_data::Field<IsisTiLfa<'a, Mode>>,
    pub l2vlan_profiles: ::validated_data::Field<L2vlanProfiles<'a, Mode>>,
    pub l3_edge: ::validated_data::Field<L3Edge<'a, Mode>>,
    pub l3_interface_profiles: ::validated_data::Field<L3InterfaceProfiles<'a, Mode>>,
    pub load_interval: ::validated_data::Field<super::eos_cli_config_gen::LoadInterval<'a, Mode>>,
    pub logging_settings: ::validated_data::Field<LoggingSettings<'a, Mode>>,
    pub mac_acls: ::validated_data::Field<MacAcls<'a, Mode>>,
    pub mac_address_table: ::validated_data::Field<super::eos_cli_config_gen::MacAddressTable<'a, Mode>>,
    pub management_eapi: ::validated_data::Field<ManagementEapi<'a, Mode>>,
    pub management_settings: ::validated_data::Field<ManagementSettings<'a, Mode>>,
    pub mgmt_destination_networks: ::validated_data::Field<MgmtDestinationNetworks<'a, Mode>>,
    pub mgmt_gateway: ::validated_data::Field<&'a str>,
    pub mgmt_interface: ::validated_data::Field<&'a str>,
    pub mgmt_interface_description: ::validated_data::Field<&'a str>,
    pub mgmt_interface_settings: ::validated_data::Field<MgmtInterfaceSettings<'a, Mode>>,
    pub mgmt_interface_vrf: ::validated_data::Field<&'a str>,
    pub mgmt_vrf_routing: ::validated_data::Field<bool>,
    pub mlag_bgp_peer_description: ::validated_data::Field<&'a str>,
    pub mlag_bgp_peer_group_description: ::validated_data::Field<&'a str>,
    pub mlag_ibgp_peering_vrfs: ::validated_data::Field<MlagIbgpPeeringVrfs<'a, Mode>>,
    pub mlag_member_description: ::validated_data::Field<&'a str>,
    pub mlag_on_orphan_port_channel_downlink: ::validated_data::Field<bool>,
    pub mlag_peer_l3_svi_description: ::validated_data::Field<&'a str>,
    pub mlag_peer_l3_vlan_name: ::validated_data::Field<&'a str>,
    pub mlag_peer_l3_vrf_svi_description: ::validated_data::Field<&'a str>,
    pub mlag_peer_l3_vrf_vlan_name: ::validated_data::Field<&'a str>,
    pub mlag_peer_svi_description: ::validated_data::Field<&'a str>,
    pub mlag_peer_vlan_name: ::validated_data::Field<&'a str>,
    pub mlag_port_channel_description: ::validated_data::Field<&'a str>,
    pub monitor_connectivity: ::validated_data::Field<MonitorConnectivity<'a, Mode>>,
    pub network_ports: ::validated_data::Field<NetworkPorts<'a, Mode>>,
    pub network_services: ::validated_data::Field<NetworkServices<'a, Mode>>,
    pub network_services_keys: ::validated_data::Field<NetworkServicesKeys<'a, Mode>>,
    pub custom_node_type_keys: ::validated_data::Field<CustomNodeTypeKeys<'a, Mode>>,
    pub node_type_keys: ::validated_data::Field<NodeTypeKeys<'a, Mode>>,
    pub ntp_settings: ::validated_data::Field<NtpSettings<'a, Mode>>,
    pub only_local_vlan_trunk_groups: ::validated_data::Field<bool>,
    pub overlay_bgp_peer_description: ::validated_data::Field<&'a str>,
    pub overlay_cvx_servers: ::validated_data::Field<OverlayCvxServers<'a, Mode>>,
    pub overlay_her_flood_list_per_vni: ::validated_data::Field<bool>,
    pub overlay_her_flood_list_scope: ::validated_data::Field<&'a str>,
    pub overlay_mlag_rfc5549: ::validated_data::Field<bool>,
    pub overlay_rd_type: ::validated_data::Field<OverlayRdType<'a, Mode>>,
    pub overlay_routing_protocol: ::validated_data::Field<&'a str>,
    pub overlay_routing_protocol_address_family: ::validated_data::Field<&'a str>,
    pub overlay_rt_type: ::validated_data::Field<OverlayRtType<'a, Mode>>,
    pub p2p_uplinks_mtu: ::validated_data::Field<i64>,
    pub p2p_uplinks_qos_profile: ::validated_data::Field<&'a str>,
    pub custom_platform_settings: ::validated_data::Field<CustomPlatformSettings<'a, Mode>>,
    pub platform_settings: ::validated_data::Field<PlatformSettings<'a, Mode>>,
    pub platform_speed_groups: ::validated_data::Field<PlatformSpeedGroups<'a, Mode>>,
    pub pod_name: ::validated_data::Field<&'a str>,
    pub port_profiles: ::validated_data::Field<PortProfiles<'a, Mode>>,
    pub ptp_profiles: ::validated_data::Field<PtpProfiles<'a, Mode>>,
    pub ptp_settings: ::validated_data::Field<PtpSettings<'a, Mode>>,
    pub queue_monitor_length: ::validated_data::Field<QueueMonitorLength<'a, Mode>>,
    pub queue_monitor_streaming: ::validated_data::Field<super::eos_cli_config_gen::QueueMonitorStreaming<'a, Mode>>,
    pub redundancy: ::validated_data::Field<Redundancy<'a, Mode>>,
    pub router_id_loopback_description: ::validated_data::Field<&'a str>,
    pub serial_number: ::validated_data::Field<&'a str>,
    pub sflow_settings: ::validated_data::Field<SflowSettings<'a, Mode>>,
    pub shutdown_bgp_towards_undeployed_peers: ::validated_data::Field<bool>,
    pub shutdown_interfaces_towards_undeployed_peers: ::validated_data::Field<bool>,
    pub snmp_settings: ::validated_data::Field<SnmpSettings<'a, Mode>>,
    pub source_interfaces: ::validated_data::Field<SourceInterfaces<'a, Mode>>,
    pub spanning_tree_settings: ::validated_data::Field<SpanningTreeSettings<'a, Mode>>,
    pub ssh_settings: ::validated_data::Field<SshSettings<'a, Mode>>,
    pub svi_profiles: ::validated_data::Field<SviProfiles<'a, Mode>>,
    pub system_mac_address: ::validated_data::Field<&'a str>,
    pub tcam_profiles: ::validated_data::Field<super::eos_cli_config_gen::tcam_profile::Profiles<'a, Mode>>,
    pub timezone: ::validated_data::Field<&'a str>,
    pub trunk_groups: ::validated_data::Field<TrunkGroups<'a, Mode>>,
    #[data_view(rename = "type")]
    pub field_type: ::validated_data::Field<&'a str>,
    pub underlay_filter_peer_as: ::validated_data::Field<bool>,
    pub underlay_filter_redistribute_connected: ::validated_data::Field<bool>,
    pub underlay_ipv6: ::validated_data::Field<bool>,
    pub underlay_ipv6_numbered: ::validated_data::Field<bool>,
    pub underlay_isis_authentication_cleartext_key: ::validated_data::Field<&'a str>,
    pub underlay_isis_authentication_key: ::validated_data::Field<&'a str>,
    pub underlay_isis_authentication_mode: ::validated_data::Field<&'a str>,
    pub underlay_isis_bfd: ::validated_data::Field<bool>,
    pub underlay_isis_instance_name: ::validated_data::Field<&'a str>,
    pub underlay_l2_ethernet_description: ::validated_data::Field<&'a str>,
    pub underlay_l2_port_channel_description: ::validated_data::Field<&'a str>,
    pub underlay_multicast_anycast_rp: ::validated_data::Field<UnderlayMulticastAnycastRp<'a, Mode>>,
    pub underlay_multicast_pim_sm: ::validated_data::Field<bool>,
    pub underlay_multicast_rps: ::validated_data::Field<UnderlayMulticastRps<'a, Mode>>,
    pub underlay_multicast_static: ::validated_data::Field<bool>,
    pub underlay_ospf_area: ::validated_data::Field<&'a str>,
    pub underlay_ospf_authentication: ::validated_data::Field<UnderlayOspfAuthentication<'a, Mode>>,
    pub underlay_ospf_bfd_enable: ::validated_data::Field<bool>,
    pub underlay_ospf_graceful_restart: ::validated_data::Field<bool>,
    pub underlay_ospf_max_lsa: ::validated_data::Field<i64>,
    pub underlay_ospf_maximum_paths: ::validated_data::Field<i64>,
    pub underlay_ospf_process_id: ::validated_data::Field<i64>,
    pub underlay_rfc5549: ::validated_data::Field<bool>,
    pub underlay_routing_protocol: ::validated_data::Field<&'a str>,
    pub unsupported_transceiver: ::validated_data::Field<super::eos_cli_config_gen::ServiceUnsupportedTransceiver<'a, Mode>>,
    pub uplink_ptp: ::validated_data::Field<UplinkPtp<'a, Mode>>,
    pub use_cv_topology: ::validated_data::Field<bool>,
    pub use_router_general_for_router_id: ::validated_data::Field<bool>,
    pub validation_profiles: ::validated_data::Field<ValidationProfiles<'a, Mode>>,
    pub vtep_loopback_description: ::validated_data::Field<&'a str>,
    pub vtep_vvtep_ip: ::validated_data::Field<&'a str>,
    pub wan_carriers: ::validated_data::Field<WanCarriers<'a, Mode>>,
    pub wan_encapsulation: ::validated_data::Field<&'a str>,
    pub wan_ha: ::validated_data::Field<WanHa<'a, Mode>>,
    pub wan_ipsec_profiles: ::validated_data::Field<WanIpsecProfiles<'a, Mode>>,
    pub wan_mode: ::validated_data::Field<&'a str>,
    pub wan_path_groups: ::validated_data::Field<WanPathGroups<'a, Mode>>,
    pub wan_route_servers: ::validated_data::Field<WanRouteServers<'a, Mode>>,
    pub wan_stun_dtls_disable: ::validated_data::Field<bool>,
    pub wan_stun_dtls_profile_name: ::validated_data::Field<&'a str>,
    pub wan_virtual_topologies: ::validated_data::Field<WanVirtualTopologies<'a, Mode>>,
    pub zscaler_endpoints: ::validated_data::Field<ZscalerEndpoints<'a, Mode>>,
    #[data_view(dynamic = "custom_node_type_keys.key")]
    pub custom_node_type_keys_key: ::validated_data::Field<DynamicSlot12224<'a, Mode>>,
    #[data_view(dynamic = "connected_endpoints_keys.key")]
    pub connected_endpoints_keys_key: ::validated_data::Field<DynamicSlot13723<'a, Mode>>,
    #[data_view(dynamic = "custom_connected_endpoints_keys.key")]
    pub custom_connected_endpoints_keys_key: ::validated_data::Field<DynamicSlot13911<'a, Mode>>,
    #[data_view(dynamic = "network_services_keys.name")]
    pub network_services_keys_name: ::validated_data::Field<DynamicSlot14099<'a, Mode>>,
    #[data_view(dynamic = "node_type_keys.key")]
    pub node_type_keys_key: ::validated_data::Field<DynamicSlot15021<'a, Mode>>,
}

#[::validated_data::data_view]
pub struct AaaSettings<'a, Mode> {
    pub enable_password: ::validated_data::Field<aaa_settings::EnablePassword<'a, Mode>>,
    pub tacacs: ::validated_data::Field<aaa_settings::Tacacs<'a, Mode>>,
    pub radius: ::validated_data::Field<aaa_settings::Radius<'a, Mode>>,
    pub authentication: ::validated_data::Field<aaa_settings::Authentication<'a, Mode>>,
    pub authorization: ::validated_data::Field<aaa_settings::Authorization<'a, Mode>>,
    pub accounting: ::validated_data::Field<aaa_settings::Accounting<'a, Mode>>,
    pub root_login: ::validated_data::Field<aaa_settings::RootLogin<'a, Mode>>,
    pub local_users: ::validated_data::Field<aaa_settings::LocalUsers<'a, Mode>>,
}

pub mod aaa_settings;

#[::validated_data::data_view]
pub struct AddressLockingSettings<'a, Mode> {
    pub local_interface: ::validated_data::Field<&'a str>,
    pub dhcp_server_interfaces: ::validated_data::Field<address_locking_settings::DhcpServerInterfaces<'a, Mode>>,
    pub dhcp_servers_ipv4: ::validated_data::Field<address_locking_settings::DhcpServersIpv4<'a, Mode>>,
    pub disabled: ::validated_data::Field<bool>,
    pub leases: ::validated_data::Field<address_locking_settings::Leases<'a, Mode>>,
    pub locked_address: ::validated_data::Field<address_locking_settings::LockedAddress<'a, Mode>>,
}

pub mod address_locking_settings;

#[::validated_data::data_view]
pub struct AvdDesignFuture<'a, Mode> {
    pub accept_dhcp_default_route_for_mgmt_ip_dhcp: ::validated_data::Field<bool>,
    pub accept_ra_default_route_for_ipv6_mgmt_ip_auto_config: ::validated_data::Field<bool>,
    pub accept_dhcp_default_route_for_inband_mgmt_ip_dhcp: ::validated_data::Field<bool>,
    pub allow_recursive_profile_inheritance: ::validated_data::Field<bool>,
    pub configure_inband_mgmt_ipv6_vrf: ::validated_data::Field<bool>,
    pub consistent_uplink_vlans: ::validated_data::Field<bool>,
    pub fix_address_locking_dhcp_server_interfaces: ::validated_data::Field<bool>,
    pub fix_match_ipv6_prefix_list_on_mlag_route_map: ::validated_data::Field<bool>,
    pub fix_radius_server_group_tls: ::validated_data::Field<bool>,
    pub only_configure_ipv6_inband_mgmt_prefix_list_when_used: ::validated_data::Field<bool>,
    pub only_configure_mlag_vrfs_peer_group_when_used: ::validated_data::Field<bool>,
    pub only_configure_pvst_border_when_mode_is_mstp: ::validated_data::Field<bool>,
    pub only_configure_route_map_connected_to_bgp_vrfs_when_used: ::validated_data::Field<bool>,
    pub raise_for_port_channels_without_members: ::validated_data::Field<bool>,
    pub raise_for_underlay_router_with_uplink_type_port_channel: ::validated_data::Field<bool>,
    pub remove_redundant_ipv4_unicast_for_peer_groups: ::validated_data::Field<bool>,
}

#[::validated_data::data_view]
pub struct EosDesignsValidationConfiguration<'a, Mode> {
    pub warn_eos_config_keys: ::validated_data::Field<bool>,
}

#[::validated_data::data_view]
pub struct BfdMultihop<'a, Mode> {
    pub interval: ::validated_data::RequiredValue<i64, Mode>,
    pub min_rx: ::validated_data::RequiredValue<i64, Mode>,
    pub multiplier: ::validated_data::RequiredValue<i64, Mode>,
}

#[::validated_data::data_view]
pub struct BgpGracefulRestart<'a, Mode> {
    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    pub restart_time: ::validated_data::Field<i64>,
}

#[::validated_data::data_view]
pub struct BgpPeerGroups<'a, Mode> {
    pub ipv4_underlay_peers: ::validated_data::Field<bgp_peer_groups::Ipv4UnderlayPeers<'a, Mode>>,
    pub mlag_ipv4_vrfs_peer: ::validated_data::Field<bgp_peer_groups::MlagIpv4VrfsPeer<'a, Mode>>,
    pub mlag_ipv4_underlay_peer: ::validated_data::Field<bgp_peer_groups::MlagIpv4UnderlayPeer<'a, Mode>>,
    pub evpn_overlay_peers: ::validated_data::Field<bgp_peer_groups::EvpnOverlayPeers<'a, Mode>>,
    pub evpn_overlay_core: ::validated_data::Field<bgp_peer_groups::EvpnOverlayCore<'a, Mode>>,
    pub mpls_overlay_peers: ::validated_data::Field<bgp_peer_groups::MplsOverlayPeers<'a, Mode>>,
    pub rr_overlay_peers: ::validated_data::Field<bgp_peer_groups::RrOverlayPeers<'a, Mode>>,
    pub ipvpn_gateway_peers: ::validated_data::Field<bgp_peer_groups::IpvpnGatewayPeers<'a, Mode>>,
    pub wan_overlay_peers: ::validated_data::Field<bgp_peer_groups::WanOverlayPeers<'a, Mode>>,
    pub wan_rr_overlay_peers: ::validated_data::Field<bgp_peer_groups::WanRrOverlayPeers<'a, Mode>>,
}

pub mod bgp_peer_groups;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct ConnectedEndpoints<'a, Mode> (::validated_data::Field<connected_endpoints::Item<'a, Mode>>);

pub mod connected_endpoints;

#[::validated_data::data_view(indexed_list, primary_key(key))]
pub struct CustomConnectedEndpointsKeys<'a, Mode> (::validated_data::Field<custom_connected_endpoints_keys::Item<'a, Mode>>);

pub mod custom_connected_endpoints_keys;

#[::validated_data::data_view(indexed_list, primary_key(key))]
pub struct ConnectedEndpointsKeys<'a, Mode> (::validated_data::Field<connected_endpoints_keys::Item<'a, Mode>>);

pub mod connected_endpoints_keys;

#[::validated_data::data_view]
pub struct CoreInterfaces<'a, Mode> {
    pub p2p_links_ip_pools: ::validated_data::Field<core_interfaces::P2pLinksIpPools<'a, Mode>>,
    pub p2p_links_profiles: ::validated_data::Field<core_interfaces::P2pLinksProfiles<'a, Mode>>,
    pub p2p_links: ::validated_data::Field<core_interfaces::P2pLinks<'a, Mode>>,
}

pub mod core_interfaces;

#[::validated_data::data_view(list)]
pub struct CustomStructuredConfigurationPrefix<'a, Mode> (::validated_data::Field<&'a str>);

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct CvPathfinderGlobalSites<'a, Mode> (::validated_data::Field<cv_pathfinder_global_sites::Item<'a, Mode>>);

pub mod cv_pathfinder_global_sites;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct CvPathfinderInternetExitPolicies<'a, Mode> (::validated_data::Field<cv_pathfinder_internet_exit_policies::Item<'a, Mode>>);

pub mod cv_pathfinder_internet_exit_policies;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct CvPathfinderRegions<'a, Mode> (::validated_data::Field<cv_pathfinder_regions::Item<'a, Mode>>);

pub mod cv_pathfinder_regions;

#[::validated_data::data_view]
pub struct CvSettings<'a, Mode> {
    pub cvaas: ::validated_data::Field<cv_settings::Cvaas<'a, Mode>>,
    pub onprem_clusters: ::validated_data::Field<cv_settings::OnpremClusters<'a, Mode>>,
    pub terminattr: ::validated_data::Field<cv_settings::Terminattr<'a, Mode>>,
    pub set_source_interfaces: ::validated_data::Field<bool>,
}

pub mod cv_settings;

#[::validated_data::data_view(indexed_list, primary_key(hostname))]
pub struct CvTopology<'a, Mode> (::validated_data::Field<cv_topology::Item<'a, Mode>>);

pub mod cv_topology;

#[::validated_data::data_view(indexed_list, primary_key(field_type))]
pub struct CvTopologyLevels<'a, Mode> (::validated_data::Field<cv_topology_levels::Item<'a, Mode>>);

pub mod cv_topology_levels;

#[::validated_data::data_view(list)]
pub struct DefaultInterfaces<'a, Mode> (::validated_data::Field<default_interfaces::Item<'a, Mode>>);

pub mod default_interfaces;

#[::validated_data::data_view(indexed_list, primary_key(node_type))]
pub struct DefaultNodeTypes<'a, Mode> (::validated_data::Field<default_node_types::Item<'a, Mode>>);

pub mod default_node_types;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct DeviceProfiles<'a, Mode> (::validated_data::Field<device_profiles::Item<'a, Mode>>);

pub mod device_profiles;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Devices<'a, Mode> (::validated_data::Field<devices::Item<'a, Mode>>);

pub mod devices;

#[::validated_data::data_view]
pub struct DigitalTwin<'a, Mode> {
    pub environment: ::validated_data::Field<&'a str>,
    pub fabric: ::validated_data::RequiredValue<digital_twin::Fabric<'a, Mode>, Mode>,
    pub use_default_interfaces_of_digital_twin_platform: ::validated_data::Field<bool>,
}

pub mod digital_twin;

#[::validated_data::data_view]
pub struct DnsSettings<'a, Mode> {
    pub domain: ::validated_data::Field<&'a str>,
    pub domain_list: ::validated_data::Field<dns_settings::DomainList<'a, Mode>>,
    pub servers: ::validated_data::RequiredValue<dns_settings::Servers<'a, Mode>, Mode>,
    pub vrfs: ::validated_data::Field<dns_settings::Vrfs<'a, Mode>>,
    pub set_source_interfaces: ::validated_data::Field<bool>,
    pub ip_hosts: ::validated_data::Field<super::eos_cli_config_gen::IpHosts<'a, Mode>>,
}

pub mod dns_settings;

#[::validated_data::data_view]
pub struct Dot1xSettings<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub authentication: ::validated_data::Field<dot1x_settings::Authentication<'a, Mode>>,
    pub accounting: ::validated_data::Field<dot1x_settings::Accounting<'a, Mode>>,
    pub bypass_bpdu: ::validated_data::Field<bool>,
    pub bypass_lldp: ::validated_data::Field<bool>,
    pub dynamic_authorization: ::validated_data::Field<dot1x_settings::DynamicAuthorization<'a, Mode>>,
    pub mac_based_authentication: ::validated_data::Field<dot1x_settings::MacBasedAuthentication<'a, Mode>>,
    pub radius_av_pairs: ::validated_data::Field<dot1x_settings::RadiusAvPairs<'a, Mode>>,
    pub device_profiling: ::validated_data::Field<dot1x_settings::DeviceProfiling<'a, Mode>>,
    pub redistribute_in_evpn: ::validated_data::Field<bool>,
    pub web_authentication: ::validated_data::Field<dot1x_settings::WebAuthentication<'a, Mode>>,
}

pub mod dot1x_settings;

#[::validated_data::data_view(list)]
pub struct EosDesignsCustomTemplates<'a, Mode> (::validated_data::Field<eos_designs_custom_templates::Item<'a, Mode>>);

pub mod eos_designs_custom_templates;

#[::validated_data::data_view]
pub struct EosDesignsDocumentation<'a, Mode> {
    pub enable: ::validated_data::Field<bool>,
    pub connected_endpoints: ::validated_data::Field<bool>,
    pub topology_csv: ::validated_data::Field<bool>,
    pub p2p_links_csv: ::validated_data::Field<bool>,
    pub toc: ::validated_data::Field<bool>,
}

#[::validated_data::data_view]
pub struct ErrdisableSettings<'a, Mode> {
    pub recovery_interval: ::validated_data::Field<i64>,
    pub causes: ::validated_data::Field<errdisable_settings::Causes<'a, Mode>>,
}

pub mod errdisable_settings;

#[::validated_data::data_view]
pub struct EvpnHostflapDetection<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub threshold: ::validated_data::Field<i64>,
    pub window: ::validated_data::Field<i64>,
    pub expiry_timeout: ::validated_data::Field<i64>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct EvpnVlanBundles<'a, Mode> (::validated_data::Field<evpn_vlan_bundles::Item<'a, Mode>>);

pub mod evpn_vlan_bundles;

#[::validated_data::data_view]
pub struct FabricFlowTracking<'a, Mode> {
    pub uplinks: ::validated_data::Field<fabric_flow_tracking::Uplinks<'a, Mode>>,
    pub downlinks: ::validated_data::Field<fabric_flow_tracking::Downlinks<'a, Mode>>,
    pub endpoints: ::validated_data::Field<fabric_flow_tracking::Endpoints<'a, Mode>>,
    pub l3_edge: ::validated_data::Field<fabric_flow_tracking::L3Edge<'a, Mode>>,
    pub core_interfaces: ::validated_data::Field<fabric_flow_tracking::CoreInterfaces<'a, Mode>>,
    pub mlag_interfaces: ::validated_data::Field<fabric_flow_tracking::MlagInterfaces<'a, Mode>>,
    pub l3_interfaces: ::validated_data::Field<fabric_flow_tracking::L3Interfaces<'a, Mode>>,
    pub l3_port_channels: ::validated_data::Field<fabric_flow_tracking::L3PortChannels<'a, Mode>>,
    pub dps_interfaces: ::validated_data::Field<fabric_flow_tracking::DpsInterfaces<'a, Mode>>,
    pub direct_wan_ha_links: ::validated_data::Field<fabric_flow_tracking::DirectWanHaLinks<'a, Mode>>,
}

pub mod fabric_flow_tracking;

#[::validated_data::data_view]
pub struct FabricIpAddressing<'a, Mode> {
    pub loopback: ::validated_data::Field<fabric_ip_addressing::Loopback<'a, Mode>>,
    pub mlag: ::validated_data::Field<fabric_ip_addressing::Mlag<'a, Mode>>,
    pub p2p_uplinks: ::validated_data::Field<fabric_ip_addressing::P2pUplinks<'a, Mode>>,
    pub wan_ha: ::validated_data::Field<fabric_ip_addressing::WanHa<'a, Mode>>,
}

pub mod fabric_ip_addressing;

#[::validated_data::data_view]
pub struct FabricNumbering<'a, Mode> {
    pub node_id: ::validated_data::Field<fabric_numbering::NodeId<'a, Mode>>,
}

pub mod fabric_numbering;

#[::validated_data::data_view]
pub struct FabricSflow<'a, Mode> {
    pub uplinks: ::validated_data::Field<bool>,
    pub downlinks: ::validated_data::Field<bool>,
    pub endpoints: ::validated_data::Field<bool>,
    pub l3_edge: ::validated_data::Field<bool>,
    pub core_interfaces: ::validated_data::Field<bool>,
    pub mlag_interfaces: ::validated_data::Field<bool>,
    pub l3_interfaces: ::validated_data::Field<bool>,
}

#[::validated_data::data_view]
pub struct FlowTrackingSettings<'a, Mode> {
    pub sampled: ::validated_data::Field<flow_tracking_settings::Sampled<'a, Mode>>,
    pub hardware: ::validated_data::Field<flow_tracking_settings::Hardware<'a, Mode>>,
    pub cloudvision_exporter: ::validated_data::Field<flow_tracking_settings::CloudvisionExporter<'a, Mode>>,
    pub trackers: ::validated_data::Field<flow_tracking_settings::Trackers<'a, Mode>>,
}

pub mod flow_tracking_settings;

#[::validated_data::data_view]
pub struct GeneralSettings<'a, Mode> {
    pub interface_defaults: ::validated_data::Field<general_settings::InterfaceDefaults<'a, Mode>>,
    pub arp: ::validated_data::Field<general_settings::Arp<'a, Mode>>,
    pub ip_icmp_redirect: ::validated_data::Field<bool>,
    pub dhcp_relay: ::validated_data::Field<general_settings::DhcpRelay<'a, Mode>>,
    pub suspended_vlans: ::validated_data::Field<general_settings::SuspendedVlans<'a, Mode>>,
}

pub mod general_settings;

#[::validated_data::data_view]
pub struct GenerateCvTags<'a, Mode> {
    pub topology_hints: ::validated_data::Field<bool>,
    pub campus_fabric: ::validated_data::Field<bool>,
    pub interface_tags: ::validated_data::Field<generate_cv_tags::InterfaceTags<'a, Mode>>,
    pub device_tags: ::validated_data::Field<generate_cv_tags::DeviceTags<'a, Mode>>,
}

pub mod generate_cv_tags;

#[::validated_data::data_view]
pub struct InternalVlanOrder<'a, Mode> {
    pub allocation: ::validated_data::RequiredValue<&'a str, Mode>,
    pub range: ::validated_data::Field<internal_vlan_order::Range<'a, Mode>>,
}

pub mod internal_vlan_order;

#[::validated_data::data_view]
pub struct IpsecSettings<'a, Mode> {
    pub bind_connection_to_interface: ::validated_data::Field<bool>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Ipv4Acls<'a, Mode> (::validated_data::Field<ipv4_acls::Item<'a, Mode>>);

pub mod ipv4_acls;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Ipv4PrefixListCatalog<'a, Mode> (::validated_data::Field<ipv4_prefix_list_catalog::Item<'a, Mode>>);

pub mod ipv4_prefix_list_catalog;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Ipv4StandardAcls<'a, Mode> (::validated_data::Field<ipv4_standard_acls::Item<'a, Mode>>);

pub mod ipv4_standard_acls;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Ipv6Acls<'a, Mode> (::validated_data::Field<ipv6_acls::Item<'a, Mode>>);

pub mod ipv6_acls;

#[::validated_data::data_view(list)]
pub struct Ipv6MgmtDestinationNetworks<'a, Mode> (::validated_data::Field<&'a str>);

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Ipv6PrefixListCatalog<'a, Mode> (::validated_data::Field<ipv6_prefix_list_catalog::Item<'a, Mode>>);

pub mod ipv6_prefix_list_catalog;

#[::validated_data::data_view]
pub struct IsisTiLfa<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub protection: ::validated_data::Field<&'a str>,
    pub local_convergence_delay: ::validated_data::Field<i64>,
}

#[::validated_data::data_view(indexed_list, primary_key(profile))]
pub struct L2vlanProfiles<'a, Mode> (::validated_data::Field<l2vlan_profiles::Item<'a, Mode>>);

pub mod l2vlan_profiles;

#[::validated_data::data_view]
pub struct L3Edge<'a, Mode> {
    pub p2p_links_ip_pools: ::validated_data::Field<l3_edge::P2pLinksIpPools<'a, Mode>>,
    pub p2p_links_profiles: ::validated_data::Field<l3_edge::P2pLinksProfiles<'a, Mode>>,
    pub p2p_links: ::validated_data::Field<l3_edge::P2pLinks<'a, Mode>>,
}

pub mod l3_edge;

#[::validated_data::data_view(indexed_list, primary_key(profile))]
pub struct L3InterfaceProfiles<'a, Mode> (::validated_data::Field<l3_interface_profiles::Item<'a, Mode>>);

pub mod l3_interface_profiles;

#[::validated_data::data_view]
pub struct LoggingSettings<'a, Mode> {
    pub use_local_interface_cli: ::validated_data::Field<bool>,
    pub hosts: ::validated_data::Field<logging_settings::Hosts<'a, Mode>>,
    pub vrfs: ::validated_data::Field<logging_settings::Vrfs<'a, Mode>>,
    pub console: ::validated_data::Field<&'a str>,
    pub monitor: ::validated_data::Field<&'a str>,
    pub buffered: ::validated_data::Field<super::eos_cli_config_gen::logging::Buffered<'a, Mode>>,
    pub repeat_messages: ::validated_data::Field<bool>,
    pub trap: ::validated_data::Field<&'a str>,
    pub synchronous: ::validated_data::Field<super::eos_cli_config_gen::logging::Synchronous<'a, Mode>>,
    pub format: ::validated_data::Field<super::eos_cli_config_gen::logging::Format<'a, Mode>>,
    pub facility: ::validated_data::Field<&'a str>,
    pub policy: ::validated_data::Field<super::eos_cli_config_gen::logging::Policy<'a, Mode>>,
    pub event: ::validated_data::Field<super::eos_cli_config_gen::logging::Event<'a, Mode>>,
    pub level: ::validated_data::Field<super::eos_cli_config_gen::logging::Level<'a, Mode>>,
    pub monitor_layer1: ::validated_data::Field<super::eos_cli_config_gen::MonitorLayer1<'a, Mode>>,
}

pub mod logging_settings;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct MacAcls<'a, Mode> (::validated_data::Field<mac_acls::Item<'a, Mode>>);

pub mod mac_acls;

#[::validated_data::data_view]
pub struct ManagementEapi<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub enable_http: ::validated_data::Field<bool>,
    pub enable_https: ::validated_data::Field<bool>,
    pub default_services: ::validated_data::Field<bool>,
    pub vrfs: ::validated_data::Field<management_eapi::Vrfs<'a, Mode>>,
}

pub mod management_eapi;

#[::validated_data::data_view]
pub struct ManagementSettings<'a, Mode> {
    pub console: ::validated_data::Field<super::eos_cli_config_gen::ManagementConsole<'a, Mode>>,
    pub banners: ::validated_data::Field<super::eos_cli_config_gen::Banners<'a, Mode>>,
}

pub mod management_settings;

#[::validated_data::data_view(list)]
pub struct MgmtDestinationNetworks<'a, Mode> (::validated_data::Field<&'a str>);

#[::validated_data::data_view]
pub struct MgmtInterfaceSettings<'a, Mode> {
    pub description: ::validated_data::Field<&'a str>,
    pub vrf: ::validated_data::Field<&'a str>,
    pub vrf_routing: ::validated_data::Field<bool>,
    pub interface: ::validated_data::Field<&'a str>,
    pub lldp: ::validated_data::Field<mgmt_interface_settings::Lldp<'a, Mode>>,
}

pub mod mgmt_interface_settings;

#[::validated_data::data_view]
pub struct MlagIbgpPeeringVrfs<'a, Mode> {
    pub base_vlan: ::validated_data::Field<i64>,
}

#[::validated_data::data_view]
pub struct MonitorConnectivity<'a, Mode> {
    pub shutdown: ::validated_data::Field<bool>,
    pub interval: ::validated_data::Field<i64>,
    pub interface_sets: ::validated_data::Field<monitor_connectivity::InterfaceSets<'a, Mode>>,
    pub local_interfaces: ::validated_data::Field<&'a str>,
    pub address_only: ::validated_data::Field<bool>,
    pub hosts: ::validated_data::Field<monitor_connectivity::Hosts<'a, Mode>>,
    pub name_server_group: ::validated_data::Field<&'a str>,
    pub vrfs: ::validated_data::Field<monitor_connectivity::Vrfs<'a, Mode>>,
}

pub mod monitor_connectivity;

#[::validated_data::data_view(list)]
pub struct NetworkPorts<'a, Mode> (::validated_data::Field<network_ports::Item<'a, Mode>>);

pub mod network_ports;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct NetworkServices<'a, Mode> (::validated_data::Field<network_services::Item<'a, Mode>>);

pub mod network_services;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct NetworkServicesKeys<'a, Mode> (::validated_data::Field<network_services_keys::Item<'a, Mode>>);

pub mod network_services_keys;

#[::validated_data::data_view(indexed_list, primary_key(key))]
pub struct CustomNodeTypeKeys<'a, Mode> (::validated_data::Field<custom_node_type_keys::Item<'a, Mode>>);

pub mod custom_node_type_keys;

#[::validated_data::data_view(indexed_list, primary_key(key))]
pub struct NodeTypeKeys<'a, Mode> (::validated_data::Field<node_type_keys::Item<'a, Mode>>);

pub mod node_type_keys;

#[::validated_data::data_view]
pub struct NtpSettings<'a, Mode> {
    pub server_vrf: ::validated_data::Field<&'a str>,
    pub set_first_ntp_server_as_preferred: ::validated_data::Field<bool>,
    pub servers: ::validated_data::Field<ntp_settings::Servers<'a, Mode>>,
    pub authenticate: ::validated_data::Field<bool>,
    pub authenticate_servers_only: ::validated_data::Field<bool>,
    pub authentication_keys: ::validated_data::Field<ntp_settings::AuthenticationKeys<'a, Mode>>,
    pub trusted_keys: ::validated_data::Field<&'a str>,
}

pub mod ntp_settings;

#[::validated_data::data_view(list)]
pub struct OverlayCvxServers<'a, Mode> (::validated_data::Field<&'a str>);

#[::validated_data::data_view]
pub struct OverlayRdType<'a, Mode> {
    pub admin_subfield: ::validated_data::Field<&'a str>,
    pub admin_subfield_offset: ::validated_data::Field<i64>,
    pub vrf_admin_subfield: ::validated_data::Field<&'a str>,
    pub vrf_admin_subfield_offset: ::validated_data::Field<i64>,
    pub vlan_assigned_number_subfield: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct OverlayRtType<'a, Mode> {
    pub admin_subfield: ::validated_data::Field<&'a str>,
    pub vrf_admin_subfield: ::validated_data::Field<&'a str>,
    pub vlan_assigned_number_subfield: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(list)]
pub struct CustomPlatformSettings<'a, Mode> (::validated_data::Field<custom_platform_settings::Item<'a, Mode>>);

pub mod custom_platform_settings;

#[::validated_data::data_view(list)]
pub struct PlatformSettings<'a, Mode> (::validated_data::Field<platform_settings::Item<'a, Mode>>);

pub mod platform_settings;

#[::validated_data::data_view(indexed_list, primary_key(platform))]
pub struct PlatformSpeedGroups<'a, Mode> (::validated_data::Field<platform_speed_groups::Item<'a, Mode>>);

pub mod platform_speed_groups;

#[::validated_data::data_view(indexed_list, primary_key(profile))]
pub struct PortProfiles<'a, Mode> (::validated_data::Field<port_profiles::Item<'a, Mode>>);

pub mod port_profiles;

#[::validated_data::data_view(indexed_list, primary_key(profile))]
pub struct PtpProfiles<'a, Mode> (::validated_data::Field<ptp_profiles::Item<'a, Mode>>);

pub mod ptp_profiles;

#[::validated_data::data_view]
pub struct PtpSettings<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub profile: ::validated_data::Field<&'a str>,
    pub domain: ::validated_data::Field<i64>,
    pub auto_clock_identity: ::validated_data::Field<bool>,
    pub forward_v1: ::validated_data::Field<bool>,
    pub free_running: ::validated_data::Field<super::eos_cli_config_gen::ptp::FreeRunning<'a, Mode>>,
}

pub mod ptp_settings;

#[::validated_data::data_view]
pub struct QueueMonitorLength<'a, Mode> {
    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    pub notifying: ::validated_data::Field<bool>,
    pub default_thresholds: ::validated_data::Field<queue_monitor_length::DefaultThresholds<'a, Mode>>,
    pub log: ::validated_data::Field<i64>,
    pub cpu: ::validated_data::Field<queue_monitor_length::Cpu<'a, Mode>>,
    pub tx_latency: ::validated_data::Field<bool>,
    pub mirror: ::validated_data::Field<queue_monitor_length::Mirror<'a, Mode>>,
}

pub mod queue_monitor_length;

#[::validated_data::data_view]
pub struct Redundancy<'a, Mode> {
    pub protocol: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct SflowSettings<'a, Mode> {
    pub polling_interval: ::validated_data::Field<i64>,
    pub sample: ::validated_data::Field<sflow_settings::Sample<'a, Mode>>,
    pub destinations: ::validated_data::Field<sflow_settings::Destinations<'a, Mode>>,
    pub export_to_cloudvision: ::validated_data::Field<sflow_settings::ExportToCloudvision<'a, Mode>>,
    pub vrfs: ::validated_data::Field<sflow_settings::Vrfs<'a, Mode>>,
}

pub mod sflow_settings;

#[::validated_data::data_view]
pub struct SnmpSettings<'a, Mode> {
    pub contact: ::validated_data::Field<&'a str>,
    pub location: ::validated_data::Field<bool>,
    pub location_template: ::validated_data::Field<&'a str>,
    pub vrfs: ::validated_data::Field<snmp_settings::Vrfs<'a, Mode>>,
    pub compute_local_engineid: ::validated_data::Field<bool>,
    pub compute_local_engineid_source: ::validated_data::Field<&'a str>,
    pub local_engineid_ip: ::validated_data::Field<&'a str>,
    pub compute_v3_user_localized_key: ::validated_data::Field<bool>,
    pub users: ::validated_data::Field<snmp_settings::Users<'a, Mode>>,
    pub hosts: ::validated_data::Field<snmp_settings::Hosts<'a, Mode>>,
    pub communities: ::validated_data::Field<snmp_settings::Communities<'a, Mode>>,
    pub views: ::validated_data::Field<snmp_settings::Views<'a, Mode>>,
    pub groups: ::validated_data::Field<snmp_settings::Groups<'a, Mode>>,
    pub traps: ::validated_data::Field<super::eos_cli_config_gen::snmp_server::Traps<'a, Mode>>,
}

pub mod snmp_settings;

#[::validated_data::data_view]
pub struct SourceInterfaces<'a, Mode> {
    pub http_client: ::validated_data::Field<source_interfaces::HttpClient<'a, Mode>>,
    pub ssh_client: ::validated_data::Field<source_interfaces::SshClient<'a, Mode>>,
}

pub mod source_interfaces;

#[::validated_data::data_view]
pub struct SpanningTreeSettings<'a, Mode> {
    pub mode: ::validated_data::Field<&'a str>,
    pub priority: ::validated_data::Field<i64>,
    pub port_id_allocation_port_channel_range: ::validated_data::Field<super::eos_cli_config_gen::spanning_tree::PortIdAllocationPortChannelRange<'a, Mode>>,
    pub loop_guard_default: ::validated_data::Field<bool>,
    pub edge_port_bpduguard_default: ::validated_data::Field<bool>,
}

pub mod spanning_tree_settings;

#[::validated_data::data_view]
pub struct SshSettings<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub vrfs: ::validated_data::Field<ssh_settings::Vrfs<'a, Mode>>,
    pub idle_timeout: ::validated_data::Field<i64>,
    pub client_vrfs: ::validated_data::Field<ssh_settings::ClientVrfs<'a, Mode>>,
}

pub mod ssh_settings;

#[::validated_data::data_view(indexed_list, primary_key(profile))]
pub struct SviProfiles<'a, Mode> (::validated_data::Field<svi_profiles::Item<'a, Mode>>);

pub mod svi_profiles;

#[::validated_data::data_view]
pub struct TrunkGroups<'a, Mode> {
    pub mlag: ::validated_data::Field<trunk_groups::Mlag<'a, Mode>>,
    pub mlag_l3: ::validated_data::Field<trunk_groups::MlagL3<'a, Mode>>,
    pub uplink: ::validated_data::Field<trunk_groups::Uplink<'a, Mode>>,
}

pub mod trunk_groups;

#[::validated_data::data_view]
pub struct UnderlayMulticastAnycastRp<'a, Mode> {
    pub mode: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(indexed_list, primary_key(rp))]
pub struct UnderlayMulticastRps<'a, Mode> (::validated_data::Field<underlay_multicast_rps::Item<'a, Mode>>);

pub mod underlay_multicast_rps;

#[::validated_data::data_view]
pub struct UnderlayOspfAuthentication<'a, Mode> {
    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    pub message_digest_keys: ::validated_data::RequiredValue<underlay_ospf_authentication::MessageDigestKeys<'a, Mode>, Mode>,
}

pub mod underlay_ospf_authentication;

#[::validated_data::data_view]
pub struct UplinkPtp<'a, Mode> {
    pub enable: ::validated_data::Field<bool>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct ValidationProfiles<'a, Mode> (::validated_data::Field<validation_profiles::Item<'a, Mode>>);

pub mod validation_profiles;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct WanCarriers<'a, Mode> (::validated_data::Field<wan_carriers::Item<'a, Mode>>);

pub mod wan_carriers;

#[::validated_data::data_view]
pub struct WanHa<'a, Mode> {
    pub lan_ha_path_group_name: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct WanIpsecProfiles<'a, Mode> {
    pub control_plane: ::validated_data::RequiredValue<wan_ipsec_profiles::ControlPlane<'a, Mode>, Mode>,
    pub data_plane: ::validated_data::Field<wan_ipsec_profiles::DataPlane<'a, Mode>>,
}

pub mod wan_ipsec_profiles;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct WanPathGroups<'a, Mode> (::validated_data::Field<wan_path_groups::Item<'a, Mode>>);

pub mod wan_path_groups;

#[::validated_data::data_view(indexed_list, primary_key(hostname))]
pub struct WanRouteServers<'a, Mode> (::validated_data::Field<wan_route_servers::Item<'a, Mode>>);

pub mod wan_route_servers;

#[::validated_data::data_view]
pub struct WanVirtualTopologies<'a, Mode> {
    pub vrfs: ::validated_data::Field<wan_virtual_topologies::Vrfs<'a, Mode>>,
    pub control_plane_virtual_topology: ::validated_data::Field<wan_virtual_topologies::ControlPlaneVirtualTopology<'a, Mode>>,
    pub policies: ::validated_data::Field<wan_virtual_topologies::Policies<'a, Mode>>,
}

pub mod wan_virtual_topologies;

#[::validated_data::data_view]
pub struct ZscalerEndpoints<'a, Mode> {
    pub primary: ::validated_data::RequiredValue<zscaler_endpoints::Primary<'a, Mode>, Mode>,
    pub secondary: ::validated_data::Field<zscaler_endpoints::Secondary<'a, Mode>>,
    pub tertiary: ::validated_data::Field<zscaler_endpoints::Tertiary<'a, Mode>>,
    pub cloud_name: ::validated_data::RequiredValue<&'a str, Mode>,
    pub device_location: ::validated_data::RequiredValue<zscaler_endpoints::DeviceLocation<'a, Mode>, Mode>,
}

pub mod zscaler_endpoints;

#[::validated_data::data_view]
pub struct DynamicSlot12224<'a, Mode> {
    pub defaults: ::validated_data::Field<dynamic_slot_12224::Defaults<'a, Mode>>,
    pub node_groups: ::validated_data::Field<dynamic_slot_12224::NodeGroups<'a, Mode>>,
    pub nodes: ::validated_data::Field<dynamic_slot_12224::Nodes<'a, Mode>>,
}

pub mod dynamic_slot_12224;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct DynamicSlot13723<'a, Mode> (::validated_data::Field<dynamic_slot_13723::Item<'a, Mode>>);

pub mod dynamic_slot_13723;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct DynamicSlot13911<'a, Mode> (::validated_data::Field<dynamic_slot_13911::Item<'a, Mode>>);

pub mod dynamic_slot_13911;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct DynamicSlot14099<'a, Mode> (::validated_data::Field<dynamic_slot_14099::Item<'a, Mode>>);

pub mod dynamic_slot_14099;

#[::validated_data::data_view]
pub struct DynamicSlot15021<'a, Mode> {
    pub defaults: ::validated_data::Field<dynamic_slot_15021::Defaults<'a, Mode>>,
    pub node_groups: ::validated_data::Field<dynamic_slot_15021::NodeGroups<'a, Mode>>,
    pub nodes: ::validated_data::Field<dynamic_slot_15021::Nodes<'a, Mode>>,
}

pub mod dynamic_slot_15021;
