// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar profile("profile", 0) -> &'a str;
        scalar parent_profile("parent_profile", 1) -> &'a str;
        model address_locking("address_locking", 2) -> super::super::eos_cli_config_gen::vlans::item::address_locking::AddressFamily<'a>;
        scalar vni_override("vni_override", 3) -> i64;
        scalar rt_override("rt_override", 4) -> &'a str;
        scalar rd_override("rd_override", 5) -> &'a str;
        scalar vxlan("vxlan", 6) -> bool;
        scalar spanning_tree_priority("spanning_tree_priority", 7) -> i64;
        scalar evpn_vlan_bundle("evpn_vlan_bundle", 8) -> &'a str;
        model trunk_groups("trunk_groups", 9) -> item::TrunkGroups<'a>;
        scalar evpn_l2_multi_domain("evpn_l2_multi_domain", 10) -> bool;
        model evpn_l2_multicast("evpn_l2_multicast", 11) -> item::EvpnL2Multicast<'a>;
        model vxlan_flood_multicast("vxlan_flood_multicast", 12) -> item::VxlanFloodMulticast<'a>;
        model igmp_snooping("igmp_snooping", 13) -> item::IgmpSnooping<'a>;
        scalar igmp_snooping_enabled("igmp_snooping_enabled", 14) -> bool;
        model igmp_snooping_querier("igmp_snooping_querier", 15) -> item::IgmpSnoopingQuerier<'a>;
        model bgp("bgp", 16) -> item::Bgp<'a>;
        model private_vlan("private_vlan", 17) -> item::PrivateVlan<'a>;
    }
}

pub mod item {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct TrunkGroups {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct EvpnL2Multicast {
            scalar enabled("enabled", 0) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct VxlanFloodMulticast {
            scalar enabled("enabled", 0) -> bool;
            scalar underlay_multicast_group("underlay_multicast_group", 1) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct IgmpSnooping {
            scalar enabled("enabled", 0) -> bool;
            model querier("querier", 1) -> igmp_snooping::Querier<'a>;
            scalar fast_leave("fast_leave", 2) -> bool;
        }
    }

    pub mod igmp_snooping {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Querier {
                scalar enabled("enabled", 0) -> bool;
                scalar source_address("source_address", 1) -> &'a str;
                scalar version("version", 2) -> i64;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct IgmpSnoopingQuerier {
            scalar enabled("enabled", 0) -> bool;
            scalar source_address("source_address", 1) -> &'a str;
            scalar version("version", 2) -> i64;
            scalar fast_leave("fast_leave", 3) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Bgp {
            model structured_config("structured_config", 0) -> super::super::super::eos_cli_config_gen::router_bgp::vlans::Item<'a>;
            scalar raw_eos_cli("raw_eos_cli", 1) -> &'a str;
        }
    }

    pub mod bgp {
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PrivateVlan {
            scalar field_type("type", 0) -> &'a str;
            scalar primary_vlan("primary_vlan", 1) -> i64;
        }
    }
}
