// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.ValidationProfiles", schema = List(787), item_name = "AVDDesign.ValidationProfilesItem"))]
pub struct ValidationProfiles<'a, Mode> (::validated_data::Field<Item<'a, Mode>>);

#[::validated_data::data_view(python(name = "AVDDesign.ValidationProfilesItemBase", schema = Dict(1776)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(3))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1663))]
    pub parent_profile: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1774))]
    pub hardware: ::validated_data::Field<item::Hardware<'a, Mode>>,
    #[data_view(schema = Dict(1775))]
    pub logging: ::validated_data::Field<item::Logging<'a, Mode>>,
    #[data_view(schema = Bool(675))]
    pub exclude_as_extra_fabric_validation_target: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(640))]
    pub interfaces: ::validated_data::Field<super::super::eos_config::metadata::Interfaces<'a, Mode>>,
    #[data_view(schema = Dict(641))]
    pub bgp: ::validated_data::Field<super::super::eos_config::metadata::Bgp<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view(python(name = "AVDDesign.ValidationProfilesItemBase.Hardware", schema = Dict(1774)))]
    pub struct Hardware<'a, Mode> {
        #[data_view(schema = Int(638))]
        pub min_power_supplies: ::validated_data::Field<i64>,
        #[data_view(schema = Int(639))]
        pub min_fans: ::validated_data::Field<i64>,
        #[data_view(schema = Int(640))]
        pub min_supervisors: ::validated_data::Field<i64>,
        #[data_view(schema = Int(641))]
        pub min_line_cards: ::validated_data::Field<i64>,
        #[data_view(schema = Int(642))]
        pub min_fabric_cards: ::validated_data::Field<i64>,
        #[data_view(schema = List(276))]
        pub transceiver_manufacturers: ::validated_data::Field<hardware::TransceiverManufacturers<'a, Mode>>,
        #[data_view(schema = Bool(261))]
        pub ignore_no_transceivers: ::validated_data::Field<bool>,
    }

    pub mod hardware {

        #[::validated_data::data_view(list, python(name = "AVDDesign.ValidationProfilesItemBase.Hardware.TransceiverManufacturers", schema = List(276)))]
        pub struct TransceiverManufacturers<'a, Mode> (::validated_data::Field<&'a str>);
    }

    #[::validated_data::data_view(python(name = "AVDDesign.ValidationProfilesItemBase.Logging", schema = Dict(1775)))]
    pub struct Logging<'a, Mode> {
        #[data_view(schema = Int(254))]
        pub validate_no_errors_period: ::validated_data::Field<i64>,
    }
}
