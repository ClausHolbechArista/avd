// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.WanPathGroupsItemBase", schema = Dict(1785)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(892))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(643))]
    pub id: ::validated_data::RequiredValue<i64, Mode>,
    #[data_view(schema = Str(1680))]
    pub description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1782))]
    pub ipsec: ::validated_data::Field<item::Ipsec<'a, Mode>>,
    #[data_view(schema = List(789))]
    pub import_path_groups: ::validated_data::Field<item::ImportPathGroups<'a, Mode>>,
    #[data_view(schema = Str(1683))]
    pub default_preference: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(679))]
    pub excluded_from_default_policy: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1784))]
    pub dps_keepalive: ::validated_data::Field<item::DpsKeepalive<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view(python(name = "AVDDesign.WanPathGroupsItemBase.Ipsec", schema = Dict(1782)))]
    pub struct Ipsec<'a, Mode> {
        #[data_view(schema = Bool(677))]
        pub dynamic_peers: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(678))]
        pub static_peers: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(list, python(name = "AVDDesign.WanPathGroupsItemBase.ImportPathGroups", schema = List(789)))]
    pub struct ImportPathGroups<'a, Mode> (::validated_data::Field<import_path_groups::Item<'a, Mode>>);

    pub mod import_path_groups {

        #[::validated_data::data_view(python(name = "AVDDesign.WanPathGroupsItemBase.ImportPathGroupsItem", schema = Dict(1783)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(1681))]
            pub remote: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1682))]
            pub local: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesign.WanPathGroupsItemBase.DpsKeepalive", schema = Dict(1784)))]
    pub struct DpsKeepalive<'a, Mode> {
        #[data_view(schema = Str(1684))]
        pub interval: ::validated_data::Field<&'a str>,
        #[data_view(schema = Int(644))]
        pub failure_threshold: ::validated_data::Field<i64>,
    }
}
