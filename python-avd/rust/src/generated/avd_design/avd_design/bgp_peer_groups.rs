// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Ipv4UnderlayPeers<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub password: ::validated_data::Field<&'a str>,
    pub cleartext_password: ::validated_data::Field<&'a str>,
    pub bfd: ::validated_data::Field<bool>,
    pub maximum_routes: ::validated_data::Field<i64>,
    #[data_view(relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_cli_config_gen::router_bgp::peer_groups::Item<'a, ::validated_data::RelaxedValidated>>,
}

pub mod ipv4_underlay_peers {
}

#[::validated_data::data_view]
pub struct MlagIpv4VrfsPeer<'a, Mode> {
    pub name: ::validated_data::RequiredValue<&'a str, Mode>,
    pub password: ::validated_data::Field<&'a str>,
    pub cleartext_password: ::validated_data::Field<&'a str>,
    pub bfd: ::validated_data::Field<bool>,
    pub maximum_routes: ::validated_data::Field<i64>,
    #[data_view(relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_cli_config_gen::router_bgp::peer_groups::Item<'a, ::validated_data::RelaxedValidated>>,
}

pub mod mlag_ipv4_vrfs_peer {
}

#[::validated_data::data_view]
pub struct MlagIpv4UnderlayPeer<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub password: ::validated_data::Field<&'a str>,
    pub cleartext_password: ::validated_data::Field<&'a str>,
    pub bfd: ::validated_data::Field<bool>,
    pub maximum_routes: ::validated_data::Field<i64>,
    #[data_view(relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_cli_config_gen::router_bgp::peer_groups::Item<'a, ::validated_data::RelaxedValidated>>,
}

pub mod mlag_ipv4_underlay_peer {
}

#[::validated_data::data_view]
pub struct EvpnOverlayPeers<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub password: ::validated_data::Field<&'a str>,
    pub cleartext_password: ::validated_data::Field<&'a str>,
    pub bfd: ::validated_data::Field<bool>,
    #[data_view(relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_cli_config_gen::router_bgp::peer_groups::Item<'a, ::validated_data::RelaxedValidated>>,
}

pub mod evpn_overlay_peers {
}

#[::validated_data::data_view]
pub struct EvpnOverlayCore<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub password: ::validated_data::Field<&'a str>,
    pub cleartext_password: ::validated_data::Field<&'a str>,
    pub bfd: ::validated_data::Field<bool>,
    #[data_view(relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_cli_config_gen::router_bgp::peer_groups::Item<'a, ::validated_data::RelaxedValidated>>,
}

pub mod evpn_overlay_core {
}

#[::validated_data::data_view]
pub struct MplsOverlayPeers<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub password: ::validated_data::Field<&'a str>,
    pub cleartext_password: ::validated_data::Field<&'a str>,
    pub bfd: ::validated_data::Field<bool>,
    #[data_view(relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_cli_config_gen::router_bgp::peer_groups::Item<'a, ::validated_data::RelaxedValidated>>,
}

pub mod mpls_overlay_peers {
}

#[::validated_data::data_view]
pub struct RrOverlayPeers<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub password: ::validated_data::Field<&'a str>,
    pub cleartext_password: ::validated_data::Field<&'a str>,
    pub bfd: ::validated_data::Field<bool>,
    #[data_view(relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_cli_config_gen::router_bgp::peer_groups::Item<'a, ::validated_data::RelaxedValidated>>,
}

pub mod rr_overlay_peers {
}

#[::validated_data::data_view]
pub struct IpvpnGatewayPeers<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub password: ::validated_data::Field<&'a str>,
    pub cleartext_password: ::validated_data::Field<&'a str>,
    pub bfd: ::validated_data::Field<bool>,
    #[data_view(relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_cli_config_gen::router_bgp::peer_groups::Item<'a, ::validated_data::RelaxedValidated>>,
}

pub mod ipvpn_gateway_peers {
}

#[::validated_data::data_view]
pub struct WanOverlayPeers<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub password: ::validated_data::Field<&'a str>,
    pub cleartext_password: ::validated_data::Field<&'a str>,
    pub bfd: ::validated_data::Field<bool>,
    pub bfd_timers: ::validated_data::Field<wan_overlay_peers::BfdTimers<'a, Mode>>,
    pub listen_range_prefixes: ::validated_data::Field<wan_overlay_peers::ListenRangePrefixes<'a, Mode>>,
    pub ttl_maximum_hops: ::validated_data::Field<i64>,
    #[data_view(relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_cli_config_gen::router_bgp::peer_groups::Item<'a, ::validated_data::RelaxedValidated>>,
}

pub mod wan_overlay_peers {

    #[::validated_data::data_view]
    pub struct BfdTimers<'a, Mode> {
        pub interval: ::validated_data::RequiredValue<i64, Mode>,
        pub min_rx: ::validated_data::RequiredValue<i64, Mode>,
        pub multiplier: ::validated_data::RequiredValue<i64, Mode>,
    }

    #[::validated_data::data_view(list)]
    pub struct ListenRangePrefixes<'a, Mode> (::validated_data::Field<&'a str>);
}

#[::validated_data::data_view]
pub struct WanRrOverlayPeers<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub password: ::validated_data::Field<&'a str>,
    pub cleartext_password: ::validated_data::Field<&'a str>,
    pub bfd: ::validated_data::Field<bool>,
    pub bfd_timers: ::validated_data::Field<wan_rr_overlay_peers::BfdTimers<'a, Mode>>,
    pub ttl_maximum_hops: ::validated_data::Field<i64>,
    #[data_view(relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_cli_config_gen::router_bgp::peer_groups::Item<'a, ::validated_data::RelaxedValidated>>,
}

pub mod wan_rr_overlay_peers {

    #[::validated_data::data_view]
    pub struct BfdTimers<'a, Mode> {
        pub interval: ::validated_data::RequiredValue<i64, Mode>,
        pub min_rx: ::validated_data::RequiredValue<i64, Mode>,
        pub multiplier: ::validated_data::RequiredValue<i64, Mode>,
    }
}
