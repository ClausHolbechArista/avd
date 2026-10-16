// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.WanRouteServersItemBase", schema = Dict(1788)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1685))]
    pub hostname: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1686))]
    pub vtep_ip: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(792))]
    pub path_groups: ::validated_data::Field<item::PathGroups<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.WanRouteServersItemBase.PathGroups", schema = List(792), item_name = "AVDDesign.WanRouteServersItemBase.PathGroupsItem"))]
    pub struct PathGroups<'a, Mode> (::validated_data::Field<path_groups::Item<'a, Mode>>);

    pub mod path_groups {

        #[::validated_data::data_view(python(name = "AVDDesign.WanRouteServersItemBase.PathGroupsItemBase", schema = Dict(1787)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(892))]
            pub name: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(791))]
            pub interfaces: ::validated_data::RequiredValue<item::Interfaces<'a, Mode>, Mode>,
        }

        pub mod item {

            #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.WanRouteServersItemBase.PathGroupsItemBase.Interfaces", schema = List(791), item_name = "AVDDesign.WanRouteServersItemBase.PathGroupsItemBase.InterfacesItem"))]
            pub struct Interfaces<'a, Mode> (::validated_data::Field<interfaces::Item<'a, Mode>>);

            pub mod interfaces {

                #[::validated_data::data_view(python(name = "AVDDesign.WanRouteServersItemBase.PathGroupsItemBase.InterfacesItemBase", schema = Dict(1786)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(30))]
                    pub name: ::validated_data::Field<&'a str>,
                    #[data_view(schema = Str(1687))]
                    pub public_ip: ::validated_data::Field<&'a str>,
                }
            }
        }
    }
}
