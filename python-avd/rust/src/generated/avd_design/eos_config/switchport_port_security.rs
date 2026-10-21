// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct SwitchportPortSecurity<'a, Mode> {
    pub mac_address: ::validated_data::Field<MacAddress<'a, Mode>>,
    pub persistence_disabled: ::validated_data::Field<bool>,
    pub violation_protect_chip_based: ::validated_data::Field<bool>,
}

#[::validated_data::data_view]
pub struct MacAddress<'a, Mode> {
    pub aging: ::validated_data::Field<bool>,
    pub moveable: ::validated_data::Field<bool>,
}
