// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.PlatformSpeedGroupsItemBase"))]
pub struct Item<'a, Mode> {
    pub platform: ::validated_data::Field<&'a str>,
    pub speeds: ::validated_data::Field<item::Speeds<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view(indexed_list, primary_key(speed), python(name = "AVDDesign.PlatformSpeedGroupsItemBase.Speeds", item_name = "AVDDesign.PlatformSpeedGroupsItemBase.SpeedsItem"))]
    pub struct Speeds<'a, Mode> (::validated_data::Field<speeds::Item<'a, Mode>>);

    pub mod speeds {

        #[::validated_data::data_view(python(name = "AVDDesign.PlatformSpeedGroupsItemBase.SpeedsItemBase"))]
        pub struct Item<'a, Mode> {
            pub speed: ::validated_data::Field<&'a str>,
            pub speed_groups: ::validated_data::Field<item::SpeedGroups<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "AVDDesign.PlatformSpeedGroupsItemBase.SpeedsItemBase.SpeedGroups"))]
            pub struct SpeedGroups<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }
}
