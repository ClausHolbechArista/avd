// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar description("description", 1) -> &'a str;
        scalar shutdown("shutdown", 2) -> bool;
        scalar vrf("vrf", 3) -> &'a str;
        scalar ip_address("ip_address", 4) -> &'a str;
        model ip_address_secondaries("ip_address_secondaries", 5) -> item::IpAddressSecondaries<'a>;
        scalar ipv6_enable("ipv6_enable", 6) -> bool;
        scalar ipv6_address("ipv6_address", 7) -> &'a str;
        model ipv6_addresses("ipv6_addresses", 8) -> item::Ipv6Addresses<'a>;
        scalar ipv6_address_auto_config("ipv6_address_auto_config", 9) -> bool;
        scalar ip_proxy_arp("ip_proxy_arp", 10) -> bool;
        scalar ospf_area("ospf_area", 11) -> &'a str;
        model mpls("mpls", 12) -> item::Mpls<'a>;
        scalar isis_enable("isis_enable", 13) -> &'a str;
        scalar isis_bfd("isis_bfd", 14) -> bool;
        scalar isis_passive("isis_passive", 15) -> bool;
        scalar isis_metric("isis_metric", 16) -> i64;
        scalar isis_network_point_to_point("isis_network_point_to_point", 17) -> bool;
        model node_segment("node_segment", 18) -> item::NodeSegment<'a>;
        scalar hardware_forwarding_id("hardware_forwarding_id", 19) -> bool;
        scalar eos_cli("eos_cli", 20) -> &'a str;
    }
}

pub mod item {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct IpAddressSecondaries {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipv6Addresses {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Mpls {
            model ldp("ldp", 0) -> mpls::Ldp<'a>;
        }
    }

    pub mod mpls {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ldp {
                scalar interface("interface", 0) -> bool;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct NodeSegment {
            scalar ipv4_index("ipv4_index", 0) -> i64;
            scalar ipv6_index("ipv6_index", 1) -> i64;
        }
    }
}
