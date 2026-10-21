// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct Poe<'a, Mode> {
    pub reboot: ::validated_data::Field<Reboot<'a, Mode>>,
    pub interface_shutdown: ::validated_data::Field<InterfaceShutdown<'a, Mode>>,
}

#[::validated_data::data_view]
pub struct Reboot<'a, Mode> {
    pub action: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct InterfaceShutdown<'a, Mode> {
    pub action: ::validated_data::Field<&'a str>,
}
