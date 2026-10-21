// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(profile), python(name = "AVDDesign.L2vlanProfiles", schema = List(668), item_name = "AVDDesign.L2vlanProfilesItem"))]
pub struct L2vlanProfiles<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.L2vlanProfilesItemBase", schema = Dict(1613)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(608))]
    pub profile: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1431))]
    pub parent_profile: ::data_views::Field<&'a str>,
    #[data_view(schema = Dict(1340))]
    pub address_locking: ::data_views::Field<super::super::eos_config::vlans::item::address_locking::AddressFamily<'a, Mode>>,
    #[data_view(schema = Int(593))]
    pub vni_override: ::data_views::Field<i64>,
    #[data_view(schema = Str(1432))]
    pub rt_override: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1433))]
    pub rd_override: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(543))]
    pub vxlan: ::data_views::Field<bool>,
    #[data_view(schema = Int(594))]
    pub spanning_tree_priority: ::data_views::Field<i64>,
    #[data_view(schema = Str(1434))]
    pub evpn_vlan_bundle: ::data_views::Field<&'a str>,
    #[data_view(schema = List(667))]
    pub trunk_groups: ::data_views::Field<item::TrunkGroups<'a, Mode>>,
    #[data_view(schema = Bool(544))]
    pub evpn_l2_multi_domain: ::data_views::Field<bool>,
    #[data_view(schema = Dict(1605))]
    pub evpn_l2_multicast: ::data_views::Field<item::EvpnL2Multicast<'a, Mode>>,
    #[data_view(schema = Dict(1606))]
    pub vxlan_flood_multicast: ::data_views::Field<item::VxlanFloodMulticast<'a, Mode>>,
    #[data_view(schema = Dict(1608))]
    pub igmp_snooping: ::data_views::Field<item::IgmpSnooping<'a, Mode>>,
    #[data_view(schema = Bool(549))]
    pub igmp_snooping_enabled: ::data_views::Field<bool>,
    #[data_view(schema = Dict(1609))]
    pub igmp_snooping_querier: ::data_views::Field<item::IgmpSnoopingQuerier<'a, Mode>>,
    #[data_view(schema = Dict(1611))]
    pub bgp: ::data_views::Field<item::Bgp<'a, Mode>>,
    #[data_view(schema = Dict(1612))]
    pub private_vlan: ::data_views::Field<item::PrivateVlan<'a, Mode>>,
}

pub mod item {

    #[::data_views::data_view(list, python(name = "AVDDesign.L2vlanProfilesItemBase.TrunkGroups", schema = List(667)))]
    pub struct TrunkGroups<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(python(name = "AVDDesign.L2vlanProfilesItemBase.EvpnL2Multicast", schema = Dict(1605)))]
    pub struct EvpnL2Multicast<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub enabled: ::data_views::Field<bool>,
    }

    #[::data_views::data_view(python(name = "AVDDesign.L2vlanProfilesItemBase.VxlanFloodMulticast", schema = Dict(1606)))]
    pub struct VxlanFloodMulticast<'a, Mode> {
        #[data_view(schema = Bool(545))]
        pub enabled: ::data_views::Field<bool>,
        #[data_view(schema = Str(1436))]
        pub underlay_multicast_group: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view(python(name = "AVDDesign.L2vlanProfilesItemBase.IgmpSnooping", schema = Dict(1608)))]
    pub struct IgmpSnooping<'a, Mode> {
        #[data_view(schema = Bool(546))]
        pub enabled: ::data_views::Field<bool>,
        #[data_view(schema = Dict(1607))]
        pub querier: ::data_views::Field<igmp_snooping::Querier<'a, Mode>>,
        #[data_view(schema = Bool(548))]
        pub fast_leave: ::data_views::Field<bool>,
    }

    pub mod igmp_snooping {

        #[::data_views::data_view(python(name = "AVDDesign.L2vlanProfilesItemBase.IgmpSnooping.Querier", schema = Dict(1607)))]
        pub struct Querier<'a, Mode> {
            #[data_view(schema = Bool(547))]
            pub enabled: ::data_views::Field<bool>,
            #[data_view(schema = Str(1437))]
            pub source_address: ::data_views::Field<&'a str>,
            #[data_view(schema = Int(595))]
            pub version: ::data_views::Field<i64>,
        }
    }

    #[::data_views::data_view(python(name = "AVDDesign.L2vlanProfilesItemBase.IgmpSnoopingQuerier", schema = Dict(1609)))]
    pub struct IgmpSnoopingQuerier<'a, Mode> {
        #[data_view(schema = Bool(550))]
        pub enabled: ::data_views::Field<bool>,
        #[data_view(schema = Str(1438))]
        pub source_address: ::data_views::Field<&'a str>,
        #[data_view(schema = Int(595))]
        pub version: ::data_views::Field<i64>,
        #[data_view(schema = Bool(548))]
        pub fast_leave: ::data_views::Field<bool>,
    }

    #[::data_views::data_view(python(name = "AVDDesign.L2vlanProfilesItemBase.Bgp", schema = Dict(1611)))]
    pub struct Bgp<'a, Mode> {
        #[data_view(schema = Dict(1610), relaxed)]
        pub structured_config: ::data_views::Field<super::super::super::eos_config::router_bgp::vlans::Item<'a, ::data_views::RelaxedValidated>>,
        #[data_view(schema = Str(1439))]
        pub raw_eos_cli: ::data_views::Field<&'a str>,
    }

    pub mod bgp {
    }

    #[::data_views::data_view(python(name = "AVDDesign.L2vlanProfilesItemBase.PrivateVlan", schema = Dict(1612)))]
    pub struct PrivateVlan<'a, Mode> {
        #[data_view(schema = Str(1440), rename = "type")]
        pub field_type: ::data_views::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Int(596))]
        pub primary_vlan: ::data_views::RequiredValue<i64, Mode>,
    }
}
