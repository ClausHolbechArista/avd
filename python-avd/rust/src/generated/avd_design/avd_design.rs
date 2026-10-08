// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AvdDesign {
        model aaa_settings("aaa_settings", 0) -> AaaSettings<'a>;
        model address_locking_settings("address_locking_settings", 1) -> AddressLockingSettings<'a>;
        model application_classification("application_classification", 2) -> super::eos_cli_config_gen::ApplicationTrafficRecognition<'a>;
        model avd_design_future("avd_design_future", 3) -> AvdDesignFuture<'a>;
        scalar avd_digital_twin_mode("avd_digital_twin_mode", 4) -> bool;
        scalar avd_eos_designs_structured_config("avd_eos_designs_structured_config", 5) -> bool;
        scalar avd_structured_config_file_format("avd_structured_config_file_format", 6) -> &'a str;
        model eos_designs_validation_configuration("eos_designs_validation_configuration", 7) -> EosDesignsValidationConfiguration<'a>;
        scalar avd_vault_id("avd_vault_id", 8) -> &'a str;
        model bfd_multihop("bfd_multihop", 9) -> BfdMultihop<'a>;
        scalar bgp_as("bgp_as", 10) -> &'a str;
        scalar bgp_as_notation("bgp_as_notation", 11) -> &'a str;
        scalar bgp_default_ipv4_unicast("bgp_default_ipv4_unicast", 12) -> bool;
        model bgp_distance("bgp_distance", 13) -> super::eos_cli_config_gen::router_bgp::Distance<'a>;
        scalar bgp_ecmp("bgp_ecmp", 14) -> i64;
        model bgp_graceful_restart("bgp_graceful_restart", 15) -> BgpGracefulRestart<'a>;
        scalar bgp_maximum_paths("bgp_maximum_paths", 16) -> i64;
        scalar bgp_mesh_pes("bgp_mesh_pes", 17) -> bool;
        model bgp_peer_filters_catalog("bgp_peer_filters_catalog", 18) -> super::eos_cli_config_gen::PeerFilters<'a>;
        model bgp_peer_groups("bgp_peer_groups", 19) -> BgpPeerGroups<'a>;
        scalar bgp_update_wait_install("bgp_update_wait_install", 20) -> bool;
        scalar bgp_update_wait_for_convergence("bgp_update_wait_for_convergence", 21) -> bool;
        scalar campus("campus", 22) -> &'a str;
        scalar campus_access_pod("campus_access_pod", 23) -> &'a str;
        scalar campus_pod("campus_pod", 24) -> &'a str;
        model connected_endpoints("connected_endpoints", 25) -> ConnectedEndpoints<'a>;
        model custom_connected_endpoints_keys("custom_connected_endpoints_keys", 26) -> CustomConnectedEndpointsKeys<'a>;
        model connected_endpoints_keys("connected_endpoints_keys", 27) -> ConnectedEndpointsKeys<'a>;
        model core_interfaces("core_interfaces", 28) -> CoreInterfaces<'a>;
        scalar custom_structured_configuration_list_merge("custom_structured_configuration_list_merge", 29) -> &'a str;
        model custom_structured_configuration_prefix("custom_structured_configuration_prefix", 30) -> CustomStructuredConfigurationPrefix<'a>;
        model cv_pathfinder_global_sites("cv_pathfinder_global_sites", 31) -> CvPathfinderGlobalSites<'a>;
        model cv_pathfinder_internet_exit_policies("cv_pathfinder_internet_exit_policies", 32) -> CvPathfinderInternetExitPolicies<'a>;
        model cv_pathfinder_regions("cv_pathfinder_regions", 33) -> CvPathfinderRegions<'a>;
        scalar cv_server("cv_server", 34) -> &'a str;
        model cv_settings("cv_settings", 35) -> CvSettings<'a>;
        scalar cv_tags_topology_type("cv_tags_topology_type", 36) -> &'a str;
        scalar cv_token("cv_token", 37) -> &'a str;
        model cv_topology("cv_topology", 38) -> CvTopology<'a>;
        model cv_topology_levels("cv_topology_levels", 39) -> CvTopologyLevels<'a>;
        scalar dc_name("dc_name", 40) -> &'a str;
        scalar default_connected_endpoints_description("default_connected_endpoints_description", 41) -> &'a str;
        scalar default_connected_endpoints_port_channel_description("default_connected_endpoints_port_channel_description", 42) -> &'a str;
        scalar default_igmp_snooping_enabled("default_igmp_snooping_enabled", 43) -> bool;
        scalar default_interface_mtu("default_interface_mtu", 44) -> i64;
        model default_interfaces("default_interfaces", 45) -> DefaultInterfaces<'a>;
        scalar default_mgmt_method("default_mgmt_method", 46) -> &'a str;
        scalar default_network_ports_description("default_network_ports_description", 47) -> &'a str;
        scalar default_network_ports_port_channel_description("default_network_ports_port_channel_description", 48) -> &'a str;
        model default_node_types("default_node_types", 49) -> DefaultNodeTypes<'a>;
        scalar default_underlay_p2p_ethernet_description("default_underlay_p2p_ethernet_description", 50) -> &'a str;
        scalar default_underlay_p2p_port_channel_description("default_underlay_p2p_port_channel_description", 51) -> &'a str;
        scalar default_vrf_diag_loopback_description("default_vrf_diag_loopback_description", 52) -> &'a str;
        scalar device_profile("device_profile", 53) -> &'a str;
        model device_profiles("device_profiles", 54) -> DeviceProfiles<'a>;
        model devices("devices", 55) -> Devices<'a>;
        model digital_twin("digital_twin", 56) -> DigitalTwin<'a>;
        model dns_settings("dns_settings", 57) -> DnsSettings<'a>;
        model dot1x_settings("dot1x_settings", 58) -> Dot1xSettings<'a>;
        scalar enable_trunk_groups("enable_trunk_groups", 59) -> bool;
        model eos_designs_custom_templates("eos_designs_custom_templates", 60) -> EosDesignsCustomTemplates<'a>;
        model eos_designs_documentation("eos_designs_documentation", 61) -> EosDesignsDocumentation<'a>;
        scalar eos_designs_keep_tmp_files("eos_designs_keep_tmp_files", 62) -> bool;
        scalar eos_designs_return_structured_config("eos_designs_return_structured_config", 63) -> bool;
        scalar eos_designs_tmp_dir("eos_designs_tmp_dir", 64) -> &'a str;
        scalar eos_designs_validate_inputs_batch_size("eos_designs_validate_inputs_batch_size", 65) -> i64;
        scalar eos_designs_validate_inputs_template_with_multiprocessing("eos_designs_validate_inputs_template_with_multiprocessing", 66) -> bool;
        model errdisable_settings("errdisable_settings", 67) -> ErrdisableSettings<'a>;
        model event_handlers("event_handlers", 68) -> super::eos_cli_config_gen::EventHandlers<'a>;
        model event_monitor("event_monitor", 69) -> super::eos_cli_config_gen::EventMonitor<'a>;
        scalar evpn_ebgp_gateway_multihop("evpn_ebgp_gateway_multihop", 70) -> i64;
        scalar evpn_ebgp_multihop("evpn_ebgp_multihop", 71) -> i64;
        model evpn_hostflap_detection("evpn_hostflap_detection", 72) -> EvpnHostflapDetection<'a>;
        scalar evpn_import_pruning("evpn_import_pruning", 73) -> bool;
        scalar evpn_multicast("evpn_multicast", 74) -> bool;
        scalar evpn_overlay_bgp_rtc("evpn_overlay_bgp_rtc", 75) -> bool;
        scalar evpn_prevent_readvertise_to_server("evpn_prevent_readvertise_to_server", 76) -> bool;
        scalar evpn_prevent_readvertise_to_server_mode("evpn_prevent_readvertise_to_server_mode", 77) -> &'a str;
        scalar evpn_short_esi_prefix("evpn_short_esi_prefix", 78) -> &'a str;
        scalar evpn_vlan_aware_bundles("evpn_vlan_aware_bundles", 79) -> bool;
        model evpn_vlan_bundles("evpn_vlan_bundles", 80) -> EvpnVlanBundles<'a>;
        scalar fabric_evpn_encapsulation("fabric_evpn_encapsulation", 81) -> &'a str;
        model fabric_flow_tracking("fabric_flow_tracking", 82) -> FabricFlowTracking<'a>;
        model fabric_ip_addressing("fabric_ip_addressing", 83) -> FabricIpAddressing<'a>;
        scalar fabric_name("fabric_name", 84) -> &'a str;
        model fabric_numbering("fabric_numbering", 85) -> FabricNumbering<'a>;
        scalar fabric_numbering_node_id_pool("fabric_numbering_node_id_pool", 86) -> &'a str;
        model fabric_sflow("fabric_sflow", 87) -> FabricSflow<'a>;
        model flow_tracking_settings("flow_tracking_settings", 88) -> FlowTrackingSettings<'a>;
        model general_settings("general_settings", 89) -> GeneralSettings<'a>;
        model generate_cv_tags("generate_cv_tags", 90) -> GenerateCvTags<'a>;
        model hardware_counters("hardware_counters", 91) -> super::eos_cli_config_gen::HardwareCounters<'a>;
        scalar inband_ztp_bootstrap_file("inband_ztp_bootstrap_file", 92) -> &'a str;
        model internal_vlan_order("internal_vlan_order", 93) -> InternalVlanOrder<'a>;
        model ipsec_settings("ipsec_settings", 94) -> IpsecSettings<'a>;
        model ipv4_acls("ipv4_acls", 95) -> Ipv4Acls<'a>;
        model ipv4_prefix_list_catalog("ipv4_prefix_list_catalog", 96) -> Ipv4PrefixListCatalog<'a>;
        model ipv4_standard_acls("ipv4_standard_acls", 97) -> Ipv4StandardAcls<'a>;
        model ipv6_acls("ipv6_acls", 98) -> Ipv6Acls<'a>;
        model ipv6_mgmt_destination_networks("ipv6_mgmt_destination_networks", 99) -> Ipv6MgmtDestinationNetworks<'a>;
        scalar ipv6_mgmt_gateway("ipv6_mgmt_gateway", 100) -> &'a str;
        model ipv6_prefix_list_catalog("ipv6_prefix_list_catalog", 101) -> Ipv6PrefixListCatalog<'a>;
        scalar is_deployed("is_deployed", 102) -> bool;
        scalar isis_advertise_passive_only("isis_advertise_passive_only", 103) -> bool;
        scalar isis_area_id("isis_area_id", 104) -> &'a str;
        scalar isis_default_circuit_type("isis_default_circuit_type", 105) -> &'a str;
        scalar isis_default_is_type("isis_default_is_type", 106) -> &'a str;
        scalar isis_default_metric("isis_default_metric", 107) -> i64;
        scalar isis_maximum_paths("isis_maximum_paths", 108) -> i64;
        scalar isis_system_id_format("isis_system_id_format", 109) -> &'a str;
        model isis_ti_lfa("isis_ti_lfa", 110) -> IsisTiLfa<'a>;
        model l2vlan_profiles("l2vlan_profiles", 111) -> L2vlanProfiles<'a>;
        model l3_edge("l3_edge", 112) -> L3Edge<'a>;
        model l3_interface_profiles("l3_interface_profiles", 113) -> L3InterfaceProfiles<'a>;
        model load_interval("load_interval", 114) -> super::eos_cli_config_gen::LoadInterval<'a>;
        model logging_settings("logging_settings", 115) -> LoggingSettings<'a>;
        model mac_acls("mac_acls", 116) -> MacAcls<'a>;
        model mac_address_table("mac_address_table", 117) -> super::eos_cli_config_gen::MacAddressTable<'a>;
        model management_eapi("management_eapi", 118) -> ManagementEapi<'a>;
        model management_settings("management_settings", 119) -> ManagementSettings<'a>;
        model mgmt_destination_networks("mgmt_destination_networks", 120) -> MgmtDestinationNetworks<'a>;
        scalar mgmt_gateway("mgmt_gateway", 121) -> &'a str;
        scalar mgmt_interface("mgmt_interface", 122) -> &'a str;
        scalar mgmt_interface_description("mgmt_interface_description", 123) -> &'a str;
        model mgmt_interface_settings("mgmt_interface_settings", 124) -> MgmtInterfaceSettings<'a>;
        scalar mgmt_interface_vrf("mgmt_interface_vrf", 125) -> &'a str;
        scalar mgmt_vrf_routing("mgmt_vrf_routing", 126) -> bool;
        scalar mlag_bgp_peer_description("mlag_bgp_peer_description", 127) -> &'a str;
        scalar mlag_bgp_peer_group_description("mlag_bgp_peer_group_description", 128) -> &'a str;
        model mlag_ibgp_peering_vrfs("mlag_ibgp_peering_vrfs", 129) -> MlagIbgpPeeringVrfs<'a>;
        scalar mlag_member_description("mlag_member_description", 130) -> &'a str;
        scalar mlag_on_orphan_port_channel_downlink("mlag_on_orphan_port_channel_downlink", 131) -> bool;
        scalar mlag_peer_l3_svi_description("mlag_peer_l3_svi_description", 132) -> &'a str;
        scalar mlag_peer_l3_vlan_name("mlag_peer_l3_vlan_name", 133) -> &'a str;
        scalar mlag_peer_l3_vrf_svi_description("mlag_peer_l3_vrf_svi_description", 134) -> &'a str;
        scalar mlag_peer_l3_vrf_vlan_name("mlag_peer_l3_vrf_vlan_name", 135) -> &'a str;
        scalar mlag_peer_svi_description("mlag_peer_svi_description", 136) -> &'a str;
        scalar mlag_peer_vlan_name("mlag_peer_vlan_name", 137) -> &'a str;
        scalar mlag_port_channel_description("mlag_port_channel_description", 138) -> &'a str;
        model monitor_connectivity("monitor_connectivity", 139) -> MonitorConnectivity<'a>;
        model network_ports("network_ports", 140) -> NetworkPorts<'a>;
        model network_services("network_services", 141) -> NetworkServices<'a>;
        model network_services_keys("network_services_keys", 142) -> NetworkServicesKeys<'a>;
        model custom_node_type_keys("custom_node_type_keys", 143) -> CustomNodeTypeKeys<'a>;
        model node_type_keys("node_type_keys", 144) -> NodeTypeKeys<'a>;
        model ntp_settings("ntp_settings", 145) -> NtpSettings<'a>;
        scalar only_local_vlan_trunk_groups("only_local_vlan_trunk_groups", 146) -> bool;
        scalar overlay_bgp_peer_description("overlay_bgp_peer_description", 147) -> &'a str;
        model overlay_cvx_servers("overlay_cvx_servers", 148) -> OverlayCvxServers<'a>;
        scalar overlay_her_flood_list_per_vni("overlay_her_flood_list_per_vni", 149) -> bool;
        scalar overlay_her_flood_list_scope("overlay_her_flood_list_scope", 150) -> &'a str;
        scalar overlay_mlag_rfc5549("overlay_mlag_rfc5549", 151) -> bool;
        model overlay_rd_type("overlay_rd_type", 152) -> OverlayRdType<'a>;
        scalar overlay_routing_protocol("overlay_routing_protocol", 153) -> &'a str;
        scalar overlay_routing_protocol_address_family("overlay_routing_protocol_address_family", 154) -> &'a str;
        model overlay_rt_type("overlay_rt_type", 155) -> OverlayRtType<'a>;
        scalar p2p_uplinks_mtu("p2p_uplinks_mtu", 156) -> i64;
        scalar p2p_uplinks_qos_profile("p2p_uplinks_qos_profile", 157) -> &'a str;
        model custom_platform_settings("custom_platform_settings", 158) -> CustomPlatformSettings<'a>;
        model platform_settings("platform_settings", 159) -> PlatformSettings<'a>;
        model platform_speed_groups("platform_speed_groups", 160) -> PlatformSpeedGroups<'a>;
        scalar pod_name("pod_name", 161) -> &'a str;
        model port_profiles("port_profiles", 162) -> PortProfiles<'a>;
        model ptp_profiles("ptp_profiles", 163) -> PtpProfiles<'a>;
        model ptp_settings("ptp_settings", 164) -> PtpSettings<'a>;
        model queue_monitor_length("queue_monitor_length", 165) -> QueueMonitorLength<'a>;
        model queue_monitor_streaming("queue_monitor_streaming", 166) -> super::eos_cli_config_gen::QueueMonitorStreaming<'a>;
        model redundancy("redundancy", 167) -> Redundancy<'a>;
        scalar router_id_loopback_description("router_id_loopback_description", 168) -> &'a str;
        scalar serial_number("serial_number", 169) -> &'a str;
        model sflow_settings("sflow_settings", 170) -> SflowSettings<'a>;
        scalar shutdown_bgp_towards_undeployed_peers("shutdown_bgp_towards_undeployed_peers", 171) -> bool;
        scalar shutdown_interfaces_towards_undeployed_peers("shutdown_interfaces_towards_undeployed_peers", 172) -> bool;
        model snmp_settings("snmp_settings", 173) -> SnmpSettings<'a>;
        model source_interfaces("source_interfaces", 174) -> SourceInterfaces<'a>;
        model spanning_tree_settings("spanning_tree_settings", 175) -> SpanningTreeSettings<'a>;
        model ssh_settings("ssh_settings", 176) -> SshSettings<'a>;
        model svi_profiles("svi_profiles", 177) -> SviProfiles<'a>;
        scalar system_mac_address("system_mac_address", 178) -> &'a str;
        model tcam_profiles("tcam_profiles", 179) -> super::eos_cli_config_gen::tcam_profile::Profiles<'a>;
        scalar timezone("timezone", 180) -> &'a str;
        model trunk_groups("trunk_groups", 181) -> TrunkGroups<'a>;
        scalar field_type("type", 182) -> &'a str;
        scalar underlay_filter_peer_as("underlay_filter_peer_as", 183) -> bool;
        scalar underlay_filter_redistribute_connected("underlay_filter_redistribute_connected", 184) -> bool;
        scalar underlay_ipv6("underlay_ipv6", 185) -> bool;
        scalar underlay_ipv6_numbered("underlay_ipv6_numbered", 186) -> bool;
        scalar underlay_isis_authentication_cleartext_key("underlay_isis_authentication_cleartext_key", 187) -> &'a str;
        scalar underlay_isis_authentication_key("underlay_isis_authentication_key", 188) -> &'a str;
        scalar underlay_isis_authentication_mode("underlay_isis_authentication_mode", 189) -> &'a str;
        scalar underlay_isis_bfd("underlay_isis_bfd", 190) -> bool;
        scalar underlay_isis_instance_name("underlay_isis_instance_name", 191) -> &'a str;
        scalar underlay_l2_ethernet_description("underlay_l2_ethernet_description", 192) -> &'a str;
        scalar underlay_l2_port_channel_description("underlay_l2_port_channel_description", 193) -> &'a str;
        model underlay_multicast_anycast_rp("underlay_multicast_anycast_rp", 194) -> UnderlayMulticastAnycastRp<'a>;
        scalar underlay_multicast_pim_sm("underlay_multicast_pim_sm", 195) -> bool;
        model underlay_multicast_rps("underlay_multicast_rps", 196) -> UnderlayMulticastRps<'a>;
        scalar underlay_multicast_static("underlay_multicast_static", 197) -> bool;
        scalar underlay_ospf_area("underlay_ospf_area", 198) -> &'a str;
        model underlay_ospf_authentication("underlay_ospf_authentication", 199) -> UnderlayOspfAuthentication<'a>;
        scalar underlay_ospf_bfd_enable("underlay_ospf_bfd_enable", 200) -> bool;
        scalar underlay_ospf_graceful_restart("underlay_ospf_graceful_restart", 201) -> bool;
        scalar underlay_ospf_max_lsa("underlay_ospf_max_lsa", 202) -> i64;
        scalar underlay_ospf_maximum_paths("underlay_ospf_maximum_paths", 203) -> i64;
        scalar underlay_ospf_process_id("underlay_ospf_process_id", 204) -> i64;
        scalar underlay_rfc5549("underlay_rfc5549", 205) -> bool;
        scalar underlay_routing_protocol("underlay_routing_protocol", 206) -> &'a str;
        model unsupported_transceiver("unsupported_transceiver", 207) -> super::eos_cli_config_gen::ServiceUnsupportedTransceiver<'a>;
        model uplink_ptp("uplink_ptp", 208) -> UplinkPtp<'a>;
        scalar use_cv_topology("use_cv_topology", 209) -> bool;
        scalar use_router_general_for_router_id("use_router_general_for_router_id", 210) -> bool;
        model validation_profiles("validation_profiles", 211) -> ValidationProfiles<'a>;
        scalar vtep_loopback_description("vtep_loopback_description", 212) -> &'a str;
        scalar vtep_vvtep_ip("vtep_vvtep_ip", 213) -> &'a str;
        model wan_carriers("wan_carriers", 214) -> WanCarriers<'a>;
        scalar wan_encapsulation("wan_encapsulation", 215) -> &'a str;
        model wan_ha("wan_ha", 216) -> WanHa<'a>;
        model wan_ipsec_profiles("wan_ipsec_profiles", 217) -> WanIpsecProfiles<'a>;
        scalar wan_mode("wan_mode", 218) -> &'a str;
        model wan_path_groups("wan_path_groups", 219) -> WanPathGroups<'a>;
        model wan_route_servers("wan_route_servers", 220) -> WanRouteServers<'a>;
        scalar wan_stun_dtls_disable("wan_stun_dtls_disable", 221) -> bool;
        scalar wan_stun_dtls_profile_name("wan_stun_dtls_profile_name", 222) -> &'a str;
        model wan_virtual_topologies("wan_virtual_topologies", 223) -> WanVirtualTopologies<'a>;
        model zscaler_endpoints("zscaler_endpoints", 224) -> ZscalerEndpoints<'a>;
        dynamic_model custom_node_type_keys_key("custom_node_type_keys.key", 225) -> DynamicSlot12216<'a>;
        dynamic_model connected_endpoints_keys_key("connected_endpoints_keys.key", 226) -> DynamicSlot13711<'a>;
        dynamic_model custom_connected_endpoints_keys_key("custom_connected_endpoints_keys.key", 227) -> DynamicSlot13899<'a>;
        dynamic_model network_services_keys_name("network_services_keys.name", 228) -> DynamicSlot14087<'a>;
        dynamic_model node_type_keys_key("node_type_keys.key", 229) -> DynamicSlot15007<'a>;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AaaSettings {
        model enable_password("enable_password", 0) -> aaa_settings::EnablePassword<'a>;
        model tacacs("tacacs", 1) -> aaa_settings::Tacacs<'a>;
        model radius("radius", 2) -> aaa_settings::Radius<'a>;
        model authentication("authentication", 3) -> aaa_settings::Authentication<'a>;
        model authorization("authorization", 4) -> aaa_settings::Authorization<'a>;
        model accounting("accounting", 5) -> aaa_settings::Accounting<'a>;
        model root_login("root_login", 6) -> aaa_settings::RootLogin<'a>;
        model local_users("local_users", 7) -> aaa_settings::LocalUsers<'a>;
    }
}

pub mod aaa_settings;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AddressLockingSettings {
        scalar local_interface("local_interface", 0) -> &'a str;
        model dhcp_server_interfaces("dhcp_server_interfaces", 1) -> address_locking_settings::DhcpServerInterfaces<'a>;
        model dhcp_servers_ipv4("dhcp_servers_ipv4", 2) -> address_locking_settings::DhcpServersIpv4<'a>;
        scalar disabled("disabled", 3) -> bool;
        model leases("leases", 4) -> address_locking_settings::Leases<'a>;
        model locked_address("locked_address", 5) -> address_locking_settings::LockedAddress<'a>;
    }
}

pub mod address_locking_settings;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AvdDesignFuture {
        scalar accept_dhcp_default_route_for_mgmt_ip_dhcp("accept_dhcp_default_route_for_mgmt_ip_dhcp", 0) -> bool;
        scalar accept_ra_default_route_for_ipv6_mgmt_ip_auto_config("accept_ra_default_route_for_ipv6_mgmt_ip_auto_config", 1) -> bool;
        scalar accept_dhcp_default_route_for_inband_mgmt_ip_dhcp("accept_dhcp_default_route_for_inband_mgmt_ip_dhcp", 2) -> bool;
        scalar allow_recursive_profile_inheritance("allow_recursive_profile_inheritance", 3) -> bool;
        scalar configure_inband_mgmt_ipv6_vrf("configure_inband_mgmt_ipv6_vrf", 4) -> bool;
        scalar consistent_uplink_vlans("consistent_uplink_vlans", 5) -> bool;
        scalar fix_address_locking_dhcp_server_interfaces("fix_address_locking_dhcp_server_interfaces", 6) -> bool;
        scalar fix_match_ipv6_prefix_list_on_mlag_route_map("fix_match_ipv6_prefix_list_on_mlag_route_map", 7) -> bool;
        scalar fix_radius_server_group_tls("fix_radius_server_group_tls", 8) -> bool;
        scalar only_configure_ipv6_inband_mgmt_prefix_list_when_used("only_configure_ipv6_inband_mgmt_prefix_list_when_used", 9) -> bool;
        scalar only_configure_mlag_vrfs_peer_group_when_used("only_configure_mlag_vrfs_peer_group_when_used", 10) -> bool;
        scalar only_configure_pvst_border_when_mode_is_mstp("only_configure_pvst_border_when_mode_is_mstp", 11) -> bool;
        scalar only_configure_route_map_connected_to_bgp_vrfs_when_used("only_configure_route_map_connected_to_bgp_vrfs_when_used", 12) -> bool;
        scalar raise_for_port_channels_without_members("raise_for_port_channels_without_members", 13) -> bool;
        scalar raise_for_underlay_router_with_uplink_type_port_channel("raise_for_underlay_router_with_uplink_type_port_channel", 14) -> bool;
        scalar remove_redundant_ipv4_unicast_for_peer_groups("remove_redundant_ipv4_unicast_for_peer_groups", 15) -> bool;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct EosDesignsValidationConfiguration {
        scalar warn_eos_config_keys("warn_eos_config_keys", 0) -> bool;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct BfdMultihop {
        scalar interval("interval", 0) -> i64;
        scalar min_rx("min_rx", 1) -> i64;
        scalar multiplier("multiplier", 2) -> i64;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct BgpGracefulRestart {
        scalar enabled("enabled", 0) -> bool;
        scalar restart_time("restart_time", 1) -> i64;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct BgpPeerGroups {
        model ipv4_underlay_peers("ipv4_underlay_peers", 0) -> bgp_peer_groups::Ipv4UnderlayPeers<'a>;
        model mlag_ipv4_vrfs_peer("mlag_ipv4_vrfs_peer", 1) -> bgp_peer_groups::MlagIpv4VrfsPeer<'a>;
        model mlag_ipv4_underlay_peer("mlag_ipv4_underlay_peer", 2) -> bgp_peer_groups::MlagIpv4UnderlayPeer<'a>;
        model evpn_overlay_peers("evpn_overlay_peers", 3) -> bgp_peer_groups::EvpnOverlayPeers<'a>;
        model evpn_overlay_core("evpn_overlay_core", 4) -> bgp_peer_groups::EvpnOverlayCore<'a>;
        model mpls_overlay_peers("mpls_overlay_peers", 5) -> bgp_peer_groups::MplsOverlayPeers<'a>;
        model rr_overlay_peers("rr_overlay_peers", 6) -> bgp_peer_groups::RrOverlayPeers<'a>;
        model ipvpn_gateway_peers("ipvpn_gateway_peers", 7) -> bgp_peer_groups::IpvpnGatewayPeers<'a>;
        model wan_overlay_peers("wan_overlay_peers", 8) -> bgp_peer_groups::WanOverlayPeers<'a>;
        model wan_rr_overlay_peers("wan_rr_overlay_peers", 9) -> bgp_peer_groups::WanRrOverlayPeers<'a>;
    }
}

pub mod bgp_peer_groups;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ConnectedEndpoints {
        model item (0) -> connected_endpoints::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod connected_endpoints;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct CustomConnectedEndpointsKeys {
        model item (0) -> custom_connected_endpoints_keys::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod custom_connected_endpoints_keys;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ConnectedEndpointsKeys {
        model item (0) -> connected_endpoints_keys::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod connected_endpoints_keys;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct CoreInterfaces {
        model p2p_links_ip_pools("p2p_links_ip_pools", 0) -> core_interfaces::P2pLinksIpPools<'a>;
        model p2p_links_profiles("p2p_links_profiles", 1) -> core_interfaces::P2pLinksProfiles<'a>;
        model p2p_links("p2p_links", 2) -> core_interfaces::P2pLinks<'a>;
    }
}

pub mod core_interfaces;

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct CustomStructuredConfigurationPrefix {
        scalar item (0) -> &'a str;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct CvPathfinderGlobalSites {
        model item (0) -> cv_pathfinder_global_sites::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod cv_pathfinder_global_sites;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct CvPathfinderInternetExitPolicies {
        model item (0) -> cv_pathfinder_internet_exit_policies::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod cv_pathfinder_internet_exit_policies;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct CvPathfinderRegions {
        model item (0) -> cv_pathfinder_regions::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod cv_pathfinder_regions;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct CvSettings {
        model cvaas("cvaas", 0) -> cv_settings::Cvaas<'a>;
        model onprem_clusters("onprem_clusters", 1) -> cv_settings::OnpremClusters<'a>;
        model terminattr("terminattr", 2) -> cv_settings::Terminattr<'a>;
        scalar set_source_interfaces("set_source_interfaces", 3) -> bool;
    }
}

pub mod cv_settings;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct CvTopology {
        model item (0) -> cv_topology::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod cv_topology;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct CvTopologyLevels {
        model item (0) -> cv_topology_levels::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod cv_topology_levels;

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DefaultInterfaces {
        model item (0) -> default_interfaces::Item<'a>;
    }
}

pub mod default_interfaces;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DefaultNodeTypes {
        model item (0) -> default_node_types::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod default_node_types;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DeviceProfiles {
        model item (0) -> device_profiles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod device_profiles;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Devices {
        model item (0) -> devices::Item<'a>;
        primary_key_fields: [3];
    }
}

pub mod devices;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DigitalTwin {
        scalar environment("environment", 0) -> &'a str;
        model fabric("fabric", 1) -> digital_twin::Fabric<'a>;
        scalar use_default_interfaces_of_digital_twin_platform("use_default_interfaces_of_digital_twin_platform", 2) -> bool;
    }
}

pub mod digital_twin;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DnsSettings {
        scalar domain("domain", 0) -> &'a str;
        model domain_list("domain_list", 1) -> dns_settings::DomainList<'a>;
        model servers("servers", 2) -> dns_settings::Servers<'a>;
        model vrfs("vrfs", 3) -> dns_settings::Vrfs<'a>;
        scalar set_source_interfaces("set_source_interfaces", 4) -> bool;
        model ip_hosts("ip_hosts", 5) -> super::eos_cli_config_gen::IpHosts<'a>;
    }
}

pub mod dns_settings;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Dot1xSettings {
        scalar enabled("enabled", 0) -> bool;
        model authentication("authentication", 1) -> dot1x_settings::Authentication<'a>;
        model accounting("accounting", 2) -> dot1x_settings::Accounting<'a>;
        scalar bypass_bpdu("bypass_bpdu", 3) -> bool;
        scalar bypass_lldp("bypass_lldp", 4) -> bool;
        model dynamic_authorization("dynamic_authorization", 5) -> dot1x_settings::DynamicAuthorization<'a>;
        model mac_based_authentication("mac_based_authentication", 6) -> dot1x_settings::MacBasedAuthentication<'a>;
        model radius_av_pairs("radius_av_pairs", 7) -> dot1x_settings::RadiusAvPairs<'a>;
        model device_profiling("device_profiling", 8) -> dot1x_settings::DeviceProfiling<'a>;
        scalar redistribute_in_evpn("redistribute_in_evpn", 9) -> bool;
        model web_authentication("web_authentication", 10) -> dot1x_settings::WebAuthentication<'a>;
    }
}

pub mod dot1x_settings;

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct EosDesignsCustomTemplates {
        model item (0) -> eos_designs_custom_templates::Item<'a>;
    }
}

pub mod eos_designs_custom_templates;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct EosDesignsDocumentation {
        scalar enable("enable", 0) -> bool;
        scalar connected_endpoints("connected_endpoints", 1) -> bool;
        scalar topology_csv("topology_csv", 2) -> bool;
        scalar p2p_links_csv("p2p_links_csv", 3) -> bool;
        scalar toc("toc", 4) -> bool;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ErrdisableSettings {
        scalar recovery_interval("recovery_interval", 0) -> i64;
        model causes("causes", 1) -> errdisable_settings::Causes<'a>;
    }
}

pub mod errdisable_settings;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct EvpnHostflapDetection {
        scalar enabled("enabled", 0) -> bool;
        scalar threshold("threshold", 1) -> i64;
        scalar window("window", 2) -> i64;
        scalar expiry_timeout("expiry_timeout", 3) -> i64;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct EvpnVlanBundles {
        model item (0) -> evpn_vlan_bundles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod evpn_vlan_bundles;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct FabricFlowTracking {
        model uplinks("uplinks", 0) -> fabric_flow_tracking::Uplinks<'a>;
        model downlinks("downlinks", 1) -> fabric_flow_tracking::Downlinks<'a>;
        model endpoints("endpoints", 2) -> fabric_flow_tracking::Endpoints<'a>;
        model l3_edge("l3_edge", 3) -> fabric_flow_tracking::L3Edge<'a>;
        model core_interfaces("core_interfaces", 4) -> fabric_flow_tracking::CoreInterfaces<'a>;
        model mlag_interfaces("mlag_interfaces", 5) -> fabric_flow_tracking::MlagInterfaces<'a>;
        model l3_interfaces("l3_interfaces", 6) -> fabric_flow_tracking::L3Interfaces<'a>;
        model l3_port_channels("l3_port_channels", 7) -> fabric_flow_tracking::L3PortChannels<'a>;
        model dps_interfaces("dps_interfaces", 8) -> fabric_flow_tracking::DpsInterfaces<'a>;
        model direct_wan_ha_links("direct_wan_ha_links", 9) -> fabric_flow_tracking::DirectWanHaLinks<'a>;
    }
}

pub mod fabric_flow_tracking;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct FabricIpAddressing {
        model loopback("loopback", 0) -> fabric_ip_addressing::Loopback<'a>;
        model mlag("mlag", 1) -> fabric_ip_addressing::Mlag<'a>;
        model p2p_uplinks("p2p_uplinks", 2) -> fabric_ip_addressing::P2pUplinks<'a>;
        model wan_ha("wan_ha", 3) -> fabric_ip_addressing::WanHa<'a>;
    }
}

pub mod fabric_ip_addressing;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct FabricNumbering {
        model node_id("node_id", 0) -> fabric_numbering::NodeId<'a>;
    }
}

pub mod fabric_numbering;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct FabricSflow {
        scalar uplinks("uplinks", 0) -> bool;
        scalar downlinks("downlinks", 1) -> bool;
        scalar endpoints("endpoints", 2) -> bool;
        scalar l3_edge("l3_edge", 3) -> bool;
        scalar core_interfaces("core_interfaces", 4) -> bool;
        scalar mlag_interfaces("mlag_interfaces", 5) -> bool;
        scalar l3_interfaces("l3_interfaces", 6) -> bool;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct FlowTrackingSettings {
        model sampled("sampled", 0) -> flow_tracking_settings::Sampled<'a>;
        model hardware("hardware", 1) -> flow_tracking_settings::Hardware<'a>;
        model cloudvision_exporter("cloudvision_exporter", 2) -> flow_tracking_settings::CloudvisionExporter<'a>;
        model trackers("trackers", 3) -> flow_tracking_settings::Trackers<'a>;
    }
}

pub mod flow_tracking_settings;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct GeneralSettings {
        model interface_defaults("interface_defaults", 0) -> general_settings::InterfaceDefaults<'a>;
        model arp("arp", 1) -> general_settings::Arp<'a>;
        scalar ip_icmp_redirect("ip_icmp_redirect", 2) -> bool;
        model dhcp_relay("dhcp_relay", 3) -> general_settings::DhcpRelay<'a>;
        model suspended_vlans("suspended_vlans", 4) -> general_settings::SuspendedVlans<'a>;
    }
}

pub mod general_settings;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct GenerateCvTags {
        scalar topology_hints("topology_hints", 0) -> bool;
        scalar campus_fabric("campus_fabric", 1) -> bool;
        model interface_tags("interface_tags", 2) -> generate_cv_tags::InterfaceTags<'a>;
        model device_tags("device_tags", 3) -> generate_cv_tags::DeviceTags<'a>;
    }
}

pub mod generate_cv_tags;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct InternalVlanOrder {
        scalar allocation("allocation", 0) -> &'a str;
        model range("range", 1) -> internal_vlan_order::Range<'a>;
    }
}

pub mod internal_vlan_order;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpsecSettings {
        scalar bind_connection_to_interface("bind_connection_to_interface", 0) -> bool;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Ipv4Acls {
        model item (0) -> ipv4_acls::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod ipv4_acls;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Ipv4PrefixListCatalog {
        model item (0) -> ipv4_prefix_list_catalog::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod ipv4_prefix_list_catalog;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Ipv4StandardAcls {
        model item (0) -> ipv4_standard_acls::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod ipv4_standard_acls;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Ipv6Acls {
        model item (0) -> ipv6_acls::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod ipv6_acls;

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Ipv6MgmtDestinationNetworks {
        scalar item (0) -> &'a str;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Ipv6PrefixListCatalog {
        model item (0) -> ipv6_prefix_list_catalog::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod ipv6_prefix_list_catalog;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IsisTiLfa {
        scalar enabled("enabled", 0) -> bool;
        scalar protection("protection", 1) -> &'a str;
        scalar local_convergence_delay("local_convergence_delay", 2) -> i64;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct L2vlanProfiles {
        model item (0) -> l2vlan_profiles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod l2vlan_profiles;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct L3Edge {
        model p2p_links_ip_pools("p2p_links_ip_pools", 0) -> l3_edge::P2pLinksIpPools<'a>;
        model p2p_links_profiles("p2p_links_profiles", 1) -> l3_edge::P2pLinksProfiles<'a>;
        model p2p_links("p2p_links", 2) -> l3_edge::P2pLinks<'a>;
    }
}

pub mod l3_edge;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct L3InterfaceProfiles {
        model item (0) -> l3_interface_profiles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod l3_interface_profiles;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct LoggingSettings {
        scalar use_local_interface_cli("use_local_interface_cli", 0) -> bool;
        model hosts("hosts", 1) -> logging_settings::Hosts<'a>;
        model vrfs("vrfs", 2) -> logging_settings::Vrfs<'a>;
        scalar console("console", 3) -> &'a str;
        scalar monitor("monitor", 4) -> &'a str;
        model buffered("buffered", 5) -> super::eos_cli_config_gen::logging::Buffered<'a>;
        scalar repeat_messages("repeat_messages", 6) -> bool;
        scalar trap("trap", 7) -> &'a str;
        model synchronous("synchronous", 8) -> super::eos_cli_config_gen::logging::Synchronous<'a>;
        model format("format", 9) -> super::eos_cli_config_gen::logging::Format<'a>;
        scalar facility("facility", 10) -> &'a str;
        model policy("policy", 11) -> super::eos_cli_config_gen::logging::Policy<'a>;
        model event("event", 12) -> super::eos_cli_config_gen::logging::Event<'a>;
        model level("level", 13) -> super::eos_cli_config_gen::logging::Level<'a>;
        model monitor_layer1("monitor_layer1", 14) -> super::eos_cli_config_gen::MonitorLayer1<'a>;
    }
}

pub mod logging_settings;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MacAcls {
        model item (0) -> mac_acls::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod mac_acls;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ManagementEapi {
        scalar enabled("enabled", 0) -> bool;
        scalar enable_http("enable_http", 1) -> bool;
        scalar enable_https("enable_https", 2) -> bool;
        scalar default_services("default_services", 3) -> bool;
        model vrfs("vrfs", 4) -> management_eapi::Vrfs<'a>;
    }
}

pub mod management_eapi;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ManagementSettings {
        model console("console", 0) -> super::eos_cli_config_gen::ManagementConsole<'a>;
        model banners("banners", 1) -> super::eos_cli_config_gen::Banners<'a>;
    }
}

pub mod management_settings;

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MgmtDestinationNetworks {
        scalar item (0) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MgmtInterfaceSettings {
        scalar description("description", 0) -> &'a str;
        scalar vrf("vrf", 1) -> &'a str;
        scalar vrf_routing("vrf_routing", 2) -> bool;
        scalar interface("interface", 3) -> &'a str;
        model lldp("lldp", 4) -> mgmt_interface_settings::Lldp<'a>;
    }
}

pub mod mgmt_interface_settings;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MlagIbgpPeeringVrfs {
        scalar base_vlan("base_vlan", 0) -> i64;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MonitorConnectivity {
        scalar shutdown("shutdown", 0) -> bool;
        scalar interval("interval", 1) -> i64;
        model interface_sets("interface_sets", 2) -> monitor_connectivity::InterfaceSets<'a>;
        scalar local_interfaces("local_interfaces", 3) -> &'a str;
        scalar address_only("address_only", 4) -> bool;
        model hosts("hosts", 5) -> monitor_connectivity::Hosts<'a>;
        scalar name_server_group("name_server_group", 6) -> &'a str;
        model vrfs("vrfs", 7) -> monitor_connectivity::Vrfs<'a>;
    }
}

pub mod monitor_connectivity;

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct NetworkPorts {
        model item (0) -> network_ports::Item<'a>;
    }
}

pub mod network_ports;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct NetworkServices {
        model item (0) -> network_services::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod network_services;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct NetworkServicesKeys {
        model item (0) -> network_services_keys::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod network_services_keys;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct CustomNodeTypeKeys {
        model item (0) -> custom_node_type_keys::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod custom_node_type_keys;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct NodeTypeKeys {
        model item (0) -> node_type_keys::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod node_type_keys;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct NtpSettings {
        scalar server_vrf("server_vrf", 0) -> &'a str;
        scalar set_first_ntp_server_as_preferred("set_first_ntp_server_as_preferred", 1) -> bool;
        model servers("servers", 2) -> ntp_settings::Servers<'a>;
        scalar authenticate("authenticate", 3) -> bool;
        scalar authenticate_servers_only("authenticate_servers_only", 4) -> bool;
        model authentication_keys("authentication_keys", 5) -> ntp_settings::AuthenticationKeys<'a>;
        scalar trusted_keys("trusted_keys", 6) -> &'a str;
    }
}

pub mod ntp_settings;

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct OverlayCvxServers {
        scalar item (0) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct OverlayRdType {
        scalar admin_subfield("admin_subfield", 0) -> &'a str;
        scalar admin_subfield_offset("admin_subfield_offset", 1) -> i64;
        scalar vrf_admin_subfield("vrf_admin_subfield", 2) -> &'a str;
        scalar vrf_admin_subfield_offset("vrf_admin_subfield_offset", 3) -> i64;
        scalar vlan_assigned_number_subfield("vlan_assigned_number_subfield", 4) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct OverlayRtType {
        scalar admin_subfield("admin_subfield", 0) -> &'a str;
        scalar vrf_admin_subfield("vrf_admin_subfield", 1) -> &'a str;
        scalar vlan_assigned_number_subfield("vlan_assigned_number_subfield", 2) -> &'a str;
    }
}

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct CustomPlatformSettings {
        model item (0) -> custom_platform_settings::Item<'a>;
    }
}

pub mod custom_platform_settings;

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct PlatformSettings {
        model item (0) -> platform_settings::Item<'a>;
    }
}

pub mod platform_settings;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct PlatformSpeedGroups {
        model item (0) -> platform_speed_groups::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod platform_speed_groups;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct PortProfiles {
        model item (0) -> port_profiles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod port_profiles;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct PtpProfiles {
        model item (0) -> ptp_profiles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod ptp_profiles;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct PtpSettings {
        scalar enabled("enabled", 0) -> bool;
        scalar profile("profile", 1) -> &'a str;
        scalar domain("domain", 2) -> i64;
        scalar auto_clock_identity("auto_clock_identity", 3) -> bool;
        scalar forward_v1("forward_v1", 4) -> bool;
        model free_running("free_running", 5) -> super::eos_cli_config_gen::ptp::FreeRunning<'a>;
    }
}

pub mod ptp_settings;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct QueueMonitorLength {
        scalar enabled("enabled", 0) -> bool;
        scalar notifying("notifying", 1) -> bool;
        model default_thresholds("default_thresholds", 2) -> queue_monitor_length::DefaultThresholds<'a>;
        scalar log("log", 3) -> i64;
        model cpu("cpu", 4) -> queue_monitor_length::Cpu<'a>;
        scalar tx_latency("tx_latency", 5) -> bool;
        model mirror("mirror", 6) -> queue_monitor_length::Mirror<'a>;
    }
}

pub mod queue_monitor_length;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Redundancy {
        scalar protocol("protocol", 0) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SflowSettings {
        scalar polling_interval("polling_interval", 0) -> i64;
        model sample("sample", 1) -> sflow_settings::Sample<'a>;
        model destinations("destinations", 2) -> sflow_settings::Destinations<'a>;
        model export_to_cloudvision("export_to_cloudvision", 3) -> sflow_settings::ExportToCloudvision<'a>;
        model vrfs("vrfs", 4) -> sflow_settings::Vrfs<'a>;
    }
}

pub mod sflow_settings;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SnmpSettings {
        scalar contact("contact", 0) -> &'a str;
        scalar location("location", 1) -> bool;
        scalar location_template("location_template", 2) -> &'a str;
        model vrfs("vrfs", 3) -> snmp_settings::Vrfs<'a>;
        scalar compute_local_engineid("compute_local_engineid", 4) -> bool;
        scalar compute_local_engineid_source("compute_local_engineid_source", 5) -> &'a str;
        scalar local_engineid_ip("local_engineid_ip", 6) -> &'a str;
        scalar compute_v3_user_localized_key("compute_v3_user_localized_key", 7) -> bool;
        model users("users", 8) -> snmp_settings::Users<'a>;
        model hosts("hosts", 9) -> snmp_settings::Hosts<'a>;
        model communities("communities", 10) -> snmp_settings::Communities<'a>;
        model views("views", 11) -> snmp_settings::Views<'a>;
        model groups("groups", 12) -> snmp_settings::Groups<'a>;
        model traps("traps", 13) -> super::eos_cli_config_gen::snmp_server::Traps<'a>;
    }
}

pub mod snmp_settings;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SourceInterfaces {
        model http_client("http_client", 0) -> source_interfaces::HttpClient<'a>;
        model ssh_client("ssh_client", 1) -> source_interfaces::SshClient<'a>;
    }
}

pub mod source_interfaces;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SpanningTreeSettings {
        scalar mode("mode", 0) -> &'a str;
        scalar priority("priority", 1) -> i64;
        model port_id_allocation_port_channel_range("port_id_allocation_port_channel_range", 2) -> super::eos_cli_config_gen::spanning_tree::PortIdAllocationPortChannelRange<'a>;
        scalar loop_guard_default("loop_guard_default", 3) -> bool;
        scalar edge_port_bpduguard_default("edge_port_bpduguard_default", 4) -> bool;
    }
}

pub mod spanning_tree_settings;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SshSettings {
        scalar enabled("enabled", 0) -> bool;
        model vrfs("vrfs", 1) -> ssh_settings::Vrfs<'a>;
        scalar idle_timeout("idle_timeout", 2) -> i64;
        model client_vrfs("client_vrfs", 3) -> ssh_settings::ClientVrfs<'a>;
    }
}

pub mod ssh_settings;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SviProfiles {
        model item (0) -> svi_profiles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod svi_profiles;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct TrunkGroups {
        model mlag("mlag", 0) -> trunk_groups::Mlag<'a>;
        model mlag_l3("mlag_l3", 1) -> trunk_groups::MlagL3<'a>;
        model uplink("uplink", 2) -> trunk_groups::Uplink<'a>;
    }
}

pub mod trunk_groups;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct UnderlayMulticastAnycastRp {
        scalar mode("mode", 0) -> &'a str;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct UnderlayMulticastRps {
        model item (0) -> underlay_multicast_rps::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod underlay_multicast_rps;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct UnderlayOspfAuthentication {
        scalar enabled("enabled", 0) -> bool;
        model message_digest_keys("message_digest_keys", 1) -> underlay_ospf_authentication::MessageDigestKeys<'a>;
    }
}

pub mod underlay_ospf_authentication;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct UplinkPtp {
        scalar enable("enable", 0) -> bool;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ValidationProfiles {
        model item (0) -> validation_profiles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod validation_profiles;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct WanCarriers {
        model item (0) -> wan_carriers::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod wan_carriers;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct WanHa {
        scalar lan_ha_path_group_name("lan_ha_path_group_name", 0) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct WanIpsecProfiles {
        model control_plane("control_plane", 0) -> wan_ipsec_profiles::ControlPlane<'a>;
        model data_plane("data_plane", 1) -> wan_ipsec_profiles::DataPlane<'a>;
    }
}

pub mod wan_ipsec_profiles;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct WanPathGroups {
        model item (0) -> wan_path_groups::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod wan_path_groups;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct WanRouteServers {
        model item (0) -> wan_route_servers::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod wan_route_servers;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct WanVirtualTopologies {
        model vrfs("vrfs", 0) -> wan_virtual_topologies::Vrfs<'a>;
        model control_plane_virtual_topology("control_plane_virtual_topology", 1) -> wan_virtual_topologies::ControlPlaneVirtualTopology<'a>;
        model policies("policies", 2) -> wan_virtual_topologies::Policies<'a>;
    }
}

pub mod wan_virtual_topologies;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ZscalerEndpoints {
        model primary("primary", 0) -> zscaler_endpoints::Primary<'a>;
        model secondary("secondary", 1) -> zscaler_endpoints::Secondary<'a>;
        model tertiary("tertiary", 2) -> zscaler_endpoints::Tertiary<'a>;
        scalar cloud_name("cloud_name", 3) -> &'a str;
        model device_location("device_location", 4) -> zscaler_endpoints::DeviceLocation<'a>;
    }
}

pub mod zscaler_endpoints;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DynamicSlot12216 {
        model defaults("defaults", 0) -> dynamic_slot_12216::Defaults<'a>;
        model node_groups("node_groups", 1) -> dynamic_slot_12216::NodeGroups<'a>;
        model nodes("nodes", 2) -> dynamic_slot_12216::Nodes<'a>;
    }
}

pub mod dynamic_slot_12216;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DynamicSlot13711 {
        model item (0) -> dynamic_slot_13711::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod dynamic_slot_13711;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DynamicSlot13899 {
        model item (0) -> dynamic_slot_13899::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod dynamic_slot_13899;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DynamicSlot14087 {
        model item (0) -> dynamic_slot_14087::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod dynamic_slot_14087;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DynamicSlot15007 {
        model defaults("defaults", 0) -> dynamic_slot_15007::Defaults<'a>;
        model node_groups("node_groups", 1) -> dynamic_slot_15007::NodeGroups<'a>;
        model nodes("nodes", 2) -> dynamic_slot_15007::Nodes<'a>;
    }
}

pub mod dynamic_slot_15007;
