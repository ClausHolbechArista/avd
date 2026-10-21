// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(indexed_list, primary_key(platform), python(name = "AVDDesign.PlatformSpeedGroups", schema = List(766), item_name = "AVDDesign.PlatformSpeedGroupsItem"))]
pub struct PlatformSpeedGroups<'a, Mode> (::validated_data::Field<Item<'a, Mode>>);

#[::validated_data::data_view(python(name = "AVDDesign.PlatformSpeedGroupsItemBase", schema = Dict(1729)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(49))]
    pub platform: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(765))]
    pub speeds: ::validated_data::Field<item::Speeds<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view(indexed_list, primary_key(speed), python(name = "AVDDesign.PlatformSpeedGroupsItemBase.Speeds", schema = List(765), item_name = "AVDDesign.PlatformSpeedGroupsItemBase.SpeedsItem"))]
    pub struct Speeds<'a, Mode> (::validated_data::Field<speeds::Item<'a, Mode>>);

    pub mod speeds {

        #[::validated_data::data_view(python(name = "AVDDesign.PlatformSpeedGroupsItemBase.SpeedsItemBase", schema = Dict(1728)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(3))]
            pub speed: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(761))]
            pub speed_groups: ::validated_data::Field<item::SpeedGroups<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "AVDDesign.PlatformSpeedGroupsItemBase.SpeedsItemBase.SpeedGroups", schema = List(761)))]
            pub struct SpeedGroups<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }
}
