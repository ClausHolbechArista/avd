// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.EvpnVlanBundlesItemBase", schema = Dict(1553)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1396))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(576))]
    pub id: ::validated_data::RequiredValue<i64, Mode>,
    #[data_view(schema = Str(1397))]
    pub rt_override: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1398))]
    pub rd_override: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(525))]
    pub evpn_l2_multi_domain: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1552))]
    pub bgp: ::validated_data::Field<item::Bgp<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view(python(name = "AVDDesign.EvpnVlanBundlesItemBase.Bgp", schema = Dict(1552)))]
    pub struct Bgp<'a, Mode> {
        #[data_view(schema = Str(1399))]
        pub raw_eos_cli: ::validated_data::Field<&'a str>,
    }
}
