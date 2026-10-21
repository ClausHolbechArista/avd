// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.SflowSettings", schema = Dict(1743)))]
pub struct SflowSettings<'a, Mode> {
    #[data_view(schema = Int(630))]
    pub polling_interval: ::data_views::Field<i64>,
    #[data_view(schema = Dict(1739))]
    pub sample: ::data_views::Field<Sample<'a, Mode>>,
    #[data_view(schema = List(770))]
    pub destinations: ::data_views::Field<Destinations<'a, Mode>>,
    #[data_view(schema = Dict(1741))]
    pub export_to_cloudvision: ::data_views::Field<ExportToCloudvision<'a, Mode>>,
    #[data_view(schema = List(771))]
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
}

#[::data_views::data_view(python(name = "AVDDesign.SflowSettings.Sample", schema = Dict(1739)))]
pub struct Sample<'a, Mode> {
    #[data_view(schema = Int(631))]
    pub rate: ::data_views::Field<i64>,
}

#[::data_views::data_view(list, python(name = "AVDDesign.SflowSettings.Destinations", schema = List(770)))]
pub struct Destinations<'a, Mode> (::data_views::Field<destinations::Item<'a, Mode>>);

pub mod destinations {

    #[::data_views::data_view(python(name = "AVDDesign.SflowSettings.DestinationsItem", schema = Dict(1740)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1623))]
        pub destination: ::data_views::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Int(632))]
        pub port: ::data_views::Field<i64>,
        #[data_view(schema = Str(1624))]
        pub vrf: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view(python(name = "AVDDesign.SflowSettings.ExportToCloudvision", schema = Dict(1741)))]
pub struct ExportToCloudvision<'a, Mode> {
    #[data_view(schema = Bool(641))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = Str(1625))]
    pub vrf: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.SflowSettings.Vrfs", schema = List(771), item_name = "AVDDesign.SflowSettings.VrfsItem"))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view(python(name = "AVDDesign.SflowSettings.VrfsItemBase", schema = Dict(1742)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(25))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1626))]
        pub source_interface: ::data_views::Field<&'a str>,
    }
}
