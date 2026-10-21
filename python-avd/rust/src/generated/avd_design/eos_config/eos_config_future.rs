// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct EosConfigFuture<'a, Mode> {
    pub always_render_ip_routing_separator: ::data_views::Field<bool>,
    pub render_combined_separator_for_ipv6_hardware_and_unicast_routing: ::data_views::Field<bool>,
    pub new_ip_radius_cli_order: ::data_views::Field<bool>,
    pub new_ip_tacacs_cli_order: ::data_views::Field<bool>,
    pub only_render_mpls_rsvp_with_settings: ::data_views::Field<bool>,
    pub render_monitor_layer1_without_enabled: ::data_views::Field<bool>,
    pub render_spanning_tree_portfast_edge: ::data_views::Field<bool>,
    pub only_render_separator_with_boot_secret_key: ::data_views::Field<bool>,
}
