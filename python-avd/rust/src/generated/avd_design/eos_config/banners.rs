// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "EOSConfig.Banners", schema = Dict(85)))]
pub struct Banners<'a, Mode> {
    #[data_view(schema = Str(69))]
    pub login: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(70))]
    pub motd: ::data_views::Field<&'a str>,
}
