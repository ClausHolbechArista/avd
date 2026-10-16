// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.BgpPeerGroups.Ipv4UnderlayPeers", schema = Dict(1394)))]
pub struct Ipv4UnderlayPeers<'a, Mode> {
    #[data_view(schema = Str(1107))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1108))]
    pub password: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1109))]
    pub cleartext_password: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(340))]
    pub bfd: ::validated_data::Field<bool>,
    #[data_view(schema = Int(501))]
    pub maximum_routes: ::validated_data::Field<i64>,
    #[data_view(schema = Dict(1393), relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_config::router_bgp::peer_groups::Item<'a, ::validated_data::RelaxedValidated>>,
}

pub mod ipv4_underlay_peers {
}

#[::validated_data::data_view(python(name = "AVDDesign.BgpPeerGroups.MlagIpv4VrfsPeer", schema = Dict(1395)))]
pub struct MlagIpv4VrfsPeer<'a, Mode> {
    #[data_view(schema = Str(1110))]
    pub name: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Str(1108))]
    pub password: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1109))]
    pub cleartext_password: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(340))]
    pub bfd: ::validated_data::Field<bool>,
    #[data_view(schema = Int(501))]
    pub maximum_routes: ::validated_data::Field<i64>,
    #[data_view(schema = Dict(1393), relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_config::router_bgp::peer_groups::Item<'a, ::validated_data::RelaxedValidated>>,
}

pub mod mlag_ipv4_vrfs_peer {
}

#[::validated_data::data_view(python(name = "AVDDesign.BgpPeerGroups.MlagIpv4UnderlayPeer", schema = Dict(1396)))]
pub struct MlagIpv4UnderlayPeer<'a, Mode> {
    #[data_view(schema = Str(1111))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1108))]
    pub password: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1109))]
    pub cleartext_password: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(340))]
    pub bfd: ::validated_data::Field<bool>,
    #[data_view(schema = Int(501))]
    pub maximum_routes: ::validated_data::Field<i64>,
    #[data_view(schema = Dict(1393), relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_config::router_bgp::peer_groups::Item<'a, ::validated_data::RelaxedValidated>>,
}

pub mod mlag_ipv4_underlay_peer {
}

#[::validated_data::data_view(python(name = "AVDDesign.BgpPeerGroups.EvpnOverlayPeers", schema = Dict(1397)))]
pub struct EvpnOverlayPeers<'a, Mode> {
    #[data_view(schema = Str(1112))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1108))]
    pub password: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1109))]
    pub cleartext_password: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(37))]
    pub bfd: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1393), relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_config::router_bgp::peer_groups::Item<'a, ::validated_data::RelaxedValidated>>,
}

pub mod evpn_overlay_peers {
}

#[::validated_data::data_view(python(name = "AVDDesign.BgpPeerGroups.EvpnOverlayCore", schema = Dict(1398)))]
pub struct EvpnOverlayCore<'a, Mode> {
    #[data_view(schema = Str(1113))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1108))]
    pub password: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1109))]
    pub cleartext_password: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(37))]
    pub bfd: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1393), relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_config::router_bgp::peer_groups::Item<'a, ::validated_data::RelaxedValidated>>,
}

pub mod evpn_overlay_core {
}

#[::validated_data::data_view(python(name = "AVDDesign.BgpPeerGroups.MplsOverlayPeers", schema = Dict(1399)))]
pub struct MplsOverlayPeers<'a, Mode> {
    #[data_view(schema = Str(1114))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1108))]
    pub password: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1109))]
    pub cleartext_password: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(37))]
    pub bfd: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1393), relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_config::router_bgp::peer_groups::Item<'a, ::validated_data::RelaxedValidated>>,
}

pub mod mpls_overlay_peers {
}

#[::validated_data::data_view(python(name = "AVDDesign.BgpPeerGroups.RrOverlayPeers", schema = Dict(1400)))]
pub struct RrOverlayPeers<'a, Mode> {
    #[data_view(schema = Str(1115))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1108))]
    pub password: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1109))]
    pub cleartext_password: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(37))]
    pub bfd: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1393), relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_config::router_bgp::peer_groups::Item<'a, ::validated_data::RelaxedValidated>>,
}

pub mod rr_overlay_peers {
}

#[::validated_data::data_view(python(name = "AVDDesign.BgpPeerGroups.IpvpnGatewayPeers", schema = Dict(1401)))]
pub struct IpvpnGatewayPeers<'a, Mode> {
    #[data_view(schema = Str(1116))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1108))]
    pub password: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1109))]
    pub cleartext_password: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(37))]
    pub bfd: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1393), relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_config::router_bgp::peer_groups::Item<'a, ::validated_data::RelaxedValidated>>,
}

pub mod ipvpn_gateway_peers {
}

#[::validated_data::data_view(python(name = "AVDDesign.BgpPeerGroups.WanOverlayPeers", schema = Dict(1403)))]
pub struct WanOverlayPeers<'a, Mode> {
    #[data_view(schema = Str(1117))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1118))]
    pub password: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1119))]
    pub cleartext_password: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(37))]
    pub bfd: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1402))]
    pub bfd_timers: ::validated_data::Field<wan_overlay_peers::BfdTimers<'a, Mode>>,
    #[data_view(schema = List(564))]
    pub listen_range_prefixes: ::validated_data::Field<wan_overlay_peers::ListenRangePrefixes<'a, Mode>>,
    #[data_view(schema = Int(505))]
    pub ttl_maximum_hops: ::validated_data::Field<i64>,
    #[data_view(schema = Dict(1393), relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_config::router_bgp::peer_groups::Item<'a, ::validated_data::RelaxedValidated>>,
}

pub mod wan_overlay_peers {

    #[::validated_data::data_view(python(name = "AVDDesign.BgpPeerGroups.WanOverlayPeers.BfdTimers", schema = Dict(1402)))]
    pub struct BfdTimers<'a, Mode> {
        #[data_view(schema = Int(502))]
        pub interval: ::validated_data::RequiredValue<i64, Mode>,
        #[data_view(schema = Int(503))]
        pub min_rx: ::validated_data::RequiredValue<i64, Mode>,
        #[data_view(schema = Int(504))]
        pub multiplier: ::validated_data::RequiredValue<i64, Mode>,
    }

    #[::validated_data::data_view(list, python(name = "AVDDesign.BgpPeerGroups.WanOverlayPeers.ListenRangePrefixes", schema = List(564)))]
    pub struct ListenRangePrefixes<'a, Mode> (::validated_data::Field<&'a str>);
}

#[::validated_data::data_view(python(name = "AVDDesign.BgpPeerGroups.WanRrOverlayPeers", schema = Dict(1404)))]
pub struct WanRrOverlayPeers<'a, Mode> {
    #[data_view(schema = Str(1121))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1118))]
    pub password: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1119))]
    pub cleartext_password: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(37))]
    pub bfd: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1402))]
    pub bfd_timers: ::validated_data::Field<wan_rr_overlay_peers::BfdTimers<'a, Mode>>,
    #[data_view(schema = Int(505))]
    pub ttl_maximum_hops: ::validated_data::Field<i64>,
    #[data_view(schema = Dict(1393), relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_config::router_bgp::peer_groups::Item<'a, ::validated_data::RelaxedValidated>>,
}

pub mod wan_rr_overlay_peers {

    #[::validated_data::data_view(python(name = "AVDDesign.BgpPeerGroups.WanRrOverlayPeers.BfdTimers", schema = Dict(1402)))]
    pub struct BfdTimers<'a, Mode> {
        #[data_view(schema = Int(502))]
        pub interval: ::validated_data::RequiredValue<i64, Mode>,
        #[data_view(schema = Int(503))]
        pub min_rx: ::validated_data::RequiredValue<i64, Mode>,
        #[data_view(schema = Int(504))]
        pub multiplier: ::validated_data::RequiredValue<i64, Mode>,
    }
}
