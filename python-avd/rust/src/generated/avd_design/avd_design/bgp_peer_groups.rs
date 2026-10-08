// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Ipv4UnderlayPeers {
        scalar name("name", 0) -> &'a str;
        scalar password("password", 1) -> &'a str;
        scalar cleartext_password("cleartext_password", 2) -> &'a str;
        scalar bfd("bfd", 3) -> bool;
        scalar maximum_routes("maximum_routes", 4) -> i64;
        model structured_config("structured_config", 5) -> super::super::eos_cli_config_gen::router_bgp::peer_groups::Item<'a>;
    }
}

pub mod ipv4_underlay_peers {
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MlagIpv4VrfsPeer {
        scalar name("name", 0) -> &'a str;
        scalar password("password", 1) -> &'a str;
        scalar cleartext_password("cleartext_password", 2) -> &'a str;
        scalar bfd("bfd", 3) -> bool;
        scalar maximum_routes("maximum_routes", 4) -> i64;
        model structured_config("structured_config", 5) -> super::super::eos_cli_config_gen::router_bgp::peer_groups::Item<'a>;
    }
}

pub mod mlag_ipv4_vrfs_peer {
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MlagIpv4UnderlayPeer {
        scalar name("name", 0) -> &'a str;
        scalar password("password", 1) -> &'a str;
        scalar cleartext_password("cleartext_password", 2) -> &'a str;
        scalar bfd("bfd", 3) -> bool;
        scalar maximum_routes("maximum_routes", 4) -> i64;
        model structured_config("structured_config", 5) -> super::super::eos_cli_config_gen::router_bgp::peer_groups::Item<'a>;
    }
}

pub mod mlag_ipv4_underlay_peer {
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct EvpnOverlayPeers {
        scalar name("name", 0) -> &'a str;
        scalar password("password", 1) -> &'a str;
        scalar cleartext_password("cleartext_password", 2) -> &'a str;
        scalar bfd("bfd", 3) -> bool;
        model structured_config("structured_config", 4) -> super::super::eos_cli_config_gen::router_bgp::peer_groups::Item<'a>;
    }
}

pub mod evpn_overlay_peers {
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct EvpnOverlayCore {
        scalar name("name", 0) -> &'a str;
        scalar password("password", 1) -> &'a str;
        scalar cleartext_password("cleartext_password", 2) -> &'a str;
        scalar bfd("bfd", 3) -> bool;
        model structured_config("structured_config", 4) -> super::super::eos_cli_config_gen::router_bgp::peer_groups::Item<'a>;
    }
}

pub mod evpn_overlay_core {
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MplsOverlayPeers {
        scalar name("name", 0) -> &'a str;
        scalar password("password", 1) -> &'a str;
        scalar cleartext_password("cleartext_password", 2) -> &'a str;
        scalar bfd("bfd", 3) -> bool;
        model structured_config("structured_config", 4) -> super::super::eos_cli_config_gen::router_bgp::peer_groups::Item<'a>;
    }
}

pub mod mpls_overlay_peers {
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RrOverlayPeers {
        scalar name("name", 0) -> &'a str;
        scalar password("password", 1) -> &'a str;
        scalar cleartext_password("cleartext_password", 2) -> &'a str;
        scalar bfd("bfd", 3) -> bool;
        model structured_config("structured_config", 4) -> super::super::eos_cli_config_gen::router_bgp::peer_groups::Item<'a>;
    }
}

pub mod rr_overlay_peers {
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct IpvpnGatewayPeers {
        scalar name("name", 0) -> &'a str;
        scalar password("password", 1) -> &'a str;
        scalar cleartext_password("cleartext_password", 2) -> &'a str;
        scalar bfd("bfd", 3) -> bool;
        model structured_config("structured_config", 4) -> super::super::eos_cli_config_gen::router_bgp::peer_groups::Item<'a>;
    }
}

pub mod ipvpn_gateway_peers {
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct WanOverlayPeers {
        scalar name("name", 0) -> &'a str;
        scalar password("password", 1) -> &'a str;
        scalar cleartext_password("cleartext_password", 2) -> &'a str;
        scalar bfd("bfd", 3) -> bool;
        model bfd_timers("bfd_timers", 4) -> wan_overlay_peers::BfdTimers<'a>;
        model listen_range_prefixes("listen_range_prefixes", 5) -> wan_overlay_peers::ListenRangePrefixes<'a>;
        scalar ttl_maximum_hops("ttl_maximum_hops", 6) -> i64;
        model structured_config("structured_config", 7) -> super::super::eos_cli_config_gen::router_bgp::peer_groups::Item<'a>;
    }
}

pub mod wan_overlay_peers {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct BfdTimers {
            scalar interval("interval", 0) -> i64;
            scalar min_rx("min_rx", 1) -> i64;
            scalar multiplier("multiplier", 2) -> i64;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct ListenRangePrefixes {
            scalar item (0) -> &'a str;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct WanRrOverlayPeers {
        scalar name("name", 0) -> &'a str;
        scalar password("password", 1) -> &'a str;
        scalar cleartext_password("cleartext_password", 2) -> &'a str;
        scalar bfd("bfd", 3) -> bool;
        model bfd_timers("bfd_timers", 4) -> wan_rr_overlay_peers::BfdTimers<'a>;
        scalar ttl_maximum_hops("ttl_maximum_hops", 5) -> i64;
        model structured_config("structured_config", 6) -> super::super::eos_cli_config_gen::router_bgp::peer_groups::Item<'a>;
    }
}

pub mod wan_rr_overlay_peers {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct BfdTimers {
            scalar interval("interval", 0) -> i64;
            scalar min_rx("min_rx", 1) -> i64;
            scalar multiplier("multiplier", 2) -> i64;
        }
    }
}
