// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct EosCliConfigGen {
        model aaa_accounting("aaa_accounting", 0) -> AaaAccounting<'a>;
        model aaa_authentication("aaa_authentication", 1) -> AaaAuthentication<'a>;
        model aaa_authorization("aaa_authorization", 2) -> AaaAuthorization<'a>;
        model aaa_root("aaa_root", 3) -> AaaRoot<'a>;
        model aaa_server_groups("aaa_server_groups", 4) -> AaaServerGroups<'a>;
        model access_lists("access_lists", 5) -> AccessLists<'a>;
        model address_locking("address_locking", 6) -> AddressLocking<'a>;
        model agents("agents", 7) -> Agents<'a>;
        scalar aliases("aliases", 8) -> &'a str;
        model application_traffic_recognition("application_traffic_recognition", 9) -> ApplicationTrafficRecognition<'a>;
        model arp("arp", 10) -> Arp<'a>;
        model as_path("as_path", 11) -> AsPath<'a>;
        scalar avd_structured_config_file_format("avd_structured_config_file_format", 12) -> &'a str;
        scalar avd_vault_id("avd_vault_id", 13) -> &'a str;
        model banners("banners", 14) -> Banners<'a>;
        model bgp_groups("bgp_groups", 15) -> BgpGroups<'a>;
        model boot("boot", 16) -> Boot<'a>;
        model cfm("cfm", 17) -> Cfm<'a>;
        model class_maps("class_maps", 18) -> ClassMaps<'a>;
        model clock("clock", 19) -> Clock<'a>;
        scalar config_comment("config_comment", 20) -> &'a str;
        scalar config_end("config_end", 21) -> bool;
        model custom_templates("custom_templates", 22) -> CustomTemplates<'a>;
        model cvx("cvx", 23) -> Cvx<'a>;
        model daemon_terminattr("daemon_terminattr", 24) -> DaemonTerminattr<'a>;
        model daemons("daemons", 25) -> Daemons<'a>;
        model dhcp_relay("dhcp_relay", 26) -> DhcpRelay<'a>;
        model dhcp_servers("dhcp_servers", 27) -> DhcpServers<'a>;
        scalar dns_domain("dns_domain", 28) -> &'a str;
        model domain_list("domain_list", 29) -> DomainList<'a>;
        model dot1x("dot1x", 30) -> Dot1x<'a>;
        model dps_interfaces("dps_interfaces", 31) -> DpsInterfaces<'a>;
        model dynamic_prefix_lists("dynamic_prefix_lists", 32) -> DynamicPrefixLists<'a>;
        model enable_password("enable_password", 33) -> EnablePassword<'a>;
        model environment_fan_speed("environment_fan_speed", 34) -> EnvironmentFanSpeed<'a>;
        scalar eos_cli("eos_cli", 35) -> &'a str;
        model eos_cli_config_gen_configuration("eos_cli_config_gen_configuration", 36) -> EosCliConfigGenConfiguration<'a>;
        model eos_cli_config_gen_documentation("eos_cli_config_gen_documentation", 37) -> EosCliConfigGenDocumentation<'a>;
        scalar eos_cli_config_gen_keep_tmp_files("eos_cli_config_gen_keep_tmp_files", 38) -> bool;
        scalar eos_cli_config_gen_tmp_dir("eos_cli_config_gen_tmp_dir", 39) -> &'a str;
        scalar eos_cli_config_gen_validate_inputs_batch_size("eos_cli_config_gen_validate_inputs_batch_size", 40) -> i64;
        model eos_config_future("eos_config_future", 41) -> EosConfigFuture<'a>;
        model errdisable("errdisable", 42) -> Errdisable<'a>;
        model ethernet_interfaces("ethernet_interfaces", 43) -> EthernetInterfaces<'a>;
        model event_handlers("event_handlers", 44) -> EventHandlers<'a>;
        model event_monitor("event_monitor", 45) -> EventMonitor<'a>;
        model flow_tracking("flow_tracking", 46) -> FlowTracking<'a>;
        model hardware("hardware", 47) -> Hardware<'a>;
        model hardware_counters("hardware_counters", 48) -> HardwareCounters<'a>;
        scalar hostname("hostname", 49) -> &'a str;
        model interface_defaults("interface_defaults", 50) -> InterfaceDefaults<'a>;
        model interface_groups("interface_groups", 51) -> InterfaceGroups<'a>;
        model interface_profiles("interface_profiles", 52) -> InterfaceProfiles<'a>;
        model ip_access_lists("ip_access_lists", 53) -> IpAccessLists<'a>;
        scalar ip_access_lists_max_entries("ip_access_lists_max_entries", 54) -> i64;
        model ip_community_lists("ip_community_lists", 55) -> IpCommunityLists<'a>;
        model ip_dhcp_relay("ip_dhcp_relay", 56) -> IpDhcpRelay<'a>;
        model ip_dhcp_snooping("ip_dhcp_snooping", 57) -> IpDhcpSnooping<'a>;
        model ip_domain_lookup("ip_domain_lookup", 58) -> IpDomainLookup<'a>;
        model ip_extcommunity_lists("ip_extcommunity_lists", 59) -> IpExtcommunityLists<'a>;
        model ip_extcommunity_lists_regexp("ip_extcommunity_lists_regexp", 60) -> IpExtcommunityListsRegexp<'a>;
        model ip_ftp_client("ip_ftp_client", 61) -> IpFtpClient<'a>;
        model ip_hardware("ip_hardware", 62) -> IpHardware<'a>;
        model ip_hosts("ip_hosts", 63) -> IpHosts<'a>;
        model ip_http_client("ip_http_client", 64) -> IpHttpClient<'a>;
        scalar ip_icmp_redirect("ip_icmp_redirect", 65) -> bool;
        model ip_igmp_snooping("ip_igmp_snooping", 66) -> IpIgmpSnooping<'a>;
        model ip_large_community_lists("ip_large_community_lists", 67) -> IpLargeCommunityLists<'a>;
        model ip_name_server("ip_name_server", 68) -> IpNameServer<'a>;
        model ip_name_server_groups("ip_name_server_groups", 69) -> IpNameServerGroups<'a>;
        model ip_nat("ip_nat", 70) -> IpNat<'a>;
        scalar ip_ospf_router_id_output_format_hostnames("ip_ospf_router_id_output_format_hostnames", 71) -> bool;
        model ip_radius("ip_radius", 72) -> IpRadius<'a>;
        model ip_radius_source_interfaces("ip_radius_source_interfaces", 73) -> IpRadiusSourceInterfaces<'a>;
        scalar ip_routing("ip_routing", 74) -> bool;
        scalar ip_routing_ipv6_interfaces("ip_routing_ipv6_interfaces", 75) -> bool;
        model ip_security("ip_security", 76) -> IpSecurity<'a>;
        model ip_software_forwarding("ip_software_forwarding", 77) -> IpSoftwareForwarding<'a>;
        model ip_ssh_client("ip_ssh_client", 78) -> IpSshClient<'a>;
        model ip_tacacs("ip_tacacs", 79) -> IpTacacs<'a>;
        model ip_tacacs_source_interfaces("ip_tacacs_source_interfaces", 80) -> IpTacacsSourceInterfaces<'a>;
        model ip_telnet_client("ip_telnet_client", 81) -> IpTelnetClient<'a>;
        model ip_tftp_client("ip_tftp_client", 82) -> IpTftpClient<'a>;
        scalar ip_virtual_router_mac_address("ip_virtual_router_mac_address", 83) -> &'a str;
        scalar ip_virtual_router_mac_address_advertisement_interval("ip_virtual_router_mac_address_advertisement_interval", 84) -> i64;
        scalar ip_virtual_router_mac_address_mlag_peer("ip_virtual_router_mac_address_mlag_peer", 85) -> bool;
        model ipv6_access_lists("ipv6_access_lists", 86) -> Ipv6AccessLists<'a>;
        model ipv6_dhcp_relay("ipv6_dhcp_relay", 87) -> Ipv6DhcpRelay<'a>;
        model ipv6_hardware("ipv6_hardware", 88) -> Ipv6Hardware<'a>;
        scalar ipv6_icmp_redirect("ipv6_icmp_redirect", 89) -> bool;
        model ipv6_neighbor("ipv6_neighbor", 90) -> Ipv6Neighbor<'a>;
        model ipv6_prefix_lists("ipv6_prefix_lists", 91) -> Ipv6PrefixLists<'a>;
        model ipv6_router_ospf("ipv6_router_ospf", 92) -> Ipv6RouterOspf<'a>;
        model ipv6_standard_access_lists("ipv6_standard_access_lists", 93) -> Ipv6StandardAccessLists<'a>;
        model ipv6_static_routes("ipv6_static_routes", 94) -> Ipv6StaticRoutes<'a>;
        scalar ipv6_unicast_routing("ipv6_unicast_routing", 95) -> bool;
        model kernel("kernel", 96) -> Kernel<'a>;
        model l2_protocol("l2_protocol", 97) -> L2Protocol<'a>;
        model lacp("lacp", 98) -> Lacp<'a>;
        model link_tracking_groups("link_tracking_groups", 99) -> LinkTrackingGroups<'a>;
        model lldp("lldp", 100) -> Lldp<'a>;
        model load_balance("load_balance", 101) -> LoadBalance<'a>;
        model load_interval("load_interval", 102) -> LoadInterval<'a>;
        model local_users("local_users", 103) -> LocalUsers<'a>;
        model logging("logging", 104) -> Logging<'a>;
        model loopback_interfaces("loopback_interfaces", 105) -> LoopbackInterfaces<'a>;
        model mac_access_lists("mac_access_lists", 106) -> MacAccessLists<'a>;
        model mac_address_table("mac_address_table", 107) -> MacAddressTable<'a>;
        model mac_security("mac_security", 108) -> MacSecurity<'a>;
        model maintenance("maintenance", 109) -> Maintenance<'a>;
        model management_accounts("management_accounts", 110) -> ManagementAccounts<'a>;
        model management_api_gnmi("management_api_gnmi", 111) -> ManagementApiGnmi<'a>;
        model management_api_http("management_api_http", 112) -> ManagementApiHttp<'a>;
        model management_api_models("management_api_models", 113) -> ManagementApiModels<'a>;
        model management_console("management_console", 114) -> ManagementConsole<'a>;
        model management_cvx("management_cvx", 115) -> ManagementCvx<'a>;
        model management_defaults("management_defaults", 116) -> ManagementDefaults<'a>;
        model management_interfaces("management_interfaces", 117) -> ManagementInterfaces<'a>;
        model management_ldap("management_ldap", 118) -> ManagementLdap<'a>;
        model management_security("management_security", 119) -> ManagementSecurity<'a>;
        model management_ssh("management_ssh", 120) -> ManagementSsh<'a>;
        model management_tech_support("management_tech_support", 121) -> ManagementTechSupport<'a>;
        model match_list_input("match_list_input", 122) -> MatchListInput<'a>;
        model mcs_client("mcs_client", 123) -> McsClient<'a>;
        model metadata("metadata", 124) -> Metadata<'a>;
        model mlag_configuration("mlag_configuration", 125) -> MlagConfiguration<'a>;
        model monitor_connectivity("monitor_connectivity", 126) -> MonitorConnectivity<'a>;
        model monitor_layer1("monitor_layer1", 127) -> MonitorLayer1<'a>;
        model monitor_link_flap_policy("monitor_link_flap_policy", 128) -> MonitorLinkFlapPolicy<'a>;
        model monitor_loop_protection("monitor_loop_protection", 129) -> MonitorLoopProtection<'a>;
        model monitor_server_radius("monitor_server_radius", 130) -> MonitorServerRadius<'a>;
        model monitor_session_default_encapsulation_gre("monitor_session_default_encapsulation_gre", 131) -> MonitorSessionDefaultEncapsulationGre<'a>;
        model monitor_sessions("monitor_sessions", 132) -> MonitorSessions<'a>;
        model monitor_telemetry_influx("monitor_telemetry_influx", 133) -> MonitorTelemetryInflux<'a>;
        model monitor_telemetry_postcard_policy("monitor_telemetry_postcard_policy", 134) -> MonitorTelemetryPostcardPolicy<'a>;
        model monitor_twamp("monitor_twamp", 135) -> MonitorTwamp<'a>;
        model mpls("mpls", 136) -> Mpls<'a>;
        model ntp("ntp", 137) -> Ntp<'a>;
        model patch_panel("patch_panel", 138) -> PatchPanel<'a>;
        model peer_filters("peer_filters", 139) -> PeerFilters<'a>;
        model platform("platform", 140) -> Platform<'a>;
        model poe("poe", 141) -> Poe<'a>;
        model policy_maps("policy_maps", 142) -> PolicyMaps<'a>;
        model port_channel("port_channel", 143) -> PortChannel<'a>;
        model port_channel_interfaces("port_channel_interfaces", 144) -> PortChannelInterfaces<'a>;
        model prefix_lists("prefix_lists", 145) -> PrefixLists<'a>;
        model priority_flow_control("priority_flow_control", 146) -> PriorityFlowControl<'a>;
        scalar prompt("prompt", 147) -> &'a str;
        model ptp("ptp", 148) -> Ptp<'a>;
        model qos("qos", 149) -> Qos<'a>;
        model qos_profiles("qos_profiles", 150) -> QosProfiles<'a>;
        model queue_monitor_length("queue_monitor_length", 151) -> QueueMonitorLength<'a>;
        model queue_monitor_streaming("queue_monitor_streaming", 152) -> QueueMonitorStreaming<'a>;
        model radius_proxy("radius_proxy", 153) -> RadiusProxy<'a>;
        model radius_server("radius_server", 154) -> RadiusServer<'a>;
        scalar read_structured_config_from_file("read_structured_config_from_file", 155) -> bool;
        model redundancy("redundancy", 156) -> Redundancy<'a>;
        model roles("roles", 157) -> Roles<'a>;
        model route_maps("route_maps", 158) -> RouteMaps<'a>;
        model router_adaptive_virtual_topology("router_adaptive_virtual_topology", 159) -> RouterAdaptiveVirtualTopology<'a>;
        model router_bfd("router_bfd", 160) -> RouterBfd<'a>;
        model router_bgp("router_bgp", 161) -> RouterBgp<'a>;
        model router_general("router_general", 162) -> RouterGeneral<'a>;
        model router_igmp("router_igmp", 163) -> RouterIgmp<'a>;
        model router_internet_exit("router_internet_exit", 164) -> RouterInternetExit<'a>;
        model router_isis("router_isis", 165) -> RouterIsis<'a>;
        model router_l2_vpn("router_l2_vpn", 166) -> RouterL2Vpn<'a>;
        model router_msdp("router_msdp", 167) -> RouterMsdp<'a>;
        model router_multicast("router_multicast", 168) -> RouterMulticast<'a>;
        model router_ospf("router_ospf", 169) -> RouterOspf<'a>;
        model router_ospfv3("router_ospfv3", 170) -> RouterOspfv3<'a>;
        model router_path_selection("router_path_selection", 171) -> RouterPathSelection<'a>;
        model router_pim_sparse_mode("router_pim_sparse_mode", 172) -> RouterPimSparseMode<'a>;
        model router_rip("router_rip", 173) -> RouterRip<'a>;
        model router_segment_security("router_segment_security", 174) -> RouterSegmentSecurity<'a>;
        model router_service_insertion("router_service_insertion", 175) -> RouterServiceInsertion<'a>;
        model router_traffic_engineering("router_traffic_engineering", 176) -> RouterTrafficEngineering<'a>;
        model schedule("schedule", 177) -> Schedule<'a>;
        model service_routing_configuration_bgp("service_routing_configuration_bgp", 178) -> ServiceRoutingConfigurationBgp<'a>;
        scalar service_routing_protocols_model("service_routing_protocols_model", 179) -> &'a str;
        model service_unsupported_transceiver("service_unsupported_transceiver", 180) -> ServiceUnsupportedTransceiver<'a>;
        model sflow("sflow", 181) -> Sflow<'a>;
        model snmp_server("snmp_server", 182) -> SnmpServer<'a>;
        model spanning_tree("spanning_tree", 183) -> SpanningTree<'a>;
        model standard_access_lists("standard_access_lists", 184) -> StandardAccessLists<'a>;
        model static_routes("static_routes", 185) -> StaticRoutes<'a>;
        model stun("stun", 186) -> Stun<'a>;
        model switchport_default("switchport_default", 187) -> SwitchportDefault<'a>;
        scalar switchport_ethernet_llc_validation("switchport_ethernet_llc_validation", 188) -> bool;
        model switchport_port_security("switchport_port_security", 189) -> SwitchportPortSecurity<'a>;
        scalar switchport_vlan_tag_validation("switchport_vlan_tag_validation", 190) -> bool;
        model sync_e("sync_e", 191) -> SyncE<'a>;
        model system("system", 192) -> System<'a>;
        model tacacs_servers("tacacs_servers", 193) -> TacacsServers<'a>;
        model tap_aggregation("tap_aggregation", 194) -> TapAggregation<'a>;
        model tcam_profile("tcam_profile", 195) -> TcamProfile<'a>;
        model terminal("terminal", 196) -> Terminal<'a>;
        model trackers("trackers", 197) -> Trackers<'a>;
        model traffic_policies("traffic_policies", 198) -> TrafficPolicies<'a>;
        model transceiver("transceiver", 199) -> Transceiver<'a>;
        scalar transceiver_qsfp_default_mode_4x10("transceiver_qsfp_default_mode_4x10", 200) -> bool;
        model tunnel_interfaces("tunnel_interfaces", 201) -> TunnelInterfaces<'a>;
        model virtual_source_nat_vrfs("virtual_source_nat_vrfs", 202) -> VirtualSourceNatVrfs<'a>;
        model vlan_interfaces("vlan_interfaces", 203) -> VlanInterfaces<'a>;
        model vlan_internal_order("vlan_internal_order", 204) -> VlanInternalOrder<'a>;
        model vlans("vlans", 205) -> Vlans<'a>;
        model vmtracer_sessions("vmtracer_sessions", 206) -> VmtracerSessions<'a>;
        model vrfs("vrfs", 207) -> Vrfs<'a>;
        model vxlan_interface("vxlan_interface", 208) -> VxlanInterface<'a>;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AaaAccounting {
        model exec("exec", 0) -> aaa_accounting::Exec<'a>;
        model system("system", 1) -> aaa_accounting::System<'a>;
        model dot1x("dot1x", 2) -> aaa_accounting::Dot1x<'a>;
        model commands("commands", 3) -> aaa_accounting::Commands<'a>;
    }
}

pub mod aaa_accounting;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AaaAuthentication {
        model login("login", 0) -> aaa_authentication::Login<'a>;
        model enable("enable", 1) -> aaa_authentication::Enable<'a>;
        model dot1x("dot1x", 2) -> aaa_authentication::Dot1x<'a>;
        model policies("policies", 3) -> aaa_authentication::Policies<'a>;
    }
}

pub mod aaa_authentication;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AaaAuthorization {
        model policy("policy", 0) -> aaa_authorization::Policy<'a>;
        model exec("exec", 1) -> aaa_authorization::Exec<'a>;
        scalar config_commands("config_commands", 2) -> bool;
        scalar serial_console("serial_console", 3) -> bool;
        model dynamic("dynamic", 4) -> aaa_authorization::Dynamic<'a>;
        model commands("commands", 5) -> aaa_authorization::Commands<'a>;
    }
}

pub mod aaa_authorization;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AaaRoot {
        scalar disabled("disabled", 0) -> bool;
        model secret("secret", 1) -> aaa_root::Secret<'a>;
    }
}

pub mod aaa_root;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AaaServerGroups {
        model item (0) -> aaa_server_groups::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod aaa_server_groups;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AccessLists {
        model item (0) -> access_lists::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod access_lists;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AddressLocking {
        model dhcp_servers_ipv4("dhcp_servers_ipv4", 0) -> address_locking::DhcpServersIpv4<'a>;
        model dhcp_server_interfaces("dhcp_server_interfaces", 1) -> address_locking::DhcpServerInterfaces<'a>;
        scalar disabled("disabled", 2) -> bool;
        model leases("leases", 3) -> address_locking::Leases<'a>;
        scalar local_interface("local_interface", 4) -> &'a str;
        model locked_address("locked_address", 5) -> address_locking::LockedAddress<'a>;
    }
}

pub mod address_locking;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Agents {
        model item (0) -> agents::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod agents;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ApplicationTrafficRecognition {
        model categories("categories", 0) -> application_traffic_recognition::Categories<'a>;
        model field_sets("field_sets", 1) -> application_traffic_recognition::FieldSets<'a>;
        model applications("applications", 2) -> application_traffic_recognition::Applications<'a>;
        model application_profiles("application_profiles", 3) -> application_traffic_recognition::ApplicationProfiles<'a>;
    }
}

pub mod application_traffic_recognition;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Arp {
        model persistent("persistent", 0) -> arp::Persistent<'a>;
        model aging("aging", 1) -> arp::Aging<'a>;
        model static_entries("static_entries", 2) -> arp::StaticEntries<'a>;
    }
}

pub mod arp;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AsPath {
        scalar regex_mode("regex_mode", 0) -> &'a str;
        model access_lists("access_lists", 1) -> as_path::AccessLists<'a>;
    }
}

pub mod as_path;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Banners {
        scalar login("login", 0) -> &'a str;
        scalar motd("motd", 1) -> &'a str;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct BgpGroups {
        model item (0) -> bgp_groups::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod bgp_groups;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Boot {
        model secret("secret", 0) -> boot::Secret<'a>;
    }
}

pub mod boot;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Cfm {
        scalar continuity_check_loc_state_action_disable_interface_routing("continuity_check_loc_state_action_disable_interface_routing", 0) -> bool;
        model domains("domains", 1) -> cfm::Domains<'a>;
        model measurement_loss("measurement_loss", 2) -> cfm::MeasurementLoss<'a>;
        model profiles("profiles", 3) -> cfm::Profiles<'a>;
    }
}

pub mod cfm;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ClassMaps {
        model pbr("pbr", 0) -> class_maps::Pbr<'a>;
        model qos("qos", 1) -> class_maps::Qos<'a>;
    }
}

pub mod class_maps;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Clock {
        scalar timezone("timezone", 0) -> &'a str;
    }
}

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct CustomTemplates {
        scalar item (0) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Cvx {
        scalar shutdown("shutdown", 0) -> bool;
        model peer_hosts("peer_hosts", 1) -> cvx::PeerHosts<'a>;
        model services("services", 2) -> cvx::Services<'a>;
    }
}

pub mod cvx;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DaemonTerminattr {
        model cvaddrs("cvaddrs", 0) -> daemon_terminattr::Cvaddrs<'a>;
        model clusters("clusters", 1) -> daemon_terminattr::Clusters<'a>;
        model cvauth("cvauth", 2) -> daemon_terminattr::Cvauth<'a>;
        scalar cvobscurekeyfile("cvobscurekeyfile", 3) -> bool;
        scalar cvproxy("cvproxy", 4) -> &'a str;
        scalar cvsourceip("cvsourceip", 5) -> &'a str;
        scalar cvsourceintf("cvsourceintf", 6) -> &'a str;
        scalar cvvrf("cvvrf", 7) -> &'a str;
        scalar cvgnmi("cvgnmi", 8) -> bool;
        scalar disable_aaa("disable_aaa", 9) -> bool;
        scalar grpcaddr("grpcaddr", 10) -> &'a str;
        scalar grpcreadonly("grpcreadonly", 11) -> bool;
        scalar ingestexclude("ingestexclude", 12) -> &'a str;
        scalar smashexcludes("smashexcludes", 13) -> &'a str;
        scalar sysdbexcludes("sysdbexcludes", 14) -> &'a str;
        scalar taillogs("taillogs", 15) -> &'a str;
        scalar ecodhcpaddr("ecodhcpaddr", 16) -> &'a str;
        scalar ipfix("ipfix", 17) -> bool;
        scalar ipfixaddr("ipfixaddr", 18) -> &'a str;
        scalar sflow("sflow", 19) -> bool;
        scalar sflowaddr("sflowaddr", 20) -> &'a str;
        scalar cvconfig("cvconfig", 21) -> bool;
        scalar cv_loss_timeout("cv_loss_timeout", 22) -> i64;
        model cvtargetconfigs("cvtargetconfigs", 23) -> daemon_terminattr::Cvtargetconfigs<'a>;
        scalar flowdns("flowdns", 24) -> bool;
        model custom_cv_options("custom_cv_options", 25) -> daemon_terminattr::CustomCvOptions<'a>;
    }
}

pub mod daemon_terminattr;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Daemons {
        model item (0) -> daemons::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod daemons;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DhcpRelay {
        model servers("servers", 0) -> dhcp_relay::Servers<'a>;
        scalar tunnel_requests_disabled("tunnel_requests_disabled", 1) -> bool;
        scalar mlag_peerlink_requests_disabled("mlag_peerlink_requests_disabled", 2) -> bool;
        model client_requests("client_requests", 3) -> dhcp_relay::ClientRequests<'a>;
        scalar reply_source_address_validation("reply_source_address_validation", 4) -> bool;
    }
}

pub mod dhcp_relay;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DhcpServers {
        model item (0) -> dhcp_servers::Item<'a>;
        primary_key_fields: [1];
    }
}

pub mod dhcp_servers;

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DomainList {
        scalar item (0) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Dot1x {
        scalar system_auth_control("system_auth_control", 0) -> bool;
        scalar protocol_lldp_bypass("protocol_lldp_bypass", 1) -> bool;
        scalar protocol_bpdu_bypass("protocol_bpdu_bypass", 2) -> bool;
        scalar dynamic_authorization("dynamic_authorization", 3) -> bool;
        scalar statistics_packets_dropped("statistics_packets_dropped", 4) -> bool;
        model mac_based_authentication("mac_based_authentication", 5) -> dot1x::MacBasedAuthentication<'a>;
        model radius_av_pair_username_format("radius_av_pair_username_format", 6) -> dot1x::RadiusAvPairUsernameFormat<'a>;
        model radius_av_pair("radius_av_pair", 7) -> dot1x::RadiusAvPair<'a>;
        model aaa("aaa", 8) -> dot1x::Aaa<'a>;
        model captive_portal("captive_portal", 9) -> dot1x::CaptivePortal<'a>;
        model supplicant("supplicant", 10) -> dot1x::Supplicant<'a>;
        model vlan_assignment_groups("vlan_assignment_groups", 11) -> dot1x::VlanAssignmentGroups<'a>;
        model eapol("eapol", 12) -> dot1x::Eapol<'a>;
    }
}

pub mod dot1x;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DpsInterfaces {
        model item (0) -> dps_interfaces::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod dps_interfaces;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DynamicPrefixLists {
        model item (0) -> dynamic_prefix_lists::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod dynamic_prefix_lists;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct EnablePassword {
        scalar disabled("disabled", 0) -> bool;
        scalar hash_algorithm("hash_algorithm", 1) -> &'a str;
        scalar key("key", 2) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct EnvironmentFanSpeed {
        scalar minimum("minimum", 0) -> i64;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct EosCliConfigGenConfiguration {
        scalar enable("enable", 0) -> bool;
        scalar hide_passwords("hide_passwords", 1) -> bool;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct EosCliConfigGenDocumentation {
        scalar enable("enable", 0) -> bool;
        scalar hide_passwords("hide_passwords", 1) -> bool;
        scalar toc("toc", 2) -> bool;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct EosConfigFuture {
        scalar always_render_ip_routing_separator("always_render_ip_routing_separator", 0) -> bool;
        scalar render_combined_separator_for_ipv6_hardware_and_unicast_routing("render_combined_separator_for_ipv6_hardware_and_unicast_routing", 1) -> bool;
        scalar new_ip_radius_cli_order("new_ip_radius_cli_order", 2) -> bool;
        scalar new_ip_tacacs_cli_order("new_ip_tacacs_cli_order", 3) -> bool;
        scalar only_render_mpls_rsvp_with_settings("only_render_mpls_rsvp_with_settings", 4) -> bool;
        scalar render_monitor_layer1_without_enabled("render_monitor_layer1_without_enabled", 5) -> bool;
        scalar render_spanning_tree_portfast_edge("render_spanning_tree_portfast_edge", 6) -> bool;
        scalar only_render_separator_with_boot_secret_key("only_render_separator_with_boot_secret_key", 7) -> bool;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Errdisable {
        model detect("detect", 0) -> errdisable::Detect<'a>;
        model detect_cause("detect_cause", 1) -> errdisable::DetectCause<'a>;
        model recovery("recovery", 2) -> errdisable::Recovery<'a>;
        model recovery_cause("recovery_cause", 3) -> errdisable::RecoveryCause<'a>;
        scalar recovery_interval("recovery_interval", 4) -> i64;
    }
}

pub mod errdisable;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct EthernetInterfaces {
        model item (0) -> ethernet_interfaces::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod ethernet_interfaces;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct EventHandlers {
        model item (0) -> event_handlers::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod event_handlers;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct EventMonitor {
        scalar enabled("enabled", 0) -> bool;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct FlowTracking {
        model sampled("sampled", 0) -> flow_tracking::Sampled<'a>;
        model hardware("hardware", 1) -> flow_tracking::Hardware<'a>;
        model mirror_on_drop("mirror_on_drop", 2) -> flow_tracking::MirrorOnDrop<'a>;
    }
}

pub mod flow_tracking;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Hardware {
        model access_list("access_list", 0) -> hardware::AccessList<'a>;
        model speed_groups("speed_groups", 1) -> hardware::SpeedGroups<'a>;
        model port_groups("port_groups", 2) -> hardware::PortGroups<'a>;
    }
}

pub mod hardware;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct HardwareCounters {
        model features("features", 0) -> hardware_counters::Features<'a>;
    }
}

pub mod hardware_counters;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct InterfaceDefaults {
        model ethernet("ethernet", 0) -> interface_defaults::Ethernet<'a>;
        scalar mtu("mtu", 1) -> i64;
    }
}

pub mod interface_defaults;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct InterfaceGroups {
        model item (0) -> interface_groups::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod interface_groups;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct InterfaceProfiles {
        model item (0) -> interface_profiles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod interface_profiles;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpAccessLists {
        model item (0) -> ip_access_lists::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod ip_access_lists;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpCommunityLists {
        model item (0) -> ip_community_lists::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod ip_community_lists;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpDhcpRelay {
        scalar always_on("always_on", 0) -> bool;
        scalar all_subnets("all_subnets", 1) -> bool;
        scalar information_option("information_option", 2) -> bool;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpDhcpSnooping {
        scalar enabled("enabled", 0) -> bool;
        scalar bridging("bridging", 1) -> bool;
        model information_option("information_option", 2) -> ip_dhcp_snooping::InformationOption<'a>;
        scalar vlan("vlan", 3) -> &'a str;
    }
}

pub mod ip_dhcp_snooping;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpDomainLookup {
        model source_interfaces("source_interfaces", 0) -> ip_domain_lookup::SourceInterfaces<'a>;
    }
}

pub mod ip_domain_lookup;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpExtcommunityLists {
        model item (0) -> ip_extcommunity_lists::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod ip_extcommunity_lists;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpExtcommunityListsRegexp {
        model item (0) -> ip_extcommunity_lists_regexp::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod ip_extcommunity_lists_regexp;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpFtpClient {
        scalar source_interface("source_interface", 0) -> &'a str;
        model vrfs("vrfs", 1) -> ip_ftp_client::Vrfs<'a>;
    }
}

pub mod ip_ftp_client;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpHardware {
        model fib("fib", 0) -> ip_hardware::Fib<'a>;
    }
}

pub mod ip_hardware;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpHosts {
        model item (0) -> ip_hosts::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod ip_hosts;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpHttpClient {
        scalar source_interface("source_interface", 0) -> &'a str;
        model vrfs("vrfs", 1) -> ip_http_client::Vrfs<'a>;
    }
}

pub mod ip_http_client;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpIgmpSnooping {
        scalar globally_enabled("globally_enabled", 0) -> bool;
        scalar robustness_variable("robustness_variable", 1) -> i64;
        scalar restart_query_interval("restart_query_interval", 2) -> i64;
        scalar interface_restart_query("interface_restart_query", 3) -> i64;
        scalar fast_leave("fast_leave", 4) -> bool;
        model querier("querier", 5) -> ip_igmp_snooping::Querier<'a>;
        scalar proxy("proxy", 6) -> bool;
        model vlans("vlans", 7) -> ip_igmp_snooping::Vlans<'a>;
    }
}

pub mod ip_igmp_snooping;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpLargeCommunityLists {
        model item (0) -> ip_large_community_lists::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod ip_large_community_lists;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpNameServer {
        model vrfs("vrfs", 0) -> ip_name_server::Vrfs<'a>;
    }
}

pub mod ip_name_server;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpNameServerGroups {
        model item (0) -> ip_name_server_groups::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod ip_name_server_groups;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpNat {
        scalar kernel_buffer_size("kernel_buffer_size", 0) -> i64;
        model profiles("profiles", 1) -> ip_nat::Profiles<'a>;
        model pools("pools", 2) -> ip_nat::Pools<'a>;
        model synchronization("synchronization", 3) -> ip_nat::Synchronization<'a>;
        model translation("translation", 4) -> ip_nat::Translation<'a>;
    }
}

pub mod ip_nat;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpRadius {
        scalar source_interface("source_interface", 0) -> &'a str;
        model vrfs("vrfs", 1) -> ip_radius::Vrfs<'a>;
    }
}

pub mod ip_radius;

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpRadiusSourceInterfaces {
        model item (0) -> ip_radius_source_interfaces::Item<'a>;
    }
}

pub mod ip_radius_source_interfaces;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpSecurity {
        model ike_policies("ike_policies", 0) -> ip_security::IkePolicies<'a>;
        model sa_policies("sa_policies", 1) -> ip_security::SaPolicies<'a>;
        model profiles("profiles", 2) -> ip_security::Profiles<'a>;
        model key_controller("key_controller", 3) -> ip_security::KeyController<'a>;
        scalar hardware_encryption_disabled("hardware_encryption_disabled", 4) -> bool;
        scalar connection_tx_interface_match_source_ip("connection_tx_interface_match_source_ip", 5) -> bool;
    }
}

pub mod ip_security;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpSoftwareForwarding {
        model mtu("mtu", 0) -> ip_software_forwarding::Mtu<'a>;
    }
}

pub mod ip_software_forwarding;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpSshClient {
        scalar source_interface("source_interface", 0) -> &'a str;
        model vrfs("vrfs", 1) -> ip_ssh_client::Vrfs<'a>;
    }
}

pub mod ip_ssh_client;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpTacacs {
        scalar source_interface("source_interface", 0) -> &'a str;
        model vrfs("vrfs", 1) -> ip_tacacs::Vrfs<'a>;
    }
}

pub mod ip_tacacs;

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpTacacsSourceInterfaces {
        model item (0) -> ip_tacacs_source_interfaces::Item<'a>;
    }
}

pub mod ip_tacacs_source_interfaces;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpTelnetClient {
        scalar source_interface("source_interface", 0) -> &'a str;
        model vrfs("vrfs", 1) -> ip_telnet_client::Vrfs<'a>;
    }
}

pub mod ip_telnet_client;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpTftpClient {
        scalar source_interface("source_interface", 0) -> &'a str;
        model vrfs("vrfs", 1) -> ip_tftp_client::Vrfs<'a>;
    }
}

pub mod ip_tftp_client;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Ipv6AccessLists {
        model item (0) -> ipv6_access_lists::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod ipv6_access_lists;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Ipv6DhcpRelay {
        scalar always_on("always_on", 0) -> bool;
        scalar all_subnets("all_subnets", 1) -> bool;
        model option("option", 2) -> ipv6_dhcp_relay::Option<'a>;
    }
}

pub mod ipv6_dhcp_relay;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Ipv6Hardware {
        model fib("fib", 0) -> ipv6_hardware::Fib<'a>;
    }
}

pub mod ipv6_hardware;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Ipv6Neighbor {
        model static_entries("static_entries", 0) -> ipv6_neighbor::StaticEntries<'a>;
        model persistent("persistent", 1) -> ipv6_neighbor::Persistent<'a>;
    }
}

pub mod ipv6_neighbor;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Ipv6PrefixLists {
        model item (0) -> ipv6_prefix_lists::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod ipv6_prefix_lists;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Ipv6RouterOspf {
        model process_ids("process_ids", 0) -> ipv6_router_ospf::ProcessIds<'a>;
    }
}

pub mod ipv6_router_ospf;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Ipv6StandardAccessLists {
        model item (0) -> ipv6_standard_access_lists::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod ipv6_standard_access_lists;

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Ipv6StaticRoutes {
        model item (0) -> ipv6_static_routes::Item<'a>;
    }
}

pub mod ipv6_static_routes;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Kernel {
        scalar software_forwarding_ecmp("software_forwarding_ecmp", 0) -> bool;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct L2Protocol {
        model forwarding_profiles("forwarding_profiles", 0) -> l2_protocol::ForwardingProfiles<'a>;
    }
}

pub mod l2_protocol;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Lacp {
        model port_id("port_id", 0) -> lacp::PortId<'a>;
        model rate_limit("rate_limit", 1) -> lacp::RateLimit<'a>;
        scalar system_priority("system_priority", 2) -> i64;
    }
}

pub mod lacp;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct LinkTrackingGroups {
        model item (0) -> link_tracking_groups::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod link_tracking_groups;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Lldp {
        scalar timer("timer", 0) -> i64;
        scalar timer_reinitialization("timer_reinitialization", 1) -> i64;
        scalar holdtime("holdtime", 2) -> i64;
        scalar management_address("management_address", 3) -> &'a str;
        scalar vrf("vrf", 4) -> &'a str;
        scalar receive_packet_tagged_drop("receive_packet_tagged_drop", 5) -> bool;
        model tlvs("tlvs", 6) -> lldp::Tlvs<'a>;
        scalar run("run", 7) -> bool;
    }
}

pub mod lldp;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct LoadBalance {
        model policies("policies", 0) -> load_balance::Policies<'a>;
        model cluster("cluster", 1) -> load_balance::Cluster<'a>;
    }
}

pub mod load_balance;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct LoadInterval {
        scalar default("default", 0) -> i64;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct LocalUsers {
        model item (0) -> local_users::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod local_users;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Logging {
        scalar console("console", 0) -> &'a str;
        scalar monitor("monitor", 1) -> &'a str;
        model buffered("buffered", 2) -> logging::Buffered<'a>;
        scalar repeat_messages("repeat_messages", 3) -> bool;
        scalar trap("trap", 4) -> &'a str;
        model synchronous("synchronous", 5) -> logging::Synchronous<'a>;
        model format("format", 6) -> logging::Format<'a>;
        scalar facility("facility", 7) -> &'a str;
        scalar source_interface("source_interface", 8) -> &'a str;
        scalar local_interface("local_interface", 9) -> &'a str;
        model vrfs("vrfs", 10) -> logging::Vrfs<'a>;
        model policy("policy", 11) -> logging::Policy<'a>;
        model event("event", 12) -> logging::Event<'a>;
        model level("level", 13) -> logging::Level<'a>;
    }
}

pub mod logging;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct LoopbackInterfaces {
        model item (0) -> loopback_interfaces::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod loopback_interfaces;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MacAccessLists {
        model item (0) -> mac_access_lists::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod mac_access_lists;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MacAddressTable {
        scalar aging_time("aging_time", 0) -> i64;
        model notification_host_flap("notification_host_flap", 1) -> mac_address_table::NotificationHostFlap<'a>;
        model static_entries("static_entries", 2) -> mac_address_table::StaticEntries<'a>;
    }
}

pub mod mac_address_table;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MacSecurity {
        model license("license", 0) -> mac_security::License<'a>;
        scalar fips_restrictions("fips_restrictions", 1) -> bool;
        model profiles("profiles", 2) -> mac_security::Profiles<'a>;
    }
}

pub mod mac_security;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Maintenance {
        scalar default_interface_profile("default_interface_profile", 0) -> &'a str;
        scalar default_bgp_profile("default_bgp_profile", 1) -> &'a str;
        scalar default_unit_profile("default_unit_profile", 2) -> &'a str;
        model interface_profiles("interface_profiles", 3) -> maintenance::InterfaceProfiles<'a>;
        model bgp_profiles("bgp_profiles", 4) -> maintenance::BgpProfiles<'a>;
        model unit_profiles("unit_profiles", 5) -> maintenance::UnitProfiles<'a>;
        model units("units", 6) -> maintenance::Units<'a>;
    }
}

pub mod maintenance;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ManagementAccounts {
        model password("password", 0) -> management_accounts::Password<'a>;
    }
}

pub mod management_accounts;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ManagementApiGnmi {
        scalar provider("provider", 0) -> &'a str;
        model transport("transport", 1) -> management_api_gnmi::Transport<'a>;
    }
}

pub mod management_api_gnmi;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ManagementApiHttp {
        scalar enable_http("enable_http", 0) -> bool;
        scalar enable_https("enable_https", 1) -> bool;
        scalar enable_unix("enable_unix", 2) -> bool;
        scalar https_ssl_profile("https_ssl_profile", 3) -> &'a str;
        scalar default_services("default_services", 4) -> bool;
        scalar session_timeout("session_timeout", 5) -> i64;
        model enable_vrfs("enable_vrfs", 6) -> management_api_http::EnableVrfs<'a>;
        model protocol_https_certificate("protocol_https_certificate", 7) -> management_api_http::ProtocolHttpsCertificate<'a>;
    }
}

pub mod management_api_http;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ManagementApiModels {
        model provider("provider", 0) -> management_api_models::Provider<'a>;
    }
}

pub mod management_api_models;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ManagementConsole {
        scalar idle_timeout("idle_timeout", 0) -> i64;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ManagementCvx {
        scalar shutdown("shutdown", 0) -> bool;
        model server_hosts("server_hosts", 1) -> management_cvx::ServerHosts<'a>;
        scalar source_interface("source_interface", 2) -> &'a str;
        scalar vrf("vrf", 3) -> &'a str;
    }
}

pub mod management_cvx;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ManagementDefaults {
        model secret("secret", 0) -> management_defaults::Secret<'a>;
    }
}

pub mod management_defaults;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ManagementInterfaces {
        model item (0) -> management_interfaces::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod management_interfaces;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ManagementLdap {
        model server_defaults("server_defaults", 0) -> management_ldap::ServerDefaults<'a>;
        model server_hosts("server_hosts", 1) -> management_ldap::ServerHosts<'a>;
        model group_policies("group_policies", 2) -> management_ldap::GroupPolicies<'a>;
    }
}

pub mod management_ldap;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ManagementSecurity {
        model auto_certificate("auto_certificate", 0) -> management_security::AutoCertificate<'a>;
        model entropy_sources("entropy_sources", 1) -> management_security::EntropySources<'a>;
        model signature_verification("signature_verification", 2) -> management_security::SignatureVerification<'a>;
        model password("password", 3) -> management_security::Password<'a>;
        model ssl_profiles("ssl_profiles", 4) -> management_security::SslProfiles<'a>;
        model shared_secret_profiles("shared_secret_profiles", 5) -> management_security::SharedSecretProfiles<'a>;
    }
}

pub mod management_security;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ManagementSsh {
        model authentication("authentication", 0) -> management_ssh::Authentication<'a>;
        scalar ip_access_group_in("ip_access_group_in", 1) -> &'a str;
        scalar ipv6_access_group_in("ipv6_access_group_in", 2) -> &'a str;
        scalar idle_timeout("idle_timeout", 3) -> i64;
        model cipher("cipher", 4) -> management_ssh::Cipher<'a>;
        model key_exchange("key_exchange", 5) -> management_ssh::KeyExchange<'a>;
        model mac("mac", 6) -> management_ssh::Mac<'a>;
        scalar fips_restrictions("fips_restrictions", 7) -> bool;
        model hostkey("hostkey", 8) -> management_ssh::Hostkey<'a>;
        scalar enable("enable", 9) -> bool;
        model connection("connection", 10) -> management_ssh::Connection<'a>;
        model vrfs("vrfs", 11) -> management_ssh::Vrfs<'a>;
        scalar log_level("log_level", 12) -> &'a str;
        model client_alive("client_alive", 13) -> management_ssh::ClientAlive<'a>;
    }
}

pub mod management_ssh;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ManagementTechSupport {
        model policy_show_tech_support("policy_show_tech_support", 0) -> management_tech_support::PolicyShowTechSupport<'a>;
    }
}

pub mod management_tech_support;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MatchListInput {
        model prefix_ipv4("prefix_ipv4", 0) -> match_list_input::PrefixIpv4<'a>;
        model prefix_ipv6("prefix_ipv6", 1) -> match_list_input::PrefixIpv6<'a>;
        model string("string", 2) -> match_list_input::String<'a>;
    }
}

pub mod match_list_input;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct McsClient {
        scalar shutdown("shutdown", 0) -> bool;
        model cvx_secondary("cvx_secondary", 1) -> mcs_client::CvxSecondary<'a>;
    }
}

pub mod mcs_client;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Metadata {
        scalar is_deployed("is_deployed", 0) -> bool;
        scalar platform("platform", 1) -> &'a str;
        scalar system_mac_address("system_mac_address", 2) -> &'a str;
        scalar rack("rack", 3) -> &'a str;
        scalar pod_name("pod_name", 4) -> &'a str;
        scalar dc_name("dc_name", 5) -> &'a str;
        scalar fabric_name("fabric_name", 6) -> &'a str;
        scalar serial_number("serial_number", 7) -> &'a str;
        model validate_hardware("validate_hardware", 8) -> metadata::ValidateHardware<'a>;
        model cv_tags("cv_tags", 9) -> metadata::CvTags<'a>;
        scalar cv_use_static_config_manifest("cv_use_static_config_manifest", 10) -> bool;
        model cv_pathfinder("cv_pathfinder", 11) -> metadata::CvPathfinder<'a>;
        model digital_twin("digital_twin", 12) -> metadata::DigitalTwin<'a>;
        scalar validate_no_errors_period("validate_no_errors_period", 13) -> i64;
        scalar exclude_as_extra_fabric_validation_target("exclude_as_extra_fabric_validation_target", 14) -> bool;
        model interfaces("interfaces", 15) -> metadata::Interfaces<'a>;
        model bgp("bgp", 16) -> metadata::Bgp<'a>;
    }
}

pub mod metadata;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MlagConfiguration {
        scalar domain_id("domain_id", 0) -> &'a str;
        scalar heartbeat_interval("heartbeat_interval", 1) -> i64;
        scalar local_interface("local_interface", 2) -> &'a str;
        scalar peer_address("peer_address", 3) -> &'a str;
        model peer_address_heartbeat("peer_address_heartbeat", 4) -> mlag_configuration::PeerAddressHeartbeat<'a>;
        scalar dual_primary_detection_delay("dual_primary_detection_delay", 5) -> i64;
        scalar dual_primary_recovery_delay_mlag("dual_primary_recovery_delay_mlag", 6) -> i64;
        scalar dual_primary_recovery_delay_non_mlag("dual_primary_recovery_delay_non_mlag", 7) -> i64;
        scalar peer_link("peer_link", 8) -> &'a str;
        scalar reload_delay_mlag("reload_delay_mlag", 9) -> &'a str;
        scalar reload_delay_non_mlag("reload_delay_non_mlag", 10) -> &'a str;
    }
}

pub mod mlag_configuration;

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

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MonitorLayer1 {
        scalar enabled("enabled", 0) -> bool;
        scalar logging_mac_fault("logging_mac_fault", 1) -> bool;
        model logging_transceiver("logging_transceiver", 2) -> monitor_layer1::LoggingTransceiver<'a>;
    }
}

pub mod monitor_layer1;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MonitorLinkFlapPolicy {
        model damping_profiles("damping_profiles", 0) -> monitor_link_flap_policy::DampingProfiles<'a>;
        model max_flap_profiles("max_flap_profiles", 1) -> monitor_link_flap_policy::MaxFlapProfiles<'a>;
        model default_profiles("default_profiles", 2) -> monitor_link_flap_policy::DefaultProfiles<'a>;
    }
}

pub mod monitor_link_flap_policy;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MonitorLoopProtection {
        scalar enabled("enabled", 0) -> bool;
        scalar disabled_time("disabled_time", 1) -> i64;
        scalar protect_vlan("protect_vlan", 2) -> &'a str;
        scalar rate_limit("rate_limit", 3) -> i64;
        scalar transmit_interval("transmit_interval", 4) -> i64;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MonitorServerRadius {
        scalar service_dot1x("service_dot1x", 0) -> bool;
        model probe("probe", 1) -> monitor_server_radius::Probe<'a>;
    }
}

pub mod monitor_server_radius;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MonitorSessionDefaultEncapsulationGre {
        scalar payload("payload", 0) -> &'a str;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MonitorSessions {
        model item (0) -> monitor_sessions::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod monitor_sessions;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MonitorTelemetryInflux {
        scalar vrf("vrf", 0) -> &'a str;
        model destinations("destinations", 1) -> monitor_telemetry_influx::Destinations<'a>;
        scalar source_group_standard_disabled("source_group_standard_disabled", 2) -> bool;
        model source_sockets("source_sockets", 3) -> monitor_telemetry_influx::SourceSockets<'a>;
        model tags("tags", 4) -> monitor_telemetry_influx::Tags<'a>;
    }
}

pub mod monitor_telemetry_influx;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MonitorTelemetryPostcardPolicy {
        scalar disabled("disabled", 0) -> bool;
        model ingress("ingress", 1) -> monitor_telemetry_postcard_policy::Ingress<'a>;
        model marker_vxlan("marker_vxlan", 2) -> monitor_telemetry_postcard_policy::MarkerVxlan<'a>;
        model profiles("profiles", 3) -> monitor_telemetry_postcard_policy::Profiles<'a>;
        model sample_policies("sample_policies", 4) -> monitor_telemetry_postcard_policy::SamplePolicies<'a>;
    }
}

pub mod monitor_telemetry_postcard_policy;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MonitorTwamp {
        model twamp_light("twamp_light", 0) -> monitor_twamp::TwampLight<'a>;
    }
}

pub mod monitor_twamp;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Mpls {
        scalar ip("ip", 0) -> bool;
        model ldp("ldp", 1) -> mpls::Ldp<'a>;
        model icmp("icmp", 2) -> mpls::Icmp<'a>;
        model rsvp("rsvp", 3) -> mpls::Rsvp<'a>;
        model label_ranges("label_ranges", 4) -> mpls::LabelRanges<'a>;
        model tunnel("tunnel", 5) -> mpls::Tunnel<'a>;
    }
}

pub mod mpls;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Ntp {
        model local_interface("local_interface", 0) -> ntp::LocalInterface<'a>;
        model servers("servers", 1) -> ntp::Servers<'a>;
        scalar vrf("vrf", 2) -> &'a str;
        scalar authenticate("authenticate", 3) -> bool;
        scalar authenticate_servers_only("authenticate_servers_only", 4) -> bool;
        model authentication_keys("authentication_keys", 5) -> ntp::AuthenticationKeys<'a>;
        scalar trusted_keys("trusted_keys", 6) -> &'a str;
        model serve("serve", 7) -> ntp::Serve<'a>;
    }
}

pub mod ntp;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct PatchPanel {
        model connector("connector", 0) -> patch_panel::Connector<'a>;
        model patches("patches", 1) -> patch_panel::Patches<'a>;
    }
}

pub mod patch_panel;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct PeerFilters {
        model item (0) -> peer_filters::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod peer_filters;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Platform {
        model trident("trident", 0) -> platform::Trident<'a>;
        model sand("sand", 1) -> platform::Sand<'a>;
        model sfe("sfe", 2) -> platform::Sfe<'a>;
        model fap("fap", 3) -> platform::Fap<'a>;
    }
}

pub mod platform;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Poe {
        model reboot("reboot", 0) -> poe::Reboot<'a>;
        model interface_shutdown("interface_shutdown", 1) -> poe::InterfaceShutdown<'a>;
    }
}

pub mod poe;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct PolicyMaps {
        model pbr("pbr", 0) -> policy_maps::Pbr<'a>;
        model qos("qos", 1) -> policy_maps::Qos<'a>;
        model copp_system_policy("copp_system_policy", 2) -> policy_maps::CoppSystemPolicy<'a>;
    }
}

pub mod policy_maps;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct PortChannel {
        model load_balance_trident_udf("load_balance_trident_udf", 0) -> port_channel::LoadBalanceTridentUdf<'a>;
        scalar load_balance_sand_profile("load_balance_sand_profile", 1) -> &'a str;
    }
}

pub mod port_channel;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct PortChannelInterfaces {
        model item (0) -> port_channel_interfaces::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod port_channel_interfaces;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct PrefixLists {
        model item (0) -> prefix_lists::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod prefix_lists;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct PriorityFlowControl {
        scalar all_off("all_off", 0) -> bool;
        model watchdog("watchdog", 1) -> priority_flow_control::Watchdog<'a>;
    }
}

pub mod priority_flow_control;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Ptp {
        scalar mode("mode", 0) -> &'a str;
        scalar profile("profile", 1) -> &'a str;
        scalar mode_one_step("mode_one_step", 2) -> bool;
        scalar forward_unicast("forward_unicast", 3) -> bool;
        scalar clock_identity("clock_identity", 4) -> &'a str;
        model source("source", 5) -> ptp::Source<'a>;
        scalar priority1("priority1", 6) -> i64;
        scalar priority2("priority2", 7) -> i64;
        scalar ttl("ttl", 8) -> i64;
        scalar domain("domain", 9) -> i64;
        scalar hold_ptp_time("hold_ptp_time", 10) -> i64;
        model message_type("message_type", 11) -> ptp::MessageType<'a>;
        model monitor("monitor", 12) -> ptp::Monitor<'a>;
        model free_running("free_running", 13) -> ptp::FreeRunning<'a>;
        scalar forward_v1("forward_v1", 14) -> bool;
    }
}

pub mod ptp;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Qos {
        model map("map", 0) -> qos::Map<'a>;
        scalar rewrite_dscp("rewrite_dscp", 1) -> bool;
        model random_detect("random_detect", 2) -> qos::RandomDetect<'a>;
        model tx_queue("tx_queue", 3) -> qos::TxQueue<'a>;
    }
}

pub mod qos;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct QosProfiles {
        model item (0) -> qos_profiles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod qos_profiles;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct QueueMonitorLength {
        scalar enabled("enabled", 0) -> bool;
        model default_thresholds("default_thresholds", 1) -> queue_monitor_length::DefaultThresholds<'a>;
        scalar log("log", 2) -> i64;
        scalar notifying("notifying", 3) -> bool;
        model cpu("cpu", 4) -> queue_monitor_length::Cpu<'a>;
        scalar tx_latency("tx_latency", 5) -> bool;
        model mirror("mirror", 6) -> queue_monitor_length::Mirror<'a>;
    }
}

pub mod queue_monitor_length;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct QueueMonitorStreaming {
        scalar enable("enable", 0) -> bool;
        scalar ip_access_group("ip_access_group", 1) -> &'a str;
        scalar ipv6_access_group("ipv6_access_group", 2) -> &'a str;
        scalar max_connections("max_connections", 3) -> i64;
        scalar vrf("vrf", 4) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RadiusProxy {
        scalar client_key("client_key", 0) -> &'a str;
        scalar client_session_idle_timeout("client_session_idle_timeout", 1) -> i64;
        scalar dynamic_authorization("dynamic_authorization", 2) -> bool;
        model client_groups("client_groups", 3) -> radius_proxy::ClientGroups<'a>;
    }
}

pub mod radius_proxy;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RadiusServer {
        model attribute_32_include_in_access_req("attribute_32_include_in_access_req", 0) -> radius_server::Attribute32IncludeInAccessReq<'a>;
        scalar deadtime("deadtime", 1) -> i64;
        model dynamic_authorization("dynamic_authorization", 2) -> radius_server::DynamicAuthorization<'a>;
        model servers("servers", 3) -> radius_server::Servers<'a>;
        model vrfs("vrfs", 4) -> radius_server::Vrfs<'a>;
        scalar tls_ssl_profile("tls_ssl_profile", 5) -> &'a str;
    }
}

pub mod radius_server;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Redundancy {
        scalar protocol("protocol", 0) -> &'a str;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Roles {
        model item (0) -> roles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod roles;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RouteMaps {
        model item (0) -> route_maps::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod route_maps;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RouterAdaptiveVirtualTopology {
        scalar topology_role("topology_role", 0) -> &'a str;
        scalar gateway_vxlan("gateway_vxlan", 1) -> bool;
        model region("region", 2) -> router_adaptive_virtual_topology::Region<'a>;
        model zone("zone", 3) -> router_adaptive_virtual_topology::Zone<'a>;
        model site("site", 4) -> router_adaptive_virtual_topology::Site<'a>;
        model profiles("profiles", 5) -> router_adaptive_virtual_topology::Profiles<'a>;
        model policies("policies", 6) -> router_adaptive_virtual_topology::Policies<'a>;
        model vrfs("vrfs", 7) -> router_adaptive_virtual_topology::Vrfs<'a>;
    }
}

pub mod router_adaptive_virtual_topology;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RouterBfd {
        scalar interval("interval", 0) -> i64;
        scalar local_address("local_address", 1) -> &'a str;
        scalar min_rx("min_rx", 2) -> i64;
        scalar multiplier("multiplier", 3) -> i64;
        model multihop("multihop", 4) -> router_bfd::Multihop<'a>;
        scalar session_snapshot_interval("session_snapshot_interval", 5) -> i64;
        scalar session_snapshot_interval_dangerous("session_snapshot_interval_dangerous", 6) -> bool;
        model sbfd("sbfd", 7) -> router_bfd::Sbfd<'a>;
        scalar slow_timer("slow_timer", 8) -> i64;
    }
}

pub mod router_bfd;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RouterBgp {
        scalar field_as("as", 0) -> &'a str;
        scalar as_notation("as_notation", 1) -> &'a str;
        scalar router_id("router_id", 2) -> &'a str;
        model timers("timers", 3) -> router_bgp::Timers<'a>;
        model distance("distance", 4) -> router_bgp::Distance<'a>;
        model graceful_restart("graceful_restart", 5) -> router_bgp::GracefulRestart<'a>;
        model graceful_restart_helper("graceful_restart_helper", 6) -> router_bgp::GracefulRestartHelper<'a>;
        model maximum_paths("maximum_paths", 7) -> router_bgp::MaximumPaths<'a>;
        model route_distinguisher("route_distinguisher", 8) -> router_bgp::RouteDistinguisher<'a>;
        model updates("updates", 9) -> router_bgp::Updates<'a>;
        scalar bgp_cluster_id("bgp_cluster_id", 10) -> &'a str;
        model bgp_defaults("bgp_defaults", 11) -> router_bgp::BgpDefaults<'a>;
        model bgp("bgp", 12) -> router_bgp::Bgp<'a>;
        model listen_ranges("listen_ranges", 13) -> router_bgp::ListenRanges<'a>;
        model neighbor_default("neighbor_default", 14) -> router_bgp::NeighborDefault<'a>;
        model peer_groups("peer_groups", 15) -> router_bgp::PeerGroups<'a>;
        model neighbors("neighbors", 16) -> router_bgp::Neighbors<'a>;
        model neighbor_interfaces("neighbor_interfaces", 17) -> router_bgp::NeighborInterfaces<'a>;
        model aggregate_addresses("aggregate_addresses", 18) -> router_bgp::AggregateAddresses<'a>;
        model redistribute("redistribute", 19) -> router_bgp::Redistribute<'a>;
        model vlan_aware_bundles("vlan_aware_bundles", 20) -> router_bgp::VlanAwareBundles<'a>;
        model vlans("vlans", 21) -> router_bgp::Vlans<'a>;
        model vpws("vpws", 22) -> router_bgp::Vpws<'a>;
        model address_family_evpn("address_family_evpn", 23) -> router_bgp::AddressFamilyEvpn<'a>;
        model address_family_rtc("address_family_rtc", 24) -> router_bgp::AddressFamilyRtc<'a>;
        model address_family_ipv4("address_family_ipv4", 25) -> router_bgp::AddressFamilyIpv4<'a>;
        model address_family_ipv4_labeled_unicast("address_family_ipv4_labeled_unicast", 26) -> router_bgp::AddressFamilyIpv4LabeledUnicast<'a>;
        model address_family_ipv4_multicast("address_family_ipv4_multicast", 27) -> router_bgp::AddressFamilyIpv4Multicast<'a>;
        model address_family_ipv4_sr_te("address_family_ipv4_sr_te", 28) -> router_bgp::AddressFamilyIpv4SrTe<'a>;
        model address_family_ipv6("address_family_ipv6", 29) -> router_bgp::AddressFamilyIpv6<'a>;
        model address_family_ipv6_multicast("address_family_ipv6_multicast", 30) -> router_bgp::AddressFamilyIpv6Multicast<'a>;
        model address_family_ipv6_sr_te("address_family_ipv6_sr_te", 31) -> router_bgp::AddressFamilyIpv6SrTe<'a>;
        model address_family_link_state("address_family_link_state", 32) -> router_bgp::AddressFamilyLinkState<'a>;
        model address_family_flow_spec_ipv4("address_family_flow_spec_ipv4", 33) -> router_bgp::AddressFamilyFlowSpecIpv4<'a>;
        model address_family_flow_spec_ipv6("address_family_flow_spec_ipv6", 34) -> router_bgp::AddressFamilyFlowSpecIpv6<'a>;
        model address_family_path_selection("address_family_path_selection", 35) -> router_bgp::AddressFamilyPathSelection<'a>;
        model address_family_vpn_ipv4("address_family_vpn_ipv4", 36) -> router_bgp::AddressFamilyVpnIpv4<'a>;
        model address_family_vpn_ipv6("address_family_vpn_ipv6", 37) -> router_bgp::AddressFamilyVpnIpv6<'a>;
        model vrfs("vrfs", 38) -> router_bgp::Vrfs<'a>;
        model session_trackers("session_trackers", 39) -> router_bgp::SessionTrackers<'a>;
        scalar eos_cli("eos_cli", 40) -> &'a str;
    }
}

pub mod router_bgp;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RouterGeneral {
        model router_id("router_id", 0) -> router_general::RouterId<'a>;
        scalar nexthop_fast_failover("nexthop_fast_failover", 1) -> bool;
        scalar software_forwarding_hardware_offload_mtu("software_forwarding_hardware_offload_mtu", 2) -> i64;
        model vrfs("vrfs", 3) -> router_general::Vrfs<'a>;
        model control_functions("control_functions", 4) -> router_general::ControlFunctions<'a>;
    }
}

pub mod router_general;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RouterIgmp {
        scalar host_proxy_match_mroute("host_proxy_match_mroute", 0) -> &'a str;
        scalar ssm_aware("ssm_aware", 1) -> bool;
        model vrfs("vrfs", 2) -> router_igmp::Vrfs<'a>;
    }
}

pub mod router_igmp;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RouterInternetExit {
        model policies("policies", 0) -> router_internet_exit::Policies<'a>;
        model exit_groups("exit_groups", 1) -> router_internet_exit::ExitGroups<'a>;
    }
}

pub mod router_internet_exit;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RouterIsis {
        scalar instance("instance", 0) -> &'a str;
        scalar net("net", 1) -> &'a str;
        scalar router_id("router_id", 2) -> &'a str;
        scalar is_hostname("is_hostname", 3) -> &'a str;
        scalar is_type("is_type", 4) -> &'a str;
        scalar log_adjacency_changes("log_adjacency_changes", 5) -> bool;
        scalar mpls_ldp_sync_default("mpls_ldp_sync_default", 6) -> bool;
        model timers("timers", 7) -> router_isis::Timers<'a>;
        model set_overload_bit("set_overload_bit", 8) -> router_isis::SetOverloadBit<'a>;
        model authentication("authentication", 9) -> router_isis::Authentication<'a>;
        model advertise("advertise", 10) -> router_isis::Advertise<'a>;
        model redistribute_routes("redistribute_routes", 11) -> router_isis::RedistributeRoutes<'a>;
        model address_family_ipv4("address_family_ipv4", 12) -> router_isis::AddressFamilyIpv4<'a>;
        model address_family_ipv6("address_family_ipv6", 13) -> router_isis::AddressFamilyIpv6<'a>;
        model segment_routing_mpls("segment_routing_mpls", 14) -> router_isis::SegmentRoutingMpls<'a>;
        model spf_interval("spf_interval", 15) -> router_isis::SpfInterval<'a>;
        model graceful_restart("graceful_restart", 16) -> router_isis::GracefulRestart<'a>;
        scalar eos_cli("eos_cli", 17) -> &'a str;
    }
}

pub mod router_isis;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RouterL2Vpn {
        scalar arp_learning_bridged("arp_learning_bridged", 0) -> bool;
        model arp_proxy("arp_proxy", 1) -> router_l2_vpn::ArpProxy<'a>;
        scalar arp_selective_install("arp_selective_install", 2) -> bool;
        scalar nd_learning_bridged("nd_learning_bridged", 3) -> bool;
        model nd_proxy("nd_proxy", 4) -> router_l2_vpn::NdProxy<'a>;
        scalar nd_rs_flooding_disabled("nd_rs_flooding_disabled", 5) -> bool;
        scalar virtual_router_nd_ra_flooding_disabled("virtual_router_nd_ra_flooding_disabled", 6) -> bool;
    }
}

pub mod router_l2_vpn;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RouterMsdp {
        scalar originator_id_local_interface("originator_id_local_interface", 0) -> &'a str;
        scalar rejected_limit("rejected_limit", 1) -> i64;
        scalar forward_register_packets("forward_register_packets", 2) -> bool;
        scalar connection_retry_interval("connection_retry_interval", 3) -> i64;
        model group_limits("group_limits", 4) -> router_msdp::GroupLimits<'a>;
        model peers("peers", 5) -> router_msdp::Peers<'a>;
        model vrfs("vrfs", 6) -> router_msdp::Vrfs<'a>;
    }
}

pub mod router_msdp;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RouterMulticast {
        model ipv4("ipv4", 0) -> router_multicast::Ipv4<'a>;
        model ipv6("ipv6", 1) -> router_multicast::Ipv6<'a>;
        model vrfs("vrfs", 2) -> router_multicast::Vrfs<'a>;
    }
}

pub mod router_multicast;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RouterOspf {
        model process_ids("process_ids", 0) -> router_ospf::ProcessIds<'a>;
    }
}

pub mod router_ospf;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RouterOspfv3 {
        scalar router_id("router_id", 0) -> &'a str;
        scalar passive_interface_default("passive_interface_default", 1) -> bool;
        scalar auto_cost_reference_bandwidth("auto_cost_reference_bandwidth", 2) -> i64;
        model address_family_ipv4("address_family_ipv4", 3) -> router_ospfv3::AddressFamilyIpv4<'a>;
        model address_family_ipv6("address_family_ipv6", 4) -> router_ospfv3::AddressFamilyIpv6<'a>;
        model vrfs("vrfs", 5) -> router_ospfv3::Vrfs<'a>;
        scalar eos_cli("eos_cli", 6) -> &'a str;
    }
}

pub mod router_ospfv3;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RouterPathSelection {
        scalar peer_dynamic_source("peer_dynamic_source", 0) -> &'a str;
        scalar mtu_discovery_interval("mtu_discovery_interval", 1) -> i64;
        model mtu_discovery_hosts("mtu_discovery_hosts", 2) -> router_path_selection::MtuDiscoveryHosts<'a>;
        model path_groups("path_groups", 3) -> router_path_selection::PathGroups<'a>;
        model load_balance_policies("load_balance_policies", 4) -> router_path_selection::LoadBalancePolicies<'a>;
        model policies("policies", 5) -> router_path_selection::Policies<'a>;
        model vrfs("vrfs", 6) -> router_path_selection::Vrfs<'a>;
        model tcp_mss_ceiling("tcp_mss_ceiling", 7) -> router_path_selection::TcpMssCeiling<'a>;
        model interfaces("interfaces", 8) -> router_path_selection::Interfaces<'a>;
    }
}

pub mod router_path_selection;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RouterPimSparseMode {
        model ipv4("ipv4", 0) -> router_pim_sparse_mode::Ipv4<'a>;
        model vrfs("vrfs", 1) -> router_pim_sparse_mode::Vrfs<'a>;
    }
}

pub mod router_pim_sparse_mode;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RouterRip {
        model vrfs("vrfs", 0) -> router_rip::Vrfs<'a>;
    }
}

pub mod router_rip;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RouterSegmentSecurity {
        scalar enabled("enabled", 0) -> bool;
        model policies("policies", 1) -> router_segment_security::Policies<'a>;
        model vrfs("vrfs", 2) -> router_segment_security::Vrfs<'a>;
    }
}

pub mod router_segment_security;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RouterServiceInsertion {
        scalar enabled("enabled", 0) -> bool;
        model connections("connections", 1) -> router_service_insertion::Connections<'a>;
    }
}

pub mod router_service_insertion;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RouterTrafficEngineering {
        scalar enabled("enabled", 0) -> bool;
        model router_id("router_id", 1) -> router_traffic_engineering::RouterId<'a>;
        model segment_routing("segment_routing", 2) -> router_traffic_engineering::SegmentRouting<'a>;
        scalar twamp_light_sender_profile("twamp_light_sender_profile", 3) -> &'a str;
        model flex_algos("flex_algos", 4) -> router_traffic_engineering::FlexAlgos<'a>;
    }
}

pub mod router_traffic_engineering;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Schedule {
        model config("config", 0) -> schedule::Config<'a>;
        model jobs("jobs", 1) -> schedule::Jobs<'a>;
    }
}

pub mod schedule;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ServiceRoutingConfigurationBgp {
        scalar no_equals_default("no_equals_default", 0) -> bool;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ServiceUnsupportedTransceiver {
        scalar license_name("license_name", 0) -> &'a str;
        scalar license_key("license_key", 1) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Sflow {
        scalar sample("sample", 0) -> i64;
        scalar sample_truncate_size("sample_truncate_size", 1) -> i64;
        scalar sample_input_subinterface("sample_input_subinterface", 2) -> bool;
        scalar sample_output_subinterface("sample_output_subinterface", 3) -> bool;
        scalar dangerous("dangerous", 4) -> bool;
        scalar polling_interval("polling_interval", 5) -> i64;
        model vrfs("vrfs", 6) -> sflow::Vrfs<'a>;
        model destinations("destinations", 7) -> sflow::Destinations<'a>;
        scalar source("source", 8) -> &'a str;
        scalar source_interface("source_interface", 9) -> &'a str;
        model extensions("extensions", 10) -> sflow::Extensions<'a>;
        model interface("interface", 11) -> sflow::Interface<'a>;
        scalar run("run", 12) -> bool;
        model hardware_acceleration("hardware_acceleration", 13) -> sflow::HardwareAcceleration<'a>;
    }
}

pub mod sflow;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SnmpServer {
        model engine_ids("engine_ids", 0) -> snmp_server::EngineIds<'a>;
        model extensions("extensions", 1) -> snmp_server::Extensions<'a>;
        scalar contact("contact", 2) -> &'a str;
        scalar location("location", 3) -> &'a str;
        model communities("communities", 4) -> snmp_server::Communities<'a>;
        model ipv4_acls("ipv4_acls", 5) -> snmp_server::Ipv4Acls<'a>;
        model ipv6_acls("ipv6_acls", 6) -> snmp_server::Ipv6Acls<'a>;
        model local_interfaces("local_interfaces", 7) -> snmp_server::LocalInterfaces<'a>;
        model views("views", 8) -> snmp_server::Views<'a>;
        model groups("groups", 9) -> snmp_server::Groups<'a>;
        model users("users", 10) -> snmp_server::Users<'a>;
        model hosts("hosts", 11) -> snmp_server::Hosts<'a>;
        model traps("traps", 12) -> snmp_server::Traps<'a>;
        model vrfs("vrfs", 13) -> snmp_server::Vrfs<'a>;
        scalar ifmib_ifspeed_shape_rate("ifmib_ifspeed_shape_rate", 14) -> bool;
    }
}

pub mod snmp_server;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SpanningTree {
        scalar root_super("root_super", 0) -> bool;
        model edge_port("edge_port", 1) -> spanning_tree::EdgePort<'a>;
        scalar mode("mode", 2) -> &'a str;
        model bpduguard_rate_limit("bpduguard_rate_limit", 3) -> spanning_tree::BpduguardRateLimit<'a>;
        scalar rstp_priority("rstp_priority", 4) -> i64;
        model mst("mst", 5) -> spanning_tree::Mst<'a>;
        model mst_instances("mst_instances", 6) -> spanning_tree::MstInstances<'a>;
        scalar no_spanning_tree_vlan("no_spanning_tree_vlan", 7) -> &'a str;
        model rapid_pvst_instances("rapid_pvst_instances", 8) -> spanning_tree::RapidPvstInstances<'a>;
        model port_id_allocation_port_channel_range("port_id_allocation_port_channel_range", 9) -> spanning_tree::PortIdAllocationPortChannelRange<'a>;
        scalar loop_guard_default("loop_guard_default", 10) -> bool;
    }
}

pub mod spanning_tree;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct StandardAccessLists {
        model item (0) -> standard_access_lists::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod standard_access_lists;

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct StaticRoutes {
        model item (0) -> static_routes::Item<'a>;
    }
}

pub mod static_routes;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Stun {
        model client("client", 0) -> stun::Client<'a>;
        model server("server", 1) -> stun::Server<'a>;
    }
}

pub mod stun;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SwitchportDefault {
        scalar mode("mode", 0) -> &'a str;
        model phone("phone", 1) -> switchport_default::Phone<'a>;
    }
}

pub mod switchport_default;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SwitchportPortSecurity {
        model mac_address("mac_address", 0) -> switchport_port_security::MacAddress<'a>;
        scalar persistence_disabled("persistence_disabled", 1) -> bool;
        scalar violation_protect_chip_based("violation_protect_chip_based", 2) -> bool;
    }
}

pub mod switchport_port_security;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SyncE {
        scalar network_option("network_option", 0) -> i64;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct System {
        model control_plane("control_plane", 0) -> system::ControlPlane<'a>;
        model l1("l1", 1) -> system::L1<'a>;
    }
}

pub mod system;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct TacacsServers {
        scalar timeout("timeout", 0) -> i64;
        model hosts("hosts", 1) -> tacacs_servers::Hosts<'a>;
        scalar policy_unknown_mandatory_attribute_ignore("policy_unknown_mandatory_attribute_ignore", 2) -> bool;
    }
}

pub mod tacacs_servers;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct TapAggregation {
        model mode("mode", 0) -> tap_aggregation::Mode<'a>;
        scalar encapsulation_dot1br_strip("encapsulation_dot1br_strip", 1) -> bool;
        scalar encapsulation_vn_tag_strip("encapsulation_vn_tag_strip", 2) -> bool;
        scalar protocol_lldp_trap("protocol_lldp_trap", 3) -> bool;
        scalar truncation_size("truncation_size", 4) -> i64;
        model mac("mac", 5) -> tap_aggregation::Mac<'a>;
    }
}

pub mod tap_aggregation;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct TcamProfile {
        scalar system("system", 0) -> &'a str;
        model profiles("profiles", 1) -> tcam_profile::Profiles<'a>;
    }
}

pub mod tcam_profile;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Terminal {
        scalar length("length", 0) -> i64;
        scalar width("width", 1) -> i64;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Trackers {
        model item (0) -> trackers::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod trackers;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct TrafficPolicies {
        model cpu_traffic_policy("cpu_traffic_policy", 0) -> traffic_policies::CpuTrafficPolicy<'a>;
        model vrfs("vrfs", 1) -> traffic_policies::Vrfs<'a>;
        model options("options", 2) -> traffic_policies::Options<'a>;
        model field_sets("field_sets", 3) -> traffic_policies::FieldSets<'a>;
        model policies("policies", 4) -> traffic_policies::Policies<'a>;
    }
}

pub mod traffic_policies;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Transceiver {
        scalar dom_threshold_file("dom_threshold_file", 0) -> &'a str;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct TunnelInterfaces {
        model item (0) -> tunnel_interfaces::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod tunnel_interfaces;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct VirtualSourceNatVrfs {
        model item (0) -> virtual_source_nat_vrfs::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod virtual_source_nat_vrfs;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct VlanInterfaces {
        model item (0) -> vlan_interfaces::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod vlan_interfaces;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct VlanInternalOrder {
        scalar allocation("allocation", 0) -> &'a str;
        model range("range", 1) -> vlan_internal_order::Range<'a>;
    }
}

pub mod vlan_internal_order;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Vlans {
        model item (0) -> vlans::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod vlans;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct VmtracerSessions {
        model item (0) -> vmtracer_sessions::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod vmtracer_sessions;

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Vrfs {
        model item (0) -> vrfs::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod vrfs;

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct VxlanInterface {
        model vxlan1("vxlan1", 0) -> vxlan_interface::Vxlan1<'a>;
    }
}

pub mod vxlan_interface;
