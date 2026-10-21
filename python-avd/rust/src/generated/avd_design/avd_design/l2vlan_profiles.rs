// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(indexed_list, primary_key(profile), python(name = "AVDDesign.L2vlanProfiles", schema = List(668), item_name = "AVDDesign.L2vlanProfilesItem"))]
pub struct L2vlanProfiles<'a, Mode> (::validated_data::Field<Item<'a, Mode>>);

#[::validated_data::data_view(python(name = "AVDDesign.L2vlanProfilesItemBase", schema = Dict(1613)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(608))]
    pub profile: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1431))]
    pub parent_profile: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1340))]
    pub address_locking: ::validated_data::Field<super::super::eos_config::vlans::item::address_locking::AddressFamily<'a, Mode>>,
    #[data_view(schema = Int(593))]
    pub vni_override: ::validated_data::Field<i64>,
    #[data_view(schema = Str(1432))]
    pub rt_override: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1433))]
    pub rd_override: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(543))]
    pub vxlan: ::validated_data::Field<bool>,
    #[data_view(schema = Int(594))]
    pub spanning_tree_priority: ::validated_data::Field<i64>,
    #[data_view(schema = Str(1434))]
    pub evpn_vlan_bundle: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(667))]
    pub trunk_groups: ::validated_data::Field<item::TrunkGroups<'a, Mode>>,
    #[data_view(schema = Bool(544))]
    pub evpn_l2_multi_domain: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1605))]
    pub evpn_l2_multicast: ::validated_data::Field<item::EvpnL2Multicast<'a, Mode>>,
    #[data_view(schema = Dict(1606))]
    pub vxlan_flood_multicast: ::validated_data::Field<item::VxlanFloodMulticast<'a, Mode>>,
    #[data_view(schema = Dict(1608))]
    pub igmp_snooping: ::validated_data::Field<item::IgmpSnooping<'a, Mode>>,
    #[data_view(schema = Bool(549))]
    pub igmp_snooping_enabled: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1609))]
    pub igmp_snooping_querier: ::validated_data::Field<item::IgmpSnoopingQuerier<'a, Mode>>,
    #[data_view(schema = Dict(1611))]
    pub bgp: ::validated_data::Field<item::Bgp<'a, Mode>>,
    #[data_view(schema = Dict(1612))]
    pub private_vlan: ::validated_data::Field<item::PrivateVlan<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view(list, python(name = "AVDDesign.L2vlanProfilesItemBase.TrunkGroups", schema = List(667)))]
    pub struct TrunkGroups<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(python(name = "AVDDesign.L2vlanProfilesItemBase.EvpnL2Multicast", schema = Dict(1605)))]
    pub struct EvpnL2Multicast<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub enabled: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.L2vlanProfilesItemBase.VxlanFloodMulticast", schema = Dict(1606)))]
    pub struct VxlanFloodMulticast<'a, Mode> {
        #[data_view(schema = Bool(545))]
        pub enabled: ::validated_data::Field<bool>,
        #[data_view(schema = Str(1436))]
        pub underlay_multicast_group: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.L2vlanProfilesItemBase.IgmpSnooping", schema = Dict(1608)))]
    pub struct IgmpSnooping<'a, Mode> {
        #[data_view(schema = Bool(546))]
        pub enabled: ::validated_data::Field<bool>,
        #[data_view(schema = Dict(1607))]
        pub querier: ::validated_data::Field<igmp_snooping::Querier<'a, Mode>>,
        #[data_view(schema = Bool(548))]
        pub fast_leave: ::validated_data::Field<bool>,
    }

    pub mod igmp_snooping {

        #[::validated_data::data_view(python(name = "AVDDesign.L2vlanProfilesItemBase.IgmpSnooping.Querier", schema = Dict(1607)))]
        pub struct Querier<'a, Mode> {
            #[data_view(schema = Bool(547))]
            pub enabled: ::validated_data::Field<bool>,
            #[data_view(schema = Str(1437))]
            pub source_address: ::validated_data::Field<&'a str>,
            #[data_view(schema = Int(595))]
            pub version: ::validated_data::Field<i64>,
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesign.L2vlanProfilesItemBase.IgmpSnoopingQuerier", schema = Dict(1609)))]
    pub struct IgmpSnoopingQuerier<'a, Mode> {
        #[data_view(schema = Bool(550))]
        pub enabled: ::validated_data::Field<bool>,
        #[data_view(schema = Str(1438))]
        pub source_address: ::validated_data::Field<&'a str>,
        #[data_view(schema = Int(595))]
        pub version: ::validated_data::Field<i64>,
        #[data_view(schema = Bool(548))]
        pub fast_leave: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.L2vlanProfilesItemBase.Bgp", schema = Dict(1611)))]
    pub struct Bgp<'a, Mode> {
        #[data_view(schema = Dict(1610), relaxed)]
        pub structured_config: ::validated_data::Field<super::super::super::eos_config::router_bgp::vlans::Item<'a, ::validated_data::RelaxedValidated>>,
        #[data_view(schema = Str(1439))]
        pub raw_eos_cli: ::validated_data::Field<&'a str>,
    }

    pub mod bgp {
    }

    #[::validated_data::data_view(python(name = "AVDDesign.L2vlanProfilesItemBase.PrivateVlan", schema = Dict(1612)))]
    pub struct PrivateVlan<'a, Mode> {
        #[data_view(schema = Str(1440), rename = "type")]
        pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Int(596))]
        pub primary_vlan: ::validated_data::RequiredValue<i64, Mode>,
    }
}
