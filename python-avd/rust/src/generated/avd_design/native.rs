// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

use ::validated_data_py::Target;

::validated_data_py::python_data_views! {
    pub BINDINGS {
        "EosCliConfigGenAaaAccountingExec" => dict {
            "console": 0 => Target::Model("EosCliConfigGenAaaAccountingExecConsole"),
            "default": 1 => Target::Model("EosCliConfigGenAaaAccountingExecDefault"),
        };
        "EosCliConfigGenAaaAccountingExecConsole" => dict {
            "field_type": 0 => Target::Scalar,
            "methods": 1 => Target::Model("EosCliConfigGenAaaAccountingExecConsoleMethodsList"),
        };
        "EosCliConfigGenAaaAccountingExecConsoleMethodsList" => list(Target::Model("EosCliConfigGenAaaAccountingExecConsoleMethodsItems"));
        "EosCliConfigGenAaaAccountingExecConsoleMethodsItems" => dict {
            "method": 0 => Target::Scalar,
            "group": 1 => Target::Scalar,
        };
        "EosCliConfigGenAaaAccountingExecDefault" => dict {
            "field_type": 0 => Target::Scalar,
            "methods": 1 => Target::Model("EosCliConfigGenAaaAccountingExecDefaultMethodsList"),
        };
        "EosCliConfigGenAaaAccountingExecDefaultMethodsList" => list(Target::Model("EosCliConfigGenAaaAccountingExecDefaultMethodsItems"));
        "EosCliConfigGenAaaAccountingExecDefaultMethodsItems" => dict {
            "method": 0 => Target::Scalar,
            "group": 1 => Target::Scalar,
        };
        "EosCliConfigGenAaaAccountingSystem" => dict {
            "default": 0 => Target::Model("EosCliConfigGenAaaAccountingSystemDefault"),
        };
        "EosCliConfigGenAaaAccountingSystemDefault" => dict {
            "field_type": 0 => Target::Scalar,
            "methods": 1 => Target::Model("EosCliConfigGenAaaAccountingSystemDefaultMethodsList"),
        };
        "EosCliConfigGenAaaAccountingSystemDefaultMethodsList" => list(Target::Model("EosCliConfigGenAaaAccountingSystemDefaultMethodsItems"));
        "EosCliConfigGenAaaAccountingSystemDefaultMethodsItems" => dict {
            "method": 0 => Target::Scalar,
            "group": 1 => Target::Scalar,
        };
        "EosCliConfigGenAaaAccountingCommands" => dict {
            "console": 0 => Target::Model("EosCliConfigGenAaaAccountingCommandsConsoleList"),
            "default": 1 => Target::Model("EosCliConfigGenAaaAccountingCommandsDefaultList"),
        };
        "EosCliConfigGenAaaAccountingCommandsConsoleList" => list(Target::Model("EosCliConfigGenAaaAccountingCommandsConsoleItems"));
        "EosCliConfigGenAaaAccountingCommandsConsoleItems" => dict {
            "commands": 0 => Target::Scalar,
            "field_type": 1 => Target::Scalar,
            "methods": 2 => Target::Model("EosCliConfigGenAaaAccountingCommandsConsoleItemsMethodsList"),
        };
        "EosCliConfigGenAaaAccountingCommandsConsoleItemsMethodsList" => list(Target::Model("EosCliConfigGenAaaAccountingCommandsConsoleItemsMethodsItems"));
        "EosCliConfigGenAaaAccountingCommandsConsoleItemsMethodsItems" => dict {
            "method": 0 => Target::Scalar,
            "group": 1 => Target::Scalar,
        };
        "EosCliConfigGenAaaAccountingCommandsDefaultList" => list(Target::Model("EosCliConfigGenAaaAccountingCommandsDefaultItems"));
        "EosCliConfigGenAaaAccountingCommandsDefaultItems" => dict {
            "commands": 0 => Target::Scalar,
            "field_type": 1 => Target::Scalar,
            "methods": 2 => Target::Model("EosCliConfigGenAaaAccountingCommandsDefaultItemsMethodsList"),
        };
        "EosCliConfigGenAaaAccountingCommandsDefaultItemsMethodsList" => list(Target::Model("EosCliConfigGenAaaAccountingCommandsDefaultItemsMethodsItems"));
        "EosCliConfigGenAaaAccountingCommandsDefaultItemsMethodsItems" => dict {
            "method": 0 => Target::Scalar,
            "group": 1 => Target::Scalar,
        };
        "EosCliConfigGenAaaAuthenticationLogin" => dict {
            "default": 0 => Target::Scalar,
            "command_api": 1 => Target::Scalar,
            "console": 2 => Target::Scalar,
        };
        "EosCliConfigGenAaaAuthenticationEnable" => dict {
            "default": 0 => Target::Scalar,
        };
        "EosCliConfigGenAaaAuthenticationPolicies" => dict {
            "on_failure_log": 0 => Target::Scalar,
            "on_success_log": 1 => Target::Scalar,
            "local": 2 => Target::Model("EosCliConfigGenAaaAuthenticationPoliciesLocal"),
            "lockout": 3 => Target::Model("EosCliConfigGenAaaAuthenticationPoliciesLockout"),
        };
        "EosCliConfigGenAaaAuthenticationPoliciesLocal" => dict {
            "allow_nopassword": 0 => Target::Scalar,
        };
        "EosCliConfigGenAaaAuthenticationPoliciesLockout" => dict {
            "failure": 0 => Target::Scalar,
            "duration": 1 => Target::Scalar,
            "window": 2 => Target::Scalar,
        };
        "EosCliConfigGenAaaAuthorizationPolicy" => dict {
            "local_default_role": 0 => Target::Scalar,
        };
        "EosCliConfigGenAaaAuthorizationExec" => dict {
            "default": 0 => Target::Scalar,
        };
        "EosCliConfigGenAaaAuthorizationCommands" => dict {
            "all_default": 0 => Target::Scalar,
            "privilege": 1 => Target::Model("EosCliConfigGenAaaAuthorizationCommandsPrivilegeList"),
        };
        "EosCliConfigGenAaaAuthorizationCommandsPrivilegeList" => list(Target::Model("EosCliConfigGenAaaAuthorizationCommandsPrivilegeItems"));
        "EosCliConfigGenAaaAuthorizationCommandsPrivilegeItems" => dict {
            "level": 0 => Target::Scalar,
            "default": 1 => Target::Scalar,
        };
        "EosCliConfigGenApplicationTrafficRecognition" => dict {
            "categories": 0 => Target::Model("EosCliConfigGenApplicationTrafficRecognitionCategoriesList"),
            "field_sets": 1 => Target::Model("EosCliConfigGenApplicationTrafficRecognitionFieldSets"),
            "applications": 2 => Target::Model("EosCliConfigGenApplicationTrafficRecognitionApplications"),
            "application_profiles": 3 => Target::Model("EosCliConfigGenApplicationTrafficRecognitionApplicationProfilesList"),
        };
        "EosCliConfigGenApplicationTrafficRecognitionCategoriesList" => indexed(Target::Model("EosCliConfigGenApplicationTrafficRecognitionCategoriesListIndexedItem"), [0]);
        "EosCliConfigGenApplicationTrafficRecognitionCategoriesItems" => dict {
            "name": 0 => Target::Scalar,
            "applications": 1 => Target::Model("EosCliConfigGenApplicationTrafficRecognitionCategoriesItemsApplicationsList"),
        };
        "EosCliConfigGenApplicationTrafficRecognitionCategoriesItemsApplicationsList" => list(Target::Model("EosCliConfigGenApplicationTrafficRecognitionCategoriesItemsApplicationsItems"));
        "EosCliConfigGenApplicationTrafficRecognitionCategoriesItemsApplicationsItems" => dict {
            "name": 0 => Target::Scalar,
            "service": 1 => Target::Scalar,
        };
        "EosCliConfigGenApplicationTrafficRecognitionFieldSets" => dict {
            "l4_ports": 0 => Target::Model("EosCliConfigGenApplicationTrafficRecognitionFieldSetsL4PortsList"),
            "ipv4_prefixes": 1 => Target::Model("EosCliConfigGenApplicationTrafficRecognitionFieldSetsIpv4PrefixesList"),
        };
        "EosCliConfigGenApplicationTrafficRecognitionFieldSetsL4PortsList" => indexed(Target::Model("EosCliConfigGenApplicationTrafficRecognitionFieldSetsL4PortsListIndexedItem"), [0]);
        "EosCliConfigGenApplicationTrafficRecognitionFieldSetsL4PortsItems" => dict {
            "name": 0 => Target::Scalar,
            "port_values": 1 => Target::Model("EosCliConfigGenApplicationTrafficRecognitionFieldSetsL4PortsItemsPortValuesList"),
        };
        "EosCliConfigGenApplicationTrafficRecognitionFieldSetsL4PortsItemsPortValuesList" => list(Target::Scalar);
        "EosCliConfigGenApplicationTrafficRecognitionFieldSetsIpv4PrefixesList" => indexed(Target::Model("EosCliConfigGenApplicationTrafficRecognitionFieldSetsIpv4PrefixesListIndexedItem"), [0]);
        "EosCliConfigGenApplicationTrafficRecognitionFieldSetsIpv4PrefixesItems" => dict {
            "name": 0 => Target::Scalar,
            "prefix_values": 1 => Target::Model("EosCliConfigGenApplicationTrafficRecognitionFieldSetsIpv4PrefixesItemsPrefixValuesList"),
        };
        "EosCliConfigGenApplicationTrafficRecognitionFieldSetsIpv4PrefixesItemsPrefixValuesList" => list(Target::Scalar);
        "EosCliConfigGenApplicationTrafficRecognitionApplications" => dict {
            "ipv4_applications": 0 => Target::Model("EosCliConfigGenApplicationTrafficRecognitionApplicationsIpv4ApplicationsList"),
            "l4_applications": 1 => Target::Model("EosCliConfigGenApplicationTrafficRecognitionApplicationsL4ApplicationsList"),
        };
        "EosCliConfigGenApplicationTrafficRecognitionApplicationsIpv4ApplicationsList" => indexed(Target::Model("EosCliConfigGenApplicationTrafficRecognitionApplicationsIpv4ApplicationsListIndexedItem"), [0]);
        "EosCliConfigGenApplicationTrafficRecognitionApplicationsIpv4ApplicationsItems" => dict {
            "name": 0 => Target::Scalar,
            "src_prefix_set_name": 1 => Target::Scalar,
            "dest_prefix_set_name": 2 => Target::Scalar,
            "dscp_ranges": 3 => Target::Model("EosCliConfigGenApplicationTrafficRecognitionApplicationsIpv4ApplicationsItemsDscpRangesList"),
            "protocols": 4 => Target::Model("EosCliConfigGenApplicationTrafficRecognitionApplicationsIpv4ApplicationsItemsProtocolsList"),
            "protocol_ranges": 5 => Target::Model("EosCliConfigGenApplicationTrafficRecognitionApplicationsIpv4ApplicationsItemsProtocolRangesList"),
            "udp_src_port_set_name": 6 => Target::Scalar,
            "tcp_src_port_set_name": 7 => Target::Scalar,
            "udp_dest_port_set_name": 8 => Target::Scalar,
            "tcp_dest_port_set_name": 9 => Target::Scalar,
        };
        "EosCliConfigGenApplicationTrafficRecognitionApplicationsIpv4ApplicationsItemsDscpRangesList" => list(Target::Scalar);
        "EosCliConfigGenApplicationTrafficRecognitionApplicationsIpv4ApplicationsItemsProtocolsList" => list(Target::Scalar);
        "EosCliConfigGenApplicationTrafficRecognitionApplicationsIpv4ApplicationsItemsProtocolRangesList" => list(Target::Scalar);
        "EosCliConfigGenApplicationTrafficRecognitionApplicationsL4ApplicationsList" => indexed(Target::Model("EosCliConfigGenApplicationTrafficRecognitionApplicationsL4ApplicationsListIndexedItem"), [0]);
        "EosCliConfigGenApplicationTrafficRecognitionApplicationsL4ApplicationsItems" => dict {
            "name": 0 => Target::Scalar,
            "protocols": 1 => Target::Model("EosCliConfigGenApplicationTrafficRecognitionApplicationsL4ApplicationsItemsProtocolsList"),
            "protocol_ranges": 2 => Target::Model("EosCliConfigGenApplicationTrafficRecognitionApplicationsL4ApplicationsItemsProtocolRangesList"),
            "udp_src_port_set_name": 3 => Target::Scalar,
            "tcp_src_port_set_name": 4 => Target::Scalar,
            "udp_dest_port_set_name": 5 => Target::Scalar,
            "tcp_dest_port_set_name": 6 => Target::Scalar,
        };
        "EosCliConfigGenApplicationTrafficRecognitionApplicationsL4ApplicationsItemsProtocolsList" => list(Target::Scalar);
        "EosCliConfigGenApplicationTrafficRecognitionApplicationsL4ApplicationsItemsProtocolRangesList" => list(Target::Scalar);
        "EosCliConfigGenApplicationTrafficRecognitionApplicationProfilesList" => indexed(Target::Model("EosCliConfigGenApplicationTrafficRecognitionApplicationProfilesListIndexedItem"), [0]);
        "EosCliConfigGenApplicationTrafficRecognitionApplicationProfilesItems" => dict {
            "name": 0 => Target::Scalar,
            "applications": 1 => Target::Model("EosCliConfigGenApplicationTrafficRecognitionApplicationProfilesItemsApplicationsList"),
            "application_transports": 2 => Target::Model("EosCliConfigGenApplicationTrafficRecognitionApplicationProfilesItemsApplicationTransportsList"),
            "categories": 3 => Target::Model("EosCliConfigGenApplicationTrafficRecognitionApplicationProfilesItemsCategoriesList"),
        };
        "EosCliConfigGenApplicationTrafficRecognitionApplicationProfilesItemsApplicationsList" => list(Target::Model("EosCliConfigGenApplicationTrafficRecognitionApplicationProfilesItemsApplicationsItems"));
        "EosCliConfigGenApplicationTrafficRecognitionApplicationProfilesItemsApplicationsItems" => dict {
            "name": 0 => Target::Scalar,
            "service": 1 => Target::Scalar,
        };
        "EosCliConfigGenApplicationTrafficRecognitionApplicationProfilesItemsApplicationTransportsList" => list(Target::Scalar);
        "EosCliConfigGenApplicationTrafficRecognitionApplicationProfilesItemsCategoriesList" => list(Target::Model("EosCliConfigGenApplicationTrafficRecognitionApplicationProfilesItemsCategoriesItems"));
        "EosCliConfigGenApplicationTrafficRecognitionApplicationProfilesItemsCategoriesItems" => dict {
            "name": 0 => Target::Scalar,
            "service": 1 => Target::Scalar,
        };
        "EosCliConfigGenArpPersistent" => dict {
            "enabled": 0 => Target::Scalar,
            "refresh_delay": 1 => Target::Scalar,
        };
        "EosCliConfigGenArpAging" => dict {
            "timeout_default": 0 => Target::Scalar,
        };
        "EosCliConfigGenBanners" => dict {
            "login": 0 => Target::Scalar,
            "motd": 1 => Target::Scalar,
        };
        "EosCliConfigGenDot1xAaaUnresponsivePhoneAction" => dict {
            "apply_cached_results": 0 => Target::Scalar,
            "cached_results_timeout": 1 => Target::Model("EosCliConfigGenDot1xAaaUnresponsivePhoneActionCachedResultsTimeout"),
            "apply_alternate": 2 => Target::Scalar,
            "traffic_allow": 3 => Target::Scalar,
        };
        "EosCliConfigGenDot1xAaaUnresponsivePhoneActionCachedResultsTimeout" => dict {
            "time_duration": 0 => Target::Scalar,
            "time_duration_unit": 1 => Target::Scalar,
        };
        "EosCliConfigGenEthernetInterfacesItemsFlowcontrol" => dict {
            "received": 0 => Target::Scalar,
        };
        "EosCliConfigGenEthernetInterfacesItemsPoe" => dict {
            "disabled": 0 => Target::Scalar,
            "priority": 1 => Target::Scalar,
            "reboot": 2 => Target::Model("EosCliConfigGenEthernetInterfacesItemsPoeReboot"),
            "link_down": 3 => Target::Model("EosCliConfigGenEthernetInterfacesItemsPoeLinkDown"),
            "shutdown": 4 => Target::Model("EosCliConfigGenEthernetInterfacesItemsPoeShutdown"),
            "limit": 5 => Target::Model("EosCliConfigGenEthernetInterfacesItemsPoeLimit"),
            "negotiation_lldp": 6 => Target::Scalar,
            "legacy_detect": 7 => Target::Scalar,
        };
        "EosCliConfigGenEthernetInterfacesItemsPoeReboot" => dict {
            "action": 0 => Target::Scalar,
        };
        "EosCliConfigGenEthernetInterfacesItemsPoeLinkDown" => dict {
            "action": 0 => Target::Scalar,
            "power_off_delay": 1 => Target::Scalar,
        };
        "EosCliConfigGenEthernetInterfacesItemsPoeShutdown" => dict {
            "action": 0 => Target::Scalar,
        };
        "EosCliConfigGenEthernetInterfacesItemsPoeLimit" => dict {
            "field_class": 0 => Target::Scalar,
            "watts": 1 => Target::Scalar,
            "fixed": 2 => Target::Scalar,
        };
        "EosCliConfigGenEventHandlersList" => indexed(Target::Model("EosCliConfigGenEventHandlersListIndexedItem"), [0]);
        "EosCliConfigGenEventHandlersItems" => dict {
            "name": 0 => Target::Scalar,
            "actions": 1 => Target::Model("EosCliConfigGenEventHandlersItemsActions"),
            "delay": 2 => Target::Scalar,
            "trigger": 3 => Target::Scalar,
            "trigger_on_counters": 4 => Target::Model("EosCliConfigGenEventHandlersItemsTriggerOnCounters"),
            "trigger_on_logging": 5 => Target::Model("EosCliConfigGenEventHandlersItemsTriggerOnLogging"),
            "trigger_on_intf": 6 => Target::Model("EosCliConfigGenEventHandlersItemsTriggerOnIntf"),
            "trigger_on_maintenance": 7 => Target::Model("EosCliConfigGenEventHandlersItemsTriggerOnMaintenance"),
            "asynchronous": 8 => Target::Scalar,
        };
        "EosCliConfigGenEventHandlersItemsActions" => dict {
            "bash_command": 0 => Target::Scalar,
            "log": 1 => Target::Scalar,
            "increment_device_health_metric": 2 => Target::Scalar,
        };
        "EosCliConfigGenEventHandlersItemsTriggerOnCounters" => dict {
            "condition": 0 => Target::Scalar,
            "granularity_per_source": 1 => Target::Scalar,
            "poll_interval": 2 => Target::Scalar,
        };
        "EosCliConfigGenEventHandlersItemsTriggerOnLogging" => dict {
            "poll_interval": 0 => Target::Scalar,
            "regex": 1 => Target::Scalar,
        };
        "EosCliConfigGenEventHandlersItemsTriggerOnIntf" => dict {
            "interface": 0 => Target::Scalar,
            "ip": 1 => Target::Scalar,
            "ipv6": 2 => Target::Scalar,
            "operstatus": 3 => Target::Scalar,
        };
        "EosCliConfigGenEventHandlersItemsTriggerOnMaintenance" => dict {
            "operation": 0 => Target::Scalar,
            "bgp_peer": 1 => Target::Scalar,
            "action": 2 => Target::Scalar,
            "stage": 3 => Target::Scalar,
            "vrf": 4 => Target::Scalar,
            "interface": 5 => Target::Scalar,
            "unit": 6 => Target::Scalar,
        };
        "EosCliConfigGenEventMonitor" => dict {
            "enabled": 0 => Target::Scalar,
        };
        "EosCliConfigGenHardwareCounters" => dict {
            "features": 0 => Target::Model("EosCliConfigGenHardwareCountersFeaturesList"),
        };
        "EosCliConfigGenHardwareCountersFeaturesList" => list(Target::Model("EosCliConfigGenHardwareCountersFeaturesItems"));
        "EosCliConfigGenHardwareCountersFeaturesItems" => dict {
            "name": 0 => Target::Scalar,
            "direction": 1 => Target::Scalar,
            "enabled": 2 => Target::Scalar,
            "address_type": 3 => Target::Scalar,
            "layer3": 4 => Target::Scalar,
            "vrf": 5 => Target::Scalar,
            "prefix": 6 => Target::Scalar,
            "units_packets": 7 => Target::Scalar,
        };
        "EosCliConfigGenIpHostsList" => indexed(Target::Model("EosCliConfigGenIpHostsListIndexedItem"), [0]);
        "EosCliConfigGenIpHostsItems" => dict {
            "hostname": 0 => Target::Scalar,
            "ipv4_addresses": 1 => Target::Model("EosCliConfigGenIpHostsItemsIpv4AddressesList"),
        };
        "EosCliConfigGenIpHostsItemsIpv4AddressesList" => list(Target::Scalar);
        "EosCliConfigGenLoadInterval" => dict {
            "default": 0 => Target::Scalar,
        };
        "EosCliConfigGenLoggingBuffered" => dict {
            "size": 0 => Target::Scalar,
            "level": 1 => Target::Scalar,
        };
        "EosCliConfigGenLoggingSynchronous" => dict {
            "level": 0 => Target::Scalar,
        };
        "EosCliConfigGenLoggingFormat" => dict {
            "timestamp": 0 => Target::Scalar,
            "hostname": 1 => Target::Scalar,
            "sequence_numbers": 2 => Target::Scalar,
            "rfc5424": 3 => Target::Scalar,
        };
        "EosCliConfigGenLoggingPolicy" => dict {
            "field_match": 0 => Target::Model("EosCliConfigGenLoggingPolicyMatch"),
        };
        "EosCliConfigGenLoggingPolicyMatch" => dict {
            "match_lists": 0 => Target::Model("EosCliConfigGenLoggingPolicyMatchMatchListsList"),
        };
        "EosCliConfigGenLoggingPolicyMatchMatchListsList" => indexed(Target::Model("EosCliConfigGenLoggingPolicyMatchMatchListsListIndexedItem"), [0]);
        "EosCliConfigGenLoggingPolicyMatchMatchListsItems" => dict {
            "name": 0 => Target::Scalar,
            "action": 1 => Target::Scalar,
        };
        "EosCliConfigGenLoggingEvent" => dict {
            "congestion_drops_interval": 0 => Target::Scalar,
            "global_link_status": 1 => Target::Scalar,
            "storm_control": 2 => Target::Model("EosCliConfigGenLoggingEventStormControl"),
        };
        "EosCliConfigGenLoggingEventStormControl" => dict {
            "discards": 0 => Target::Model("EosCliConfigGenLoggingEventStormControlDiscards"),
        };
        "EosCliConfigGenLoggingEventStormControlDiscards" => dict {
            "field_global": 0 => Target::Scalar,
            "interval": 1 => Target::Scalar,
        };
        "EosCliConfigGenLoggingLevelList" => indexed(Target::Model("EosCliConfigGenLoggingLevelListIndexedItem"), [0]);
        "EosCliConfigGenLoggingLevelItems" => dict {
            "facility": 0 => Target::Scalar,
            "severity": 1 => Target::Scalar,
        };
        "EosCliConfigGenMacAddressTable" => dict {
            "aging_time": 0 => Target::Scalar,
            "notification_host_flap": 1 => Target::Model("EosCliConfigGenMacAddressTableNotificationHostFlap"),
            "static_entries": 2 => Target::Model("EosCliConfigGenMacAddressTableStaticEntriesList"),
        };
        "EosCliConfigGenMacAddressTableNotificationHostFlap" => dict {
            "logging": 0 => Target::Scalar,
            "detection": 1 => Target::Model("EosCliConfigGenMacAddressTableNotificationHostFlapDetection"),
        };
        "EosCliConfigGenMacAddressTableNotificationHostFlapDetection" => dict {
            "window": 0 => Target::Scalar,
            "moves": 1 => Target::Scalar,
        };
        "EosCliConfigGenMacAddressTableStaticEntriesList" => list(Target::Model("EosCliConfigGenMacAddressTableStaticEntriesItems"));
        "EosCliConfigGenMacAddressTableStaticEntriesItems" => dict {
            "mac_address": 0 => Target::Scalar,
            "vlan": 1 => Target::Scalar,
            "drop": 2 => Target::Scalar,
            "interface": 3 => Target::Scalar,
            "eligibility_forwarding": 4 => Target::Scalar,
        };
        "EosCliConfigGenManagementConsole" => dict {
            "idle_timeout": 0 => Target::Scalar,
        };
        "EosCliConfigGenMetadataInterfaces" => dict {
            "errdisable": 0 => Target::Model("EosCliConfigGenMetadataInterfacesErrdisable"),
        };
        "EosCliConfigGenMetadataInterfacesErrdisable" => dict {
            "only_avd_interfaces": 0 => Target::Scalar,
        };
        "EosCliConfigGenMetadataBgp" => dict {
            "check_tcp_queues": 0 => Target::Scalar,
            "minimum_established_time": 1 => Target::Scalar,
        };
        "EosCliConfigGenMonitorLayer1" => dict {
            "enabled": 0 => Target::Scalar,
            "logging_mac_fault": 1 => Target::Scalar,
            "logging_transceiver": 2 => Target::Model("EosCliConfigGenMonitorLayer1LoggingTransceiver"),
        };
        "EosCliConfigGenMonitorLayer1LoggingTransceiver" => dict {
            "dom": 0 => Target::Scalar,
            "communication": 1 => Target::Scalar,
            "enabled": 2 => Target::Scalar,
        };
        "EosCliConfigGenPeerFiltersList" => indexed(Target::Model("EosCliConfigGenPeerFiltersListIndexedItem"), [0]);
        "EosCliConfigGenPeerFiltersItems" => dict {
            "name": 0 => Target::Scalar,
            "sequence_numbers": 1 => Target::Model("EosCliConfigGenPeerFiltersItemsSequenceNumbersList"),
        };
        "EosCliConfigGenPeerFiltersItemsSequenceNumbersList" => indexed(Target::Model("EosCliConfigGenPeerFiltersItemsSequenceNumbersListIndexedItem"), [0]);
        "EosCliConfigGenPeerFiltersItemsSequenceNumbersItems" => dict {
            "sequence": 0 => Target::Scalar,
            "field_match": 1 => Target::Scalar,
        };
        "EosCliConfigGenPtpFreeRunning" => dict {
            "enabled": 0 => Target::Scalar,
            "source_clock_hardware": 1 => Target::Scalar,
        };
        "EosCliConfigGenQueueMonitorStreaming" => dict {
            "enable": 0 => Target::Scalar,
            "ip_access_group": 1 => Target::Scalar,
            "ipv6_access_group": 2 => Target::Scalar,
            "max_connections": 3 => Target::Scalar,
            "vrf": 4 => Target::Scalar,
        };
        "EosCliConfigGenRadiusServerServersItemsTls" => dict {
            "enabled": 0 => Target::Scalar,
            "ssl_profile": 1 => Target::Scalar,
            "port": 2 => Target::Scalar,
        };
        "EosCliConfigGenRouterAdaptiveVirtualTopologyProfilesItemsMetricOrder" => dict {
            "preferred_metric": 0 => Target::Scalar,
        };
        "EosCliConfigGenRouterAdaptiveVirtualTopologyProfilesItemsOutlierElimination" => dict {
            "disabled": 0 => Target::Scalar,
            "threshold": 1 => Target::Model("EosCliConfigGenRouterAdaptiveVirtualTopologyProfilesItemsOutlierEliminationThreshold"),
        };
        "EosCliConfigGenRouterAdaptiveVirtualTopologyProfilesItemsOutlierEliminationThreshold" => dict {
            "jitter": 0 => Target::Scalar,
            "latency": 1 => Target::Scalar,
            "load": 2 => Target::Scalar,
            "loss_rate": 3 => Target::Scalar,
        };
        "EosCliConfigGenRouterBgpDistance" => dict {
            "external_routes": 0 => Target::Scalar,
            "internal_routes": 1 => Target::Scalar,
            "local_routes": 2 => Target::Scalar,
        };
        "EosCliConfigGenRouterBgpAddressFamilyIpv4PeerGroupsItemsDefaultOriginate" => dict {
            "always": 0 => Target::Scalar,
            "route_map": 1 => Target::Scalar,
        };
        "EosCliConfigGenRouterBgpAddressFamilyIpv4PeerGroupsItemsNextHop" => dict {
            "address_family_ipv6": 0 => Target::Model("EosCliConfigGenRouterBgpAddressFamilyIpv4PeerGroupsItemsNextHopAddressFamilyIpv6"),
        };
        "EosCliConfigGenRouterBgpAddressFamilyIpv4PeerGroupsItemsNextHopAddressFamilyIpv6" => dict {
            "enabled": 0 => Target::Scalar,
            "originate": 1 => Target::Scalar,
        };
        "EosCliConfigGenRouterBgpVrfsItemsNeighborsItemsBfdTimers" => dict {
            "interval": 0 => Target::Scalar,
            "min_rx": 1 => Target::Scalar,
            "multiplier": 2 => Target::Scalar,
        };
        "EosCliConfigGenServiceUnsupportedTransceiver" => dict {
            "license_name": 0 => Target::Scalar,
            "license_key": 1 => Target::Scalar,
        };
        "EosCliConfigGenSnmpServerTraps" => dict {
            "enable": 0 => Target::Scalar,
            "snmp_traps": 1 => Target::Model("EosCliConfigGenSnmpServerTrapsSnmpTrapsList"),
        };
        "EosCliConfigGenSnmpServerTrapsSnmpTrapsList" => list(Target::Model("EosCliConfigGenSnmpServerTrapsSnmpTrapsItems"));
        "EosCliConfigGenSnmpServerTrapsSnmpTrapsItems" => dict {
            "name": 0 => Target::Scalar,
            "enabled": 1 => Target::Scalar,
        };
        "EosCliConfigGenSpanningTreePortIdAllocationPortChannelRange" => dict {
            "minimum": 0 => Target::Scalar,
            "maximum": 1 => Target::Scalar,
        };
        "EosCliConfigGenTcamProfileProfilesList" => indexed(Target::Model("EosCliConfigGenTcamProfileProfilesListIndexedItem"), [0]);
        "EosCliConfigGenTcamProfileProfilesItems" => dict {
            "name": 0 => Target::Scalar,
            "config": 1 => Target::Scalar,
            "source": 2 => Target::Scalar,
        };
        "EosCliConfigGenVlansItemsAddressLockingAddressFamily" => dict {
            "ipv4": 0 => Target::Scalar,
            "ipv6": 1 => Target::Scalar,
        };
        "AVDDesign" => dict {
            "aaa_settings": 0 => Target::Model("AVDDesignAaaSettings"),
            "address_locking_settings": 1 => Target::Model("AVDDesignAddressLockingSettings"),
            "application_classification": 2 => Target::Model("EosCliConfigGenApplicationTrafficRecognition"),
            "avd_design_future": 3 => Target::Model("AVDDesignAvdDesignFuture"),
            "avd_digital_twin_mode": 4 => Target::Scalar,
            "avd_eos_designs_structured_config": 5 => Target::Scalar,
            "avd_structured_config_file_format": 6 => Target::Scalar,
            "eos_designs_validation_configuration": 7 => Target::Model("AVDDesignEosDesignsValidationConfiguration"),
            "avd_vault_id": 8 => Target::Scalar,
            "bfd_multihop": 9 => Target::Model("AVDDesignBfdMultihop"),
            "bgp_as": 10 => Target::Scalar,
            "bgp_as_notation": 11 => Target::Scalar,
            "bgp_default_ipv4_unicast": 12 => Target::Scalar,
            "bgp_distance": 13 => Target::Model("EosCliConfigGenRouterBgpDistance"),
            "bgp_ecmp": 14 => Target::Scalar,
            "bgp_graceful_restart": 15 => Target::Model("AVDDesignBgpGracefulRestart"),
            "bgp_maximum_paths": 16 => Target::Scalar,
            "bgp_mesh_pes": 17 => Target::Scalar,
            "bgp_peer_filters_catalog": 18 => Target::Model("EosCliConfigGenPeerFiltersList"),
            "bgp_peer_groups": 19 => Target::Model("AVDDesignBgpPeerGroups"),
            "bgp_update_wait_install": 20 => Target::Scalar,
            "bgp_update_wait_for_convergence": 21 => Target::Scalar,
            "campus": 22 => Target::Scalar,
            "campus_access_pod": 23 => Target::Scalar,
            "campus_pod": 24 => Target::Scalar,
            "connected_endpoints": 25 => Target::Model("AVDDesignConnectedEndpointsList"),
            "custom_connected_endpoints_keys": 26 => Target::Model("AVDDesignCustomConnectedEndpointsKeysList"),
            "connected_endpoints_keys": 27 => Target::Model("AVDDesignConnectedEndpointsKeysList"),
            "core_interfaces": 28 => Target::Model("AVDDesignCoreInterfaces"),
            "custom_structured_configuration_list_merge": 29 => Target::Scalar,
            "custom_structured_configuration_prefix": 30 => Target::Model("AVDDesignCustomStructuredConfigurationPrefixList"),
            "custom_system_mac_address": 31 => Target::Scalar,
            "cv_pathfinder_global_sites": 32 => Target::Model("AVDDesignCvPathfinderGlobalSitesList"),
            "cv_pathfinder_internet_exit_policies": 33 => Target::Model("AVDDesignCvPathfinderInternetExitPoliciesList"),
            "cv_pathfinder_regions": 34 => Target::Model("AVDDesignCvPathfinderRegionsList"),
            "cv_server": 35 => Target::Scalar,
            "cv_settings": 36 => Target::Model("AVDDesignCvSettings"),
            "cv_tags_topology_type": 37 => Target::Scalar,
            "cv_token": 38 => Target::Scalar,
            "cv_topology": 39 => Target::Model("AVDDesignCvTopologyList"),
            "cv_topology_levels": 40 => Target::Model("AVDDesignCvTopologyLevelsList"),
            "dc_name": 41 => Target::Scalar,
            "default_connected_endpoints_description": 42 => Target::Scalar,
            "default_connected_endpoints_port_channel_description": 43 => Target::Scalar,
            "default_igmp_snooping_enabled": 44 => Target::Scalar,
            "default_interface_mtu": 45 => Target::Scalar,
            "default_interfaces": 46 => Target::Model("AVDDesignDefaultInterfacesList"),
            "default_mgmt_method": 47 => Target::Scalar,
            "default_network_ports_description": 48 => Target::Scalar,
            "default_network_ports_port_channel_description": 49 => Target::Scalar,
            "default_node_types": 50 => Target::Model("AVDDesignDefaultNodeTypesList"),
            "default_underlay_p2p_ethernet_description": 51 => Target::Scalar,
            "default_underlay_p2p_port_channel_description": 52 => Target::Scalar,
            "default_vrf_diag_loopback_description": 53 => Target::Scalar,
            "device_profile": 54 => Target::Scalar,
            "device_profiles": 55 => Target::Model("AVDDesignDeviceProfilesList"),
            "devices": 56 => Target::Model("AVDDesignDevicesList"),
            "digital_twin": 57 => Target::Model("AVDDesignDigitalTwin"),
            "dns_settings": 58 => Target::Model("AVDDesignDnsSettings"),
            "dot1x_settings": 59 => Target::Model("AVDDesignDot1xSettings"),
            "enable_trunk_groups": 60 => Target::Scalar,
            "eos_designs_custom_templates": 61 => Target::Model("AVDDesignEosDesignsCustomTemplatesList"),
            "eos_designs_documentation": 62 => Target::Model("AVDDesignEosDesignsDocumentation"),
            "eos_designs_keep_tmp_files": 63 => Target::Scalar,
            "eos_designs_return_structured_config": 64 => Target::Scalar,
            "eos_designs_tmp_dir": 65 => Target::Scalar,
            "eos_designs_validate_inputs_batch_size": 66 => Target::Scalar,
            "eos_designs_validate_inputs_template_with_multiprocessing": 67 => Target::Scalar,
            "errdisable_settings": 68 => Target::Model("AVDDesignErrdisableSettings"),
            "event_handlers": 69 => Target::Model("EosCliConfigGenEventHandlersList"),
            "event_monitor": 70 => Target::Model("EosCliConfigGenEventMonitor"),
            "evpn_ebgp_gateway_multihop": 71 => Target::Scalar,
            "evpn_ebgp_multihop": 72 => Target::Scalar,
            "evpn_hostflap_detection": 73 => Target::Model("AVDDesignEvpnHostflapDetection"),
            "evpn_import_pruning": 74 => Target::Scalar,
            "evpn_multicast": 75 => Target::Scalar,
            "evpn_overlay_bgp_rtc": 76 => Target::Scalar,
            "evpn_prevent_readvertise_to_server": 77 => Target::Scalar,
            "evpn_prevent_readvertise_to_server_mode": 78 => Target::Scalar,
            "evpn_short_esi_prefix": 79 => Target::Scalar,
            "evpn_vlan_aware_bundles": 80 => Target::Scalar,
            "evpn_vlan_bundles": 81 => Target::Model("AVDDesignEvpnVlanBundlesList"),
            "fabric_evpn_encapsulation": 82 => Target::Scalar,
            "fabric_flow_tracking": 83 => Target::Model("AVDDesignFabricFlowTracking"),
            "fabric_ip_addressing": 84 => Target::Model("AVDDesignFabricIpAddressing"),
            "fabric_name": 85 => Target::Scalar,
            "fabric_numbering": 86 => Target::Model("AVDDesignFabricNumbering"),
            "fabric_numbering_node_id_pool": 87 => Target::Scalar,
            "fabric_sflow": 88 => Target::Model("AVDDesignFabricSflow"),
            "flow_tracking_settings": 89 => Target::Model("AVDDesignFlowTrackingSettings"),
            "general_settings": 90 => Target::Model("AVDDesignGeneralSettings"),
            "generate_cv_tags": 91 => Target::Model("AVDDesignGenerateCvTags"),
            "hardware_counters": 92 => Target::Model("EosCliConfigGenHardwareCounters"),
            "inband_ztp_bootstrap_file": 93 => Target::Scalar,
            "internal_vlan_order": 94 => Target::Model("AVDDesignInternalVlanOrder"),
            "ipsec_settings": 95 => Target::Model("AVDDesignIpsecSettings"),
            "ipv4_acls": 96 => Target::Model("AVDDesignIpv4AclsList"),
            "ipv4_prefix_list_catalog": 97 => Target::Model("AVDDesignIpv4PrefixListCatalogList"),
            "ipv4_standard_acls": 98 => Target::Model("AVDDesignIpv4StandardAclsList"),
            "ipv6_acls": 99 => Target::Model("AVDDesignIpv6AclsList"),
            "ipv6_mgmt_destination_networks": 100 => Target::Model("AVDDesignIpv6MgmtDestinationNetworksList"),
            "ipv6_mgmt_gateway": 101 => Target::Scalar,
            "ipv6_prefix_list_catalog": 102 => Target::Model("AVDDesignIpv6PrefixListCatalogList"),
            "is_deployed": 103 => Target::Scalar,
            "isis_advertise_passive_only": 104 => Target::Scalar,
            "isis_area_id": 105 => Target::Scalar,
            "isis_default_circuit_type": 106 => Target::Scalar,
            "isis_default_is_type": 107 => Target::Scalar,
            "isis_default_metric": 108 => Target::Scalar,
            "isis_maximum_paths": 109 => Target::Scalar,
            "isis_system_id_format": 110 => Target::Scalar,
            "isis_ti_lfa": 111 => Target::Model("AVDDesignIsisTiLfa"),
            "l2vlan_profiles": 112 => Target::Model("AVDDesignL2vlanProfilesList"),
            "l3_edge": 113 => Target::Model("AVDDesignL3Edge"),
            "l3_interface_profiles": 114 => Target::Model("AVDDesignL3InterfaceProfilesList"),
            "load_interval": 115 => Target::Model("EosCliConfigGenLoadInterval"),
            "logging_settings": 116 => Target::Model("AVDDesignLoggingSettings"),
            "mac_acls": 117 => Target::Model("AVDDesignMacAclsList"),
            "mac_address_table": 118 => Target::Model("EosCliConfigGenMacAddressTable"),
            "management_eapi": 119 => Target::Model("AVDDesignManagementEapi"),
            "management_settings": 120 => Target::Model("AVDDesignManagementSettings"),
            "mgmt_destination_networks": 121 => Target::Model("AVDDesignMgmtDestinationNetworksList"),
            "mgmt_gateway": 122 => Target::Scalar,
            "mgmt_interface": 123 => Target::Scalar,
            "mgmt_interface_description": 124 => Target::Scalar,
            "mgmt_interface_settings": 125 => Target::Model("AVDDesignMgmtInterfaceSettings"),
            "mgmt_interface_vrf": 126 => Target::Scalar,
            "mgmt_vrf_routing": 127 => Target::Scalar,
            "mlag_bgp_peer_description": 128 => Target::Scalar,
            "mlag_bgp_peer_group_description": 129 => Target::Scalar,
            "mlag_ibgp_peering_vrfs": 130 => Target::Model("AVDDesignMlagIbgpPeeringVrfs"),
            "mlag_member_description": 131 => Target::Scalar,
            "mlag_on_orphan_port_channel_downlink": 132 => Target::Scalar,
            "mlag_peer_l3_svi_description": 133 => Target::Scalar,
            "mlag_peer_l3_vlan_name": 134 => Target::Scalar,
            "mlag_peer_l3_vrf_svi_description": 135 => Target::Scalar,
            "mlag_peer_l3_vrf_vlan_name": 136 => Target::Scalar,
            "mlag_peer_svi_description": 137 => Target::Scalar,
            "mlag_peer_vlan_name": 138 => Target::Scalar,
            "mlag_port_channel_description": 139 => Target::Scalar,
            "monitor_connectivity": 140 => Target::Model("AVDDesignMonitorConnectivity"),
            "network_ports": 141 => Target::Model("AVDDesignNetworkPortsList"),
            "network_services": 142 => Target::Model("AVDDesignNetworkServicesList"),
            "network_services_keys": 143 => Target::Model("AVDDesignNetworkServicesKeysList"),
            "custom_node_type_keys": 144 => Target::Model("AVDDesignCustomNodeTypeKeysList"),
            "node_type_keys": 145 => Target::Model("AVDDesignNodeTypeKeysList"),
            "ntp_settings": 146 => Target::Model("AVDDesignNtpSettings"),
            "only_local_vlan_trunk_groups": 147 => Target::Scalar,
            "overlay_bgp_peer_description": 148 => Target::Scalar,
            "overlay_cvx_servers": 149 => Target::Model("AVDDesignOverlayCvxServersList"),
            "overlay_her_flood_list_per_vni": 150 => Target::Scalar,
            "overlay_her_flood_list_scope": 151 => Target::Scalar,
            "overlay_mlag_rfc5549": 152 => Target::Scalar,
            "overlay_rd_type": 153 => Target::Model("AVDDesignOverlayRdType"),
            "overlay_routing_protocol": 154 => Target::Scalar,
            "overlay_routing_protocol_address_family": 155 => Target::Scalar,
            "overlay_rt_type": 156 => Target::Model("AVDDesignOverlayRtType"),
            "p2p_uplinks_mtu": 157 => Target::Scalar,
            "p2p_uplinks_qos_profile": 158 => Target::Scalar,
            "custom_platform_settings": 159 => Target::Model("AVDDesignCustomPlatformSettingsList"),
            "platform_settings": 160 => Target::Model("AVDDesignPlatformSettingsList"),
            "platform_speed_groups": 161 => Target::Model("AVDDesignPlatformSpeedGroupsList"),
            "pod_name": 162 => Target::Scalar,
            "port_profiles": 163 => Target::Model("AVDDesignPortProfilesList"),
            "ptp_profiles": 164 => Target::Model("AVDDesignPtpProfilesList"),
            "ptp_settings": 165 => Target::Model("AVDDesignPtpSettings"),
            "queue_monitor_length": 166 => Target::Model("AVDDesignQueueMonitorLength"),
            "queue_monitor_streaming": 167 => Target::Model("EosCliConfigGenQueueMonitorStreaming"),
            "redundancy": 168 => Target::Model("AVDDesignRedundancy"),
            "router_id_loopback_description": 169 => Target::Scalar,
            "serial_number": 170 => Target::Scalar,
            "sflow_settings": 171 => Target::Model("AVDDesignSflowSettings"),
            "shutdown_bgp_towards_undeployed_peers": 172 => Target::Scalar,
            "shutdown_interfaces_towards_undeployed_peers": 173 => Target::Scalar,
            "snmp_settings": 174 => Target::Model("AVDDesignSnmpSettings"),
            "source_interfaces": 175 => Target::Model("AVDDesignSourceInterfaces"),
            "spanning_tree_settings": 176 => Target::Model("AVDDesignSpanningTreeSettings"),
            "ssh_settings": 177 => Target::Model("AVDDesignSshSettings"),
            "svi_profiles": 178 => Target::Model("AVDDesignSviProfilesList"),
            "system_mac_address": 179 => Target::Scalar,
            "tcam_profiles": 180 => Target::Model("EosCliConfigGenTcamProfileProfilesList"),
            "timezone": 181 => Target::Scalar,
            "trunk_groups": 182 => Target::Model("AVDDesignTrunkGroups"),
            "field_type": 183 => Target::Scalar,
            "underlay_filter_peer_as": 184 => Target::Scalar,
            "underlay_filter_redistribute_connected": 185 => Target::Scalar,
            "underlay_ipv6": 186 => Target::Scalar,
            "underlay_ipv6_numbered": 187 => Target::Scalar,
            "underlay_isis_authentication_cleartext_key": 188 => Target::Scalar,
            "underlay_isis_authentication_key": 189 => Target::Scalar,
            "underlay_isis_authentication_mode": 190 => Target::Scalar,
            "underlay_isis_bfd": 191 => Target::Scalar,
            "underlay_isis_instance_name": 192 => Target::Scalar,
            "underlay_l2_ethernet_description": 193 => Target::Scalar,
            "underlay_l2_port_channel_description": 194 => Target::Scalar,
            "underlay_multicast_anycast_rp": 195 => Target::Model("AVDDesignUnderlayMulticastAnycastRp"),
            "underlay_multicast_pim_sm": 196 => Target::Scalar,
            "underlay_multicast_rps": 197 => Target::Model("AVDDesignUnderlayMulticastRpsList"),
            "underlay_multicast_static": 198 => Target::Scalar,
            "underlay_ospf_area": 199 => Target::Scalar,
            "underlay_ospf_authentication": 200 => Target::Model("AVDDesignUnderlayOspfAuthentication"),
            "underlay_ospf_bfd_enable": 201 => Target::Scalar,
            "underlay_ospf_graceful_restart": 202 => Target::Scalar,
            "underlay_ospf_max_lsa": 203 => Target::Scalar,
            "underlay_ospf_maximum_paths": 204 => Target::Scalar,
            "underlay_ospf_process_id": 205 => Target::Scalar,
            "underlay_rfc5549": 206 => Target::Scalar,
            "underlay_routing_protocol": 207 => Target::Scalar,
            "unsupported_transceiver": 208 => Target::Model("EosCliConfigGenServiceUnsupportedTransceiver"),
            "uplink_ptp": 209 => Target::Model("AVDDesignUplinkPtp"),
            "use_cv_topology": 210 => Target::Scalar,
            "use_router_general_for_router_id": 211 => Target::Scalar,
            "validation_profiles": 212 => Target::Model("AVDDesignValidationProfilesList"),
            "vtep_loopback_description": 213 => Target::Scalar,
            "vtep_vvtep_ip": 214 => Target::Scalar,
            "wan_carriers": 215 => Target::Model("AVDDesignWanCarriersList"),
            "wan_encapsulation": 216 => Target::Scalar,
            "wan_ha": 217 => Target::Model("AVDDesignWanHa"),
            "wan_ipsec_profiles": 218 => Target::Model("AVDDesignWanIpsecProfiles"),
            "wan_mode": 219 => Target::Scalar,
            "wan_path_groups": 220 => Target::Model("AVDDesignWanPathGroupsList"),
            "wan_route_servers": 221 => Target::Model("AVDDesignWanRouteServersList"),
            "wan_stun_dtls_disable": 222 => Target::Scalar,
            "wan_stun_dtls_profile_name": 223 => Target::Scalar,
            "wan_virtual_topologies": 224 => Target::Model("AVDDesignWanVirtualTopologies"),
            "zscaler_endpoints": 225 => Target::Model("AVDDesignZscalerEndpoints"),
        };
        "AVDDesignAaaSettings" => dict {
            "enable_password": 0 => Target::Model("AVDDesignAaaSettingsEnablePassword"),
            "tacacs": 1 => Target::Model("AVDDesignAaaSettingsTacacs"),
            "radius": 2 => Target::Model("AVDDesignAaaSettingsRadius"),
            "authentication": 3 => Target::Model("AVDDesignAaaSettingsAuthentication"),
            "authorization": 4 => Target::Model("AVDDesignAaaSettingsAuthorization"),
            "accounting": 5 => Target::Model("AVDDesignAaaSettingsAccounting"),
            "root_login": 6 => Target::Model("AVDDesignAaaSettingsRootLogin"),
            "local_users": 7 => Target::Model("AVDDesignAaaSettingsLocalUsersList"),
        };
        "AVDDesignAaaSettingsEnablePassword" => dict {
            "password": 0 => Target::Scalar,
            "cleartext_password": 1 => Target::Scalar,
            "password_type": 2 => Target::Scalar,
        };
        "AVDDesignAaaSettingsTacacs" => dict {
            "servers": 0 => Target::Model("AVDDesignAaaSettingsTacacsServersList"),
            "vrfs": 1 => Target::Model("AVDDesignAaaSettingsTacacsVrfsList"),
            "policy": 2 => Target::Model("AVDDesignAaaSettingsTacacsPolicy"),
        };
        "AVDDesignAaaSettingsTacacsServersList" => list(Target::Model("AVDDesignAaaSettingsTacacsServersItems"));
        "AVDDesignAaaSettingsTacacsServersItems" => dict {
            "host": 0 => Target::Scalar,
            "groups": 1 => Target::Model("AVDDesignAaaSettingsTacacsServersItemsGroupsList"),
            "vrf": 2 => Target::Scalar,
            "timeout": 3 => Target::Scalar,
            "key": 4 => Target::Scalar,
            "cleartext_key": 5 => Target::Scalar,
        };
        "AVDDesignAaaSettingsTacacsServersItemsGroupsList" => list(Target::Scalar);
        "AVDDesignAaaSettingsTacacsVrfsList" => indexed(Target::Model("AVDDesignAaaSettingsTacacsVrfsListIndexedItem"), [0]);
        "AVDDesignAaaSettingsTacacsVrfsItems" => dict {
            "name": 0 => Target::Scalar,
            "source_interface": 1 => Target::Scalar,
        };
        "AVDDesignAaaSettingsTacacsPolicy" => dict {
            "ignore_unknown_mandatory_attribute": 0 => Target::Scalar,
        };
        "AVDDesignAaaSettingsRadius" => dict {
            "servers": 0 => Target::Model("AVDDesignAaaSettingsRadiusServersList"),
            "vrfs": 1 => Target::Model("AVDDesignAaaSettingsRadiusVrfsList"),
        };
        "AVDDesignAaaSettingsRadiusServersList" => list(Target::Model("AVDDesignAaaSettingsRadiusServersItems"));
        "AVDDesignAaaSettingsRadiusServersItems" => dict {
            "host": 0 => Target::Scalar,
            "groups": 1 => Target::Model("AVDDesignAaaSettingsRadiusServersItemsGroupsList"),
            "vrf": 2 => Target::Scalar,
            "timeout": 3 => Target::Scalar,
            "retransmit": 4 => Target::Scalar,
            "key": 5 => Target::Scalar,
            "cleartext_key": 6 => Target::Scalar,
            "tls": 7 => Target::Model("EosCliConfigGenRadiusServerServersItemsTls"),
        };
        "AVDDesignAaaSettingsRadiusServersItemsGroupsList" => list(Target::Scalar);
        "AVDDesignAaaSettingsRadiusVrfsList" => indexed(Target::Model("AVDDesignAaaSettingsRadiusVrfsListIndexedItem"), [0]);
        "AVDDesignAaaSettingsRadiusVrfsItems" => dict {
            "name": 0 => Target::Scalar,
            "source_interface": 1 => Target::Scalar,
        };
        "AVDDesignAaaSettingsAuthentication" => dict {
            "login": 0 => Target::Model("EosCliConfigGenAaaAuthenticationLogin"),
            "enable": 1 => Target::Model("EosCliConfigGenAaaAuthenticationEnable"),
            "policies": 2 => Target::Model("EosCliConfigGenAaaAuthenticationPolicies"),
        };
        "AVDDesignAaaSettingsAuthorization" => dict {
            "policy": 0 => Target::Model("EosCliConfigGenAaaAuthorizationPolicy"),
            "exec": 1 => Target::Model("EosCliConfigGenAaaAuthorizationExec"),
            "config_commands": 2 => Target::Scalar,
            "serial_console": 3 => Target::Scalar,
            "commands": 4 => Target::Model("EosCliConfigGenAaaAuthorizationCommands"),
        };
        "AVDDesignAaaSettingsAccounting" => dict {
            "exec": 0 => Target::Model("EosCliConfigGenAaaAccountingExec"),
            "system": 1 => Target::Model("EosCliConfigGenAaaAccountingSystem"),
            "commands": 2 => Target::Model("EosCliConfigGenAaaAccountingCommands"),
        };
        "AVDDesignAaaSettingsRootLogin" => dict {
            "enabled": 0 => Target::Scalar,
            "sha512_password": 1 => Target::Scalar,
        };
        "AVDDesignAaaSettingsLocalUsersList" => indexed(Target::Model("AVDDesignAaaSettingsLocalUsersListIndexedItem"), [3]);
        "AVDDesignAaaSettingsLocalUsersItems" => dict {
            "sha512_password": 0 => Target::Scalar,
            "cleartext_password": 1 => Target::Scalar,
            "password_type": 2 => Target::Scalar,
            "name": 3 => Target::Scalar,
            "disabled": 4 => Target::Scalar,
            "privilege": 5 => Target::Scalar,
            "role": 6 => Target::Scalar,
            "no_password": 7 => Target::Scalar,
            "ssh_key": 8 => Target::Scalar,
            "secondary_ssh_key": 9 => Target::Scalar,
            "shell": 10 => Target::Scalar,
        };
        "AVDDesignAddressLockingSettings" => dict {
            "local_interface": 0 => Target::Scalar,
            "dhcp_server_interfaces": 1 => Target::Model("AVDDesignAddressLockingSettingsDhcpServerInterfacesList"),
            "dhcp_servers_ipv4": 2 => Target::Model("AVDDesignAddressLockingSettingsDhcpServersIpv4List"),
            "disabled": 3 => Target::Scalar,
            "leases": 4 => Target::Model("AVDDesignAddressLockingSettingsLeasesList"),
            "locked_address": 5 => Target::Model("AVDDesignAddressLockingSettingsLockedAddress"),
        };
        "AVDDesignAddressLockingSettingsDhcpServerInterfacesList" => list(Target::Scalar);
        "AVDDesignAddressLockingSettingsDhcpServersIpv4List" => list(Target::Scalar);
        "AVDDesignAddressLockingSettingsLeasesList" => list(Target::Model("AVDDesignAddressLockingSettingsLeasesItems"));
        "AVDDesignAddressLockingSettingsLeasesItems" => dict {
            "ip": 0 => Target::Scalar,
            "mac": 1 => Target::Scalar,
        };
        "AVDDesignAddressLockingSettingsLockedAddress" => dict {
            "expiration_mac_disabled": 0 => Target::Scalar,
            "ipv4_enforcement_disabled": 1 => Target::Scalar,
            "ipv6_enforcement_disabled": 2 => Target::Scalar,
        };
        "AVDDesignAvdDesignFuture" => dict {
            "accept_dhcp_default_route_for_mgmt_ip_dhcp": 0 => Target::Scalar,
            "accept_ra_default_route_for_ipv6_mgmt_ip_auto_config": 1 => Target::Scalar,
            "accept_dhcp_default_route_for_inband_mgmt_ip_dhcp": 2 => Target::Scalar,
            "allow_recursive_profile_inheritance": 3 => Target::Scalar,
            "configure_inband_mgmt_ipv6_vrf": 4 => Target::Scalar,
            "consistent_uplink_vlans": 5 => Target::Scalar,
            "fix_address_locking_dhcp_server_interfaces": 6 => Target::Scalar,
            "fix_match_ipv6_prefix_list_on_mlag_route_map": 7 => Target::Scalar,
            "fix_radius_server_group_tls": 8 => Target::Scalar,
            "only_configure_ipv6_inband_mgmt_prefix_list_when_used": 9 => Target::Scalar,
            "only_configure_mlag_vrfs_peer_group_when_used": 10 => Target::Scalar,
            "only_configure_pvst_border_when_mode_is_mstp": 11 => Target::Scalar,
            "only_configure_route_map_connected_to_bgp_vrfs_when_used": 12 => Target::Scalar,
            "raise_for_port_channels_without_members": 13 => Target::Scalar,
            "raise_for_underlay_router_with_uplink_type_port_channel": 14 => Target::Scalar,
            "remove_redundant_ipv4_unicast_for_peer_groups": 15 => Target::Scalar,
        };
        "AVDDesignEosDesignsValidationConfiguration" => dict {
            "warn_eos_config_keys": 0 => Target::Scalar,
        };
        "AVDDesignBfdMultihop" => dict {
            "interval": 0 => Target::Scalar,
            "min_rx": 1 => Target::Scalar,
            "multiplier": 2 => Target::Scalar,
        };
        "AVDDesignBgpGracefulRestart" => dict {
            "enabled": 0 => Target::Scalar,
            "restart_time": 1 => Target::Scalar,
        };
        "AVDDesignBgpPeerGroups" => dict {
            "ipv4_underlay_peers": 0 => Target::Model("AVDDesignBgpPeerGroupsIpv4UnderlayPeers"),
            "mlag_ipv4_vrfs_peer": 1 => Target::Model("AVDDesignBgpPeerGroupsMlagIpv4VrfsPeer"),
            "mlag_ipv4_underlay_peer": 2 => Target::Model("AVDDesignBgpPeerGroupsMlagIpv4UnderlayPeer"),
            "evpn_overlay_peers": 3 => Target::Model("AVDDesignBgpPeerGroupsEvpnOverlayPeers"),
            "evpn_overlay_core": 4 => Target::Model("AVDDesignBgpPeerGroupsEvpnOverlayCore"),
            "mpls_overlay_peers": 5 => Target::Model("AVDDesignBgpPeerGroupsMplsOverlayPeers"),
            "rr_overlay_peers": 6 => Target::Model("AVDDesignBgpPeerGroupsRrOverlayPeers"),
            "ipvpn_gateway_peers": 7 => Target::Model("AVDDesignBgpPeerGroupsIpvpnGatewayPeers"),
            "wan_overlay_peers": 8 => Target::Model("AVDDesignBgpPeerGroupsWanOverlayPeers"),
            "wan_rr_overlay_peers": 9 => Target::Model("AVDDesignBgpPeerGroupsWanRrOverlayPeers"),
        };
        "AVDDesignBgpPeerGroupsIpv4UnderlayPeers" => dict {
            "name": 0 => Target::Scalar,
            "password": 1 => Target::Scalar,
            "cleartext_password": 2 => Target::Scalar,
            "bfd": 3 => Target::Scalar,
            "maximum_routes": 4 => Target::Scalar,
            "structured_config": 5 => Target::Opaque,
        };
        "AVDDesignBgpPeerGroupsMlagIpv4VrfsPeer" => dict {
            "name": 0 => Target::Scalar,
            "password": 1 => Target::Scalar,
            "cleartext_password": 2 => Target::Scalar,
            "bfd": 3 => Target::Scalar,
            "maximum_routes": 4 => Target::Scalar,
            "structured_config": 5 => Target::Opaque,
        };
        "AVDDesignBgpPeerGroupsMlagIpv4UnderlayPeer" => dict {
            "name": 0 => Target::Scalar,
            "password": 1 => Target::Scalar,
            "cleartext_password": 2 => Target::Scalar,
            "bfd": 3 => Target::Scalar,
            "maximum_routes": 4 => Target::Scalar,
            "structured_config": 5 => Target::Opaque,
        };
        "AVDDesignBgpPeerGroupsEvpnOverlayPeers" => dict {
            "name": 0 => Target::Scalar,
            "password": 1 => Target::Scalar,
            "cleartext_password": 2 => Target::Scalar,
            "bfd": 3 => Target::Scalar,
            "structured_config": 4 => Target::Opaque,
        };
        "AVDDesignBgpPeerGroupsEvpnOverlayCore" => dict {
            "name": 0 => Target::Scalar,
            "password": 1 => Target::Scalar,
            "cleartext_password": 2 => Target::Scalar,
            "bfd": 3 => Target::Scalar,
            "structured_config": 4 => Target::Opaque,
        };
        "AVDDesignBgpPeerGroupsMplsOverlayPeers" => dict {
            "name": 0 => Target::Scalar,
            "password": 1 => Target::Scalar,
            "cleartext_password": 2 => Target::Scalar,
            "bfd": 3 => Target::Scalar,
            "structured_config": 4 => Target::Opaque,
        };
        "AVDDesignBgpPeerGroupsRrOverlayPeers" => dict {
            "name": 0 => Target::Scalar,
            "password": 1 => Target::Scalar,
            "cleartext_password": 2 => Target::Scalar,
            "bfd": 3 => Target::Scalar,
            "structured_config": 4 => Target::Opaque,
        };
        "AVDDesignBgpPeerGroupsIpvpnGatewayPeers" => dict {
            "name": 0 => Target::Scalar,
            "password": 1 => Target::Scalar,
            "cleartext_password": 2 => Target::Scalar,
            "bfd": 3 => Target::Scalar,
            "structured_config": 4 => Target::Opaque,
        };
        "AVDDesignBgpPeerGroupsWanOverlayPeers" => dict {
            "name": 0 => Target::Scalar,
            "password": 1 => Target::Scalar,
            "cleartext_password": 2 => Target::Scalar,
            "bfd": 3 => Target::Scalar,
            "bfd_timers": 4 => Target::Model("AVDDesignBgpPeerGroupsWanOverlayPeersBfdTimers"),
            "listen_range_prefixes": 5 => Target::Model("AVDDesignBgpPeerGroupsWanOverlayPeersListenRangePrefixesList"),
            "ttl_maximum_hops": 6 => Target::Scalar,
            "structured_config": 7 => Target::Opaque,
        };
        "AVDDesignBgpPeerGroupsWanOverlayPeersBfdTimers" => dict {
            "interval": 0 => Target::Scalar,
            "min_rx": 1 => Target::Scalar,
            "multiplier": 2 => Target::Scalar,
        };
        "AVDDesignBgpPeerGroupsWanOverlayPeersListenRangePrefixesList" => list(Target::Scalar);
        "AVDDesignBgpPeerGroupsWanRrOverlayPeers" => dict {
            "name": 0 => Target::Scalar,
            "password": 1 => Target::Scalar,
            "cleartext_password": 2 => Target::Scalar,
            "bfd": 3 => Target::Scalar,
            "bfd_timers": 4 => Target::Model("AVDDesignBgpPeerGroupsWanRrOverlayPeersBfdTimers"),
            "ttl_maximum_hops": 5 => Target::Scalar,
            "structured_config": 6 => Target::Opaque,
        };
        "AVDDesignBgpPeerGroupsWanRrOverlayPeersBfdTimers" => dict {
            "interval": 0 => Target::Scalar,
            "min_rx": 1 => Target::Scalar,
            "multiplier": 2 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsList" => indexed(Target::Model("AVDDesignConnectedEndpointsListIndexedItem"), [0]);
        "AVDDesignConnectedEndpointsItems" => dict {
            "name": 0 => Target::Scalar,
            "field_type": 1 => Target::Scalar,
            "rack": 2 => Target::Scalar,
            "adapters": 3 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersList"),
        };
        "AVDDesignConnectedEndpointsItemsAdaptersList" => list(Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItems"));
        "AVDDesignConnectedEndpointsItemsAdaptersItems" => dict {
            "switch_ports": 0 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsSwitchPortsList"),
            "switches": 1 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsSwitchesList"),
            "endpoint_ports": 2 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsEndpointPortsList"),
            "descriptions": 3 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsDescriptionsList"),
            "subinterfaces": 4 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsSubinterfacesList"),
            "port_channel": 5 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsPortChannel"),
            "speed": 6 => Target::Scalar,
            "description": 7 => Target::Scalar,
            "profile": 8 => Target::Scalar,
            "enabled": 9 => Target::Scalar,
            "mode": 10 => Target::Scalar,
            "mtu": 11 => Target::Scalar,
            "l2_mtu": 12 => Target::Scalar,
            "l2_mru": 13 => Target::Scalar,
            "native_vlan": 14 => Target::Scalar,
            "native_vlan_tag": 15 => Target::Scalar,
            "phone_vlan": 16 => Target::Scalar,
            "phone_trunk_mode": 17 => Target::Scalar,
            "trunk_groups": 18 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsTrunkGroupsList"),
            "vlans": 19 => Target::Scalar,
            "mac_acl_in": 20 => Target::Scalar,
            "mac_acl_out": 21 => Target::Scalar,
            "spanning_tree_portfast": 22 => Target::Scalar,
            "spanning_tree_bpdufilter": 23 => Target::Scalar,
            "spanning_tree_bpduguard": 24 => Target::Scalar,
            "spanning_tree_link_type": 25 => Target::Scalar,
            "flowcontrol": 26 => Target::Model("EosCliConfigGenEthernetInterfacesItemsFlowcontrol"),
            "qos_profile": 27 => Target::Scalar,
            "ptp": 28 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsPtp"),
            "sflow": 29 => Target::Scalar,
            "flow_tracking": 30 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsFlowTracking"),
            "link_tracking": 31 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsLinkTracking"),
            "dot1x": 32 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsDot1x"),
            "address_locking": 33 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsAddressLocking"),
            "poe": 34 => Target::Model("EosCliConfigGenEthernetInterfacesItemsPoe"),
            "storm_control": 35 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsStormControl"),
            "monitor_sessions": 36 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsMonitorSessionsList"),
            "ethernet_segment": 37 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsEthernetSegment"),
            "validate_state": 38 => Target::Scalar,
            "validate_lldp": 39 => Target::Scalar,
            "campus_link_type": 40 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsCampusLinkTypeList"),
            "raw_eos_cli": 41 => Target::Scalar,
            "structured_config": 42 => Target::Opaque,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsSwitchPortsList" => list(Target::Scalar);
        "AVDDesignConnectedEndpointsItemsAdaptersItemsSwitchesList" => list(Target::Scalar);
        "AVDDesignConnectedEndpointsItemsAdaptersItemsEndpointPortsList" => list(Target::Scalar);
        "AVDDesignConnectedEndpointsItemsAdaptersItemsDescriptionsList" => list(Target::Scalar);
        "AVDDesignConnectedEndpointsItemsAdaptersItemsSubinterfacesList" => indexed(Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsSubinterfacesListIndexedItem"), [0]);
        "AVDDesignConnectedEndpointsItemsAdaptersItemsSubinterfacesItems" => dict {
            "number": 0 => Target::Scalar,
            "description": 1 => Target::Scalar,
            "short_esi": 2 => Target::Scalar,
            "vlan_id": 3 => Target::Scalar,
            "encapsulation_vlan": 4 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsSubinterfacesItemsEncapsulationVlan"),
            "raw_eos_cli": 5 => Target::Scalar,
            "structured_config": 6 => Target::Opaque,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsSubinterfacesItemsEncapsulationVlan" => dict {
            "client_dot1q": 0 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsPortChannel" => dict {
            "subinterfaces": 0 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsPortChannelSubinterfacesList"),
            "mode": 1 => Target::Scalar,
            "channel_id": 2 => Target::Scalar,
            "description": 3 => Target::Scalar,
            "endpoint_port_channel": 4 => Target::Scalar,
            "enabled": 5 => Target::Scalar,
            "ptp_mpass": 6 => Target::Scalar,
            "lacp_fallback": 7 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsPortChannelLacpFallback"),
            "lacp_timer": 8 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsPortChannelLacpTimer"),
            "raw_eos_cli": 9 => Target::Scalar,
            "structured_config": 10 => Target::Opaque,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsPortChannelSubinterfacesList" => indexed(Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsPortChannelSubinterfacesListIndexedItem"), [0]);
        "AVDDesignConnectedEndpointsItemsAdaptersItemsPortChannelSubinterfacesItems" => dict {
            "number": 0 => Target::Scalar,
            "description": 1 => Target::Scalar,
            "short_esi": 2 => Target::Scalar,
            "vlan_id": 3 => Target::Scalar,
            "encapsulation_vlan": 4 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsPortChannelSubinterfacesItemsEncapsulationVlan"),
            "raw_eos_cli": 5 => Target::Scalar,
            "structured_config": 6 => Target::Opaque,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsPortChannelSubinterfacesItemsEncapsulationVlan" => dict {
            "client_dot1q": 0 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsPortChannelLacpFallback" => dict {
            "mode": 0 => Target::Scalar,
            "individual": 1 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsPortChannelLacpFallbackIndividual"),
            "timeout": 2 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsPortChannelLacpFallbackIndividual" => dict {
            "profile": 0 => Target::Scalar,
            "vlans": 1 => Target::Scalar,
            "native_vlan": 2 => Target::Scalar,
            "mode": 3 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsPortChannelLacpTimer" => dict {
            "mode": 0 => Target::Scalar,
            "multiplier": 1 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsTrunkGroupsList" => list(Target::Scalar);
        "AVDDesignConnectedEndpointsItemsAdaptersItemsPtp" => dict {
            "enabled": 0 => Target::Scalar,
            "endpoint_role": 1 => Target::Scalar,
            "profile": 2 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsFlowTracking" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsLinkTracking" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsDot1x" => dict {
            "authentication_failure": 0 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsDot1xAuthenticationFailure"),
            "port_control": 1 => Target::Scalar,
            "port_control_force_authorized_phone": 2 => Target::Scalar,
            "reauthentication": 3 => Target::Scalar,
            "pae": 4 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsDot1xPae"),
            "host_mode": 5 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsDot1xHostMode"),
            "mac_based_authentication": 6 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsDot1xMacBasedAuthentication"),
            "mac_based_access_list": 7 => Target::Scalar,
            "timeout": 8 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsDot1xTimeout"),
            "reauthorization_request_limit": 9 => Target::Scalar,
            "unauthorized": 10 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsDot1xUnauthorized"),
            "eapol": 11 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsDot1xEapol"),
            "aaa": 12 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsDot1xAaa"),
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsDot1xAuthenticationFailure" => dict {
            "allow_access_list": 0 => Target::Scalar,
            "action": 1 => Target::Scalar,
            "allow_vlan": 2 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsDot1xPae" => dict {
            "mode": 0 => Target::Scalar,
            "supplicant_profile": 1 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsDot1xHostMode" => dict {
            "mode": 0 => Target::Scalar,
            "multi_host_authenticated": 1 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsDot1xMacBasedAuthentication" => dict {
            "enabled": 0 => Target::Scalar,
            "always": 1 => Target::Scalar,
            "host_mode_common": 2 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsDot1xTimeout" => dict {
            "idle_host": 0 => Target::Scalar,
            "quiet_period": 1 => Target::Scalar,
            "reauth_period": 2 => Target::Scalar,
            "reauth_timeout_ignore": 3 => Target::Scalar,
            "tx_period": 4 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsDot1xUnauthorized" => dict {
            "access_vlan_membership_egress": 0 => Target::Scalar,
            "native_vlan_membership_egress": 1 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsDot1xEapol" => dict {
            "disabled": 0 => Target::Scalar,
            "authentication_failure_fallback_mba": 1 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsDot1xEapolAuthenticationFailureFallbackMba"),
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsDot1xEapolAuthenticationFailureFallbackMba" => dict {
            "enabled": 0 => Target::Scalar,
            "timeout": 1 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsDot1xAaa" => dict {
            "unresponsive": 0 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsDot1xAaaUnresponsive"),
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsDot1xAaaUnresponsive" => dict {
            "eap_response": 0 => Target::Scalar,
            "action": 1 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsDot1xAaaUnresponsiveAction"),
            "phone_action": 2 => Target::Model("EosCliConfigGenDot1xAaaUnresponsivePhoneAction"),
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsDot1xAaaUnresponsiveAction" => dict {
            "traffic_allow_access_list": 0 => Target::Scalar,
            "apply_alternate": 1 => Target::Scalar,
            "traffic_allow_vlan": 2 => Target::Scalar,
            "apply_cached_results": 3 => Target::Scalar,
            "cached_results_timeout": 4 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsDot1xAaaUnresponsiveActionCachedResultsTimeout"),
            "traffic_allow": 5 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsDot1xAaaUnresponsiveActionCachedResultsTimeout" => dict {
            "time_duration": 0 => Target::Scalar,
            "time_duration_unit": 1 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsAddressLocking" => dict {
            "ipv4": 0 => Target::Scalar,
            "ipv6": 1 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsStormControl" => dict {
            "all": 0 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsStormControlAll"),
            "broadcast": 1 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsStormControlBroadcast"),
            "multicast": 2 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsStormControlMulticast"),
            "unknown_unicast": 3 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsStormControlUnknownUnicast"),
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsStormControlAll" => dict {
            "level": 0 => Target::Scalar,
            "unit": 1 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsStormControlBroadcast" => dict {
            "level": 0 => Target::Scalar,
            "unit": 1 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsStormControlMulticast" => dict {
            "level": 0 => Target::Scalar,
            "unit": 1 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsStormControlUnknownUnicast" => dict {
            "level": 0 => Target::Scalar,
            "unit": 1 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsMonitorSessionsList" => list(Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsMonitorSessionsItems"));
        "AVDDesignConnectedEndpointsItemsAdaptersItemsMonitorSessionsItems" => dict {
            "name": 0 => Target::Scalar,
            "role": 1 => Target::Scalar,
            "source_settings": 2 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsMonitorSessionsItemsSourceSettings"),
            "session_settings": 3 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsMonitorSessionsItemsSessionSettings"),
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsMonitorSessionsItemsSourceSettings" => dict {
            "direction": 0 => Target::Scalar,
            "access_group": 1 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsMonitorSessionsItemsSourceSettingsAccessGroup"),
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsMonitorSessionsItemsSourceSettingsAccessGroup" => dict {
            "field_type": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
            "priority": 2 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsMonitorSessionsItemsSessionSettings" => dict {
            "encapsulation_gre_metadata_tx": 0 => Target::Scalar,
            "header_remove_size": 1 => Target::Scalar,
            "access_group": 2 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsMonitorSessionsItemsSessionSettingsAccessGroup"),
            "rate_limit_per_ingress_chip": 3 => Target::Scalar,
            "rate_limit_per_egress_chip": 4 => Target::Scalar,
            "sample": 5 => Target::Scalar,
            "truncate": 6 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsMonitorSessionsItemsSessionSettingsTruncate"),
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsMonitorSessionsItemsSessionSettingsAccessGroup" => dict {
            "field_type": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsMonitorSessionsItemsSessionSettingsTruncate" => dict {
            "enabled": 0 => Target::Scalar,
            "size": 1 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsEthernetSegment" => dict {
            "short_esi": 0 => Target::Scalar,
            "redundancy": 1 => Target::Scalar,
            "designated_forwarder_algorithm": 2 => Target::Scalar,
            "designated_forwarder_preferences": 3 => Target::Model("AVDDesignConnectedEndpointsItemsAdaptersItemsEthernetSegmentDesignatedForwarderPreferencesList"),
            "dont_preempt": 4 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsItemsAdaptersItemsEthernetSegmentDesignatedForwarderPreferencesList" => list(Target::Scalar);
        "AVDDesignConnectedEndpointsItemsAdaptersItemsCampusLinkTypeList" => list(Target::Scalar);
        "AVDDesignCustomConnectedEndpointsKeysList" => indexed(Target::Model("AVDDesignCustomConnectedEndpointsKeysListIndexedItem"), [0]);
        "AVDDesignCustomConnectedEndpointsKeysItems" => dict {
            "key": 0 => Target::Scalar,
            "field_type": 1 => Target::Scalar,
            "description": 2 => Target::Scalar,
        };
        "AVDDesignConnectedEndpointsKeysList" => indexed(Target::Model("AVDDesignConnectedEndpointsKeysListIndexedItem"), [0]);
        "AVDDesignConnectedEndpointsKeysItems" => dict {
            "key": 0 => Target::Scalar,
            "field_type": 1 => Target::Scalar,
            "description": 2 => Target::Scalar,
        };
        "AVDDesignCoreInterfaces" => dict {
            "p2p_links_ip_pools": 0 => Target::Model("AVDDesignCoreInterfacesP2pLinksIpPoolsList"),
            "p2p_links_profiles": 1 => Target::Model("AVDDesignCoreInterfacesP2pLinksProfilesList"),
            "p2p_links": 2 => Target::Model("AVDDesignCoreInterfacesP2pLinksList"),
        };
        "AVDDesignCoreInterfacesP2pLinksIpPoolsList" => indexed(Target::Model("AVDDesignCoreInterfacesP2pLinksIpPoolsListIndexedItem"), [0]);
        "AVDDesignCoreInterfacesP2pLinksIpPoolsItems" => dict {
            "name": 0 => Target::Scalar,
            "ipv4_pool": 1 => Target::Scalar,
            "prefix_size": 2 => Target::Scalar,
            "ipv6_pool": 3 => Target::Scalar,
            "ipv6_prefix_size": 4 => Target::Scalar,
        };
        "AVDDesignCoreInterfacesP2pLinksProfilesList" => indexed(Target::Model("AVDDesignCoreInterfacesP2pLinksProfilesListIndexedItem"), [0]);
        "AVDDesignCoreInterfacesP2pLinksProfilesItems" => dict {
            "name": 0 => Target::Scalar,
            "id": 1 => Target::Scalar,
            "speed": 2 => Target::Scalar,
            "ip_pool": 3 => Target::Scalar,
            "subnet": 4 => Target::Scalar,
            "ip": 5 => Target::Model("AVDDesignCoreInterfacesP2pLinksProfilesItemsIpList"),
            "ipv6_enable": 6 => Target::Scalar,
            "ipv6_prefix": 7 => Target::Scalar,
            "ipv6": 8 => Target::Model("AVDDesignCoreInterfacesP2pLinksProfilesItemsIpv6List"),
            "nodes": 9 => Target::Model("AVDDesignCoreInterfacesP2pLinksProfilesItemsNodesList"),
            "interfaces": 10 => Target::Model("AVDDesignCoreInterfacesP2pLinksProfilesItemsInterfacesList"),
            "field_as": 11 => Target::Model("AVDDesignCoreInterfacesP2pLinksProfilesItemsAsList"),
            "descriptions": 12 => Target::Model("AVDDesignCoreInterfacesP2pLinksProfilesItemsDescriptionsList"),
            "include_in_underlay_protocol": 13 => Target::Scalar,
            "use_underlay_ospf_authentication": 14 => Target::Scalar,
            "isis_hello_padding": 15 => Target::Scalar,
            "isis_metric": 16 => Target::Scalar,
            "isis_circuit_type": 17 => Target::Scalar,
            "isis_authentication_mode": 18 => Target::Scalar,
            "isis_authentication_key": 19 => Target::Scalar,
            "isis_authentication_cleartext_key": 20 => Target::Scalar,
            "isis_network_type": 21 => Target::Scalar,
            "mpls_ip": 22 => Target::Scalar,
            "mpls_ldp": 23 => Target::Scalar,
            "mtu": 24 => Target::Scalar,
            "bfd": 25 => Target::Scalar,
            "ptp": 26 => Target::Model("AVDDesignCoreInterfacesP2pLinksProfilesItemsPtp"),
            "sflow": 27 => Target::Scalar,
            "multicast_pim_sm": 28 => Target::Scalar,
            "multicast_static": 29 => Target::Scalar,
            "flow_tracking": 30 => Target::Model("AVDDesignCoreInterfacesP2pLinksProfilesItemsFlowTracking"),
            "qos_profile": 31 => Target::Scalar,
            "macsec_profile": 32 => Target::Scalar,
            "port_channel": 33 => Target::Model("AVDDesignCoreInterfacesP2pLinksProfilesItemsPortChannel"),
            "campus_link_type": 34 => Target::Model("AVDDesignCoreInterfacesP2pLinksProfilesItemsCampusLinkTypeList"),
            "raw_eos_cli": 35 => Target::Scalar,
            "routing_protocol": 36 => Target::Scalar,
            "ethernet_structured_config": 37 => Target::Opaque,
            "port_channel_structured_config": 38 => Target::Opaque,
        };
        "AVDDesignCoreInterfacesP2pLinksProfilesItemsIpList" => list(Target::Scalar);
        "AVDDesignCoreInterfacesP2pLinksProfilesItemsIpv6List" => list(Target::Scalar);
        "AVDDesignCoreInterfacesP2pLinksProfilesItemsNodesList" => list(Target::Scalar);
        "AVDDesignCoreInterfacesP2pLinksProfilesItemsInterfacesList" => list(Target::Scalar);
        "AVDDesignCoreInterfacesP2pLinksProfilesItemsAsList" => list(Target::Scalar);
        "AVDDesignCoreInterfacesP2pLinksProfilesItemsDescriptionsList" => list(Target::Scalar);
        "AVDDesignCoreInterfacesP2pLinksProfilesItemsPtp" => dict {
            "enabled": 0 => Target::Scalar,
            "roles": 1 => Target::Model("AVDDesignCoreInterfacesP2pLinksProfilesItemsPtpRolesList"),
            "profile": 2 => Target::Scalar,
        };
        "AVDDesignCoreInterfacesP2pLinksProfilesItemsPtpRolesList" => list(Target::Scalar);
        "AVDDesignCoreInterfacesP2pLinksProfilesItemsFlowTracking" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignCoreInterfacesP2pLinksProfilesItemsPortChannel" => dict {
            "description": 0 => Target::Scalar,
            "mode": 1 => Target::Scalar,
            "channel_id_algorithm": 2 => Target::Scalar,
            "channel_id_offset": 3 => Target::Scalar,
            "nodes_child_interfaces": 4 => Target::Model("AVDDesignCoreInterfacesP2pLinksProfilesItemsPortChannelNodesChildInterfacesList"),
        };
        "AVDDesignCoreInterfacesP2pLinksProfilesItemsPortChannelNodesChildInterfacesList" => indexed(Target::Model("AVDDesignCoreInterfacesP2pLinksProfilesItemsPortChannelNodesChildInterfacesListIndexedItem"), [0]);
        "AVDDesignCoreInterfacesP2pLinksProfilesItemsPortChannelNodesChildInterfacesItems" => dict {
            "node": 0 => Target::Scalar,
            "interfaces": 1 => Target::Model("AVDDesignCoreInterfacesP2pLinksProfilesItemsPortChannelNodesChildInterfacesItemsInterfacesList"),
            "channel_id": 2 => Target::Scalar,
        };
        "AVDDesignCoreInterfacesP2pLinksProfilesItemsPortChannelNodesChildInterfacesItemsInterfacesList" => list(Target::Scalar);
        "AVDDesignCoreInterfacesP2pLinksProfilesItemsCampusLinkTypeList" => list(Target::Scalar);
        "AVDDesignCoreInterfacesP2pLinksList" => list(Target::Model("AVDDesignCoreInterfacesP2pLinksItems"));
        "AVDDesignCoreInterfacesP2pLinksItems" => dict {
            "nodes": 0 => Target::Model("AVDDesignCoreInterfacesP2pLinksItemsNodesList"),
            "profile": 1 => Target::Scalar,
            "id": 2 => Target::Scalar,
            "speed": 3 => Target::Scalar,
            "ip_pool": 4 => Target::Scalar,
            "subnet": 5 => Target::Scalar,
            "ip": 6 => Target::Model("AVDDesignCoreInterfacesP2pLinksItemsIpList"),
            "ipv6_enable": 7 => Target::Scalar,
            "ipv6_prefix": 8 => Target::Scalar,
            "ipv6": 9 => Target::Model("AVDDesignCoreInterfacesP2pLinksItemsIpv6List"),
            "interfaces": 10 => Target::Model("AVDDesignCoreInterfacesP2pLinksItemsInterfacesList"),
            "field_as": 11 => Target::Model("AVDDesignCoreInterfacesP2pLinksItemsAsList"),
            "descriptions": 12 => Target::Model("AVDDesignCoreInterfacesP2pLinksItemsDescriptionsList"),
            "include_in_underlay_protocol": 13 => Target::Scalar,
            "use_underlay_ospf_authentication": 14 => Target::Scalar,
            "isis_hello_padding": 15 => Target::Scalar,
            "isis_metric": 16 => Target::Scalar,
            "isis_circuit_type": 17 => Target::Scalar,
            "isis_authentication_mode": 18 => Target::Scalar,
            "isis_authentication_key": 19 => Target::Scalar,
            "isis_authentication_cleartext_key": 20 => Target::Scalar,
            "isis_network_type": 21 => Target::Scalar,
            "mpls_ip": 22 => Target::Scalar,
            "mpls_ldp": 23 => Target::Scalar,
            "mtu": 24 => Target::Scalar,
            "bfd": 25 => Target::Scalar,
            "ptp": 26 => Target::Model("AVDDesignCoreInterfacesP2pLinksItemsPtp"),
            "sflow": 27 => Target::Scalar,
            "multicast_pim_sm": 28 => Target::Scalar,
            "multicast_static": 29 => Target::Scalar,
            "flow_tracking": 30 => Target::Model("AVDDesignCoreInterfacesP2pLinksItemsFlowTracking"),
            "qos_profile": 31 => Target::Scalar,
            "macsec_profile": 32 => Target::Scalar,
            "port_channel": 33 => Target::Model("AVDDesignCoreInterfacesP2pLinksItemsPortChannel"),
            "campus_link_type": 34 => Target::Model("AVDDesignCoreInterfacesP2pLinksItemsCampusLinkTypeList"),
            "raw_eos_cli": 35 => Target::Scalar,
            "routing_protocol": 36 => Target::Scalar,
            "ethernet_structured_config": 37 => Target::Opaque,
            "port_channel_structured_config": 38 => Target::Opaque,
        };
        "AVDDesignCoreInterfacesP2pLinksItemsNodesList" => list(Target::Scalar);
        "AVDDesignCoreInterfacesP2pLinksItemsIpList" => list(Target::Scalar);
        "AVDDesignCoreInterfacesP2pLinksItemsIpv6List" => list(Target::Scalar);
        "AVDDesignCoreInterfacesP2pLinksItemsInterfacesList" => list(Target::Scalar);
        "AVDDesignCoreInterfacesP2pLinksItemsAsList" => list(Target::Scalar);
        "AVDDesignCoreInterfacesP2pLinksItemsDescriptionsList" => list(Target::Scalar);
        "AVDDesignCoreInterfacesP2pLinksItemsPtp" => dict {
            "enabled": 0 => Target::Scalar,
            "roles": 1 => Target::Model("AVDDesignCoreInterfacesP2pLinksItemsPtpRolesList"),
            "profile": 2 => Target::Scalar,
        };
        "AVDDesignCoreInterfacesP2pLinksItemsPtpRolesList" => list(Target::Scalar);
        "AVDDesignCoreInterfacesP2pLinksItemsFlowTracking" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignCoreInterfacesP2pLinksItemsPortChannel" => dict {
            "description": 0 => Target::Scalar,
            "mode": 1 => Target::Scalar,
            "channel_id_algorithm": 2 => Target::Scalar,
            "channel_id_offset": 3 => Target::Scalar,
            "nodes_child_interfaces": 4 => Target::Model("AVDDesignCoreInterfacesP2pLinksItemsPortChannelNodesChildInterfacesList"),
        };
        "AVDDesignCoreInterfacesP2pLinksItemsPortChannelNodesChildInterfacesList" => indexed(Target::Model("AVDDesignCoreInterfacesP2pLinksItemsPortChannelNodesChildInterfacesListIndexedItem"), [0]);
        "AVDDesignCoreInterfacesP2pLinksItemsPortChannelNodesChildInterfacesItems" => dict {
            "node": 0 => Target::Scalar,
            "interfaces": 1 => Target::Model("AVDDesignCoreInterfacesP2pLinksItemsPortChannelNodesChildInterfacesItemsInterfacesList"),
            "channel_id": 2 => Target::Scalar,
        };
        "AVDDesignCoreInterfacesP2pLinksItemsPortChannelNodesChildInterfacesItemsInterfacesList" => list(Target::Scalar);
        "AVDDesignCoreInterfacesP2pLinksItemsCampusLinkTypeList" => list(Target::Scalar);
        "AVDDesignCustomStructuredConfigurationPrefixList" => list(Target::Scalar);
        "AVDDesignCvPathfinderGlobalSitesList" => indexed(Target::Model("AVDDesignCvPathfinderGlobalSitesListIndexedItem"), [0]);
        "AVDDesignCvPathfinderGlobalSitesItems" => dict {
            "name": 0 => Target::Scalar,
            "description": 1 => Target::Scalar,
            "location": 2 => Target::Scalar,
        };
        "AVDDesignCvPathfinderInternetExitPoliciesList" => indexed(Target::Model("AVDDesignCvPathfinderInternetExitPoliciesListIndexedItem"), [0]);
        "AVDDesignCvPathfinderInternetExitPoliciesItems" => dict {
            "name": 0 => Target::Scalar,
            "field_type": 1 => Target::Scalar,
            "fallback_to_system_default": 2 => Target::Scalar,
            "zscaler": 3 => Target::Model("AVDDesignCvPathfinderInternetExitPoliciesItemsZscaler"),
        };
        "AVDDesignCvPathfinderInternetExitPoliciesItemsZscaler" => dict {
            "ipsec_key_salt": 0 => Target::Scalar,
            "domain_name": 1 => Target::Scalar,
            "encrypt_traffic": 2 => Target::Scalar,
            "download_bandwidth": 3 => Target::Scalar,
            "upload_bandwidth": 4 => Target::Scalar,
            "firewall": 5 => Target::Model("AVDDesignCvPathfinderInternetExitPoliciesItemsZscalerFirewall"),
            "acceptable_use_policy": 6 => Target::Scalar,
        };
        "AVDDesignCvPathfinderInternetExitPoliciesItemsZscalerFirewall" => dict {
            "enabled": 0 => Target::Scalar,
            "ips": 1 => Target::Scalar,
        };
        "AVDDesignCvPathfinderRegionsList" => indexed(Target::Model("AVDDesignCvPathfinderRegionsListIndexedItem"), [0]);
        "AVDDesignCvPathfinderRegionsItems" => dict {
            "name": 0 => Target::Scalar,
            "description": 1 => Target::Scalar,
            "id": 2 => Target::Scalar,
            "sites": 3 => Target::Model("AVDDesignCvPathfinderRegionsItemsSitesList"),
        };
        "AVDDesignCvPathfinderRegionsItemsSitesList" => indexed(Target::Model("AVDDesignCvPathfinderRegionsItemsSitesListIndexedItem"), [0]);
        "AVDDesignCvPathfinderRegionsItemsSitesItems" => dict {
            "name": 0 => Target::Scalar,
            "description": 1 => Target::Scalar,
            "id": 2 => Target::Scalar,
            "location": 3 => Target::Scalar,
            "site_contact": 4 => Target::Scalar,
            "site_after_hours_contact": 5 => Target::Scalar,
        };
        "AVDDesignCvSettings" => dict {
            "cvaas": 0 => Target::Model("AVDDesignCvSettingsCvaas"),
            "onprem_clusters": 1 => Target::Model("AVDDesignCvSettingsOnpremClustersList"),
            "terminattr": 2 => Target::Model("AVDDesignCvSettingsTerminattr"),
            "set_source_interfaces": 3 => Target::Scalar,
        };
        "AVDDesignCvSettingsCvaas" => dict {
            "enabled": 0 => Target::Scalar,
            "clusters": 1 => Target::Model("AVDDesignCvSettingsCvaasClustersList"),
        };
        "AVDDesignCvSettingsCvaasClustersList" => indexed(Target::Model("AVDDesignCvSettingsCvaasClustersListIndexedItem"), [0]);
        "AVDDesignCvSettingsCvaasClustersItems" => dict {
            "name": 0 => Target::Scalar,
            "region": 1 => Target::Scalar,
            "vrf": 2 => Target::Scalar,
            "token_file": 3 => Target::Scalar,
            "source_interface": 4 => Target::Scalar,
        };
        "AVDDesignCvSettingsOnpremClustersList" => indexed(Target::Model("AVDDesignCvSettingsOnpremClustersListIndexedItem"), [0]);
        "AVDDesignCvSettingsOnpremClustersItems" => dict {
            "name": 0 => Target::Scalar,
            "servers": 1 => Target::Model("AVDDesignCvSettingsOnpremClustersItemsServersList"),
            "vrf": 2 => Target::Scalar,
            "token_file": 3 => Target::Scalar,
            "source_interface": 4 => Target::Scalar,
        };
        "AVDDesignCvSettingsOnpremClustersItemsServersList" => indexed(Target::Model("AVDDesignCvSettingsOnpremClustersItemsServersListIndexedItem"), [0]);
        "AVDDesignCvSettingsOnpremClustersItemsServersItems" => dict {
            "name": 0 => Target::Scalar,
            "port": 1 => Target::Scalar,
        };
        "AVDDesignCvSettingsTerminattr" => dict {
            "ingestexclude": 0 => Target::Scalar,
            "smashexcludes": 1 => Target::Scalar,
            "disable_aaa": 2 => Target::Scalar,
            "cvtargetconfigs": 3 => Target::Model("AVDDesignCvSettingsTerminattrCvtargetconfigsList"),
            "flowdns": 4 => Target::Scalar,
            "custom_cv_options": 5 => Target::Model("AVDDesignCvSettingsTerminattrCustomCvOptionsList"),
        };
        "AVDDesignCvSettingsTerminattrCvtargetconfigsList" => list(Target::Scalar);
        "AVDDesignCvSettingsTerminattrCustomCvOptionsList" => list(Target::Model("AVDDesignCvSettingsTerminattrCustomCvOptionsItems"));
        "AVDDesignCvSettingsTerminattrCustomCvOptionsItems" => dict {
            "flag": 0 => Target::Scalar,
            "value": 1 => Target::Scalar,
        };
        "AVDDesignCvTopologyList" => indexed(Target::Model("AVDDesignCvTopologyListIndexedItem"), [0]);
        "AVDDesignCvTopologyItems" => dict {
            "hostname": 0 => Target::Scalar,
            "platform": 1 => Target::Scalar,
            "interfaces": 2 => Target::Model("AVDDesignCvTopologyItemsInterfacesList"),
        };
        "AVDDesignCvTopologyItemsInterfacesList" => indexed(Target::Model("AVDDesignCvTopologyItemsInterfacesListIndexedItem"), [0]);
        "AVDDesignCvTopologyItemsInterfacesItems" => dict {
            "name": 0 => Target::Scalar,
            "neighbor": 1 => Target::Scalar,
            "neighbor_interface": 2 => Target::Scalar,
        };
        "AVDDesignCvTopologyLevelsList" => indexed(Target::Model("AVDDesignCvTopologyLevelsListIndexedItem"), [0]);
        "AVDDesignCvTopologyLevelsItems" => dict {
            "field_type": 0 => Target::Scalar,
            "level": 1 => Target::Scalar,
        };
        "AVDDesignDefaultInterfacesList" => list(Target::Model("AVDDesignDefaultInterfacesItems"));
        "AVDDesignDefaultInterfacesItems" => dict {
            "types": 0 => Target::Model("AVDDesignDefaultInterfacesItemsTypesList"),
            "platforms": 1 => Target::Model("AVDDesignDefaultInterfacesItemsPlatformsList"),
            "uplink_interfaces": 2 => Target::Model("AVDDesignDefaultInterfacesItemsUplinkInterfacesList"),
            "mlag_interfaces": 3 => Target::Model("AVDDesignDefaultInterfacesItemsMlagInterfacesList"),
            "mlag_interfaces_speed": 4 => Target::Scalar,
            "downlink_interfaces": 5 => Target::Model("AVDDesignDefaultInterfacesItemsDownlinkInterfacesList"),
            "uplink_interface_speed": 6 => Target::Scalar,
        };
        "AVDDesignDefaultInterfacesItemsTypesList" => list(Target::Scalar);
        "AVDDesignDefaultInterfacesItemsPlatformsList" => list(Target::Scalar);
        "AVDDesignDefaultInterfacesItemsUplinkInterfacesList" => list(Target::Scalar);
        "AVDDesignDefaultInterfacesItemsMlagInterfacesList" => list(Target::Scalar);
        "AVDDesignDefaultInterfacesItemsDownlinkInterfacesList" => list(Target::Scalar);
        "AVDDesignDefaultNodeTypesList" => indexed(Target::Model("AVDDesignDefaultNodeTypesListIndexedItem"), [0]);
        "AVDDesignDefaultNodeTypesItems" => dict {
            "node_type": 0 => Target::Scalar,
            "match_hostnames": 1 => Target::Model("AVDDesignDefaultNodeTypesItemsMatchHostnamesList"),
        };
        "AVDDesignDefaultNodeTypesItemsMatchHostnamesList" => list(Target::Scalar);
        "AVDDesignDeviceProfilesList" => indexed(Target::Model("AVDDesignDeviceProfilesListIndexedItem"), [0]);
        "AVDDesignDeviceProfilesItems" => dict {
            "name": 0 => Target::Scalar,
            "parent_profile": 1 => Target::Scalar,
            "field_type": 2 => Target::Scalar,
            "mlag_group": 3 => Target::Scalar,
            "id": 4 => Target::Scalar,
            "platform": 5 => Target::Scalar,
            "mac_address": 6 => Target::Scalar,
            "system_mac_address": 7 => Target::Scalar,
            "custom_system_mac_address": 8 => Target::Scalar,
            "serial_number": 9 => Target::Scalar,
            "rack": 10 => Target::Scalar,
            "mgmt_ip": 11 => Target::Scalar,
            "mgmt_gateway": 12 => Target::Scalar,
            "ipv6_mgmt_ip": 13 => Target::Scalar,
            "ipv6_mgmt_gateway": 14 => Target::Scalar,
            "mgmt_interface": 15 => Target::Scalar,
            "link_tracking": 16 => Target::Model("AVDDesignDeviceProfilesItemsLinkTracking"),
            "lacp_port_id_range": 17 => Target::Model("AVDDesignDeviceProfilesItemsLacpPortIdRange"),
            "always_configure_ip_routing": 18 => Target::Scalar,
            "raw_eos_cli": 19 => Target::Scalar,
            "structured_config": 20 => Target::Opaque,
            "uplink_type": 21 => Target::Scalar,
            "uplink_ipv4_pool": 22 => Target::Scalar,
            "uplink_ipv6_pool": 23 => Target::Scalar,
            "uplink_interfaces": 24 => Target::Model("AVDDesignDeviceProfilesItemsUplinkInterfacesList"),
            "uplink_switch_interfaces": 25 => Target::Model("AVDDesignDeviceProfilesItemsUplinkSwitchInterfacesList"),
            "uplink_switches": 26 => Target::Model("AVDDesignDeviceProfilesItemsUplinkSwitchesList"),
            "uplink_interface_speed": 27 => Target::Scalar,
            "uplink_switch_interface_speed": 28 => Target::Scalar,
            "uplink_mtu": 29 => Target::Scalar,
            "max_uplink_switches": 30 => Target::Scalar,
            "max_parallel_uplinks": 31 => Target::Scalar,
            "uplink_bfd": 32 => Target::Scalar,
            "uplink_native_vlan": 33 => Target::Scalar,
            "uplink_ptp": 34 => Target::Model("AVDDesignDeviceProfilesItemsUplinkPtp"),
            "uplink_macsec": 35 => Target::Model("AVDDesignDeviceProfilesItemsUplinkMacsec"),
            "uplink_port_channel_id": 36 => Target::Scalar,
            "uplink_switch_port_channel_id": 37 => Target::Scalar,
            "uplink_ethernet_structured_config": 38 => Target::Opaque,
            "uplink_port_channel_structured_config": 39 => Target::Opaque,
            "uplink_switch_ethernet_structured_config": 40 => Target::Opaque,
            "uplink_switch_port_channel_structured_config": 41 => Target::Opaque,
            "mlag_port_channel_structured_config": 42 => Target::Opaque,
            "mlag_peer_vlan_structured_config": 43 => Target::Opaque,
            "mlag_peer_l3_vlan_structured_config": 44 => Target::Opaque,
            "short_esi": 45 => Target::Scalar,
            "isis_system_id_prefix": 46 => Target::Scalar,
            "isis_maximum_paths": 47 => Target::Scalar,
            "is_type": 48 => Target::Scalar,
            "node_sid_base": 49 => Target::Scalar,
            "isis_sr": 50 => Target::Model("AVDDesignDeviceProfilesItemsIsisSr"),
            "loopback_ipv4_pool": 51 => Target::Scalar,
            "loopback_ipv4_address": 52 => Target::Scalar,
            "vtep_loopback_ipv4_pool": 53 => Target::Scalar,
            "vtep_loopback_ipv6_pool": 54 => Target::Scalar,
            "vtep_loopback_ipv4_address": 55 => Target::Scalar,
            "vtep_loopback_ipv6_address": 56 => Target::Scalar,
            "loopback_ipv4_offset": 57 => Target::Scalar,
            "router_id_pool": 58 => Target::Scalar,
            "loopback_ipv6_pool": 59 => Target::Scalar,
            "loopback_ipv6_offset": 60 => Target::Scalar,
            "vtep": 61 => Target::Scalar,
            "vtep_loopback": 62 => Target::Scalar,
            "bgp_as": 63 => Target::Scalar,
            "bgp_defaults": 64 => Target::Model("AVDDesignDeviceProfilesItemsBgpDefaultsList"),
            "evpn_role": 65 => Target::Scalar,
            "evpn_route_servers": 66 => Target::Model("AVDDesignDeviceProfilesItemsEvpnRouteServersList"),
            "evpn_services_l2_only": 67 => Target::Scalar,
            "filter": 68 => Target::Model("AVDDesignDeviceProfilesItemsFilter"),
            "igmp_snooping_enabled": 69 => Target::Scalar,
            "evpn_gateway": 70 => Target::Model("AVDDesignDeviceProfilesItemsEvpnGateway"),
            "ipvpn_gateway": 71 => Target::Model("AVDDesignDeviceProfilesItemsIpvpnGateway"),
            "mlag": 72 => Target::Scalar,
            "mlag_dual_primary_detection": 73 => Target::Scalar,
            "mlag_ibgp_origin_incomplete": 74 => Target::Scalar,
            "mlag_interfaces": 75 => Target::Model("AVDDesignDeviceProfilesItemsMlagInterfacesList"),
            "mlag_interfaces_speed": 76 => Target::Scalar,
            "mlag_peer_l3_vlan": 77 => Target::Scalar,
            "mlag_peer_l3_ipv4_pool": 78 => Target::Scalar,
            "mlag_peer_l3_ipv6_pool": 79 => Target::Scalar,
            "mlag_peer_vlan": 80 => Target::Scalar,
            "mlag_peer_link_allowed_vlans": 81 => Target::Scalar,
            "mlag_peer_address_family": 82 => Target::Scalar,
            "mlag_peer_ipv4_pool": 83 => Target::Scalar,
            "mlag_peer_ipv6_pool": 84 => Target::Scalar,
            "mlag_port_channel_id": 85 => Target::Scalar,
            "mlag_domain_id": 86 => Target::Scalar,
            "spanning_tree_mode": 87 => Target::Scalar,
            "spanning_tree_priority": 88 => Target::Scalar,
            "spanning_tree_root_super": 89 => Target::Scalar,
            "spanning_tree_mst_pvst_boundary": 90 => Target::Scalar,
            "spanning_tree_port_id_allocation_port_channel_range": 91 => Target::Model("EosCliConfigGenSpanningTreePortIdAllocationPortChannelRange"),
            "virtual_router_mac_address": 92 => Target::Scalar,
            "inband_mgmt_interface": 93 => Target::Scalar,
            "inband_mgmt_vlan": 94 => Target::Scalar,
            "inband_mgmt_subnet": 95 => Target::Scalar,
            "inband_mgmt_subnet_offset": 96 => Target::Scalar,
            "inband_mgmt_ip": 97 => Target::Scalar,
            "inband_mgmt_gateway": 98 => Target::Scalar,
            "inband_mgmt_ipv6_address": 99 => Target::Scalar,
            "inband_mgmt_ipv6_subnet": 100 => Target::Scalar,
            "inband_mgmt_ipv6_gateway": 101 => Target::Scalar,
            "inband_mgmt_description": 102 => Target::Scalar,
            "inband_mgmt_vlan_name": 103 => Target::Scalar,
            "inband_mgmt_vrf": 104 => Target::Scalar,
            "inband_mgmt_mtu": 105 => Target::Scalar,
            "inband_ztp": 106 => Target::Scalar,
            "inband_ztp_lacp_fallback_delay": 107 => Target::Scalar,
            "mpls_overlay_role": 108 => Target::Scalar,
            "overlay_address_families": 109 => Target::Model("AVDDesignDeviceProfilesItemsOverlayAddressFamiliesList"),
            "mpls_route_reflectors": 110 => Target::Model("AVDDesignDeviceProfilesItemsMplsRouteReflectorsList"),
            "bgp_cluster_id": 111 => Target::Scalar,
            "kernel_ecmp_cli": 112 => Target::Scalar,
            "ptp": 113 => Target::Model("AVDDesignDeviceProfilesItemsPtp"),
            "wan_role": 114 => Target::Scalar,
            "cv_pathfinder_transit_mode": 115 => Target::Scalar,
            "cv_pathfinder_region": 116 => Target::Scalar,
            "cv_pathfinder_site": 117 => Target::Scalar,
            "wan_ha": 118 => Target::Model("AVDDesignDeviceProfilesItemsWanHa"),
            "dps_mss_ipv4": 119 => Target::Scalar,
            "l3_interfaces": 120 => Target::Model("AVDDesignDeviceProfilesItemsL3InterfacesList"),
            "l3_port_channels": 121 => Target::Model("AVDDesignDeviceProfilesItemsL3PortChannelsList"),
            "data_plane_cpu_allocation_max": 122 => Target::Scalar,
            "flow_tracker_type": 123 => Target::Scalar,
            "underlay_multicast": 124 => Target::Model("AVDDesignDeviceProfilesItemsUnderlayMulticast"),
            "campus": 125 => Target::Scalar,
            "campus_pod": 126 => Target::Scalar,
            "campus_access_pod": 127 => Target::Scalar,
            "cv_tags_topology_type": 128 => Target::Scalar,
            "digital_twin": 129 => Target::Model("AVDDesignDeviceProfilesItemsDigitalTwin"),
            "validation_profile": 130 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsLinkTracking" => dict {
            "enabled": 0 => Target::Scalar,
            "downlinks": 1 => Target::Model("AVDDesignDeviceProfilesItemsLinkTrackingDownlinks"),
            "groups": 2 => Target::Model("AVDDesignDeviceProfilesItemsLinkTrackingGroupsList"),
        };
        "AVDDesignDeviceProfilesItemsLinkTrackingDownlinks" => dict {
            "enabled": 0 => Target::Scalar,
            "group": 1 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsLinkTrackingGroupsList" => indexed(Target::Model("AVDDesignDeviceProfilesItemsLinkTrackingGroupsListIndexedItem"), [0]);
        "AVDDesignDeviceProfilesItemsLinkTrackingGroupsItems" => dict {
            "name": 0 => Target::Scalar,
            "recovery_delay": 1 => Target::Scalar,
            "links_minimum": 2 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsLacpPortIdRange" => dict {
            "enabled": 0 => Target::Scalar,
            "size": 1 => Target::Scalar,
            "offset": 2 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsUplinkInterfacesList" => list(Target::Scalar);
        "AVDDesignDeviceProfilesItemsUplinkSwitchInterfacesList" => list(Target::Scalar);
        "AVDDesignDeviceProfilesItemsUplinkSwitchesList" => list(Target::Scalar);
        "AVDDesignDeviceProfilesItemsUplinkPtp" => dict {
            "enable": 0 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsUplinkMacsec" => dict {
            "profile": 0 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsIsisSr" => dict {
            "ipv4_node_sid_index": 0 => Target::Scalar,
            "ipv4_node_sid_index_base": 1 => Target::Scalar,
            "ipv6_node_sid_index": 2 => Target::Scalar,
            "ipv6_node_sid_index_base": 3 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsBgpDefaultsList" => list(Target::Scalar);
        "AVDDesignDeviceProfilesItemsEvpnRouteServersList" => list(Target::Scalar);
        "AVDDesignDeviceProfilesItemsFilter" => dict {
            "tenants": 0 => Target::Model("AVDDesignDeviceProfilesItemsFilterTenantsList"),
            "tags": 1 => Target::Model("AVDDesignDeviceProfilesItemsFilterTagsList"),
            "allow_vrfs": 2 => Target::Model("AVDDesignDeviceProfilesItemsFilterAllowVrfsList"),
            "deny_vrfs": 3 => Target::Model("AVDDesignDeviceProfilesItemsFilterDenyVrfsList"),
            "always_include_vrfs_in_tenants": 4 => Target::Model("AVDDesignDeviceProfilesItemsFilterAlwaysIncludeVrfsInTenantsList"),
            "only_vlans_in_use": 5 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsFilterTenantsList" => list(Target::Scalar);
        "AVDDesignDeviceProfilesItemsFilterTagsList" => list(Target::Scalar);
        "AVDDesignDeviceProfilesItemsFilterAllowVrfsList" => list(Target::Scalar);
        "AVDDesignDeviceProfilesItemsFilterDenyVrfsList" => list(Target::Scalar);
        "AVDDesignDeviceProfilesItemsFilterAlwaysIncludeVrfsInTenantsList" => list(Target::Scalar);
        "AVDDesignDeviceProfilesItemsEvpnGateway" => dict {
            "remote_peers": 0 => Target::Model("AVDDesignDeviceProfilesItemsEvpnGatewayRemotePeersList"),
            "evpn_l2": 1 => Target::Model("AVDDesignDeviceProfilesItemsEvpnGatewayEvpnL2"),
            "evpn_l3": 2 => Target::Model("AVDDesignDeviceProfilesItemsEvpnGatewayEvpnL3"),
            "d_path": 3 => Target::Model("AVDDesignDeviceProfilesItemsEvpnGatewayDPath"),
            "all_active_multihoming": 4 => Target::Model("AVDDesignDeviceProfilesItemsEvpnGatewayAllActiveMultihoming"),
        };
        "AVDDesignDeviceProfilesItemsEvpnGatewayRemotePeersList" => indexed(Target::Model("AVDDesignDeviceProfilesItemsEvpnGatewayRemotePeersListIndexedItem"), [0]);
        "AVDDesignDeviceProfilesItemsEvpnGatewayRemotePeersItems" => dict {
            "hostname": 0 => Target::Scalar,
            "ip_address": 1 => Target::Scalar,
            "bgp_as": 2 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsEvpnGatewayEvpnL2" => dict {
            "enabled": 0 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsEvpnGatewayEvpnL3" => dict {
            "enabled": 0 => Target::Scalar,
            "inter_domain": 1 => Target::Scalar,
            "mode": 2 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsEvpnGatewayDPath" => dict {
            "enabled": 0 => Target::Scalar,
            "local_domain_id": 1 => Target::Scalar,
            "remote_domain_id": 2 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsEvpnGatewayAllActiveMultihoming" => dict {
            "enabled": 0 => Target::Scalar,
            "enable_d_path": 1 => Target::Scalar,
            "evpn_domain_id_local": 2 => Target::Scalar,
            "evpn_domain_id_remote": 3 => Target::Scalar,
            "evpn_ethernet_segment": 4 => Target::Model("AVDDesignDeviceProfilesItemsEvpnGatewayAllActiveMultihomingEvpnEthernetSegment"),
        };
        "AVDDesignDeviceProfilesItemsEvpnGatewayAllActiveMultihomingEvpnEthernetSegment" => dict {
            "identifier": 0 => Target::Scalar,
            "rt_import": 1 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsIpvpnGateway" => dict {
            "enabled": 0 => Target::Scalar,
            "evpn_domain_id": 1 => Target::Scalar,
            "ipvpn_domain_id": 2 => Target::Scalar,
            "enable_d_path": 3 => Target::Scalar,
            "maximum_routes": 4 => Target::Scalar,
            "local_as": 5 => Target::Scalar,
            "address_families": 6 => Target::Model("AVDDesignDeviceProfilesItemsIpvpnGatewayAddressFamiliesList"),
            "remote_peers": 7 => Target::Model("AVDDesignDeviceProfilesItemsIpvpnGatewayRemotePeersList"),
        };
        "AVDDesignDeviceProfilesItemsIpvpnGatewayAddressFamiliesList" => list(Target::Scalar);
        "AVDDesignDeviceProfilesItemsIpvpnGatewayRemotePeersList" => indexed(Target::Model("AVDDesignDeviceProfilesItemsIpvpnGatewayRemotePeersListIndexedItem"), [0]);
        "AVDDesignDeviceProfilesItemsIpvpnGatewayRemotePeersItems" => dict {
            "hostname": 0 => Target::Scalar,
            "ip_address": 1 => Target::Scalar,
            "bgp_as": 2 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsMlagInterfacesList" => list(Target::Scalar);
        "AVDDesignDeviceProfilesItemsOverlayAddressFamiliesList" => list(Target::Scalar);
        "AVDDesignDeviceProfilesItemsMplsRouteReflectorsList" => list(Target::Scalar);
        "AVDDesignDeviceProfilesItemsPtp" => dict {
            "enabled": 0 => Target::Scalar,
            "profile": 1 => Target::Scalar,
            "uplinks": 2 => Target::Model("AVDDesignDeviceProfilesItemsPtpUplinksList"),
            "mlag": 3 => Target::Scalar,
            "domain": 4 => Target::Scalar,
            "priority1": 5 => Target::Scalar,
            "priority2": 6 => Target::Scalar,
            "auto_clock_identity": 7 => Target::Scalar,
            "clock_identity_prefix": 8 => Target::Scalar,
            "clock_identity": 9 => Target::Scalar,
            "source_ip": 10 => Target::Scalar,
            "mode": 11 => Target::Scalar,
            "mode_one_step": 12 => Target::Scalar,
            "ttl": 13 => Target::Scalar,
            "forward_unicast": 14 => Target::Scalar,
            "forward_v1": 15 => Target::Scalar,
            "free_running": 16 => Target::Model("EosCliConfigGenPtpFreeRunning"),
            "dscp": 17 => Target::Model("AVDDesignDeviceProfilesItemsPtpDscp"),
            "monitor": 18 => Target::Model("AVDDesignDeviceProfilesItemsPtpMonitor"),
        };
        "AVDDesignDeviceProfilesItemsPtpUplinksList" => list(Target::Scalar);
        "AVDDesignDeviceProfilesItemsPtpDscp" => dict {
            "general_messages": 0 => Target::Scalar,
            "event_messages": 1 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsPtpMonitor" => dict {
            "enabled": 0 => Target::Scalar,
            "threshold": 1 => Target::Model("AVDDesignDeviceProfilesItemsPtpMonitorThreshold"),
            "missing_message": 2 => Target::Model("AVDDesignDeviceProfilesItemsPtpMonitorMissingMessage"),
        };
        "AVDDesignDeviceProfilesItemsPtpMonitorThreshold" => dict {
            "offset_from_master": 0 => Target::Scalar,
            "mean_path_delay": 1 => Target::Scalar,
            "drop": 2 => Target::Model("AVDDesignDeviceProfilesItemsPtpMonitorThresholdDrop"),
        };
        "AVDDesignDeviceProfilesItemsPtpMonitorThresholdDrop" => dict {
            "offset_from_master": 0 => Target::Scalar,
            "mean_path_delay": 1 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsPtpMonitorMissingMessage" => dict {
            "intervals": 0 => Target::Model("AVDDesignDeviceProfilesItemsPtpMonitorMissingMessageIntervals"),
            "sequence_ids": 1 => Target::Model("AVDDesignDeviceProfilesItemsPtpMonitorMissingMessageSequenceIds"),
        };
        "AVDDesignDeviceProfilesItemsPtpMonitorMissingMessageIntervals" => dict {
            "announce": 0 => Target::Scalar,
            "follow_up": 1 => Target::Scalar,
            "sync": 2 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsPtpMonitorMissingMessageSequenceIds" => dict {
            "enabled": 0 => Target::Scalar,
            "announce": 1 => Target::Scalar,
            "delay_resp": 2 => Target::Scalar,
            "follow_up": 3 => Target::Scalar,
            "sync": 4 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsWanHa" => dict {
            "enabled": 0 => Target::Scalar,
            "ipsec": 1 => Target::Scalar,
            "mtu": 2 => Target::Scalar,
            "ha_interfaces": 3 => Target::Model("AVDDesignDeviceProfilesItemsWanHaHaInterfacesList"),
            "ha_ipv4_pool": 4 => Target::Scalar,
            "port_channel_id": 5 => Target::Scalar,
            "use_port_channel_for_direct_ha": 6 => Target::Scalar,
            "flow_tracking": 7 => Target::Model("AVDDesignDeviceProfilesItemsWanHaFlowTracking"),
        };
        "AVDDesignDeviceProfilesItemsWanHaHaInterfacesList" => list(Target::Scalar);
        "AVDDesignDeviceProfilesItemsWanHaFlowTracking" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsL3InterfacesList" => indexed(Target::Model("AVDDesignDeviceProfilesItemsL3InterfacesListIndexedItem"), [1]);
        "AVDDesignDeviceProfilesItemsL3InterfacesItems" => dict {
            "profile": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
            "description": 2 => Target::Scalar,
            "ip_address": 3 => Target::Scalar,
            "ipv6_addresses": 4 => Target::Model("AVDDesignDeviceProfilesItemsL3InterfacesItemsIpv6AddressesList"),
            "dhcp_ip": 5 => Target::Scalar,
            "public_ip": 6 => Target::Scalar,
            "encapsulation_dot1q_vlan": 7 => Target::Scalar,
            "dhcp_accept_default_route": 8 => Target::Scalar,
            "enabled": 9 => Target::Scalar,
            "speed": 10 => Target::Scalar,
            "receive_bandwidth": 11 => Target::Scalar,
            "transmit_bandwidth": 12 => Target::Scalar,
            "peer": 13 => Target::Scalar,
            "peer_interface": 14 => Target::Scalar,
            "peer_ip": 15 => Target::Scalar,
            "peer_ipv6": 16 => Target::Scalar,
            "bgp": 17 => Target::Model("AVDDesignDeviceProfilesItemsL3InterfacesItemsBgp"),
            "ipv4_acl_in": 18 => Target::Scalar,
            "ipv4_acl_out": 19 => Target::Scalar,
            "ipv6_acl_in": 20 => Target::Scalar,
            "ipv6_acl_out": 21 => Target::Scalar,
            "static_routes": 22 => Target::Model("AVDDesignDeviceProfilesItemsL3InterfacesItemsStaticRoutesList"),
            "qos_profile": 23 => Target::Scalar,
            "wan_carrier": 24 => Target::Scalar,
            "wan_circuit_id": 25 => Target::Scalar,
            "connected_to_pathfinder": 26 => Target::Scalar,
            "cv_pathfinder_internet_exit": 27 => Target::Model("AVDDesignDeviceProfilesItemsL3InterfacesItemsCvPathfinderInternetExit"),
            "rx_queue": 28 => Target::Model("AVDDesignDeviceProfilesItemsL3InterfacesItemsRxQueue"),
            "raw_eos_cli": 29 => Target::Scalar,
            "flow_tracking": 30 => Target::Model("AVDDesignDeviceProfilesItemsL3InterfacesItemsFlowTracking"),
            "structured_config": 31 => Target::Opaque,
        };
        "AVDDesignDeviceProfilesItemsL3InterfacesItemsIpv6AddressesList" => list(Target::Scalar);
        "AVDDesignDeviceProfilesItemsL3InterfacesItemsBgp" => dict {
            "peer_as": 0 => Target::Scalar,
            "ipv4_prefix_list_in": 1 => Target::Scalar,
            "ipv4_prefix_list_out": 2 => Target::Scalar,
            "ipv6_prefix_list_in": 3 => Target::Scalar,
            "ipv6_prefix_list_out": 4 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsL3InterfacesItemsStaticRoutesList" => list(Target::Model("AVDDesignDeviceProfilesItemsL3InterfacesItemsStaticRoutesItems"));
        "AVDDesignDeviceProfilesItemsL3InterfacesItemsStaticRoutesItems" => dict {
            "prefix": 0 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsL3InterfacesItemsCvPathfinderInternetExit" => dict {
            "policies": 0 => Target::Model("AVDDesignDeviceProfilesItemsL3InterfacesItemsCvPathfinderInternetExitPoliciesList"),
        };
        "AVDDesignDeviceProfilesItemsL3InterfacesItemsCvPathfinderInternetExitPoliciesList" => indexed(Target::Model("AVDDesignDeviceProfilesItemsL3InterfacesItemsCvPathfinderInternetExitPoliciesListIndexedItem"), [0]);
        "AVDDesignDeviceProfilesItemsL3InterfacesItemsCvPathfinderInternetExitPoliciesItems" => dict {
            "name": 0 => Target::Scalar,
            "tunnel_interface_numbers": 1 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsL3InterfacesItemsRxQueue" => dict {
            "count": 0 => Target::Scalar,
            "workers": 1 => Target::Model("AVDDesignDeviceProfilesItemsL3InterfacesItemsRxQueueWorkersList"),
            "mode": 2 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsL3InterfacesItemsRxQueueWorkersList" => list(Target::Scalar);
        "AVDDesignDeviceProfilesItemsL3InterfacesItemsFlowTracking" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsL3PortChannelsList" => indexed(Target::Model("AVDDesignDeviceProfilesItemsL3PortChannelsListIndexedItem"), [0]);
        "AVDDesignDeviceProfilesItemsL3PortChannelsItems" => dict {
            "name": 0 => Target::Scalar,
            "description": 1 => Target::Scalar,
            "mode": 2 => Target::Scalar,
            "member_interfaces": 3 => Target::Model("AVDDesignDeviceProfilesItemsL3PortChannelsItemsMemberInterfacesList"),
            "ip_address": 4 => Target::Scalar,
            "ipv6_addresses": 5 => Target::Model("AVDDesignDeviceProfilesItemsL3PortChannelsItemsIpv6AddressesList"),
            "dhcp_ip": 6 => Target::Scalar,
            "public_ip": 7 => Target::Scalar,
            "encapsulation_dot1q_vlan": 8 => Target::Scalar,
            "dhcp_accept_default_route": 9 => Target::Scalar,
            "enabled": 10 => Target::Scalar,
            "peer": 11 => Target::Scalar,
            "peer_port_channel": 12 => Target::Scalar,
            "peer_ip": 13 => Target::Scalar,
            "peer_ipv6": 14 => Target::Scalar,
            "bgp": 15 => Target::Model("AVDDesignDeviceProfilesItemsL3PortChannelsItemsBgp"),
            "ipv4_acl_in": 16 => Target::Scalar,
            "ipv4_acl_out": 17 => Target::Scalar,
            "ipv6_acl_in": 18 => Target::Scalar,
            "ipv6_acl_out": 19 => Target::Scalar,
            "static_routes": 20 => Target::Model("AVDDesignDeviceProfilesItemsL3PortChannelsItemsStaticRoutesList"),
            "qos_profile": 21 => Target::Scalar,
            "wan_carrier": 22 => Target::Scalar,
            "wan_circuit_id": 23 => Target::Scalar,
            "connected_to_pathfinder": 24 => Target::Scalar,
            "raw_eos_cli": 25 => Target::Scalar,
            "flow_tracking": 26 => Target::Model("AVDDesignDeviceProfilesItemsL3PortChannelsItemsFlowTracking"),
            "structured_config": 27 => Target::Opaque,
        };
        "AVDDesignDeviceProfilesItemsL3PortChannelsItemsMemberInterfacesList" => indexed(Target::Model("AVDDesignDeviceProfilesItemsL3PortChannelsItemsMemberInterfacesListIndexedItem"), [0]);
        "AVDDesignDeviceProfilesItemsL3PortChannelsItemsMemberInterfacesItems" => dict {
            "name": 0 => Target::Scalar,
            "description": 1 => Target::Scalar,
            "peer": 2 => Target::Scalar,
            "peer_interface": 3 => Target::Scalar,
            "speed": 4 => Target::Scalar,
            "rx_queue": 5 => Target::Model("AVDDesignDeviceProfilesItemsL3PortChannelsItemsMemberInterfacesItemsRxQueue"),
            "structured_config": 6 => Target::Opaque,
        };
        "AVDDesignDeviceProfilesItemsL3PortChannelsItemsMemberInterfacesItemsRxQueue" => dict {
            "count": 0 => Target::Scalar,
            "workers": 1 => Target::Model("AVDDesignDeviceProfilesItemsL3PortChannelsItemsMemberInterfacesItemsRxQueueWorkersList"),
            "mode": 2 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsL3PortChannelsItemsMemberInterfacesItemsRxQueueWorkersList" => list(Target::Scalar);
        "AVDDesignDeviceProfilesItemsL3PortChannelsItemsIpv6AddressesList" => list(Target::Scalar);
        "AVDDesignDeviceProfilesItemsL3PortChannelsItemsBgp" => dict {
            "peer_as": 0 => Target::Scalar,
            "ipv4_prefix_list_in": 1 => Target::Scalar,
            "ipv4_prefix_list_out": 2 => Target::Scalar,
            "ipv6_prefix_list_in": 3 => Target::Scalar,
            "ipv6_prefix_list_out": 4 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsL3PortChannelsItemsStaticRoutesList" => indexed(Target::Model("AVDDesignDeviceProfilesItemsL3PortChannelsItemsStaticRoutesListIndexedItem"), [0]);
        "AVDDesignDeviceProfilesItemsL3PortChannelsItemsStaticRoutesItems" => dict {
            "prefix": 0 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsL3PortChannelsItemsFlowTracking" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsUnderlayMulticast" => dict {
            "pim_sm": 0 => Target::Model("AVDDesignDeviceProfilesItemsUnderlayMulticastPimSm"),
            "field_static": 1 => Target::Model("AVDDesignDeviceProfilesItemsUnderlayMulticastStatic"),
        };
        "AVDDesignDeviceProfilesItemsUnderlayMulticastPimSm" => dict {
            "enabled": 0 => Target::Scalar,
            "uplinks": 1 => Target::Scalar,
            "uplink_interfaces": 2 => Target::Model("AVDDesignDeviceProfilesItemsUnderlayMulticastPimSmUplinkInterfacesList"),
            "mlag": 3 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsUnderlayMulticastPimSmUplinkInterfacesList" => list(Target::Scalar);
        "AVDDesignDeviceProfilesItemsUnderlayMulticastStatic" => dict {
            "enabled": 0 => Target::Scalar,
            "uplinks": 1 => Target::Scalar,
            "uplink_interfaces": 2 => Target::Model("AVDDesignDeviceProfilesItemsUnderlayMulticastStaticUplinkInterfacesList"),
            "mlag": 3 => Target::Scalar,
        };
        "AVDDesignDeviceProfilesItemsUnderlayMulticastStaticUplinkInterfacesList" => list(Target::Scalar);
        "AVDDesignDeviceProfilesItemsDigitalTwin" => dict {
            "act_os_version": 0 => Target::Scalar,
            "mgmt_ip": 1 => Target::Scalar,
            "mgmt_gateway": 2 => Target::Scalar,
            "act_internet_access": 3 => Target::Scalar,
        };
        "AVDDesignDevicesList" => indexed(Target::Model("AVDDesignDevicesListIndexedItem"), [3]);
        "AVDDesignDevicesItems" => dict {
            "profile": 0 => Target::Scalar,
            "field_type": 1 => Target::Scalar,
            "mlag_group": 2 => Target::Scalar,
            "name": 3 => Target::Scalar,
            "downlink_pools": 4 => Target::Model("AVDDesignDevicesItemsDownlinkPoolsList"),
            "id": 5 => Target::Scalar,
            "platform": 6 => Target::Scalar,
            "mac_address": 7 => Target::Scalar,
            "system_mac_address": 8 => Target::Scalar,
            "custom_system_mac_address": 9 => Target::Scalar,
            "serial_number": 10 => Target::Scalar,
            "rack": 11 => Target::Scalar,
            "mgmt_ip": 12 => Target::Scalar,
            "mgmt_gateway": 13 => Target::Scalar,
            "ipv6_mgmt_ip": 14 => Target::Scalar,
            "ipv6_mgmt_gateway": 15 => Target::Scalar,
            "mgmt_interface": 16 => Target::Scalar,
            "link_tracking": 17 => Target::Model("AVDDesignDevicesItemsLinkTracking"),
            "lacp_port_id_range": 18 => Target::Model("AVDDesignDevicesItemsLacpPortIdRange"),
            "always_configure_ip_routing": 19 => Target::Scalar,
            "raw_eos_cli": 20 => Target::Scalar,
            "structured_config": 21 => Target::Opaque,
            "uplink_type": 22 => Target::Scalar,
            "uplink_ipv4_pool": 23 => Target::Scalar,
            "uplink_ipv6_pool": 24 => Target::Scalar,
            "uplink_interfaces": 25 => Target::Model("AVDDesignDevicesItemsUplinkInterfacesList"),
            "uplink_switch_interfaces": 26 => Target::Model("AVDDesignDevicesItemsUplinkSwitchInterfacesList"),
            "uplink_switches": 27 => Target::Model("AVDDesignDevicesItemsUplinkSwitchesList"),
            "uplink_interface_speed": 28 => Target::Scalar,
            "uplink_switch_interface_speed": 29 => Target::Scalar,
            "uplink_mtu": 30 => Target::Scalar,
            "max_uplink_switches": 31 => Target::Scalar,
            "max_parallel_uplinks": 32 => Target::Scalar,
            "uplink_bfd": 33 => Target::Scalar,
            "uplink_native_vlan": 34 => Target::Scalar,
            "uplink_ptp": 35 => Target::Model("AVDDesignDevicesItemsUplinkPtp"),
            "uplink_macsec": 36 => Target::Model("AVDDesignDevicesItemsUplinkMacsec"),
            "uplink_port_channel_id": 37 => Target::Scalar,
            "uplink_switch_port_channel_id": 38 => Target::Scalar,
            "uplink_ethernet_structured_config": 39 => Target::Opaque,
            "uplink_port_channel_structured_config": 40 => Target::Opaque,
            "uplink_switch_ethernet_structured_config": 41 => Target::Opaque,
            "uplink_switch_port_channel_structured_config": 42 => Target::Opaque,
            "mlag_port_channel_structured_config": 43 => Target::Opaque,
            "mlag_peer_vlan_structured_config": 44 => Target::Opaque,
            "mlag_peer_l3_vlan_structured_config": 45 => Target::Opaque,
            "short_esi": 46 => Target::Scalar,
            "isis_system_id_prefix": 47 => Target::Scalar,
            "isis_maximum_paths": 48 => Target::Scalar,
            "is_type": 49 => Target::Scalar,
            "node_sid_base": 50 => Target::Scalar,
            "isis_sr": 51 => Target::Model("AVDDesignDevicesItemsIsisSr"),
            "loopback_ipv4_pool": 52 => Target::Scalar,
            "loopback_ipv4_address": 53 => Target::Scalar,
            "vtep_loopback_ipv4_pool": 54 => Target::Scalar,
            "vtep_loopback_ipv6_pool": 55 => Target::Scalar,
            "vtep_loopback_ipv4_address": 56 => Target::Scalar,
            "vtep_loopback_ipv6_address": 57 => Target::Scalar,
            "loopback_ipv4_offset": 58 => Target::Scalar,
            "router_id_pool": 59 => Target::Scalar,
            "loopback_ipv6_pool": 60 => Target::Scalar,
            "loopback_ipv6_offset": 61 => Target::Scalar,
            "vtep": 62 => Target::Scalar,
            "vtep_loopback": 63 => Target::Scalar,
            "bgp_as": 64 => Target::Scalar,
            "bgp_defaults": 65 => Target::Model("AVDDesignDevicesItemsBgpDefaultsList"),
            "evpn_role": 66 => Target::Scalar,
            "evpn_route_servers": 67 => Target::Model("AVDDesignDevicesItemsEvpnRouteServersList"),
            "evpn_services_l2_only": 68 => Target::Scalar,
            "filter": 69 => Target::Model("AVDDesignDevicesItemsFilter"),
            "igmp_snooping_enabled": 70 => Target::Scalar,
            "evpn_gateway": 71 => Target::Model("AVDDesignDevicesItemsEvpnGateway"),
            "ipvpn_gateway": 72 => Target::Model("AVDDesignDevicesItemsIpvpnGateway"),
            "mlag": 73 => Target::Scalar,
            "mlag_dual_primary_detection": 74 => Target::Scalar,
            "mlag_ibgp_origin_incomplete": 75 => Target::Scalar,
            "mlag_interfaces": 76 => Target::Model("AVDDesignDevicesItemsMlagInterfacesList"),
            "mlag_interfaces_speed": 77 => Target::Scalar,
            "mlag_peer_l3_vlan": 78 => Target::Scalar,
            "mlag_peer_l3_ipv4_pool": 79 => Target::Scalar,
            "mlag_peer_l3_ipv6_pool": 80 => Target::Scalar,
            "mlag_peer_vlan": 81 => Target::Scalar,
            "mlag_peer_link_allowed_vlans": 82 => Target::Scalar,
            "mlag_peer_address_family": 83 => Target::Scalar,
            "mlag_peer_ipv4_pool": 84 => Target::Scalar,
            "mlag_peer_ipv6_pool": 85 => Target::Scalar,
            "mlag_port_channel_id": 86 => Target::Scalar,
            "mlag_domain_id": 87 => Target::Scalar,
            "spanning_tree_mode": 88 => Target::Scalar,
            "spanning_tree_priority": 89 => Target::Scalar,
            "spanning_tree_root_super": 90 => Target::Scalar,
            "spanning_tree_mst_pvst_boundary": 91 => Target::Scalar,
            "spanning_tree_port_id_allocation_port_channel_range": 92 => Target::Model("EosCliConfigGenSpanningTreePortIdAllocationPortChannelRange"),
            "virtual_router_mac_address": 93 => Target::Scalar,
            "inband_mgmt_interface": 94 => Target::Scalar,
            "inband_mgmt_vlan": 95 => Target::Scalar,
            "inband_mgmt_subnet": 96 => Target::Scalar,
            "inband_mgmt_subnet_offset": 97 => Target::Scalar,
            "inband_mgmt_ip": 98 => Target::Scalar,
            "inband_mgmt_gateway": 99 => Target::Scalar,
            "inband_mgmt_ipv6_address": 100 => Target::Scalar,
            "inband_mgmt_ipv6_subnet": 101 => Target::Scalar,
            "inband_mgmt_ipv6_gateway": 102 => Target::Scalar,
            "inband_mgmt_description": 103 => Target::Scalar,
            "inband_mgmt_vlan_name": 104 => Target::Scalar,
            "inband_mgmt_vrf": 105 => Target::Scalar,
            "inband_mgmt_mtu": 106 => Target::Scalar,
            "inband_ztp": 107 => Target::Scalar,
            "inband_ztp_lacp_fallback_delay": 108 => Target::Scalar,
            "mpls_overlay_role": 109 => Target::Scalar,
            "overlay_address_families": 110 => Target::Model("AVDDesignDevicesItemsOverlayAddressFamiliesList"),
            "mpls_route_reflectors": 111 => Target::Model("AVDDesignDevicesItemsMplsRouteReflectorsList"),
            "bgp_cluster_id": 112 => Target::Scalar,
            "kernel_ecmp_cli": 113 => Target::Scalar,
            "ptp": 114 => Target::Model("AVDDesignDevicesItemsPtp"),
            "wan_role": 115 => Target::Scalar,
            "cv_pathfinder_transit_mode": 116 => Target::Scalar,
            "cv_pathfinder_region": 117 => Target::Scalar,
            "cv_pathfinder_site": 118 => Target::Scalar,
            "wan_ha": 119 => Target::Model("AVDDesignDevicesItemsWanHa"),
            "dps_mss_ipv4": 120 => Target::Scalar,
            "l3_interfaces": 121 => Target::Model("AVDDesignDevicesItemsL3InterfacesList"),
            "l3_port_channels": 122 => Target::Model("AVDDesignDevicesItemsL3PortChannelsList"),
            "data_plane_cpu_allocation_max": 123 => Target::Scalar,
            "flow_tracker_type": 124 => Target::Scalar,
            "underlay_multicast": 125 => Target::Model("AVDDesignDevicesItemsUnderlayMulticast"),
            "campus": 126 => Target::Scalar,
            "campus_pod": 127 => Target::Scalar,
            "campus_access_pod": 128 => Target::Scalar,
            "cv_tags_topology_type": 129 => Target::Scalar,
            "digital_twin": 130 => Target::Model("AVDDesignDevicesItemsDigitalTwin"),
            "validation_profile": 131 => Target::Scalar,
        };
        "AVDDesignDevicesItemsDownlinkPoolsList" => list(Target::Model("AVDDesignDevicesItemsDownlinkPoolsItems"));
        "AVDDesignDevicesItemsDownlinkPoolsItems" => dict {
            "ipv4_pool": 0 => Target::Scalar,
            "ipv6_pool": 1 => Target::Scalar,
            "downlink_interfaces": 2 => Target::Model("AVDDesignDevicesItemsDownlinkPoolsItemsDownlinkInterfacesList"),
        };
        "AVDDesignDevicesItemsDownlinkPoolsItemsDownlinkInterfacesList" => list(Target::Scalar);
        "AVDDesignDevicesItemsLinkTracking" => dict {
            "enabled": 0 => Target::Scalar,
            "downlinks": 1 => Target::Model("AVDDesignDevicesItemsLinkTrackingDownlinks"),
            "groups": 2 => Target::Model("AVDDesignDevicesItemsLinkTrackingGroupsList"),
        };
        "AVDDesignDevicesItemsLinkTrackingDownlinks" => dict {
            "enabled": 0 => Target::Scalar,
            "group": 1 => Target::Scalar,
        };
        "AVDDesignDevicesItemsLinkTrackingGroupsList" => indexed(Target::Model("AVDDesignDevicesItemsLinkTrackingGroupsListIndexedItem"), [0]);
        "AVDDesignDevicesItemsLinkTrackingGroupsItems" => dict {
            "name": 0 => Target::Scalar,
            "recovery_delay": 1 => Target::Scalar,
            "links_minimum": 2 => Target::Scalar,
        };
        "AVDDesignDevicesItemsLacpPortIdRange" => dict {
            "enabled": 0 => Target::Scalar,
            "size": 1 => Target::Scalar,
            "offset": 2 => Target::Scalar,
        };
        "AVDDesignDevicesItemsUplinkInterfacesList" => list(Target::Scalar);
        "AVDDesignDevicesItemsUplinkSwitchInterfacesList" => list(Target::Scalar);
        "AVDDesignDevicesItemsUplinkSwitchesList" => list(Target::Scalar);
        "AVDDesignDevicesItemsUplinkPtp" => dict {
            "enable": 0 => Target::Scalar,
        };
        "AVDDesignDevicesItemsUplinkMacsec" => dict {
            "profile": 0 => Target::Scalar,
        };
        "AVDDesignDevicesItemsIsisSr" => dict {
            "ipv4_node_sid_index": 0 => Target::Scalar,
            "ipv4_node_sid_index_base": 1 => Target::Scalar,
            "ipv6_node_sid_index": 2 => Target::Scalar,
            "ipv6_node_sid_index_base": 3 => Target::Scalar,
        };
        "AVDDesignDevicesItemsBgpDefaultsList" => list(Target::Scalar);
        "AVDDesignDevicesItemsEvpnRouteServersList" => list(Target::Scalar);
        "AVDDesignDevicesItemsFilter" => dict {
            "tenants": 0 => Target::Model("AVDDesignDevicesItemsFilterTenantsList"),
            "tags": 1 => Target::Model("AVDDesignDevicesItemsFilterTagsList"),
            "allow_vrfs": 2 => Target::Model("AVDDesignDevicesItemsFilterAllowVrfsList"),
            "deny_vrfs": 3 => Target::Model("AVDDesignDevicesItemsFilterDenyVrfsList"),
            "always_include_vrfs_in_tenants": 4 => Target::Model("AVDDesignDevicesItemsFilterAlwaysIncludeVrfsInTenantsList"),
            "only_vlans_in_use": 5 => Target::Scalar,
        };
        "AVDDesignDevicesItemsFilterTenantsList" => list(Target::Scalar);
        "AVDDesignDevicesItemsFilterTagsList" => list(Target::Scalar);
        "AVDDesignDevicesItemsFilterAllowVrfsList" => list(Target::Scalar);
        "AVDDesignDevicesItemsFilterDenyVrfsList" => list(Target::Scalar);
        "AVDDesignDevicesItemsFilterAlwaysIncludeVrfsInTenantsList" => list(Target::Scalar);
        "AVDDesignDevicesItemsEvpnGateway" => dict {
            "remote_peers": 0 => Target::Model("AVDDesignDevicesItemsEvpnGatewayRemotePeersList"),
            "evpn_l2": 1 => Target::Model("AVDDesignDevicesItemsEvpnGatewayEvpnL2"),
            "evpn_l3": 2 => Target::Model("AVDDesignDevicesItemsEvpnGatewayEvpnL3"),
            "d_path": 3 => Target::Model("AVDDesignDevicesItemsEvpnGatewayDPath"),
            "all_active_multihoming": 4 => Target::Model("AVDDesignDevicesItemsEvpnGatewayAllActiveMultihoming"),
        };
        "AVDDesignDevicesItemsEvpnGatewayRemotePeersList" => indexed(Target::Model("AVDDesignDevicesItemsEvpnGatewayRemotePeersListIndexedItem"), [0]);
        "AVDDesignDevicesItemsEvpnGatewayRemotePeersItems" => dict {
            "hostname": 0 => Target::Scalar,
            "ip_address": 1 => Target::Scalar,
            "bgp_as": 2 => Target::Scalar,
        };
        "AVDDesignDevicesItemsEvpnGatewayEvpnL2" => dict {
            "enabled": 0 => Target::Scalar,
        };
        "AVDDesignDevicesItemsEvpnGatewayEvpnL3" => dict {
            "enabled": 0 => Target::Scalar,
            "inter_domain": 1 => Target::Scalar,
            "mode": 2 => Target::Scalar,
        };
        "AVDDesignDevicesItemsEvpnGatewayDPath" => dict {
            "enabled": 0 => Target::Scalar,
            "local_domain_id": 1 => Target::Scalar,
            "remote_domain_id": 2 => Target::Scalar,
        };
        "AVDDesignDevicesItemsEvpnGatewayAllActiveMultihoming" => dict {
            "enabled": 0 => Target::Scalar,
            "enable_d_path": 1 => Target::Scalar,
            "evpn_domain_id_local": 2 => Target::Scalar,
            "evpn_domain_id_remote": 3 => Target::Scalar,
            "evpn_ethernet_segment": 4 => Target::Model("AVDDesignDevicesItemsEvpnGatewayAllActiveMultihomingEvpnEthernetSegment"),
        };
        "AVDDesignDevicesItemsEvpnGatewayAllActiveMultihomingEvpnEthernetSegment" => dict {
            "identifier": 0 => Target::Scalar,
            "rt_import": 1 => Target::Scalar,
        };
        "AVDDesignDevicesItemsIpvpnGateway" => dict {
            "enabled": 0 => Target::Scalar,
            "evpn_domain_id": 1 => Target::Scalar,
            "ipvpn_domain_id": 2 => Target::Scalar,
            "enable_d_path": 3 => Target::Scalar,
            "maximum_routes": 4 => Target::Scalar,
            "local_as": 5 => Target::Scalar,
            "address_families": 6 => Target::Model("AVDDesignDevicesItemsIpvpnGatewayAddressFamiliesList"),
            "remote_peers": 7 => Target::Model("AVDDesignDevicesItemsIpvpnGatewayRemotePeersList"),
        };
        "AVDDesignDevicesItemsIpvpnGatewayAddressFamiliesList" => list(Target::Scalar);
        "AVDDesignDevicesItemsIpvpnGatewayRemotePeersList" => indexed(Target::Model("AVDDesignDevicesItemsIpvpnGatewayRemotePeersListIndexedItem"), [0]);
        "AVDDesignDevicesItemsIpvpnGatewayRemotePeersItems" => dict {
            "hostname": 0 => Target::Scalar,
            "ip_address": 1 => Target::Scalar,
            "bgp_as": 2 => Target::Scalar,
        };
        "AVDDesignDevicesItemsMlagInterfacesList" => list(Target::Scalar);
        "AVDDesignDevicesItemsOverlayAddressFamiliesList" => list(Target::Scalar);
        "AVDDesignDevicesItemsMplsRouteReflectorsList" => list(Target::Scalar);
        "AVDDesignDevicesItemsPtp" => dict {
            "enabled": 0 => Target::Scalar,
            "profile": 1 => Target::Scalar,
            "uplinks": 2 => Target::Model("AVDDesignDevicesItemsPtpUplinksList"),
            "mlag": 3 => Target::Scalar,
            "domain": 4 => Target::Scalar,
            "priority1": 5 => Target::Scalar,
            "priority2": 6 => Target::Scalar,
            "auto_clock_identity": 7 => Target::Scalar,
            "clock_identity_prefix": 8 => Target::Scalar,
            "clock_identity": 9 => Target::Scalar,
            "source_ip": 10 => Target::Scalar,
            "mode": 11 => Target::Scalar,
            "mode_one_step": 12 => Target::Scalar,
            "ttl": 13 => Target::Scalar,
            "forward_unicast": 14 => Target::Scalar,
            "forward_v1": 15 => Target::Scalar,
            "free_running": 16 => Target::Model("EosCliConfigGenPtpFreeRunning"),
            "dscp": 17 => Target::Model("AVDDesignDevicesItemsPtpDscp"),
            "monitor": 18 => Target::Model("AVDDesignDevicesItemsPtpMonitor"),
        };
        "AVDDesignDevicesItemsPtpUplinksList" => list(Target::Scalar);
        "AVDDesignDevicesItemsPtpDscp" => dict {
            "general_messages": 0 => Target::Scalar,
            "event_messages": 1 => Target::Scalar,
        };
        "AVDDesignDevicesItemsPtpMonitor" => dict {
            "enabled": 0 => Target::Scalar,
            "threshold": 1 => Target::Model("AVDDesignDevicesItemsPtpMonitorThreshold"),
            "missing_message": 2 => Target::Model("AVDDesignDevicesItemsPtpMonitorMissingMessage"),
        };
        "AVDDesignDevicesItemsPtpMonitorThreshold" => dict {
            "offset_from_master": 0 => Target::Scalar,
            "mean_path_delay": 1 => Target::Scalar,
            "drop": 2 => Target::Model("AVDDesignDevicesItemsPtpMonitorThresholdDrop"),
        };
        "AVDDesignDevicesItemsPtpMonitorThresholdDrop" => dict {
            "offset_from_master": 0 => Target::Scalar,
            "mean_path_delay": 1 => Target::Scalar,
        };
        "AVDDesignDevicesItemsPtpMonitorMissingMessage" => dict {
            "intervals": 0 => Target::Model("AVDDesignDevicesItemsPtpMonitorMissingMessageIntervals"),
            "sequence_ids": 1 => Target::Model("AVDDesignDevicesItemsPtpMonitorMissingMessageSequenceIds"),
        };
        "AVDDesignDevicesItemsPtpMonitorMissingMessageIntervals" => dict {
            "announce": 0 => Target::Scalar,
            "follow_up": 1 => Target::Scalar,
            "sync": 2 => Target::Scalar,
        };
        "AVDDesignDevicesItemsPtpMonitorMissingMessageSequenceIds" => dict {
            "enabled": 0 => Target::Scalar,
            "announce": 1 => Target::Scalar,
            "delay_resp": 2 => Target::Scalar,
            "follow_up": 3 => Target::Scalar,
            "sync": 4 => Target::Scalar,
        };
        "AVDDesignDevicesItemsWanHa" => dict {
            "enabled": 0 => Target::Scalar,
            "ipsec": 1 => Target::Scalar,
            "mtu": 2 => Target::Scalar,
            "ha_interfaces": 3 => Target::Model("AVDDesignDevicesItemsWanHaHaInterfacesList"),
            "ha_ipv4_pool": 4 => Target::Scalar,
            "port_channel_id": 5 => Target::Scalar,
            "use_port_channel_for_direct_ha": 6 => Target::Scalar,
            "flow_tracking": 7 => Target::Model("AVDDesignDevicesItemsWanHaFlowTracking"),
        };
        "AVDDesignDevicesItemsWanHaHaInterfacesList" => list(Target::Scalar);
        "AVDDesignDevicesItemsWanHaFlowTracking" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignDevicesItemsL3InterfacesList" => indexed(Target::Model("AVDDesignDevicesItemsL3InterfacesListIndexedItem"), [1]);
        "AVDDesignDevicesItemsL3InterfacesItems" => dict {
            "profile": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
            "description": 2 => Target::Scalar,
            "ip_address": 3 => Target::Scalar,
            "ipv6_addresses": 4 => Target::Model("AVDDesignDevicesItemsL3InterfacesItemsIpv6AddressesList"),
            "dhcp_ip": 5 => Target::Scalar,
            "public_ip": 6 => Target::Scalar,
            "encapsulation_dot1q_vlan": 7 => Target::Scalar,
            "dhcp_accept_default_route": 8 => Target::Scalar,
            "enabled": 9 => Target::Scalar,
            "speed": 10 => Target::Scalar,
            "receive_bandwidth": 11 => Target::Scalar,
            "transmit_bandwidth": 12 => Target::Scalar,
            "peer": 13 => Target::Scalar,
            "peer_interface": 14 => Target::Scalar,
            "peer_ip": 15 => Target::Scalar,
            "peer_ipv6": 16 => Target::Scalar,
            "bgp": 17 => Target::Model("AVDDesignDevicesItemsL3InterfacesItemsBgp"),
            "ipv4_acl_in": 18 => Target::Scalar,
            "ipv4_acl_out": 19 => Target::Scalar,
            "ipv6_acl_in": 20 => Target::Scalar,
            "ipv6_acl_out": 21 => Target::Scalar,
            "static_routes": 22 => Target::Model("AVDDesignDevicesItemsL3InterfacesItemsStaticRoutesList"),
            "qos_profile": 23 => Target::Scalar,
            "wan_carrier": 24 => Target::Scalar,
            "wan_circuit_id": 25 => Target::Scalar,
            "connected_to_pathfinder": 26 => Target::Scalar,
            "cv_pathfinder_internet_exit": 27 => Target::Model("AVDDesignDevicesItemsL3InterfacesItemsCvPathfinderInternetExit"),
            "rx_queue": 28 => Target::Model("AVDDesignDevicesItemsL3InterfacesItemsRxQueue"),
            "raw_eos_cli": 29 => Target::Scalar,
            "flow_tracking": 30 => Target::Model("AVDDesignDevicesItemsL3InterfacesItemsFlowTracking"),
            "structured_config": 31 => Target::Opaque,
        };
        "AVDDesignDevicesItemsL3InterfacesItemsIpv6AddressesList" => list(Target::Scalar);
        "AVDDesignDevicesItemsL3InterfacesItemsBgp" => dict {
            "peer_as": 0 => Target::Scalar,
            "ipv4_prefix_list_in": 1 => Target::Scalar,
            "ipv4_prefix_list_out": 2 => Target::Scalar,
            "ipv6_prefix_list_in": 3 => Target::Scalar,
            "ipv6_prefix_list_out": 4 => Target::Scalar,
        };
        "AVDDesignDevicesItemsL3InterfacesItemsStaticRoutesList" => list(Target::Model("AVDDesignDevicesItemsL3InterfacesItemsStaticRoutesItems"));
        "AVDDesignDevicesItemsL3InterfacesItemsStaticRoutesItems" => dict {
            "prefix": 0 => Target::Scalar,
        };
        "AVDDesignDevicesItemsL3InterfacesItemsCvPathfinderInternetExit" => dict {
            "policies": 0 => Target::Model("AVDDesignDevicesItemsL3InterfacesItemsCvPathfinderInternetExitPoliciesList"),
        };
        "AVDDesignDevicesItemsL3InterfacesItemsCvPathfinderInternetExitPoliciesList" => indexed(Target::Model("AVDDesignDevicesItemsL3InterfacesItemsCvPathfinderInternetExitPoliciesListIndexedItem"), [0]);
        "AVDDesignDevicesItemsL3InterfacesItemsCvPathfinderInternetExitPoliciesItems" => dict {
            "name": 0 => Target::Scalar,
            "tunnel_interface_numbers": 1 => Target::Scalar,
        };
        "AVDDesignDevicesItemsL3InterfacesItemsRxQueue" => dict {
            "count": 0 => Target::Scalar,
            "workers": 1 => Target::Model("AVDDesignDevicesItemsL3InterfacesItemsRxQueueWorkersList"),
            "mode": 2 => Target::Scalar,
        };
        "AVDDesignDevicesItemsL3InterfacesItemsRxQueueWorkersList" => list(Target::Scalar);
        "AVDDesignDevicesItemsL3InterfacesItemsFlowTracking" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignDevicesItemsL3PortChannelsList" => indexed(Target::Model("AVDDesignDevicesItemsL3PortChannelsListIndexedItem"), [0]);
        "AVDDesignDevicesItemsL3PortChannelsItems" => dict {
            "name": 0 => Target::Scalar,
            "description": 1 => Target::Scalar,
            "mode": 2 => Target::Scalar,
            "member_interfaces": 3 => Target::Model("AVDDesignDevicesItemsL3PortChannelsItemsMemberInterfacesList"),
            "ip_address": 4 => Target::Scalar,
            "ipv6_addresses": 5 => Target::Model("AVDDesignDevicesItemsL3PortChannelsItemsIpv6AddressesList"),
            "dhcp_ip": 6 => Target::Scalar,
            "public_ip": 7 => Target::Scalar,
            "encapsulation_dot1q_vlan": 8 => Target::Scalar,
            "dhcp_accept_default_route": 9 => Target::Scalar,
            "enabled": 10 => Target::Scalar,
            "peer": 11 => Target::Scalar,
            "peer_port_channel": 12 => Target::Scalar,
            "peer_ip": 13 => Target::Scalar,
            "peer_ipv6": 14 => Target::Scalar,
            "bgp": 15 => Target::Model("AVDDesignDevicesItemsL3PortChannelsItemsBgp"),
            "ipv4_acl_in": 16 => Target::Scalar,
            "ipv4_acl_out": 17 => Target::Scalar,
            "ipv6_acl_in": 18 => Target::Scalar,
            "ipv6_acl_out": 19 => Target::Scalar,
            "static_routes": 20 => Target::Model("AVDDesignDevicesItemsL3PortChannelsItemsStaticRoutesList"),
            "qos_profile": 21 => Target::Scalar,
            "wan_carrier": 22 => Target::Scalar,
            "wan_circuit_id": 23 => Target::Scalar,
            "connected_to_pathfinder": 24 => Target::Scalar,
            "raw_eos_cli": 25 => Target::Scalar,
            "flow_tracking": 26 => Target::Model("AVDDesignDevicesItemsL3PortChannelsItemsFlowTracking"),
            "structured_config": 27 => Target::Opaque,
        };
        "AVDDesignDevicesItemsL3PortChannelsItemsMemberInterfacesList" => indexed(Target::Model("AVDDesignDevicesItemsL3PortChannelsItemsMemberInterfacesListIndexedItem"), [0]);
        "AVDDesignDevicesItemsL3PortChannelsItemsMemberInterfacesItems" => dict {
            "name": 0 => Target::Scalar,
            "description": 1 => Target::Scalar,
            "peer": 2 => Target::Scalar,
            "peer_interface": 3 => Target::Scalar,
            "speed": 4 => Target::Scalar,
            "rx_queue": 5 => Target::Model("AVDDesignDevicesItemsL3PortChannelsItemsMemberInterfacesItemsRxQueue"),
            "structured_config": 6 => Target::Opaque,
        };
        "AVDDesignDevicesItemsL3PortChannelsItemsMemberInterfacesItemsRxQueue" => dict {
            "count": 0 => Target::Scalar,
            "workers": 1 => Target::Model("AVDDesignDevicesItemsL3PortChannelsItemsMemberInterfacesItemsRxQueueWorkersList"),
            "mode": 2 => Target::Scalar,
        };
        "AVDDesignDevicesItemsL3PortChannelsItemsMemberInterfacesItemsRxQueueWorkersList" => list(Target::Scalar);
        "AVDDesignDevicesItemsL3PortChannelsItemsIpv6AddressesList" => list(Target::Scalar);
        "AVDDesignDevicesItemsL3PortChannelsItemsBgp" => dict {
            "peer_as": 0 => Target::Scalar,
            "ipv4_prefix_list_in": 1 => Target::Scalar,
            "ipv4_prefix_list_out": 2 => Target::Scalar,
            "ipv6_prefix_list_in": 3 => Target::Scalar,
            "ipv6_prefix_list_out": 4 => Target::Scalar,
        };
        "AVDDesignDevicesItemsL3PortChannelsItemsStaticRoutesList" => indexed(Target::Model("AVDDesignDevicesItemsL3PortChannelsItemsStaticRoutesListIndexedItem"), [0]);
        "AVDDesignDevicesItemsL3PortChannelsItemsStaticRoutesItems" => dict {
            "prefix": 0 => Target::Scalar,
        };
        "AVDDesignDevicesItemsL3PortChannelsItemsFlowTracking" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignDevicesItemsUnderlayMulticast" => dict {
            "pim_sm": 0 => Target::Model("AVDDesignDevicesItemsUnderlayMulticastPimSm"),
            "field_static": 1 => Target::Model("AVDDesignDevicesItemsUnderlayMulticastStatic"),
        };
        "AVDDesignDevicesItemsUnderlayMulticastPimSm" => dict {
            "enabled": 0 => Target::Scalar,
            "uplinks": 1 => Target::Scalar,
            "uplink_interfaces": 2 => Target::Model("AVDDesignDevicesItemsUnderlayMulticastPimSmUplinkInterfacesList"),
            "mlag": 3 => Target::Scalar,
        };
        "AVDDesignDevicesItemsUnderlayMulticastPimSmUplinkInterfacesList" => list(Target::Scalar);
        "AVDDesignDevicesItemsUnderlayMulticastStatic" => dict {
            "enabled": 0 => Target::Scalar,
            "uplinks": 1 => Target::Scalar,
            "uplink_interfaces": 2 => Target::Model("AVDDesignDevicesItemsUnderlayMulticastStaticUplinkInterfacesList"),
            "mlag": 3 => Target::Scalar,
        };
        "AVDDesignDevicesItemsUnderlayMulticastStaticUplinkInterfacesList" => list(Target::Scalar);
        "AVDDesignDevicesItemsDigitalTwin" => dict {
            "act_os_version": 0 => Target::Scalar,
            "mgmt_ip": 1 => Target::Scalar,
            "mgmt_gateway": 2 => Target::Scalar,
            "act_internet_access": 3 => Target::Scalar,
        };
        "AVDDesignDigitalTwin" => dict {
            "environment": 0 => Target::Scalar,
            "fabric": 1 => Target::Model("AVDDesignDigitalTwinFabric"),
            "use_default_interfaces_of_digital_twin_platform": 2 => Target::Scalar,
        };
        "AVDDesignDigitalTwinFabric" => dict {
            "act_os_version": 0 => Target::Scalar,
            "act_username": 1 => Target::Scalar,
            "act_password": 2 => Target::Scalar,
            "act_internet_access": 3 => Target::Scalar,
            "act_ensure_eapi_access": 4 => Target::Scalar,
        };
        "AVDDesignDnsSettings" => dict {
            "domain": 0 => Target::Scalar,
            "domain_list": 1 => Target::Model("AVDDesignDnsSettingsDomainList"),
            "servers": 2 => Target::Model("AVDDesignDnsSettingsServersList"),
            "vrfs": 3 => Target::Model("AVDDesignDnsSettingsVrfsList"),
            "set_source_interfaces": 4 => Target::Scalar,
            "ip_hosts": 5 => Target::Model("EosCliConfigGenIpHostsList"),
        };
        "AVDDesignDnsSettingsDomainList" => list(Target::Scalar);
        "AVDDesignDnsSettingsServersList" => list(Target::Model("AVDDesignDnsSettingsServersItems"));
        "AVDDesignDnsSettingsServersItems" => dict {
            "vrf": 0 => Target::Scalar,
            "ip_address": 1 => Target::Scalar,
            "priority": 2 => Target::Scalar,
        };
        "AVDDesignDnsSettingsVrfsList" => indexed(Target::Model("AVDDesignDnsSettingsVrfsListIndexedItem"), [0]);
        "AVDDesignDnsSettingsVrfsItems" => dict {
            "name": 0 => Target::Scalar,
            "source_interface": 1 => Target::Scalar,
        };
        "AVDDesignDot1xSettings" => dict {
            "enabled": 0 => Target::Scalar,
            "authentication": 1 => Target::Model("AVDDesignDot1xSettingsAuthentication"),
            "accounting": 2 => Target::Model("AVDDesignDot1xSettingsAccounting"),
            "bypass_bpdu": 3 => Target::Scalar,
            "bypass_lldp": 4 => Target::Scalar,
            "dynamic_authorization": 5 => Target::Model("AVDDesignDot1xSettingsDynamicAuthorization"),
            "mac_based_authentication": 6 => Target::Model("AVDDesignDot1xSettingsMacBasedAuthentication"),
            "radius_av_pairs": 7 => Target::Model("AVDDesignDot1xSettingsRadiusAvPairs"),
            "device_profiling": 8 => Target::Model("AVDDesignDot1xSettingsDeviceProfiling"),
            "redistribute_in_evpn": 9 => Target::Scalar,
            "web_authentication": 10 => Target::Model("AVDDesignDot1xSettingsWebAuthentication"),
        };
        "AVDDesignDot1xSettingsAuthentication" => dict {
            "radius_groups": 0 => Target::Model("AVDDesignDot1xSettingsAuthenticationRadiusGroupsList"),
        };
        "AVDDesignDot1xSettingsAuthenticationRadiusGroupsList" => list(Target::Scalar);
        "AVDDesignDot1xSettingsAccounting" => dict {
            "enabled": 0 => Target::Scalar,
            "mode": 1 => Target::Scalar,
            "radius_groups": 2 => Target::Model("AVDDesignDot1xSettingsAccountingRadiusGroupsList"),
            "multicast": 3 => Target::Scalar,
            "syslog": 4 => Target::Scalar,
        };
        "AVDDesignDot1xSettingsAccountingRadiusGroupsList" => list(Target::Scalar);
        "AVDDesignDot1xSettingsDynamicAuthorization" => dict {
            "enabled": 0 => Target::Scalar,
            "additional_groups": 1 => Target::Model("AVDDesignDot1xSettingsDynamicAuthorizationAdditionalGroupsList"),
        };
        "AVDDesignDot1xSettingsDynamicAuthorizationAdditionalGroupsList" => list(Target::Scalar);
        "AVDDesignDot1xSettingsMacBasedAuthentication" => dict {
            "username_format": 0 => Target::Model("AVDDesignDot1xSettingsMacBasedAuthenticationUsernameFormat"),
            "delay": 1 => Target::Scalar,
            "hold_period": 2 => Target::Scalar,
        };
        "AVDDesignDot1xSettingsMacBasedAuthenticationUsernameFormat" => dict {
            "delimiter": 0 => Target::Scalar,
            "letter_case": 1 => Target::Scalar,
        };
        "AVDDesignDot1xSettingsRadiusAvPairs" => dict {
            "service_type": 0 => Target::Scalar,
            "framed_mtu": 1 => Target::Scalar,
        };
        "AVDDesignDot1xSettingsDeviceProfiling" => dict {
            "enabled": 0 => Target::Scalar,
            "dhcp": 1 => Target::Model("AVDDesignDot1xSettingsDeviceProfilingDhcp"),
            "lldp": 2 => Target::Model("AVDDesignDot1xSettingsDeviceProfilingLldp"),
        };
        "AVDDesignDot1xSettingsDeviceProfilingDhcp" => dict {
            "enabled": 0 => Target::Scalar,
            "hostname": 1 => Target::Model("AVDDesignDot1xSettingsDeviceProfilingDhcpHostname"),
            "parameter_request_list": 2 => Target::Model("AVDDesignDot1xSettingsDeviceProfilingDhcpParameterRequestList"),
            "vendor_class_id": 3 => Target::Model("AVDDesignDot1xSettingsDeviceProfilingDhcpVendorClassId"),
        };
        "AVDDesignDot1xSettingsDeviceProfilingDhcpHostname" => dict {
            "enabled": 0 => Target::Scalar,
            "auth_only": 1 => Target::Scalar,
        };
        "AVDDesignDot1xSettingsDeviceProfilingDhcpParameterRequestList" => dict {
            "enabled": 0 => Target::Scalar,
            "auth_only": 1 => Target::Scalar,
        };
        "AVDDesignDot1xSettingsDeviceProfilingDhcpVendorClassId" => dict {
            "enabled": 0 => Target::Scalar,
            "auth_only": 1 => Target::Scalar,
        };
        "AVDDesignDot1xSettingsDeviceProfilingLldp" => dict {
            "enabled": 0 => Target::Scalar,
            "system_name": 1 => Target::Model("AVDDesignDot1xSettingsDeviceProfilingLldpSystemName"),
            "system_description": 2 => Target::Model("AVDDesignDot1xSettingsDeviceProfilingLldpSystemDescription"),
        };
        "AVDDesignDot1xSettingsDeviceProfilingLldpSystemName" => dict {
            "enabled": 0 => Target::Scalar,
            "auth_only": 1 => Target::Scalar,
        };
        "AVDDesignDot1xSettingsDeviceProfilingLldpSystemDescription" => dict {
            "enabled": 0 => Target::Scalar,
            "auth_only": 1 => Target::Scalar,
        };
        "AVDDesignDot1xSettingsWebAuthentication" => dict {
            "enabled": 0 => Target::Scalar,
            "ipv4_acl": 1 => Target::Scalar,
            "url": 2 => Target::Scalar,
            "ssl_profile": 3 => Target::Scalar,
            "start_limit_infinite": 4 => Target::Scalar,
        };
        "AVDDesignEosDesignsCustomTemplatesList" => list(Target::Model("AVDDesignEosDesignsCustomTemplatesItems"));
        "AVDDesignEosDesignsCustomTemplatesItems" => dict {
            "template": 0 => Target::Scalar,
            "options": 1 => Target::Model("AVDDesignEosDesignsCustomTemplatesItemsOptions"),
        };
        "AVDDesignEosDesignsCustomTemplatesItemsOptions" => dict {
            "list_merge": 0 => Target::Scalar,
            "strip_empty_keys": 1 => Target::Scalar,
        };
        "AVDDesignEosDesignsDocumentation" => dict {
            "enable": 0 => Target::Scalar,
            "connected_endpoints": 1 => Target::Scalar,
            "topology_csv": 2 => Target::Scalar,
            "p2p_links_csv": 3 => Target::Scalar,
            "toc": 4 => Target::Scalar,
        };
        "AVDDesignErrdisableSettings" => dict {
            "recovery_interval": 0 => Target::Scalar,
            "causes": 1 => Target::Model("AVDDesignErrdisableSettingsCauses"),
        };
        "AVDDesignErrdisableSettingsCauses" => dict {
            "acl": 0 => Target::Model("AVDDesignErrdisableSettingsCausesAcl"),
            "arp_inspection": 1 => Target::Model("AVDDesignErrdisableSettingsCausesArpInspection"),
            "bpduguard": 2 => Target::Model("AVDDesignErrdisableSettingsCausesBpduguard"),
            "dot1x": 3 => Target::Model("AVDDesignErrdisableSettingsCausesDot1x"),
            "dot1x_coa": 4 => Target::Model("AVDDesignErrdisableSettingsCausesDot1xCoa"),
            "dot1x_phone_classification": 5 => Target::Model("AVDDesignErrdisableSettingsCausesDot1xPhoneClassification"),
            "dot1x_session_replace": 6 => Target::Model("AVDDesignErrdisableSettingsCausesDot1xSessionReplace"),
            "error_correction_encoding": 7 => Target::Model("AVDDesignErrdisableSettingsCausesErrorCorrectionEncoding"),
            "fabric_capacity_low": 8 => Target::Model("AVDDesignErrdisableSettingsCausesFabricCapacityLow"),
            "hardware_speed_group": 9 => Target::Model("AVDDesignErrdisableSettingsCausesHardwareSpeedGroup"),
            "hitless_reload_down": 10 => Target::Model("AVDDesignErrdisableSettingsCausesHitlessReloadDown"),
            "interface_speed": 11 => Target::Model("AVDDesignErrdisableSettingsCausesInterfaceSpeed"),
            "internal_error": 12 => Target::Model("AVDDesignErrdisableSettingsCausesInternalError"),
            "lacp_rate_limit": 13 => Target::Model("AVDDesignErrdisableSettingsCausesLacpRateLimit"),
            "link_change": 14 => Target::Model("AVDDesignErrdisableSettingsCausesLinkChange"),
            "link_flap": 15 => Target::Model("AVDDesignErrdisableSettingsCausesLinkFlap"),
            "no_internal_vlan": 16 => Target::Model("AVDDesignErrdisableSettingsCausesNoInternalVlan"),
            "port_breakout": 17 => Target::Model("AVDDesignErrdisableSettingsCausesPortBreakout"),
            "portchannelguard": 18 => Target::Model("AVDDesignErrdisableSettingsCausesPortchannelguard"),
            "portsec": 19 => Target::Model("AVDDesignErrdisableSettingsCausesPortsec"),
            "speed_misconfigured": 20 => Target::Model("AVDDesignErrdisableSettingsCausesSpeedMisconfigured"),
            "storm_control": 21 => Target::Model("AVDDesignErrdisableSettingsCausesStormControl"),
            "stuck_queue": 22 => Target::Model("AVDDesignErrdisableSettingsCausesStuckQueue"),
            "switchcard_unreachable": 23 => Target::Model("AVDDesignErrdisableSettingsCausesSwitchcardUnreachable"),
            "tap_port_init": 24 => Target::Model("AVDDesignErrdisableSettingsCausesTapPortInit"),
            "tapagg": 25 => Target::Model("AVDDesignErrdisableSettingsCausesTapagg"),
            "tpid": 26 => Target::Model("AVDDesignErrdisableSettingsCausesTpid"),
            "transceiver_adapter": 27 => Target::Model("AVDDesignErrdisableSettingsCausesTransceiverAdapter"),
            "uplink_failure_detection": 28 => Target::Model("AVDDesignErrdisableSettingsCausesUplinkFailureDetection"),
            "xcvr_misconfigured": 29 => Target::Model("AVDDesignErrdisableSettingsCausesXcvrMisconfigured"),
            "xcvr_overheat": 30 => Target::Model("AVDDesignErrdisableSettingsCausesXcvrOverheat"),
            "xcvr_power_unsupported": 31 => Target::Model("AVDDesignErrdisableSettingsCausesXcvrPowerUnsupported"),
            "xcvr_unsupported": 32 => Target::Model("AVDDesignErrdisableSettingsCausesXcvrUnsupported"),
        };
        "AVDDesignErrdisableSettingsCausesAcl" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
            "recovery_interval": 2 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesArpInspection" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
            "recovery_interval": 2 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesBpduguard" => dict {
            "recovery": 0 => Target::Scalar,
            "recovery_interval": 1 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesDot1x" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
            "recovery_interval": 2 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesDot1xCoa" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
            "recovery_interval": 2 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesDot1xPhoneClassification" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
            "recovery_interval": 2 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesDot1xSessionReplace" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
            "recovery_interval": 2 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesErrorCorrectionEncoding" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
            "recovery_interval": 2 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesFabricCapacityLow" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
            "recovery_interval": 2 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesHardwareSpeedGroup" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
            "recovery_interval": 2 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesHitlessReloadDown" => dict {
            "recovery": 0 => Target::Scalar,
            "recovery_interval": 1 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesInterfaceSpeed" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
            "recovery_interval": 2 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesInternalError" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
            "recovery_interval": 2 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesLacpRateLimit" => dict {
            "recovery": 0 => Target::Scalar,
            "recovery_interval": 1 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesLinkChange" => dict {
            "detection": 0 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesLinkFlap" => dict {
            "recovery": 0 => Target::Scalar,
            "recovery_interval": 1 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesNoInternalVlan" => dict {
            "recovery": 0 => Target::Scalar,
            "recovery_interval": 1 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesPortBreakout" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
            "recovery_interval": 2 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesPortchannelguard" => dict {
            "recovery": 0 => Target::Scalar,
            "recovery_interval": 1 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesPortsec" => dict {
            "recovery": 0 => Target::Scalar,
            "recovery_interval": 1 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesSpeedMisconfigured" => dict {
            "recovery": 0 => Target::Scalar,
            "recovery_interval": 1 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesStormControl" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
            "recovery_interval": 2 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesStuckQueue" => dict {
            "recovery": 0 => Target::Scalar,
            "recovery_interval": 1 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesSwitchcardUnreachable" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
            "recovery_interval": 2 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesTapPortInit" => dict {
            "recovery": 0 => Target::Scalar,
            "recovery_interval": 1 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesTapagg" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
            "recovery_interval": 2 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesTpid" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
            "recovery_interval": 2 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesTransceiverAdapter" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
            "recovery_interval": 2 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesUplinkFailureDetection" => dict {
            "recovery": 0 => Target::Scalar,
            "recovery_interval": 1 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesXcvrMisconfigured" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
            "recovery_interval": 2 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesXcvrOverheat" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
            "recovery_interval": 2 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesXcvrPowerUnsupported" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
            "recovery_interval": 2 => Target::Scalar,
        };
        "AVDDesignErrdisableSettingsCausesXcvrUnsupported" => dict {
            "recovery": 0 => Target::Scalar,
            "recovery_interval": 1 => Target::Scalar,
        };
        "AVDDesignEvpnHostflapDetection" => dict {
            "enabled": 0 => Target::Scalar,
            "threshold": 1 => Target::Scalar,
            "window": 2 => Target::Scalar,
            "expiry_timeout": 3 => Target::Scalar,
        };
        "AVDDesignEvpnVlanBundlesList" => indexed(Target::Model("AVDDesignEvpnVlanBundlesListIndexedItem"), [0]);
        "AVDDesignEvpnVlanBundlesItems" => dict {
            "name": 0 => Target::Scalar,
            "id": 1 => Target::Scalar,
            "rt_override": 2 => Target::Scalar,
            "rd_override": 3 => Target::Scalar,
            "evpn_l2_multi_domain": 4 => Target::Scalar,
            "bgp": 5 => Target::Model("AVDDesignEvpnVlanBundlesItemsBgp"),
        };
        "AVDDesignEvpnVlanBundlesItemsBgp" => dict {
            "raw_eos_cli": 0 => Target::Scalar,
        };
        "AVDDesignFabricFlowTracking" => dict {
            "uplinks": 0 => Target::Model("AVDDesignFabricFlowTrackingUplinks"),
            "downlinks": 1 => Target::Model("AVDDesignFabricFlowTrackingDownlinks"),
            "endpoints": 2 => Target::Model("AVDDesignFabricFlowTrackingEndpoints"),
            "l3_edge": 3 => Target::Model("AVDDesignFabricFlowTrackingL3Edge"),
            "core_interfaces": 4 => Target::Model("AVDDesignFabricFlowTrackingCoreInterfaces"),
            "mlag_interfaces": 5 => Target::Model("AVDDesignFabricFlowTrackingMlagInterfaces"),
            "l3_interfaces": 6 => Target::Model("AVDDesignFabricFlowTrackingL3Interfaces"),
            "l3_port_channels": 7 => Target::Model("AVDDesignFabricFlowTrackingL3PortChannels"),
            "dps_interfaces": 8 => Target::Model("AVDDesignFabricFlowTrackingDpsInterfaces"),
            "direct_wan_ha_links": 9 => Target::Model("AVDDesignFabricFlowTrackingDirectWanHaLinks"),
        };
        "AVDDesignFabricFlowTrackingUplinks" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignFabricFlowTrackingDownlinks" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignFabricFlowTrackingEndpoints" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignFabricFlowTrackingL3Edge" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignFabricFlowTrackingCoreInterfaces" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignFabricFlowTrackingMlagInterfaces" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignFabricFlowTrackingL3Interfaces" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignFabricFlowTrackingL3PortChannels" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignFabricFlowTrackingDpsInterfaces" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignFabricFlowTrackingDirectWanHaLinks" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignFabricIpAddressing" => dict {
            "loopback": 0 => Target::Model("AVDDesignFabricIpAddressingLoopback"),
            "mlag": 1 => Target::Model("AVDDesignFabricIpAddressingMlag"),
            "p2p_uplinks": 2 => Target::Model("AVDDesignFabricIpAddressingP2pUplinks"),
            "wan_ha": 3 => Target::Model("AVDDesignFabricIpAddressingWanHa"),
        };
        "AVDDesignFabricIpAddressingLoopback" => dict {
            "ipv6_prefix_length": 0 => Target::Scalar,
        };
        "AVDDesignFabricIpAddressingMlag" => dict {
            "algorithm": 0 => Target::Scalar,
            "ipv4_prefix_length": 1 => Target::Scalar,
            "ipv6_prefix_length": 2 => Target::Scalar,
        };
        "AVDDesignFabricIpAddressingP2pUplinks" => dict {
            "ipv4_prefix_length": 0 => Target::Scalar,
            "ipv6_prefix_length": 1 => Target::Scalar,
        };
        "AVDDesignFabricIpAddressingWanHa" => dict {
            "ipv4_prefix_length": 0 => Target::Scalar,
        };
        "AVDDesignFabricNumbering" => dict {
            "node_id": 0 => Target::Model("AVDDesignFabricNumberingNodeId"),
        };
        "AVDDesignFabricNumberingNodeId" => dict {
            "algorithm": 0 => Target::Scalar,
            "pools_file": 1 => Target::Scalar,
        };
        "AVDDesignFabricSflow" => dict {
            "uplinks": 0 => Target::Scalar,
            "downlinks": 1 => Target::Scalar,
            "endpoints": 2 => Target::Scalar,
            "l3_edge": 3 => Target::Scalar,
            "core_interfaces": 4 => Target::Scalar,
            "mlag_interfaces": 5 => Target::Scalar,
            "l3_interfaces": 6 => Target::Scalar,
        };
        "AVDDesignFlowTrackingSettings" => dict {
            "sampled": 0 => Target::Model("AVDDesignFlowTrackingSettingsSampled"),
            "hardware": 1 => Target::Model("AVDDesignFlowTrackingSettingsHardware"),
            "cloudvision_exporter": 2 => Target::Model("AVDDesignFlowTrackingSettingsCloudvisionExporter"),
            "trackers": 3 => Target::Model("AVDDesignFlowTrackingSettingsTrackersList"),
        };
        "AVDDesignFlowTrackingSettingsSampled" => dict {
            "encapsulation": 0 => Target::Model("AVDDesignFlowTrackingSettingsSampledEncapsulation"),
            "sample": 1 => Target::Scalar,
            "hardware_offload": 2 => Target::Model("AVDDesignFlowTrackingSettingsSampledHardwareOffload"),
        };
        "AVDDesignFlowTrackingSettingsSampledEncapsulation" => dict {
            "ipv4_ipv6": 0 => Target::Scalar,
            "mpls": 1 => Target::Scalar,
        };
        "AVDDesignFlowTrackingSettingsSampledHardwareOffload" => dict {
            "ipv4": 0 => Target::Scalar,
            "ipv6": 1 => Target::Scalar,
            "threshold_minimum": 2 => Target::Scalar,
        };
        "AVDDesignFlowTrackingSettingsHardware" => dict {
            "record": 0 => Target::Model("AVDDesignFlowTrackingSettingsHardwareRecord"),
        };
        "AVDDesignFlowTrackingSettingsHardwareRecord" => dict {
            "format_ipfix_standard_timestamps_counters": 0 => Target::Scalar,
        };
        "AVDDesignFlowTrackingSettingsCloudvisionExporter" => dict {
            "name": 0 => Target::Scalar,
            "vrf": 1 => Target::Scalar,
            "source_interface": 2 => Target::Scalar,
        };
        "AVDDesignFlowTrackingSettingsTrackersList" => indexed(Target::Model("AVDDesignFlowTrackingSettingsTrackersListIndexedItem"), [0]);
        "AVDDesignFlowTrackingSettingsTrackersItems" => dict {
            "name": 0 => Target::Scalar,
            "sampled": 1 => Target::Model("AVDDesignFlowTrackingSettingsTrackersItemsSampled"),
            "record_export": 2 => Target::Model("AVDDesignFlowTrackingSettingsTrackersItemsRecordExport"),
            "export_to_cloudvision": 3 => Target::Scalar,
            "exporters": 4 => Target::Model("AVDDesignFlowTrackingSettingsTrackersItemsExportersList"),
        };
        "AVDDesignFlowTrackingSettingsTrackersItemsSampled" => dict {
            "table_size": 0 => Target::Scalar,
            "record_export": 1 => Target::Model("AVDDesignFlowTrackingSettingsTrackersItemsSampledRecordExport"),
        };
        "AVDDesignFlowTrackingSettingsTrackersItemsSampledRecordExport" => dict {
            "mpls": 0 => Target::Scalar,
        };
        "AVDDesignFlowTrackingSettingsTrackersItemsRecordExport" => dict {
            "on_inactive_timeout": 0 => Target::Scalar,
            "on_interval": 1 => Target::Scalar,
        };
        "AVDDesignFlowTrackingSettingsTrackersItemsExportersList" => indexed(Target::Model("AVDDesignFlowTrackingSettingsTrackersItemsExportersListIndexedItem"), [0]);
        "AVDDesignFlowTrackingSettingsTrackersItemsExportersItems" => dict {
            "name": 0 => Target::Scalar,
            "collectors": 1 => Target::Model("AVDDesignFlowTrackingSettingsTrackersItemsExportersItemsCollectorsList"),
            "format": 2 => Target::Model("AVDDesignFlowTrackingSettingsTrackersItemsExportersItemsFormat"),
            "local_interface": 3 => Target::Scalar,
            "template_interval": 4 => Target::Scalar,
        };
        "AVDDesignFlowTrackingSettingsTrackersItemsExportersItemsCollectorsList" => indexed(Target::Model("AVDDesignFlowTrackingSettingsTrackersItemsExportersItemsCollectorsListIndexedItem"), [0]);
        "AVDDesignFlowTrackingSettingsTrackersItemsExportersItemsCollectorsItems" => dict {
            "host": 0 => Target::Scalar,
            "port": 1 => Target::Scalar,
        };
        "AVDDesignFlowTrackingSettingsTrackersItemsExportersItemsFormat" => dict {
            "ipfix_version": 0 => Target::Scalar,
        };
        "AVDDesignGeneralSettings" => dict {
            "interface_defaults": 0 => Target::Model("AVDDesignGeneralSettingsInterfaceDefaults"),
            "arp": 1 => Target::Model("AVDDesignGeneralSettingsArp"),
            "ip_icmp_redirect": 2 => Target::Scalar,
            "dhcp_relay": 3 => Target::Model("AVDDesignGeneralSettingsDhcpRelay"),
            "suspended_vlans": 4 => Target::Model("AVDDesignGeneralSettingsSuspendedVlansList"),
        };
        "AVDDesignGeneralSettingsInterfaceDefaults" => dict {
            "ethernet_shutdown": 0 => Target::Scalar,
        };
        "AVDDesignGeneralSettingsArp" => dict {
            "persistent": 0 => Target::Model("EosCliConfigGenArpPersistent"),
            "aging": 1 => Target::Model("EosCliConfigGenArpAging"),
        };
        "AVDDesignGeneralSettingsDhcpRelay" => dict {
            "information_option": 0 => Target::Scalar,
            "tunnel_requests_disabled": 1 => Target::Scalar,
            "mlag_peerlink_requests_disabled": 2 => Target::Scalar,
        };
        "AVDDesignGeneralSettingsSuspendedVlansList" => indexed(Target::Model("AVDDesignGeneralSettingsSuspendedVlansListIndexedItem"), [0]);
        "AVDDesignGeneralSettingsSuspendedVlansItems" => dict {
            "id": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignGenerateCvTags" => dict {
            "topology_hints": 0 => Target::Scalar,
            "campus_fabric": 1 => Target::Scalar,
            "interface_tags": 2 => Target::Model("AVDDesignGenerateCvTagsInterfaceTagsList"),
            "device_tags": 3 => Target::Model("AVDDesignGenerateCvTagsDeviceTagsList"),
        };
        "AVDDesignGenerateCvTagsInterfaceTagsList" => indexed(Target::Model("AVDDesignGenerateCvTagsInterfaceTagsListIndexedItem"), [0]);
        "AVDDesignGenerateCvTagsInterfaceTagsItems" => dict {
            "name": 0 => Target::Scalar,
            "data_path": 1 => Target::Scalar,
            "value": 2 => Target::Scalar,
        };
        "AVDDesignGenerateCvTagsDeviceTagsList" => list(Target::Model("AVDDesignGenerateCvTagsDeviceTagsItems"));
        "AVDDesignGenerateCvTagsDeviceTagsItems" => dict {
            "name": 0 => Target::Scalar,
            "data_path": 1 => Target::Scalar,
            "value": 2 => Target::Scalar,
        };
        "AVDDesignInternalVlanOrder" => dict {
            "allocation": 0 => Target::Scalar,
            "range": 1 => Target::Model("AVDDesignInternalVlanOrderRange"),
        };
        "AVDDesignInternalVlanOrderRange" => dict {
            "beginning": 0 => Target::Scalar,
            "ending": 1 => Target::Scalar,
        };
        "AVDDesignIpsecSettings" => dict {
            "bind_connection_to_interface": 0 => Target::Scalar,
        };
        "AVDDesignIpv4AclsList" => indexed(Target::Model("AVDDesignIpv4AclsListIndexedItem"), [0]);
        "AVDDesignIpv4AclsItems" => dict {
            "name": 0 => Target::Scalar,
            "entries": 1 => Target::Model("AVDDesignIpv4AclsItemsEntriesList"),
            "counters_per_entry": 2 => Target::Scalar,
            "permit_response_traffic": 3 => Target::Scalar,
        };
        "AVDDesignIpv4AclsItemsEntriesList" => list(Target::Model("AVDDesignIpv4AclsItemsEntriesItems"));
        "AVDDesignIpv4AclsItemsEntriesItems" => dict {
            "source": 0 => Target::Scalar,
            "destination": 1 => Target::Scalar,
            "sequence": 2 => Target::Scalar,
            "remark": 3 => Target::Scalar,
            "action": 4 => Target::Scalar,
            "protocol": 5 => Target::Scalar,
            "fragments": 6 => Target::Scalar,
            "ttl": 7 => Target::Scalar,
            "ttl_match": 8 => Target::Scalar,
            "vlan_inner": 9 => Target::Scalar,
            "source_ports_match": 10 => Target::Scalar,
            "source_ports": 11 => Target::Model("AVDDesignIpv4AclsItemsEntriesItemsSourcePortsList"),
            "destination_ports_match": 12 => Target::Scalar,
            "destination_ports": 13 => Target::Model("AVDDesignIpv4AclsItemsEntriesItemsDestinationPortsList"),
            "tcp_flags": 14 => Target::Model("AVDDesignIpv4AclsItemsEntriesItemsTcpFlagsList"),
            "copy_captive_portal": 15 => Target::Scalar,
            "log": 16 => Target::Scalar,
            "icmp_type": 17 => Target::Scalar,
            "icmp_code": 18 => Target::Scalar,
            "nexthop_group": 19 => Target::Scalar,
            "tracked": 20 => Target::Scalar,
            "dscp": 21 => Target::Scalar,
            "vlan_number": 22 => Target::Scalar,
            "vlan_mask": 23 => Target::Scalar,
            "inner_vlan_number": 24 => Target::Scalar,
            "inner_vlan_mask": 25 => Target::Scalar,
        };
        "AVDDesignIpv4AclsItemsEntriesItemsSourcePortsList" => list(Target::Scalar);
        "AVDDesignIpv4AclsItemsEntriesItemsDestinationPortsList" => list(Target::Scalar);
        "AVDDesignIpv4AclsItemsEntriesItemsTcpFlagsList" => list(Target::Scalar);
        "AVDDesignIpv4PrefixListCatalogList" => indexed(Target::Model("AVDDesignIpv4PrefixListCatalogListIndexedItem"), [0]);
        "AVDDesignIpv4PrefixListCatalogItems" => dict {
            "name": 0 => Target::Scalar,
            "sequence_numbers": 1 => Target::Model("AVDDesignIpv4PrefixListCatalogItemsSequenceNumbersList"),
        };
        "AVDDesignIpv4PrefixListCatalogItemsSequenceNumbersList" => indexed(Target::Model("AVDDesignIpv4PrefixListCatalogItemsSequenceNumbersListIndexedItem"), [0]);
        "AVDDesignIpv4PrefixListCatalogItemsSequenceNumbersItems" => dict {
            "sequence": 0 => Target::Scalar,
            "action": 1 => Target::Scalar,
        };
        "AVDDesignIpv4StandardAclsList" => indexed(Target::Model("AVDDesignIpv4StandardAclsListIndexedItem"), [0]);
        "AVDDesignIpv4StandardAclsItems" => dict {
            "name": 0 => Target::Scalar,
            "counters_per_entry": 1 => Target::Scalar,
            "entries": 2 => Target::Model("AVDDesignIpv4StandardAclsItemsEntriesList"),
        };
        "AVDDesignIpv4StandardAclsItemsEntriesList" => list(Target::Model("AVDDesignIpv4StandardAclsItemsEntriesItems"));
        "AVDDesignIpv4StandardAclsItemsEntriesItems" => dict {
            "sequence": 0 => Target::Scalar,
            "action": 1 => Target::Scalar,
            "remark": 2 => Target::Scalar,
            "source": 3 => Target::Scalar,
            "vlan": 4 => Target::Scalar,
            "vlan_mask": 5 => Target::Scalar,
            "inner_vlan": 6 => Target::Scalar,
            "inner_vlan_mask": 7 => Target::Scalar,
            "log": 8 => Target::Scalar,
            "mirror_session": 9 => Target::Scalar,
        };
        "AVDDesignIpv6AclsList" => indexed(Target::Model("AVDDesignIpv6AclsListIndexedItem"), [0]);
        "AVDDesignIpv6AclsItems" => dict {
            "name": 0 => Target::Scalar,
            "entries": 1 => Target::Model("AVDDesignIpv6AclsItemsEntriesList"),
            "counters_per_entry": 2 => Target::Scalar,
            "sequence_numbers": 3 => Target::Model("AVDDesignIpv6AclsItemsSequenceNumbersList"),
        };
        "AVDDesignIpv6AclsItemsEntriesList" => list(Target::Model("AVDDesignIpv6AclsItemsEntriesItems"));
        "AVDDesignIpv6AclsItemsEntriesItems" => dict {
            "source": 0 => Target::Scalar,
            "destination": 1 => Target::Scalar,
            "protocol": 2 => Target::Scalar,
            "hop_limit": 3 => Target::Scalar,
            "hop_limit_match": 4 => Target::Scalar,
            "dscp_mask": 5 => Target::Scalar,
            "sequence": 6 => Target::Scalar,
            "remark": 7 => Target::Scalar,
            "action": 8 => Target::Scalar,
            "source_ports_match": 9 => Target::Scalar,
            "source_ports": 10 => Target::Model("AVDDesignIpv6AclsItemsEntriesItemsSourcePortsList"),
            "destination_ports_match": 11 => Target::Scalar,
            "destination_ports": 12 => Target::Model("AVDDesignIpv6AclsItemsEntriesItemsDestinationPortsList"),
            "tcp_flags": 13 => Target::Model("AVDDesignIpv6AclsItemsEntriesItemsTcpFlagsList"),
            "copy_captive_portal": 14 => Target::Scalar,
            "log": 15 => Target::Scalar,
            "icmp_type": 16 => Target::Scalar,
            "icmp_code": 17 => Target::Scalar,
            "nexthop_group": 18 => Target::Scalar,
            "tracked": 19 => Target::Scalar,
            "dscp": 20 => Target::Scalar,
            "vlan_number": 21 => Target::Scalar,
            "vlan_mask": 22 => Target::Scalar,
            "inner_vlan_number": 23 => Target::Scalar,
            "inner_vlan_mask": 24 => Target::Scalar,
        };
        "AVDDesignIpv6AclsItemsEntriesItemsSourcePortsList" => list(Target::Scalar);
        "AVDDesignIpv6AclsItemsEntriesItemsDestinationPortsList" => list(Target::Scalar);
        "AVDDesignIpv6AclsItemsEntriesItemsTcpFlagsList" => list(Target::Scalar);
        "AVDDesignIpv6AclsItemsSequenceNumbersList" => indexed(Target::Model("AVDDesignIpv6AclsItemsSequenceNumbersListIndexedItem"), [0]);
        "AVDDesignIpv6AclsItemsSequenceNumbersItems" => dict {
            "sequence": 0 => Target::Scalar,
            "action": 1 => Target::Scalar,
        };
        "AVDDesignIpv6MgmtDestinationNetworksList" => list(Target::Scalar);
        "AVDDesignIpv6PrefixListCatalogList" => indexed(Target::Model("AVDDesignIpv6PrefixListCatalogListIndexedItem"), [0]);
        "AVDDesignIpv6PrefixListCatalogItems" => dict {
            "name": 0 => Target::Scalar,
            "sequence_numbers": 1 => Target::Model("AVDDesignIpv6PrefixListCatalogItemsSequenceNumbersList"),
        };
        "AVDDesignIpv6PrefixListCatalogItemsSequenceNumbersList" => indexed(Target::Model("AVDDesignIpv6PrefixListCatalogItemsSequenceNumbersListIndexedItem"), [0]);
        "AVDDesignIpv6PrefixListCatalogItemsSequenceNumbersItems" => dict {
            "sequence": 0 => Target::Scalar,
            "action": 1 => Target::Scalar,
        };
        "AVDDesignIsisTiLfa" => dict {
            "enabled": 0 => Target::Scalar,
            "protection": 1 => Target::Scalar,
            "local_convergence_delay": 2 => Target::Scalar,
        };
        "AVDDesignL2vlanProfilesList" => indexed(Target::Model("AVDDesignL2vlanProfilesListIndexedItem"), [0]);
        "AVDDesignL2vlanProfilesItems" => dict {
            "profile": 0 => Target::Scalar,
            "parent_profile": 1 => Target::Scalar,
            "address_locking": 2 => Target::Model("EosCliConfigGenVlansItemsAddressLockingAddressFamily"),
            "vni_override": 3 => Target::Scalar,
            "rt_override": 4 => Target::Scalar,
            "rd_override": 5 => Target::Scalar,
            "vxlan": 6 => Target::Scalar,
            "spanning_tree_priority": 7 => Target::Scalar,
            "evpn_vlan_bundle": 8 => Target::Scalar,
            "trunk_groups": 9 => Target::Model("AVDDesignL2vlanProfilesItemsTrunkGroupsList"),
            "evpn_l2_multi_domain": 10 => Target::Scalar,
            "evpn_l2_multicast": 11 => Target::Model("AVDDesignL2vlanProfilesItemsEvpnL2Multicast"),
            "vxlan_flood_multicast": 12 => Target::Model("AVDDesignL2vlanProfilesItemsVxlanFloodMulticast"),
            "igmp_snooping": 13 => Target::Model("AVDDesignL2vlanProfilesItemsIgmpSnooping"),
            "igmp_snooping_enabled": 14 => Target::Scalar,
            "igmp_snooping_querier": 15 => Target::Model("AVDDesignL2vlanProfilesItemsIgmpSnoopingQuerier2"),
            "bgp": 16 => Target::Model("AVDDesignL2vlanProfilesItemsBgp"),
            "private_vlan": 17 => Target::Model("AVDDesignL2vlanProfilesItemsPrivateVlan"),
        };
        "AVDDesignL2vlanProfilesItemsTrunkGroupsList" => list(Target::Scalar);
        "AVDDesignL2vlanProfilesItemsEvpnL2Multicast" => dict {
            "enabled": 0 => Target::Scalar,
        };
        "AVDDesignL2vlanProfilesItemsVxlanFloodMulticast" => dict {
            "enabled": 0 => Target::Scalar,
            "underlay_multicast_group": 1 => Target::Scalar,
        };
        "AVDDesignL2vlanProfilesItemsIgmpSnooping" => dict {
            "enabled": 0 => Target::Scalar,
            "querier": 1 => Target::Model("AVDDesignL2vlanProfilesItemsIgmpSnoopingQuerier"),
            "fast_leave": 2 => Target::Scalar,
        };
        "AVDDesignL2vlanProfilesItemsIgmpSnoopingQuerier" => dict {
            "enabled": 0 => Target::Scalar,
            "source_address": 1 => Target::Scalar,
            "version": 2 => Target::Scalar,
        };
        "AVDDesignL2vlanProfilesItemsIgmpSnoopingQuerier2" => dict {
            "enabled": 0 => Target::Scalar,
            "source_address": 1 => Target::Scalar,
            "version": 2 => Target::Scalar,
            "fast_leave": 3 => Target::Scalar,
        };
        "AVDDesignL2vlanProfilesItemsBgp" => dict {
            "structured_config": 0 => Target::Opaque,
            "raw_eos_cli": 1 => Target::Scalar,
        };
        "AVDDesignL2vlanProfilesItemsPrivateVlan" => dict {
            "field_type": 0 => Target::Scalar,
            "primary_vlan": 1 => Target::Scalar,
        };
        "AVDDesignL3Edge" => dict {
            "p2p_links_ip_pools": 0 => Target::Model("AVDDesignL3EdgeP2pLinksIpPoolsList"),
            "p2p_links_profiles": 1 => Target::Model("AVDDesignL3EdgeP2pLinksProfilesList"),
            "p2p_links": 2 => Target::Model("AVDDesignL3EdgeP2pLinksList"),
        };
        "AVDDesignL3EdgeP2pLinksIpPoolsList" => indexed(Target::Model("AVDDesignL3EdgeP2pLinksIpPoolsListIndexedItem"), [0]);
        "AVDDesignL3EdgeP2pLinksIpPoolsItems" => dict {
            "name": 0 => Target::Scalar,
            "ipv4_pool": 1 => Target::Scalar,
            "prefix_size": 2 => Target::Scalar,
            "ipv6_pool": 3 => Target::Scalar,
            "ipv6_prefix_size": 4 => Target::Scalar,
        };
        "AVDDesignL3EdgeP2pLinksProfilesList" => indexed(Target::Model("AVDDesignL3EdgeP2pLinksProfilesListIndexedItem"), [0]);
        "AVDDesignL3EdgeP2pLinksProfilesItems" => dict {
            "name": 0 => Target::Scalar,
            "id": 1 => Target::Scalar,
            "speed": 2 => Target::Scalar,
            "ip_pool": 3 => Target::Scalar,
            "subnet": 4 => Target::Scalar,
            "ip": 5 => Target::Model("AVDDesignL3EdgeP2pLinksProfilesItemsIpList"),
            "ipv6_enable": 6 => Target::Scalar,
            "ipv6_prefix": 7 => Target::Scalar,
            "ipv6": 8 => Target::Model("AVDDesignL3EdgeP2pLinksProfilesItemsIpv6List"),
            "nodes": 9 => Target::Model("AVDDesignL3EdgeP2pLinksProfilesItemsNodesList"),
            "interfaces": 10 => Target::Model("AVDDesignL3EdgeP2pLinksProfilesItemsInterfacesList"),
            "field_as": 11 => Target::Model("AVDDesignL3EdgeP2pLinksProfilesItemsAsList"),
            "descriptions": 12 => Target::Model("AVDDesignL3EdgeP2pLinksProfilesItemsDescriptionsList"),
            "include_in_underlay_protocol": 13 => Target::Scalar,
            "use_underlay_ospf_authentication": 14 => Target::Scalar,
            "isis_hello_padding": 15 => Target::Scalar,
            "isis_metric": 16 => Target::Scalar,
            "isis_circuit_type": 17 => Target::Scalar,
            "isis_authentication_mode": 18 => Target::Scalar,
            "isis_authentication_key": 19 => Target::Scalar,
            "isis_authentication_cleartext_key": 20 => Target::Scalar,
            "isis_network_type": 21 => Target::Scalar,
            "mpls_ip": 22 => Target::Scalar,
            "mpls_ldp": 23 => Target::Scalar,
            "mtu": 24 => Target::Scalar,
            "bfd": 25 => Target::Scalar,
            "ptp": 26 => Target::Model("AVDDesignL3EdgeP2pLinksProfilesItemsPtp"),
            "sflow": 27 => Target::Scalar,
            "multicast_pim_sm": 28 => Target::Scalar,
            "multicast_static": 29 => Target::Scalar,
            "flow_tracking": 30 => Target::Model("AVDDesignL3EdgeP2pLinksProfilesItemsFlowTracking"),
            "qos_profile": 31 => Target::Scalar,
            "macsec_profile": 32 => Target::Scalar,
            "port_channel": 33 => Target::Model("AVDDesignL3EdgeP2pLinksProfilesItemsPortChannel"),
            "campus_link_type": 34 => Target::Model("AVDDesignL3EdgeP2pLinksProfilesItemsCampusLinkTypeList"),
            "raw_eos_cli": 35 => Target::Scalar,
            "routing_protocol": 36 => Target::Scalar,
            "ethernet_structured_config": 37 => Target::Opaque,
            "port_channel_structured_config": 38 => Target::Opaque,
        };
        "AVDDesignL3EdgeP2pLinksProfilesItemsIpList" => list(Target::Scalar);
        "AVDDesignL3EdgeP2pLinksProfilesItemsIpv6List" => list(Target::Scalar);
        "AVDDesignL3EdgeP2pLinksProfilesItemsNodesList" => list(Target::Scalar);
        "AVDDesignL3EdgeP2pLinksProfilesItemsInterfacesList" => list(Target::Scalar);
        "AVDDesignL3EdgeP2pLinksProfilesItemsAsList" => list(Target::Scalar);
        "AVDDesignL3EdgeP2pLinksProfilesItemsDescriptionsList" => list(Target::Scalar);
        "AVDDesignL3EdgeP2pLinksProfilesItemsPtp" => dict {
            "enabled": 0 => Target::Scalar,
            "roles": 1 => Target::Model("AVDDesignL3EdgeP2pLinksProfilesItemsPtpRolesList"),
            "profile": 2 => Target::Scalar,
        };
        "AVDDesignL3EdgeP2pLinksProfilesItemsPtpRolesList" => list(Target::Scalar);
        "AVDDesignL3EdgeP2pLinksProfilesItemsFlowTracking" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignL3EdgeP2pLinksProfilesItemsPortChannel" => dict {
            "description": 0 => Target::Scalar,
            "mode": 1 => Target::Scalar,
            "channel_id_algorithm": 2 => Target::Scalar,
            "channel_id_offset": 3 => Target::Scalar,
            "nodes_child_interfaces": 4 => Target::Model("AVDDesignL3EdgeP2pLinksProfilesItemsPortChannelNodesChildInterfacesList"),
        };
        "AVDDesignL3EdgeP2pLinksProfilesItemsPortChannelNodesChildInterfacesList" => indexed(Target::Model("AVDDesignL3EdgeP2pLinksProfilesItemsPortChannelNodesChildInterfacesListIndexedItem"), [0]);
        "AVDDesignL3EdgeP2pLinksProfilesItemsPortChannelNodesChildInterfacesItems" => dict {
            "node": 0 => Target::Scalar,
            "interfaces": 1 => Target::Model("AVDDesignL3EdgeP2pLinksProfilesItemsPortChannelNodesChildInterfacesItemsInterfacesList"),
            "channel_id": 2 => Target::Scalar,
        };
        "AVDDesignL3EdgeP2pLinksProfilesItemsPortChannelNodesChildInterfacesItemsInterfacesList" => list(Target::Scalar);
        "AVDDesignL3EdgeP2pLinksProfilesItemsCampusLinkTypeList" => list(Target::Scalar);
        "AVDDesignL3EdgeP2pLinksList" => list(Target::Model("AVDDesignL3EdgeP2pLinksItems"));
        "AVDDesignL3EdgeP2pLinksItems" => dict {
            "nodes": 0 => Target::Model("AVDDesignL3EdgeP2pLinksItemsNodesList"),
            "profile": 1 => Target::Scalar,
            "id": 2 => Target::Scalar,
            "speed": 3 => Target::Scalar,
            "ip_pool": 4 => Target::Scalar,
            "subnet": 5 => Target::Scalar,
            "ip": 6 => Target::Model("AVDDesignL3EdgeP2pLinksItemsIpList"),
            "ipv6_enable": 7 => Target::Scalar,
            "ipv6_prefix": 8 => Target::Scalar,
            "ipv6": 9 => Target::Model("AVDDesignL3EdgeP2pLinksItemsIpv6List"),
            "interfaces": 10 => Target::Model("AVDDesignL3EdgeP2pLinksItemsInterfacesList"),
            "field_as": 11 => Target::Model("AVDDesignL3EdgeP2pLinksItemsAsList"),
            "descriptions": 12 => Target::Model("AVDDesignL3EdgeP2pLinksItemsDescriptionsList"),
            "include_in_underlay_protocol": 13 => Target::Scalar,
            "use_underlay_ospf_authentication": 14 => Target::Scalar,
            "isis_hello_padding": 15 => Target::Scalar,
            "isis_metric": 16 => Target::Scalar,
            "isis_circuit_type": 17 => Target::Scalar,
            "isis_authentication_mode": 18 => Target::Scalar,
            "isis_authentication_key": 19 => Target::Scalar,
            "isis_authentication_cleartext_key": 20 => Target::Scalar,
            "isis_network_type": 21 => Target::Scalar,
            "mpls_ip": 22 => Target::Scalar,
            "mpls_ldp": 23 => Target::Scalar,
            "mtu": 24 => Target::Scalar,
            "bfd": 25 => Target::Scalar,
            "ptp": 26 => Target::Model("AVDDesignL3EdgeP2pLinksItemsPtp"),
            "sflow": 27 => Target::Scalar,
            "multicast_pim_sm": 28 => Target::Scalar,
            "multicast_static": 29 => Target::Scalar,
            "flow_tracking": 30 => Target::Model("AVDDesignL3EdgeP2pLinksItemsFlowTracking"),
            "qos_profile": 31 => Target::Scalar,
            "macsec_profile": 32 => Target::Scalar,
            "port_channel": 33 => Target::Model("AVDDesignL3EdgeP2pLinksItemsPortChannel"),
            "campus_link_type": 34 => Target::Model("AVDDesignL3EdgeP2pLinksItemsCampusLinkTypeList"),
            "raw_eos_cli": 35 => Target::Scalar,
            "routing_protocol": 36 => Target::Scalar,
            "ethernet_structured_config": 37 => Target::Opaque,
            "port_channel_structured_config": 38 => Target::Opaque,
        };
        "AVDDesignL3EdgeP2pLinksItemsNodesList" => list(Target::Scalar);
        "AVDDesignL3EdgeP2pLinksItemsIpList" => list(Target::Scalar);
        "AVDDesignL3EdgeP2pLinksItemsIpv6List" => list(Target::Scalar);
        "AVDDesignL3EdgeP2pLinksItemsInterfacesList" => list(Target::Scalar);
        "AVDDesignL3EdgeP2pLinksItemsAsList" => list(Target::Scalar);
        "AVDDesignL3EdgeP2pLinksItemsDescriptionsList" => list(Target::Scalar);
        "AVDDesignL3EdgeP2pLinksItemsPtp" => dict {
            "enabled": 0 => Target::Scalar,
            "roles": 1 => Target::Model("AVDDesignL3EdgeP2pLinksItemsPtpRolesList"),
            "profile": 2 => Target::Scalar,
        };
        "AVDDesignL3EdgeP2pLinksItemsPtpRolesList" => list(Target::Scalar);
        "AVDDesignL3EdgeP2pLinksItemsFlowTracking" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignL3EdgeP2pLinksItemsPortChannel" => dict {
            "description": 0 => Target::Scalar,
            "mode": 1 => Target::Scalar,
            "channel_id_algorithm": 2 => Target::Scalar,
            "channel_id_offset": 3 => Target::Scalar,
            "nodes_child_interfaces": 4 => Target::Model("AVDDesignL3EdgeP2pLinksItemsPortChannelNodesChildInterfacesList"),
        };
        "AVDDesignL3EdgeP2pLinksItemsPortChannelNodesChildInterfacesList" => indexed(Target::Model("AVDDesignL3EdgeP2pLinksItemsPortChannelNodesChildInterfacesListIndexedItem"), [0]);
        "AVDDesignL3EdgeP2pLinksItemsPortChannelNodesChildInterfacesItems" => dict {
            "node": 0 => Target::Scalar,
            "interfaces": 1 => Target::Model("AVDDesignL3EdgeP2pLinksItemsPortChannelNodesChildInterfacesItemsInterfacesList"),
            "channel_id": 2 => Target::Scalar,
        };
        "AVDDesignL3EdgeP2pLinksItemsPortChannelNodesChildInterfacesItemsInterfacesList" => list(Target::Scalar);
        "AVDDesignL3EdgeP2pLinksItemsCampusLinkTypeList" => list(Target::Scalar);
        "AVDDesignL3InterfaceProfilesList" => indexed(Target::Model("AVDDesignL3InterfaceProfilesListIndexedItem"), [0]);
        "AVDDesignL3InterfaceProfilesItems" => dict {
            "profile": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
            "description": 2 => Target::Scalar,
            "ip_address": 3 => Target::Scalar,
            "ipv6_addresses": 4 => Target::Model("AVDDesignL3InterfaceProfilesItemsIpv6AddressesList"),
            "dhcp_ip": 5 => Target::Scalar,
            "public_ip": 6 => Target::Scalar,
            "encapsulation_dot1q_vlan": 7 => Target::Scalar,
            "dhcp_accept_default_route": 8 => Target::Scalar,
            "enabled": 9 => Target::Scalar,
            "speed": 10 => Target::Scalar,
            "receive_bandwidth": 11 => Target::Scalar,
            "transmit_bandwidth": 12 => Target::Scalar,
            "peer": 13 => Target::Scalar,
            "peer_interface": 14 => Target::Scalar,
            "peer_ip": 15 => Target::Scalar,
            "peer_ipv6": 16 => Target::Scalar,
            "bgp": 17 => Target::Model("AVDDesignL3InterfaceProfilesItemsBgp"),
            "ipv4_acl_in": 18 => Target::Scalar,
            "ipv4_acl_out": 19 => Target::Scalar,
            "ipv6_acl_in": 20 => Target::Scalar,
            "ipv6_acl_out": 21 => Target::Scalar,
            "static_routes": 22 => Target::Model("AVDDesignL3InterfaceProfilesItemsStaticRoutesList"),
            "qos_profile": 23 => Target::Scalar,
            "wan_carrier": 24 => Target::Scalar,
            "wan_circuit_id": 25 => Target::Scalar,
            "connected_to_pathfinder": 26 => Target::Scalar,
            "cv_pathfinder_internet_exit": 27 => Target::Model("AVDDesignL3InterfaceProfilesItemsCvPathfinderInternetExit"),
            "rx_queue": 28 => Target::Model("AVDDesignL3InterfaceProfilesItemsRxQueue"),
            "raw_eos_cli": 29 => Target::Scalar,
            "flow_tracking": 30 => Target::Model("AVDDesignL3InterfaceProfilesItemsFlowTracking"),
            "structured_config": 31 => Target::Opaque,
        };
        "AVDDesignL3InterfaceProfilesItemsIpv6AddressesList" => list(Target::Scalar);
        "AVDDesignL3InterfaceProfilesItemsBgp" => dict {
            "peer_as": 0 => Target::Scalar,
            "ipv4_prefix_list_in": 1 => Target::Scalar,
            "ipv4_prefix_list_out": 2 => Target::Scalar,
            "ipv6_prefix_list_in": 3 => Target::Scalar,
            "ipv6_prefix_list_out": 4 => Target::Scalar,
        };
        "AVDDesignL3InterfaceProfilesItemsStaticRoutesList" => list(Target::Model("AVDDesignL3InterfaceProfilesItemsStaticRoutesItems"));
        "AVDDesignL3InterfaceProfilesItemsStaticRoutesItems" => dict {
            "prefix": 0 => Target::Scalar,
        };
        "AVDDesignL3InterfaceProfilesItemsCvPathfinderInternetExit" => dict {
            "policies": 0 => Target::Model("AVDDesignL3InterfaceProfilesItemsCvPathfinderInternetExitPoliciesList"),
        };
        "AVDDesignL3InterfaceProfilesItemsCvPathfinderInternetExitPoliciesList" => indexed(Target::Model("AVDDesignL3InterfaceProfilesItemsCvPathfinderInternetExitPoliciesListIndexedItem"), [0]);
        "AVDDesignL3InterfaceProfilesItemsCvPathfinderInternetExitPoliciesItems" => dict {
            "name": 0 => Target::Scalar,
            "tunnel_interface_numbers": 1 => Target::Scalar,
        };
        "AVDDesignL3InterfaceProfilesItemsRxQueue" => dict {
            "count": 0 => Target::Scalar,
            "workers": 1 => Target::Model("AVDDesignL3InterfaceProfilesItemsRxQueueWorkersList"),
            "mode": 2 => Target::Scalar,
        };
        "AVDDesignL3InterfaceProfilesItemsRxQueueWorkersList" => list(Target::Scalar);
        "AVDDesignL3InterfaceProfilesItemsFlowTracking" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignLoggingSettings" => dict {
            "use_local_interface_cli": 0 => Target::Scalar,
            "hosts": 1 => Target::Model("AVDDesignLoggingSettingsHostsList"),
            "vrfs": 2 => Target::Model("AVDDesignLoggingSettingsVrfsList"),
            "console": 3 => Target::Scalar,
            "monitor": 4 => Target::Scalar,
            "buffered": 5 => Target::Model("EosCliConfigGenLoggingBuffered"),
            "repeat_messages": 6 => Target::Scalar,
            "trap": 7 => Target::Scalar,
            "synchronous": 8 => Target::Model("EosCliConfigGenLoggingSynchronous"),
            "format": 9 => Target::Model("EosCliConfigGenLoggingFormat"),
            "facility": 10 => Target::Scalar,
            "policy": 11 => Target::Model("EosCliConfigGenLoggingPolicy"),
            "event": 12 => Target::Model("EosCliConfigGenLoggingEvent"),
            "level": 13 => Target::Model("EosCliConfigGenLoggingLevelList"),
            "monitor_layer1": 14 => Target::Model("EosCliConfigGenMonitorLayer1"),
        };
        "AVDDesignLoggingSettingsHostsList" => list(Target::Model("AVDDesignLoggingSettingsHostsItems"));
        "AVDDesignLoggingSettingsHostsItems" => dict {
            "name": 0 => Target::Scalar,
            "vrf": 1 => Target::Scalar,
            "protocol": 2 => Target::Scalar,
            "ports": 3 => Target::Model("AVDDesignLoggingSettingsHostsItemsPortsList"),
            "ssl_profile": 4 => Target::Scalar,
        };
        "AVDDesignLoggingSettingsHostsItemsPortsList" => list(Target::Scalar);
        "AVDDesignLoggingSettingsVrfsList" => indexed(Target::Model("AVDDesignLoggingSettingsVrfsListIndexedItem"), [0]);
        "AVDDesignLoggingSettingsVrfsItems" => dict {
            "name": 0 => Target::Scalar,
            "source_interface": 1 => Target::Scalar,
        };
        "AVDDesignMacAclsList" => indexed(Target::Model("AVDDesignMacAclsListIndexedItem"), [0]);
        "AVDDesignMacAclsItems" => dict {
            "name": 0 => Target::Scalar,
            "counters_per_entry": 1 => Target::Scalar,
            "entries": 2 => Target::Model("AVDDesignMacAclsItemsEntriesList"),
        };
        "AVDDesignMacAclsItemsEntriesList" => list(Target::Model("AVDDesignMacAclsItemsEntriesItems"));
        "AVDDesignMacAclsItemsEntriesItems" => dict {
            "sequence": 0 => Target::Scalar,
            "action": 1 => Target::Scalar,
            "remark": 2 => Target::Scalar,
            "source": 3 => Target::Scalar,
            "source_wildcard": 4 => Target::Scalar,
            "destination": 5 => Target::Scalar,
            "destination_wildcard": 6 => Target::Scalar,
        };
        "AVDDesignManagementEapi" => dict {
            "enabled": 0 => Target::Scalar,
            "enable_http": 1 => Target::Scalar,
            "enable_https": 2 => Target::Scalar,
            "default_services": 3 => Target::Scalar,
            "vrfs": 4 => Target::Model("AVDDesignManagementEapiVrfsList"),
        };
        "AVDDesignManagementEapiVrfsList" => indexed(Target::Model("AVDDesignManagementEapiVrfsListIndexedItem"), [0]);
        "AVDDesignManagementEapiVrfsItems" => dict {
            "name": 0 => Target::Scalar,
            "enabled": 1 => Target::Scalar,
            "ipv4_acl": 2 => Target::Scalar,
            "ipv6_acl": 3 => Target::Scalar,
        };
        "AVDDesignManagementSettings" => dict {
            "console": 0 => Target::Model("EosCliConfigGenManagementConsole"),
            "banners": 1 => Target::Model("EosCliConfigGenBanners"),
        };
        "AVDDesignMgmtDestinationNetworksList" => list(Target::Scalar);
        "AVDDesignMgmtInterfaceSettings" => dict {
            "description": 0 => Target::Scalar,
            "vrf": 1 => Target::Scalar,
            "vrf_routing": 2 => Target::Scalar,
            "interface": 3 => Target::Scalar,
            "lldp": 4 => Target::Model("AVDDesignMgmtInterfaceSettingsLldp"),
        };
        "AVDDesignMgmtInterfaceSettingsLldp" => dict {
            "transmit": 0 => Target::Scalar,
            "receive": 1 => Target::Scalar,
            "ztp_vlan": 2 => Target::Scalar,
        };
        "AVDDesignMlagIbgpPeeringVrfs" => dict {
            "base_vlan": 0 => Target::Scalar,
        };
        "AVDDesignMonitorConnectivity" => dict {
            "shutdown": 0 => Target::Scalar,
            "interval": 1 => Target::Scalar,
            "interface_sets": 2 => Target::Model("AVDDesignMonitorConnectivityInterfaceSetsList"),
            "local_interfaces": 3 => Target::Scalar,
            "address_only": 4 => Target::Scalar,
            "hosts": 5 => Target::Model("AVDDesignMonitorConnectivityHostsList"),
            "name_server_group": 6 => Target::Scalar,
            "vrfs": 7 => Target::Model("AVDDesignMonitorConnectivityVrfsList"),
        };
        "AVDDesignMonitorConnectivityInterfaceSetsList" => indexed(Target::Model("AVDDesignMonitorConnectivityInterfaceSetsListIndexedItem"), [0]);
        "AVDDesignMonitorConnectivityInterfaceSetsItems" => dict {
            "name": 0 => Target::Scalar,
            "interfaces": 1 => Target::Model("AVDDesignMonitorConnectivityInterfaceSetsItemsInterfacesList"),
        };
        "AVDDesignMonitorConnectivityInterfaceSetsItemsInterfacesList" => list(Target::Scalar);
        "AVDDesignMonitorConnectivityHostsList" => indexed(Target::Model("AVDDesignMonitorConnectivityHostsListIndexedItem"), [0]);
        "AVDDesignMonitorConnectivityHostsItems" => dict {
            "name": 0 => Target::Scalar,
            "description": 1 => Target::Scalar,
            "single_line_description": 2 => Target::Scalar,
            "ip": 3 => Target::Scalar,
            "icmp_echo_size": 4 => Target::Scalar,
            "local_interfaces": 5 => Target::Scalar,
            "address_only": 6 => Target::Scalar,
            "url": 7 => Target::Scalar,
        };
        "AVDDesignMonitorConnectivityVrfsList" => indexed(Target::Model("AVDDesignMonitorConnectivityVrfsListIndexedItem"), [0]);
        "AVDDesignMonitorConnectivityVrfsItems" => dict {
            "name": 0 => Target::Scalar,
            "description": 1 => Target::Scalar,
            "single_line_description": 2 => Target::Scalar,
            "interface_sets": 3 => Target::Model("AVDDesignMonitorConnectivityVrfsItemsInterfaceSetsList"),
            "local_interfaces": 4 => Target::Scalar,
            "address_only": 5 => Target::Scalar,
            "hosts": 6 => Target::Model("AVDDesignMonitorConnectivityVrfsItemsHostsList"),
        };
        "AVDDesignMonitorConnectivityVrfsItemsInterfaceSetsList" => indexed(Target::Model("AVDDesignMonitorConnectivityVrfsItemsInterfaceSetsListIndexedItem"), [0]);
        "AVDDesignMonitorConnectivityVrfsItemsInterfaceSetsItems" => dict {
            "name": 0 => Target::Scalar,
            "interfaces": 1 => Target::Model("AVDDesignMonitorConnectivityVrfsItemsInterfaceSetsItemsInterfacesList"),
        };
        "AVDDesignMonitorConnectivityVrfsItemsInterfaceSetsItemsInterfacesList" => list(Target::Scalar);
        "AVDDesignMonitorConnectivityVrfsItemsHostsList" => indexed(Target::Model("AVDDesignMonitorConnectivityVrfsItemsHostsListIndexedItem"), [0]);
        "AVDDesignMonitorConnectivityVrfsItemsHostsItems" => dict {
            "name": 0 => Target::Scalar,
            "description": 1 => Target::Scalar,
            "single_line_description": 2 => Target::Scalar,
            "ip": 3 => Target::Scalar,
            "icmp_echo_size": 4 => Target::Scalar,
            "local_interfaces": 5 => Target::Scalar,
            "address_only": 6 => Target::Scalar,
            "url": 7 => Target::Scalar,
        };
        "AVDDesignNetworkPortsList" => list(Target::Model("AVDDesignNetworkPortsItems"));
        "AVDDesignNetworkPortsItems" => dict {
            "switches": 0 => Target::Model("AVDDesignNetworkPortsItemsSwitchesList"),
            "platforms": 1 => Target::Model("AVDDesignNetworkPortsItemsPlatformsList"),
            "switch_ports": 2 => Target::Model("AVDDesignNetworkPortsItemsSwitchPortsList"),
            "description": 3 => Target::Scalar,
            "endpoint": 4 => Target::Scalar,
            "speed": 5 => Target::Scalar,
            "profile": 6 => Target::Scalar,
            "enabled": 7 => Target::Scalar,
            "mode": 8 => Target::Scalar,
            "mtu": 9 => Target::Scalar,
            "l2_mtu": 10 => Target::Scalar,
            "l2_mru": 11 => Target::Scalar,
            "native_vlan": 12 => Target::Scalar,
            "native_vlan_tag": 13 => Target::Scalar,
            "phone_vlan": 14 => Target::Scalar,
            "phone_trunk_mode": 15 => Target::Scalar,
            "trunk_groups": 16 => Target::Model("AVDDesignNetworkPortsItemsTrunkGroupsList"),
            "vlans": 17 => Target::Scalar,
            "mac_acl_in": 18 => Target::Scalar,
            "mac_acl_out": 19 => Target::Scalar,
            "spanning_tree_portfast": 20 => Target::Scalar,
            "spanning_tree_bpdufilter": 21 => Target::Scalar,
            "spanning_tree_bpduguard": 22 => Target::Scalar,
            "spanning_tree_link_type": 23 => Target::Scalar,
            "flowcontrol": 24 => Target::Model("EosCliConfigGenEthernetInterfacesItemsFlowcontrol"),
            "qos_profile": 25 => Target::Scalar,
            "ptp": 26 => Target::Model("AVDDesignNetworkPortsItemsPtp"),
            "sflow": 27 => Target::Scalar,
            "flow_tracking": 28 => Target::Model("AVDDesignNetworkPortsItemsFlowTracking"),
            "link_tracking": 29 => Target::Model("AVDDesignNetworkPortsItemsLinkTracking"),
            "dot1x": 30 => Target::Model("AVDDesignNetworkPortsItemsDot1x"),
            "address_locking": 31 => Target::Model("AVDDesignNetworkPortsItemsAddressLocking"),
            "poe": 32 => Target::Model("EosCliConfigGenEthernetInterfacesItemsPoe"),
            "storm_control": 33 => Target::Model("AVDDesignNetworkPortsItemsStormControl"),
            "monitor_sessions": 34 => Target::Model("AVDDesignNetworkPortsItemsMonitorSessionsList"),
            "ethernet_segment": 35 => Target::Model("AVDDesignNetworkPortsItemsEthernetSegment"),
            "port_channel": 36 => Target::Model("AVDDesignNetworkPortsItemsPortChannel"),
            "validate_state": 37 => Target::Scalar,
            "validate_lldp": 38 => Target::Scalar,
            "campus_link_type": 39 => Target::Model("AVDDesignNetworkPortsItemsCampusLinkTypeList"),
            "raw_eos_cli": 40 => Target::Scalar,
            "structured_config": 41 => Target::Opaque,
        };
        "AVDDesignNetworkPortsItemsSwitchesList" => list(Target::Scalar);
        "AVDDesignNetworkPortsItemsPlatformsList" => list(Target::Scalar);
        "AVDDesignNetworkPortsItemsSwitchPortsList" => list(Target::Scalar);
        "AVDDesignNetworkPortsItemsTrunkGroupsList" => list(Target::Scalar);
        "AVDDesignNetworkPortsItemsPtp" => dict {
            "enabled": 0 => Target::Scalar,
            "endpoint_role": 1 => Target::Scalar,
            "profile": 2 => Target::Scalar,
        };
        "AVDDesignNetworkPortsItemsFlowTracking" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignNetworkPortsItemsLinkTracking" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignNetworkPortsItemsDot1x" => dict {
            "authentication_failure": 0 => Target::Model("AVDDesignNetworkPortsItemsDot1xAuthenticationFailure"),
            "port_control": 1 => Target::Scalar,
            "port_control_force_authorized_phone": 2 => Target::Scalar,
            "reauthentication": 3 => Target::Scalar,
            "pae": 4 => Target::Model("AVDDesignNetworkPortsItemsDot1xPae"),
            "host_mode": 5 => Target::Model("AVDDesignNetworkPortsItemsDot1xHostMode"),
            "mac_based_authentication": 6 => Target::Model("AVDDesignNetworkPortsItemsDot1xMacBasedAuthentication"),
            "mac_based_access_list": 7 => Target::Scalar,
            "timeout": 8 => Target::Model("AVDDesignNetworkPortsItemsDot1xTimeout"),
            "reauthorization_request_limit": 9 => Target::Scalar,
            "unauthorized": 10 => Target::Model("AVDDesignNetworkPortsItemsDot1xUnauthorized"),
            "eapol": 11 => Target::Model("AVDDesignNetworkPortsItemsDot1xEapol"),
            "aaa": 12 => Target::Model("AVDDesignNetworkPortsItemsDot1xAaa"),
        };
        "AVDDesignNetworkPortsItemsDot1xAuthenticationFailure" => dict {
            "allow_access_list": 0 => Target::Scalar,
            "action": 1 => Target::Scalar,
            "allow_vlan": 2 => Target::Scalar,
        };
        "AVDDesignNetworkPortsItemsDot1xPae" => dict {
            "mode": 0 => Target::Scalar,
            "supplicant_profile": 1 => Target::Scalar,
        };
        "AVDDesignNetworkPortsItemsDot1xHostMode" => dict {
            "mode": 0 => Target::Scalar,
            "multi_host_authenticated": 1 => Target::Scalar,
        };
        "AVDDesignNetworkPortsItemsDot1xMacBasedAuthentication" => dict {
            "enabled": 0 => Target::Scalar,
            "always": 1 => Target::Scalar,
            "host_mode_common": 2 => Target::Scalar,
        };
        "AVDDesignNetworkPortsItemsDot1xTimeout" => dict {
            "idle_host": 0 => Target::Scalar,
            "quiet_period": 1 => Target::Scalar,
            "reauth_period": 2 => Target::Scalar,
            "reauth_timeout_ignore": 3 => Target::Scalar,
            "tx_period": 4 => Target::Scalar,
        };
        "AVDDesignNetworkPortsItemsDot1xUnauthorized" => dict {
            "access_vlan_membership_egress": 0 => Target::Scalar,
            "native_vlan_membership_egress": 1 => Target::Scalar,
        };
        "AVDDesignNetworkPortsItemsDot1xEapol" => dict {
            "disabled": 0 => Target::Scalar,
            "authentication_failure_fallback_mba": 1 => Target::Model("AVDDesignNetworkPortsItemsDot1xEapolAuthenticationFailureFallbackMba"),
        };
        "AVDDesignNetworkPortsItemsDot1xEapolAuthenticationFailureFallbackMba" => dict {
            "enabled": 0 => Target::Scalar,
            "timeout": 1 => Target::Scalar,
        };
        "AVDDesignNetworkPortsItemsDot1xAaa" => dict {
            "unresponsive": 0 => Target::Model("AVDDesignNetworkPortsItemsDot1xAaaUnresponsive"),
        };
        "AVDDesignNetworkPortsItemsDot1xAaaUnresponsive" => dict {
            "eap_response": 0 => Target::Scalar,
            "action": 1 => Target::Model("AVDDesignNetworkPortsItemsDot1xAaaUnresponsiveAction"),
            "phone_action": 2 => Target::Model("EosCliConfigGenDot1xAaaUnresponsivePhoneAction"),
        };
        "AVDDesignNetworkPortsItemsDot1xAaaUnresponsiveAction" => dict {
            "traffic_allow_access_list": 0 => Target::Scalar,
            "apply_alternate": 1 => Target::Scalar,
            "traffic_allow_vlan": 2 => Target::Scalar,
            "apply_cached_results": 3 => Target::Scalar,
            "cached_results_timeout": 4 => Target::Model("AVDDesignNetworkPortsItemsDot1xAaaUnresponsiveActionCachedResultsTimeout"),
            "traffic_allow": 5 => Target::Scalar,
        };
        "AVDDesignNetworkPortsItemsDot1xAaaUnresponsiveActionCachedResultsTimeout" => dict {
            "time_duration": 0 => Target::Scalar,
            "time_duration_unit": 1 => Target::Scalar,
        };
        "AVDDesignNetworkPortsItemsAddressLocking" => dict {
            "ipv4": 0 => Target::Scalar,
            "ipv6": 1 => Target::Scalar,
        };
        "AVDDesignNetworkPortsItemsStormControl" => dict {
            "all": 0 => Target::Model("AVDDesignNetworkPortsItemsStormControlAll"),
            "broadcast": 1 => Target::Model("AVDDesignNetworkPortsItemsStormControlBroadcast"),
            "multicast": 2 => Target::Model("AVDDesignNetworkPortsItemsStormControlMulticast"),
            "unknown_unicast": 3 => Target::Model("AVDDesignNetworkPortsItemsStormControlUnknownUnicast"),
        };
        "AVDDesignNetworkPortsItemsStormControlAll" => dict {
            "level": 0 => Target::Scalar,
            "unit": 1 => Target::Scalar,
        };
        "AVDDesignNetworkPortsItemsStormControlBroadcast" => dict {
            "level": 0 => Target::Scalar,
            "unit": 1 => Target::Scalar,
        };
        "AVDDesignNetworkPortsItemsStormControlMulticast" => dict {
            "level": 0 => Target::Scalar,
            "unit": 1 => Target::Scalar,
        };
        "AVDDesignNetworkPortsItemsStormControlUnknownUnicast" => dict {
            "level": 0 => Target::Scalar,
            "unit": 1 => Target::Scalar,
        };
        "AVDDesignNetworkPortsItemsMonitorSessionsList" => list(Target::Model("AVDDesignNetworkPortsItemsMonitorSessionsItems"));
        "AVDDesignNetworkPortsItemsMonitorSessionsItems" => dict {
            "name": 0 => Target::Scalar,
            "role": 1 => Target::Scalar,
            "source_settings": 2 => Target::Model("AVDDesignNetworkPortsItemsMonitorSessionsItemsSourceSettings"),
            "session_settings": 3 => Target::Model("AVDDesignNetworkPortsItemsMonitorSessionsItemsSessionSettings"),
        };
        "AVDDesignNetworkPortsItemsMonitorSessionsItemsSourceSettings" => dict {
            "direction": 0 => Target::Scalar,
            "access_group": 1 => Target::Model("AVDDesignNetworkPortsItemsMonitorSessionsItemsSourceSettingsAccessGroup"),
        };
        "AVDDesignNetworkPortsItemsMonitorSessionsItemsSourceSettingsAccessGroup" => dict {
            "field_type": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
            "priority": 2 => Target::Scalar,
        };
        "AVDDesignNetworkPortsItemsMonitorSessionsItemsSessionSettings" => dict {
            "encapsulation_gre_metadata_tx": 0 => Target::Scalar,
            "header_remove_size": 1 => Target::Scalar,
            "access_group": 2 => Target::Model("AVDDesignNetworkPortsItemsMonitorSessionsItemsSessionSettingsAccessGroup"),
            "rate_limit_per_ingress_chip": 3 => Target::Scalar,
            "rate_limit_per_egress_chip": 4 => Target::Scalar,
            "sample": 5 => Target::Scalar,
            "truncate": 6 => Target::Model("AVDDesignNetworkPortsItemsMonitorSessionsItemsSessionSettingsTruncate"),
        };
        "AVDDesignNetworkPortsItemsMonitorSessionsItemsSessionSettingsAccessGroup" => dict {
            "field_type": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignNetworkPortsItemsMonitorSessionsItemsSessionSettingsTruncate" => dict {
            "enabled": 0 => Target::Scalar,
            "size": 1 => Target::Scalar,
        };
        "AVDDesignNetworkPortsItemsEthernetSegment" => dict {
            "short_esi": 0 => Target::Scalar,
            "redundancy": 1 => Target::Scalar,
            "designated_forwarder_algorithm": 2 => Target::Scalar,
            "designated_forwarder_preferences": 3 => Target::Model("AVDDesignNetworkPortsItemsEthernetSegmentDesignatedForwarderPreferencesList"),
            "dont_preempt": 4 => Target::Scalar,
        };
        "AVDDesignNetworkPortsItemsEthernetSegmentDesignatedForwarderPreferencesList" => list(Target::Scalar);
        "AVDDesignNetworkPortsItemsPortChannel" => dict {
            "mode": 0 => Target::Scalar,
            "channel_id": 1 => Target::Scalar,
            "description": 2 => Target::Scalar,
            "endpoint_port_channel": 3 => Target::Scalar,
            "enabled": 4 => Target::Scalar,
            "ptp_mpass": 5 => Target::Scalar,
            "lacp_fallback": 6 => Target::Model("AVDDesignNetworkPortsItemsPortChannelLacpFallback"),
            "lacp_timer": 7 => Target::Model("AVDDesignNetworkPortsItemsPortChannelLacpTimer"),
            "raw_eos_cli": 8 => Target::Scalar,
            "structured_config": 9 => Target::Opaque,
        };
        "AVDDesignNetworkPortsItemsPortChannelLacpFallback" => dict {
            "mode": 0 => Target::Scalar,
            "individual": 1 => Target::Model("AVDDesignNetworkPortsItemsPortChannelLacpFallbackIndividual"),
            "timeout": 2 => Target::Scalar,
        };
        "AVDDesignNetworkPortsItemsPortChannelLacpFallbackIndividual" => dict {
            "profile": 0 => Target::Scalar,
            "vlans": 1 => Target::Scalar,
            "native_vlan": 2 => Target::Scalar,
            "mode": 3 => Target::Scalar,
        };
        "AVDDesignNetworkPortsItemsPortChannelLacpTimer" => dict {
            "mode": 0 => Target::Scalar,
            "multiplier": 1 => Target::Scalar,
        };
        "AVDDesignNetworkPortsItemsCampusLinkTypeList" => list(Target::Scalar);
        "AVDDesignNetworkServicesList" => indexed(Target::Model("AVDDesignNetworkServicesListIndexedItem"), [0]);
        "AVDDesignNetworkServicesItems" => dict {
            "name": 0 => Target::Scalar,
            "mac_vrf_vni_base": 1 => Target::Scalar,
            "mac_vrf_id_base": 2 => Target::Scalar,
            "vlan_aware_bundle_number_base": 3 => Target::Scalar,
            "pseudowire_rt_base": 4 => Target::Scalar,
            "enable_mlag_ibgp_peering_vrfs": 5 => Target::Scalar,
            "redistribute_mlag_ibgp_peering_vrfs": 6 => Target::Scalar,
            "evpn_vlan_bundle": 7 => Target::Scalar,
            "bgp_peer_groups": 8 => Target::Model("AVDDesignNetworkServicesItemsBgpPeerGroupsList"),
            "igmp_snooping": 9 => Target::Model("AVDDesignNetworkServicesItemsIgmpSnooping"),
            "evpn_l2_multicast": 10 => Target::Model("AVDDesignNetworkServicesItemsEvpnL2Multicast"),
            "vxlan_flood_multicast": 11 => Target::Model("AVDDesignNetworkServicesItemsVxlanFloodMulticast"),
            "evpn_l3_multicast": 12 => Target::Model("AVDDesignNetworkServicesItemsEvpnL3Multicast"),
            "pim_rp_addresses": 13 => Target::Model("AVDDesignNetworkServicesItemsPimRpAddressesList"),
            "igmp_snooping_querier": 14 => Target::Model("AVDDesignNetworkServicesItemsIgmpSnoopingQuerier2"),
            "evpn_l2_multi_domain": 15 => Target::Scalar,
            "vrfs": 16 => Target::Model("AVDDesignNetworkServicesItemsVrfsList"),
            "l2vlans": 17 => Target::Model("AVDDesignNetworkServicesItemsL2vlansList"),
            "vpws": 18 => Target::Model("AVDDesignNetworkServicesItemsVpws"),
            "point_to_point_services": 19 => Target::Model("AVDDesignNetworkServicesItemsPointToPointServicesList"),
        };
        "AVDDesignNetworkServicesItemsBgpPeerGroupsList" => indexed(Target::Model("AVDDesignNetworkServicesItemsBgpPeerGroupsListIndexedItem"), [0]);
        "AVDDesignNetworkServicesItemsBgpPeerGroupsItems" => dict {
            "name": 0 => Target::Scalar,
            "password": 1 => Target::Scalar,
            "cleartext_password": 2 => Target::Scalar,
            "nodes": 3 => Target::Model("AVDDesignNetworkServicesItemsBgpPeerGroupsItemsNodesList"),
            "address_family_ipv4": 4 => Target::Model("AVDDesignNetworkServicesItemsBgpPeerGroupsItemsAddressFamilyIpv4"),
            "address_family_ipv6": 5 => Target::Model("AVDDesignNetworkServicesItemsBgpPeerGroupsItemsAddressFamilyIpv6"),
            "listen_ranges": 6 => Target::Model("AVDDesignNetworkServicesItemsBgpPeerGroupsItemsListenRangesList"),
            "metadata": 7 => Target::Model("AVDDesignNetworkServicesItemsBgpPeerGroupsItemsMetadata"),
            "remote_as": 8 => Target::Scalar,
            "local_as": 9 => Target::Scalar,
            "description": 10 => Target::Scalar,
            "shutdown": 11 => Target::Scalar,
            "as_path": 12 => Target::Model("AVDDesignNetworkServicesItemsBgpPeerGroupsItemsAsPath"),
            "remove_private_as": 13 => Target::Model("AVDDesignNetworkServicesItemsBgpPeerGroupsItemsRemovePrivateAs"),
            "remove_private_as_ingress": 14 => Target::Model("AVDDesignNetworkServicesItemsBgpPeerGroupsItemsRemovePrivateAsIngress"),
            "next_hop_unchanged": 15 => Target::Scalar,
            "update_source": 16 => Target::Scalar,
            "route_reflector_client": 17 => Target::Scalar,
            "bfd": 18 => Target::Scalar,
            "bfd_timers": 19 => Target::Model("AVDDesignNetworkServicesItemsBgpPeerGroupsItemsBfdTimers"),
            "ebgp_multihop": 20 => Target::Scalar,
            "next_hop_peer": 21 => Target::Scalar,
            "next_hop_self": 22 => Target::Scalar,
            "password_type": 23 => Target::Scalar,
            "passive": 24 => Target::Scalar,
            "default_originate": 25 => Target::Model("AVDDesignNetworkServicesItemsBgpPeerGroupsItemsDefaultOriginate"),
            "enforce_first_as": 26 => Target::Scalar,
            "send_community": 27 => Target::Scalar,
            "maximum_routes": 28 => Target::Scalar,
            "maximum_routes_warning_limit": 29 => Target::Scalar,
            "maximum_routes_warning_only": 30 => Target::Scalar,
            "maximum_accepted_routes": 31 => Target::Model("AVDDesignNetworkServicesItemsBgpPeerGroupsItemsMaximumAcceptedRoutes"),
            "missing_policy": 32 => Target::Model("AVDDesignNetworkServicesItemsBgpPeerGroupsItemsMissingPolicy"),
            "link_bandwidth": 33 => Target::Model("AVDDesignNetworkServicesItemsBgpPeerGroupsItemsLinkBandwidth"),
            "allowas_in": 34 => Target::Model("AVDDesignNetworkServicesItemsBgpPeerGroupsItemsAllowasIn"),
            "weight": 35 => Target::Scalar,
            "timers": 36 => Target::Scalar,
            "rib_in_pre_policy_retain": 37 => Target::Model("AVDDesignNetworkServicesItemsBgpPeerGroupsItemsRibInPrePolicyRetain"),
            "route_map_in": 38 => Target::Scalar,
            "route_map_out": 39 => Target::Scalar,
            "peer_tag_in": 40 => Target::Scalar,
            "peer_tag_out_discard": 41 => Target::Scalar,
            "session_tracker": 42 => Target::Scalar,
            "shared_secret": 43 => Target::Model("AVDDesignNetworkServicesItemsBgpPeerGroupsItemsSharedSecret"),
            "ttl_maximum_hops": 44 => Target::Scalar,
            "maximum_advertised_routes": 45 => Target::Scalar,
            "maximum_advertised_routes_warning_limit": 46 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsNodesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsAddressFamilyIpv4" => dict {
            "activate": 0 => Target::Scalar,
            "route_map_in": 1 => Target::Scalar,
            "route_map_out": 2 => Target::Scalar,
            "rcf_in": 3 => Target::Scalar,
            "rcf_out": 4 => Target::Scalar,
            "default_originate": 5 => Target::Model("EosCliConfigGenRouterBgpAddressFamilyIpv4PeerGroupsItemsDefaultOriginate"),
            "next_hop": 6 => Target::Model("EosCliConfigGenRouterBgpAddressFamilyIpv4PeerGroupsItemsNextHop"),
            "prefix_list_in": 7 => Target::Scalar,
            "prefix_list_out": 8 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsAddressFamilyIpv6" => dict {
            "activate": 0 => Target::Scalar,
            "route_map_in": 1 => Target::Scalar,
            "route_map_out": 2 => Target::Scalar,
            "rcf_in": 3 => Target::Scalar,
            "rcf_out": 4 => Target::Scalar,
            "default_originate": 5 => Target::Model("AVDDesignNetworkServicesItemsBgpPeerGroupsItemsAddressFamilyIpv6DefaultOriginate"),
            "prefix_list_in": 6 => Target::Scalar,
            "prefix_list_out": 7 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsAddressFamilyIpv6DefaultOriginate" => dict {
            "enabled": 0 => Target::Scalar,
            "always": 1 => Target::Scalar,
            "route_map": 2 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsListenRangesList" => list(Target::Model("AVDDesignNetworkServicesItemsBgpPeerGroupsItemsListenRangesItems"));
        "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsListenRangesItems" => dict {
            "prefix": 0 => Target::Scalar,
            "remote_as": 1 => Target::Scalar,
            "peer_id_include_router_id": 2 => Target::Scalar,
            "peer_filter": 3 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsMetadata" => dict {
            "field_type": 0 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsAsPath" => dict {
            "remote_as_replace_out": 0 => Target::Scalar,
            "prepend_own_disabled": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsRemovePrivateAs" => dict {
            "enabled": 0 => Target::Scalar,
            "all": 1 => Target::Scalar,
            "replace_as": 2 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsRemovePrivateAsIngress" => dict {
            "enabled": 0 => Target::Scalar,
            "replace_as": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsBfdTimers" => dict {
            "interval": 0 => Target::Scalar,
            "min_rx": 1 => Target::Scalar,
            "multiplier": 2 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsDefaultOriginate" => dict {
            "enabled": 0 => Target::Scalar,
            "always": 1 => Target::Scalar,
            "route_map": 2 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsMaximumAcceptedRoutes" => dict {
            "limit": 0 => Target::Scalar,
            "warning_limit": 1 => Target::Model("AVDDesignNetworkServicesItemsBgpPeerGroupsItemsMaximumAcceptedRoutesWarningLimit"),
        };
        "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsMaximumAcceptedRoutesWarningLimit" => dict {
            "count": 0 => Target::Scalar,
            "percent": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsMissingPolicy" => dict {
            "direction_in": 0 => Target::Model("AVDDesignNetworkServicesItemsBgpPeerGroupsItemsMissingPolicyDirectionIn"),
            "direction_out": 1 => Target::Model("AVDDesignNetworkServicesItemsBgpPeerGroupsItemsMissingPolicyDirectionOut"),
        };
        "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsMissingPolicyDirectionIn" => dict {
            "action": 0 => Target::Scalar,
            "include_community_list": 1 => Target::Scalar,
            "include_prefix_list": 2 => Target::Scalar,
            "include_sub_route_map": 3 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsMissingPolicyDirectionOut" => dict {
            "action": 0 => Target::Scalar,
            "include_community_list": 1 => Target::Scalar,
            "include_prefix_list": 2 => Target::Scalar,
            "include_sub_route_map": 3 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsLinkBandwidth" => dict {
            "enabled": 0 => Target::Scalar,
            "default": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsAllowasIn" => dict {
            "enabled": 0 => Target::Scalar,
            "times": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsRibInPrePolicyRetain" => dict {
            "enabled": 0 => Target::Scalar,
            "all": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsBgpPeerGroupsItemsSharedSecret" => dict {
            "profile": 0 => Target::Scalar,
            "hash_algorithm": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsIgmpSnooping" => dict {
            "querier": 0 => Target::Model("AVDDesignNetworkServicesItemsIgmpSnoopingQuerier"),
            "fast_leave": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsIgmpSnoopingQuerier" => dict {
            "enabled": 0 => Target::Scalar,
            "source_address": 1 => Target::Scalar,
            "version": 2 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsEvpnL2Multicast" => dict {
            "enabled": 0 => Target::Scalar,
            "underlay_l2_multicast_group_ipv4_pool": 1 => Target::Scalar,
            "underlay_l2_multicast_group_ipv4_pool_offset": 2 => Target::Scalar,
            "fast_leave": 3 => Target::Scalar,
            "always_redistribute_igmp": 4 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVxlanFloodMulticast" => dict {
            "enabled": 0 => Target::Scalar,
            "underlay_l2_multicast_group_ipv4_pool": 1 => Target::Scalar,
            "underlay_l2_multicast_group_ipv4_pool_offset": 2 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsEvpnL3Multicast" => dict {
            "enabled": 0 => Target::Scalar,
            "evpn_underlay_l3_multicast_group_ipv4_pool": 1 => Target::Scalar,
            "evpn_underlay_l3_multicast_group_ipv4_pool_offset": 2 => Target::Scalar,
            "evpn_peg": 3 => Target::Model("AVDDesignNetworkServicesItemsEvpnL3MulticastEvpnPegList"),
        };
        "AVDDesignNetworkServicesItemsEvpnL3MulticastEvpnPegList" => list(Target::Model("AVDDesignNetworkServicesItemsEvpnL3MulticastEvpnPegItems"));
        "AVDDesignNetworkServicesItemsEvpnL3MulticastEvpnPegItems" => dict {
            "nodes": 0 => Target::Model("AVDDesignNetworkServicesItemsEvpnL3MulticastEvpnPegItemsNodesList"),
            "transit": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsEvpnL3MulticastEvpnPegItemsNodesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsPimRpAddressesList" => list(Target::Model("AVDDesignNetworkServicesItemsPimRpAddressesItems"));
        "AVDDesignNetworkServicesItemsPimRpAddressesItems" => dict {
            "rps": 0 => Target::Model("AVDDesignNetworkServicesItemsPimRpAddressesItemsRpsList"),
            "nodes": 1 => Target::Model("AVDDesignNetworkServicesItemsPimRpAddressesItemsNodesList"),
            "groups": 2 => Target::Model("AVDDesignNetworkServicesItemsPimRpAddressesItemsGroupsList"),
            "access_list_name": 3 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsPimRpAddressesItemsRpsList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsPimRpAddressesItemsNodesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsPimRpAddressesItemsGroupsList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsIgmpSnoopingQuerier2" => dict {
            "enabled": 0 => Target::Scalar,
            "source_address": 1 => Target::Scalar,
            "version": 2 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsList" => indexed(Target::Model("AVDDesignNetworkServicesItemsVrfsListIndexedItem"), [0]);
        "AVDDesignNetworkServicesItemsVrfsItems" => dict {
            "name": 0 => Target::Scalar,
            "address_families": 1 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsAddressFamiliesList"),
            "description": 2 => Target::Scalar,
            "vrf_vni": 3 => Target::Scalar,
            "vrf_id": 4 => Target::Scalar,
            "rd_override": 5 => Target::Scalar,
            "rt_override": 6 => Target::Scalar,
            "rt_import": 7 => Target::Scalar,
            "rt_export": 8 => Target::Scalar,
            "rt_import_evpn_remote": 9 => Target::Scalar,
            "rt_export_evpn_remote": 10 => Target::Scalar,
            "evpn_vlan_bundle": 11 => Target::Scalar,
            "mlag_ibgp_peering_ipv4_pool": 12 => Target::Scalar,
            "mlag_ibgp_peering_ipv6_pool": 13 => Target::Scalar,
            "ip_helpers": 14 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsIpHelpersList"),
            "enable_mlag_ibgp_peering_vrfs": 15 => Target::Scalar,
            "redistribute_mlag_ibgp_peering_vrfs": 16 => Target::Scalar,
            "mlag_ibgp_peering_vlan": 17 => Target::Scalar,
            "vtep_diagnostic": 18 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsVtepDiagnostic"),
            "ospf": 19 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsOspf"),
            "redistribute_ospf": 20 => Target::Scalar,
            "evpn_l3_multicast": 21 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsEvpnL3Multicast"),
            "pim_rp_addresses": 22 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsPimRpAddressesList"),
            "evpn_l2_multi_domain": 23 => Target::Scalar,
            "svis": 24 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisList"),
            "l3_interfaces": 25 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesList"),
            "l3_port_channels": 26 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsList"),
            "loopbacks": 27 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsLoopbacksList"),
            "static_routes": 28 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsStaticRoutesList"),
            "ipv6_static_routes": 29 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsIpv6StaticRoutesList"),
            "redistribute_static": 30 => Target::Scalar,
            "redistribute_connected": 31 => Target::Scalar,
            "static_arp_entries": 32 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsStaticArpEntriesList"),
            "bgp_peers": 33 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeersList"),
            "bgp": 34 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgp"),
            "bgp_peer_groups": 35 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsList"),
            "additional_route_targets": 36 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsAdditionalRouteTargetsList"),
            "aggregate_addresses": 37 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsAggregateAddressesList"),
            "validate_bgp_peers": 38 => Target::Scalar,
            "raw_eos_cli": 39 => Target::Scalar,
            "structured_config": 40 => Target::Opaque,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsAddressFamiliesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsIpHelpersList" => indexed(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsIpHelpersListIndexedItem"), [0]);
        "AVDDesignNetworkServicesItemsVrfsItemsIpHelpersItems" => dict {
            "ip_helper": 0 => Target::Scalar,
            "source_interface": 1 => Target::Scalar,
            "source_vrf": 2 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsVtepDiagnostic" => dict {
            "loopback": 0 => Target::Scalar,
            "loopback_description": 1 => Target::Scalar,
            "loopback_ip_range": 2 => Target::Scalar,
            "loopback_ipv6_range": 3 => Target::Scalar,
            "loopback_ip_pools": 4 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsVtepDiagnosticLoopbackIpPoolsList"),
            "hardware_forwarding": 5 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsVtepDiagnosticLoopbackIpPoolsList" => indexed(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsVtepDiagnosticLoopbackIpPoolsListIndexedItem"), [0]);
        "AVDDesignNetworkServicesItemsVrfsItemsVtepDiagnosticLoopbackIpPoolsItems" => dict {
            "pod": 0 => Target::Scalar,
            "ipv4_pool": 1 => Target::Scalar,
            "ipv6_pool": 2 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsOspf" => dict {
            "enabled": 0 => Target::Scalar,
            "process_id": 1 => Target::Scalar,
            "router_id": 2 => Target::Scalar,
            "max_lsa": 3 => Target::Scalar,
            "bfd": 4 => Target::Scalar,
            "redistribute_bgp": 5 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsOspfRedistributeBgp"),
            "redistribute_connected": 6 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsOspfRedistributeConnected"),
            "authentication": 7 => Target::Scalar,
            "cleartext_simple_auth_key": 8 => Target::Scalar,
            "message_digest_keys": 9 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsOspfMessageDigestKeysList"),
            "nodes": 10 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsOspfNodesList"),
            "structured_config": 11 => Target::Opaque,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsOspfRedistributeBgp" => dict {
            "enabled": 0 => Target::Scalar,
            "route_map": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsOspfRedistributeConnected" => dict {
            "enabled": 0 => Target::Scalar,
            "route_map": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsOspfMessageDigestKeysList" => indexed(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsOspfMessageDigestKeysListIndexedItem"), [0]);
        "AVDDesignNetworkServicesItemsVrfsItemsOspfMessageDigestKeysItems" => dict {
            "id": 0 => Target::Scalar,
            "hash_algorithm": 1 => Target::Scalar,
            "cleartext_key": 2 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsOspfNodesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsEvpnL3Multicast" => dict {
            "enabled": 0 => Target::Scalar,
            "evpn_underlay_l3_multicast_group": 1 => Target::Scalar,
            "evpn_peg": 2 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsEvpnL3MulticastEvpnPegList"),
        };
        "AVDDesignNetworkServicesItemsVrfsItemsEvpnL3MulticastEvpnPegList" => list(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsEvpnL3MulticastEvpnPegItems"));
        "AVDDesignNetworkServicesItemsVrfsItemsEvpnL3MulticastEvpnPegItems" => dict {
            "nodes": 0 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsEvpnL3MulticastEvpnPegItemsNodesList"),
            "transit": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsEvpnL3MulticastEvpnPegItemsNodesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsPimRpAddressesList" => list(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsPimRpAddressesItems"));
        "AVDDesignNetworkServicesItemsVrfsItemsPimRpAddressesItems" => dict {
            "rps": 0 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsPimRpAddressesItemsRpsList"),
            "nodes": 1 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsPimRpAddressesItemsNodesList"),
            "groups": 2 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsPimRpAddressesItemsGroupsList"),
            "access_list_name": 3 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsPimRpAddressesItemsRpsList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsPimRpAddressesItemsNodesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsPimRpAddressesItemsGroupsList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsSvisList" => list(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisListKeyedItem"));
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItems" => dict {
            "id": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
            "address_locking": 2 => Target::Model("EosCliConfigGenVlansItemsAddressLockingAddressFamily"),
            "profile": 3 => Target::Scalar,
            "tags": 4 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsTagsList"),
            "evpn_vlan_bundle": 5 => Target::Scalar,
            "nodes": 6 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesList"),
            "enabled": 7 => Target::Scalar,
            "autostate": 8 => Target::Scalar,
            "description": 9 => Target::Scalar,
            "arp_gratuitous_accept": 10 => Target::Scalar,
            "ip_address": 11 => Target::Scalar,
            "ip_address_secondaries": 12 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpAddressSecondariesList"),
            "ipv6_address": 13 => Target::Scalar,
            "ipv6_enable": 14 => Target::Scalar,
            "ip_address_virtual": 15 => Target::Scalar,
            "ipv6_address_virtuals": 16 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6AddressVirtualsList"),
            "ipv6_nd": 17 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6Nd"),
            "ipv6_dhcp_relay": 18 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6DhcpRelay"),
            "ip_address_virtual_secondaries": 19 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpAddressVirtualSecondariesList"),
            "ip_virtual_router_addresses": 20 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpVirtualRouterAddressesList"),
            "ipv6_virtual_router_addresses": 21 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6VirtualRouterAddressesList"),
            "ipv4_acl_in": 22 => Target::Scalar,
            "ipv4_acl_out": 23 => Target::Scalar,
            "ipv6_acl_in": 24 => Target::Scalar,
            "ipv6_acl_out": 25 => Target::Scalar,
            "ip_helpers": 26 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpHelpersList"),
            "static_routes": 27 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsStaticRoutesList"),
            "ipv6_static_routes": 28 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6StaticRoutesList"),
            "vni_override": 29 => Target::Scalar,
            "rt_override": 30 => Target::Scalar,
            "rd_override": 31 => Target::Scalar,
            "trunk_groups": 32 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsTrunkGroupsList"),
            "evpn_l2_multicast": 33 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsEvpnL2Multicast"),
            "evpn_redistribute_router_mac_system": 34 => Target::Scalar,
            "vxlan_flood_multicast": 35 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsVxlanFloodMulticast"),
            "evpn_l3_multicast": 36 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsEvpnL3Multicast"),
            "igmp_snooping": 37 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIgmpSnooping"),
            "igmp_snooping_enabled": 38 => Target::Scalar,
            "igmp_snooping_querier": 39 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIgmpSnoopingQuerier2"),
            "vxlan": 40 => Target::Scalar,
            "spanning_tree_priority": 41 => Target::Scalar,
            "mtu": 42 => Target::Scalar,
            "ospf": 43 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsOspf"),
            "bgp": 44 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsBgp"),
            "raw_eos_cli": 45 => Target::Scalar,
            "structured_config": 46 => Target::Opaque,
            "evpn_l2_multi_domain": 47 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsTagsList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesList" => indexed(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesListIndexedItem"), [0]);
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItems" => dict {
            "node": 0 => Target::Scalar,
            "tags": 1 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsTagsList"),
            "name": 2 => Target::Scalar,
            "enabled": 3 => Target::Scalar,
            "autostate": 4 => Target::Scalar,
            "description": 5 => Target::Scalar,
            "arp_gratuitous_accept": 6 => Target::Scalar,
            "ip_address": 7 => Target::Scalar,
            "ip_address_secondaries": 8 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpAddressSecondariesList"),
            "ipv6_address": 9 => Target::Scalar,
            "ipv6_enable": 10 => Target::Scalar,
            "ip_address_virtual": 11 => Target::Scalar,
            "ipv6_address_virtuals": 12 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6AddressVirtualsList"),
            "ipv6_nd": 13 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6Nd"),
            "ipv6_dhcp_relay": 14 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6DhcpRelay"),
            "ip_address_virtual_secondaries": 15 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpAddressVirtualSecondariesList"),
            "ip_virtual_router_addresses": 16 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpVirtualRouterAddressesList"),
            "ipv6_virtual_router_addresses": 17 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6VirtualRouterAddressesList"),
            "ipv4_acl_in": 18 => Target::Scalar,
            "ipv4_acl_out": 19 => Target::Scalar,
            "ipv6_acl_in": 20 => Target::Scalar,
            "ipv6_acl_out": 21 => Target::Scalar,
            "ip_helpers": 22 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpHelpersList"),
            "static_routes": 23 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsStaticRoutesList"),
            "ipv6_static_routes": 24 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6StaticRoutesList"),
            "vni_override": 25 => Target::Scalar,
            "rt_override": 26 => Target::Scalar,
            "rd_override": 27 => Target::Scalar,
            "trunk_groups": 28 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsTrunkGroupsList"),
            "evpn_l2_multicast": 29 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsEvpnL2Multicast"),
            "evpn_redistribute_router_mac_system": 30 => Target::Scalar,
            "vxlan_flood_multicast": 31 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsVxlanFloodMulticast"),
            "evpn_l3_multicast": 32 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsEvpnL3Multicast"),
            "igmp_snooping": 33 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIgmpSnooping"),
            "igmp_snooping_enabled": 34 => Target::Scalar,
            "igmp_snooping_querier": 35 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIgmpSnoopingQuerier2"),
            "vxlan": 36 => Target::Scalar,
            "spanning_tree_priority": 37 => Target::Scalar,
            "mtu": 38 => Target::Scalar,
            "ospf": 39 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsOspf"),
            "bgp": 40 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsBgp"),
            "raw_eos_cli": 41 => Target::Scalar,
            "structured_config": 42 => Target::Opaque,
            "evpn_l2_multi_domain": 43 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsTagsList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpAddressSecondariesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6AddressVirtualsList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6Nd" => dict {
            "advertise_ipv6_address_virtuals": 0 => Target::Scalar,
            "valid_lifetime": 1 => Target::Scalar,
            "preferred_lifetime": 2 => Target::Scalar,
            "ra_dns_servers": 3 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6NdRaDnsServers"),
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6NdRaDnsServers" => dict {
            "servers": 0 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6NdRaDnsServersServersList"),
            "dns_servers_lifetime": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6NdRaDnsServersServersList" => indexed(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6NdRaDnsServersServersListIndexedItem"), [0]);
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6NdRaDnsServersServersItems" => dict {
            "address": 0 => Target::Scalar,
            "lifetime": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6DhcpRelay" => dict {
            "destinations": 0 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6DhcpRelayDestinationsList"),
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6DhcpRelayDestinationsList" => indexed(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6DhcpRelayDestinationsListIndexedItem"), [0]);
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6DhcpRelayDestinationsItems" => dict {
            "address": 0 => Target::Scalar,
            "vrf": 1 => Target::Scalar,
            "local_interface": 2 => Target::Scalar,
            "source_address": 3 => Target::Scalar,
            "link_address": 4 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpAddressVirtualSecondariesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpVirtualRouterAddressesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6VirtualRouterAddressesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpHelpersList" => indexed(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpHelpersListIndexedItem"), [0]);
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpHelpersItems" => dict {
            "ip_helper": 0 => Target::Scalar,
            "source_interface": 1 => Target::Scalar,
            "source_vrf": 2 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsStaticRoutesList" => list(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsStaticRoutesItems"));
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsStaticRoutesItems" => dict {
            "prefix": 0 => Target::Scalar,
            "next_hop": 1 => Target::Scalar,
            "track_bfd": 2 => Target::Scalar,
            "distance": 3 => Target::Scalar,
            "tag": 4 => Target::Scalar,
            "name": 5 => Target::Scalar,
            "metric": 6 => Target::Scalar,
            "interface": 7 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6StaticRoutesList" => list(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6StaticRoutesItems"));
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6StaticRoutesItems" => dict {
            "prefix": 0 => Target::Scalar,
            "next_hop": 1 => Target::Scalar,
            "track_bfd": 2 => Target::Scalar,
            "distance": 3 => Target::Scalar,
            "tag": 4 => Target::Scalar,
            "name": 5 => Target::Scalar,
            "metric": 6 => Target::Scalar,
            "interface": 7 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsTrunkGroupsList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsEvpnL2Multicast" => dict {
            "enabled": 0 => Target::Scalar,
            "always_redistribute_igmp": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsVxlanFloodMulticast" => dict {
            "enabled": 0 => Target::Scalar,
            "underlay_multicast_group": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsEvpnL3Multicast" => dict {
            "enabled": 0 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIgmpSnooping" => dict {
            "enabled": 0 => Target::Scalar,
            "querier": 1 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIgmpSnoopingQuerier"),
            "fast_leave": 2 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIgmpSnoopingQuerier" => dict {
            "enabled": 0 => Target::Scalar,
            "source_address": 1 => Target::Scalar,
            "version": 2 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIgmpSnoopingQuerier2" => dict {
            "enabled": 0 => Target::Scalar,
            "source_address": 1 => Target::Scalar,
            "version": 2 => Target::Scalar,
            "fast_leave": 3 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsOspf" => dict {
            "enabled": 0 => Target::Scalar,
            "point_to_point": 1 => Target::Scalar,
            "area": 2 => Target::Scalar,
            "cost": 3 => Target::Scalar,
            "authentication": 4 => Target::Scalar,
            "simple_auth_key": 5 => Target::Scalar,
            "cleartext_simple_auth_key": 6 => Target::Scalar,
            "message_digest_keys": 7 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsOspfMessageDigestKeysList"),
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsOspfMessageDigestKeysList" => indexed(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsOspfMessageDigestKeysListIndexedItem"), [0]);
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsOspfMessageDigestKeysItems" => dict {
            "id": 0 => Target::Scalar,
            "hash_algorithm": 1 => Target::Scalar,
            "key": 2 => Target::Scalar,
            "cleartext_key": 3 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsBgp" => dict {
            "structured_config": 0 => Target::Opaque,
            "raw_eos_cli": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpAddressSecondariesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6AddressVirtualsList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6Nd" => dict {
            "advertise_ipv6_address_virtuals": 0 => Target::Scalar,
            "valid_lifetime": 1 => Target::Scalar,
            "preferred_lifetime": 2 => Target::Scalar,
            "ra_dns_servers": 3 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6NdRaDnsServers"),
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6NdRaDnsServers" => dict {
            "servers": 0 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6NdRaDnsServersServersList"),
            "dns_servers_lifetime": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6NdRaDnsServersServersList" => indexed(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6NdRaDnsServersServersListIndexedItem"), [0]);
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6NdRaDnsServersServersItems" => dict {
            "address": 0 => Target::Scalar,
            "lifetime": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6DhcpRelay" => dict {
            "destinations": 0 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6DhcpRelayDestinationsList"),
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6DhcpRelayDestinationsList" => indexed(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6DhcpRelayDestinationsListIndexedItem"), [0]);
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6DhcpRelayDestinationsItems" => dict {
            "address": 0 => Target::Scalar,
            "vrf": 1 => Target::Scalar,
            "local_interface": 2 => Target::Scalar,
            "source_address": 3 => Target::Scalar,
            "link_address": 4 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpAddressVirtualSecondariesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpVirtualRouterAddressesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6VirtualRouterAddressesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpHelpersList" => indexed(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpHelpersListIndexedItem"), [0]);
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpHelpersItems" => dict {
            "ip_helper": 0 => Target::Scalar,
            "source_interface": 1 => Target::Scalar,
            "source_vrf": 2 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsStaticRoutesList" => list(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsStaticRoutesItems"));
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsStaticRoutesItems" => dict {
            "prefix": 0 => Target::Scalar,
            "next_hop": 1 => Target::Scalar,
            "track_bfd": 2 => Target::Scalar,
            "distance": 3 => Target::Scalar,
            "tag": 4 => Target::Scalar,
            "name": 5 => Target::Scalar,
            "metric": 6 => Target::Scalar,
            "interface": 7 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6StaticRoutesList" => list(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6StaticRoutesItems"));
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6StaticRoutesItems" => dict {
            "prefix": 0 => Target::Scalar,
            "next_hop": 1 => Target::Scalar,
            "track_bfd": 2 => Target::Scalar,
            "distance": 3 => Target::Scalar,
            "tag": 4 => Target::Scalar,
            "name": 5 => Target::Scalar,
            "metric": 6 => Target::Scalar,
            "interface": 7 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsTrunkGroupsList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsEvpnL2Multicast" => dict {
            "enabled": 0 => Target::Scalar,
            "always_redistribute_igmp": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsVxlanFloodMulticast" => dict {
            "enabled": 0 => Target::Scalar,
            "underlay_multicast_group": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsEvpnL3Multicast" => dict {
            "enabled": 0 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIgmpSnooping" => dict {
            "enabled": 0 => Target::Scalar,
            "querier": 1 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIgmpSnoopingQuerier"),
            "fast_leave": 2 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIgmpSnoopingQuerier" => dict {
            "enabled": 0 => Target::Scalar,
            "source_address": 1 => Target::Scalar,
            "version": 2 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIgmpSnoopingQuerier2" => dict {
            "enabled": 0 => Target::Scalar,
            "source_address": 1 => Target::Scalar,
            "version": 2 => Target::Scalar,
            "fast_leave": 3 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsOspf" => dict {
            "enabled": 0 => Target::Scalar,
            "point_to_point": 1 => Target::Scalar,
            "area": 2 => Target::Scalar,
            "cost": 3 => Target::Scalar,
            "authentication": 4 => Target::Scalar,
            "simple_auth_key": 5 => Target::Scalar,
            "cleartext_simple_auth_key": 6 => Target::Scalar,
            "message_digest_keys": 7 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsOspfMessageDigestKeysList"),
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsOspfMessageDigestKeysList" => indexed(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsOspfMessageDigestKeysListIndexedItem"), [0]);
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsOspfMessageDigestKeysItems" => dict {
            "id": 0 => Target::Scalar,
            "hash_algorithm": 1 => Target::Scalar,
            "key": 2 => Target::Scalar,
            "cleartext_key": 3 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsBgp" => dict {
            "structured_config": 0 => Target::Opaque,
            "raw_eos_cli": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesList" => list(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItems"));
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItems" => dict {
            "interfaces": 0 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsInterfacesList"),
            "encapsulation_dot1q_vlan": 1 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsEncapsulationDot1qVlanList"),
            "ip_addresses": 2 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsIpAddressesList"),
            "ipv6_addresses": 3 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsIpv6AddressesList"),
            "static_routes": 4 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsStaticRoutesList"),
            "ipv6_static_routes": 5 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsIpv6StaticRoutesList"),
            "nodes": 6 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsNodesList"),
            "arp_gratuitous_accept": 7 => Target::Scalar,
            "description": 8 => Target::Scalar,
            "descriptions": 9 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsDescriptionsList"),
            "enabled": 10 => Target::Scalar,
            "mtu": 11 => Target::Scalar,
            "ipv4_acl_in": 12 => Target::Scalar,
            "ipv4_acl_out": 13 => Target::Scalar,
            "ipv6_acl_in": 14 => Target::Scalar,
            "ipv6_acl_out": 15 => Target::Scalar,
            "ospf": 16 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsOspf"),
            "pim": 17 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsPim"),
            "flow_tracking": 18 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsFlowTracking"),
            "sflow": 19 => Target::Scalar,
            "monitor_sessions": 20 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsMonitorSessionsList"),
            "campus_link_type": 21 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsCampusLinkTypeList"),
            "structured_config": 22 => Target::Opaque,
            "raw_eos_cli": 23 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsInterfacesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsEncapsulationDot1qVlanList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsIpAddressesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsIpv6AddressesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsStaticRoutesList" => list(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsStaticRoutesItems"));
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsStaticRoutesItems" => dict {
            "prefix": 0 => Target::Scalar,
            "next_hop": 1 => Target::Scalar,
            "track_bfd": 2 => Target::Scalar,
            "distance": 3 => Target::Scalar,
            "tag": 4 => Target::Scalar,
            "name": 5 => Target::Scalar,
            "metric": 6 => Target::Scalar,
            "interface": 7 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsIpv6StaticRoutesList" => list(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsIpv6StaticRoutesItems"));
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsIpv6StaticRoutesItems" => dict {
            "prefix": 0 => Target::Scalar,
            "next_hop": 1 => Target::Scalar,
            "track_bfd": 2 => Target::Scalar,
            "distance": 3 => Target::Scalar,
            "tag": 4 => Target::Scalar,
            "name": 5 => Target::Scalar,
            "metric": 6 => Target::Scalar,
            "interface": 7 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsNodesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsDescriptionsList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsOspf" => dict {
            "enabled": 0 => Target::Scalar,
            "point_to_point": 1 => Target::Scalar,
            "area": 2 => Target::Scalar,
            "cost": 3 => Target::Scalar,
            "authentication": 4 => Target::Scalar,
            "simple_auth_key": 5 => Target::Scalar,
            "cleartext_simple_auth_key": 6 => Target::Scalar,
            "message_digest_keys": 7 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsOspfMessageDigestKeysList"),
        };
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsOspfMessageDigestKeysList" => indexed(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsOspfMessageDigestKeysListIndexedItem"), [0]);
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsOspfMessageDigestKeysItems" => dict {
            "id": 0 => Target::Scalar,
            "hash_algorithm": 1 => Target::Scalar,
            "key": 2 => Target::Scalar,
            "cleartext_key": 3 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsPim" => dict {
            "enabled": 0 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsFlowTracking" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsMonitorSessionsList" => list(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsMonitorSessionsItems"));
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsMonitorSessionsItems" => dict {
            "name": 0 => Target::Scalar,
            "role": 1 => Target::Scalar,
            "source_settings": 2 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsMonitorSessionsItemsSourceSettings"),
            "session_settings": 3 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsMonitorSessionsItemsSessionSettings"),
        };
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsMonitorSessionsItemsSourceSettings" => dict {
            "direction": 0 => Target::Scalar,
            "access_group": 1 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsMonitorSessionsItemsSourceSettingsAccessGroup"),
        };
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsMonitorSessionsItemsSourceSettingsAccessGroup" => dict {
            "field_type": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
            "priority": 2 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsMonitorSessionsItemsSessionSettings" => dict {
            "encapsulation_gre_metadata_tx": 0 => Target::Scalar,
            "header_remove_size": 1 => Target::Scalar,
            "access_group": 2 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsMonitorSessionsItemsSessionSettingsAccessGroup"),
            "rate_limit_per_ingress_chip": 3 => Target::Scalar,
            "rate_limit_per_egress_chip": 4 => Target::Scalar,
            "sample": 5 => Target::Scalar,
            "truncate": 6 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsMonitorSessionsItemsSessionSettingsTruncate"),
        };
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsMonitorSessionsItemsSessionSettingsAccessGroup" => dict {
            "field_type": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsMonitorSessionsItemsSessionSettingsTruncate" => dict {
            "enabled": 0 => Target::Scalar,
            "size": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsCampusLinkTypeList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsList" => list(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItems"));
        "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItems" => dict {
            "name": 0 => Target::Scalar,
            "node": 1 => Target::Scalar,
            "arp_gratuitous_accept": 2 => Target::Scalar,
            "description": 3 => Target::Scalar,
            "mode": 4 => Target::Scalar,
            "member_interfaces": 5 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsMemberInterfacesList"),
            "ip_address": 6 => Target::Scalar,
            "ip_address_secondaries": 7 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsIpAddressSecondariesList"),
            "ipv6_addresses": 8 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsIpv6AddressesList"),
            "encapsulation_dot1q_vlan": 9 => Target::Scalar,
            "enabled": 10 => Target::Scalar,
            "peer": 11 => Target::Scalar,
            "peer_port_channel": 12 => Target::Scalar,
            "mtu": 13 => Target::Scalar,
            "ipv4_acl_in": 14 => Target::Scalar,
            "ipv4_acl_out": 15 => Target::Scalar,
            "ipv6_acl_in": 16 => Target::Scalar,
            "ipv6_acl_out": 17 => Target::Scalar,
            "static_routes": 18 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsStaticRoutesList"),
            "ipv6_static_routes": 19 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsIpv6StaticRoutesList"),
            "ospf": 20 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsOspf"),
            "flow_tracking": 21 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsFlowTracking"),
            "structured_config": 22 => Target::Opaque,
            "raw_eos_cli": 23 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsMemberInterfacesList" => indexed(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsMemberInterfacesListIndexedItem"), [0]);
        "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsMemberInterfacesItems" => dict {
            "name": 0 => Target::Scalar,
            "description": 1 => Target::Scalar,
            "peer": 2 => Target::Scalar,
            "peer_interface": 3 => Target::Scalar,
            "speed": 4 => Target::Scalar,
            "structured_config": 5 => Target::Opaque,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsIpAddressSecondariesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsIpv6AddressesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsStaticRoutesList" => list(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsStaticRoutesItems"));
        "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsStaticRoutesItems" => dict {
            "prefix": 0 => Target::Scalar,
            "next_hop": 1 => Target::Scalar,
            "track_bfd": 2 => Target::Scalar,
            "distance": 3 => Target::Scalar,
            "tag": 4 => Target::Scalar,
            "name": 5 => Target::Scalar,
            "metric": 6 => Target::Scalar,
            "interface": 7 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsIpv6StaticRoutesList" => list(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsIpv6StaticRoutesItems"));
        "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsIpv6StaticRoutesItems" => dict {
            "prefix": 0 => Target::Scalar,
            "next_hop": 1 => Target::Scalar,
            "track_bfd": 2 => Target::Scalar,
            "distance": 3 => Target::Scalar,
            "tag": 4 => Target::Scalar,
            "name": 5 => Target::Scalar,
            "metric": 6 => Target::Scalar,
            "interface": 7 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsOspf" => dict {
            "enabled": 0 => Target::Scalar,
            "point_to_point": 1 => Target::Scalar,
            "area": 2 => Target::Scalar,
            "cost": 3 => Target::Scalar,
            "authentication": 4 => Target::Scalar,
            "simple_auth_key": 5 => Target::Scalar,
            "cleartext_simple_auth_key": 6 => Target::Scalar,
            "message_digest_keys": 7 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsOspfMessageDigestKeysList"),
        };
        "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsOspfMessageDigestKeysList" => indexed(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsOspfMessageDigestKeysListIndexedItem"), [0]);
        "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsOspfMessageDigestKeysItems" => dict {
            "id": 0 => Target::Scalar,
            "hash_algorithm": 1 => Target::Scalar,
            "key": 2 => Target::Scalar,
            "cleartext_key": 3 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsFlowTracking" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsLoopbacksList" => list(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsLoopbacksItems"));
        "AVDDesignNetworkServicesItemsVrfsItemsLoopbacksItems" => dict {
            "node": 0 => Target::Scalar,
            "loopback": 1 => Target::Scalar,
            "ip_address": 2 => Target::Scalar,
            "description": 3 => Target::Scalar,
            "enabled": 4 => Target::Scalar,
            "ospf": 5 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsLoopbacksItemsOspf"),
            "hardware_forwarding": 6 => Target::Scalar,
            "raw_eos_cli": 7 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsLoopbacksItemsOspf" => dict {
            "enabled": 0 => Target::Scalar,
            "area": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsStaticRoutesList" => list(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsStaticRoutesItems"));
        "AVDDesignNetworkServicesItemsVrfsItemsStaticRoutesItems" => dict {
            "nodes": 0 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsStaticRoutesItemsNodesList"),
            "prefix": 1 => Target::Scalar,
            "next_hop": 2 => Target::Scalar,
            "track_bfd": 3 => Target::Scalar,
            "distance": 4 => Target::Scalar,
            "tag": 5 => Target::Scalar,
            "name": 6 => Target::Scalar,
            "metric": 7 => Target::Scalar,
            "interface": 8 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsStaticRoutesItemsNodesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsIpv6StaticRoutesList" => list(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsIpv6StaticRoutesItems"));
        "AVDDesignNetworkServicesItemsVrfsItemsIpv6StaticRoutesItems" => dict {
            "nodes": 0 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsIpv6StaticRoutesItemsNodesList"),
            "prefix": 1 => Target::Scalar,
            "next_hop": 2 => Target::Scalar,
            "track_bfd": 3 => Target::Scalar,
            "distance": 4 => Target::Scalar,
            "tag": 5 => Target::Scalar,
            "name": 6 => Target::Scalar,
            "metric": 7 => Target::Scalar,
            "interface": 8 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsIpv6StaticRoutesItemsNodesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsStaticArpEntriesList" => list(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsStaticArpEntriesItems"));
        "AVDDesignNetworkServicesItemsVrfsItemsStaticArpEntriesItems" => dict {
            "ipv4_address": 0 => Target::Scalar,
            "mac_address": 1 => Target::Scalar,
            "nodes": 2 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsStaticArpEntriesItemsNodesList"),
        };
        "AVDDesignNetworkServicesItemsVrfsItemsStaticArpEntriesItemsNodesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeersList" => list(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeersListKeyedItem"));
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeersItems" => dict {
            "ip_address": 0 => Target::Scalar,
            "peer_group": 1 => Target::Scalar,
            "remote_as": 2 => Target::Scalar,
            "description": 3 => Target::Scalar,
            "password": 4 => Target::Scalar,
            "cleartext_password": 5 => Target::Scalar,
            "send_community": 6 => Target::Scalar,
            "next_hop_self": 7 => Target::Scalar,
            "timers": 8 => Target::Scalar,
            "maximum_routes": 9 => Target::Scalar,
            "maximum_routes_warning_only": 10 => Target::Scalar,
            "default_originate": 11 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeersItemsDefaultOriginate"),
            "update_source": 12 => Target::Scalar,
            "ebgp_multihop": 13 => Target::Scalar,
            "nodes": 14 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeersItemsNodesList"),
            "set_ipv4_next_hop": 15 => Target::Scalar,
            "set_ipv6_next_hop": 16 => Target::Scalar,
            "route_map_out": 17 => Target::Scalar,
            "route_map_in": 18 => Target::Scalar,
            "prefix_list_in": 19 => Target::Scalar,
            "prefix_list_out": 20 => Target::Scalar,
            "local_as": 21 => Target::Scalar,
            "weight": 22 => Target::Scalar,
            "bfd": 23 => Target::Scalar,
            "bfd_timers": 24 => Target::Model("EosCliConfigGenRouterBgpVrfsItemsNeighborsItemsBfdTimers"),
            "route_reflector_client": 25 => Target::Scalar,
            "shutdown": 26 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeersItemsDefaultOriginate" => dict {
            "always": 0 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeersItemsNodesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsBgp" => dict {
            "enabled": 0 => Target::Scalar,
            "router_id": 1 => Target::Scalar,
            "graceful_restart": 2 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpGracefulRestart"),
            "raw_eos_cli": 3 => Target::Scalar,
            "structured_config": 4 => Target::Opaque,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsBgpGracefulRestart" => dict {
            "enabled": 0 => Target::Scalar,
            "restart_time": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsList" => indexed(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsListIndexedItem"), [0]);
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItems" => dict {
            "name": 0 => Target::Scalar,
            "nodes": 1 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsNodesList"),
            "password": 2 => Target::Scalar,
            "cleartext_password": 3 => Target::Scalar,
            "address_family_ipv4": 4 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsAddressFamilyIpv4"),
            "address_family_ipv6": 5 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsAddressFamilyIpv6"),
            "listen_ranges": 6 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsListenRangesList"),
            "metadata": 7 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsMetadata"),
            "remote_as": 8 => Target::Scalar,
            "local_as": 9 => Target::Scalar,
            "description": 10 => Target::Scalar,
            "shutdown": 11 => Target::Scalar,
            "as_path": 12 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsAsPath"),
            "remove_private_as": 13 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsRemovePrivateAs"),
            "remove_private_as_ingress": 14 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsRemovePrivateAsIngress"),
            "next_hop_unchanged": 15 => Target::Scalar,
            "update_source": 16 => Target::Scalar,
            "route_reflector_client": 17 => Target::Scalar,
            "bfd": 18 => Target::Scalar,
            "bfd_timers": 19 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsBfdTimers"),
            "ebgp_multihop": 20 => Target::Scalar,
            "next_hop_peer": 21 => Target::Scalar,
            "next_hop_self": 22 => Target::Scalar,
            "password_type": 23 => Target::Scalar,
            "passive": 24 => Target::Scalar,
            "default_originate": 25 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsDefaultOriginate"),
            "enforce_first_as": 26 => Target::Scalar,
            "send_community": 27 => Target::Scalar,
            "maximum_routes": 28 => Target::Scalar,
            "maximum_routes_warning_limit": 29 => Target::Scalar,
            "maximum_routes_warning_only": 30 => Target::Scalar,
            "maximum_accepted_routes": 31 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsMaximumAcceptedRoutes"),
            "missing_policy": 32 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsMissingPolicy"),
            "link_bandwidth": 33 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsLinkBandwidth"),
            "allowas_in": 34 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsAllowasIn"),
            "weight": 35 => Target::Scalar,
            "timers": 36 => Target::Scalar,
            "rib_in_pre_policy_retain": 37 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsRibInPrePolicyRetain"),
            "route_map_in": 38 => Target::Scalar,
            "route_map_out": 39 => Target::Scalar,
            "peer_tag_in": 40 => Target::Scalar,
            "peer_tag_out_discard": 41 => Target::Scalar,
            "session_tracker": 42 => Target::Scalar,
            "shared_secret": 43 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsSharedSecret"),
            "ttl_maximum_hops": 44 => Target::Scalar,
            "maximum_advertised_routes": 45 => Target::Scalar,
            "maximum_advertised_routes_warning_limit": 46 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsNodesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsAddressFamilyIpv4" => dict {
            "activate": 0 => Target::Scalar,
            "route_map_in": 1 => Target::Scalar,
            "route_map_out": 2 => Target::Scalar,
            "rcf_in": 3 => Target::Scalar,
            "rcf_out": 4 => Target::Scalar,
            "default_originate": 5 => Target::Model("EosCliConfigGenRouterBgpAddressFamilyIpv4PeerGroupsItemsDefaultOriginate"),
            "next_hop": 6 => Target::Model("EosCliConfigGenRouterBgpAddressFamilyIpv4PeerGroupsItemsNextHop"),
            "prefix_list_in": 7 => Target::Scalar,
            "prefix_list_out": 8 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsAddressFamilyIpv6" => dict {
            "activate": 0 => Target::Scalar,
            "route_map_in": 1 => Target::Scalar,
            "route_map_out": 2 => Target::Scalar,
            "rcf_in": 3 => Target::Scalar,
            "rcf_out": 4 => Target::Scalar,
            "default_originate": 5 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsAddressFamilyIpv6DefaultOriginate"),
            "prefix_list_in": 6 => Target::Scalar,
            "prefix_list_out": 7 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsAddressFamilyIpv6DefaultOriginate" => dict {
            "enabled": 0 => Target::Scalar,
            "always": 1 => Target::Scalar,
            "route_map": 2 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsListenRangesList" => list(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsListenRangesItems"));
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsListenRangesItems" => dict {
            "prefix": 0 => Target::Scalar,
            "remote_as": 1 => Target::Scalar,
            "peer_id_include_router_id": 2 => Target::Scalar,
            "peer_filter": 3 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsMetadata" => dict {
            "field_type": 0 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsAsPath" => dict {
            "remote_as_replace_out": 0 => Target::Scalar,
            "prepend_own_disabled": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsRemovePrivateAs" => dict {
            "enabled": 0 => Target::Scalar,
            "all": 1 => Target::Scalar,
            "replace_as": 2 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsRemovePrivateAsIngress" => dict {
            "enabled": 0 => Target::Scalar,
            "replace_as": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsBfdTimers" => dict {
            "interval": 0 => Target::Scalar,
            "min_rx": 1 => Target::Scalar,
            "multiplier": 2 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsDefaultOriginate" => dict {
            "enabled": 0 => Target::Scalar,
            "always": 1 => Target::Scalar,
            "route_map": 2 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsMaximumAcceptedRoutes" => dict {
            "limit": 0 => Target::Scalar,
            "warning_limit": 1 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsMaximumAcceptedRoutesWarningLimit"),
        };
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsMaximumAcceptedRoutesWarningLimit" => dict {
            "count": 0 => Target::Scalar,
            "percent": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsMissingPolicy" => dict {
            "direction_in": 0 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsMissingPolicyDirectionIn"),
            "direction_out": 1 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsMissingPolicyDirectionOut"),
        };
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsMissingPolicyDirectionIn" => dict {
            "action": 0 => Target::Scalar,
            "include_community_list": 1 => Target::Scalar,
            "include_prefix_list": 2 => Target::Scalar,
            "include_sub_route_map": 3 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsMissingPolicyDirectionOut" => dict {
            "action": 0 => Target::Scalar,
            "include_community_list": 1 => Target::Scalar,
            "include_prefix_list": 2 => Target::Scalar,
            "include_sub_route_map": 3 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsLinkBandwidth" => dict {
            "enabled": 0 => Target::Scalar,
            "default": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsAllowasIn" => dict {
            "enabled": 0 => Target::Scalar,
            "times": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsRibInPrePolicyRetain" => dict {
            "enabled": 0 => Target::Scalar,
            "all": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItemsSharedSecret" => dict {
            "profile": 0 => Target::Scalar,
            "hash_algorithm": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVrfsItemsAdditionalRouteTargetsList" => list(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsAdditionalRouteTargetsItems"));
        "AVDDesignNetworkServicesItemsVrfsItemsAdditionalRouteTargetsItems" => dict {
            "field_type": 0 => Target::Scalar,
            "address_family": 1 => Target::Scalar,
            "route_target": 2 => Target::Scalar,
            "nodes": 3 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsAdditionalRouteTargetsItemsNodesList"),
        };
        "AVDDesignNetworkServicesItemsVrfsItemsAdditionalRouteTargetsItemsNodesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsAggregateAddressesList" => list(Target::Model("AVDDesignNetworkServicesItemsVrfsItemsAggregateAddressesItems"));
        "AVDDesignNetworkServicesItemsVrfsItemsAggregateAddressesItems" => dict {
            "nodes": 0 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsAggregateAddressesItemsNodesList"),
            "prefix": 1 => Target::Scalar,
            "advertise_only": 2 => Target::Scalar,
            "as_set": 3 => Target::Scalar,
            "summary_only": 4 => Target::Scalar,
            "attribute_map": 5 => Target::Scalar,
            "match_map": 6 => Target::Scalar,
            "attribute": 7 => Target::Model("AVDDesignNetworkServicesItemsVrfsItemsAggregateAddressesItemsAttribute"),
        };
        "AVDDesignNetworkServicesItemsVrfsItemsAggregateAddressesItemsNodesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsVrfsItemsAggregateAddressesItemsAttribute" => dict {
            "rcf": 0 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsL2vlansList" => list(Target::Model("AVDDesignNetworkServicesItemsL2vlansListKeyedItem"));
        "AVDDesignNetworkServicesItemsL2vlansItems" => dict {
            "id": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
            "profile": 2 => Target::Scalar,
            "tags": 3 => Target::Model("AVDDesignNetworkServicesItemsL2vlansItemsTagsList"),
            "address_locking": 4 => Target::Model("EosCliConfigGenVlansItemsAddressLockingAddressFamily"),
            "vni_override": 5 => Target::Scalar,
            "rt_override": 6 => Target::Scalar,
            "rd_override": 7 => Target::Scalar,
            "vxlan": 8 => Target::Scalar,
            "spanning_tree_priority": 9 => Target::Scalar,
            "evpn_vlan_bundle": 10 => Target::Scalar,
            "trunk_groups": 11 => Target::Model("AVDDesignNetworkServicesItemsL2vlansItemsTrunkGroupsList"),
            "evpn_l2_multi_domain": 12 => Target::Scalar,
            "evpn_l2_multicast": 13 => Target::Model("AVDDesignNetworkServicesItemsL2vlansItemsEvpnL2Multicast"),
            "vxlan_flood_multicast": 14 => Target::Model("AVDDesignNetworkServicesItemsL2vlansItemsVxlanFloodMulticast"),
            "igmp_snooping": 15 => Target::Model("AVDDesignNetworkServicesItemsL2vlansItemsIgmpSnooping"),
            "igmp_snooping_enabled": 16 => Target::Scalar,
            "igmp_snooping_querier": 17 => Target::Model("AVDDesignNetworkServicesItemsL2vlansItemsIgmpSnoopingQuerier2"),
            "bgp": 18 => Target::Model("AVDDesignNetworkServicesItemsL2vlansItemsBgp"),
            "private_vlan": 19 => Target::Model("AVDDesignNetworkServicesItemsL2vlansItemsPrivateVlan"),
        };
        "AVDDesignNetworkServicesItemsL2vlansItemsTagsList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsL2vlansItemsTrunkGroupsList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsL2vlansItemsEvpnL2Multicast" => dict {
            "enabled": 0 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsL2vlansItemsVxlanFloodMulticast" => dict {
            "enabled": 0 => Target::Scalar,
            "underlay_multicast_group": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsL2vlansItemsIgmpSnooping" => dict {
            "enabled": 0 => Target::Scalar,
            "querier": 1 => Target::Model("AVDDesignNetworkServicesItemsL2vlansItemsIgmpSnoopingQuerier"),
            "fast_leave": 2 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsL2vlansItemsIgmpSnoopingQuerier" => dict {
            "enabled": 0 => Target::Scalar,
            "source_address": 1 => Target::Scalar,
            "version": 2 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsL2vlansItemsIgmpSnoopingQuerier2" => dict {
            "enabled": 0 => Target::Scalar,
            "source_address": 1 => Target::Scalar,
            "version": 2 => Target::Scalar,
            "fast_leave": 3 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsL2vlansItemsBgp" => dict {
            "structured_config": 0 => Target::Opaque,
            "raw_eos_cli": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsL2vlansItemsPrivateVlan" => dict {
            "field_type": 0 => Target::Scalar,
            "primary_vlan": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsVpws" => dict {
            "mpls_control_word": 0 => Target::Scalar,
            "mtu": 1 => Target::Scalar,
            "label_flow": 2 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsPointToPointServicesList" => indexed(Target::Model("AVDDesignNetworkServicesItemsPointToPointServicesListIndexedItem"), [0]);
        "AVDDesignNetworkServicesItemsPointToPointServicesItems" => dict {
            "name": 0 => Target::Scalar,
            "field_type": 1 => Target::Scalar,
            "subinterfaces": 2 => Target::Model("AVDDesignNetworkServicesItemsPointToPointServicesItemsSubinterfacesList"),
            "endpoints": 3 => Target::Model("AVDDesignNetworkServicesItemsPointToPointServicesItemsEndpointsList"),
            "lldp_disable": 4 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsPointToPointServicesItemsSubinterfacesList" => indexed(Target::Model("AVDDesignNetworkServicesItemsPointToPointServicesItemsSubinterfacesListIndexedItem"), [0]);
        "AVDDesignNetworkServicesItemsPointToPointServicesItemsSubinterfacesItems" => dict {
            "number": 0 => Target::Scalar,
            "port_channel": 1 => Target::Model("AVDDesignNetworkServicesItemsPointToPointServicesItemsSubinterfacesItemsPortChannel"),
            "structured_config": 2 => Target::Opaque,
            "raw_eos_cli": 3 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsPointToPointServicesItemsSubinterfacesItemsPortChannel" => dict {
            "structured_config": 0 => Target::Opaque,
            "raw_eos_cli": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesItemsPointToPointServicesItemsEndpointsList" => list(Target::Model("AVDDesignNetworkServicesItemsPointToPointServicesItemsEndpointsItems"));
        "AVDDesignNetworkServicesItemsPointToPointServicesItemsEndpointsItems" => dict {
            "id": 0 => Target::Scalar,
            "nodes": 1 => Target::Model("AVDDesignNetworkServicesItemsPointToPointServicesItemsEndpointsItemsNodesList"),
            "interfaces": 2 => Target::Model("AVDDesignNetworkServicesItemsPointToPointServicesItemsEndpointsItemsInterfacesList"),
            "port_channel": 3 => Target::Model("AVDDesignNetworkServicesItemsPointToPointServicesItemsEndpointsItemsPortChannel"),
        };
        "AVDDesignNetworkServicesItemsPointToPointServicesItemsEndpointsItemsNodesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsPointToPointServicesItemsEndpointsItemsInterfacesList" => list(Target::Scalar);
        "AVDDesignNetworkServicesItemsPointToPointServicesItemsEndpointsItemsPortChannel" => dict {
            "mode": 0 => Target::Scalar,
            "short_esi": 1 => Target::Scalar,
        };
        "AVDDesignNetworkServicesKeysList" => indexed(Target::Model("AVDDesignNetworkServicesKeysListIndexedItem"), [0]);
        "AVDDesignNetworkServicesKeysItems" => dict {
            "name": 0 => Target::Scalar,
        };
        "AVDDesignCustomNodeTypeKeysList" => indexed(Target::Model("AVDDesignCustomNodeTypeKeysListIndexedItem"), [0]);
        "AVDDesignCustomNodeTypeKeysItems" => dict {
            "key": 0 => Target::Scalar,
            "field_type": 1 => Target::Scalar,
            "connected_endpoints": 2 => Target::Scalar,
            "default_evpn_role": 3 => Target::Scalar,
            "default_ptp_priority1": 4 => Target::Scalar,
            "default_underlay_routing_protocol": 5 => Target::Scalar,
            "default_overlay_routing_protocol": 6 => Target::Scalar,
            "default_mpls_overlay_role": 7 => Target::Scalar,
            "default_overlay_address_families": 8 => Target::Model("AVDDesignCustomNodeTypeKeysItemsDefaultOverlayAddressFamiliesList"),
            "default_evpn_encapsulation": 9 => Target::Scalar,
            "default_wan_role": 10 => Target::Scalar,
            "default_flow_tracker_type": 11 => Target::Scalar,
            "mlag_support": 12 => Target::Scalar,
            "network_services": 13 => Target::Model("AVDDesignCustomNodeTypeKeysItemsNetworkServices"),
            "underlay_router": 14 => Target::Scalar,
            "uplink_type": 15 => Target::Scalar,
            "vtep": 16 => Target::Scalar,
            "mpls_lsr": 17 => Target::Scalar,
            "ip_addressing": 18 => Target::Model("AVDDesignCustomNodeTypeKeysItemsIpAddressing"),
            "interface_descriptions": 19 => Target::Model("AVDDesignCustomNodeTypeKeysItemsInterfaceDescriptions"),
            "cv_tags_topology_type": 20 => Target::Scalar,
        };
        "AVDDesignCustomNodeTypeKeysItemsDefaultOverlayAddressFamiliesList" => list(Target::Scalar);
        "AVDDesignCustomNodeTypeKeysItemsNetworkServices" => dict {
            "l1": 0 => Target::Scalar,
            "l2": 1 => Target::Scalar,
            "l3": 2 => Target::Scalar,
        };
        "AVDDesignCustomNodeTypeKeysItemsIpAddressing" => dict {
            "python_module": 0 => Target::Scalar,
            "python_class_name": 1 => Target::Scalar,
            "router_id": 2 => Target::Scalar,
            "router_id_ipv6": 3 => Target::Scalar,
            "mlag_ip_primary": 4 => Target::Scalar,
            "mlag_ip_secondary": 5 => Target::Scalar,
            "mlag_l3_ip_primary": 6 => Target::Scalar,
            "mlag_l3_ip_secondary": 7 => Target::Scalar,
            "mlag_ibgp_peering_ip_primary": 8 => Target::Scalar,
            "mlag_ibgp_peering_ip_secondary": 9 => Target::Scalar,
            "p2p_uplinks_ip": 10 => Target::Scalar,
            "p2p_uplinks_peer_ip": 11 => Target::Scalar,
            "vtep_ip_mlag": 12 => Target::Scalar,
            "vtep_ip": 13 => Target::Scalar,
        };
        "AVDDesignCustomNodeTypeKeysItemsInterfaceDescriptions" => dict {
            "python_module": 0 => Target::Scalar,
            "python_class_name": 1 => Target::Scalar,
            "underlay_ethernet_interfaces": 2 => Target::Scalar,
            "underlay_port_channel_interfaces": 3 => Target::Scalar,
            "mlag_ethernet_interfaces": 4 => Target::Scalar,
            "mlag_port_channel_interfaces": 5 => Target::Scalar,
            "connected_endpoints_ethernet_interfaces": 6 => Target::Scalar,
            "connected_endpoints_port_channel_interfaces": 7 => Target::Scalar,
            "router_id_loopback_interface": 8 => Target::Scalar,
            "vtep_loopback_interface": 9 => Target::Scalar,
        };
        "AVDDesignNodeTypeKeysList" => indexed(Target::Model("AVDDesignNodeTypeKeysListIndexedItem"), [0]);
        "AVDDesignNodeTypeKeysItems" => dict {
            "key": 0 => Target::Scalar,
            "field_type": 1 => Target::Scalar,
            "connected_endpoints": 2 => Target::Scalar,
            "default_evpn_role": 3 => Target::Scalar,
            "default_ptp_priority1": 4 => Target::Scalar,
            "default_underlay_routing_protocol": 5 => Target::Scalar,
            "default_overlay_routing_protocol": 6 => Target::Scalar,
            "default_mpls_overlay_role": 7 => Target::Scalar,
            "default_overlay_address_families": 8 => Target::Model("AVDDesignNodeTypeKeysItemsDefaultOverlayAddressFamiliesList"),
            "default_evpn_encapsulation": 9 => Target::Scalar,
            "default_wan_role": 10 => Target::Scalar,
            "default_flow_tracker_type": 11 => Target::Scalar,
            "mlag_support": 12 => Target::Scalar,
            "network_services": 13 => Target::Model("AVDDesignNodeTypeKeysItemsNetworkServices"),
            "underlay_router": 14 => Target::Scalar,
            "uplink_type": 15 => Target::Scalar,
            "vtep": 16 => Target::Scalar,
            "mpls_lsr": 17 => Target::Scalar,
            "ip_addressing": 18 => Target::Model("AVDDesignNodeTypeKeysItemsIpAddressing"),
            "interface_descriptions": 19 => Target::Model("AVDDesignNodeTypeKeysItemsInterfaceDescriptions"),
            "cv_tags_topology_type": 20 => Target::Scalar,
        };
        "AVDDesignNodeTypeKeysItemsDefaultOverlayAddressFamiliesList" => list(Target::Scalar);
        "AVDDesignNodeTypeKeysItemsNetworkServices" => dict {
            "l1": 0 => Target::Scalar,
            "l2": 1 => Target::Scalar,
            "l3": 2 => Target::Scalar,
        };
        "AVDDesignNodeTypeKeysItemsIpAddressing" => dict {
            "python_module": 0 => Target::Scalar,
            "python_class_name": 1 => Target::Scalar,
            "router_id": 2 => Target::Scalar,
            "router_id_ipv6": 3 => Target::Scalar,
            "mlag_ip_primary": 4 => Target::Scalar,
            "mlag_ip_secondary": 5 => Target::Scalar,
            "mlag_l3_ip_primary": 6 => Target::Scalar,
            "mlag_l3_ip_secondary": 7 => Target::Scalar,
            "mlag_ibgp_peering_ip_primary": 8 => Target::Scalar,
            "mlag_ibgp_peering_ip_secondary": 9 => Target::Scalar,
            "p2p_uplinks_ip": 10 => Target::Scalar,
            "p2p_uplinks_peer_ip": 11 => Target::Scalar,
            "vtep_ip_mlag": 12 => Target::Scalar,
            "vtep_ip": 13 => Target::Scalar,
        };
        "AVDDesignNodeTypeKeysItemsInterfaceDescriptions" => dict {
            "python_module": 0 => Target::Scalar,
            "python_class_name": 1 => Target::Scalar,
            "underlay_ethernet_interfaces": 2 => Target::Scalar,
            "underlay_port_channel_interfaces": 3 => Target::Scalar,
            "mlag_ethernet_interfaces": 4 => Target::Scalar,
            "mlag_port_channel_interfaces": 5 => Target::Scalar,
            "connected_endpoints_ethernet_interfaces": 6 => Target::Scalar,
            "connected_endpoints_port_channel_interfaces": 7 => Target::Scalar,
            "router_id_loopback_interface": 8 => Target::Scalar,
            "vtep_loopback_interface": 9 => Target::Scalar,
        };
        "AVDDesignNtpSettings" => dict {
            "server_vrf": 0 => Target::Scalar,
            "set_first_ntp_server_as_preferred": 1 => Target::Scalar,
            "servers": 2 => Target::Model("AVDDesignNtpSettingsServersList"),
            "authenticate": 3 => Target::Scalar,
            "authenticate_servers_only": 4 => Target::Scalar,
            "authentication_keys": 5 => Target::Model("AVDDesignNtpSettingsAuthenticationKeysList"),
            "trusted_keys": 6 => Target::Scalar,
        };
        "AVDDesignNtpSettingsServersList" => indexed(Target::Model("AVDDesignNtpSettingsServersListIndexedItem"), [0]);
        "AVDDesignNtpSettingsServersItems" => dict {
            "name": 0 => Target::Scalar,
            "burst": 1 => Target::Scalar,
            "iburst": 2 => Target::Scalar,
            "key": 3 => Target::Scalar,
            "maxpoll": 4 => Target::Scalar,
            "minpoll": 5 => Target::Scalar,
            "version": 6 => Target::Scalar,
            "source_address": 7 => Target::Scalar,
        };
        "AVDDesignNtpSettingsAuthenticationKeysList" => indexed(Target::Model("AVDDesignNtpSettingsAuthenticationKeysListIndexedItem"), [3]);
        "AVDDesignNtpSettingsAuthenticationKeysItems" => dict {
            "key": 0 => Target::Scalar,
            "cleartext_key": 1 => Target::Scalar,
            "key_type": 2 => Target::Scalar,
            "id": 3 => Target::Scalar,
            "hash_algorithm": 4 => Target::Scalar,
        };
        "AVDDesignOverlayCvxServersList" => list(Target::Scalar);
        "AVDDesignOverlayRdType" => dict {
            "admin_subfield": 0 => Target::Scalar,
            "admin_subfield_offset": 1 => Target::Scalar,
            "vrf_admin_subfield": 2 => Target::Scalar,
            "vrf_admin_subfield_offset": 3 => Target::Scalar,
            "vlan_assigned_number_subfield": 4 => Target::Scalar,
        };
        "AVDDesignOverlayRtType" => dict {
            "admin_subfield": 0 => Target::Scalar,
            "vrf_admin_subfield": 1 => Target::Scalar,
            "vlan_assigned_number_subfield": 2 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsList" => list(Target::Model("AVDDesignCustomPlatformSettingsItems"));
        "AVDDesignCustomPlatformSettingsItems" => dict {
            "platforms": 0 => Target::Model("AVDDesignCustomPlatformSettingsItemsPlatformsList"),
            "trident_forwarding_table_partition": 1 => Target::Scalar,
            "reload_delay": 2 => Target::Model("AVDDesignCustomPlatformSettingsItemsReloadDelay"),
            "tcam_profile": 3 => Target::Scalar,
            "additional_tcam_profiles": 4 => Target::Model("AVDDesignCustomPlatformSettingsItemsAdditionalTcamProfilesList"),
            "lag_hardware_only": 5 => Target::Scalar,
            "default_interface_mtu": 6 => Target::Scalar,
            "p2p_uplinks_mtu": 7 => Target::Scalar,
            "feature_support": 8 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupport"),
            "management_interface": 9 => Target::Scalar,
            "security_entropy_sources": 10 => Target::Model("AVDDesignCustomPlatformSettingsItemsSecurityEntropySources"),
            "digital_twin": 11 => Target::Model("AVDDesignCustomPlatformSettingsItemsDigitalTwin"),
            "structured_config": 12 => Target::Opaque,
            "raw_eos_cli": 13 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsPlatformsList" => list(Target::Scalar);
        "AVDDesignCustomPlatformSettingsItemsReloadDelay" => dict {
            "mlag": 0 => Target::Scalar,
            "non_mlag": 1 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsAdditionalTcamProfilesList" => list(Target::Scalar);
        "AVDDesignCustomPlatformSettingsItemsFeatureSupport" => dict {
            "address_locking": 0 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportAddressLocking"),
            "queue_monitor": 1 => Target::Scalar,
            "queue_monitor_length_notify": 2 => Target::Scalar,
            "interface_storm_control": 3 => Target::Scalar,
            "poe": 4 => Target::Scalar,
            "subinterface_mtu": 5 => Target::Scalar,
            "subinterface_monitor_session": 6 => Target::Scalar,
            "per_interface_mtu": 7 => Target::Scalar,
            "per_interface_l2_mtu": 8 => Target::Scalar,
            "per_interface_l2_mru": 9 => Target::Scalar,
            "bgp_update_wait_install": 10 => Target::Scalar,
            "bgp_update_wait_for_convergence": 11 => Target::Scalar,
            "platform_sfe_interface_profile": 12 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportPlatformSfeInterfaceProfile"),
            "evpn_gateway_all_active_multihoming": 13 => Target::Scalar,
            "evpn_gateway_rd_rt_rewrite": 14 => Target::Scalar,
            "hardware_counters": 15 => Target::Scalar,
            "hardware_counter_features": 16 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportHardwareCounterFeatures"),
            "hardware_speed_group": 17 => Target::Scalar,
            "private_vlan": 18 => Target::Scalar,
            "sflow": 19 => Target::Scalar,
            "sflow_subinterfaces": 20 => Target::Scalar,
            "wan": 21 => Target::Scalar,
            "ptp": 22 => Target::Scalar,
            "hardware_validation": 23 => Target::Scalar,
            "errdisable_causes": 24 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCauses"),
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportAddressLocking" => dict {
            "supported": 0 => Target::Scalar,
            "ipv4_enforcement_disabled": 1 => Target::Scalar,
            "ipv6_enforcement_disabled": 2 => Target::Scalar,
            "ipv6_ethernet_interface": 3 => Target::Scalar,
            "ipv6_vlan": 4 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportPlatformSfeInterfaceProfile" => dict {
            "supported": 0 => Target::Scalar,
            "max_rx_queues": 1 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportHardwareCounterFeatures" => dict {
            "acl": 0 => Target::Scalar,
            "decap_group": 1 => Target::Scalar,
            "directflow": 2 => Target::Scalar,
            "ecn": 3 => Target::Scalar,
            "flow_spec": 4 => Target::Scalar,
            "gre_tunnel_interface": 5 => Target::Scalar,
            "ip": 6 => Target::Scalar,
            "mpls_interface": 7 => Target::Scalar,
            "mpls_lfib": 8 => Target::Scalar,
            "mpls_tunnel": 9 => Target::Scalar,
            "multicast": 10 => Target::Scalar,
            "nexthop": 11 => Target::Scalar,
            "pbr": 12 => Target::Scalar,
            "pdp": 13 => Target::Scalar,
            "policing_interface": 14 => Target::Scalar,
            "qos": 15 => Target::Scalar,
            "qos_dual_rate_policer": 16 => Target::Scalar,
            "route": 17 => Target::Scalar,
            "routed_port": 18 => Target::Scalar,
            "segment_security": 19 => Target::Scalar,
            "subinterface": 20 => Target::Scalar,
            "tapagg": 21 => Target::Scalar,
            "traffic_class": 22 => Target::Scalar,
            "traffic_policy": 23 => Target::Scalar,
            "vlan": 24 => Target::Scalar,
            "vlan_interface": 25 => Target::Scalar,
            "vni_decap": 26 => Target::Scalar,
            "vni_encap": 27 => Target::Scalar,
            "vtep_decap": 28 => Target::Scalar,
            "vtep_encap": 29 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCauses" => dict {
            "acl": 0 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesAcl"),
            "arp_inspection": 1 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesArpInspection"),
            "bpduguard": 2 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesBpduguard"),
            "dot1x": 3 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesDot1x"),
            "dot1x_coa": 4 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesDot1xCoa"),
            "dot1x_phone_classification": 5 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesDot1xPhoneClassification"),
            "dot1x_session_replace": 6 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesDot1xSessionReplace"),
            "error_correction_encoding": 7 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesErrorCorrectionEncoding"),
            "fabric_capacity_low": 8 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesFabricCapacityLow"),
            "hardware_speed_group": 9 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesHardwareSpeedGroup"),
            "hitless_reload_down": 10 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesHitlessReloadDown"),
            "interface_speed": 11 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesInterfaceSpeed"),
            "internal_error": 12 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesInternalError"),
            "lacp_rate_limit": 13 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesLacpRateLimit"),
            "link_change": 14 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesLinkChange"),
            "link_flap": 15 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesLinkFlap"),
            "no_internal_vlan": 16 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesNoInternalVlan"),
            "port_breakout": 17 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesPortBreakout"),
            "portchannelguard": 18 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesPortchannelguard"),
            "portsec": 19 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesPortsec"),
            "speed_misconfigured": 20 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesSpeedMisconfigured"),
            "storm_control": 21 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesStormControl"),
            "stuck_queue": 22 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesStuckQueue"),
            "switchcard_unreachable": 23 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesSwitchcardUnreachable"),
            "tap_port_init": 24 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesTapPortInit"),
            "tapagg": 25 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesTapagg"),
            "tpid": 26 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesTpid"),
            "transceiver_adapter": 27 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesTransceiverAdapter"),
            "uplink_failure_detection": 28 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesUplinkFailureDetection"),
            "xcvr_misconfigured": 29 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesXcvrMisconfigured"),
            "xcvr_overheat": 30 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesXcvrOverheat"),
            "xcvr_power_unsupported": 31 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesXcvrPowerUnsupported"),
            "xcvr_unsupported": 32 => Target::Model("AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesXcvrUnsupported"),
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesAcl" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesArpInspection" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesBpduguard" => dict {
            "recovery": 0 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesDot1x" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesDot1xCoa" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesDot1xPhoneClassification" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesDot1xSessionReplace" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesErrorCorrectionEncoding" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesFabricCapacityLow" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesHardwareSpeedGroup" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesHitlessReloadDown" => dict {
            "recovery": 0 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesInterfaceSpeed" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesInternalError" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesLacpRateLimit" => dict {
            "recovery": 0 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesLinkChange" => dict {
            "detection": 0 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesLinkFlap" => dict {
            "recovery": 0 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesNoInternalVlan" => dict {
            "recovery": 0 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesPortBreakout" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesPortchannelguard" => dict {
            "recovery": 0 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesPortsec" => dict {
            "recovery": 0 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesSpeedMisconfigured" => dict {
            "recovery": 0 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesStormControl" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesStuckQueue" => dict {
            "recovery": 0 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesSwitchcardUnreachable" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesTapPortInit" => dict {
            "recovery": 0 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesTapagg" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesTpid" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesTransceiverAdapter" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesUplinkFailureDetection" => dict {
            "recovery": 0 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesXcvrMisconfigured" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesXcvrOverheat" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesXcvrPowerUnsupported" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsFeatureSupportErrdisableCausesXcvrUnsupported" => dict {
            "recovery": 0 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsSecurityEntropySources" => dict {
            "hardware": 0 => Target::Scalar,
            "haveged": 1 => Target::Scalar,
            "cpu_jitter": 2 => Target::Scalar,
            "hardware_exclusive": 3 => Target::Scalar,
        };
        "AVDDesignCustomPlatformSettingsItemsDigitalTwin" => dict {
            "platform": 0 => Target::Scalar,
            "act_node_type": 1 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsList" => list(Target::Model("AVDDesignPlatformSettingsItems"));
        "AVDDesignPlatformSettingsItems" => dict {
            "platforms": 0 => Target::Model("AVDDesignPlatformSettingsItemsPlatformsList"),
            "trident_forwarding_table_partition": 1 => Target::Scalar,
            "reload_delay": 2 => Target::Model("AVDDesignPlatformSettingsItemsReloadDelay"),
            "tcam_profile": 3 => Target::Scalar,
            "additional_tcam_profiles": 4 => Target::Model("AVDDesignPlatformSettingsItemsAdditionalTcamProfilesList"),
            "lag_hardware_only": 5 => Target::Scalar,
            "default_interface_mtu": 6 => Target::Scalar,
            "p2p_uplinks_mtu": 7 => Target::Scalar,
            "feature_support": 8 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupport"),
            "management_interface": 9 => Target::Scalar,
            "security_entropy_sources": 10 => Target::Model("AVDDesignPlatformSettingsItemsSecurityEntropySources"),
            "digital_twin": 11 => Target::Model("AVDDesignPlatformSettingsItemsDigitalTwin"),
            "structured_config": 12 => Target::Opaque,
            "raw_eos_cli": 13 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsPlatformsList" => list(Target::Scalar);
        "AVDDesignPlatformSettingsItemsReloadDelay" => dict {
            "mlag": 0 => Target::Scalar,
            "non_mlag": 1 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsAdditionalTcamProfilesList" => list(Target::Scalar);
        "AVDDesignPlatformSettingsItemsFeatureSupport" => dict {
            "address_locking": 0 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportAddressLocking"),
            "queue_monitor": 1 => Target::Scalar,
            "queue_monitor_length_notify": 2 => Target::Scalar,
            "interface_storm_control": 3 => Target::Scalar,
            "poe": 4 => Target::Scalar,
            "subinterface_mtu": 5 => Target::Scalar,
            "subinterface_monitor_session": 6 => Target::Scalar,
            "per_interface_mtu": 7 => Target::Scalar,
            "per_interface_l2_mtu": 8 => Target::Scalar,
            "per_interface_l2_mru": 9 => Target::Scalar,
            "bgp_update_wait_install": 10 => Target::Scalar,
            "bgp_update_wait_for_convergence": 11 => Target::Scalar,
            "platform_sfe_interface_profile": 12 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportPlatformSfeInterfaceProfile"),
            "evpn_gateway_all_active_multihoming": 13 => Target::Scalar,
            "evpn_gateway_rd_rt_rewrite": 14 => Target::Scalar,
            "hardware_counters": 15 => Target::Scalar,
            "hardware_counter_features": 16 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportHardwareCounterFeatures"),
            "hardware_speed_group": 17 => Target::Scalar,
            "private_vlan": 18 => Target::Scalar,
            "sflow": 19 => Target::Scalar,
            "sflow_subinterfaces": 20 => Target::Scalar,
            "wan": 21 => Target::Scalar,
            "ptp": 22 => Target::Scalar,
            "hardware_validation": 23 => Target::Scalar,
            "errdisable_causes": 24 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCauses"),
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportAddressLocking" => dict {
            "supported": 0 => Target::Scalar,
            "ipv4_enforcement_disabled": 1 => Target::Scalar,
            "ipv6_enforcement_disabled": 2 => Target::Scalar,
            "ipv6_ethernet_interface": 3 => Target::Scalar,
            "ipv6_vlan": 4 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportPlatformSfeInterfaceProfile" => dict {
            "supported": 0 => Target::Scalar,
            "max_rx_queues": 1 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportHardwareCounterFeatures" => dict {
            "acl": 0 => Target::Scalar,
            "decap_group": 1 => Target::Scalar,
            "directflow": 2 => Target::Scalar,
            "ecn": 3 => Target::Scalar,
            "flow_spec": 4 => Target::Scalar,
            "gre_tunnel_interface": 5 => Target::Scalar,
            "ip": 6 => Target::Scalar,
            "mpls_interface": 7 => Target::Scalar,
            "mpls_lfib": 8 => Target::Scalar,
            "mpls_tunnel": 9 => Target::Scalar,
            "multicast": 10 => Target::Scalar,
            "nexthop": 11 => Target::Scalar,
            "pbr": 12 => Target::Scalar,
            "pdp": 13 => Target::Scalar,
            "policing_interface": 14 => Target::Scalar,
            "qos": 15 => Target::Scalar,
            "qos_dual_rate_policer": 16 => Target::Scalar,
            "route": 17 => Target::Scalar,
            "routed_port": 18 => Target::Scalar,
            "segment_security": 19 => Target::Scalar,
            "subinterface": 20 => Target::Scalar,
            "tapagg": 21 => Target::Scalar,
            "traffic_class": 22 => Target::Scalar,
            "traffic_policy": 23 => Target::Scalar,
            "vlan": 24 => Target::Scalar,
            "vlan_interface": 25 => Target::Scalar,
            "vni_decap": 26 => Target::Scalar,
            "vni_encap": 27 => Target::Scalar,
            "vtep_decap": 28 => Target::Scalar,
            "vtep_encap": 29 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCauses" => dict {
            "acl": 0 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesAcl"),
            "arp_inspection": 1 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesArpInspection"),
            "bpduguard": 2 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesBpduguard"),
            "dot1x": 3 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesDot1x"),
            "dot1x_coa": 4 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesDot1xCoa"),
            "dot1x_phone_classification": 5 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesDot1xPhoneClassification"),
            "dot1x_session_replace": 6 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesDot1xSessionReplace"),
            "error_correction_encoding": 7 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesErrorCorrectionEncoding"),
            "fabric_capacity_low": 8 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesFabricCapacityLow"),
            "hardware_speed_group": 9 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesHardwareSpeedGroup"),
            "hitless_reload_down": 10 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesHitlessReloadDown"),
            "interface_speed": 11 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesInterfaceSpeed"),
            "internal_error": 12 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesInternalError"),
            "lacp_rate_limit": 13 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesLacpRateLimit"),
            "link_change": 14 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesLinkChange"),
            "link_flap": 15 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesLinkFlap"),
            "no_internal_vlan": 16 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesNoInternalVlan"),
            "port_breakout": 17 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesPortBreakout"),
            "portchannelguard": 18 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesPortchannelguard"),
            "portsec": 19 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesPortsec"),
            "speed_misconfigured": 20 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesSpeedMisconfigured"),
            "storm_control": 21 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesStormControl"),
            "stuck_queue": 22 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesStuckQueue"),
            "switchcard_unreachable": 23 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesSwitchcardUnreachable"),
            "tap_port_init": 24 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesTapPortInit"),
            "tapagg": 25 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesTapagg"),
            "tpid": 26 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesTpid"),
            "transceiver_adapter": 27 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesTransceiverAdapter"),
            "uplink_failure_detection": 28 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesUplinkFailureDetection"),
            "xcvr_misconfigured": 29 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesXcvrMisconfigured"),
            "xcvr_overheat": 30 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesXcvrOverheat"),
            "xcvr_power_unsupported": 31 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesXcvrPowerUnsupported"),
            "xcvr_unsupported": 32 => Target::Model("AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesXcvrUnsupported"),
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesAcl" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesArpInspection" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesBpduguard" => dict {
            "recovery": 0 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesDot1x" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesDot1xCoa" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesDot1xPhoneClassification" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesDot1xSessionReplace" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesErrorCorrectionEncoding" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesFabricCapacityLow" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesHardwareSpeedGroup" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesHitlessReloadDown" => dict {
            "recovery": 0 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesInterfaceSpeed" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesInternalError" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesLacpRateLimit" => dict {
            "recovery": 0 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesLinkChange" => dict {
            "detection": 0 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesLinkFlap" => dict {
            "recovery": 0 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesNoInternalVlan" => dict {
            "recovery": 0 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesPortBreakout" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesPortchannelguard" => dict {
            "recovery": 0 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesPortsec" => dict {
            "recovery": 0 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesSpeedMisconfigured" => dict {
            "recovery": 0 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesStormControl" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesStuckQueue" => dict {
            "recovery": 0 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesSwitchcardUnreachable" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesTapPortInit" => dict {
            "recovery": 0 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesTapagg" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesTpid" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesTransceiverAdapter" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesUplinkFailureDetection" => dict {
            "recovery": 0 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesXcvrMisconfigured" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesXcvrOverheat" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesXcvrPowerUnsupported" => dict {
            "detection": 0 => Target::Scalar,
            "recovery": 1 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsFeatureSupportErrdisableCausesXcvrUnsupported" => dict {
            "recovery": 0 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsSecurityEntropySources" => dict {
            "hardware": 0 => Target::Scalar,
            "haveged": 1 => Target::Scalar,
            "cpu_jitter": 2 => Target::Scalar,
            "hardware_exclusive": 3 => Target::Scalar,
        };
        "AVDDesignPlatformSettingsItemsDigitalTwin" => dict {
            "platform": 0 => Target::Scalar,
            "act_node_type": 1 => Target::Scalar,
        };
        "AVDDesignPlatformSpeedGroupsList" => indexed(Target::Model("AVDDesignPlatformSpeedGroupsListIndexedItem"), [0]);
        "AVDDesignPlatformSpeedGroupsItems" => dict {
            "platform": 0 => Target::Scalar,
            "speeds": 1 => Target::Model("AVDDesignPlatformSpeedGroupsItemsSpeedsList"),
        };
        "AVDDesignPlatformSpeedGroupsItemsSpeedsList" => indexed(Target::Model("AVDDesignPlatformSpeedGroupsItemsSpeedsListIndexedItem"), [0]);
        "AVDDesignPlatformSpeedGroupsItemsSpeedsItems" => dict {
            "speed": 0 => Target::Scalar,
            "speed_groups": 1 => Target::Model("AVDDesignPlatformSpeedGroupsItemsSpeedsItemsSpeedGroupsList"),
        };
        "AVDDesignPlatformSpeedGroupsItemsSpeedsItemsSpeedGroupsList" => list(Target::Scalar);
        "AVDDesignPortProfilesList" => indexed(Target::Model("AVDDesignPortProfilesListIndexedItem"), [0]);
        "AVDDesignPortProfilesItems" => dict {
            "profile": 0 => Target::Scalar,
            "parent_profile": 1 => Target::Scalar,
            "port_channel": 2 => Target::Model("AVDDesignPortProfilesItemsPortChannel"),
            "speed": 3 => Target::Scalar,
            "description": 4 => Target::Scalar,
            "enabled": 5 => Target::Scalar,
            "mode": 6 => Target::Scalar,
            "mtu": 7 => Target::Scalar,
            "l2_mtu": 8 => Target::Scalar,
            "l2_mru": 9 => Target::Scalar,
            "native_vlan": 10 => Target::Scalar,
            "native_vlan_tag": 11 => Target::Scalar,
            "phone_vlan": 12 => Target::Scalar,
            "phone_trunk_mode": 13 => Target::Scalar,
            "trunk_groups": 14 => Target::Model("AVDDesignPortProfilesItemsTrunkGroupsList"),
            "vlans": 15 => Target::Scalar,
            "mac_acl_in": 16 => Target::Scalar,
            "mac_acl_out": 17 => Target::Scalar,
            "spanning_tree_portfast": 18 => Target::Scalar,
            "spanning_tree_bpdufilter": 19 => Target::Scalar,
            "spanning_tree_bpduguard": 20 => Target::Scalar,
            "spanning_tree_link_type": 21 => Target::Scalar,
            "flowcontrol": 22 => Target::Model("EosCliConfigGenEthernetInterfacesItemsFlowcontrol"),
            "qos_profile": 23 => Target::Scalar,
            "ptp": 24 => Target::Model("AVDDesignPortProfilesItemsPtp"),
            "sflow": 25 => Target::Scalar,
            "flow_tracking": 26 => Target::Model("AVDDesignPortProfilesItemsFlowTracking"),
            "link_tracking": 27 => Target::Model("AVDDesignPortProfilesItemsLinkTracking"),
            "dot1x": 28 => Target::Model("AVDDesignPortProfilesItemsDot1x"),
            "address_locking": 29 => Target::Model("AVDDesignPortProfilesItemsAddressLocking"),
            "poe": 30 => Target::Model("EosCliConfigGenEthernetInterfacesItemsPoe"),
            "storm_control": 31 => Target::Model("AVDDesignPortProfilesItemsStormControl"),
            "monitor_sessions": 32 => Target::Model("AVDDesignPortProfilesItemsMonitorSessionsList"),
            "ethernet_segment": 33 => Target::Model("AVDDesignPortProfilesItemsEthernetSegment"),
            "validate_state": 34 => Target::Scalar,
            "validate_lldp": 35 => Target::Scalar,
            "campus_link_type": 36 => Target::Model("AVDDesignPortProfilesItemsCampusLinkTypeList"),
            "raw_eos_cli": 37 => Target::Scalar,
            "structured_config": 38 => Target::Opaque,
        };
        "AVDDesignPortProfilesItemsPortChannel" => dict {
            "subinterfaces": 0 => Target::Model("AVDDesignPortProfilesItemsPortChannelSubinterfacesList"),
            "mode": 1 => Target::Scalar,
            "channel_id": 2 => Target::Scalar,
            "description": 3 => Target::Scalar,
            "endpoint_port_channel": 4 => Target::Scalar,
            "enabled": 5 => Target::Scalar,
            "ptp_mpass": 6 => Target::Scalar,
            "lacp_fallback": 7 => Target::Model("AVDDesignPortProfilesItemsPortChannelLacpFallback"),
            "lacp_timer": 8 => Target::Model("AVDDesignPortProfilesItemsPortChannelLacpTimer"),
            "raw_eos_cli": 9 => Target::Scalar,
            "structured_config": 10 => Target::Opaque,
        };
        "AVDDesignPortProfilesItemsPortChannelSubinterfacesList" => indexed(Target::Model("AVDDesignPortProfilesItemsPortChannelSubinterfacesListIndexedItem"), [0]);
        "AVDDesignPortProfilesItemsPortChannelSubinterfacesItems" => dict {
            "number": 0 => Target::Scalar,
            "description": 1 => Target::Scalar,
            "short_esi": 2 => Target::Scalar,
            "vlan_id": 3 => Target::Scalar,
            "encapsulation_vlan": 4 => Target::Model("AVDDesignPortProfilesItemsPortChannelSubinterfacesItemsEncapsulationVlan"),
            "raw_eos_cli": 5 => Target::Scalar,
            "structured_config": 6 => Target::Opaque,
        };
        "AVDDesignPortProfilesItemsPortChannelSubinterfacesItemsEncapsulationVlan" => dict {
            "client_dot1q": 0 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsPortChannelLacpFallback" => dict {
            "mode": 0 => Target::Scalar,
            "individual": 1 => Target::Model("AVDDesignPortProfilesItemsPortChannelLacpFallbackIndividual"),
            "timeout": 2 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsPortChannelLacpFallbackIndividual" => dict {
            "profile": 0 => Target::Scalar,
            "vlans": 1 => Target::Scalar,
            "native_vlan": 2 => Target::Scalar,
            "mode": 3 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsPortChannelLacpTimer" => dict {
            "mode": 0 => Target::Scalar,
            "multiplier": 1 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsTrunkGroupsList" => list(Target::Scalar);
        "AVDDesignPortProfilesItemsPtp" => dict {
            "enabled": 0 => Target::Scalar,
            "endpoint_role": 1 => Target::Scalar,
            "profile": 2 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsFlowTracking" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsLinkTracking" => dict {
            "enabled": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsDot1x" => dict {
            "authentication_failure": 0 => Target::Model("AVDDesignPortProfilesItemsDot1xAuthenticationFailure"),
            "port_control": 1 => Target::Scalar,
            "port_control_force_authorized_phone": 2 => Target::Scalar,
            "reauthentication": 3 => Target::Scalar,
            "pae": 4 => Target::Model("AVDDesignPortProfilesItemsDot1xPae"),
            "host_mode": 5 => Target::Model("AVDDesignPortProfilesItemsDot1xHostMode"),
            "mac_based_authentication": 6 => Target::Model("AVDDesignPortProfilesItemsDot1xMacBasedAuthentication"),
            "mac_based_access_list": 7 => Target::Scalar,
            "timeout": 8 => Target::Model("AVDDesignPortProfilesItemsDot1xTimeout"),
            "reauthorization_request_limit": 9 => Target::Scalar,
            "unauthorized": 10 => Target::Model("AVDDesignPortProfilesItemsDot1xUnauthorized"),
            "eapol": 11 => Target::Model("AVDDesignPortProfilesItemsDot1xEapol"),
            "aaa": 12 => Target::Model("AVDDesignPortProfilesItemsDot1xAaa"),
        };
        "AVDDesignPortProfilesItemsDot1xAuthenticationFailure" => dict {
            "allow_access_list": 0 => Target::Scalar,
            "action": 1 => Target::Scalar,
            "allow_vlan": 2 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsDot1xPae" => dict {
            "mode": 0 => Target::Scalar,
            "supplicant_profile": 1 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsDot1xHostMode" => dict {
            "mode": 0 => Target::Scalar,
            "multi_host_authenticated": 1 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsDot1xMacBasedAuthentication" => dict {
            "enabled": 0 => Target::Scalar,
            "always": 1 => Target::Scalar,
            "host_mode_common": 2 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsDot1xTimeout" => dict {
            "idle_host": 0 => Target::Scalar,
            "quiet_period": 1 => Target::Scalar,
            "reauth_period": 2 => Target::Scalar,
            "reauth_timeout_ignore": 3 => Target::Scalar,
            "tx_period": 4 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsDot1xUnauthorized" => dict {
            "access_vlan_membership_egress": 0 => Target::Scalar,
            "native_vlan_membership_egress": 1 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsDot1xEapol" => dict {
            "disabled": 0 => Target::Scalar,
            "authentication_failure_fallback_mba": 1 => Target::Model("AVDDesignPortProfilesItemsDot1xEapolAuthenticationFailureFallbackMba"),
        };
        "AVDDesignPortProfilesItemsDot1xEapolAuthenticationFailureFallbackMba" => dict {
            "enabled": 0 => Target::Scalar,
            "timeout": 1 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsDot1xAaa" => dict {
            "unresponsive": 0 => Target::Model("AVDDesignPortProfilesItemsDot1xAaaUnresponsive"),
        };
        "AVDDesignPortProfilesItemsDot1xAaaUnresponsive" => dict {
            "eap_response": 0 => Target::Scalar,
            "action": 1 => Target::Model("AVDDesignPortProfilesItemsDot1xAaaUnresponsiveAction"),
            "phone_action": 2 => Target::Model("EosCliConfigGenDot1xAaaUnresponsivePhoneAction"),
        };
        "AVDDesignPortProfilesItemsDot1xAaaUnresponsiveAction" => dict {
            "traffic_allow_access_list": 0 => Target::Scalar,
            "apply_alternate": 1 => Target::Scalar,
            "traffic_allow_vlan": 2 => Target::Scalar,
            "apply_cached_results": 3 => Target::Scalar,
            "cached_results_timeout": 4 => Target::Model("AVDDesignPortProfilesItemsDot1xAaaUnresponsiveActionCachedResultsTimeout"),
            "traffic_allow": 5 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsDot1xAaaUnresponsiveActionCachedResultsTimeout" => dict {
            "time_duration": 0 => Target::Scalar,
            "time_duration_unit": 1 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsAddressLocking" => dict {
            "ipv4": 0 => Target::Scalar,
            "ipv6": 1 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsStormControl" => dict {
            "all": 0 => Target::Model("AVDDesignPortProfilesItemsStormControlAll"),
            "broadcast": 1 => Target::Model("AVDDesignPortProfilesItemsStormControlBroadcast"),
            "multicast": 2 => Target::Model("AVDDesignPortProfilesItemsStormControlMulticast"),
            "unknown_unicast": 3 => Target::Model("AVDDesignPortProfilesItemsStormControlUnknownUnicast"),
        };
        "AVDDesignPortProfilesItemsStormControlAll" => dict {
            "level": 0 => Target::Scalar,
            "unit": 1 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsStormControlBroadcast" => dict {
            "level": 0 => Target::Scalar,
            "unit": 1 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsStormControlMulticast" => dict {
            "level": 0 => Target::Scalar,
            "unit": 1 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsStormControlUnknownUnicast" => dict {
            "level": 0 => Target::Scalar,
            "unit": 1 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsMonitorSessionsList" => list(Target::Model("AVDDesignPortProfilesItemsMonitorSessionsItems"));
        "AVDDesignPortProfilesItemsMonitorSessionsItems" => dict {
            "name": 0 => Target::Scalar,
            "role": 1 => Target::Scalar,
            "source_settings": 2 => Target::Model("AVDDesignPortProfilesItemsMonitorSessionsItemsSourceSettings"),
            "session_settings": 3 => Target::Model("AVDDesignPortProfilesItemsMonitorSessionsItemsSessionSettings"),
        };
        "AVDDesignPortProfilesItemsMonitorSessionsItemsSourceSettings" => dict {
            "direction": 0 => Target::Scalar,
            "access_group": 1 => Target::Model("AVDDesignPortProfilesItemsMonitorSessionsItemsSourceSettingsAccessGroup"),
        };
        "AVDDesignPortProfilesItemsMonitorSessionsItemsSourceSettingsAccessGroup" => dict {
            "field_type": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
            "priority": 2 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsMonitorSessionsItemsSessionSettings" => dict {
            "encapsulation_gre_metadata_tx": 0 => Target::Scalar,
            "header_remove_size": 1 => Target::Scalar,
            "access_group": 2 => Target::Model("AVDDesignPortProfilesItemsMonitorSessionsItemsSessionSettingsAccessGroup"),
            "rate_limit_per_ingress_chip": 3 => Target::Scalar,
            "rate_limit_per_egress_chip": 4 => Target::Scalar,
            "sample": 5 => Target::Scalar,
            "truncate": 6 => Target::Model("AVDDesignPortProfilesItemsMonitorSessionsItemsSessionSettingsTruncate"),
        };
        "AVDDesignPortProfilesItemsMonitorSessionsItemsSessionSettingsAccessGroup" => dict {
            "field_type": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsMonitorSessionsItemsSessionSettingsTruncate" => dict {
            "enabled": 0 => Target::Scalar,
            "size": 1 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsEthernetSegment" => dict {
            "short_esi": 0 => Target::Scalar,
            "redundancy": 1 => Target::Scalar,
            "designated_forwarder_algorithm": 2 => Target::Scalar,
            "designated_forwarder_preferences": 3 => Target::Model("AVDDesignPortProfilesItemsEthernetSegmentDesignatedForwarderPreferencesList"),
            "dont_preempt": 4 => Target::Scalar,
        };
        "AVDDesignPortProfilesItemsEthernetSegmentDesignatedForwarderPreferencesList" => list(Target::Scalar);
        "AVDDesignPortProfilesItemsCampusLinkTypeList" => list(Target::Scalar);
        "AVDDesignPtpProfilesList" => indexed(Target::Model("AVDDesignPtpProfilesListIndexedItem"), [0]);
        "AVDDesignPtpProfilesItems" => dict {
            "profile": 0 => Target::Scalar,
            "announce": 1 => Target::Model("AVDDesignPtpProfilesItemsAnnounce"),
            "delay_req": 2 => Target::Scalar,
            "sync_message": 3 => Target::Model("AVDDesignPtpProfilesItemsSyncMessage"),
            "transport": 4 => Target::Scalar,
            "management": 5 => Target::Model("AVDDesignPtpProfilesItemsManagement"),
        };
        "AVDDesignPtpProfilesItemsAnnounce" => dict {
            "interval": 0 => Target::Scalar,
            "timeout": 1 => Target::Scalar,
        };
        "AVDDesignPtpProfilesItemsSyncMessage" => dict {
            "interval": 0 => Target::Scalar,
        };
        "AVDDesignPtpProfilesItemsManagement" => dict {
            "drop": 0 => Target::Scalar,
        };
        "AVDDesignPtpSettings" => dict {
            "enabled": 0 => Target::Scalar,
            "profile": 1 => Target::Scalar,
            "domain": 2 => Target::Scalar,
            "auto_clock_identity": 3 => Target::Scalar,
            "forward_v1": 4 => Target::Scalar,
            "free_running": 5 => Target::Model("EosCliConfigGenPtpFreeRunning"),
        };
        "AVDDesignQueueMonitorLength" => dict {
            "enabled": 0 => Target::Scalar,
            "notifying": 1 => Target::Scalar,
            "default_thresholds": 2 => Target::Model("AVDDesignQueueMonitorLengthDefaultThresholds"),
            "log": 3 => Target::Scalar,
            "cpu": 4 => Target::Model("AVDDesignQueueMonitorLengthCpu"),
            "tx_latency": 5 => Target::Scalar,
            "mirror": 6 => Target::Model("AVDDesignQueueMonitorLengthMirror"),
        };
        "AVDDesignQueueMonitorLengthDefaultThresholds" => dict {
            "high": 0 => Target::Scalar,
            "low": 1 => Target::Scalar,
        };
        "AVDDesignQueueMonitorLengthCpu" => dict {
            "thresholds": 0 => Target::Model("AVDDesignQueueMonitorLengthCpuThresholds"),
        };
        "AVDDesignQueueMonitorLengthCpuThresholds" => dict {
            "high": 0 => Target::Scalar,
            "low": 1 => Target::Scalar,
        };
        "AVDDesignQueueMonitorLengthMirror" => dict {
            "enabled": 0 => Target::Scalar,
            "destination": 1 => Target::Model("AVDDesignQueueMonitorLengthMirrorDestination"),
        };
        "AVDDesignQueueMonitorLengthMirrorDestination" => dict {
            "cpu": 0 => Target::Scalar,
            "ethernet_interfaces": 1 => Target::Model("AVDDesignQueueMonitorLengthMirrorDestinationEthernetInterfacesList"),
            "tunnel_mode_gre": 2 => Target::Model("AVDDesignQueueMonitorLengthMirrorDestinationTunnelModeGre"),
        };
        "AVDDesignQueueMonitorLengthMirrorDestinationEthernetInterfacesList" => list(Target::Scalar);
        "AVDDesignQueueMonitorLengthMirrorDestinationTunnelModeGre" => dict {
            "source": 0 => Target::Scalar,
            "destination": 1 => Target::Scalar,
            "dscp": 2 => Target::Scalar,
            "ttl": 3 => Target::Scalar,
            "protocol": 4 => Target::Scalar,
            "vrf": 5 => Target::Scalar,
        };
        "AVDDesignRedundancy" => dict {
            "protocol": 0 => Target::Scalar,
        };
        "AVDDesignSflowSettings" => dict {
            "polling_interval": 0 => Target::Scalar,
            "sample": 1 => Target::Model("AVDDesignSflowSettingsSample"),
            "destinations": 2 => Target::Model("AVDDesignSflowSettingsDestinationsList"),
            "export_to_cloudvision": 3 => Target::Model("AVDDesignSflowSettingsExportToCloudvision"),
            "vrfs": 4 => Target::Model("AVDDesignSflowSettingsVrfsList"),
        };
        "AVDDesignSflowSettingsSample" => dict {
            "rate": 0 => Target::Scalar,
        };
        "AVDDesignSflowSettingsDestinationsList" => list(Target::Model("AVDDesignSflowSettingsDestinationsItems"));
        "AVDDesignSflowSettingsDestinationsItems" => dict {
            "destination": 0 => Target::Scalar,
            "port": 1 => Target::Scalar,
            "vrf": 2 => Target::Scalar,
        };
        "AVDDesignSflowSettingsExportToCloudvision" => dict {
            "enabled": 0 => Target::Scalar,
            "vrf": 1 => Target::Scalar,
        };
        "AVDDesignSflowSettingsVrfsList" => indexed(Target::Model("AVDDesignSflowSettingsVrfsListIndexedItem"), [0]);
        "AVDDesignSflowSettingsVrfsItems" => dict {
            "name": 0 => Target::Scalar,
            "source_interface": 1 => Target::Scalar,
        };
        "AVDDesignSnmpSettings" => dict {
            "contact": 0 => Target::Scalar,
            "location": 1 => Target::Scalar,
            "location_template": 2 => Target::Scalar,
            "vrfs": 3 => Target::Model("AVDDesignSnmpSettingsVrfsList"),
            "compute_local_engineid": 4 => Target::Scalar,
            "compute_local_engineid_source": 5 => Target::Scalar,
            "local_engineid_ip": 6 => Target::Scalar,
            "compute_v3_user_localized_key": 7 => Target::Scalar,
            "users": 8 => Target::Model("AVDDesignSnmpSettingsUsersList"),
            "hosts": 9 => Target::Model("AVDDesignSnmpSettingsHostsList"),
            "communities": 10 => Target::Model("AVDDesignSnmpSettingsCommunitiesList"),
            "views": 11 => Target::Model("AVDDesignSnmpSettingsViewsList"),
            "groups": 12 => Target::Model("AVDDesignSnmpSettingsGroupsList"),
            "traps": 13 => Target::Model("EosCliConfigGenSnmpServerTraps"),
        };
        "AVDDesignSnmpSettingsVrfsList" => indexed(Target::Model("AVDDesignSnmpSettingsVrfsListIndexedItem"), [0]);
        "AVDDesignSnmpSettingsVrfsItems" => dict {
            "name": 0 => Target::Scalar,
            "enable": 1 => Target::Scalar,
            "source_interface": 2 => Target::Scalar,
            "ipv4_acl": 3 => Target::Scalar,
            "ipv6_acl": 4 => Target::Scalar,
        };
        "AVDDesignSnmpSettingsUsersList" => list(Target::Model("AVDDesignSnmpSettingsUsersItems"));
        "AVDDesignSnmpSettingsUsersItems" => dict {
            "name": 0 => Target::Scalar,
            "group": 1 => Target::Scalar,
            "version": 2 => Target::Scalar,
            "auth": 3 => Target::Scalar,
            "auth_passphrase": 4 => Target::Scalar,
            "field_priv": 5 => Target::Scalar,
            "priv_passphrase": 6 => Target::Scalar,
        };
        "AVDDesignSnmpSettingsHostsList" => list(Target::Model("AVDDesignSnmpSettingsHostsItems"));
        "AVDDesignSnmpSettingsHostsItems" => dict {
            "host": 0 => Target::Scalar,
            "vrf": 1 => Target::Scalar,
            "version": 2 => Target::Scalar,
            "community": 3 => Target::Scalar,
            "users": 4 => Target::Model("AVDDesignSnmpSettingsHostsItemsUsersList"),
        };
        "AVDDesignSnmpSettingsHostsItemsUsersList" => list(Target::Model("AVDDesignSnmpSettingsHostsItemsUsersItems"));
        "AVDDesignSnmpSettingsHostsItemsUsersItems" => dict {
            "username": 0 => Target::Scalar,
            "authentication_level": 1 => Target::Scalar,
        };
        "AVDDesignSnmpSettingsCommunitiesList" => indexed(Target::Model("AVDDesignSnmpSettingsCommunitiesListIndexedItem"), [0]);
        "AVDDesignSnmpSettingsCommunitiesItems" => dict {
            "name": 0 => Target::Scalar,
            "access": 1 => Target::Scalar,
            "access_list_ipv4": 2 => Target::Model("AVDDesignSnmpSettingsCommunitiesItemsAccessListIpv4"),
            "ipv4_standard_acl": 3 => Target::Scalar,
            "access_list_ipv6": 4 => Target::Model("AVDDesignSnmpSettingsCommunitiesItemsAccessListIpv6"),
            "view": 5 => Target::Scalar,
        };
        "AVDDesignSnmpSettingsCommunitiesItemsAccessListIpv4" => dict {
            "name": 0 => Target::Scalar,
        };
        "AVDDesignSnmpSettingsCommunitiesItemsAccessListIpv6" => dict {
            "name": 0 => Target::Scalar,
        };
        "AVDDesignSnmpSettingsViewsList" => list(Target::Model("AVDDesignSnmpSettingsViewsItems"));
        "AVDDesignSnmpSettingsViewsItems" => dict {
            "name": 0 => Target::Scalar,
            "mib_family_name": 1 => Target::Scalar,
            "included": 2 => Target::Scalar,
        };
        "AVDDesignSnmpSettingsGroupsList" => list(Target::Model("AVDDesignSnmpSettingsGroupsItems"));
        "AVDDesignSnmpSettingsGroupsItems" => dict {
            "name": 0 => Target::Scalar,
            "version": 1 => Target::Scalar,
            "authentication": 2 => Target::Scalar,
            "read": 3 => Target::Scalar,
            "write": 4 => Target::Scalar,
            "notify": 5 => Target::Scalar,
        };
        "AVDDesignSourceInterfaces" => dict {
            "http_client": 0 => Target::Model("AVDDesignSourceInterfacesHttpClient"),
            "ssh_client": 1 => Target::Model("AVDDesignSourceInterfacesSshClient"),
        };
        "AVDDesignSourceInterfacesHttpClient" => dict {
            "mgmt_interface": 0 => Target::Scalar,
            "inband_mgmt_interface": 1 => Target::Scalar,
        };
        "AVDDesignSourceInterfacesSshClient" => dict {
            "mgmt_interface": 0 => Target::Scalar,
            "inband_mgmt_interface": 1 => Target::Scalar,
        };
        "AVDDesignSpanningTreeSettings" => dict {
            "mode": 0 => Target::Scalar,
            "priority": 1 => Target::Scalar,
            "port_id_allocation_port_channel_range": 2 => Target::Model("EosCliConfigGenSpanningTreePortIdAllocationPortChannelRange"),
            "loop_guard_default": 3 => Target::Scalar,
            "edge_port_bpduguard_default": 4 => Target::Scalar,
        };
        "AVDDesignSshSettings" => dict {
            "enabled": 0 => Target::Scalar,
            "vrfs": 1 => Target::Model("AVDDesignSshSettingsVrfsList"),
            "idle_timeout": 2 => Target::Scalar,
            "client_vrfs": 3 => Target::Model("AVDDesignSshSettingsClientVrfsList"),
        };
        "AVDDesignSshSettingsVrfsList" => indexed(Target::Model("AVDDesignSshSettingsVrfsListIndexedItem"), [0]);
        "AVDDesignSshSettingsVrfsItems" => dict {
            "name": 0 => Target::Scalar,
            "enabled": 1 => Target::Scalar,
            "ipv4_acl": 2 => Target::Scalar,
            "ipv6_acl": 3 => Target::Scalar,
        };
        "AVDDesignSshSettingsClientVrfsList" => indexed(Target::Model("AVDDesignSshSettingsClientVrfsListIndexedItem"), [0]);
        "AVDDesignSshSettingsClientVrfsItems" => dict {
            "name": 0 => Target::Scalar,
            "source_interface": 1 => Target::Scalar,
        };
        "AVDDesignSviProfilesList" => indexed(Target::Model("AVDDesignSviProfilesListIndexedItem"), [0]);
        "AVDDesignSviProfilesItems" => dict {
            "profile": 0 => Target::Scalar,
            "parent_profile": 1 => Target::Scalar,
            "nodes": 2 => Target::Model("AVDDesignSviProfilesItemsNodesList"),
            "name": 3 => Target::Scalar,
            "enabled": 4 => Target::Scalar,
            "autostate": 5 => Target::Scalar,
            "description": 6 => Target::Scalar,
            "arp_gratuitous_accept": 7 => Target::Scalar,
            "ip_address": 8 => Target::Scalar,
            "ip_address_secondaries": 9 => Target::Model("AVDDesignSviProfilesItemsIpAddressSecondariesList"),
            "ipv6_address": 10 => Target::Scalar,
            "ipv6_enable": 11 => Target::Scalar,
            "ip_address_virtual": 12 => Target::Scalar,
            "ipv6_address_virtuals": 13 => Target::Model("AVDDesignSviProfilesItemsIpv6AddressVirtualsList"),
            "ipv6_nd": 14 => Target::Model("AVDDesignSviProfilesItemsIpv6Nd"),
            "ipv6_dhcp_relay": 15 => Target::Model("AVDDesignSviProfilesItemsIpv6DhcpRelay"),
            "ip_address_virtual_secondaries": 16 => Target::Model("AVDDesignSviProfilesItemsIpAddressVirtualSecondariesList"),
            "ip_virtual_router_addresses": 17 => Target::Model("AVDDesignSviProfilesItemsIpVirtualRouterAddressesList"),
            "ipv6_virtual_router_addresses": 18 => Target::Model("AVDDesignSviProfilesItemsIpv6VirtualRouterAddressesList"),
            "ipv4_acl_in": 19 => Target::Scalar,
            "ipv4_acl_out": 20 => Target::Scalar,
            "ipv6_acl_in": 21 => Target::Scalar,
            "ipv6_acl_out": 22 => Target::Scalar,
            "ip_helpers": 23 => Target::Model("AVDDesignSviProfilesItemsIpHelpersList"),
            "static_routes": 24 => Target::Model("AVDDesignSviProfilesItemsStaticRoutesList"),
            "ipv6_static_routes": 25 => Target::Model("AVDDesignSviProfilesItemsIpv6StaticRoutesList"),
            "vni_override": 26 => Target::Scalar,
            "rt_override": 27 => Target::Scalar,
            "rd_override": 28 => Target::Scalar,
            "trunk_groups": 29 => Target::Model("AVDDesignSviProfilesItemsTrunkGroupsList"),
            "evpn_l2_multicast": 30 => Target::Model("AVDDesignSviProfilesItemsEvpnL2Multicast"),
            "evpn_redistribute_router_mac_system": 31 => Target::Scalar,
            "vxlan_flood_multicast": 32 => Target::Model("AVDDesignSviProfilesItemsVxlanFloodMulticast"),
            "evpn_l3_multicast": 33 => Target::Model("AVDDesignSviProfilesItemsEvpnL3Multicast"),
            "igmp_snooping": 34 => Target::Model("AVDDesignSviProfilesItemsIgmpSnooping"),
            "igmp_snooping_enabled": 35 => Target::Scalar,
            "igmp_snooping_querier": 36 => Target::Model("AVDDesignSviProfilesItemsIgmpSnoopingQuerier2"),
            "vxlan": 37 => Target::Scalar,
            "spanning_tree_priority": 38 => Target::Scalar,
            "mtu": 39 => Target::Scalar,
            "ospf": 40 => Target::Model("AVDDesignSviProfilesItemsOspf"),
            "bgp": 41 => Target::Model("AVDDesignSviProfilesItemsBgp"),
            "raw_eos_cli": 42 => Target::Scalar,
            "structured_config": 43 => Target::Opaque,
            "evpn_l2_multi_domain": 44 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsNodesList" => indexed(Target::Model("AVDDesignSviProfilesItemsNodesListIndexedItem"), [0]);
        "AVDDesignSviProfilesItemsNodesItems" => dict {
            "node": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
            "enabled": 2 => Target::Scalar,
            "autostate": 3 => Target::Scalar,
            "description": 4 => Target::Scalar,
            "arp_gratuitous_accept": 5 => Target::Scalar,
            "ip_address": 6 => Target::Scalar,
            "ip_address_secondaries": 7 => Target::Model("AVDDesignSviProfilesItemsNodesItemsIpAddressSecondariesList"),
            "ipv6_address": 8 => Target::Scalar,
            "ipv6_enable": 9 => Target::Scalar,
            "ip_address_virtual": 10 => Target::Scalar,
            "ipv6_address_virtuals": 11 => Target::Model("AVDDesignSviProfilesItemsNodesItemsIpv6AddressVirtualsList"),
            "ipv6_nd": 12 => Target::Model("AVDDesignSviProfilesItemsNodesItemsIpv6Nd"),
            "ipv6_dhcp_relay": 13 => Target::Model("AVDDesignSviProfilesItemsNodesItemsIpv6DhcpRelay"),
            "ip_address_virtual_secondaries": 14 => Target::Model("AVDDesignSviProfilesItemsNodesItemsIpAddressVirtualSecondariesList"),
            "ip_virtual_router_addresses": 15 => Target::Model("AVDDesignSviProfilesItemsNodesItemsIpVirtualRouterAddressesList"),
            "ipv6_virtual_router_addresses": 16 => Target::Model("AVDDesignSviProfilesItemsNodesItemsIpv6VirtualRouterAddressesList"),
            "ipv4_acl_in": 17 => Target::Scalar,
            "ipv4_acl_out": 18 => Target::Scalar,
            "ipv6_acl_in": 19 => Target::Scalar,
            "ipv6_acl_out": 20 => Target::Scalar,
            "ip_helpers": 21 => Target::Model("AVDDesignSviProfilesItemsNodesItemsIpHelpersList"),
            "static_routes": 22 => Target::Model("AVDDesignSviProfilesItemsNodesItemsStaticRoutesList"),
            "ipv6_static_routes": 23 => Target::Model("AVDDesignSviProfilesItemsNodesItemsIpv6StaticRoutesList"),
            "vni_override": 24 => Target::Scalar,
            "rt_override": 25 => Target::Scalar,
            "rd_override": 26 => Target::Scalar,
            "trunk_groups": 27 => Target::Model("AVDDesignSviProfilesItemsNodesItemsTrunkGroupsList"),
            "evpn_l2_multicast": 28 => Target::Model("AVDDesignSviProfilesItemsNodesItemsEvpnL2Multicast"),
            "evpn_redistribute_router_mac_system": 29 => Target::Scalar,
            "vxlan_flood_multicast": 30 => Target::Model("AVDDesignSviProfilesItemsNodesItemsVxlanFloodMulticast"),
            "evpn_l3_multicast": 31 => Target::Model("AVDDesignSviProfilesItemsNodesItemsEvpnL3Multicast"),
            "igmp_snooping": 32 => Target::Model("AVDDesignSviProfilesItemsNodesItemsIgmpSnooping"),
            "igmp_snooping_enabled": 33 => Target::Scalar,
            "igmp_snooping_querier": 34 => Target::Model("AVDDesignSviProfilesItemsNodesItemsIgmpSnoopingQuerier2"),
            "vxlan": 35 => Target::Scalar,
            "spanning_tree_priority": 36 => Target::Scalar,
            "mtu": 37 => Target::Scalar,
            "ospf": 38 => Target::Model("AVDDesignSviProfilesItemsNodesItemsOspf"),
            "bgp": 39 => Target::Model("AVDDesignSviProfilesItemsNodesItemsBgp"),
            "raw_eos_cli": 40 => Target::Scalar,
            "structured_config": 41 => Target::Opaque,
            "evpn_l2_multi_domain": 42 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsNodesItemsIpAddressSecondariesList" => list(Target::Scalar);
        "AVDDesignSviProfilesItemsNodesItemsIpv6AddressVirtualsList" => list(Target::Scalar);
        "AVDDesignSviProfilesItemsNodesItemsIpv6Nd" => dict {
            "advertise_ipv6_address_virtuals": 0 => Target::Scalar,
            "valid_lifetime": 1 => Target::Scalar,
            "preferred_lifetime": 2 => Target::Scalar,
            "ra_dns_servers": 3 => Target::Model("AVDDesignSviProfilesItemsNodesItemsIpv6NdRaDnsServers"),
        };
        "AVDDesignSviProfilesItemsNodesItemsIpv6NdRaDnsServers" => dict {
            "servers": 0 => Target::Model("AVDDesignSviProfilesItemsNodesItemsIpv6NdRaDnsServersServersList"),
            "dns_servers_lifetime": 1 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsNodesItemsIpv6NdRaDnsServersServersList" => indexed(Target::Model("AVDDesignSviProfilesItemsNodesItemsIpv6NdRaDnsServersServersListIndexedItem"), [0]);
        "AVDDesignSviProfilesItemsNodesItemsIpv6NdRaDnsServersServersItems" => dict {
            "address": 0 => Target::Scalar,
            "lifetime": 1 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsNodesItemsIpv6DhcpRelay" => dict {
            "destinations": 0 => Target::Model("AVDDesignSviProfilesItemsNodesItemsIpv6DhcpRelayDestinationsList"),
        };
        "AVDDesignSviProfilesItemsNodesItemsIpv6DhcpRelayDestinationsList" => indexed(Target::Model("AVDDesignSviProfilesItemsNodesItemsIpv6DhcpRelayDestinationsListIndexedItem"), [0]);
        "AVDDesignSviProfilesItemsNodesItemsIpv6DhcpRelayDestinationsItems" => dict {
            "address": 0 => Target::Scalar,
            "vrf": 1 => Target::Scalar,
            "local_interface": 2 => Target::Scalar,
            "source_address": 3 => Target::Scalar,
            "link_address": 4 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsNodesItemsIpAddressVirtualSecondariesList" => list(Target::Scalar);
        "AVDDesignSviProfilesItemsNodesItemsIpVirtualRouterAddressesList" => list(Target::Scalar);
        "AVDDesignSviProfilesItemsNodesItemsIpv6VirtualRouterAddressesList" => list(Target::Scalar);
        "AVDDesignSviProfilesItemsNodesItemsIpHelpersList" => indexed(Target::Model("AVDDesignSviProfilesItemsNodesItemsIpHelpersListIndexedItem"), [0]);
        "AVDDesignSviProfilesItemsNodesItemsIpHelpersItems" => dict {
            "ip_helper": 0 => Target::Scalar,
            "source_interface": 1 => Target::Scalar,
            "source_vrf": 2 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsNodesItemsStaticRoutesList" => list(Target::Model("AVDDesignSviProfilesItemsNodesItemsStaticRoutesItems"));
        "AVDDesignSviProfilesItemsNodesItemsStaticRoutesItems" => dict {
            "prefix": 0 => Target::Scalar,
            "next_hop": 1 => Target::Scalar,
            "track_bfd": 2 => Target::Scalar,
            "distance": 3 => Target::Scalar,
            "tag": 4 => Target::Scalar,
            "name": 5 => Target::Scalar,
            "metric": 6 => Target::Scalar,
            "interface": 7 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsNodesItemsIpv6StaticRoutesList" => list(Target::Model("AVDDesignSviProfilesItemsNodesItemsIpv6StaticRoutesItems"));
        "AVDDesignSviProfilesItemsNodesItemsIpv6StaticRoutesItems" => dict {
            "prefix": 0 => Target::Scalar,
            "next_hop": 1 => Target::Scalar,
            "track_bfd": 2 => Target::Scalar,
            "distance": 3 => Target::Scalar,
            "tag": 4 => Target::Scalar,
            "name": 5 => Target::Scalar,
            "metric": 6 => Target::Scalar,
            "interface": 7 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsNodesItemsTrunkGroupsList" => list(Target::Scalar);
        "AVDDesignSviProfilesItemsNodesItemsEvpnL2Multicast" => dict {
            "enabled": 0 => Target::Scalar,
            "always_redistribute_igmp": 1 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsNodesItemsVxlanFloodMulticast" => dict {
            "enabled": 0 => Target::Scalar,
            "underlay_multicast_group": 1 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsNodesItemsEvpnL3Multicast" => dict {
            "enabled": 0 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsNodesItemsIgmpSnooping" => dict {
            "enabled": 0 => Target::Scalar,
            "querier": 1 => Target::Model("AVDDesignSviProfilesItemsNodesItemsIgmpSnoopingQuerier"),
            "fast_leave": 2 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsNodesItemsIgmpSnoopingQuerier" => dict {
            "enabled": 0 => Target::Scalar,
            "source_address": 1 => Target::Scalar,
            "version": 2 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsNodesItemsIgmpSnoopingQuerier2" => dict {
            "enabled": 0 => Target::Scalar,
            "source_address": 1 => Target::Scalar,
            "version": 2 => Target::Scalar,
            "fast_leave": 3 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsNodesItemsOspf" => dict {
            "enabled": 0 => Target::Scalar,
            "point_to_point": 1 => Target::Scalar,
            "area": 2 => Target::Scalar,
            "cost": 3 => Target::Scalar,
            "authentication": 4 => Target::Scalar,
            "simple_auth_key": 5 => Target::Scalar,
            "cleartext_simple_auth_key": 6 => Target::Scalar,
            "message_digest_keys": 7 => Target::Model("AVDDesignSviProfilesItemsNodesItemsOspfMessageDigestKeysList"),
        };
        "AVDDesignSviProfilesItemsNodesItemsOspfMessageDigestKeysList" => indexed(Target::Model("AVDDesignSviProfilesItemsNodesItemsOspfMessageDigestKeysListIndexedItem"), [0]);
        "AVDDesignSviProfilesItemsNodesItemsOspfMessageDigestKeysItems" => dict {
            "id": 0 => Target::Scalar,
            "hash_algorithm": 1 => Target::Scalar,
            "key": 2 => Target::Scalar,
            "cleartext_key": 3 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsNodesItemsBgp" => dict {
            "structured_config": 0 => Target::Opaque,
            "raw_eos_cli": 1 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsIpAddressSecondariesList" => list(Target::Scalar);
        "AVDDesignSviProfilesItemsIpv6AddressVirtualsList" => list(Target::Scalar);
        "AVDDesignSviProfilesItemsIpv6Nd" => dict {
            "advertise_ipv6_address_virtuals": 0 => Target::Scalar,
            "valid_lifetime": 1 => Target::Scalar,
            "preferred_lifetime": 2 => Target::Scalar,
            "ra_dns_servers": 3 => Target::Model("AVDDesignSviProfilesItemsIpv6NdRaDnsServers"),
        };
        "AVDDesignSviProfilesItemsIpv6NdRaDnsServers" => dict {
            "servers": 0 => Target::Model("AVDDesignSviProfilesItemsIpv6NdRaDnsServersServersList"),
            "dns_servers_lifetime": 1 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsIpv6NdRaDnsServersServersList" => indexed(Target::Model("AVDDesignSviProfilesItemsIpv6NdRaDnsServersServersListIndexedItem"), [0]);
        "AVDDesignSviProfilesItemsIpv6NdRaDnsServersServersItems" => dict {
            "address": 0 => Target::Scalar,
            "lifetime": 1 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsIpv6DhcpRelay" => dict {
            "destinations": 0 => Target::Model("AVDDesignSviProfilesItemsIpv6DhcpRelayDestinationsList"),
        };
        "AVDDesignSviProfilesItemsIpv6DhcpRelayDestinationsList" => indexed(Target::Model("AVDDesignSviProfilesItemsIpv6DhcpRelayDestinationsListIndexedItem"), [0]);
        "AVDDesignSviProfilesItemsIpv6DhcpRelayDestinationsItems" => dict {
            "address": 0 => Target::Scalar,
            "vrf": 1 => Target::Scalar,
            "local_interface": 2 => Target::Scalar,
            "source_address": 3 => Target::Scalar,
            "link_address": 4 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsIpAddressVirtualSecondariesList" => list(Target::Scalar);
        "AVDDesignSviProfilesItemsIpVirtualRouterAddressesList" => list(Target::Scalar);
        "AVDDesignSviProfilesItemsIpv6VirtualRouterAddressesList" => list(Target::Scalar);
        "AVDDesignSviProfilesItemsIpHelpersList" => indexed(Target::Model("AVDDesignSviProfilesItemsIpHelpersListIndexedItem"), [0]);
        "AVDDesignSviProfilesItemsIpHelpersItems" => dict {
            "ip_helper": 0 => Target::Scalar,
            "source_interface": 1 => Target::Scalar,
            "source_vrf": 2 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsStaticRoutesList" => list(Target::Model("AVDDesignSviProfilesItemsStaticRoutesItems"));
        "AVDDesignSviProfilesItemsStaticRoutesItems" => dict {
            "prefix": 0 => Target::Scalar,
            "next_hop": 1 => Target::Scalar,
            "track_bfd": 2 => Target::Scalar,
            "distance": 3 => Target::Scalar,
            "tag": 4 => Target::Scalar,
            "name": 5 => Target::Scalar,
            "metric": 6 => Target::Scalar,
            "interface": 7 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsIpv6StaticRoutesList" => list(Target::Model("AVDDesignSviProfilesItemsIpv6StaticRoutesItems"));
        "AVDDesignSviProfilesItemsIpv6StaticRoutesItems" => dict {
            "prefix": 0 => Target::Scalar,
            "next_hop": 1 => Target::Scalar,
            "track_bfd": 2 => Target::Scalar,
            "distance": 3 => Target::Scalar,
            "tag": 4 => Target::Scalar,
            "name": 5 => Target::Scalar,
            "metric": 6 => Target::Scalar,
            "interface": 7 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsTrunkGroupsList" => list(Target::Scalar);
        "AVDDesignSviProfilesItemsEvpnL2Multicast" => dict {
            "enabled": 0 => Target::Scalar,
            "always_redistribute_igmp": 1 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsVxlanFloodMulticast" => dict {
            "enabled": 0 => Target::Scalar,
            "underlay_multicast_group": 1 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsEvpnL3Multicast" => dict {
            "enabled": 0 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsIgmpSnooping" => dict {
            "enabled": 0 => Target::Scalar,
            "querier": 1 => Target::Model("AVDDesignSviProfilesItemsIgmpSnoopingQuerier"),
            "fast_leave": 2 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsIgmpSnoopingQuerier" => dict {
            "enabled": 0 => Target::Scalar,
            "source_address": 1 => Target::Scalar,
            "version": 2 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsIgmpSnoopingQuerier2" => dict {
            "enabled": 0 => Target::Scalar,
            "source_address": 1 => Target::Scalar,
            "version": 2 => Target::Scalar,
            "fast_leave": 3 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsOspf" => dict {
            "enabled": 0 => Target::Scalar,
            "point_to_point": 1 => Target::Scalar,
            "area": 2 => Target::Scalar,
            "cost": 3 => Target::Scalar,
            "authentication": 4 => Target::Scalar,
            "simple_auth_key": 5 => Target::Scalar,
            "cleartext_simple_auth_key": 6 => Target::Scalar,
            "message_digest_keys": 7 => Target::Model("AVDDesignSviProfilesItemsOspfMessageDigestKeysList"),
        };
        "AVDDesignSviProfilesItemsOspfMessageDigestKeysList" => indexed(Target::Model("AVDDesignSviProfilesItemsOspfMessageDigestKeysListIndexedItem"), [0]);
        "AVDDesignSviProfilesItemsOspfMessageDigestKeysItems" => dict {
            "id": 0 => Target::Scalar,
            "hash_algorithm": 1 => Target::Scalar,
            "key": 2 => Target::Scalar,
            "cleartext_key": 3 => Target::Scalar,
        };
        "AVDDesignSviProfilesItemsBgp" => dict {
            "structured_config": 0 => Target::Opaque,
            "raw_eos_cli": 1 => Target::Scalar,
        };
        "AVDDesignTrunkGroups" => dict {
            "mlag": 0 => Target::Model("AVDDesignTrunkGroupsMlag"),
            "mlag_l3": 1 => Target::Model("AVDDesignTrunkGroupsMlagL3"),
            "uplink": 2 => Target::Model("AVDDesignTrunkGroupsUplink"),
        };
        "AVDDesignTrunkGroupsMlag" => dict {
            "name": 0 => Target::Scalar,
        };
        "AVDDesignTrunkGroupsMlagL3" => dict {
            "name": 0 => Target::Scalar,
        };
        "AVDDesignTrunkGroupsUplink" => dict {
            "name": 0 => Target::Scalar,
        };
        "AVDDesignUnderlayMulticastAnycastRp" => dict {
            "mode": 0 => Target::Scalar,
        };
        "AVDDesignUnderlayMulticastRpsList" => indexed(Target::Model("AVDDesignUnderlayMulticastRpsListIndexedItem"), [0]);
        "AVDDesignUnderlayMulticastRpsItems" => dict {
            "rp": 0 => Target::Scalar,
            "nodes": 1 => Target::Model("AVDDesignUnderlayMulticastRpsItemsNodesList"),
            "groups": 2 => Target::Model("AVDDesignUnderlayMulticastRpsItemsGroupsList"),
            "access_list_name": 3 => Target::Scalar,
        };
        "AVDDesignUnderlayMulticastRpsItemsNodesList" => indexed(Target::Model("AVDDesignUnderlayMulticastRpsItemsNodesListIndexedItem"), [0]);
        "AVDDesignUnderlayMulticastRpsItemsNodesItems" => dict {
            "name": 0 => Target::Scalar,
            "loopback_number": 1 => Target::Scalar,
            "description": 2 => Target::Scalar,
        };
        "AVDDesignUnderlayMulticastRpsItemsGroupsList" => list(Target::Scalar);
        "AVDDesignUnderlayOspfAuthentication" => dict {
            "enabled": 0 => Target::Scalar,
            "message_digest_keys": 1 => Target::Model("AVDDesignUnderlayOspfAuthenticationMessageDigestKeysList"),
        };
        "AVDDesignUnderlayOspfAuthenticationMessageDigestKeysList" => indexed(Target::Model("AVDDesignUnderlayOspfAuthenticationMessageDigestKeysListIndexedItem"), [0]);
        "AVDDesignUnderlayOspfAuthenticationMessageDigestKeysItems" => dict {
            "id": 0 => Target::Scalar,
            "hash_algorithm": 1 => Target::Scalar,
            "cleartext_key": 2 => Target::Scalar,
        };
        "AVDDesignUplinkPtp" => dict {
            "enable": 0 => Target::Scalar,
        };
        "AVDDesignValidationProfilesList" => indexed(Target::Model("AVDDesignValidationProfilesListIndexedItem"), [0]);
        "AVDDesignValidationProfilesItems" => dict {
            "name": 0 => Target::Scalar,
            "parent_profile": 1 => Target::Scalar,
            "hardware": 2 => Target::Model("AVDDesignValidationProfilesItemsHardware"),
            "logging": 3 => Target::Model("AVDDesignValidationProfilesItemsLogging"),
            "exclude_as_extra_fabric_validation_target": 4 => Target::Scalar,
            "interfaces": 5 => Target::Model("EosCliConfigGenMetadataInterfaces"),
            "bgp": 6 => Target::Model("EosCliConfigGenMetadataBgp"),
        };
        "AVDDesignValidationProfilesItemsHardware" => dict {
            "min_power_supplies": 0 => Target::Scalar,
            "min_fans": 1 => Target::Scalar,
            "min_supervisors": 2 => Target::Scalar,
            "min_line_cards": 3 => Target::Scalar,
            "min_fabric_cards": 4 => Target::Scalar,
            "transceiver_manufacturers": 5 => Target::Model("AVDDesignValidationProfilesItemsHardwareTransceiverManufacturersList"),
            "ignore_no_transceivers": 6 => Target::Scalar,
        };
        "AVDDesignValidationProfilesItemsHardwareTransceiverManufacturersList" => list(Target::Scalar);
        "AVDDesignValidationProfilesItemsLogging" => dict {
            "validate_no_errors_period": 0 => Target::Scalar,
        };
        "AVDDesignWanCarriersList" => indexed(Target::Model("AVDDesignWanCarriersListIndexedItem"), [0]);
        "AVDDesignWanCarriersItems" => dict {
            "name": 0 => Target::Scalar,
            "description": 1 => Target::Scalar,
            "path_group": 2 => Target::Scalar,
            "trusted": 3 => Target::Scalar,
        };
        "AVDDesignWanHa" => dict {
            "lan_ha_path_group_name": 0 => Target::Scalar,
        };
        "AVDDesignWanIpsecProfiles" => dict {
            "control_plane": 0 => Target::Model("AVDDesignWanIpsecProfilesControlPlane"),
            "data_plane": 1 => Target::Model("AVDDesignWanIpsecProfilesDataPlane"),
        };
        "AVDDesignWanIpsecProfilesControlPlane" => dict {
            "ike_policy_name": 0 => Target::Scalar,
            "sa_policy_name": 1 => Target::Scalar,
            "profile_name": 2 => Target::Scalar,
            "shared_key": 3 => Target::Scalar,
            "cleartext_shared_key": 4 => Target::Scalar,
        };
        "AVDDesignWanIpsecProfilesDataPlane" => dict {
            "ike_policy_name": 0 => Target::Scalar,
            "sa_policy_name": 1 => Target::Scalar,
            "profile_name": 2 => Target::Scalar,
            "shared_key": 3 => Target::Scalar,
            "cleartext_shared_key": 4 => Target::Scalar,
        };
        "AVDDesignWanPathGroupsList" => indexed(Target::Model("AVDDesignWanPathGroupsListIndexedItem"), [0]);
        "AVDDesignWanPathGroupsItems" => dict {
            "name": 0 => Target::Scalar,
            "id": 1 => Target::Scalar,
            "description": 2 => Target::Scalar,
            "ipsec": 3 => Target::Model("AVDDesignWanPathGroupsItemsIpsec"),
            "import_path_groups": 4 => Target::Model("AVDDesignWanPathGroupsItemsImportPathGroupsList"),
            "default_preference": 5 => Target::Scalar,
            "excluded_from_default_policy": 6 => Target::Scalar,
            "dps_keepalive": 7 => Target::Model("AVDDesignWanPathGroupsItemsDpsKeepalive"),
        };
        "AVDDesignWanPathGroupsItemsIpsec" => dict {
            "dynamic_peers": 0 => Target::Scalar,
            "static_peers": 1 => Target::Scalar,
        };
        "AVDDesignWanPathGroupsItemsImportPathGroupsList" => list(Target::Model("AVDDesignWanPathGroupsItemsImportPathGroupsItems"));
        "AVDDesignWanPathGroupsItemsImportPathGroupsItems" => dict {
            "remote": 0 => Target::Scalar,
            "local": 1 => Target::Scalar,
        };
        "AVDDesignWanPathGroupsItemsDpsKeepalive" => dict {
            "interval": 0 => Target::Scalar,
            "failure_threshold": 1 => Target::Scalar,
        };
        "AVDDesignWanRouteServersList" => indexed(Target::Model("AVDDesignWanRouteServersListIndexedItem"), [0]);
        "AVDDesignWanRouteServersItems" => dict {
            "hostname": 0 => Target::Scalar,
            "vtep_ip": 1 => Target::Scalar,
            "path_groups": 2 => Target::Model("AVDDesignWanRouteServersItemsPathGroupsList"),
        };
        "AVDDesignWanRouteServersItemsPathGroupsList" => indexed(Target::Model("AVDDesignWanRouteServersItemsPathGroupsListIndexedItem"), [0]);
        "AVDDesignWanRouteServersItemsPathGroupsItems" => dict {
            "name": 0 => Target::Scalar,
            "interfaces": 1 => Target::Model("AVDDesignWanRouteServersItemsPathGroupsItemsInterfacesList"),
        };
        "AVDDesignWanRouteServersItemsPathGroupsItemsInterfacesList" => indexed(Target::Model("AVDDesignWanRouteServersItemsPathGroupsItemsInterfacesListIndexedItem"), [0]);
        "AVDDesignWanRouteServersItemsPathGroupsItemsInterfacesItems" => dict {
            "name": 0 => Target::Scalar,
            "public_ip": 1 => Target::Scalar,
        };
        "AVDDesignWanVirtualTopologies" => dict {
            "vrfs": 0 => Target::Model("AVDDesignWanVirtualTopologiesVrfsList"),
            "control_plane_virtual_topology": 1 => Target::Model("AVDDesignWanVirtualTopologiesControlPlaneVirtualTopology"),
            "policies": 2 => Target::Model("AVDDesignWanVirtualTopologiesPoliciesList"),
        };
        "AVDDesignWanVirtualTopologiesVrfsList" => indexed(Target::Model("AVDDesignWanVirtualTopologiesVrfsListIndexedItem"), [0]);
        "AVDDesignWanVirtualTopologiesVrfsItems" => dict {
            "name": 0 => Target::Scalar,
            "policy": 1 => Target::Scalar,
            "wan_vni": 2 => Target::Scalar,
        };
        "AVDDesignWanVirtualTopologiesControlPlaneVirtualTopology" => dict {
            "name": 0 => Target::Scalar,
            "application_profile": 1 => Target::Scalar,
            "traffic_class": 2 => Target::Scalar,
            "dscp": 3 => Target::Scalar,
            "lowest_hop_count": 4 => Target::Scalar,
            "constraints": 5 => Target::Model("AVDDesignWanVirtualTopologiesControlPlaneVirtualTopologyConstraints"),
            "outlier_elimination": 6 => Target::Model("EosCliConfigGenRouterAdaptiveVirtualTopologyProfilesItemsOutlierElimination"),
            "metric_order": 7 => Target::Model("EosCliConfigGenRouterAdaptiveVirtualTopologyProfilesItemsMetricOrder"),
            "path_groups": 8 => Target::Model("AVDDesignWanVirtualTopologiesControlPlaneVirtualTopologyPathGroupsList"),
            "internet_exit": 9 => Target::Model("AVDDesignWanVirtualTopologiesControlPlaneVirtualTopologyInternetExit"),
        };
        "AVDDesignWanVirtualTopologiesControlPlaneVirtualTopologyConstraints" => dict {
            "jitter": 0 => Target::Scalar,
            "latency": 1 => Target::Scalar,
            "loss_rate": 2 => Target::Scalar,
        };
        "AVDDesignWanVirtualTopologiesControlPlaneVirtualTopologyPathGroupsList" => list(Target::Model("AVDDesignWanVirtualTopologiesControlPlaneVirtualTopologyPathGroupsItems"));
        "AVDDesignWanVirtualTopologiesControlPlaneVirtualTopologyPathGroupsItems" => dict {
            "names": 0 => Target::Model("AVDDesignWanVirtualTopologiesControlPlaneVirtualTopologyPathGroupsItemsNamesList"),
            "preference": 1 => Target::Scalar,
        };
        "AVDDesignWanVirtualTopologiesControlPlaneVirtualTopologyPathGroupsItemsNamesList" => list(Target::Scalar);
        "AVDDesignWanVirtualTopologiesControlPlaneVirtualTopologyInternetExit" => dict {
            "policy": 0 => Target::Scalar,
        };
        "AVDDesignWanVirtualTopologiesPoliciesList" => indexed(Target::Model("AVDDesignWanVirtualTopologiesPoliciesListIndexedItem"), [0]);
        "AVDDesignWanVirtualTopologiesPoliciesItems" => dict {
            "name": 0 => Target::Scalar,
            "application_virtual_topologies": 1 => Target::Model("AVDDesignWanVirtualTopologiesPoliciesItemsApplicationVirtualTopologiesList"),
            "default_virtual_topology": 2 => Target::Model("AVDDesignWanVirtualTopologiesPoliciesItemsDefaultVirtualTopology"),
        };
        "AVDDesignWanVirtualTopologiesPoliciesItemsApplicationVirtualTopologiesList" => indexed(Target::Model("AVDDesignWanVirtualTopologiesPoliciesItemsApplicationVirtualTopologiesListIndexedItem"), [0]);
        "AVDDesignWanVirtualTopologiesPoliciesItemsApplicationVirtualTopologiesItems" => dict {
            "application_profile": 0 => Target::Scalar,
            "name": 1 => Target::Scalar,
            "id": 2 => Target::Scalar,
            "traffic_class": 3 => Target::Scalar,
            "dscp": 4 => Target::Scalar,
            "lowest_hop_count": 5 => Target::Scalar,
            "constraints": 6 => Target::Model("AVDDesignWanVirtualTopologiesPoliciesItemsApplicationVirtualTopologiesItemsConstraints"),
            "outlier_elimination": 7 => Target::Model("EosCliConfigGenRouterAdaptiveVirtualTopologyProfilesItemsOutlierElimination"),
            "metric_order": 8 => Target::Model("EosCliConfigGenRouterAdaptiveVirtualTopologyProfilesItemsMetricOrder"),
            "path_groups": 9 => Target::Model("AVDDesignWanVirtualTopologiesPoliciesItemsApplicationVirtualTopologiesItemsPathGroupsList"),
            "internet_exit": 10 => Target::Model("AVDDesignWanVirtualTopologiesPoliciesItemsApplicationVirtualTopologiesItemsInternetExit"),
        };
        "AVDDesignWanVirtualTopologiesPoliciesItemsApplicationVirtualTopologiesItemsConstraints" => dict {
            "jitter": 0 => Target::Scalar,
            "latency": 1 => Target::Scalar,
            "loss_rate": 2 => Target::Scalar,
        };
        "AVDDesignWanVirtualTopologiesPoliciesItemsApplicationVirtualTopologiesItemsPathGroupsList" => list(Target::Model("AVDDesignWanVirtualTopologiesPoliciesItemsApplicationVirtualTopologiesItemsPathGroupsItems"));
        "AVDDesignWanVirtualTopologiesPoliciesItemsApplicationVirtualTopologiesItemsPathGroupsItems" => dict {
            "names": 0 => Target::Model("AVDDesignWanVirtualTopologiesPoliciesItemsApplicationVirtualTopologiesItemsPathGroupsItemsNamesList"),
            "preference": 1 => Target::Scalar,
        };
        "AVDDesignWanVirtualTopologiesPoliciesItemsApplicationVirtualTopologiesItemsPathGroupsItemsNamesList" => list(Target::Scalar);
        "AVDDesignWanVirtualTopologiesPoliciesItemsApplicationVirtualTopologiesItemsInternetExit" => dict {
            "policy": 0 => Target::Scalar,
        };
        "AVDDesignWanVirtualTopologiesPoliciesItemsDefaultVirtualTopology" => dict {
            "name": 0 => Target::Scalar,
            "drop_unmatched": 1 => Target::Scalar,
            "traffic_class": 2 => Target::Scalar,
            "dscp": 3 => Target::Scalar,
            "lowest_hop_count": 4 => Target::Scalar,
            "constraints": 5 => Target::Model("AVDDesignWanVirtualTopologiesPoliciesItemsDefaultVirtualTopologyConstraints"),
            "outlier_elimination": 6 => Target::Model("EosCliConfigGenRouterAdaptiveVirtualTopologyProfilesItemsOutlierElimination"),
            "metric_order": 7 => Target::Model("EosCliConfigGenRouterAdaptiveVirtualTopologyProfilesItemsMetricOrder"),
            "path_groups": 8 => Target::Model("AVDDesignWanVirtualTopologiesPoliciesItemsDefaultVirtualTopologyPathGroupsList"),
            "internet_exit": 9 => Target::Model("AVDDesignWanVirtualTopologiesPoliciesItemsDefaultVirtualTopologyInternetExit"),
        };
        "AVDDesignWanVirtualTopologiesPoliciesItemsDefaultVirtualTopologyConstraints" => dict {
            "jitter": 0 => Target::Scalar,
            "latency": 1 => Target::Scalar,
            "loss_rate": 2 => Target::Scalar,
        };
        "AVDDesignWanVirtualTopologiesPoliciesItemsDefaultVirtualTopologyPathGroupsList" => list(Target::Model("AVDDesignWanVirtualTopologiesPoliciesItemsDefaultVirtualTopologyPathGroupsItems"));
        "AVDDesignWanVirtualTopologiesPoliciesItemsDefaultVirtualTopologyPathGroupsItems" => dict {
            "names": 0 => Target::Model("AVDDesignWanVirtualTopologiesPoliciesItemsDefaultVirtualTopologyPathGroupsItemsNamesList"),
            "preference": 1 => Target::Scalar,
        };
        "AVDDesignWanVirtualTopologiesPoliciesItemsDefaultVirtualTopologyPathGroupsItemsNamesList" => list(Target::Scalar);
        "AVDDesignWanVirtualTopologiesPoliciesItemsDefaultVirtualTopologyInternetExit" => dict {
            "policy": 0 => Target::Scalar,
        };
        "AVDDesignZscalerEndpoints" => dict {
            "primary": 0 => Target::Model("AVDDesignZscalerEndpointsPrimary"),
            "secondary": 1 => Target::Model("AVDDesignZscalerEndpointsSecondary"),
            "tertiary": 2 => Target::Model("AVDDesignZscalerEndpointsTertiary"),
            "cloud_name": 3 => Target::Scalar,
            "device_location": 4 => Target::Model("AVDDesignZscalerEndpointsDeviceLocation"),
        };
        "AVDDesignZscalerEndpointsPrimary" => dict {
            "ip_address": 0 => Target::Scalar,
            "datacenter": 1 => Target::Scalar,
            "city": 2 => Target::Scalar,
            "country": 3 => Target::Scalar,
            "region": 4 => Target::Scalar,
            "latitude": 5 => Target::Scalar,
            "longitude": 6 => Target::Scalar,
        };
        "AVDDesignZscalerEndpointsSecondary" => dict {
            "ip_address": 0 => Target::Scalar,
            "datacenter": 1 => Target::Scalar,
            "city": 2 => Target::Scalar,
            "country": 3 => Target::Scalar,
            "region": 4 => Target::Scalar,
            "latitude": 5 => Target::Scalar,
            "longitude": 6 => Target::Scalar,
        };
        "AVDDesignZscalerEndpointsTertiary" => dict {
            "ip_address": 0 => Target::Scalar,
            "datacenter": 1 => Target::Scalar,
            "city": 2 => Target::Scalar,
            "country": 3 => Target::Scalar,
            "region": 4 => Target::Scalar,
            "latitude": 5 => Target::Scalar,
            "longitude": 6 => Target::Scalar,
        };
        "AVDDesignZscalerEndpointsDeviceLocation" => dict {
            "city": 0 => Target::Scalar,
            "country": 1 => Target::Scalar,
        };
        "EosCliConfigGenApplicationTrafficRecognitionCategoriesListIndexedItem" => alias("EosCliConfigGenApplicationTrafficRecognitionCategoriesItems");
        "EosCliConfigGenApplicationTrafficRecognitionFieldSetsL4PortsListIndexedItem" => alias("EosCliConfigGenApplicationTrafficRecognitionFieldSetsL4PortsItems");
        "EosCliConfigGenApplicationTrafficRecognitionFieldSetsIpv4PrefixesListIndexedItem" => alias("EosCliConfigGenApplicationTrafficRecognitionFieldSetsIpv4PrefixesItems");
        "EosCliConfigGenApplicationTrafficRecognitionApplicationsIpv4ApplicationsListIndexedItem" => alias("EosCliConfigGenApplicationTrafficRecognitionApplicationsIpv4ApplicationsItems");
        "EosCliConfigGenApplicationTrafficRecognitionApplicationsL4ApplicationsListIndexedItem" => alias("EosCliConfigGenApplicationTrafficRecognitionApplicationsL4ApplicationsItems");
        "EosCliConfigGenApplicationTrafficRecognitionApplicationProfilesListIndexedItem" => alias("EosCliConfigGenApplicationTrafficRecognitionApplicationProfilesItems");
        "EosCliConfigGenEventHandlersListIndexedItem" => alias("EosCliConfigGenEventHandlersItems");
        "EosCliConfigGenIpHostsListIndexedItem" => alias("EosCliConfigGenIpHostsItems");
        "EosCliConfigGenLoggingPolicyMatchMatchListsListIndexedItem" => alias("EosCliConfigGenLoggingPolicyMatchMatchListsItems");
        "EosCliConfigGenLoggingLevelListIndexedItem" => alias("EosCliConfigGenLoggingLevelItems");
        "EosCliConfigGenPeerFiltersListIndexedItem" => alias("EosCliConfigGenPeerFiltersItems");
        "EosCliConfigGenPeerFiltersItemsSequenceNumbersListIndexedItem" => alias("EosCliConfigGenPeerFiltersItemsSequenceNumbersItems");
        "EosCliConfigGenTcamProfileProfilesListIndexedItem" => alias("EosCliConfigGenTcamProfileProfilesItems");
        "AVDDesignAaaSettingsTacacsVrfsListIndexedItem" => alias("AVDDesignAaaSettingsTacacsVrfsItems");
        "AVDDesignAaaSettingsRadiusVrfsListIndexedItem" => alias("AVDDesignAaaSettingsRadiusVrfsItems");
        "AVDDesignAaaSettingsLocalUsersListIndexedItem" => alias("AVDDesignAaaSettingsLocalUsersItems");
        "AVDDesignConnectedEndpointsListIndexedItem" => alias("AVDDesignConnectedEndpointsItems");
        "AVDDesignConnectedEndpointsItemsAdaptersItemsSubinterfacesListIndexedItem" => alias("AVDDesignConnectedEndpointsItemsAdaptersItemsSubinterfacesItems");
        "AVDDesignConnectedEndpointsItemsAdaptersItemsPortChannelSubinterfacesListIndexedItem" => alias("AVDDesignConnectedEndpointsItemsAdaptersItemsPortChannelSubinterfacesItems");
        "AVDDesignCustomConnectedEndpointsKeysListIndexedItem" => alias("AVDDesignCustomConnectedEndpointsKeysItems");
        "AVDDesignConnectedEndpointsKeysListIndexedItem" => alias("AVDDesignConnectedEndpointsKeysItems");
        "AVDDesignCoreInterfacesP2pLinksIpPoolsListIndexedItem" => alias("AVDDesignCoreInterfacesP2pLinksIpPoolsItems");
        "AVDDesignCoreInterfacesP2pLinksProfilesListIndexedItem" => alias("AVDDesignCoreInterfacesP2pLinksProfilesItems");
        "AVDDesignCoreInterfacesP2pLinksProfilesItemsPortChannelNodesChildInterfacesListIndexedItem" => alias("AVDDesignCoreInterfacesP2pLinksProfilesItemsPortChannelNodesChildInterfacesItems");
        "AVDDesignCoreInterfacesP2pLinksItemsPortChannelNodesChildInterfacesListIndexedItem" => alias("AVDDesignCoreInterfacesP2pLinksItemsPortChannelNodesChildInterfacesItems");
        "AVDDesignCvPathfinderGlobalSitesListIndexedItem" => alias("AVDDesignCvPathfinderGlobalSitesItems");
        "AVDDesignCvPathfinderInternetExitPoliciesListIndexedItem" => alias("AVDDesignCvPathfinderInternetExitPoliciesItems");
        "AVDDesignCvPathfinderRegionsListIndexedItem" => alias("AVDDesignCvPathfinderRegionsItems");
        "AVDDesignCvPathfinderRegionsItemsSitesListIndexedItem" => alias("AVDDesignCvPathfinderRegionsItemsSitesItems");
        "AVDDesignCvSettingsCvaasClustersListIndexedItem" => alias("AVDDesignCvSettingsCvaasClustersItems");
        "AVDDesignCvSettingsOnpremClustersListIndexedItem" => alias("AVDDesignCvSettingsOnpremClustersItems");
        "AVDDesignCvSettingsOnpremClustersItemsServersListIndexedItem" => alias("AVDDesignCvSettingsOnpremClustersItemsServersItems");
        "AVDDesignCvTopologyListIndexedItem" => alias("AVDDesignCvTopologyItems");
        "AVDDesignCvTopologyItemsInterfacesListIndexedItem" => alias("AVDDesignCvTopologyItemsInterfacesItems");
        "AVDDesignCvTopologyLevelsListIndexedItem" => alias("AVDDesignCvTopologyLevelsItems");
        "AVDDesignDefaultNodeTypesListIndexedItem" => alias("AVDDesignDefaultNodeTypesItems");
        "AVDDesignDeviceProfilesListIndexedItem" => alias("AVDDesignDeviceProfilesItems");
        "AVDDesignDeviceProfilesItemsLinkTrackingGroupsListIndexedItem" => alias("AVDDesignDeviceProfilesItemsLinkTrackingGroupsItems");
        "AVDDesignDeviceProfilesItemsEvpnGatewayRemotePeersListIndexedItem" => alias("AVDDesignDeviceProfilesItemsEvpnGatewayRemotePeersItems");
        "AVDDesignDeviceProfilesItemsIpvpnGatewayRemotePeersListIndexedItem" => alias("AVDDesignDeviceProfilesItemsIpvpnGatewayRemotePeersItems");
        "AVDDesignDeviceProfilesItemsL3InterfacesListIndexedItem" => alias("AVDDesignDeviceProfilesItemsL3InterfacesItems");
        "AVDDesignDeviceProfilesItemsL3InterfacesItemsCvPathfinderInternetExitPoliciesListIndexedItem" => alias("AVDDesignDeviceProfilesItemsL3InterfacesItemsCvPathfinderInternetExitPoliciesItems");
        "AVDDesignDeviceProfilesItemsL3PortChannelsListIndexedItem" => alias("AVDDesignDeviceProfilesItemsL3PortChannelsItems");
        "AVDDesignDeviceProfilesItemsL3PortChannelsItemsMemberInterfacesListIndexedItem" => alias("AVDDesignDeviceProfilesItemsL3PortChannelsItemsMemberInterfacesItems");
        "AVDDesignDeviceProfilesItemsL3PortChannelsItemsStaticRoutesListIndexedItem" => alias("AVDDesignDeviceProfilesItemsL3PortChannelsItemsStaticRoutesItems");
        "AVDDesignDevicesListIndexedItem" => alias("AVDDesignDevicesItems");
        "AVDDesignDevicesItemsLinkTrackingGroupsListIndexedItem" => alias("AVDDesignDevicesItemsLinkTrackingGroupsItems");
        "AVDDesignDevicesItemsEvpnGatewayRemotePeersListIndexedItem" => alias("AVDDesignDevicesItemsEvpnGatewayRemotePeersItems");
        "AVDDesignDevicesItemsIpvpnGatewayRemotePeersListIndexedItem" => alias("AVDDesignDevicesItemsIpvpnGatewayRemotePeersItems");
        "AVDDesignDevicesItemsL3InterfacesListIndexedItem" => alias("AVDDesignDevicesItemsL3InterfacesItems");
        "AVDDesignDevicesItemsL3InterfacesItemsCvPathfinderInternetExitPoliciesListIndexedItem" => alias("AVDDesignDevicesItemsL3InterfacesItemsCvPathfinderInternetExitPoliciesItems");
        "AVDDesignDevicesItemsL3PortChannelsListIndexedItem" => alias("AVDDesignDevicesItemsL3PortChannelsItems");
        "AVDDesignDevicesItemsL3PortChannelsItemsMemberInterfacesListIndexedItem" => alias("AVDDesignDevicesItemsL3PortChannelsItemsMemberInterfacesItems");
        "AVDDesignDevicesItemsL3PortChannelsItemsStaticRoutesListIndexedItem" => alias("AVDDesignDevicesItemsL3PortChannelsItemsStaticRoutesItems");
        "AVDDesignDnsSettingsVrfsListIndexedItem" => alias("AVDDesignDnsSettingsVrfsItems");
        "AVDDesignEvpnVlanBundlesListIndexedItem" => alias("AVDDesignEvpnVlanBundlesItems");
        "AVDDesignFlowTrackingSettingsTrackersListIndexedItem" => alias("AVDDesignFlowTrackingSettingsTrackersItems");
        "AVDDesignFlowTrackingSettingsTrackersItemsExportersListIndexedItem" => alias("AVDDesignFlowTrackingSettingsTrackersItemsExportersItems");
        "AVDDesignFlowTrackingSettingsTrackersItemsExportersItemsCollectorsListIndexedItem" => alias("AVDDesignFlowTrackingSettingsTrackersItemsExportersItemsCollectorsItems");
        "AVDDesignGeneralSettingsSuspendedVlansListIndexedItem" => alias("AVDDesignGeneralSettingsSuspendedVlansItems");
        "AVDDesignGenerateCvTagsInterfaceTagsListIndexedItem" => alias("AVDDesignGenerateCvTagsInterfaceTagsItems");
        "AVDDesignIpv4AclsListIndexedItem" => alias("AVDDesignIpv4AclsItems");
        "AVDDesignIpv4PrefixListCatalogListIndexedItem" => alias("AVDDesignIpv4PrefixListCatalogItems");
        "AVDDesignIpv4PrefixListCatalogItemsSequenceNumbersListIndexedItem" => alias("AVDDesignIpv4PrefixListCatalogItemsSequenceNumbersItems");
        "AVDDesignIpv4StandardAclsListIndexedItem" => alias("AVDDesignIpv4StandardAclsItems");
        "AVDDesignIpv6AclsListIndexedItem" => alias("AVDDesignIpv6AclsItems");
        "AVDDesignIpv6AclsItemsSequenceNumbersListIndexedItem" => alias("AVDDesignIpv6AclsItemsSequenceNumbersItems");
        "AVDDesignIpv6PrefixListCatalogListIndexedItem" => alias("AVDDesignIpv6PrefixListCatalogItems");
        "AVDDesignIpv6PrefixListCatalogItemsSequenceNumbersListIndexedItem" => alias("AVDDesignIpv6PrefixListCatalogItemsSequenceNumbersItems");
        "AVDDesignL2vlanProfilesListIndexedItem" => alias("AVDDesignL2vlanProfilesItems");
        "AVDDesignL3EdgeP2pLinksIpPoolsListIndexedItem" => alias("AVDDesignL3EdgeP2pLinksIpPoolsItems");
        "AVDDesignL3EdgeP2pLinksProfilesListIndexedItem" => alias("AVDDesignL3EdgeP2pLinksProfilesItems");
        "AVDDesignL3EdgeP2pLinksProfilesItemsPortChannelNodesChildInterfacesListIndexedItem" => alias("AVDDesignL3EdgeP2pLinksProfilesItemsPortChannelNodesChildInterfacesItems");
        "AVDDesignL3EdgeP2pLinksItemsPortChannelNodesChildInterfacesListIndexedItem" => alias("AVDDesignL3EdgeP2pLinksItemsPortChannelNodesChildInterfacesItems");
        "AVDDesignL3InterfaceProfilesListIndexedItem" => alias("AVDDesignL3InterfaceProfilesItems");
        "AVDDesignL3InterfaceProfilesItemsCvPathfinderInternetExitPoliciesListIndexedItem" => alias("AVDDesignL3InterfaceProfilesItemsCvPathfinderInternetExitPoliciesItems");
        "AVDDesignLoggingSettingsVrfsListIndexedItem" => alias("AVDDesignLoggingSettingsVrfsItems");
        "AVDDesignMacAclsListIndexedItem" => alias("AVDDesignMacAclsItems");
        "AVDDesignManagementEapiVrfsListIndexedItem" => alias("AVDDesignManagementEapiVrfsItems");
        "AVDDesignMonitorConnectivityInterfaceSetsListIndexedItem" => alias("AVDDesignMonitorConnectivityInterfaceSetsItems");
        "AVDDesignMonitorConnectivityHostsListIndexedItem" => alias("AVDDesignMonitorConnectivityHostsItems");
        "AVDDesignMonitorConnectivityVrfsListIndexedItem" => alias("AVDDesignMonitorConnectivityVrfsItems");
        "AVDDesignMonitorConnectivityVrfsItemsInterfaceSetsListIndexedItem" => alias("AVDDesignMonitorConnectivityVrfsItemsInterfaceSetsItems");
        "AVDDesignMonitorConnectivityVrfsItemsHostsListIndexedItem" => alias("AVDDesignMonitorConnectivityVrfsItemsHostsItems");
        "AVDDesignNetworkServicesListIndexedItem" => alias("AVDDesignNetworkServicesItems");
        "AVDDesignNetworkServicesItemsBgpPeerGroupsListIndexedItem" => alias("AVDDesignNetworkServicesItemsBgpPeerGroupsItems");
        "AVDDesignNetworkServicesItemsVrfsListIndexedItem" => alias("AVDDesignNetworkServicesItemsVrfsItems");
        "AVDDesignNetworkServicesItemsVrfsItemsIpHelpersListIndexedItem" => alias("AVDDesignNetworkServicesItemsVrfsItemsIpHelpersItems");
        "AVDDesignNetworkServicesItemsVrfsItemsVtepDiagnosticLoopbackIpPoolsListIndexedItem" => alias("AVDDesignNetworkServicesItemsVrfsItemsVtepDiagnosticLoopbackIpPoolsItems");
        "AVDDesignNetworkServicesItemsVrfsItemsOspfMessageDigestKeysListIndexedItem" => alias("AVDDesignNetworkServicesItemsVrfsItemsOspfMessageDigestKeysItems");
        "AVDDesignNetworkServicesItemsVrfsItemsSvisListKeyedItem" => alias("AVDDesignNetworkServicesItemsVrfsItemsSvisItems");
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesListIndexedItem" => alias("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItems");
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6NdRaDnsServersServersListIndexedItem" => alias("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6NdRaDnsServersServersItems");
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6DhcpRelayDestinationsListIndexedItem" => alias("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpv6DhcpRelayDestinationsItems");
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpHelpersListIndexedItem" => alias("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsIpHelpersItems");
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsOspfMessageDigestKeysListIndexedItem" => alias("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsNodesItemsOspfMessageDigestKeysItems");
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6NdRaDnsServersServersListIndexedItem" => alias("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6NdRaDnsServersServersItems");
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6DhcpRelayDestinationsListIndexedItem" => alias("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpv6DhcpRelayDestinationsItems");
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpHelpersListIndexedItem" => alias("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsIpHelpersItems");
        "AVDDesignNetworkServicesItemsVrfsItemsSvisItemsOspfMessageDigestKeysListIndexedItem" => alias("AVDDesignNetworkServicesItemsVrfsItemsSvisItemsOspfMessageDigestKeysItems");
        "AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsOspfMessageDigestKeysListIndexedItem" => alias("AVDDesignNetworkServicesItemsVrfsItemsL3InterfacesItemsOspfMessageDigestKeysItems");
        "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsMemberInterfacesListIndexedItem" => alias("AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsMemberInterfacesItems");
        "AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsOspfMessageDigestKeysListIndexedItem" => alias("AVDDesignNetworkServicesItemsVrfsItemsL3PortChannelsItemsOspfMessageDigestKeysItems");
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeersListKeyedItem" => alias("AVDDesignNetworkServicesItemsVrfsItemsBgpPeersItems");
        "AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsListIndexedItem" => alias("AVDDesignNetworkServicesItemsVrfsItemsBgpPeerGroupsItems");
        "AVDDesignNetworkServicesItemsL2vlansListKeyedItem" => alias("AVDDesignNetworkServicesItemsL2vlansItems");
        "AVDDesignNetworkServicesItemsPointToPointServicesListIndexedItem" => alias("AVDDesignNetworkServicesItemsPointToPointServicesItems");
        "AVDDesignNetworkServicesItemsPointToPointServicesItemsSubinterfacesListIndexedItem" => alias("AVDDesignNetworkServicesItemsPointToPointServicesItemsSubinterfacesItems");
        "AVDDesignNetworkServicesKeysListIndexedItem" => alias("AVDDesignNetworkServicesKeysItems");
        "AVDDesignCustomNodeTypeKeysListIndexedItem" => alias("AVDDesignCustomNodeTypeKeysItems");
        "AVDDesignNodeTypeKeysListIndexedItem" => alias("AVDDesignNodeTypeKeysItems");
        "AVDDesignNtpSettingsServersListIndexedItem" => alias("AVDDesignNtpSettingsServersItems");
        "AVDDesignNtpSettingsAuthenticationKeysListIndexedItem" => alias("AVDDesignNtpSettingsAuthenticationKeysItems");
        "AVDDesignPlatformSpeedGroupsListIndexedItem" => alias("AVDDesignPlatformSpeedGroupsItems");
        "AVDDesignPlatformSpeedGroupsItemsSpeedsListIndexedItem" => alias("AVDDesignPlatformSpeedGroupsItemsSpeedsItems");
        "AVDDesignPortProfilesListIndexedItem" => alias("AVDDesignPortProfilesItems");
        "AVDDesignPortProfilesItemsPortChannelSubinterfacesListIndexedItem" => alias("AVDDesignPortProfilesItemsPortChannelSubinterfacesItems");
        "AVDDesignPtpProfilesListIndexedItem" => alias("AVDDesignPtpProfilesItems");
        "AVDDesignSflowSettingsVrfsListIndexedItem" => alias("AVDDesignSflowSettingsVrfsItems");
        "AVDDesignSnmpSettingsVrfsListIndexedItem" => alias("AVDDesignSnmpSettingsVrfsItems");
        "AVDDesignSnmpSettingsCommunitiesListIndexedItem" => alias("AVDDesignSnmpSettingsCommunitiesItems");
        "AVDDesignSshSettingsVrfsListIndexedItem" => alias("AVDDesignSshSettingsVrfsItems");
        "AVDDesignSshSettingsClientVrfsListIndexedItem" => alias("AVDDesignSshSettingsClientVrfsItems");
        "AVDDesignSviProfilesListIndexedItem" => alias("AVDDesignSviProfilesItems");
        "AVDDesignSviProfilesItemsNodesListIndexedItem" => alias("AVDDesignSviProfilesItemsNodesItems");
        "AVDDesignSviProfilesItemsNodesItemsIpv6NdRaDnsServersServersListIndexedItem" => alias("AVDDesignSviProfilesItemsNodesItemsIpv6NdRaDnsServersServersItems");
        "AVDDesignSviProfilesItemsNodesItemsIpv6DhcpRelayDestinationsListIndexedItem" => alias("AVDDesignSviProfilesItemsNodesItemsIpv6DhcpRelayDestinationsItems");
        "AVDDesignSviProfilesItemsNodesItemsIpHelpersListIndexedItem" => alias("AVDDesignSviProfilesItemsNodesItemsIpHelpersItems");
        "AVDDesignSviProfilesItemsNodesItemsOspfMessageDigestKeysListIndexedItem" => alias("AVDDesignSviProfilesItemsNodesItemsOspfMessageDigestKeysItems");
        "AVDDesignSviProfilesItemsIpv6NdRaDnsServersServersListIndexedItem" => alias("AVDDesignSviProfilesItemsIpv6NdRaDnsServersServersItems");
        "AVDDesignSviProfilesItemsIpv6DhcpRelayDestinationsListIndexedItem" => alias("AVDDesignSviProfilesItemsIpv6DhcpRelayDestinationsItems");
        "AVDDesignSviProfilesItemsIpHelpersListIndexedItem" => alias("AVDDesignSviProfilesItemsIpHelpersItems");
        "AVDDesignSviProfilesItemsOspfMessageDigestKeysListIndexedItem" => alias("AVDDesignSviProfilesItemsOspfMessageDigestKeysItems");
        "AVDDesignUnderlayMulticastRpsListIndexedItem" => alias("AVDDesignUnderlayMulticastRpsItems");
        "AVDDesignUnderlayMulticastRpsItemsNodesListIndexedItem" => alias("AVDDesignUnderlayMulticastRpsItemsNodesItems");
        "AVDDesignUnderlayOspfAuthenticationMessageDigestKeysListIndexedItem" => alias("AVDDesignUnderlayOspfAuthenticationMessageDigestKeysItems");
        "AVDDesignValidationProfilesListIndexedItem" => alias("AVDDesignValidationProfilesItems");
        "AVDDesignWanCarriersListIndexedItem" => alias("AVDDesignWanCarriersItems");
        "AVDDesignWanPathGroupsListIndexedItem" => alias("AVDDesignWanPathGroupsItems");
        "AVDDesignWanRouteServersListIndexedItem" => alias("AVDDesignWanRouteServersItems");
        "AVDDesignWanRouteServersItemsPathGroupsListIndexedItem" => alias("AVDDesignWanRouteServersItemsPathGroupsItems");
        "AVDDesignWanRouteServersItemsPathGroupsItemsInterfacesListIndexedItem" => alias("AVDDesignWanRouteServersItemsPathGroupsItemsInterfacesItems");
        "AVDDesignWanVirtualTopologiesVrfsListIndexedItem" => alias("AVDDesignWanVirtualTopologiesVrfsItems");
        "AVDDesignWanVirtualTopologiesPoliciesListIndexedItem" => alias("AVDDesignWanVirtualTopologiesPoliciesItems");
        "AVDDesignWanVirtualTopologiesPoliciesItemsApplicationVirtualTopologiesListIndexedItem" => alias("AVDDesignWanVirtualTopologiesPoliciesItemsApplicationVirtualTopologiesItems");
    }
}
