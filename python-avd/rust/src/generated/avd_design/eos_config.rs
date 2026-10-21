// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct EosConfig<'a, Mode> {
    pub aaa_accounting: ::data_views::Field<AaaAccounting<'a, Mode>>,
    pub aaa_authentication: ::data_views::Field<AaaAuthentication<'a, Mode>>,
    pub aaa_authorization: ::data_views::Field<AaaAuthorization<'a, Mode>>,
    pub aaa_root: ::data_views::Field<AaaRoot<'a, Mode>>,
    pub aaa_server_groups: ::data_views::Field<AaaServerGroups<'a, Mode>>,
    pub access_lists: ::data_views::Field<AccessLists<'a, Mode>>,
    pub address_locking: ::data_views::Field<AddressLocking<'a, Mode>>,
    pub agents: ::data_views::Field<Agents<'a, Mode>>,
    pub aliases: ::data_views::Field<&'a str>,
    pub application_traffic_recognition: ::data_views::Field<ApplicationTrafficRecognition<'a, Mode>>,
    pub arp: ::data_views::Field<Arp<'a, Mode>>,
    pub as_path: ::data_views::Field<AsPath<'a, Mode>>,
    pub avd_structured_config_file_format: ::data_views::Field<&'a str>,
    pub avd_vault_id: ::data_views::Field<&'a str>,
    pub banners: ::data_views::Field<Banners<'a, Mode>>,
    pub bgp_groups: ::data_views::Field<BgpGroups<'a, Mode>>,
    pub boot: ::data_views::Field<Boot<'a, Mode>>,
    pub cfm: ::data_views::Field<Cfm<'a, Mode>>,
    pub class_maps: ::data_views::Field<ClassMaps<'a, Mode>>,
    pub clock: ::data_views::Field<Clock<'a, Mode>>,
    pub config_comment: ::data_views::Field<&'a str>,
    pub config_end: ::data_views::Field<bool>,
    pub custom_templates: ::data_views::Field<CustomTemplates<'a, Mode>>,
    pub cvx: ::data_views::Field<Cvx<'a, Mode>>,
    pub daemon_terminattr: ::data_views::Field<DaemonTerminattr<'a, Mode>>,
    pub daemons: ::data_views::Field<Daemons<'a, Mode>>,
    pub dhcp_relay: ::data_views::Field<DhcpRelay<'a, Mode>>,
    pub dhcp_servers: ::data_views::Field<DhcpServers<'a, Mode>>,
    pub dns_domain: ::data_views::Field<&'a str>,
    pub domain_list: ::data_views::Field<DomainList<'a, Mode>>,
    pub dot1x: ::data_views::Field<Dot1x<'a, Mode>>,
    pub dps_interfaces: ::data_views::Field<DpsInterfaces<'a, Mode>>,
    pub dynamic_prefix_lists: ::data_views::Field<DynamicPrefixLists<'a, Mode>>,
    pub enable_password: ::data_views::Field<EnablePassword<'a, Mode>>,
    pub environment_fan_speed: ::data_views::Field<EnvironmentFanSpeed<'a, Mode>>,
    pub eos_cli: ::data_views::Field<&'a str>,
    pub eos_cli_config_gen_configuration: ::data_views::Field<EosCliConfigGenConfiguration<'a, Mode>>,
    pub eos_cli_config_gen_documentation: ::data_views::Field<EosCliConfigGenDocumentation<'a, Mode>>,
    pub eos_cli_config_gen_keep_tmp_files: ::data_views::Field<bool>,
    pub eos_cli_config_gen_tmp_dir: ::data_views::Field<&'a str>,
    pub eos_cli_config_gen_validate_inputs_batch_size: ::data_views::Field<i64>,
    pub eos_config_future: ::data_views::Field<EosConfigFuture<'a, Mode>>,
    pub errdisable: ::data_views::Field<Errdisable<'a, Mode>>,
    pub ethernet_interfaces: ::data_views::Field<EthernetInterfaces<'a, Mode>>,
    pub event_handlers: ::data_views::Field<EventHandlers<'a, Mode>>,
    pub event_monitor: ::data_views::Field<EventMonitor<'a, Mode>>,
    pub flow_tracking: ::data_views::Field<FlowTracking<'a, Mode>>,
    pub hardware: ::data_views::Field<Hardware<'a, Mode>>,
    pub hardware_counters: ::data_views::Field<HardwareCounters<'a, Mode>>,
    pub hostname: ::data_views::Field<&'a str>,
    pub interface_defaults: ::data_views::Field<InterfaceDefaults<'a, Mode>>,
    pub interface_groups: ::data_views::Field<InterfaceGroups<'a, Mode>>,
    pub interface_profiles: ::data_views::Field<InterfaceProfiles<'a, Mode>>,
    pub ip_access_lists: ::data_views::Field<IpAccessLists<'a, Mode>>,
    pub ip_access_lists_max_entries: ::data_views::Field<i64>,
    pub ip_community_lists: ::data_views::Field<IpCommunityLists<'a, Mode>>,
    pub ip_dhcp_relay: ::data_views::Field<IpDhcpRelay<'a, Mode>>,
    pub ip_dhcp_snooping: ::data_views::Field<IpDhcpSnooping<'a, Mode>>,
    pub ip_domain_lookup: ::data_views::Field<IpDomainLookup<'a, Mode>>,
    pub ip_extcommunity_lists: ::data_views::Field<IpExtcommunityLists<'a, Mode>>,
    pub ip_extcommunity_lists_regexp: ::data_views::Field<IpExtcommunityListsRegexp<'a, Mode>>,
    pub ip_ftp_client: ::data_views::Field<IpFtpClient<'a, Mode>>,
    pub ip_hardware: ::data_views::Field<IpHardware<'a, Mode>>,
    pub ip_hosts: ::data_views::Field<IpHosts<'a, Mode>>,
    pub ip_http_client: ::data_views::Field<IpHttpClient<'a, Mode>>,
    pub ip_icmp_redirect: ::data_views::Field<bool>,
    pub ip_igmp_snooping: ::data_views::Field<IpIgmpSnooping<'a, Mode>>,
    pub ip_large_community_lists: ::data_views::Field<IpLargeCommunityLists<'a, Mode>>,
    pub ip_name_server: ::data_views::Field<IpNameServer<'a, Mode>>,
    pub ip_name_server_groups: ::data_views::Field<IpNameServerGroups<'a, Mode>>,
    pub ip_nat: ::data_views::Field<IpNat<'a, Mode>>,
    pub ip_ospf_router_id_output_format_hostnames: ::data_views::Field<bool>,
    pub ip_radius: ::data_views::Field<IpRadius<'a, Mode>>,
    pub ip_radius_source_interfaces: ::data_views::Field<IpRadiusSourceInterfaces<'a, Mode>>,
    pub ip_routing: ::data_views::Field<bool>,
    pub ip_routing_ipv6_interfaces: ::data_views::Field<bool>,
    pub ip_security: ::data_views::Field<IpSecurity<'a, Mode>>,
    pub ip_software_forwarding: ::data_views::Field<IpSoftwareForwarding<'a, Mode>>,
    pub ip_ssh_client: ::data_views::Field<IpSshClient<'a, Mode>>,
    pub ip_tacacs: ::data_views::Field<IpTacacs<'a, Mode>>,
    pub ip_tacacs_source_interfaces: ::data_views::Field<IpTacacsSourceInterfaces<'a, Mode>>,
    pub ip_telnet_client: ::data_views::Field<IpTelnetClient<'a, Mode>>,
    pub ip_tftp_client: ::data_views::Field<IpTftpClient<'a, Mode>>,
    pub ip_virtual_router_mac_address: ::data_views::Field<&'a str>,
    pub ip_virtual_router_mac_address_advertisement_interval: ::data_views::Field<i64>,
    pub ip_virtual_router_mac_address_mlag_peer: ::data_views::Field<bool>,
    pub ipv6_access_lists: ::data_views::Field<Ipv6AccessLists<'a, Mode>>,
    pub ipv6_dhcp_relay: ::data_views::Field<Ipv6DhcpRelay<'a, Mode>>,
    pub ipv6_hardware: ::data_views::Field<Ipv6Hardware<'a, Mode>>,
    pub ipv6_icmp_redirect: ::data_views::Field<bool>,
    pub ipv6_neighbor: ::data_views::Field<Ipv6Neighbor<'a, Mode>>,
    pub ipv6_prefix_lists: ::data_views::Field<Ipv6PrefixLists<'a, Mode>>,
    pub ipv6_router_ospf: ::data_views::Field<Ipv6RouterOspf<'a, Mode>>,
    pub ipv6_standard_access_lists: ::data_views::Field<Ipv6StandardAccessLists<'a, Mode>>,
    pub ipv6_static_routes: ::data_views::Field<Ipv6StaticRoutes<'a, Mode>>,
    pub ipv6_unicast_routing: ::data_views::Field<bool>,
    pub kernel: ::data_views::Field<Kernel<'a, Mode>>,
    pub l2_protocol: ::data_views::Field<L2Protocol<'a, Mode>>,
    pub lacp: ::data_views::Field<Lacp<'a, Mode>>,
    pub link_tracking_groups: ::data_views::Field<LinkTrackingGroups<'a, Mode>>,
    pub lldp: ::data_views::Field<Lldp<'a, Mode>>,
    pub load_balance: ::data_views::Field<LoadBalance<'a, Mode>>,
    pub load_interval: ::data_views::Field<LoadInterval<'a, Mode>>,
    pub local_users: ::data_views::Field<LocalUsers<'a, Mode>>,
    pub logging: ::data_views::Field<Logging<'a, Mode>>,
    pub loopback_interfaces: ::data_views::Field<LoopbackInterfaces<'a, Mode>>,
    pub mac_access_lists: ::data_views::Field<MacAccessLists<'a, Mode>>,
    pub mac_address_table: ::data_views::Field<MacAddressTable<'a, Mode>>,
    pub mac_security: ::data_views::Field<MacSecurity<'a, Mode>>,
    pub maintenance: ::data_views::Field<Maintenance<'a, Mode>>,
    pub management_accounts: ::data_views::Field<ManagementAccounts<'a, Mode>>,
    pub management_api_gnmi: ::data_views::Field<ManagementApiGnmi<'a, Mode>>,
    pub management_api_http: ::data_views::Field<ManagementApiHttp<'a, Mode>>,
    pub management_api_models: ::data_views::Field<ManagementApiModels<'a, Mode>>,
    pub management_console: ::data_views::Field<ManagementConsole<'a, Mode>>,
    pub management_cvx: ::data_views::Field<ManagementCvx<'a, Mode>>,
    pub management_defaults: ::data_views::Field<ManagementDefaults<'a, Mode>>,
    pub management_interfaces: ::data_views::Field<ManagementInterfaces<'a, Mode>>,
    pub management_ldap: ::data_views::Field<ManagementLdap<'a, Mode>>,
    pub management_security: ::data_views::Field<ManagementSecurity<'a, Mode>>,
    pub management_ssh: ::data_views::Field<ManagementSsh<'a, Mode>>,
    pub management_tech_support: ::data_views::Field<ManagementTechSupport<'a, Mode>>,
    pub match_list_input: ::data_views::Field<MatchListInput<'a, Mode>>,
    pub mcs_client: ::data_views::Field<McsClient<'a, Mode>>,
    pub metadata: ::data_views::Field<Metadata<'a, Mode>>,
    pub mlag_configuration: ::data_views::Field<MlagConfiguration<'a, Mode>>,
    pub monitor_connectivity: ::data_views::Field<MonitorConnectivity<'a, Mode>>,
    pub monitor_layer1: ::data_views::Field<MonitorLayer1<'a, Mode>>,
    pub monitor_link_flap_policy: ::data_views::Field<MonitorLinkFlapPolicy<'a, Mode>>,
    pub monitor_loop_protection: ::data_views::Field<MonitorLoopProtection<'a, Mode>>,
    pub monitor_server_radius: ::data_views::Field<MonitorServerRadius<'a, Mode>>,
    pub monitor_session_default_encapsulation_gre: ::data_views::Field<MonitorSessionDefaultEncapsulationGre<'a, Mode>>,
    pub monitor_sessions: ::data_views::Field<MonitorSessions<'a, Mode>>,
    pub monitor_telemetry_influx: ::data_views::Field<MonitorTelemetryInflux<'a, Mode>>,
    pub monitor_telemetry_postcard_policy: ::data_views::Field<MonitorTelemetryPostcardPolicy<'a, Mode>>,
    pub monitor_twamp: ::data_views::Field<MonitorTwamp<'a, Mode>>,
    pub mpls: ::data_views::Field<Mpls<'a, Mode>>,
    pub ntp: ::data_views::Field<Ntp<'a, Mode>>,
    pub patch_panel: ::data_views::Field<PatchPanel<'a, Mode>>,
    pub peer_filters: ::data_views::Field<PeerFilters<'a, Mode>>,
    pub platform: ::data_views::Field<Platform<'a, Mode>>,
    pub poe: ::data_views::Field<Poe<'a, Mode>>,
    pub policy_maps: ::data_views::Field<PolicyMaps<'a, Mode>>,
    pub port_channel: ::data_views::Field<PortChannel<'a, Mode>>,
    pub port_channel_interfaces: ::data_views::Field<PortChannelInterfaces<'a, Mode>>,
    pub prefix_lists: ::data_views::Field<PrefixLists<'a, Mode>>,
    pub priority_flow_control: ::data_views::Field<PriorityFlowControl<'a, Mode>>,
    pub prompt: ::data_views::Field<&'a str>,
    pub ptp: ::data_views::Field<Ptp<'a, Mode>>,
    pub qos: ::data_views::Field<Qos<'a, Mode>>,
    pub qos_profiles: ::data_views::Field<QosProfiles<'a, Mode>>,
    pub queue_monitor_length: ::data_views::Field<QueueMonitorLength<'a, Mode>>,
    pub queue_monitor_streaming: ::data_views::Field<QueueMonitorStreaming<'a, Mode>>,
    pub radius_proxy: ::data_views::Field<RadiusProxy<'a, Mode>>,
    pub radius_server: ::data_views::Field<RadiusServer<'a, Mode>>,
    pub read_structured_config_from_file: ::data_views::Field<bool>,
    pub redundancy: ::data_views::Field<Redundancy<'a, Mode>>,
    pub roles: ::data_views::Field<Roles<'a, Mode>>,
    pub route_maps: ::data_views::Field<RouteMaps<'a, Mode>>,
    pub router_adaptive_virtual_topology: ::data_views::Field<RouterAdaptiveVirtualTopology<'a, Mode>>,
    pub router_bfd: ::data_views::Field<RouterBfd<'a, Mode>>,
    pub router_bgp: ::data_views::Field<RouterBgp<'a, Mode>>,
    pub router_general: ::data_views::Field<RouterGeneral<'a, Mode>>,
    pub router_igmp: ::data_views::Field<RouterIgmp<'a, Mode>>,
    pub router_internet_exit: ::data_views::Field<RouterInternetExit<'a, Mode>>,
    pub router_isis: ::data_views::Field<RouterIsis<'a, Mode>>,
    pub router_l2_vpn: ::data_views::Field<RouterL2Vpn<'a, Mode>>,
    pub router_msdp: ::data_views::Field<RouterMsdp<'a, Mode>>,
    pub router_multicast: ::data_views::Field<RouterMulticast<'a, Mode>>,
    pub router_ospf: ::data_views::Field<RouterOspf<'a, Mode>>,
    pub router_ospfv3: ::data_views::Field<RouterOspfv3<'a, Mode>>,
    pub router_path_selection: ::data_views::Field<RouterPathSelection<'a, Mode>>,
    pub router_pim_sparse_mode: ::data_views::Field<RouterPimSparseMode<'a, Mode>>,
    pub router_rip: ::data_views::Field<RouterRip<'a, Mode>>,
    pub router_segment_security: ::data_views::Field<RouterSegmentSecurity<'a, Mode>>,
    pub router_service_insertion: ::data_views::Field<RouterServiceInsertion<'a, Mode>>,
    pub router_traffic_engineering: ::data_views::Field<RouterTrafficEngineering<'a, Mode>>,
    pub schedule: ::data_views::Field<Schedule<'a, Mode>>,
    pub service_routing_configuration_bgp: ::data_views::Field<ServiceRoutingConfigurationBgp<'a, Mode>>,
    pub service_routing_protocols_model: ::data_views::Field<&'a str>,
    pub service_unsupported_transceiver: ::data_views::Field<ServiceUnsupportedTransceiver<'a, Mode>>,
    pub sflow: ::data_views::Field<Sflow<'a, Mode>>,
    pub snmp_server: ::data_views::Field<SnmpServer<'a, Mode>>,
    pub spanning_tree: ::data_views::Field<SpanningTree<'a, Mode>>,
    pub standard_access_lists: ::data_views::Field<StandardAccessLists<'a, Mode>>,
    pub static_routes: ::data_views::Field<StaticRoutes<'a, Mode>>,
    pub stun: ::data_views::Field<Stun<'a, Mode>>,
    pub switchport_default: ::data_views::Field<SwitchportDefault<'a, Mode>>,
    pub switchport_ethernet_llc_validation: ::data_views::Field<bool>,
    pub switchport_port_security: ::data_views::Field<SwitchportPortSecurity<'a, Mode>>,
    pub switchport_vlan_tag_validation: ::data_views::Field<bool>,
    pub sync_e: ::data_views::Field<SyncE<'a, Mode>>,
    pub system: ::data_views::Field<System<'a, Mode>>,
    pub tacacs_servers: ::data_views::Field<TacacsServers<'a, Mode>>,
    pub tap_aggregation: ::data_views::Field<TapAggregation<'a, Mode>>,
    pub tcam_profile: ::data_views::Field<TcamProfile<'a, Mode>>,
    pub terminal: ::data_views::Field<Terminal<'a, Mode>>,
    pub trackers: ::data_views::Field<Trackers<'a, Mode>>,
    pub traffic_policies: ::data_views::Field<TrafficPolicies<'a, Mode>>,
    pub transceiver: ::data_views::Field<Transceiver<'a, Mode>>,
    pub transceiver_qsfp_default_mode_4x10: ::data_views::Field<bool>,
    pub tunnel_interfaces: ::data_views::Field<TunnelInterfaces<'a, Mode>>,
    pub virtual_source_nat_vrfs: ::data_views::Field<VirtualSourceNatVrfs<'a, Mode>>,
    pub vlan_interfaces: ::data_views::Field<VlanInterfaces<'a, Mode>>,
    pub vlan_internal_order: ::data_views::Field<VlanInternalOrder<'a, Mode>>,
    pub vlans: ::data_views::Field<Vlans<'a, Mode>>,
    pub vmtracer_sessions: ::data_views::Field<VmtracerSessions<'a, Mode>>,
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
    pub vrrp: ::data_views::Field<Vrrp<'a, Mode>>,
    pub vxlan_interface: ::data_views::Field<VxlanInterface<'a, Mode>>,
}

pub mod aaa_accounting;
pub use aaa_accounting::AaaAccounting;

pub mod aaa_authentication;
pub use aaa_authentication::AaaAuthentication;

pub mod aaa_authorization;
pub use aaa_authorization::AaaAuthorization;

pub mod aaa_root;
pub use aaa_root::AaaRoot;

pub mod aaa_server_groups;
pub use aaa_server_groups::AaaServerGroups;

pub mod access_lists;
pub use access_lists::AccessLists;

pub mod address_locking;
pub use address_locking::AddressLocking;

pub mod agents;
pub use agents::Agents;

pub mod application_traffic_recognition;
pub use application_traffic_recognition::ApplicationTrafficRecognition;

pub mod arp;
pub use arp::Arp;

pub mod as_path;
pub use as_path::AsPath;

pub mod banners;
pub use banners::Banners;

pub mod bgp_groups;
pub use bgp_groups::BgpGroups;

pub mod boot;
pub use boot::Boot;

pub mod cfm;
pub use cfm::Cfm;

pub mod class_maps;
pub use class_maps::ClassMaps;

pub mod clock;
pub use clock::Clock;

pub mod custom_templates;
pub use custom_templates::CustomTemplates;

pub mod cvx;
pub use cvx::Cvx;

pub mod daemon_terminattr;
pub use daemon_terminattr::DaemonTerminattr;

pub mod daemons;
pub use daemons::Daemons;

pub mod dhcp_relay;
pub use dhcp_relay::DhcpRelay;

pub mod dhcp_servers;
pub use dhcp_servers::DhcpServers;

pub mod domain_list;
pub use domain_list::DomainList;

pub mod dot1x;
pub use dot1x::Dot1x;

pub mod dps_interfaces;
pub use dps_interfaces::DpsInterfaces;

pub mod dynamic_prefix_lists;
pub use dynamic_prefix_lists::DynamicPrefixLists;

pub mod enable_password;
pub use enable_password::EnablePassword;

pub mod environment_fan_speed;
pub use environment_fan_speed::EnvironmentFanSpeed;

pub mod eos_cli_config_gen_configuration;
pub use eos_cli_config_gen_configuration::EosCliConfigGenConfiguration;

pub mod eos_cli_config_gen_documentation;
pub use eos_cli_config_gen_documentation::EosCliConfigGenDocumentation;

pub mod eos_config_future;
pub use eos_config_future::EosConfigFuture;

pub mod errdisable;
pub use errdisable::Errdisable;

pub mod ethernet_interfaces;
pub use ethernet_interfaces::EthernetInterfaces;

pub mod event_handlers;
pub use event_handlers::EventHandlers;

pub mod event_monitor;
pub use event_monitor::EventMonitor;

pub mod flow_tracking;
pub use flow_tracking::FlowTracking;

pub mod hardware;
pub use hardware::Hardware;

pub mod hardware_counters;
pub use hardware_counters::HardwareCounters;

pub mod interface_defaults;
pub use interface_defaults::InterfaceDefaults;

pub mod interface_groups;
pub use interface_groups::InterfaceGroups;

pub mod interface_profiles;
pub use interface_profiles::InterfaceProfiles;

pub mod ip_access_lists;
pub use ip_access_lists::IpAccessLists;

pub mod ip_community_lists;
pub use ip_community_lists::IpCommunityLists;

pub mod ip_dhcp_relay;
pub use ip_dhcp_relay::IpDhcpRelay;

pub mod ip_dhcp_snooping;
pub use ip_dhcp_snooping::IpDhcpSnooping;

pub mod ip_domain_lookup;
pub use ip_domain_lookup::IpDomainLookup;

pub mod ip_extcommunity_lists;
pub use ip_extcommunity_lists::IpExtcommunityLists;

pub mod ip_extcommunity_lists_regexp;
pub use ip_extcommunity_lists_regexp::IpExtcommunityListsRegexp;

pub mod ip_ftp_client;
pub use ip_ftp_client::IpFtpClient;

pub mod ip_hardware;
pub use ip_hardware::IpHardware;

pub mod ip_hosts;
pub use ip_hosts::IpHosts;

pub mod ip_http_client;
pub use ip_http_client::IpHttpClient;

pub mod ip_igmp_snooping;
pub use ip_igmp_snooping::IpIgmpSnooping;

pub mod ip_large_community_lists;
pub use ip_large_community_lists::IpLargeCommunityLists;

pub mod ip_name_server;
pub use ip_name_server::IpNameServer;

pub mod ip_name_server_groups;
pub use ip_name_server_groups::IpNameServerGroups;

pub mod ip_nat;
pub use ip_nat::IpNat;

pub mod ip_radius;
pub use ip_radius::IpRadius;

pub mod ip_radius_source_interfaces;
pub use ip_radius_source_interfaces::IpRadiusSourceInterfaces;

pub mod ip_security;
pub use ip_security::IpSecurity;

pub mod ip_software_forwarding;
pub use ip_software_forwarding::IpSoftwareForwarding;

pub mod ip_ssh_client;
pub use ip_ssh_client::IpSshClient;

pub mod ip_tacacs;
pub use ip_tacacs::IpTacacs;

pub mod ip_tacacs_source_interfaces;
pub use ip_tacacs_source_interfaces::IpTacacsSourceInterfaces;

pub mod ip_telnet_client;
pub use ip_telnet_client::IpTelnetClient;

pub mod ip_tftp_client;
pub use ip_tftp_client::IpTftpClient;

pub mod ipv6_access_lists;
pub use ipv6_access_lists::Ipv6AccessLists;

pub mod ipv6_dhcp_relay;
pub use ipv6_dhcp_relay::Ipv6DhcpRelay;

pub mod ipv6_hardware;
pub use ipv6_hardware::Ipv6Hardware;

pub mod ipv6_neighbor;
pub use ipv6_neighbor::Ipv6Neighbor;

pub mod ipv6_prefix_lists;
pub use ipv6_prefix_lists::Ipv6PrefixLists;

pub mod ipv6_router_ospf;
pub use ipv6_router_ospf::Ipv6RouterOspf;

pub mod ipv6_standard_access_lists;
pub use ipv6_standard_access_lists::Ipv6StandardAccessLists;

pub mod ipv6_static_routes;
pub use ipv6_static_routes::Ipv6StaticRoutes;

pub mod kernel;
pub use kernel::Kernel;

pub mod l2_protocol;
pub use l2_protocol::L2Protocol;

pub mod lacp;
pub use lacp::Lacp;

pub mod link_tracking_groups;
pub use link_tracking_groups::LinkTrackingGroups;

pub mod lldp;
pub use lldp::Lldp;

pub mod load_balance;
pub use load_balance::LoadBalance;

pub mod load_interval;
pub use load_interval::LoadInterval;

pub mod local_users;
pub use local_users::LocalUsers;

pub mod logging;
pub use logging::Logging;

pub mod loopback_interfaces;
pub use loopback_interfaces::LoopbackInterfaces;

pub mod mac_access_lists;
pub use mac_access_lists::MacAccessLists;

pub mod mac_address_table;
pub use mac_address_table::MacAddressTable;

pub mod mac_security;
pub use mac_security::MacSecurity;

pub mod maintenance;
pub use maintenance::Maintenance;

pub mod management_accounts;
pub use management_accounts::ManagementAccounts;

pub mod management_api_gnmi;
pub use management_api_gnmi::ManagementApiGnmi;

pub mod management_api_http;
pub use management_api_http::ManagementApiHttp;

pub mod management_api_models;
pub use management_api_models::ManagementApiModels;

pub mod management_console;
pub use management_console::ManagementConsole;

pub mod management_cvx;
pub use management_cvx::ManagementCvx;

pub mod management_defaults;
pub use management_defaults::ManagementDefaults;

pub mod management_interfaces;
pub use management_interfaces::ManagementInterfaces;

pub mod management_ldap;
pub use management_ldap::ManagementLdap;

pub mod management_security;
pub use management_security::ManagementSecurity;

pub mod management_ssh;
pub use management_ssh::ManagementSsh;

pub mod management_tech_support;
pub use management_tech_support::ManagementTechSupport;

pub mod match_list_input;
pub use match_list_input::MatchListInput;

pub mod mcs_client;
pub use mcs_client::McsClient;

pub mod metadata;
pub use metadata::Metadata;

pub mod mlag_configuration;
pub use mlag_configuration::MlagConfiguration;

pub mod monitor_connectivity;
pub use monitor_connectivity::MonitorConnectivity;

pub mod monitor_layer1;
pub use monitor_layer1::MonitorLayer1;

pub mod monitor_link_flap_policy;
pub use monitor_link_flap_policy::MonitorLinkFlapPolicy;

pub mod monitor_loop_protection;
pub use monitor_loop_protection::MonitorLoopProtection;

pub mod monitor_server_radius;
pub use monitor_server_radius::MonitorServerRadius;

pub mod monitor_session_default_encapsulation_gre;
pub use monitor_session_default_encapsulation_gre::MonitorSessionDefaultEncapsulationGre;

pub mod monitor_sessions;
pub use monitor_sessions::MonitorSessions;

pub mod monitor_telemetry_influx;
pub use monitor_telemetry_influx::MonitorTelemetryInflux;

pub mod monitor_telemetry_postcard_policy;
pub use monitor_telemetry_postcard_policy::MonitorTelemetryPostcardPolicy;

pub mod monitor_twamp;
pub use monitor_twamp::MonitorTwamp;

pub mod mpls;
pub use mpls::Mpls;

pub mod ntp;
pub use ntp::Ntp;

pub mod patch_panel;
pub use patch_panel::PatchPanel;

pub mod peer_filters;
pub use peer_filters::PeerFilters;

pub mod platform;
pub use platform::Platform;

pub mod poe;
pub use poe::Poe;

pub mod policy_maps;
pub use policy_maps::PolicyMaps;

pub mod port_channel;
pub use port_channel::PortChannel;

pub mod port_channel_interfaces;
pub use port_channel_interfaces::PortChannelInterfaces;

pub mod prefix_lists;
pub use prefix_lists::PrefixLists;

pub mod priority_flow_control;
pub use priority_flow_control::PriorityFlowControl;

pub mod ptp;
pub use ptp::Ptp;

pub mod qos;
pub use qos::Qos;

pub mod qos_profiles;
pub use qos_profiles::QosProfiles;

pub mod queue_monitor_length;
pub use queue_monitor_length::QueueMonitorLength;

pub mod queue_monitor_streaming;
pub use queue_monitor_streaming::QueueMonitorStreaming;

pub mod radius_proxy;
pub use radius_proxy::RadiusProxy;

pub mod radius_server;
pub use radius_server::RadiusServer;

pub mod redundancy;
pub use redundancy::Redundancy;

pub mod roles;
pub use roles::Roles;

pub mod route_maps;
pub use route_maps::RouteMaps;

pub mod router_adaptive_virtual_topology;
pub use router_adaptive_virtual_topology::RouterAdaptiveVirtualTopology;

pub mod router_bfd;
pub use router_bfd::RouterBfd;

pub mod router_bgp;
pub use router_bgp::RouterBgp;

pub mod router_general;
pub use router_general::RouterGeneral;

pub mod router_igmp;
pub use router_igmp::RouterIgmp;

pub mod router_internet_exit;
pub use router_internet_exit::RouterInternetExit;

pub mod router_isis;
pub use router_isis::RouterIsis;

pub mod router_l2_vpn;
pub use router_l2_vpn::RouterL2Vpn;

pub mod router_msdp;
pub use router_msdp::RouterMsdp;

pub mod router_multicast;
pub use router_multicast::RouterMulticast;

pub mod router_ospf;
pub use router_ospf::RouterOspf;

pub mod router_ospfv3;
pub use router_ospfv3::RouterOspfv3;

pub mod router_path_selection;
pub use router_path_selection::RouterPathSelection;

pub mod router_pim_sparse_mode;
pub use router_pim_sparse_mode::RouterPimSparseMode;

pub mod router_rip;
pub use router_rip::RouterRip;

pub mod router_segment_security;
pub use router_segment_security::RouterSegmentSecurity;

pub mod router_service_insertion;
pub use router_service_insertion::RouterServiceInsertion;

pub mod router_traffic_engineering;
pub use router_traffic_engineering::RouterTrafficEngineering;

pub mod schedule;
pub use schedule::Schedule;

pub mod service_routing_configuration_bgp;
pub use service_routing_configuration_bgp::ServiceRoutingConfigurationBgp;

pub mod service_unsupported_transceiver;
pub use service_unsupported_transceiver::ServiceUnsupportedTransceiver;

pub mod sflow;
pub use sflow::Sflow;

pub mod snmp_server;
pub use snmp_server::SnmpServer;

pub mod spanning_tree;
pub use spanning_tree::SpanningTree;

pub mod standard_access_lists;
pub use standard_access_lists::StandardAccessLists;

pub mod static_routes;
pub use static_routes::StaticRoutes;

pub mod stun;
pub use stun::Stun;

pub mod switchport_default;
pub use switchport_default::SwitchportDefault;

pub mod switchport_port_security;
pub use switchport_port_security::SwitchportPortSecurity;

pub mod sync_e;
pub use sync_e::SyncE;

pub mod system;
pub use system::System;

pub mod tacacs_servers;
pub use tacacs_servers::TacacsServers;

pub mod tap_aggregation;
pub use tap_aggregation::TapAggregation;

pub mod tcam_profile;
pub use tcam_profile::TcamProfile;

pub mod terminal;
pub use terminal::Terminal;

pub mod trackers;
pub use trackers::Trackers;

pub mod traffic_policies;
pub use traffic_policies::TrafficPolicies;

pub mod transceiver;
pub use transceiver::Transceiver;

pub mod tunnel_interfaces;
pub use tunnel_interfaces::TunnelInterfaces;

pub mod virtual_source_nat_vrfs;
pub use virtual_source_nat_vrfs::VirtualSourceNatVrfs;

pub mod vlan_interfaces;
pub use vlan_interfaces::VlanInterfaces;

pub mod vlan_internal_order;
pub use vlan_internal_order::VlanInternalOrder;

pub mod vlans;
pub use vlans::Vlans;

pub mod vmtracer_sessions;
pub use vmtracer_sessions::VmtracerSessions;

pub mod vrfs;
pub use vrfs::Vrfs;

pub mod vrrp;
pub use vrrp::Vrrp;

pub mod vxlan_interface;
pub use vxlan_interface::VxlanInterface;
