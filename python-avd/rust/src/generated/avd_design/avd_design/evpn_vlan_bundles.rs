// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.EvpnVlanBundles", schema = List(651), item_name = "AVDDesign.EvpnVlanBundlesItem"))]
pub struct EvpnVlanBundles<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.EvpnVlanBundlesItemBase", schema = Dict(1553)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1396))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = Int(576))]
    pub id: ::data_views::RequiredValue<i64, Mode>,
    #[data_view(schema = Str(1397))]
    pub rt_override: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1398))]
    pub rd_override: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(525))]
    pub evpn_l2_multi_domain: ::data_views::Field<bool>,
    #[data_view(schema = Dict(1552))]
    pub bgp: ::data_views::Field<item::Bgp<'a, Mode>>,
}

pub mod item {

    #[::data_views::data_view(python(name = "AVDDesign.EvpnVlanBundlesItemBase.Bgp", schema = Dict(1552)))]
    pub struct Bgp<'a, Mode> {
        #[data_view(schema = Str(1399))]
        pub raw_eos_cli: ::data_views::Field<&'a str>,
    }
}
