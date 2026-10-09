// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub profile: ::validated_data::Field<&'a str>,
    pub parent_profile: ::validated_data::Field<&'a str>,
    pub address_locking: ::validated_data::Field<super::super::eos_cli_config_gen::vlans::item::address_locking::AddressFamily<'a, Mode>>,
    pub vni_override: ::validated_data::Field<i64>,
    pub rt_override: ::validated_data::Field<&'a str>,
    pub rd_override: ::validated_data::Field<&'a str>,
    pub vxlan: ::validated_data::Field<bool>,
    pub spanning_tree_priority: ::validated_data::Field<i64>,
    pub evpn_vlan_bundle: ::validated_data::Field<&'a str>,
    pub trunk_groups: ::validated_data::Field<item::TrunkGroups<'a, Mode>>,
    pub evpn_l2_multi_domain: ::validated_data::Field<bool>,
    pub evpn_l2_multicast: ::validated_data::Field<item::EvpnL2Multicast<'a, Mode>>,
    pub vxlan_flood_multicast: ::validated_data::Field<item::VxlanFloodMulticast<'a, Mode>>,
    pub igmp_snooping: ::validated_data::Field<item::IgmpSnooping<'a, Mode>>,
    pub igmp_snooping_enabled: ::validated_data::Field<bool>,
    pub igmp_snooping_querier: ::validated_data::Field<item::IgmpSnoopingQuerier<'a, Mode>>,
    pub bgp: ::validated_data::Field<item::Bgp<'a, Mode>>,
    pub private_vlan: ::validated_data::Field<item::PrivateVlan<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view(list)]
    pub struct TrunkGroups<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view]
    pub struct EvpnL2Multicast<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view]
    pub struct VxlanFloodMulticast<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub underlay_multicast_group: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct IgmpSnooping<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub querier: ::validated_data::Field<igmp_snooping::Querier<'a, Mode>>,
        pub fast_leave: ::validated_data::Field<bool>,
    }

    pub mod igmp_snooping {

        #[::validated_data::data_view]
        pub struct Querier<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub source_address: ::validated_data::Field<&'a str>,
            pub version: ::validated_data::Field<i64>,
        }
    }

    #[::validated_data::data_view]
    pub struct IgmpSnoopingQuerier<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub source_address: ::validated_data::Field<&'a str>,
        pub version: ::validated_data::Field<i64>,
        pub fast_leave: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view]
    pub struct Bgp<'a, Mode> {
        #[data_view(relaxed)]
        pub structured_config: ::validated_data::Field<super::super::super::eos_cli_config_gen::router_bgp::vlans::Item<'a, ::validated_data::RelaxedValidated>>,
        pub raw_eos_cli: ::validated_data::Field<&'a str>,
    }

    pub mod bgp {
    }

    #[::validated_data::data_view]
    pub struct PrivateVlan<'a, Mode> {
        #[data_view(rename = "type")]
        pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
        pub primary_vlan: ::validated_data::RequiredValue<i64, Mode>,
    }
}
