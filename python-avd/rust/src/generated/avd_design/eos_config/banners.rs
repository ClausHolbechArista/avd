// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "EOSConfig.Banners", schema = Dict(85)))]
pub struct Banners<'a, Mode> {
    #[data_view(schema = Str(69))]
    pub login: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(70))]
    pub motd: ::validated_data::Field<&'a str>,
}
