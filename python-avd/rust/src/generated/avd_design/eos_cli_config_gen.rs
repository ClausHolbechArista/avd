// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct EosCliConfigGen<'a, Mode> {
    pub aaa_accounting: ::validated_data::Field<AaaAccounting<'a, Mode>>,
    pub aaa_authentication: ::validated_data::Field<AaaAuthentication<'a, Mode>>,
    pub aaa_authorization: ::validated_data::Field<AaaAuthorization<'a, Mode>>,
    pub aaa_root: ::validated_data::Field<AaaRoot<'a, Mode>>,
    pub aaa_server_groups: ::validated_data::Field<AaaServerGroups<'a, Mode>>,
    pub access_lists: ::validated_data::Field<AccessLists<'a, Mode>>,
    pub address_locking: ::validated_data::Field<AddressLocking<'a, Mode>>,
    pub agents: ::validated_data::Field<Agents<'a, Mode>>,
    pub aliases: ::validated_data::Field<&'a str>,
    pub application_traffic_recognition: ::validated_data::Field<ApplicationTrafficRecognition<'a, Mode>>,
    pub arp: ::validated_data::Field<Arp<'a, Mode>>,
    pub as_path: ::validated_data::Field<AsPath<'a, Mode>>,
    pub avd_structured_config_file_format: ::validated_data::Field<&'a str>,
    pub avd_vault_id: ::validated_data::Field<&'a str>,
    pub banners: ::validated_data::Field<Banners<'a, Mode>>,
    pub bgp_groups: ::validated_data::Field<BgpGroups<'a, Mode>>,
    pub boot: ::validated_data::Field<Boot<'a, Mode>>,
    pub cfm: ::validated_data::Field<Cfm<'a, Mode>>,
    pub class_maps: ::validated_data::Field<ClassMaps<'a, Mode>>,
    pub clock: ::validated_data::Field<Clock<'a, Mode>>,
    pub config_comment: ::validated_data::Field<&'a str>,
    pub config_end: ::validated_data::Field<bool>,
    pub custom_templates: ::validated_data::Field<CustomTemplates<'a, Mode>>,
    pub cvx: ::validated_data::Field<Cvx<'a, Mode>>,
    pub daemon_terminattr: ::validated_data::Field<DaemonTerminattr<'a, Mode>>,
    pub daemons: ::validated_data::Field<Daemons<'a, Mode>>,
    pub dhcp_relay: ::validated_data::Field<DhcpRelay<'a, Mode>>,
    pub dhcp_servers: ::validated_data::Field<DhcpServers<'a, Mode>>,
    pub dns_domain: ::validated_data::Field<&'a str>,
    pub domain_list: ::validated_data::Field<DomainList<'a, Mode>>,
    pub dot1x: ::validated_data::Field<Dot1x<'a, Mode>>,
    pub dps_interfaces: ::validated_data::Field<DpsInterfaces<'a, Mode>>,
    pub dynamic_prefix_lists: ::validated_data::Field<DynamicPrefixLists<'a, Mode>>,
    pub enable_password: ::validated_data::Field<EnablePassword<'a, Mode>>,
    pub environment_fan_speed: ::validated_data::Field<EnvironmentFanSpeed<'a, Mode>>,
    pub eos_cli: ::validated_data::Field<&'a str>,
    pub eos_cli_config_gen_configuration: ::validated_data::Field<EosCliConfigGenConfiguration<'a, Mode>>,
    pub eos_cli_config_gen_documentation: ::validated_data::Field<EosCliConfigGenDocumentation<'a, Mode>>,
    pub eos_cli_config_gen_keep_tmp_files: ::validated_data::Field<bool>,
    pub eos_cli_config_gen_tmp_dir: ::validated_data::Field<&'a str>,
    pub eos_cli_config_gen_validate_inputs_batch_size: ::validated_data::Field<i64>,
    pub eos_config_future: ::validated_data::Field<EosConfigFuture<'a, Mode>>,
    pub errdisable: ::validated_data::Field<Errdisable<'a, Mode>>,
    pub ethernet_interfaces: ::validated_data::Field<EthernetInterfaces<'a, Mode>>,
    pub event_handlers: ::validated_data::Field<EventHandlers<'a, Mode>>,
    pub event_monitor: ::validated_data::Field<EventMonitor<'a, Mode>>,
    pub flow_tracking: ::validated_data::Field<FlowTracking<'a, Mode>>,
    pub hardware: ::validated_data::Field<Hardware<'a, Mode>>,
    pub hardware_counters: ::validated_data::Field<HardwareCounters<'a, Mode>>,
    pub hostname: ::validated_data::Field<&'a str>,
    pub interface_defaults: ::validated_data::Field<InterfaceDefaults<'a, Mode>>,
    pub interface_groups: ::validated_data::Field<InterfaceGroups<'a, Mode>>,
    pub interface_profiles: ::validated_data::Field<InterfaceProfiles<'a, Mode>>,
    pub ip_access_lists: ::validated_data::Field<IpAccessLists<'a, Mode>>,
    pub ip_access_lists_max_entries: ::validated_data::Field<i64>,
    pub ip_community_lists: ::validated_data::Field<IpCommunityLists<'a, Mode>>,
    pub ip_dhcp_relay: ::validated_data::Field<IpDhcpRelay<'a, Mode>>,
    pub ip_dhcp_snooping: ::validated_data::Field<IpDhcpSnooping<'a, Mode>>,
    pub ip_domain_lookup: ::validated_data::Field<IpDomainLookup<'a, Mode>>,
    pub ip_extcommunity_lists: ::validated_data::Field<IpExtcommunityLists<'a, Mode>>,
    pub ip_extcommunity_lists_regexp: ::validated_data::Field<IpExtcommunityListsRegexp<'a, Mode>>,
    pub ip_ftp_client: ::validated_data::Field<IpFtpClient<'a, Mode>>,
    pub ip_hardware: ::validated_data::Field<IpHardware<'a, Mode>>,
    pub ip_hosts: ::validated_data::Field<IpHosts<'a, Mode>>,
    pub ip_http_client: ::validated_data::Field<IpHttpClient<'a, Mode>>,
    pub ip_icmp_redirect: ::validated_data::Field<bool>,
    pub ip_igmp_snooping: ::validated_data::Field<IpIgmpSnooping<'a, Mode>>,
    pub ip_large_community_lists: ::validated_data::Field<IpLargeCommunityLists<'a, Mode>>,
    pub ip_name_server: ::validated_data::Field<IpNameServer<'a, Mode>>,
    pub ip_name_server_groups: ::validated_data::Field<IpNameServerGroups<'a, Mode>>,
    pub ip_nat: ::validated_data::Field<IpNat<'a, Mode>>,
    pub ip_ospf_router_id_output_format_hostnames: ::validated_data::Field<bool>,
    pub ip_radius: ::validated_data::Field<IpRadius<'a, Mode>>,
    pub ip_radius_source_interfaces: ::validated_data::Field<IpRadiusSourceInterfaces<'a, Mode>>,
    pub ip_routing: ::validated_data::Field<bool>,
    pub ip_routing_ipv6_interfaces: ::validated_data::Field<bool>,
    pub ip_security: ::validated_data::Field<IpSecurity<'a, Mode>>,
    pub ip_software_forwarding: ::validated_data::Field<IpSoftwareForwarding<'a, Mode>>,
    pub ip_ssh_client: ::validated_data::Field<IpSshClient<'a, Mode>>,
    pub ip_tacacs: ::validated_data::Field<IpTacacs<'a, Mode>>,
    pub ip_tacacs_source_interfaces: ::validated_data::Field<IpTacacsSourceInterfaces<'a, Mode>>,
    pub ip_telnet_client: ::validated_data::Field<IpTelnetClient<'a, Mode>>,
    pub ip_tftp_client: ::validated_data::Field<IpTftpClient<'a, Mode>>,
    pub ip_virtual_router_mac_address: ::validated_data::Field<&'a str>,
    pub ip_virtual_router_mac_address_advertisement_interval: ::validated_data::Field<i64>,
    pub ip_virtual_router_mac_address_mlag_peer: ::validated_data::Field<bool>,
    pub ipv6_access_lists: ::validated_data::Field<Ipv6AccessLists<'a, Mode>>,
    pub ipv6_dhcp_relay: ::validated_data::Field<Ipv6DhcpRelay<'a, Mode>>,
    pub ipv6_hardware: ::validated_data::Field<Ipv6Hardware<'a, Mode>>,
    pub ipv6_icmp_redirect: ::validated_data::Field<bool>,
    pub ipv6_neighbor: ::validated_data::Field<Ipv6Neighbor<'a, Mode>>,
    pub ipv6_prefix_lists: ::validated_data::Field<Ipv6PrefixLists<'a, Mode>>,
    pub ipv6_router_ospf: ::validated_data::Field<Ipv6RouterOspf<'a, Mode>>,
    pub ipv6_standard_access_lists: ::validated_data::Field<Ipv6StandardAccessLists<'a, Mode>>,
    pub ipv6_static_routes: ::validated_data::Field<Ipv6StaticRoutes<'a, Mode>>,
    pub ipv6_unicast_routing: ::validated_data::Field<bool>,
    pub kernel: ::validated_data::Field<Kernel<'a, Mode>>,
    pub l2_protocol: ::validated_data::Field<L2Protocol<'a, Mode>>,
    pub lacp: ::validated_data::Field<Lacp<'a, Mode>>,
    pub link_tracking_groups: ::validated_data::Field<LinkTrackingGroups<'a, Mode>>,
    pub lldp: ::validated_data::Field<Lldp<'a, Mode>>,
    pub load_balance: ::validated_data::Field<LoadBalance<'a, Mode>>,
    pub load_interval: ::validated_data::Field<LoadInterval<'a, Mode>>,
    pub local_users: ::validated_data::Field<LocalUsers<'a, Mode>>,
    pub logging: ::validated_data::Field<Logging<'a, Mode>>,
    pub loopback_interfaces: ::validated_data::Field<LoopbackInterfaces<'a, Mode>>,
    pub mac_access_lists: ::validated_data::Field<MacAccessLists<'a, Mode>>,
    pub mac_address_table: ::validated_data::Field<MacAddressTable<'a, Mode>>,
    pub mac_security: ::validated_data::Field<MacSecurity<'a, Mode>>,
    pub maintenance: ::validated_data::Field<Maintenance<'a, Mode>>,
    pub management_accounts: ::validated_data::Field<ManagementAccounts<'a, Mode>>,
    pub management_api_gnmi: ::validated_data::Field<ManagementApiGnmi<'a, Mode>>,
    pub management_api_http: ::validated_data::Field<ManagementApiHttp<'a, Mode>>,
    pub management_api_models: ::validated_data::Field<ManagementApiModels<'a, Mode>>,
    pub management_console: ::validated_data::Field<ManagementConsole<'a, Mode>>,
    pub management_cvx: ::validated_data::Field<ManagementCvx<'a, Mode>>,
    pub management_defaults: ::validated_data::Field<ManagementDefaults<'a, Mode>>,
    pub management_interfaces: ::validated_data::Field<ManagementInterfaces<'a, Mode>>,
    pub management_ldap: ::validated_data::Field<ManagementLdap<'a, Mode>>,
    pub management_security: ::validated_data::Field<ManagementSecurity<'a, Mode>>,
    pub management_ssh: ::validated_data::Field<ManagementSsh<'a, Mode>>,
    pub management_tech_support: ::validated_data::Field<ManagementTechSupport<'a, Mode>>,
    pub match_list_input: ::validated_data::Field<MatchListInput<'a, Mode>>,
    pub mcs_client: ::validated_data::Field<McsClient<'a, Mode>>,
    pub metadata: ::validated_data::Field<Metadata<'a, Mode>>,
    pub mlag_configuration: ::validated_data::Field<MlagConfiguration<'a, Mode>>,
    pub monitor_connectivity: ::validated_data::Field<MonitorConnectivity<'a, Mode>>,
    pub monitor_layer1: ::validated_data::Field<MonitorLayer1<'a, Mode>>,
    pub monitor_link_flap_policy: ::validated_data::Field<MonitorLinkFlapPolicy<'a, Mode>>,
    pub monitor_loop_protection: ::validated_data::Field<MonitorLoopProtection<'a, Mode>>,
    pub monitor_server_radius: ::validated_data::Field<MonitorServerRadius<'a, Mode>>,
    pub monitor_session_default_encapsulation_gre: ::validated_data::Field<MonitorSessionDefaultEncapsulationGre<'a, Mode>>,
    pub monitor_sessions: ::validated_data::Field<MonitorSessions<'a, Mode>>,
    pub monitor_telemetry_influx: ::validated_data::Field<MonitorTelemetryInflux<'a, Mode>>,
    pub monitor_telemetry_postcard_policy: ::validated_data::Field<MonitorTelemetryPostcardPolicy<'a, Mode>>,
    pub monitor_twamp: ::validated_data::Field<MonitorTwamp<'a, Mode>>,
    pub mpls: ::validated_data::Field<Mpls<'a, Mode>>,
    pub ntp: ::validated_data::Field<Ntp<'a, Mode>>,
    pub patch_panel: ::validated_data::Field<PatchPanel<'a, Mode>>,
    pub peer_filters: ::validated_data::Field<PeerFilters<'a, Mode>>,
    pub platform: ::validated_data::Field<Platform<'a, Mode>>,
    pub poe: ::validated_data::Field<Poe<'a, Mode>>,
    pub policy_maps: ::validated_data::Field<PolicyMaps<'a, Mode>>,
    pub port_channel: ::validated_data::Field<PortChannel<'a, Mode>>,
    pub port_channel_interfaces: ::validated_data::Field<PortChannelInterfaces<'a, Mode>>,
    pub prefix_lists: ::validated_data::Field<PrefixLists<'a, Mode>>,
    pub priority_flow_control: ::validated_data::Field<PriorityFlowControl<'a, Mode>>,
    pub prompt: ::validated_data::Field<&'a str>,
    pub ptp: ::validated_data::Field<Ptp<'a, Mode>>,
    pub qos: ::validated_data::Field<Qos<'a, Mode>>,
    pub qos_profiles: ::validated_data::Field<QosProfiles<'a, Mode>>,
    pub queue_monitor_length: ::validated_data::Field<QueueMonitorLength<'a, Mode>>,
    pub queue_monitor_streaming: ::validated_data::Field<QueueMonitorStreaming<'a, Mode>>,
    pub radius_proxy: ::validated_data::Field<RadiusProxy<'a, Mode>>,
    pub radius_server: ::validated_data::Field<RadiusServer<'a, Mode>>,
    pub read_structured_config_from_file: ::validated_data::Field<bool>,
    pub redundancy: ::validated_data::Field<Redundancy<'a, Mode>>,
    pub roles: ::validated_data::Field<Roles<'a, Mode>>,
    pub route_maps: ::validated_data::Field<RouteMaps<'a, Mode>>,
    pub router_adaptive_virtual_topology: ::validated_data::Field<RouterAdaptiveVirtualTopology<'a, Mode>>,
    pub router_bfd: ::validated_data::Field<RouterBfd<'a, Mode>>,
    pub router_bgp: ::validated_data::Field<RouterBgp<'a, Mode>>,
    pub router_general: ::validated_data::Field<RouterGeneral<'a, Mode>>,
    pub router_igmp: ::validated_data::Field<RouterIgmp<'a, Mode>>,
    pub router_internet_exit: ::validated_data::Field<RouterInternetExit<'a, Mode>>,
    pub router_isis: ::validated_data::Field<RouterIsis<'a, Mode>>,
    pub router_l2_vpn: ::validated_data::Field<RouterL2Vpn<'a, Mode>>,
    pub router_msdp: ::validated_data::Field<RouterMsdp<'a, Mode>>,
    pub router_multicast: ::validated_data::Field<RouterMulticast<'a, Mode>>,
    pub router_ospf: ::validated_data::Field<RouterOspf<'a, Mode>>,
    pub router_ospfv3: ::validated_data::Field<RouterOspfv3<'a, Mode>>,
    pub router_path_selection: ::validated_data::Field<RouterPathSelection<'a, Mode>>,
    pub router_pim_sparse_mode: ::validated_data::Field<RouterPimSparseMode<'a, Mode>>,
    pub router_rip: ::validated_data::Field<RouterRip<'a, Mode>>,
    pub router_segment_security: ::validated_data::Field<RouterSegmentSecurity<'a, Mode>>,
    pub router_service_insertion: ::validated_data::Field<RouterServiceInsertion<'a, Mode>>,
    pub router_traffic_engineering: ::validated_data::Field<RouterTrafficEngineering<'a, Mode>>,
    pub schedule: ::validated_data::Field<Schedule<'a, Mode>>,
    pub service_routing_configuration_bgp: ::validated_data::Field<ServiceRoutingConfigurationBgp<'a, Mode>>,
    pub service_routing_protocols_model: ::validated_data::Field<&'a str>,
    pub service_unsupported_transceiver: ::validated_data::Field<ServiceUnsupportedTransceiver<'a, Mode>>,
    pub sflow: ::validated_data::Field<Sflow<'a, Mode>>,
    pub snmp_server: ::validated_data::Field<SnmpServer<'a, Mode>>,
    pub spanning_tree: ::validated_data::Field<SpanningTree<'a, Mode>>,
    pub standard_access_lists: ::validated_data::Field<StandardAccessLists<'a, Mode>>,
    pub static_routes: ::validated_data::Field<StaticRoutes<'a, Mode>>,
    pub stun: ::validated_data::Field<Stun<'a, Mode>>,
    pub switchport_default: ::validated_data::Field<SwitchportDefault<'a, Mode>>,
    pub switchport_ethernet_llc_validation: ::validated_data::Field<bool>,
    pub switchport_port_security: ::validated_data::Field<SwitchportPortSecurity<'a, Mode>>,
    pub switchport_vlan_tag_validation: ::validated_data::Field<bool>,
    pub sync_e: ::validated_data::Field<SyncE<'a, Mode>>,
    pub system: ::validated_data::Field<System<'a, Mode>>,
    pub tacacs_servers: ::validated_data::Field<TacacsServers<'a, Mode>>,
    pub tap_aggregation: ::validated_data::Field<TapAggregation<'a, Mode>>,
    pub tcam_profile: ::validated_data::Field<TcamProfile<'a, Mode>>,
    pub terminal: ::validated_data::Field<Terminal<'a, Mode>>,
    pub trackers: ::validated_data::Field<Trackers<'a, Mode>>,
    pub traffic_policies: ::validated_data::Field<TrafficPolicies<'a, Mode>>,
    pub transceiver: ::validated_data::Field<Transceiver<'a, Mode>>,
    pub transceiver_qsfp_default_mode_4x10: ::validated_data::Field<bool>,
    pub tunnel_interfaces: ::validated_data::Field<TunnelInterfaces<'a, Mode>>,
    pub virtual_source_nat_vrfs: ::validated_data::Field<VirtualSourceNatVrfs<'a, Mode>>,
    pub vlan_interfaces: ::validated_data::Field<VlanInterfaces<'a, Mode>>,
    pub vlan_internal_order: ::validated_data::Field<VlanInternalOrder<'a, Mode>>,
    pub vlans: ::validated_data::Field<Vlans<'a, Mode>>,
    pub vmtracer_sessions: ::validated_data::Field<VmtracerSessions<'a, Mode>>,
    pub vrfs: ::validated_data::Field<Vrfs<'a, Mode>>,
    pub vxlan_interface: ::validated_data::Field<VxlanInterface<'a, Mode>>,
}

#[::validated_data::data_view]
pub struct AaaAccounting<'a, Mode> {
    pub exec: ::validated_data::Field<aaa_accounting::Exec<'a, Mode>>,
    pub system: ::validated_data::Field<aaa_accounting::System<'a, Mode>>,
    pub dot1x: ::validated_data::Field<aaa_accounting::Dot1x<'a, Mode>>,
    pub commands: ::validated_data::Field<aaa_accounting::Commands<'a, Mode>>,
}

pub mod aaa_accounting;

#[::validated_data::data_view]
pub struct AaaAuthentication<'a, Mode> {
    pub login: ::validated_data::Field<aaa_authentication::Login<'a, Mode>>,
    pub enable: ::validated_data::Field<aaa_authentication::Enable<'a, Mode>>,
    pub dot1x: ::validated_data::Field<aaa_authentication::Dot1x<'a, Mode>>,
    pub policies: ::validated_data::Field<aaa_authentication::Policies<'a, Mode>>,
}

pub mod aaa_authentication;

#[::validated_data::data_view]
pub struct AaaAuthorization<'a, Mode> {
    pub policy: ::validated_data::Field<aaa_authorization::Policy<'a, Mode>>,
    pub exec: ::validated_data::Field<aaa_authorization::Exec<'a, Mode>>,
    pub config_commands: ::validated_data::Field<bool>,
    pub serial_console: ::validated_data::Field<bool>,
    pub dynamic: ::validated_data::Field<aaa_authorization::Dynamic<'a, Mode>>,
    pub commands: ::validated_data::Field<aaa_authorization::Commands<'a, Mode>>,
}

pub mod aaa_authorization;

#[::validated_data::data_view]
pub struct AaaRoot<'a, Mode> {
    pub disabled: ::validated_data::Field<bool>,
    pub secret: ::validated_data::Field<aaa_root::Secret<'a, Mode>>,
}

pub mod aaa_root;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct AaaServerGroups<'a, Mode> (::validated_data::Field<aaa_server_groups::Item<'a, Mode>>);

pub mod aaa_server_groups;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct AccessLists<'a, Mode> (::validated_data::Field<access_lists::Item<'a, Mode>>);

pub mod access_lists;

#[::validated_data::data_view]
pub struct AddressLocking<'a, Mode> {
    pub dhcp_servers_ipv4: ::validated_data::Field<address_locking::DhcpServersIpv4<'a, Mode>>,
    pub dhcp_server_interfaces: ::validated_data::Field<address_locking::DhcpServerInterfaces<'a, Mode>>,
    pub disabled: ::validated_data::Field<bool>,
    pub leases: ::validated_data::Field<address_locking::Leases<'a, Mode>>,
    pub local_interface: ::validated_data::Field<&'a str>,
    pub locked_address: ::validated_data::Field<address_locking::LockedAddress<'a, Mode>>,
}

pub mod address_locking;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Agents<'a, Mode> (::validated_data::Field<agents::Item<'a, Mode>>);

pub mod agents;

#[::validated_data::data_view]
pub struct ApplicationTrafficRecognition<'a, Mode> {
    pub categories: ::validated_data::Field<application_traffic_recognition::Categories<'a, Mode>>,
    pub field_sets: ::validated_data::Field<application_traffic_recognition::FieldSets<'a, Mode>>,
    pub applications: ::validated_data::Field<application_traffic_recognition::Applications<'a, Mode>>,
    pub application_profiles: ::validated_data::Field<application_traffic_recognition::ApplicationProfiles<'a, Mode>>,
}

pub mod application_traffic_recognition;

#[::validated_data::data_view]
pub struct Arp<'a, Mode> {
    pub persistent: ::validated_data::Field<arp::Persistent<'a, Mode>>,
    pub aging: ::validated_data::Field<arp::Aging<'a, Mode>>,
    pub static_entries: ::validated_data::Field<arp::StaticEntries<'a, Mode>>,
}

pub mod arp;

#[::validated_data::data_view]
pub struct AsPath<'a, Mode> {
    pub regex_mode: ::validated_data::Field<&'a str>,
    pub access_lists: ::validated_data::Field<as_path::AccessLists<'a, Mode>>,
}

pub mod as_path;

#[::validated_data::data_view]
pub struct Banners<'a, Mode> {
    pub login: ::validated_data::Field<&'a str>,
    pub motd: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct BgpGroups<'a, Mode> (::validated_data::Field<bgp_groups::Item<'a, Mode>>);

pub mod bgp_groups;

#[::validated_data::data_view]
pub struct Boot<'a, Mode> {
    pub secret: ::validated_data::Field<boot::Secret<'a, Mode>>,
}

pub mod boot;

#[::validated_data::data_view]
pub struct Cfm<'a, Mode> {
    pub continuity_check_loc_state_action_disable_interface_routing: ::validated_data::Field<bool>,
    pub domains: ::validated_data::Field<cfm::Domains<'a, Mode>>,
    pub measurement_loss: ::validated_data::Field<cfm::MeasurementLoss<'a, Mode>>,
    pub profiles: ::validated_data::Field<cfm::Profiles<'a, Mode>>,
}

pub mod cfm;

#[::validated_data::data_view]
pub struct ClassMaps<'a, Mode> {
    pub pbr: ::validated_data::Field<class_maps::Pbr<'a, Mode>>,
    pub qos: ::validated_data::Field<class_maps::Qos<'a, Mode>>,
}

pub mod class_maps;

#[::validated_data::data_view]
pub struct Clock<'a, Mode> {
    pub timezone: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(list)]
pub struct CustomTemplates<'a, Mode> (::validated_data::Field<&'a str>);

#[::validated_data::data_view]
pub struct Cvx<'a, Mode> {
    pub shutdown: ::validated_data::Field<bool>,
    pub peer_hosts: ::validated_data::Field<cvx::PeerHosts<'a, Mode>>,
    pub services: ::validated_data::Field<cvx::Services<'a, Mode>>,
}

pub mod cvx;

#[::validated_data::data_view]
pub struct DaemonTerminattr<'a, Mode> {
    pub cvaddrs: ::validated_data::Field<daemon_terminattr::Cvaddrs<'a, Mode>>,
    pub clusters: ::validated_data::Field<daemon_terminattr::Clusters<'a, Mode>>,
    pub cvauth: ::validated_data::Field<daemon_terminattr::Cvauth<'a, Mode>>,
    pub cvobscurekeyfile: ::validated_data::Field<bool>,
    pub cvproxy: ::validated_data::Field<&'a str>,
    pub cvsourceip: ::validated_data::Field<&'a str>,
    pub cvsourceintf: ::validated_data::Field<&'a str>,
    pub cvvrf: ::validated_data::Field<&'a str>,
    pub cvgnmi: ::validated_data::Field<bool>,
    pub disable_aaa: ::validated_data::Field<bool>,
    pub grpcaddr: ::validated_data::Field<&'a str>,
    pub grpcreadonly: ::validated_data::Field<bool>,
    pub ingestexclude: ::validated_data::Field<&'a str>,
    pub smashexcludes: ::validated_data::Field<&'a str>,
    pub sysdbexcludes: ::validated_data::Field<&'a str>,
    pub taillogs: ::validated_data::Field<&'a str>,
    pub ecodhcpaddr: ::validated_data::Field<&'a str>,
    pub ipfix: ::validated_data::Field<bool>,
    pub ipfixaddr: ::validated_data::Field<&'a str>,
    pub sflow: ::validated_data::Field<bool>,
    pub sflowaddr: ::validated_data::Field<&'a str>,
    pub cvconfig: ::validated_data::Field<bool>,
    pub cv_loss_timeout: ::validated_data::Field<i64>,
    pub cvtargetconfigs: ::validated_data::Field<daemon_terminattr::Cvtargetconfigs<'a, Mode>>,
    pub flowdns: ::validated_data::Field<bool>,
    pub custom_cv_options: ::validated_data::Field<daemon_terminattr::CustomCvOptions<'a, Mode>>,
}

pub mod daemon_terminattr;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Daemons<'a, Mode> (::validated_data::Field<daemons::Item<'a, Mode>>);

pub mod daemons;

#[::validated_data::data_view]
pub struct DhcpRelay<'a, Mode> {
    pub servers: ::validated_data::Field<dhcp_relay::Servers<'a, Mode>>,
    pub tunnel_requests_disabled: ::validated_data::Field<bool>,
    pub mlag_peerlink_requests_disabled: ::validated_data::Field<bool>,
    pub client_requests: ::validated_data::Field<dhcp_relay::ClientRequests<'a, Mode>>,
    pub reply_source_address_validation: ::validated_data::Field<bool>,
}

pub mod dhcp_relay;

#[::validated_data::data_view(indexed_list, primary_key(vrf))]
pub struct DhcpServers<'a, Mode> (::validated_data::Field<dhcp_servers::Item<'a, Mode>>);

pub mod dhcp_servers;

#[::validated_data::data_view(list)]
pub struct DomainList<'a, Mode> (::validated_data::Field<&'a str>);

#[::validated_data::data_view]
pub struct Dot1x<'a, Mode> {
    pub system_auth_control: ::validated_data::Field<bool>,
    pub protocol_lldp_bypass: ::validated_data::Field<bool>,
    pub protocol_bpdu_bypass: ::validated_data::Field<bool>,
    pub dynamic_authorization: ::validated_data::Field<bool>,
    pub statistics_packets_dropped: ::validated_data::Field<bool>,
    pub mac_based_authentication: ::validated_data::Field<dot1x::MacBasedAuthentication<'a, Mode>>,
    pub radius_av_pair_username_format: ::validated_data::Field<dot1x::RadiusAvPairUsernameFormat<'a, Mode>>,
    pub radius_av_pair: ::validated_data::Field<dot1x::RadiusAvPair<'a, Mode>>,
    pub aaa: ::validated_data::Field<dot1x::Aaa<'a, Mode>>,
    pub captive_portal: ::validated_data::Field<dot1x::CaptivePortal<'a, Mode>>,
    pub supplicant: ::validated_data::Field<dot1x::Supplicant<'a, Mode>>,
    pub vlan_assignment_groups: ::validated_data::Field<dot1x::VlanAssignmentGroups<'a, Mode>>,
    pub eapol: ::validated_data::Field<dot1x::Eapol<'a, Mode>>,
}

pub mod dot1x;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct DpsInterfaces<'a, Mode> (::validated_data::Field<dps_interfaces::Item<'a, Mode>>);

pub mod dps_interfaces;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct DynamicPrefixLists<'a, Mode> (::validated_data::Field<dynamic_prefix_lists::Item<'a, Mode>>);

pub mod dynamic_prefix_lists;

#[::validated_data::data_view]
pub struct EnablePassword<'a, Mode> {
    pub disabled: ::validated_data::Field<bool>,
    pub hash_algorithm: ::validated_data::Field<&'a str>,
    pub key: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct EnvironmentFanSpeed<'a, Mode> {
    pub minimum: ::validated_data::Field<i64>,
}

#[::validated_data::data_view]
pub struct EosCliConfigGenConfiguration<'a, Mode> {
    pub enable: ::validated_data::Field<bool>,
    pub hide_passwords: ::validated_data::Field<bool>,
}

#[::validated_data::data_view]
pub struct EosCliConfigGenDocumentation<'a, Mode> {
    pub enable: ::validated_data::Field<bool>,
    pub hide_passwords: ::validated_data::Field<bool>,
    pub toc: ::validated_data::Field<bool>,
}

#[::validated_data::data_view]
pub struct EosConfigFuture<'a, Mode> {
    pub always_render_ip_routing_separator: ::validated_data::Field<bool>,
    pub render_combined_separator_for_ipv6_hardware_and_unicast_routing: ::validated_data::Field<bool>,
    pub new_ip_radius_cli_order: ::validated_data::Field<bool>,
    pub new_ip_tacacs_cli_order: ::validated_data::Field<bool>,
    pub only_render_mpls_rsvp_with_settings: ::validated_data::Field<bool>,
    pub render_monitor_layer1_without_enabled: ::validated_data::Field<bool>,
    pub render_spanning_tree_portfast_edge: ::validated_data::Field<bool>,
    pub only_render_separator_with_boot_secret_key: ::validated_data::Field<bool>,
}

#[::validated_data::data_view]
pub struct Errdisable<'a, Mode> {
    pub detect: ::validated_data::Field<errdisable::Detect<'a, Mode>>,
    pub detect_cause: ::validated_data::Field<errdisable::DetectCause<'a, Mode>>,
    pub recovery: ::validated_data::Field<errdisable::Recovery<'a, Mode>>,
    pub recovery_cause: ::validated_data::Field<errdisable::RecoveryCause<'a, Mode>>,
    pub recovery_interval: ::validated_data::Field<i64>,
}

pub mod errdisable;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct EthernetInterfaces<'a, Mode> (::validated_data::Field<ethernet_interfaces::Item<'a, Mode>>);

pub mod ethernet_interfaces;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct EventHandlers<'a, Mode> (::validated_data::Field<event_handlers::Item<'a, Mode>>);

pub mod event_handlers;

#[::validated_data::data_view]
pub struct EventMonitor<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
}

#[::validated_data::data_view]
pub struct FlowTracking<'a, Mode> {
    pub sampled: ::validated_data::Field<flow_tracking::Sampled<'a, Mode>>,
    pub hardware: ::validated_data::Field<flow_tracking::Hardware<'a, Mode>>,
    pub mirror_on_drop: ::validated_data::Field<flow_tracking::MirrorOnDrop<'a, Mode>>,
}

pub mod flow_tracking;

#[::validated_data::data_view]
pub struct Hardware<'a, Mode> {
    pub access_list: ::validated_data::Field<hardware::AccessList<'a, Mode>>,
    pub speed_groups: ::validated_data::Field<hardware::SpeedGroups<'a, Mode>>,
    pub port_groups: ::validated_data::Field<hardware::PortGroups<'a, Mode>>,
}

pub mod hardware;

#[::validated_data::data_view]
pub struct HardwareCounters<'a, Mode> {
    pub features: ::validated_data::Field<hardware_counters::Features<'a, Mode>>,
}

pub mod hardware_counters;

#[::validated_data::data_view]
pub struct InterfaceDefaults<'a, Mode> {
    pub ethernet: ::validated_data::Field<interface_defaults::Ethernet<'a, Mode>>,
    pub mtu: ::validated_data::Field<i64>,
}

pub mod interface_defaults;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct InterfaceGroups<'a, Mode> (::validated_data::Field<interface_groups::Item<'a, Mode>>);

pub mod interface_groups;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct InterfaceProfiles<'a, Mode> (::validated_data::Field<interface_profiles::Item<'a, Mode>>);

pub mod interface_profiles;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct IpAccessLists<'a, Mode> (::validated_data::Field<ip_access_lists::Item<'a, Mode>>);

pub mod ip_access_lists;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct IpCommunityLists<'a, Mode> (::validated_data::Field<ip_community_lists::Item<'a, Mode>>);

pub mod ip_community_lists;

#[::validated_data::data_view]
pub struct IpDhcpRelay<'a, Mode> {
    pub always_on: ::validated_data::Field<bool>,
    pub all_subnets: ::validated_data::Field<bool>,
    pub information_option: ::validated_data::Field<bool>,
}

#[::validated_data::data_view]
pub struct IpDhcpSnooping<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub bridging: ::validated_data::Field<bool>,
    pub information_option: ::validated_data::Field<ip_dhcp_snooping::InformationOption<'a, Mode>>,
    pub vlan: ::validated_data::Field<&'a str>,
}

pub mod ip_dhcp_snooping;

#[::validated_data::data_view]
pub struct IpDomainLookup<'a, Mode> {
    pub source_interfaces: ::validated_data::Field<ip_domain_lookup::SourceInterfaces<'a, Mode>>,
}

pub mod ip_domain_lookup;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct IpExtcommunityLists<'a, Mode> (::validated_data::Field<ip_extcommunity_lists::Item<'a, Mode>>);

pub mod ip_extcommunity_lists;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct IpExtcommunityListsRegexp<'a, Mode> (::validated_data::Field<ip_extcommunity_lists_regexp::Item<'a, Mode>>);

pub mod ip_extcommunity_lists_regexp;

#[::validated_data::data_view]
pub struct IpFtpClient<'a, Mode> {
    pub source_interface: ::validated_data::Field<&'a str>,
    pub vrfs: ::validated_data::Field<ip_ftp_client::Vrfs<'a, Mode>>,
}

pub mod ip_ftp_client;

#[::validated_data::data_view]
pub struct IpHardware<'a, Mode> {
    pub fib: ::validated_data::Field<ip_hardware::Fib<'a, Mode>>,
}

pub mod ip_hardware;

#[::validated_data::data_view(indexed_list, primary_key(hostname))]
pub struct IpHosts<'a, Mode> (::validated_data::Field<ip_hosts::Item<'a, Mode>>);

pub mod ip_hosts;

#[::validated_data::data_view]
pub struct IpHttpClient<'a, Mode> {
    pub source_interface: ::validated_data::Field<&'a str>,
    pub vrfs: ::validated_data::Field<ip_http_client::Vrfs<'a, Mode>>,
}

pub mod ip_http_client;

#[::validated_data::data_view]
pub struct IpIgmpSnooping<'a, Mode> {
    pub globally_enabled: ::validated_data::Field<bool>,
    pub robustness_variable: ::validated_data::Field<i64>,
    pub restart_query_interval: ::validated_data::Field<i64>,
    pub interface_restart_query: ::validated_data::Field<i64>,
    pub fast_leave: ::validated_data::Field<bool>,
    pub querier: ::validated_data::Field<ip_igmp_snooping::Querier<'a, Mode>>,
    pub proxy: ::validated_data::Field<bool>,
    pub vlans: ::validated_data::Field<ip_igmp_snooping::Vlans<'a, Mode>>,
}

pub mod ip_igmp_snooping;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct IpLargeCommunityLists<'a, Mode> (::validated_data::Field<ip_large_community_lists::Item<'a, Mode>>);

pub mod ip_large_community_lists;

#[::validated_data::data_view]
pub struct IpNameServer<'a, Mode> {
    pub vrfs: ::validated_data::Field<ip_name_server::Vrfs<'a, Mode>>,
}

pub mod ip_name_server;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct IpNameServerGroups<'a, Mode> (::validated_data::Field<ip_name_server_groups::Item<'a, Mode>>);

pub mod ip_name_server_groups;

#[::validated_data::data_view]
pub struct IpNat<'a, Mode> {
    pub kernel_buffer_size: ::validated_data::Field<i64>,
    pub profiles: ::validated_data::Field<ip_nat::Profiles<'a, Mode>>,
    pub pools: ::validated_data::Field<ip_nat::Pools<'a, Mode>>,
    pub synchronization: ::validated_data::Field<ip_nat::Synchronization<'a, Mode>>,
    pub translation: ::validated_data::Field<ip_nat::Translation<'a, Mode>>,
}

pub mod ip_nat;

#[::validated_data::data_view]
pub struct IpRadius<'a, Mode> {
    pub source_interface: ::validated_data::Field<&'a str>,
    pub vrfs: ::validated_data::Field<ip_radius::Vrfs<'a, Mode>>,
}

pub mod ip_radius;

#[::validated_data::data_view(list)]
pub struct IpRadiusSourceInterfaces<'a, Mode> (::validated_data::Field<ip_radius_source_interfaces::Item<'a, Mode>>);

pub mod ip_radius_source_interfaces;

#[::validated_data::data_view]
pub struct IpSecurity<'a, Mode> {
    pub ike_policies: ::validated_data::Field<ip_security::IkePolicies<'a, Mode>>,
    pub sa_policies: ::validated_data::Field<ip_security::SaPolicies<'a, Mode>>,
    pub profiles: ::validated_data::Field<ip_security::Profiles<'a, Mode>>,
    pub key_controller: ::validated_data::Field<ip_security::KeyController<'a, Mode>>,
    pub hardware_encryption_disabled: ::validated_data::Field<bool>,
    pub connection_tx_interface_match_source_ip: ::validated_data::Field<bool>,
}

pub mod ip_security;

#[::validated_data::data_view]
pub struct IpSoftwareForwarding<'a, Mode> {
    pub mtu: ::validated_data::Field<ip_software_forwarding::Mtu<'a, Mode>>,
}

pub mod ip_software_forwarding;

#[::validated_data::data_view]
pub struct IpSshClient<'a, Mode> {
    pub source_interface: ::validated_data::Field<&'a str>,
    pub vrfs: ::validated_data::Field<ip_ssh_client::Vrfs<'a, Mode>>,
}

pub mod ip_ssh_client;

#[::validated_data::data_view]
pub struct IpTacacs<'a, Mode> {
    pub source_interface: ::validated_data::Field<&'a str>,
    pub vrfs: ::validated_data::Field<ip_tacacs::Vrfs<'a, Mode>>,
}

pub mod ip_tacacs;

#[::validated_data::data_view(list)]
pub struct IpTacacsSourceInterfaces<'a, Mode> (::validated_data::Field<ip_tacacs_source_interfaces::Item<'a, Mode>>);

pub mod ip_tacacs_source_interfaces;

#[::validated_data::data_view]
pub struct IpTelnetClient<'a, Mode> {
    pub source_interface: ::validated_data::Field<&'a str>,
    pub vrfs: ::validated_data::Field<ip_telnet_client::Vrfs<'a, Mode>>,
}

pub mod ip_telnet_client;

#[::validated_data::data_view]
pub struct IpTftpClient<'a, Mode> {
    pub source_interface: ::validated_data::Field<&'a str>,
    pub vrfs: ::validated_data::Field<ip_tftp_client::Vrfs<'a, Mode>>,
}

pub mod ip_tftp_client;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Ipv6AccessLists<'a, Mode> (::validated_data::Field<ipv6_access_lists::Item<'a, Mode>>);

pub mod ipv6_access_lists;

#[::validated_data::data_view]
pub struct Ipv6DhcpRelay<'a, Mode> {
    pub always_on: ::validated_data::Field<bool>,
    pub all_subnets: ::validated_data::Field<bool>,
    pub option: ::validated_data::Field<ipv6_dhcp_relay::Option<'a, Mode>>,
}

pub mod ipv6_dhcp_relay;

#[::validated_data::data_view]
pub struct Ipv6Hardware<'a, Mode> {
    pub fib: ::validated_data::Field<ipv6_hardware::Fib<'a, Mode>>,
}

pub mod ipv6_hardware;

#[::validated_data::data_view]
pub struct Ipv6Neighbor<'a, Mode> {
    pub static_entries: ::validated_data::Field<ipv6_neighbor::StaticEntries<'a, Mode>>,
    pub persistent: ::validated_data::Field<ipv6_neighbor::Persistent<'a, Mode>>,
}

pub mod ipv6_neighbor;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Ipv6PrefixLists<'a, Mode> (::validated_data::Field<ipv6_prefix_lists::Item<'a, Mode>>);

pub mod ipv6_prefix_lists;

#[::validated_data::data_view]
pub struct Ipv6RouterOspf<'a, Mode> {
    pub process_ids: ::validated_data::Field<ipv6_router_ospf::ProcessIds<'a, Mode>>,
}

pub mod ipv6_router_ospf;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Ipv6StandardAccessLists<'a, Mode> (::validated_data::Field<ipv6_standard_access_lists::Item<'a, Mode>>);

pub mod ipv6_standard_access_lists;

#[::validated_data::data_view(list)]
pub struct Ipv6StaticRoutes<'a, Mode> (::validated_data::Field<ipv6_static_routes::Item<'a, Mode>>);

pub mod ipv6_static_routes;

#[::validated_data::data_view]
pub struct Kernel<'a, Mode> {
    pub software_forwarding_ecmp: ::validated_data::Field<bool>,
}

#[::validated_data::data_view]
pub struct L2Protocol<'a, Mode> {
    pub forwarding_profiles: ::validated_data::Field<l2_protocol::ForwardingProfiles<'a, Mode>>,
}

pub mod l2_protocol;

#[::validated_data::data_view]
pub struct Lacp<'a, Mode> {
    pub port_id: ::validated_data::Field<lacp::PortId<'a, Mode>>,
    pub rate_limit: ::validated_data::Field<lacp::RateLimit<'a, Mode>>,
    pub system_priority: ::validated_data::Field<i64>,
}

pub mod lacp;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct LinkTrackingGroups<'a, Mode> (::validated_data::Field<link_tracking_groups::Item<'a, Mode>>);

pub mod link_tracking_groups;

#[::validated_data::data_view]
pub struct Lldp<'a, Mode> {
    pub timer: ::validated_data::Field<i64>,
    pub timer_reinitialization: ::validated_data::Field<i64>,
    pub holdtime: ::validated_data::Field<i64>,
    pub management_address: ::validated_data::Field<&'a str>,
    pub vrf: ::validated_data::Field<&'a str>,
    pub receive_packet_tagged_drop: ::validated_data::Field<bool>,
    pub tlvs: ::validated_data::Field<lldp::Tlvs<'a, Mode>>,
    pub run: ::validated_data::Field<bool>,
}

pub mod lldp;

#[::validated_data::data_view]
pub struct LoadBalance<'a, Mode> {
    pub policies: ::validated_data::Field<load_balance::Policies<'a, Mode>>,
    pub cluster: ::validated_data::Field<load_balance::Cluster<'a, Mode>>,
}

pub mod load_balance;

#[::validated_data::data_view]
pub struct LoadInterval<'a, Mode> {
    pub default: ::validated_data::Field<i64>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct LocalUsers<'a, Mode> (::validated_data::Field<local_users::Item<'a, Mode>>);

pub mod local_users;

#[::validated_data::data_view]
pub struct Logging<'a, Mode> {
    pub console: ::validated_data::Field<&'a str>,
    pub monitor: ::validated_data::Field<&'a str>,
    pub buffered: ::validated_data::Field<logging::Buffered<'a, Mode>>,
    pub repeat_messages: ::validated_data::Field<bool>,
    pub trap: ::validated_data::Field<&'a str>,
    pub synchronous: ::validated_data::Field<logging::Synchronous<'a, Mode>>,
    pub format: ::validated_data::Field<logging::Format<'a, Mode>>,
    pub facility: ::validated_data::Field<&'a str>,
    pub source_interface: ::validated_data::Field<&'a str>,
    pub local_interface: ::validated_data::Field<&'a str>,
    pub vrfs: ::validated_data::Field<logging::Vrfs<'a, Mode>>,
    pub policy: ::validated_data::Field<logging::Policy<'a, Mode>>,
    pub event: ::validated_data::Field<logging::Event<'a, Mode>>,
    pub level: ::validated_data::Field<logging::Level<'a, Mode>>,
}

pub mod logging;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct LoopbackInterfaces<'a, Mode> (::validated_data::Field<loopback_interfaces::Item<'a, Mode>>);

pub mod loopback_interfaces;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct MacAccessLists<'a, Mode> (::validated_data::Field<mac_access_lists::Item<'a, Mode>>);

pub mod mac_access_lists;

#[::validated_data::data_view]
pub struct MacAddressTable<'a, Mode> {
    pub aging_time: ::validated_data::Field<i64>,
    pub notification_host_flap: ::validated_data::Field<mac_address_table::NotificationHostFlap<'a, Mode>>,
    pub static_entries: ::validated_data::Field<mac_address_table::StaticEntries<'a, Mode>>,
}

pub mod mac_address_table;

#[::validated_data::data_view]
pub struct MacSecurity<'a, Mode> {
    pub license: ::validated_data::Field<mac_security::License<'a, Mode>>,
    pub fips_restrictions: ::validated_data::Field<bool>,
    pub profiles: ::validated_data::Field<mac_security::Profiles<'a, Mode>>,
}

pub mod mac_security;

#[::validated_data::data_view]
pub struct Maintenance<'a, Mode> {
    pub default_interface_profile: ::validated_data::Field<&'a str>,
    pub default_bgp_profile: ::validated_data::Field<&'a str>,
    pub default_unit_profile: ::validated_data::Field<&'a str>,
    pub interface_profiles: ::validated_data::Field<maintenance::InterfaceProfiles<'a, Mode>>,
    pub bgp_profiles: ::validated_data::Field<maintenance::BgpProfiles<'a, Mode>>,
    pub unit_profiles: ::validated_data::Field<maintenance::UnitProfiles<'a, Mode>>,
    pub units: ::validated_data::Field<maintenance::Units<'a, Mode>>,
}

pub mod maintenance;

#[::validated_data::data_view]
pub struct ManagementAccounts<'a, Mode> {
    pub password: ::validated_data::Field<management_accounts::Password<'a, Mode>>,
}

pub mod management_accounts;

#[::validated_data::data_view]
pub struct ManagementApiGnmi<'a, Mode> {
    pub provider: ::validated_data::Field<&'a str>,
    pub transport: ::validated_data::Field<management_api_gnmi::Transport<'a, Mode>>,
}

pub mod management_api_gnmi;

#[::validated_data::data_view]
pub struct ManagementApiHttp<'a, Mode> {
    pub enable_http: ::validated_data::Field<bool>,
    pub enable_https: ::validated_data::Field<bool>,
    pub enable_unix: ::validated_data::Field<bool>,
    pub https_ssl_profile: ::validated_data::Field<&'a str>,
    pub default_services: ::validated_data::Field<bool>,
    pub session_timeout: ::validated_data::Field<i64>,
    pub enable_vrfs: ::validated_data::Field<management_api_http::EnableVrfs<'a, Mode>>,
    pub protocol_https_certificate: ::validated_data::Field<management_api_http::ProtocolHttpsCertificate<'a, Mode>>,
}

pub mod management_api_http;

#[::validated_data::data_view]
pub struct ManagementApiModels<'a, Mode> {
    pub provider: ::validated_data::Field<management_api_models::Provider<'a, Mode>>,
}

pub mod management_api_models;

#[::validated_data::data_view]
pub struct ManagementConsole<'a, Mode> {
    pub idle_timeout: ::validated_data::Field<i64>,
}

#[::validated_data::data_view]
pub struct ManagementCvx<'a, Mode> {
    pub shutdown: ::validated_data::Field<bool>,
    pub server_hosts: ::validated_data::Field<management_cvx::ServerHosts<'a, Mode>>,
    pub source_interface: ::validated_data::Field<&'a str>,
    pub vrf: ::validated_data::Field<&'a str>,
}

pub mod management_cvx;

#[::validated_data::data_view]
pub struct ManagementDefaults<'a, Mode> {
    pub secret: ::validated_data::Field<management_defaults::Secret<'a, Mode>>,
}

pub mod management_defaults;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct ManagementInterfaces<'a, Mode> (::validated_data::Field<management_interfaces::Item<'a, Mode>>);

pub mod management_interfaces;

#[::validated_data::data_view]
pub struct ManagementLdap<'a, Mode> {
    pub server_defaults: ::validated_data::Field<management_ldap::ServerDefaults<'a, Mode>>,
    pub server_hosts: ::validated_data::Field<management_ldap::ServerHosts<'a, Mode>>,
    pub group_policies: ::validated_data::Field<management_ldap::GroupPolicies<'a, Mode>>,
}

pub mod management_ldap;

#[::validated_data::data_view]
pub struct ManagementSecurity<'a, Mode> {
    pub auto_certificate: ::validated_data::Field<management_security::AutoCertificate<'a, Mode>>,
    pub entropy_sources: ::validated_data::Field<management_security::EntropySources<'a, Mode>>,
    pub signature_verification: ::validated_data::Field<management_security::SignatureVerification<'a, Mode>>,
    pub password: ::validated_data::Field<management_security::Password<'a, Mode>>,
    pub ssl_profiles: ::validated_data::Field<management_security::SslProfiles<'a, Mode>>,
    pub shared_secret_profiles: ::validated_data::Field<management_security::SharedSecretProfiles<'a, Mode>>,
}

pub mod management_security;

#[::validated_data::data_view]
pub struct ManagementSsh<'a, Mode> {
    pub authentication: ::validated_data::Field<management_ssh::Authentication<'a, Mode>>,
    pub ip_access_group_in: ::validated_data::Field<&'a str>,
    pub ipv6_access_group_in: ::validated_data::Field<&'a str>,
    pub idle_timeout: ::validated_data::Field<i64>,
    pub cipher: ::validated_data::Field<management_ssh::Cipher<'a, Mode>>,
    pub key_exchange: ::validated_data::Field<management_ssh::KeyExchange<'a, Mode>>,
    pub mac: ::validated_data::Field<management_ssh::Mac<'a, Mode>>,
    pub fips_restrictions: ::validated_data::Field<bool>,
    pub hostkey: ::validated_data::Field<management_ssh::Hostkey<'a, Mode>>,
    pub enable: ::validated_data::Field<bool>,
    pub connection: ::validated_data::Field<management_ssh::Connection<'a, Mode>>,
    pub vrfs: ::validated_data::Field<management_ssh::Vrfs<'a, Mode>>,
    pub log_level: ::validated_data::Field<&'a str>,
    pub client_alive: ::validated_data::Field<management_ssh::ClientAlive<'a, Mode>>,
}

pub mod management_ssh;

#[::validated_data::data_view]
pub struct ManagementTechSupport<'a, Mode> {
    pub policy_show_tech_support: ::validated_data::Field<management_tech_support::PolicyShowTechSupport<'a, Mode>>,
}

pub mod management_tech_support;

#[::validated_data::data_view]
pub struct MatchListInput<'a, Mode> {
    pub prefix_ipv4: ::validated_data::Field<match_list_input::PrefixIpv4<'a, Mode>>,
    pub prefix_ipv6: ::validated_data::Field<match_list_input::PrefixIpv6<'a, Mode>>,
    pub string: ::validated_data::Field<match_list_input::String<'a, Mode>>,
}

pub mod match_list_input;

#[::validated_data::data_view]
pub struct McsClient<'a, Mode> {
    pub shutdown: ::validated_data::Field<bool>,
    pub cvx_secondary: ::validated_data::Field<mcs_client::CvxSecondary<'a, Mode>>,
}

pub mod mcs_client;

#[::validated_data::data_view]
pub struct Metadata<'a, Mode> {
    pub is_deployed: ::validated_data::Field<bool>,
    pub platform: ::validated_data::Field<&'a str>,
    pub system_mac_address: ::validated_data::Field<&'a str>,
    pub rack: ::validated_data::Field<&'a str>,
    pub pod_name: ::validated_data::Field<&'a str>,
    pub dc_name: ::validated_data::Field<&'a str>,
    pub fabric_name: ::validated_data::Field<&'a str>,
    pub serial_number: ::validated_data::Field<&'a str>,
    pub validate_hardware: ::validated_data::Field<metadata::ValidateHardware<'a, Mode>>,
    pub cv_tags: ::validated_data::Field<metadata::CvTags<'a, Mode>>,
    pub cv_use_static_config_manifest: ::validated_data::Field<bool>,
    pub cv_pathfinder: ::validated_data::Field<metadata::CvPathfinder<'a, Mode>>,
    pub digital_twin: ::validated_data::Field<metadata::DigitalTwin<'a, Mode>>,
    pub validate_no_errors_period: ::validated_data::Field<i64>,
    pub exclude_as_extra_fabric_validation_target: ::validated_data::Field<bool>,
    pub interfaces: ::validated_data::Field<metadata::Interfaces<'a, Mode>>,
    pub bgp: ::validated_data::Field<metadata::Bgp<'a, Mode>>,
}

pub mod metadata;

#[::validated_data::data_view]
pub struct MlagConfiguration<'a, Mode> {
    pub domain_id: ::validated_data::Field<&'a str>,
    pub heartbeat_interval: ::validated_data::Field<i64>,
    pub local_interface: ::validated_data::Field<&'a str>,
    pub peer_address: ::validated_data::Field<&'a str>,
    pub peer_address_heartbeat: ::validated_data::Field<mlag_configuration::PeerAddressHeartbeat<'a, Mode>>,
    pub dual_primary_detection_delay: ::validated_data::Field<i64>,
    pub dual_primary_recovery_delay_mlag: ::validated_data::Field<i64>,
    pub dual_primary_recovery_delay_non_mlag: ::validated_data::Field<i64>,
    pub peer_link: ::validated_data::Field<&'a str>,
    pub reload_delay_mlag: ::validated_data::Field<&'a str>,
    pub reload_delay_non_mlag: ::validated_data::Field<&'a str>,
}

pub mod mlag_configuration;

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

#[::validated_data::data_view]
pub struct MonitorLayer1<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub logging_mac_fault: ::validated_data::Field<bool>,
    pub logging_transceiver: ::validated_data::Field<monitor_layer1::LoggingTransceiver<'a, Mode>>,
}

pub mod monitor_layer1;

#[::validated_data::data_view]
pub struct MonitorLinkFlapPolicy<'a, Mode> {
    pub damping_profiles: ::validated_data::Field<monitor_link_flap_policy::DampingProfiles<'a, Mode>>,
    pub max_flap_profiles: ::validated_data::Field<monitor_link_flap_policy::MaxFlapProfiles<'a, Mode>>,
    pub default_profiles: ::validated_data::Field<monitor_link_flap_policy::DefaultProfiles<'a, Mode>>,
}

pub mod monitor_link_flap_policy;

#[::validated_data::data_view]
pub struct MonitorLoopProtection<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub disabled_time: ::validated_data::Field<i64>,
    pub protect_vlan: ::validated_data::Field<&'a str>,
    pub rate_limit: ::validated_data::Field<i64>,
    pub transmit_interval: ::validated_data::Field<i64>,
}

#[::validated_data::data_view]
pub struct MonitorServerRadius<'a, Mode> {
    pub service_dot1x: ::validated_data::Field<bool>,
    pub probe: ::validated_data::Field<monitor_server_radius::Probe<'a, Mode>>,
}

pub mod monitor_server_radius;

#[::validated_data::data_view]
pub struct MonitorSessionDefaultEncapsulationGre<'a, Mode> {
    pub payload: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct MonitorSessions<'a, Mode> (::validated_data::Field<monitor_sessions::Item<'a, Mode>>);

pub mod monitor_sessions;

#[::validated_data::data_view]
pub struct MonitorTelemetryInflux<'a, Mode> {
    pub vrf: ::validated_data::Field<&'a str>,
    pub destinations: ::validated_data::Field<monitor_telemetry_influx::Destinations<'a, Mode>>,
    pub source_group_standard_disabled: ::validated_data::Field<bool>,
    pub source_sockets: ::validated_data::Field<monitor_telemetry_influx::SourceSockets<'a, Mode>>,
    pub tags: ::validated_data::Field<monitor_telemetry_influx::Tags<'a, Mode>>,
}

pub mod monitor_telemetry_influx;

#[::validated_data::data_view]
pub struct MonitorTelemetryPostcardPolicy<'a, Mode> {
    pub disabled: ::validated_data::Field<bool>,
    pub ingress: ::validated_data::Field<monitor_telemetry_postcard_policy::Ingress<'a, Mode>>,
    pub marker_vxlan: ::validated_data::Field<monitor_telemetry_postcard_policy::MarkerVxlan<'a, Mode>>,
    pub profiles: ::validated_data::Field<monitor_telemetry_postcard_policy::Profiles<'a, Mode>>,
    pub sample_policies: ::validated_data::Field<monitor_telemetry_postcard_policy::SamplePolicies<'a, Mode>>,
}

pub mod monitor_telemetry_postcard_policy;

#[::validated_data::data_view]
pub struct MonitorTwamp<'a, Mode> {
    pub twamp_light: ::validated_data::Field<monitor_twamp::TwampLight<'a, Mode>>,
}

pub mod monitor_twamp;

#[::validated_data::data_view]
pub struct Mpls<'a, Mode> {
    pub ip: ::validated_data::Field<bool>,
    pub ldp: ::validated_data::Field<mpls::Ldp<'a, Mode>>,
    pub icmp: ::validated_data::Field<mpls::Icmp<'a, Mode>>,
    pub rsvp: ::validated_data::Field<mpls::Rsvp<'a, Mode>>,
    pub label_ranges: ::validated_data::Field<mpls::LabelRanges<'a, Mode>>,
    pub tunnel: ::validated_data::Field<mpls::Tunnel<'a, Mode>>,
}

pub mod mpls;

#[::validated_data::data_view]
pub struct Ntp<'a, Mode> {
    pub local_interface: ::validated_data::Field<ntp::LocalInterface<'a, Mode>>,
    pub servers: ::validated_data::Field<ntp::Servers<'a, Mode>>,
    pub vrf: ::validated_data::Field<&'a str>,
    pub authenticate: ::validated_data::Field<bool>,
    pub authenticate_servers_only: ::validated_data::Field<bool>,
    pub authentication_keys: ::validated_data::Field<ntp::AuthenticationKeys<'a, Mode>>,
    pub trusted_keys: ::validated_data::Field<&'a str>,
    pub serve: ::validated_data::Field<ntp::Serve<'a, Mode>>,
}

pub mod ntp;

#[::validated_data::data_view]
pub struct PatchPanel<'a, Mode> {
    pub connector: ::validated_data::Field<patch_panel::Connector<'a, Mode>>,
    pub patches: ::validated_data::Field<patch_panel::Patches<'a, Mode>>,
}

pub mod patch_panel;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct PeerFilters<'a, Mode> (::validated_data::Field<peer_filters::Item<'a, Mode>>);

pub mod peer_filters;

#[::validated_data::data_view]
pub struct Platform<'a, Mode> {
    pub trident: ::validated_data::Field<platform::Trident<'a, Mode>>,
    pub sand: ::validated_data::Field<platform::Sand<'a, Mode>>,
    pub sfe: ::validated_data::Field<platform::Sfe<'a, Mode>>,
    pub fap: ::validated_data::Field<platform::Fap<'a, Mode>>,
}

pub mod platform;

#[::validated_data::data_view]
pub struct Poe<'a, Mode> {
    pub reboot: ::validated_data::Field<poe::Reboot<'a, Mode>>,
    pub interface_shutdown: ::validated_data::Field<poe::InterfaceShutdown<'a, Mode>>,
}

pub mod poe;

#[::validated_data::data_view]
pub struct PolicyMaps<'a, Mode> {
    pub pbr: ::validated_data::Field<policy_maps::Pbr<'a, Mode>>,
    pub qos: ::validated_data::Field<policy_maps::Qos<'a, Mode>>,
    pub copp_system_policy: ::validated_data::Field<policy_maps::CoppSystemPolicy<'a, Mode>>,
}

pub mod policy_maps;

#[::validated_data::data_view]
pub struct PortChannel<'a, Mode> {
    pub load_balance_trident_udf: ::validated_data::Field<port_channel::LoadBalanceTridentUdf<'a, Mode>>,
    pub load_balance_sand_profile: ::validated_data::Field<&'a str>,
}

pub mod port_channel;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct PortChannelInterfaces<'a, Mode> (::validated_data::Field<port_channel_interfaces::Item<'a, Mode>>);

pub mod port_channel_interfaces;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct PrefixLists<'a, Mode> (::validated_data::Field<prefix_lists::Item<'a, Mode>>);

pub mod prefix_lists;

#[::validated_data::data_view]
pub struct PriorityFlowControl<'a, Mode> {
    pub all_off: ::validated_data::Field<bool>,
    pub watchdog: ::validated_data::Field<priority_flow_control::Watchdog<'a, Mode>>,
}

pub mod priority_flow_control;

#[::validated_data::data_view]
pub struct Ptp<'a, Mode> {
    pub mode: ::validated_data::Field<&'a str>,
    pub profile: ::validated_data::Field<&'a str>,
    pub mode_one_step: ::validated_data::Field<bool>,
    pub forward_unicast: ::validated_data::Field<bool>,
    pub clock_identity: ::validated_data::Field<&'a str>,
    pub source: ::validated_data::Field<ptp::Source<'a, Mode>>,
    pub priority1: ::validated_data::Field<i64>,
    pub priority2: ::validated_data::Field<i64>,
    pub ttl: ::validated_data::Field<i64>,
    pub domain: ::validated_data::Field<i64>,
    pub hold_ptp_time: ::validated_data::Field<i64>,
    pub message_type: ::validated_data::Field<ptp::MessageType<'a, Mode>>,
    pub monitor: ::validated_data::Field<ptp::Monitor<'a, Mode>>,
    pub free_running: ::validated_data::Field<ptp::FreeRunning<'a, Mode>>,
    pub forward_v1: ::validated_data::Field<bool>,
}

pub mod ptp;

#[::validated_data::data_view]
pub struct Qos<'a, Mode> {
    pub map: ::validated_data::Field<qos::Map<'a, Mode>>,
    pub rewrite_dscp: ::validated_data::Field<bool>,
    pub random_detect: ::validated_data::Field<qos::RandomDetect<'a, Mode>>,
    pub tx_queue: ::validated_data::Field<qos::TxQueue<'a, Mode>>,
}

pub mod qos;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct QosProfiles<'a, Mode> (::validated_data::Field<qos_profiles::Item<'a, Mode>>);

pub mod qos_profiles;

#[::validated_data::data_view]
pub struct QueueMonitorLength<'a, Mode> {
    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    pub default_thresholds: ::validated_data::Field<queue_monitor_length::DefaultThresholds<'a, Mode>>,
    pub log: ::validated_data::Field<i64>,
    pub notifying: ::validated_data::Field<bool>,
    pub cpu: ::validated_data::Field<queue_monitor_length::Cpu<'a, Mode>>,
    pub tx_latency: ::validated_data::Field<bool>,
    pub mirror: ::validated_data::Field<queue_monitor_length::Mirror<'a, Mode>>,
}

pub mod queue_monitor_length;

#[::validated_data::data_view]
pub struct QueueMonitorStreaming<'a, Mode> {
    pub enable: ::validated_data::Field<bool>,
    pub ip_access_group: ::validated_data::Field<&'a str>,
    pub ipv6_access_group: ::validated_data::Field<&'a str>,
    pub max_connections: ::validated_data::Field<i64>,
    pub vrf: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct RadiusProxy<'a, Mode> {
    pub client_key: ::validated_data::Field<&'a str>,
    pub client_session_idle_timeout: ::validated_data::Field<i64>,
    pub dynamic_authorization: ::validated_data::Field<bool>,
    pub client_groups: ::validated_data::Field<radius_proxy::ClientGroups<'a, Mode>>,
}

pub mod radius_proxy;

#[::validated_data::data_view]
pub struct RadiusServer<'a, Mode> {
    pub attribute_32_include_in_access_req: ::validated_data::Field<radius_server::Attribute32IncludeInAccessReq<'a, Mode>>,
    pub deadtime: ::validated_data::Field<i64>,
    pub dynamic_authorization: ::validated_data::Field<radius_server::DynamicAuthorization<'a, Mode>>,
    pub servers: ::validated_data::Field<radius_server::Servers<'a, Mode>>,
    pub vrfs: ::validated_data::Field<radius_server::Vrfs<'a, Mode>>,
    pub tls_ssl_profile: ::validated_data::Field<&'a str>,
}

pub mod radius_server;

#[::validated_data::data_view]
pub struct Redundancy<'a, Mode> {
    pub protocol: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Roles<'a, Mode> (::validated_data::Field<roles::Item<'a, Mode>>);

pub mod roles;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct RouteMaps<'a, Mode> (::validated_data::Field<route_maps::Item<'a, Mode>>);

pub mod route_maps;

#[::validated_data::data_view]
pub struct RouterAdaptiveVirtualTopology<'a, Mode> {
    pub topology_role: ::validated_data::Field<&'a str>,
    pub gateway_vxlan: ::validated_data::Field<bool>,
    pub region: ::validated_data::Field<router_adaptive_virtual_topology::Region<'a, Mode>>,
    pub zone: ::validated_data::Field<router_adaptive_virtual_topology::Zone<'a, Mode>>,
    pub site: ::validated_data::Field<router_adaptive_virtual_topology::Site<'a, Mode>>,
    pub profiles: ::validated_data::Field<router_adaptive_virtual_topology::Profiles<'a, Mode>>,
    pub policies: ::validated_data::Field<router_adaptive_virtual_topology::Policies<'a, Mode>>,
    pub vrfs: ::validated_data::Field<router_adaptive_virtual_topology::Vrfs<'a, Mode>>,
}

pub mod router_adaptive_virtual_topology;

#[::validated_data::data_view]
pub struct RouterBfd<'a, Mode> {
    pub interval: ::validated_data::Field<i64>,
    pub local_address: ::validated_data::Field<&'a str>,
    pub min_rx: ::validated_data::Field<i64>,
    pub multiplier: ::validated_data::Field<i64>,
    pub multihop: ::validated_data::Field<router_bfd::Multihop<'a, Mode>>,
    pub session_snapshot_interval: ::validated_data::Field<i64>,
    pub session_snapshot_interval_dangerous: ::validated_data::Field<bool>,
    pub sbfd: ::validated_data::Field<router_bfd::Sbfd<'a, Mode>>,
    pub slow_timer: ::validated_data::Field<i64>,
}

pub mod router_bfd;

#[::validated_data::data_view]
pub struct RouterBgp<'a, Mode> {
    #[data_view(rename = "as")]
    pub field_as: ::validated_data::Field<&'a str>,
    pub as_notation: ::validated_data::Field<&'a str>,
    pub router_id: ::validated_data::Field<&'a str>,
    pub timers: ::validated_data::Field<router_bgp::Timers<'a, Mode>>,
    pub distance: ::validated_data::Field<router_bgp::Distance<'a, Mode>>,
    pub graceful_restart: ::validated_data::Field<router_bgp::GracefulRestart<'a, Mode>>,
    pub graceful_restart_helper: ::validated_data::Field<router_bgp::GracefulRestartHelper<'a, Mode>>,
    pub maximum_paths: ::validated_data::Field<router_bgp::MaximumPaths<'a, Mode>>,
    pub route_distinguisher: ::validated_data::Field<router_bgp::RouteDistinguisher<'a, Mode>>,
    pub updates: ::validated_data::Field<router_bgp::Updates<'a, Mode>>,
    pub bgp_cluster_id: ::validated_data::Field<&'a str>,
    pub bgp_defaults: ::validated_data::Field<router_bgp::BgpDefaults<'a, Mode>>,
    pub bgp: ::validated_data::Field<router_bgp::Bgp<'a, Mode>>,
    pub listen_ranges: ::validated_data::Field<router_bgp::ListenRanges<'a, Mode>>,
    pub neighbor_default: ::validated_data::Field<router_bgp::NeighborDefault<'a, Mode>>,
    pub peer_groups: ::validated_data::Field<router_bgp::PeerGroups<'a, Mode>>,
    pub neighbors: ::validated_data::Field<router_bgp::Neighbors<'a, Mode>>,
    pub neighbor_interfaces: ::validated_data::Field<router_bgp::NeighborInterfaces<'a, Mode>>,
    pub aggregate_addresses: ::validated_data::Field<router_bgp::AggregateAddresses<'a, Mode>>,
    pub redistribute: ::validated_data::Field<router_bgp::Redistribute<'a, Mode>>,
    pub vlan_aware_bundles: ::validated_data::Field<router_bgp::VlanAwareBundles<'a, Mode>>,
    pub vlans: ::validated_data::Field<router_bgp::Vlans<'a, Mode>>,
    pub vpws: ::validated_data::Field<router_bgp::Vpws<'a, Mode>>,
    pub address_family_evpn: ::validated_data::Field<router_bgp::AddressFamilyEvpn<'a, Mode>>,
    pub address_family_rtc: ::validated_data::Field<router_bgp::AddressFamilyRtc<'a, Mode>>,
    pub address_family_ipv4: ::validated_data::Field<router_bgp::AddressFamilyIpv4<'a, Mode>>,
    pub address_family_ipv4_labeled_unicast: ::validated_data::Field<router_bgp::AddressFamilyIpv4LabeledUnicast<'a, Mode>>,
    pub address_family_ipv4_multicast: ::validated_data::Field<router_bgp::AddressFamilyIpv4Multicast<'a, Mode>>,
    pub address_family_ipv4_sr_te: ::validated_data::Field<router_bgp::AddressFamilyIpv4SrTe<'a, Mode>>,
    pub address_family_ipv6: ::validated_data::Field<router_bgp::AddressFamilyIpv6<'a, Mode>>,
    pub address_family_ipv6_multicast: ::validated_data::Field<router_bgp::AddressFamilyIpv6Multicast<'a, Mode>>,
    pub address_family_ipv6_sr_te: ::validated_data::Field<router_bgp::AddressFamilyIpv6SrTe<'a, Mode>>,
    pub address_family_link_state: ::validated_data::Field<router_bgp::AddressFamilyLinkState<'a, Mode>>,
    pub address_family_flow_spec_ipv4: ::validated_data::Field<router_bgp::AddressFamilyFlowSpecIpv4<'a, Mode>>,
    pub address_family_flow_spec_ipv6: ::validated_data::Field<router_bgp::AddressFamilyFlowSpecIpv6<'a, Mode>>,
    pub address_family_path_selection: ::validated_data::Field<router_bgp::AddressFamilyPathSelection<'a, Mode>>,
    pub address_family_vpn_ipv4: ::validated_data::Field<router_bgp::AddressFamilyVpnIpv4<'a, Mode>>,
    pub address_family_vpn_ipv6: ::validated_data::Field<router_bgp::AddressFamilyVpnIpv6<'a, Mode>>,
    pub vrfs: ::validated_data::Field<router_bgp::Vrfs<'a, Mode>>,
    pub session_trackers: ::validated_data::Field<router_bgp::SessionTrackers<'a, Mode>>,
    pub eos_cli: ::validated_data::Field<&'a str>,
}

pub mod router_bgp;

#[::validated_data::data_view]
pub struct RouterGeneral<'a, Mode> {
    pub router_id: ::validated_data::Field<router_general::RouterId<'a, Mode>>,
    pub nexthop_fast_failover: ::validated_data::Field<bool>,
    pub software_forwarding_hardware_offload_mtu: ::validated_data::Field<i64>,
    pub vrfs: ::validated_data::Field<router_general::Vrfs<'a, Mode>>,
    pub control_functions: ::validated_data::Field<router_general::ControlFunctions<'a, Mode>>,
}

pub mod router_general;

#[::validated_data::data_view]
pub struct RouterIgmp<'a, Mode> {
    pub host_proxy_match_mroute: ::validated_data::Field<&'a str>,
    pub ssm_aware: ::validated_data::Field<bool>,
    pub vrfs: ::validated_data::Field<router_igmp::Vrfs<'a, Mode>>,
}

pub mod router_igmp;

#[::validated_data::data_view]
pub struct RouterInternetExit<'a, Mode> {
    pub policies: ::validated_data::Field<router_internet_exit::Policies<'a, Mode>>,
    pub exit_groups: ::validated_data::Field<router_internet_exit::ExitGroups<'a, Mode>>,
}

pub mod router_internet_exit;

#[::validated_data::data_view]
pub struct RouterIsis<'a, Mode> {
    pub instance: ::validated_data::RequiredValue<&'a str, Mode>,
    pub net: ::validated_data::Field<&'a str>,
    pub router_id: ::validated_data::Field<&'a str>,
    pub is_hostname: ::validated_data::Field<&'a str>,
    pub is_type: ::validated_data::Field<&'a str>,
    pub log_adjacency_changes: ::validated_data::Field<bool>,
    pub mpls_ldp_sync_default: ::validated_data::Field<bool>,
    pub timers: ::validated_data::Field<router_isis::Timers<'a, Mode>>,
    pub set_overload_bit: ::validated_data::Field<router_isis::SetOverloadBit<'a, Mode>>,
    pub authentication: ::validated_data::Field<router_isis::Authentication<'a, Mode>>,
    pub advertise: ::validated_data::Field<router_isis::Advertise<'a, Mode>>,
    pub redistribute_routes: ::validated_data::Field<router_isis::RedistributeRoutes<'a, Mode>>,
    pub address_family_ipv4: ::validated_data::Field<router_isis::AddressFamilyIpv4<'a, Mode>>,
    pub address_family_ipv6: ::validated_data::Field<router_isis::AddressFamilyIpv6<'a, Mode>>,
    pub segment_routing_mpls: ::validated_data::Field<router_isis::SegmentRoutingMpls<'a, Mode>>,
    pub spf_interval: ::validated_data::Field<router_isis::SpfInterval<'a, Mode>>,
    pub graceful_restart: ::validated_data::Field<router_isis::GracefulRestart<'a, Mode>>,
    pub eos_cli: ::validated_data::Field<&'a str>,
}

pub mod router_isis;

#[::validated_data::data_view]
pub struct RouterL2Vpn<'a, Mode> {
    pub arp_learning_bridged: ::validated_data::Field<bool>,
    pub arp_proxy: ::validated_data::Field<router_l2_vpn::ArpProxy<'a, Mode>>,
    pub arp_selective_install: ::validated_data::Field<bool>,
    pub nd_learning_bridged: ::validated_data::Field<bool>,
    pub nd_proxy: ::validated_data::Field<router_l2_vpn::NdProxy<'a, Mode>>,
    pub nd_rs_flooding_disabled: ::validated_data::Field<bool>,
    pub virtual_router_nd_ra_flooding_disabled: ::validated_data::Field<bool>,
}

pub mod router_l2_vpn;

#[::validated_data::data_view]
pub struct RouterMsdp<'a, Mode> {
    pub originator_id_local_interface: ::validated_data::Field<&'a str>,
    pub rejected_limit: ::validated_data::Field<i64>,
    pub forward_register_packets: ::validated_data::Field<bool>,
    pub connection_retry_interval: ::validated_data::Field<i64>,
    pub group_limits: ::validated_data::Field<router_msdp::GroupLimits<'a, Mode>>,
    pub peers: ::validated_data::Field<router_msdp::Peers<'a, Mode>>,
    pub vrfs: ::validated_data::Field<router_msdp::Vrfs<'a, Mode>>,
}

pub mod router_msdp;

#[::validated_data::data_view]
pub struct RouterMulticast<'a, Mode> {
    pub ipv4: ::validated_data::Field<router_multicast::Ipv4<'a, Mode>>,
    pub ipv6: ::validated_data::Field<router_multicast::Ipv6<'a, Mode>>,
    pub vrfs: ::validated_data::Field<router_multicast::Vrfs<'a, Mode>>,
}

pub mod router_multicast;

#[::validated_data::data_view]
pub struct RouterOspf<'a, Mode> {
    pub process_ids: ::validated_data::Field<router_ospf::ProcessIds<'a, Mode>>,
}

pub mod router_ospf;

#[::validated_data::data_view]
pub struct RouterOspfv3<'a, Mode> {
    pub router_id: ::validated_data::Field<&'a str>,
    pub passive_interface_default: ::validated_data::Field<bool>,
    pub auto_cost_reference_bandwidth: ::validated_data::Field<i64>,
    pub address_family_ipv4: ::validated_data::Field<router_ospfv3::AddressFamilyIpv4<'a, Mode>>,
    pub address_family_ipv6: ::validated_data::Field<router_ospfv3::AddressFamilyIpv6<'a, Mode>>,
    pub vrfs: ::validated_data::Field<router_ospfv3::Vrfs<'a, Mode>>,
    pub eos_cli: ::validated_data::Field<&'a str>,
}

pub mod router_ospfv3;

#[::validated_data::data_view]
pub struct RouterPathSelection<'a, Mode> {
    pub peer_dynamic_source: ::validated_data::Field<&'a str>,
    pub mtu_discovery_interval: ::validated_data::Field<i64>,
    pub mtu_discovery_hosts: ::validated_data::Field<router_path_selection::MtuDiscoveryHosts<'a, Mode>>,
    pub path_groups: ::validated_data::Field<router_path_selection::PathGroups<'a, Mode>>,
    pub load_balance_policies: ::validated_data::Field<router_path_selection::LoadBalancePolicies<'a, Mode>>,
    pub policies: ::validated_data::Field<router_path_selection::Policies<'a, Mode>>,
    pub vrfs: ::validated_data::Field<router_path_selection::Vrfs<'a, Mode>>,
    pub tcp_mss_ceiling: ::validated_data::Field<router_path_selection::TcpMssCeiling<'a, Mode>>,
    pub interfaces: ::validated_data::Field<router_path_selection::Interfaces<'a, Mode>>,
}

pub mod router_path_selection;

#[::validated_data::data_view]
pub struct RouterPimSparseMode<'a, Mode> {
    pub ipv4: ::validated_data::Field<router_pim_sparse_mode::Ipv4<'a, Mode>>,
    pub vrfs: ::validated_data::Field<router_pim_sparse_mode::Vrfs<'a, Mode>>,
}

pub mod router_pim_sparse_mode;

#[::validated_data::data_view]
pub struct RouterRip<'a, Mode> {
    pub vrfs: ::validated_data::Field<router_rip::Vrfs<'a, Mode>>,
}

pub mod router_rip;

#[::validated_data::data_view]
pub struct RouterSegmentSecurity<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub policies: ::validated_data::Field<router_segment_security::Policies<'a, Mode>>,
    pub vrfs: ::validated_data::Field<router_segment_security::Vrfs<'a, Mode>>,
}

pub mod router_segment_security;

#[::validated_data::data_view]
pub struct RouterServiceInsertion<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub connections: ::validated_data::Field<router_service_insertion::Connections<'a, Mode>>,
}

pub mod router_service_insertion;

#[::validated_data::data_view]
pub struct RouterTrafficEngineering<'a, Mode> {
    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    pub router_id: ::validated_data::Field<router_traffic_engineering::RouterId<'a, Mode>>,
    pub segment_routing: ::validated_data::Field<router_traffic_engineering::SegmentRouting<'a, Mode>>,
    pub twamp_light_sender_profile: ::validated_data::Field<&'a str>,
    pub flex_algos: ::validated_data::Field<router_traffic_engineering::FlexAlgos<'a, Mode>>,
}

pub mod router_traffic_engineering;

#[::validated_data::data_view]
pub struct Schedule<'a, Mode> {
    pub config: ::validated_data::Field<schedule::Config<'a, Mode>>,
    pub jobs: ::validated_data::Field<schedule::Jobs<'a, Mode>>,
}

pub mod schedule;

#[::validated_data::data_view]
pub struct ServiceRoutingConfigurationBgp<'a, Mode> {
    pub no_equals_default: ::validated_data::Field<bool>,
}

#[::validated_data::data_view]
pub struct ServiceUnsupportedTransceiver<'a, Mode> {
    pub license_name: ::validated_data::Field<&'a str>,
    pub license_key: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct Sflow<'a, Mode> {
    pub sample: ::validated_data::Field<i64>,
    pub sample_truncate_size: ::validated_data::Field<i64>,
    pub sample_input_subinterface: ::validated_data::Field<bool>,
    pub sample_output_subinterface: ::validated_data::Field<bool>,
    pub dangerous: ::validated_data::Field<bool>,
    pub polling_interval: ::validated_data::Field<i64>,
    pub vrfs: ::validated_data::Field<sflow::Vrfs<'a, Mode>>,
    pub destinations: ::validated_data::Field<sflow::Destinations<'a, Mode>>,
    pub source: ::validated_data::Field<&'a str>,
    pub source_interface: ::validated_data::Field<&'a str>,
    pub extensions: ::validated_data::Field<sflow::Extensions<'a, Mode>>,
    pub interface: ::validated_data::Field<sflow::Interface<'a, Mode>>,
    pub run: ::validated_data::Field<bool>,
    pub hardware_acceleration: ::validated_data::Field<sflow::HardwareAcceleration<'a, Mode>>,
}

pub mod sflow;

#[::validated_data::data_view]
pub struct SnmpServer<'a, Mode> {
    pub engine_ids: ::validated_data::Field<snmp_server::EngineIds<'a, Mode>>,
    pub extensions: ::validated_data::Field<snmp_server::Extensions<'a, Mode>>,
    pub contact: ::validated_data::Field<&'a str>,
    pub location: ::validated_data::Field<&'a str>,
    pub communities: ::validated_data::Field<snmp_server::Communities<'a, Mode>>,
    pub ipv4_acls: ::validated_data::Field<snmp_server::Ipv4Acls<'a, Mode>>,
    pub ipv6_acls: ::validated_data::Field<snmp_server::Ipv6Acls<'a, Mode>>,
    pub local_interfaces: ::validated_data::Field<snmp_server::LocalInterfaces<'a, Mode>>,
    pub views: ::validated_data::Field<snmp_server::Views<'a, Mode>>,
    pub groups: ::validated_data::Field<snmp_server::Groups<'a, Mode>>,
    pub users: ::validated_data::Field<snmp_server::Users<'a, Mode>>,
    pub hosts: ::validated_data::Field<snmp_server::Hosts<'a, Mode>>,
    pub traps: ::validated_data::Field<snmp_server::Traps<'a, Mode>>,
    pub vrfs: ::validated_data::Field<snmp_server::Vrfs<'a, Mode>>,
    pub ifmib_ifspeed_shape_rate: ::validated_data::Field<bool>,
}

pub mod snmp_server;

#[::validated_data::data_view]
pub struct SpanningTree<'a, Mode> {
    pub root_super: ::validated_data::Field<bool>,
    pub edge_port: ::validated_data::Field<spanning_tree::EdgePort<'a, Mode>>,
    pub mode: ::validated_data::Field<&'a str>,
    pub bpduguard_rate_limit: ::validated_data::Field<spanning_tree::BpduguardRateLimit<'a, Mode>>,
    pub rstp_priority: ::validated_data::Field<i64>,
    pub mst: ::validated_data::Field<spanning_tree::Mst<'a, Mode>>,
    pub mst_instances: ::validated_data::Field<spanning_tree::MstInstances<'a, Mode>>,
    pub no_spanning_tree_vlan: ::validated_data::Field<&'a str>,
    pub rapid_pvst_instances: ::validated_data::Field<spanning_tree::RapidPvstInstances<'a, Mode>>,
    pub port_id_allocation_port_channel_range: ::validated_data::Field<spanning_tree::PortIdAllocationPortChannelRange<'a, Mode>>,
    pub loop_guard_default: ::validated_data::Field<bool>,
}

pub mod spanning_tree;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct StandardAccessLists<'a, Mode> (::validated_data::Field<standard_access_lists::Item<'a, Mode>>);

pub mod standard_access_lists;

#[::validated_data::data_view(list)]
pub struct StaticRoutes<'a, Mode> (::validated_data::Field<static_routes::Item<'a, Mode>>);

pub mod static_routes;

#[::validated_data::data_view]
pub struct Stun<'a, Mode> {
    pub client: ::validated_data::Field<stun::Client<'a, Mode>>,
    pub server: ::validated_data::Field<stun::Server<'a, Mode>>,
}

pub mod stun;

#[::validated_data::data_view]
pub struct SwitchportDefault<'a, Mode> {
    pub mode: ::validated_data::Field<&'a str>,
    pub phone: ::validated_data::Field<switchport_default::Phone<'a, Mode>>,
}

pub mod switchport_default;

#[::validated_data::data_view]
pub struct SwitchportPortSecurity<'a, Mode> {
    pub mac_address: ::validated_data::Field<switchport_port_security::MacAddress<'a, Mode>>,
    pub persistence_disabled: ::validated_data::Field<bool>,
    pub violation_protect_chip_based: ::validated_data::Field<bool>,
}

pub mod switchport_port_security;

#[::validated_data::data_view]
pub struct SyncE<'a, Mode> {
    pub network_option: ::validated_data::RequiredValue<i64, Mode>,
}

#[::validated_data::data_view]
pub struct System<'a, Mode> {
    pub control_plane: ::validated_data::Field<system::ControlPlane<'a, Mode>>,
    pub l1: ::validated_data::Field<system::L1<'a, Mode>>,
    pub mac_address: ::validated_data::Field<&'a str>,
}

pub mod system;

#[::validated_data::data_view]
pub struct TacacsServers<'a, Mode> {
    pub timeout: ::validated_data::Field<i64>,
    pub hosts: ::validated_data::Field<tacacs_servers::Hosts<'a, Mode>>,
    pub policy_unknown_mandatory_attribute_ignore: ::validated_data::Field<bool>,
}

pub mod tacacs_servers;

#[::validated_data::data_view]
pub struct TapAggregation<'a, Mode> {
    pub mode: ::validated_data::Field<tap_aggregation::Mode<'a, Mode>>,
    pub encapsulation_dot1br_strip: ::validated_data::Field<bool>,
    pub encapsulation_vn_tag_strip: ::validated_data::Field<bool>,
    pub protocol_lldp_trap: ::validated_data::Field<bool>,
    pub truncation_size: ::validated_data::Field<i64>,
    pub mac: ::validated_data::Field<tap_aggregation::Mac<'a, Mode>>,
}

pub mod tap_aggregation;

#[::validated_data::data_view]
pub struct TcamProfile<'a, Mode> {
    pub system: ::validated_data::Field<&'a str>,
    pub profiles: ::validated_data::Field<tcam_profile::Profiles<'a, Mode>>,
}

pub mod tcam_profile;

#[::validated_data::data_view]
pub struct Terminal<'a, Mode> {
    pub length: ::validated_data::Field<i64>,
    pub width: ::validated_data::Field<i64>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Trackers<'a, Mode> (::validated_data::Field<trackers::Item<'a, Mode>>);

pub mod trackers;

#[::validated_data::data_view]
pub struct TrafficPolicies<'a, Mode> {
    pub cpu_traffic_policy: ::validated_data::Field<traffic_policies::CpuTrafficPolicy<'a, Mode>>,
    pub vrfs: ::validated_data::Field<traffic_policies::Vrfs<'a, Mode>>,
    pub options: ::validated_data::Field<traffic_policies::Options<'a, Mode>>,
    pub field_sets: ::validated_data::Field<traffic_policies::FieldSets<'a, Mode>>,
    pub policies: ::validated_data::Field<traffic_policies::Policies<'a, Mode>>,
}

pub mod traffic_policies;

#[::validated_data::data_view]
pub struct Transceiver<'a, Mode> {
    pub dom_threshold_file: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct TunnelInterfaces<'a, Mode> (::validated_data::Field<tunnel_interfaces::Item<'a, Mode>>);

pub mod tunnel_interfaces;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct VirtualSourceNatVrfs<'a, Mode> (::validated_data::Field<virtual_source_nat_vrfs::Item<'a, Mode>>);

pub mod virtual_source_nat_vrfs;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct VlanInterfaces<'a, Mode> (::validated_data::Field<vlan_interfaces::Item<'a, Mode>>);

pub mod vlan_interfaces;

#[::validated_data::data_view]
pub struct VlanInternalOrder<'a, Mode> {
    pub allocation: ::validated_data::RequiredValue<&'a str, Mode>,
    pub range: ::validated_data::RequiredValue<vlan_internal_order::Range<'a, Mode>, Mode>,
}

pub mod vlan_internal_order;

#[::validated_data::data_view(indexed_list, primary_key(id))]
pub struct Vlans<'a, Mode> (::validated_data::Field<vlans::Item<'a, Mode>>);

pub mod vlans;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct VmtracerSessions<'a, Mode> (::validated_data::Field<vmtracer_sessions::Item<'a, Mode>>);

pub mod vmtracer_sessions;

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs;

#[::validated_data::data_view]
pub struct VxlanInterface<'a, Mode> {
    pub vxlan1: ::validated_data::Field<vxlan_interface::Vxlan1<'a, Mode>>,
}

pub mod vxlan_interface;
