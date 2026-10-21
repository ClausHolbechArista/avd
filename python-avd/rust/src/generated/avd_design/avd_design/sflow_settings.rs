// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "AVDDesign.SflowSettings", schema = Dict(1743)))]
pub struct SflowSettings<'a, Mode> {
    #[data_view(schema = Int(630))]
    pub polling_interval: ::validated_data::Field<i64>,
    #[data_view(schema = Dict(1739))]
    pub sample: ::validated_data::Field<Sample<'a, Mode>>,
    #[data_view(schema = List(770))]
    pub destinations: ::validated_data::Field<Destinations<'a, Mode>>,
    #[data_view(schema = Dict(1741))]
    pub export_to_cloudvision: ::validated_data::Field<ExportToCloudvision<'a, Mode>>,
    #[data_view(schema = List(771))]
    pub vrfs: ::validated_data::Field<Vrfs<'a, Mode>>,
}

#[::validated_data::data_view(python(name = "AVDDesign.SflowSettings.Sample", schema = Dict(1739)))]
pub struct Sample<'a, Mode> {
    #[data_view(schema = Int(631))]
    pub rate: ::validated_data::Field<i64>,
}

#[::validated_data::data_view(list, python(name = "AVDDesign.SflowSettings.Destinations", schema = List(770)))]
pub struct Destinations<'a, Mode> (::validated_data::Field<destinations::Item<'a, Mode>>);

pub mod destinations {

    #[::validated_data::data_view(python(name = "AVDDesign.SflowSettings.DestinationsItem", schema = Dict(1740)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1623))]
        pub destination: ::validated_data::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Int(632))]
        pub port: ::validated_data::Field<i64>,
        #[data_view(schema = Str(1624))]
        pub vrf: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(python(name = "AVDDesign.SflowSettings.ExportToCloudvision", schema = Dict(1741)))]
pub struct ExportToCloudvision<'a, Mode> {
    #[data_view(schema = Bool(641))]
    pub enabled: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1625))]
    pub vrf: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.SflowSettings.Vrfs", schema = List(771), item_name = "AVDDesign.SflowSettings.VrfsItem"))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view(python(name = "AVDDesign.SflowSettings.VrfsItemBase", schema = Dict(1742)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(25))]
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1626))]
        pub source_interface: ::validated_data::Field<&'a str>,
    }
}
