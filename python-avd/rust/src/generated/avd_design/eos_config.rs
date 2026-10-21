// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct EosConfig<'a, Mode> {
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
    pub vrrp: ::validated_data::Field<Vrrp<'a, Mode>>,
    pub vxlan_interface: ::validated_data::Field<VxlanInterface<'a, Mode>>,
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
