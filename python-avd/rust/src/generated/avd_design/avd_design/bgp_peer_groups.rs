// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.BgpPeerGroups", schema = Dict(1405)))]
pub struct BgpPeerGroups<'a, Mode> {
    #[data_view(schema = Dict(1394))]
    pub ipv4_underlay_peers: ::data_views::Field<Ipv4UnderlayPeers<'a, Mode>>,
    #[data_view(schema = Dict(1395))]
    pub mlag_ipv4_vrfs_peer: ::data_views::Field<MlagIpv4VrfsPeer<'a, Mode>>,
    #[data_view(schema = Dict(1396))]
    pub mlag_ipv4_underlay_peer: ::data_views::Field<MlagIpv4UnderlayPeer<'a, Mode>>,
    #[data_view(schema = Dict(1397))]
    pub evpn_overlay_peers: ::data_views::Field<EvpnOverlayPeers<'a, Mode>>,
    #[data_view(schema = Dict(1398))]
    pub evpn_overlay_core: ::data_views::Field<EvpnOverlayCore<'a, Mode>>,
    #[data_view(schema = Dict(1399))]
    pub mpls_overlay_peers: ::data_views::Field<MplsOverlayPeers<'a, Mode>>,
    #[data_view(schema = Dict(1400))]
    pub rr_overlay_peers: ::data_views::Field<RrOverlayPeers<'a, Mode>>,
    #[data_view(schema = Dict(1401))]
    pub ipvpn_gateway_peers: ::data_views::Field<IpvpnGatewayPeers<'a, Mode>>,
    #[data_view(schema = Dict(1403))]
    pub wan_overlay_peers: ::data_views::Field<WanOverlayPeers<'a, Mode>>,
    #[data_view(schema = Dict(1404))]
    pub wan_rr_overlay_peers: ::data_views::Field<WanRrOverlayPeers<'a, Mode>>,
}

#[::data_views::data_view(python(name = "AVDDesign.BgpPeerGroups.Ipv4UnderlayPeers", schema = Dict(1394)))]
pub struct Ipv4UnderlayPeers<'a, Mode> {
    #[data_view(schema = Str(1107))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1108))]
    pub password: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1109))]
    pub cleartext_password: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(340))]
    pub bfd: ::data_views::Field<bool>,
    #[data_view(schema = Int(501))]
    pub maximum_routes: ::data_views::Field<i64>,
    #[data_view(schema = Dict(1393), relaxed)]
    pub structured_config: ::data_views::Field<super::super::eos_config::router_bgp::peer_groups::Item<'a, ::data_views::RelaxedValidated>>,
}

pub mod ipv4_underlay_peers {
}

#[::data_views::data_view(python(name = "AVDDesign.BgpPeerGroups.MlagIpv4VrfsPeer", schema = Dict(1395)))]
pub struct MlagIpv4VrfsPeer<'a, Mode> {
    #[data_view(schema = Str(1110))]
    pub name: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(1108))]
    pub password: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1109))]
    pub cleartext_password: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(340))]
    pub bfd: ::data_views::Field<bool>,
    #[data_view(schema = Int(501))]
    pub maximum_routes: ::data_views::Field<i64>,
    #[data_view(schema = Dict(1393), relaxed)]
    pub structured_config: ::data_views::Field<super::super::eos_config::router_bgp::peer_groups::Item<'a, ::data_views::RelaxedValidated>>,
}

pub mod mlag_ipv4_vrfs_peer {
}

#[::data_views::data_view(python(name = "AVDDesign.BgpPeerGroups.MlagIpv4UnderlayPeer", schema = Dict(1396)))]
pub struct MlagIpv4UnderlayPeer<'a, Mode> {
    #[data_view(schema = Str(1111))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1108))]
    pub password: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1109))]
    pub cleartext_password: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(340))]
    pub bfd: ::data_views::Field<bool>,
    #[data_view(schema = Int(501))]
    pub maximum_routes: ::data_views::Field<i64>,
    #[data_view(schema = Dict(1393), relaxed)]
    pub structured_config: ::data_views::Field<super::super::eos_config::router_bgp::peer_groups::Item<'a, ::data_views::RelaxedValidated>>,
}

pub mod mlag_ipv4_underlay_peer {
}

#[::data_views::data_view(python(name = "AVDDesign.BgpPeerGroups.EvpnOverlayPeers", schema = Dict(1397)))]
pub struct EvpnOverlayPeers<'a, Mode> {
    #[data_view(schema = Str(1112))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1108))]
    pub password: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1109))]
    pub cleartext_password: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(37))]
    pub bfd: ::data_views::Field<bool>,
    #[data_view(schema = Dict(1393), relaxed)]
    pub structured_config: ::data_views::Field<super::super::eos_config::router_bgp::peer_groups::Item<'a, ::data_views::RelaxedValidated>>,
}

pub mod evpn_overlay_peers {
}

#[::data_views::data_view(python(name = "AVDDesign.BgpPeerGroups.EvpnOverlayCore", schema = Dict(1398)))]
pub struct EvpnOverlayCore<'a, Mode> {
    #[data_view(schema = Str(1113))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1108))]
    pub password: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1109))]
    pub cleartext_password: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(37))]
    pub bfd: ::data_views::Field<bool>,
    #[data_view(schema = Dict(1393), relaxed)]
    pub structured_config: ::data_views::Field<super::super::eos_config::router_bgp::peer_groups::Item<'a, ::data_views::RelaxedValidated>>,
}

pub mod evpn_overlay_core {
}

#[::data_views::data_view(python(name = "AVDDesign.BgpPeerGroups.MplsOverlayPeers", schema = Dict(1399)))]
pub struct MplsOverlayPeers<'a, Mode> {
    #[data_view(schema = Str(1114))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1108))]
    pub password: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1109))]
    pub cleartext_password: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(37))]
    pub bfd: ::data_views::Field<bool>,
    #[data_view(schema = Dict(1393), relaxed)]
    pub structured_config: ::data_views::Field<super::super::eos_config::router_bgp::peer_groups::Item<'a, ::data_views::RelaxedValidated>>,
}

pub mod mpls_overlay_peers {
}

#[::data_views::data_view(python(name = "AVDDesign.BgpPeerGroups.RrOverlayPeers", schema = Dict(1400)))]
pub struct RrOverlayPeers<'a, Mode> {
    #[data_view(schema = Str(1115))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1108))]
    pub password: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1109))]
    pub cleartext_password: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(37))]
    pub bfd: ::data_views::Field<bool>,
    #[data_view(schema = Dict(1393), relaxed)]
    pub structured_config: ::data_views::Field<super::super::eos_config::router_bgp::peer_groups::Item<'a, ::data_views::RelaxedValidated>>,
}

pub mod rr_overlay_peers {
}

#[::data_views::data_view(python(name = "AVDDesign.BgpPeerGroups.IpvpnGatewayPeers", schema = Dict(1401)))]
pub struct IpvpnGatewayPeers<'a, Mode> {
    #[data_view(schema = Str(1116))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1108))]
    pub password: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1109))]
    pub cleartext_password: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(37))]
    pub bfd: ::data_views::Field<bool>,
    #[data_view(schema = Dict(1393), relaxed)]
    pub structured_config: ::data_views::Field<super::super::eos_config::router_bgp::peer_groups::Item<'a, ::data_views::RelaxedValidated>>,
}

pub mod ipvpn_gateway_peers {
}

#[::data_views::data_view(python(name = "AVDDesign.BgpPeerGroups.WanOverlayPeers", schema = Dict(1403)))]
pub struct WanOverlayPeers<'a, Mode> {
    #[data_view(schema = Str(1117))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1118))]
    pub password: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1119))]
    pub cleartext_password: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(37))]
    pub bfd: ::data_views::Field<bool>,
    #[data_view(schema = Dict(1402))]
    pub bfd_timers: ::data_views::Field<wan_overlay_peers::BfdTimers<'a, Mode>>,
    #[data_view(schema = List(564))]
    pub listen_range_prefixes: ::data_views::Field<wan_overlay_peers::ListenRangePrefixes<'a, Mode>>,
    #[data_view(schema = Int(505))]
    pub ttl_maximum_hops: ::data_views::Field<i64>,
    #[data_view(schema = Dict(1393), relaxed)]
    pub structured_config: ::data_views::Field<super::super::eos_config::router_bgp::peer_groups::Item<'a, ::data_views::RelaxedValidated>>,
}

pub mod wan_overlay_peers {

    #[::data_views::data_view(python(name = "AVDDesign.BgpPeerGroups.WanOverlayPeers.BfdTimers", schema = Dict(1402)))]
    pub struct BfdTimers<'a, Mode> {
        #[data_view(schema = Int(502))]
        pub interval: ::data_views::RequiredValue<i64, Mode>,
        #[data_view(schema = Int(503))]
        pub min_rx: ::data_views::RequiredValue<i64, Mode>,
        #[data_view(schema = Int(504))]
        pub multiplier: ::data_views::RequiredValue<i64, Mode>,
    }

    #[::data_views::data_view(list, python(name = "AVDDesign.BgpPeerGroups.WanOverlayPeers.ListenRangePrefixes", schema = List(564)))]
    pub struct ListenRangePrefixes<'a, Mode> (::data_views::Field<&'a str>);
}

#[::data_views::data_view(python(name = "AVDDesign.BgpPeerGroups.WanRrOverlayPeers", schema = Dict(1404)))]
pub struct WanRrOverlayPeers<'a, Mode> {
    #[data_view(schema = Str(1121))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1118))]
    pub password: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1119))]
    pub cleartext_password: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(37))]
    pub bfd: ::data_views::Field<bool>,
    #[data_view(schema = Dict(1402))]
    pub bfd_timers: ::data_views::Field<wan_rr_overlay_peers::BfdTimers<'a, Mode>>,
    #[data_view(schema = Int(505))]
    pub ttl_maximum_hops: ::data_views::Field<i64>,
    #[data_view(schema = Dict(1393), relaxed)]
    pub structured_config: ::data_views::Field<super::super::eos_config::router_bgp::peer_groups::Item<'a, ::data_views::RelaxedValidated>>,
}

pub mod wan_rr_overlay_peers {

    #[::data_views::data_view(python(name = "AVDDesign.BgpPeerGroups.WanRrOverlayPeers.BfdTimers", schema = Dict(1402)))]
    pub struct BfdTimers<'a, Mode> {
        #[data_view(schema = Int(502))]
        pub interval: ::data_views::RequiredValue<i64, Mode>,
        #[data_view(schema = Int(503))]
        pub min_rx: ::data_views::RequiredValue<i64, Mode>,
        #[data_view(schema = Int(504))]
        pub multiplier: ::data_views::RequiredValue<i64, Mode>,
    }
}
