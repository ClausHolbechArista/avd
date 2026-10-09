// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub parent_profile: ::validated_data::Field<&'a str>,
    pub hardware: ::validated_data::Field<item::Hardware<'a, Mode>>,
    pub logging: ::validated_data::Field<item::Logging<'a, Mode>>,
    pub exclude_as_extra_fabric_validation_target: ::validated_data::Field<bool>,
    pub interfaces: ::validated_data::Field<super::super::eos_cli_config_gen::metadata::Interfaces<'a, Mode>>,
    pub bgp: ::validated_data::Field<super::super::eos_cli_config_gen::metadata::Bgp<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view]
    pub struct Hardware<'a, Mode> {
        pub min_power_supplies: ::validated_data::Field<i64>,
        pub min_fans: ::validated_data::Field<i64>,
        pub min_supervisors: ::validated_data::Field<i64>,
        pub min_line_cards: ::validated_data::Field<i64>,
        pub min_fabric_cards: ::validated_data::Field<i64>,
        pub transceiver_manufacturers: ::validated_data::Field<hardware::TransceiverManufacturers<'a, Mode>>,
        pub ignore_no_transceivers: ::validated_data::Field<bool>,
    }

    pub mod hardware {

        #[::validated_data::data_view(list)]
        pub struct TransceiverManufacturers<'a, Mode> (::validated_data::Field<&'a str>);
    }

    #[::validated_data::data_view]
    pub struct Logging<'a, Mode> {
        pub validate_no_errors_period: ::validated_data::Field<i64>,
    }
}
