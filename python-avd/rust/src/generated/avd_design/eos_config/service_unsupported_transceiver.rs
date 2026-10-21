// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "EOSConfig.ServiceUnsupportedTransceiver", schema = Dict(1217)))]
pub struct ServiceUnsupportedTransceiver<'a, Mode> {
    #[data_view(schema = Str(3))]
    pub license_name: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(3))]
    pub license_key: ::data_views::Field<&'a str>,
}
