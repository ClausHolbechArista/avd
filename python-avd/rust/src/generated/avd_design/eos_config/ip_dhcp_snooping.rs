// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct InformationOption<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub circuit_id_type: ::validated_data::Field<&'a str>,
    pub circuit_id_format: ::validated_data::Field<&'a str>,
}
