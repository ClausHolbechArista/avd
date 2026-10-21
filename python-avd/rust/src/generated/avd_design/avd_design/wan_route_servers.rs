// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(hostname), python(name = "AVDDesign.WanRouteServers", schema = List(793), item_name = "AVDDesign.WanRouteServersItem"))]
pub struct WanRouteServers<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.WanRouteServersItemBase", schema = Dict(1788)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1685))]
    pub hostname: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1686))]
    pub vtep_ip: ::data_views::Field<&'a str>,
    #[data_view(schema = List(792))]
    pub path_groups: ::data_views::Field<item::PathGroups<'a, Mode>>,
}

pub mod item {

    #[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.WanRouteServersItemBase.PathGroups", schema = List(792), item_name = "AVDDesign.WanRouteServersItemBase.PathGroupsItem"))]
    pub struct PathGroups<'a, Mode> (::data_views::Field<path_groups::Item<'a, Mode>>);

    pub mod path_groups {

        #[::data_views::data_view(python(name = "AVDDesign.WanRouteServersItemBase.PathGroupsItemBase", schema = Dict(1787)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(892))]
            pub name: ::data_views::Field<&'a str>,
            #[data_view(schema = List(791))]
            pub interfaces: ::data_views::RequiredValue<item::Interfaces<'a, Mode>, Mode>,
        }

        pub mod item {

            #[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.WanRouteServersItemBase.PathGroupsItemBase.Interfaces", schema = List(791), item_name = "AVDDesign.WanRouteServersItemBase.PathGroupsItemBase.InterfacesItem"))]
            pub struct Interfaces<'a, Mode> (::data_views::Field<interfaces::Item<'a, Mode>>);

            pub mod interfaces {

                #[::data_views::data_view(python(name = "AVDDesign.WanRouteServersItemBase.PathGroupsItemBase.InterfacesItemBase", schema = Dict(1786)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(30))]
                    pub name: ::data_views::Field<&'a str>,
                    #[data_view(schema = Str(1687))]
                    pub public_ip: ::data_views::Field<&'a str>,
                }
            }
        }
    }
}
