// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "EOSConfig.ServiceUnsupportedTransceiver", schema = Dict(1217)))]
pub struct ServiceUnsupportedTransceiver<'a, Mode> {
    #[data_view(schema = Str(3))]
    pub license_name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(3))]
    pub license_key: ::validated_data::Field<&'a str>,
}
