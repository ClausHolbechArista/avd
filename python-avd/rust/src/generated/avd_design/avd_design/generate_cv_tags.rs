// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.GenerateCvTags", schema = Dict(1590)))]
pub struct GenerateCvTags<'a, Mode> {
    #[data_view(schema = Bool(538))]
    pub topology_hints: ::data_views::Field<bool>,
    #[data_view(schema = Bool(539))]
    pub campus_fabric: ::data_views::Field<bool>,
    #[data_view(schema = List(655))]
    pub interface_tags: ::data_views::Field<InterfaceTags<'a, Mode>>,
    #[data_view(schema = List(656))]
    pub device_tags: ::data_views::Field<DeviceTags<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.GenerateCvTags.InterfaceTags", schema = List(655), item_name = "AVDDesign.GenerateCvTags.InterfaceTagsItem"))]
pub struct InterfaceTags<'a, Mode> (::data_views::Field<interface_tags::Item<'a, Mode>>);

pub mod interface_tags {

    #[::data_views::data_view(python(name = "AVDDesign.GenerateCvTags.InterfaceTagsItemBase", schema = Dict(1588)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1413))]
        pub name: ::data_views::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Str(1414))]
        pub data_path: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1415))]
        pub value: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view(list, python(name = "AVDDesign.GenerateCvTags.DeviceTags", schema = List(656)))]
pub struct DeviceTags<'a, Mode> (::data_views::Field<device_tags::Item<'a, Mode>>);

pub mod device_tags {

    #[::data_views::data_view(python(name = "AVDDesign.GenerateCvTags.DeviceTagsItem", schema = Dict(1589)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1413))]
        pub name: ::data_views::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Str(1416))]
        pub data_path: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1415))]
        pub value: ::data_views::Field<&'a str>,
    }
}
