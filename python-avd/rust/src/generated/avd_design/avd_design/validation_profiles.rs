// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.ValidationProfiles", schema = List(787), item_name = "AVDDesign.ValidationProfilesItem"))]
pub struct ValidationProfiles<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.ValidationProfilesItemBase", schema = Dict(1776)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(3))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1663))]
    pub parent_profile: ::data_views::Field<&'a str>,
    #[data_view(schema = Dict(1774))]
    pub hardware: ::data_views::Field<item::Hardware<'a, Mode>>,
    #[data_view(schema = Dict(1775))]
    pub logging: ::data_views::Field<item::Logging<'a, Mode>>,
    #[data_view(schema = Bool(675))]
    pub exclude_as_extra_fabric_validation_target: ::data_views::Field<bool>,
    #[data_view(schema = Dict(640))]
    pub interfaces: ::data_views::Field<super::super::eos_config::metadata::Interfaces<'a, Mode>>,
    #[data_view(schema = Dict(641))]
    pub bgp: ::data_views::Field<super::super::eos_config::metadata::Bgp<'a, Mode>>,
}

pub mod item {

    #[::data_views::data_view(python(name = "AVDDesign.ValidationProfilesItemBase.Hardware", schema = Dict(1774)))]
    pub struct Hardware<'a, Mode> {
        #[data_view(schema = Int(638))]
        pub min_power_supplies: ::data_views::Field<i64>,
        #[data_view(schema = Int(639))]
        pub min_fans: ::data_views::Field<i64>,
        #[data_view(schema = Int(640))]
        pub min_supervisors: ::data_views::Field<i64>,
        #[data_view(schema = Int(641))]
        pub min_line_cards: ::data_views::Field<i64>,
        #[data_view(schema = Int(642))]
        pub min_fabric_cards: ::data_views::Field<i64>,
        #[data_view(schema = List(276))]
        pub transceiver_manufacturers: ::data_views::Field<hardware::TransceiverManufacturers<'a, Mode>>,
        #[data_view(schema = Bool(261))]
        pub ignore_no_transceivers: ::data_views::Field<bool>,
    }

    pub mod hardware {

        #[::data_views::data_view(list, python(name = "AVDDesign.ValidationProfilesItemBase.Hardware.TransceiverManufacturers", schema = List(276)))]
        pub struct TransceiverManufacturers<'a, Mode> (::data_views::Field<&'a str>);
    }

    #[::data_views::data_view(python(name = "AVDDesign.ValidationProfilesItemBase.Logging", schema = Dict(1775)))]
    pub struct Logging<'a, Mode> {
        #[data_view(schema = Int(254))]
        pub validate_no_errors_period: ::data_views::Field<i64>,
    }
}
