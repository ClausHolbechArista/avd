// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub parent_profile: ::validated_data::Field<&'a str>,
    #[data_view(rename = "type")]
    pub field_type: ::validated_data::Field<&'a str>,
    pub mlag_group: ::validated_data::Field<&'a str>,
    pub id: ::validated_data::Field<i64>,
    pub platform: ::validated_data::Field<&'a str>,
    pub mac_address: ::validated_data::Field<&'a str>,
    pub system_mac_address: ::validated_data::Field<&'a str>,
    pub custom_system_mac_address: ::validated_data::Field<&'a str>,
    pub serial_number: ::validated_data::Field<&'a str>,
    pub rack: ::validated_data::Field<&'a str>,
    pub mgmt_ip: ::validated_data::Field<&'a str>,
    pub mgmt_gateway: ::validated_data::Field<&'a str>,
    pub ipv6_mgmt_ip: ::validated_data::Field<&'a str>,
    pub ipv6_mgmt_gateway: ::validated_data::Field<&'a str>,
    pub mgmt_interface: ::validated_data::Field<&'a str>,
    pub link_tracking: ::validated_data::Field<item::LinkTracking<'a, Mode>>,
    pub lacp_port_id_range: ::validated_data::Field<item::LacpPortIdRange<'a, Mode>>,
    pub always_configure_ip_routing: ::validated_data::Field<bool>,
    pub raw_eos_cli: ::validated_data::Field<&'a str>,
    #[data_view(relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_cli_config_gen::EosCliConfigGen<'a, ::validated_data::RelaxedValidated>>,
    pub uplink_type: ::validated_data::Field<&'a str>,
    pub uplink_ipv4_pool: ::validated_data::Field<&'a str>,
    pub uplink_ipv6_pool: ::validated_data::Field<&'a str>,
    pub uplink_interfaces: ::validated_data::Field<item::UplinkInterfaces<'a, Mode>>,
    pub uplink_switch_interfaces: ::validated_data::Field<item::UplinkSwitchInterfaces<'a, Mode>>,
    pub uplink_switches: ::validated_data::Field<item::UplinkSwitches<'a, Mode>>,
    pub uplink_interface_speed: ::validated_data::Field<&'a str>,
    pub uplink_switch_interface_speed: ::validated_data::Field<&'a str>,
    pub uplink_mtu: ::validated_data::Field<i64>,
    pub max_uplink_switches: ::validated_data::Field<i64>,
    pub max_parallel_uplinks: ::validated_data::Field<i64>,
    pub uplink_bfd: ::validated_data::Field<bool>,
    pub uplink_native_vlan: ::validated_data::Field<i64>,
    pub uplink_ptp: ::validated_data::Field<item::UplinkPtp<'a, Mode>>,
    pub uplink_macsec: ::validated_data::Field<item::UplinkMacsec<'a, Mode>>,
    pub uplink_port_channel_id: ::validated_data::Field<i64>,
    pub uplink_switch_port_channel_id: ::validated_data::Field<i64>,
    #[data_view(relaxed)]
    pub uplink_ethernet_structured_config: ::validated_data::Field<super::super::eos_cli_config_gen::ethernet_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
    #[data_view(relaxed)]
    pub uplink_port_channel_structured_config: ::validated_data::Field<super::super::eos_cli_config_gen::port_channel_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
    #[data_view(relaxed)]
    pub uplink_switch_ethernet_structured_config: ::validated_data::Field<super::super::eos_cli_config_gen::ethernet_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
    #[data_view(relaxed)]
    pub uplink_switch_port_channel_structured_config: ::validated_data::Field<super::super::eos_cli_config_gen::port_channel_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
    #[data_view(relaxed)]
    pub mlag_port_channel_structured_config: ::validated_data::Field<super::super::eos_cli_config_gen::port_channel_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
    #[data_view(relaxed)]
    pub mlag_peer_vlan_structured_config: ::validated_data::Field<super::super::eos_cli_config_gen::vlan_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
    #[data_view(relaxed)]
    pub mlag_peer_l3_vlan_structured_config: ::validated_data::Field<super::super::eos_cli_config_gen::vlan_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
    pub short_esi: ::validated_data::Field<&'a str>,
    pub isis_system_id_prefix: ::validated_data::Field<&'a str>,
    pub isis_maximum_paths: ::validated_data::Field<i64>,
    pub is_type: ::validated_data::Field<&'a str>,
    pub node_sid_base: ::validated_data::Field<i64>,
    pub isis_sr: ::validated_data::Field<item::IsisSr<'a, Mode>>,
    pub loopback_ipv4_pool: ::validated_data::Field<&'a str>,
    pub loopback_ipv4_address: ::validated_data::Field<&'a str>,
    pub vtep_loopback_ipv4_pool: ::validated_data::Field<&'a str>,
    pub vtep_loopback_ipv6_pool: ::validated_data::Field<&'a str>,
    pub vtep_loopback_ipv4_address: ::validated_data::Field<&'a str>,
    pub vtep_loopback_ipv6_address: ::validated_data::Field<&'a str>,
    pub loopback_ipv4_offset: ::validated_data::Field<i64>,
    pub router_id_pool: ::validated_data::Field<&'a str>,
    pub loopback_ipv6_pool: ::validated_data::Field<&'a str>,
    pub loopback_ipv6_offset: ::validated_data::Field<i64>,
    pub vtep: ::validated_data::Field<bool>,
    pub vtep_loopback: ::validated_data::Field<&'a str>,
    pub bgp_as: ::validated_data::Field<&'a str>,
    pub bgp_defaults: ::validated_data::Field<item::BgpDefaults<'a, Mode>>,
    pub evpn_role: ::validated_data::Field<&'a str>,
    pub evpn_route_servers: ::validated_data::Field<item::EvpnRouteServers<'a, Mode>>,
    pub evpn_services_l2_only: ::validated_data::Field<bool>,
    pub filter: ::validated_data::Field<item::Filter<'a, Mode>>,
    pub igmp_snooping_enabled: ::validated_data::Field<bool>,
    pub evpn_gateway: ::validated_data::Field<item::EvpnGateway<'a, Mode>>,
    pub ipvpn_gateway: ::validated_data::Field<item::IpvpnGateway<'a, Mode>>,
    pub mlag: ::validated_data::Field<bool>,
    pub mlag_dual_primary_detection: ::validated_data::Field<bool>,
    pub mlag_ibgp_origin_incomplete: ::validated_data::Field<bool>,
    pub mlag_interfaces: ::validated_data::Field<item::MlagInterfaces<'a, Mode>>,
    pub mlag_interfaces_speed: ::validated_data::Field<&'a str>,
    pub mlag_peer_l3_vlan: ::validated_data::Field<i64>,
    pub mlag_peer_l3_ipv4_pool: ::validated_data::Field<&'a str>,
    pub mlag_peer_l3_ipv6_pool: ::validated_data::Field<&'a str>,
    pub mlag_peer_vlan: ::validated_data::Field<i64>,
    pub mlag_peer_link_allowed_vlans: ::validated_data::Field<&'a str>,
    pub mlag_peer_address_family: ::validated_data::Field<&'a str>,
    pub mlag_peer_ipv4_pool: ::validated_data::Field<&'a str>,
    pub mlag_peer_ipv6_pool: ::validated_data::Field<&'a str>,
    pub mlag_port_channel_id: ::validated_data::Field<i64>,
    pub mlag_domain_id: ::validated_data::Field<&'a str>,
    pub spanning_tree_mode: ::validated_data::Field<&'a str>,
    pub spanning_tree_priority: ::validated_data::Field<i64>,
    pub spanning_tree_root_super: ::validated_data::Field<bool>,
    pub spanning_tree_mst_pvst_boundary: ::validated_data::Field<bool>,
    pub spanning_tree_port_id_allocation_port_channel_range: ::validated_data::Field<super::super::eos_cli_config_gen::spanning_tree::PortIdAllocationPortChannelRange<'a, Mode>>,
    pub virtual_router_mac_address: ::validated_data::Field<&'a str>,
    pub inband_mgmt_interface: ::validated_data::Field<&'a str>,
    pub inband_mgmt_vlan: ::validated_data::Field<i64>,
    pub inband_mgmt_subnet: ::validated_data::Field<&'a str>,
    pub inband_mgmt_subnet_offset: ::validated_data::Field<i64>,
    pub inband_mgmt_ip: ::validated_data::Field<&'a str>,
    pub inband_mgmt_gateway: ::validated_data::Field<&'a str>,
    pub inband_mgmt_ipv6_address: ::validated_data::Field<&'a str>,
    pub inband_mgmt_ipv6_subnet: ::validated_data::Field<&'a str>,
    pub inband_mgmt_ipv6_gateway: ::validated_data::Field<&'a str>,
    pub inband_mgmt_description: ::validated_data::Field<&'a str>,
    pub inband_mgmt_vlan_name: ::validated_data::Field<&'a str>,
    pub inband_mgmt_vrf: ::validated_data::Field<&'a str>,
    pub inband_mgmt_mtu: ::validated_data::Field<i64>,
    pub inband_ztp: ::validated_data::Field<bool>,
    pub inband_ztp_lacp_fallback_delay: ::validated_data::Field<i64>,
    pub mpls_overlay_role: ::validated_data::Field<&'a str>,
    pub overlay_address_families: ::validated_data::Field<item::OverlayAddressFamilies<'a, Mode>>,
    pub mpls_route_reflectors: ::validated_data::Field<item::MplsRouteReflectors<'a, Mode>>,
    pub bgp_cluster_id: ::validated_data::Field<&'a str>,
    pub kernel_ecmp_cli: ::validated_data::Field<bool>,
    pub ptp: ::validated_data::Field<item::Ptp<'a, Mode>>,
    pub wan_role: ::validated_data::Field<&'a str>,
    pub cv_pathfinder_transit_mode: ::validated_data::Field<&'a str>,
    pub cv_pathfinder_region: ::validated_data::Field<&'a str>,
    pub cv_pathfinder_site: ::validated_data::Field<&'a str>,
    pub wan_ha: ::validated_data::Field<item::WanHa<'a, Mode>>,
    pub dps_mss_ipv4: ::validated_data::Field<&'a str>,
    pub l3_interfaces: ::validated_data::Field<item::L3Interfaces<'a, Mode>>,
    pub l3_port_channels: ::validated_data::Field<item::L3PortChannels<'a, Mode>>,
    pub data_plane_cpu_allocation_max: ::validated_data::Field<i64>,
    pub flow_tracker_type: ::validated_data::Field<&'a str>,
    pub underlay_multicast: ::validated_data::Field<item::UnderlayMulticast<'a, Mode>>,
    pub campus: ::validated_data::Field<&'a str>,
    pub campus_pod: ::validated_data::Field<&'a str>,
    pub campus_access_pod: ::validated_data::Field<&'a str>,
    pub cv_tags_topology_type: ::validated_data::Field<&'a str>,
    pub digital_twin: ::validated_data::Field<item::DigitalTwin<'a, Mode>>,
    pub validation_profile: ::validated_data::Field<&'a str>,
}

pub mod item {

    #[::validated_data::data_view]
    pub struct LinkTracking<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub downlinks: ::validated_data::Field<link_tracking::Downlinks<'a, Mode>>,
        pub groups: ::validated_data::Field<link_tracking::Groups<'a, Mode>>,
    }

    pub mod link_tracking {

        #[::validated_data::data_view]
        pub struct Downlinks<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub group: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct Groups<'a, Mode> (::validated_data::Field<groups::Item<'a, Mode>>);

        pub mod groups {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub recovery_delay: ::validated_data::Field<i64>,
                pub links_minimum: ::validated_data::Field<i64>,
            }
        }
    }

    #[::validated_data::data_view]
    pub struct LacpPortIdRange<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub size: ::validated_data::Field<i64>,
        pub offset: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(list)]
    pub struct UplinkInterfaces<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list)]
    pub struct UplinkSwitchInterfaces<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list)]
    pub struct UplinkSwitches<'a, Mode> (::validated_data::RequiredValue<&'a str, Mode>);

    #[::validated_data::data_view]
    pub struct UplinkPtp<'a, Mode> {
        pub enable: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view]
    pub struct UplinkMacsec<'a, Mode> {
        pub profile: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct IsisSr<'a, Mode> {
        pub ipv4_node_sid_index: ::validated_data::Field<i64>,
        pub ipv4_node_sid_index_base: ::validated_data::Field<i64>,
        pub ipv6_node_sid_index: ::validated_data::Field<i64>,
        pub ipv6_node_sid_index_base: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(list)]
    pub struct BgpDefaults<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list)]
    pub struct EvpnRouteServers<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view]
    pub struct Filter<'a, Mode> {
        pub tenants: ::validated_data::Field<filter::Tenants<'a, Mode>>,
        pub tags: ::validated_data::Field<filter::Tags<'a, Mode>>,
        pub allow_vrfs: ::validated_data::Field<filter::AllowVrfs<'a, Mode>>,
        pub deny_vrfs: ::validated_data::Field<filter::DenyVrfs<'a, Mode>>,
        pub always_include_vrfs_in_tenants: ::validated_data::Field<filter::AlwaysIncludeVrfsInTenants<'a, Mode>>,
        pub only_vlans_in_use: ::validated_data::Field<bool>,
    }

    pub mod filter {

        #[::validated_data::data_view(list)]
        pub struct Tenants<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list)]
        pub struct Tags<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list)]
        pub struct AllowVrfs<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list)]
        pub struct DenyVrfs<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list)]
        pub struct AlwaysIncludeVrfsInTenants<'a, Mode> (::validated_data::Field<&'a str>);
    }

    #[::validated_data::data_view]
    pub struct EvpnGateway<'a, Mode> {
        pub remote_peers: ::validated_data::Field<evpn_gateway::RemotePeers<'a, Mode>>,
        pub evpn_l2: ::validated_data::Field<evpn_gateway::EvpnL2<'a, Mode>>,
        pub evpn_l3: ::validated_data::Field<evpn_gateway::EvpnL3<'a, Mode>>,
        pub d_path: ::validated_data::Field<evpn_gateway::DPath<'a, Mode>>,
        pub all_active_multihoming: ::validated_data::Field<evpn_gateway::AllActiveMultihoming<'a, Mode>>,
    }

    pub mod evpn_gateway {

        #[::validated_data::data_view(indexed_list, primary_key(hostname))]
        pub struct RemotePeers<'a, Mode> (::validated_data::Field<remote_peers::Item<'a, Mode>>);

        pub mod remote_peers {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub hostname: ::validated_data::Field<&'a str>,
                pub ip_address: ::validated_data::Field<&'a str>,
                pub bgp_as: ::validated_data::Field<&'a str>,
            }
        }

        #[::validated_data::data_view]
        pub struct EvpnL2<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct EvpnL3<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub inter_domain: ::validated_data::Field<bool>,
            pub mode: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct DPath<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub local_domain_id: ::validated_data::RequiredValue<&'a str, Mode>,
            pub remote_domain_id: ::validated_data::RequiredValue<&'a str, Mode>,
        }

        #[::validated_data::data_view]
        pub struct AllActiveMultihoming<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub enable_d_path: ::validated_data::Field<bool>,
            pub evpn_domain_id_local: ::validated_data::Field<&'a str>,
            pub evpn_domain_id_remote: ::validated_data::Field<&'a str>,
            pub evpn_ethernet_segment: ::validated_data::RequiredValue<all_active_multihoming::EvpnEthernetSegment<'a, Mode>, Mode>,
        }

        pub mod all_active_multihoming {

            #[::validated_data::data_view]
            pub struct EvpnEthernetSegment<'a, Mode> {
                pub identifier: ::validated_data::RequiredValue<&'a str, Mode>,
                pub rt_import: ::validated_data::RequiredValue<&'a str, Mode>,
            }
        }
    }

    #[::validated_data::data_view]
    pub struct IpvpnGateway<'a, Mode> {
        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
        pub evpn_domain_id: ::validated_data::Field<&'a str>,
        pub ipvpn_domain_id: ::validated_data::Field<&'a str>,
        pub enable_d_path: ::validated_data::Field<bool>,
        pub maximum_routes: ::validated_data::Field<i64>,
        pub local_as: ::validated_data::Field<&'a str>,
        pub address_families: ::validated_data::Field<ipvpn_gateway::AddressFamilies<'a, Mode>>,
        pub remote_peers: ::validated_data::Field<ipvpn_gateway::RemotePeers<'a, Mode>>,
    }

    pub mod ipvpn_gateway {

        #[::validated_data::data_view(list)]
        pub struct AddressFamilies<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(indexed_list, primary_key(hostname))]
        pub struct RemotePeers<'a, Mode> (::validated_data::Field<remote_peers::Item<'a, Mode>>);

        pub mod remote_peers {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub hostname: ::validated_data::Field<&'a str>,
                pub ip_address: ::validated_data::RequiredValue<&'a str, Mode>,
                pub bgp_as: ::validated_data::RequiredValue<&'a str, Mode>,
            }
        }
    }

    #[::validated_data::data_view(list)]
    pub struct MlagInterfaces<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list)]
    pub struct OverlayAddressFamilies<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list)]
    pub struct MplsRouteReflectors<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view]
    pub struct Ptp<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub profile: ::validated_data::Field<&'a str>,
        pub uplinks: ::validated_data::Field<ptp::Uplinks<'a, Mode>>,
        pub mlag: ::validated_data::Field<bool>,
        pub domain: ::validated_data::Field<i64>,
        pub priority1: ::validated_data::Field<i64>,
        pub priority2: ::validated_data::Field<i64>,
        pub auto_clock_identity: ::validated_data::Field<bool>,
        pub clock_identity_prefix: ::validated_data::Field<&'a str>,
        pub clock_identity: ::validated_data::Field<&'a str>,
        pub source_ip: ::validated_data::Field<&'a str>,
        pub mode: ::validated_data::Field<&'a str>,
        pub mode_one_step: ::validated_data::Field<bool>,
        pub ttl: ::validated_data::Field<i64>,
        pub forward_unicast: ::validated_data::Field<bool>,
        pub forward_v1: ::validated_data::Field<bool>,
        pub free_running: ::validated_data::Field<super::super::super::eos_cli_config_gen::ptp::FreeRunning<'a, Mode>>,
        pub dscp: ::validated_data::Field<ptp::Dscp<'a, Mode>>,
        pub monitor: ::validated_data::Field<ptp::Monitor<'a, Mode>>,
    }

    pub mod ptp {

        #[::validated_data::data_view(list)]
        pub struct Uplinks<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view]
        pub struct Dscp<'a, Mode> {
            pub general_messages: ::validated_data::Field<i64>,
            pub event_messages: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view]
        pub struct Monitor<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub threshold: ::validated_data::Field<monitor::Threshold<'a, Mode>>,
            pub missing_message: ::validated_data::Field<monitor::MissingMessage<'a, Mode>>,
        }

        pub mod monitor {

            #[::validated_data::data_view]
            pub struct Threshold<'a, Mode> {
                pub offset_from_master: ::validated_data::Field<i64>,
                pub mean_path_delay: ::validated_data::Field<i64>,
                pub drop: ::validated_data::Field<threshold::Drop<'a, Mode>>,
            }

            pub mod threshold {

                #[::validated_data::data_view]
                pub struct Drop<'a, Mode> {
                    pub offset_from_master: ::validated_data::Field<i64>,
                    pub mean_path_delay: ::validated_data::Field<i64>,
                }
            }

            #[::validated_data::data_view]
            pub struct MissingMessage<'a, Mode> {
                pub intervals: ::validated_data::Field<missing_message::Intervals<'a, Mode>>,
                pub sequence_ids: ::validated_data::Field<missing_message::SequenceIds<'a, Mode>>,
            }

            pub mod missing_message {

                #[::validated_data::data_view]
                pub struct Intervals<'a, Mode> {
                    pub announce: ::validated_data::Field<i64>,
                    pub follow_up: ::validated_data::Field<i64>,
                    pub sync: ::validated_data::Field<i64>,
                }

                #[::validated_data::data_view]
                pub struct SequenceIds<'a, Mode> {
                    pub enabled: ::validated_data::Field<bool>,
                    pub announce: ::validated_data::Field<i64>,
                    pub delay_resp: ::validated_data::Field<i64>,
                    pub follow_up: ::validated_data::Field<i64>,
                    pub sync: ::validated_data::Field<i64>,
                }
            }
        }
    }

    #[::validated_data::data_view]
    pub struct WanHa<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub ipsec: ::validated_data::Field<bool>,
        pub mtu: ::validated_data::Field<i64>,
        pub ha_interfaces: ::validated_data::Field<wan_ha::HaInterfaces<'a, Mode>>,
        pub ha_ipv4_pool: ::validated_data::Field<&'a str>,
        pub port_channel_id: ::validated_data::Field<i64>,
        pub use_port_channel_for_direct_ha: ::validated_data::Field<bool>,
        pub flow_tracking: ::validated_data::Field<wan_ha::FlowTracking<'a, Mode>>,
    }

    pub mod wan_ha {

        #[::validated_data::data_view(list)]
        pub struct HaInterfaces<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view]
        pub struct FlowTracking<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub name: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct L3Interfaces<'a, Mode> (::validated_data::Field<l3_interfaces::Item<'a, Mode>>);

    pub mod l3_interfaces {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub profile: ::validated_data::Field<&'a str>,
            pub name: ::validated_data::Field<&'a str>,
            pub description: ::validated_data::Field<&'a str>,
            pub ip_address: ::validated_data::Field<&'a str>,
            pub ipv6_addresses: ::validated_data::Field<item::Ipv6Addresses<'a, Mode>>,
            pub dhcp_ip: ::validated_data::Field<&'a str>,
            pub public_ip: ::validated_data::Field<&'a str>,
            pub encapsulation_dot1q_vlan: ::validated_data::Field<i64>,
            pub dhcp_accept_default_route: ::validated_data::Field<bool>,
            pub enabled: ::validated_data::Field<bool>,
            pub speed: ::validated_data::Field<&'a str>,
            pub receive_bandwidth: ::validated_data::Field<i64>,
            pub transmit_bandwidth: ::validated_data::Field<i64>,
            pub peer: ::validated_data::Field<&'a str>,
            pub peer_interface: ::validated_data::Field<&'a str>,
            pub peer_ip: ::validated_data::Field<&'a str>,
            pub peer_ipv6: ::validated_data::Field<&'a str>,
            pub bgp: ::validated_data::Field<item::Bgp<'a, Mode>>,
            pub ipv4_acl_in: ::validated_data::Field<&'a str>,
            pub ipv4_acl_out: ::validated_data::Field<&'a str>,
            pub ipv6_acl_in: ::validated_data::Field<&'a str>,
            pub ipv6_acl_out: ::validated_data::Field<&'a str>,
            pub static_routes: ::validated_data::Field<item::StaticRoutes<'a, Mode>>,
            pub qos_profile: ::validated_data::Field<&'a str>,
            pub wan_carrier: ::validated_data::Field<&'a str>,
            pub wan_circuit_id: ::validated_data::Field<&'a str>,
            pub connected_to_pathfinder: ::validated_data::Field<bool>,
            pub cv_pathfinder_internet_exit: ::validated_data::Field<item::CvPathfinderInternetExit<'a, Mode>>,
            pub rx_queue: ::validated_data::Field<item::RxQueue<'a, Mode>>,
            pub raw_eos_cli: ::validated_data::Field<&'a str>,
            pub flow_tracking: ::validated_data::Field<item::FlowTracking<'a, Mode>>,
            #[data_view(relaxed)]
            pub structured_config: ::validated_data::Field<super::super::super::super::eos_cli_config_gen::ethernet_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
        }

        pub mod item {

            #[::validated_data::data_view(list)]
            pub struct Ipv6Addresses<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view]
            pub struct Bgp<'a, Mode> {
                pub peer_as: ::validated_data::RequiredValue<&'a str, Mode>,
                pub ipv4_prefix_list_in: ::validated_data::Field<&'a str>,
                pub ipv4_prefix_list_out: ::validated_data::Field<&'a str>,
                pub ipv6_prefix_list_in: ::validated_data::Field<&'a str>,
                pub ipv6_prefix_list_out: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view(list)]
            pub struct StaticRoutes<'a, Mode> (::validated_data::Field<static_routes::Item<'a, Mode>>);

            pub mod static_routes {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub prefix: ::validated_data::RequiredValue<&'a str, Mode>,
                }
            }

            #[::validated_data::data_view]
            pub struct CvPathfinderInternetExit<'a, Mode> {
                pub policies: ::validated_data::Field<cv_pathfinder_internet_exit::Policies<'a, Mode>>,
            }

            pub mod cv_pathfinder_internet_exit {

                #[::validated_data::data_view(indexed_list, primary_key(name))]
                pub struct Policies<'a, Mode> (::validated_data::Field<policies::Item<'a, Mode>>);

                pub mod policies {

                    #[::validated_data::data_view]
                    pub struct Item<'a, Mode> {
                        pub name: ::validated_data::Field<&'a str>,
                        pub tunnel_interface_numbers: ::validated_data::Field<&'a str>,
                    }
                }
            }

            #[::validated_data::data_view]
            pub struct RxQueue<'a, Mode> {
                pub count: ::validated_data::Field<i64>,
                pub workers: ::validated_data::Field<rx_queue::Workers<'a, Mode>>,
                pub mode: ::validated_data::Field<&'a str>,
            }

            pub mod rx_queue {

                #[::validated_data::data_view(list)]
                pub struct Workers<'a, Mode> (::validated_data::Field<&'a str>);
            }

            #[::validated_data::data_view]
            pub struct FlowTracking<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub name: ::validated_data::Field<&'a str>,
            }
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct L3PortChannels<'a, Mode> (::validated_data::Field<l3_port_channels::Item<'a, Mode>>);

    pub mod l3_port_channels {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::RequiredValue<&'a str, Mode>,
            pub description: ::validated_data::Field<&'a str>,
            pub mode: ::validated_data::Field<&'a str>,
            pub member_interfaces: ::validated_data::Field<item::MemberInterfaces<'a, Mode>>,
            pub ip_address: ::validated_data::Field<&'a str>,
            pub ipv6_addresses: ::validated_data::Field<item::Ipv6Addresses<'a, Mode>>,
            pub dhcp_ip: ::validated_data::Field<&'a str>,
            pub public_ip: ::validated_data::Field<&'a str>,
            pub encapsulation_dot1q_vlan: ::validated_data::Field<i64>,
            pub dhcp_accept_default_route: ::validated_data::Field<bool>,
            pub enabled: ::validated_data::Field<bool>,
            pub peer: ::validated_data::Field<&'a str>,
            pub peer_port_channel: ::validated_data::Field<&'a str>,
            pub peer_ip: ::validated_data::Field<&'a str>,
            pub peer_ipv6: ::validated_data::Field<&'a str>,
            pub bgp: ::validated_data::Field<item::Bgp<'a, Mode>>,
            pub ipv4_acl_in: ::validated_data::Field<&'a str>,
            pub ipv4_acl_out: ::validated_data::Field<&'a str>,
            pub ipv6_acl_in: ::validated_data::Field<&'a str>,
            pub ipv6_acl_out: ::validated_data::Field<&'a str>,
            pub static_routes: ::validated_data::Field<item::StaticRoutes<'a, Mode>>,
            pub qos_profile: ::validated_data::Field<&'a str>,
            pub wan_carrier: ::validated_data::Field<&'a str>,
            pub wan_circuit_id: ::validated_data::Field<&'a str>,
            pub connected_to_pathfinder: ::validated_data::Field<bool>,
            pub raw_eos_cli: ::validated_data::Field<&'a str>,
            pub flow_tracking: ::validated_data::Field<item::FlowTracking<'a, Mode>>,
            #[data_view(relaxed)]
            pub structured_config: ::validated_data::Field<super::super::super::super::eos_cli_config_gen::port_channel_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
        }

        pub mod item {

            #[::validated_data::data_view(indexed_list, primary_key(name))]
            pub struct MemberInterfaces<'a, Mode> (::validated_data::Field<member_interfaces::Item<'a, Mode>>);

            pub mod member_interfaces {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub name: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub description: ::validated_data::Field<&'a str>,
                    pub peer: ::validated_data::Field<&'a str>,
                    pub peer_interface: ::validated_data::Field<&'a str>,
                    pub speed: ::validated_data::Field<&'a str>,
                    pub rx_queue: ::validated_data::Field<item::RxQueue<'a, Mode>>,
                    #[data_view(relaxed)]
                    pub structured_config: ::validated_data::Field<super::super::super::super::super::super::eos_cli_config_gen::ethernet_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
                }

                pub mod item {

                    #[::validated_data::data_view]
                    pub struct RxQueue<'a, Mode> {
                        pub count: ::validated_data::Field<i64>,
                        pub workers: ::validated_data::Field<rx_queue::Workers<'a, Mode>>,
                        pub mode: ::validated_data::Field<&'a str>,
                    }

                    pub mod rx_queue {

                        #[::validated_data::data_view(list)]
                        pub struct Workers<'a, Mode> (::validated_data::Field<&'a str>);
                    }
                }
            }

            #[::validated_data::data_view(list)]
            pub struct Ipv6Addresses<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view]
            pub struct Bgp<'a, Mode> {
                pub peer_as: ::validated_data::RequiredValue<&'a str, Mode>,
                pub ipv4_prefix_list_in: ::validated_data::Field<&'a str>,
                pub ipv4_prefix_list_out: ::validated_data::Field<&'a str>,
                pub ipv6_prefix_list_in: ::validated_data::Field<&'a str>,
                pub ipv6_prefix_list_out: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view(indexed_list, primary_key(prefix))]
            pub struct StaticRoutes<'a, Mode> (::validated_data::Field<static_routes::Item<'a, Mode>>);

            pub mod static_routes {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub prefix: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view]
            pub struct FlowTracking<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub name: ::validated_data::Field<&'a str>,
            }
        }
    }

    #[::validated_data::data_view]
    pub struct UnderlayMulticast<'a, Mode> {
        pub pim_sm: ::validated_data::Field<underlay_multicast::PimSm<'a, Mode>>,
        #[data_view(rename = "static")]
        pub field_static: ::validated_data::Field<underlay_multicast::FieldStatic<'a, Mode>>,
    }

    pub mod underlay_multicast {

        #[::validated_data::data_view]
        pub struct PimSm<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub uplinks: ::validated_data::Field<bool>,
            pub uplink_interfaces: ::validated_data::Field<pim_sm::UplinkInterfaces<'a, Mode>>,
            pub mlag: ::validated_data::Field<bool>,
        }

        pub mod pim_sm {

            #[::validated_data::data_view(list)]
            pub struct UplinkInterfaces<'a, Mode> (::validated_data::Field<&'a str>);
        }

        #[::validated_data::data_view]
        pub struct FieldStatic<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub uplinks: ::validated_data::Field<bool>,
            pub uplink_interfaces: ::validated_data::Field<field_static::UplinkInterfaces<'a, Mode>>,
            pub mlag: ::validated_data::Field<bool>,
        }

        pub mod field_static {

            #[::validated_data::data_view(list)]
            pub struct UplinkInterfaces<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }

    #[::validated_data::data_view]
    pub struct DigitalTwin<'a, Mode> {
        pub act_os_version: ::validated_data::Field<&'a str>,
        pub mgmt_ip: ::validated_data::Field<&'a str>,
        pub mgmt_gateway: ::validated_data::Field<&'a str>,
        pub act_internet_access: ::validated_data::Field<bool>,
    }
}
