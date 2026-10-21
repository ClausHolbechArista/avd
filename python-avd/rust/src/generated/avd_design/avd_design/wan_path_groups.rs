// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.WanPathGroups", schema = List(790), item_name = "AVDDesign.WanPathGroupsItem"))]
pub struct WanPathGroups<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.WanPathGroupsItemBase", schema = Dict(1785)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(892))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = Int(643))]
    pub id: ::data_views::RequiredValue<i64, Mode>,
    #[data_view(schema = Str(1680))]
    pub description: ::data_views::Field<&'a str>,
    #[data_view(schema = Dict(1782))]
    pub ipsec: ::data_views::Field<item::Ipsec<'a, Mode>>,
    #[data_view(schema = List(789))]
    pub import_path_groups: ::data_views::Field<item::ImportPathGroups<'a, Mode>>,
    #[data_view(schema = Str(1683))]
    pub default_preference: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(679))]
    pub excluded_from_default_policy: ::data_views::Field<bool>,
    #[data_view(schema = Dict(1784))]
    pub dps_keepalive: ::data_views::Field<item::DpsKeepalive<'a, Mode>>,
}

pub mod item {

    #[::data_views::data_view(python(name = "AVDDesign.WanPathGroupsItemBase.Ipsec", schema = Dict(1782)))]
    pub struct Ipsec<'a, Mode> {
        #[data_view(schema = Bool(677))]
        pub dynamic_peers: ::data_views::Field<bool>,
        #[data_view(schema = Bool(678))]
        pub static_peers: ::data_views::Field<bool>,
    }

    #[::data_views::data_view(list, python(name = "AVDDesign.WanPathGroupsItemBase.ImportPathGroups", schema = List(789)))]
    pub struct ImportPathGroups<'a, Mode> (::data_views::Field<import_path_groups::Item<'a, Mode>>);

    pub mod import_path_groups {

        #[::data_views::data_view(python(name = "AVDDesign.WanPathGroupsItemBase.ImportPathGroupsItem", schema = Dict(1783)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(1681))]
            pub remote: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1682))]
            pub local: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view(python(name = "AVDDesign.WanPathGroupsItemBase.DpsKeepalive", schema = Dict(1784)))]
    pub struct DpsKeepalive<'a, Mode> {
        #[data_view(schema = Str(1684))]
        pub interval: ::data_views::Field<&'a str>,
        #[data_view(schema = Int(644))]
        pub failure_threshold: ::data_views::Field<i64>,
    }
}
