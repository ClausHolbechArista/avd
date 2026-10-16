// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.GenerateCvTags.InterfaceTags", schema = List(655), item_name = "AVDDesign.GenerateCvTags.InterfaceTagsItem"))]
pub struct InterfaceTags<'a, Mode> (::validated_data::Field<interface_tags::Item<'a, Mode>>);

pub mod interface_tags {

    #[::validated_data::data_view(python(name = "AVDDesign.GenerateCvTags.InterfaceTagsItemBase", schema = Dict(1588)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1413))]
        pub name: ::validated_data::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Str(1414))]
        pub data_path: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1415))]
        pub value: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(list, python(name = "AVDDesign.GenerateCvTags.DeviceTags", schema = List(656)))]
pub struct DeviceTags<'a, Mode> (::validated_data::Field<device_tags::Item<'a, Mode>>);

pub mod device_tags {

    #[::validated_data::data_view(python(name = "AVDDesign.GenerateCvTags.DeviceTagsItem", schema = Dict(1589)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1413))]
        pub name: ::validated_data::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Str(1416))]
        pub data_path: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1415))]
        pub value: ::validated_data::Field<&'a str>,
    }
}
