// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.ManagementSettings", schema = Dict(1623)))]
pub struct ManagementSettings<'a, Mode> {
    #[data_view(schema = Dict(575))]
    pub console: ::data_views::Field<super::super::eos_config::ManagementConsole<'a, Mode>>,
    #[data_view(schema = Dict(85))]
    pub banners: ::data_views::Field<super::super::eos_config::Banners<'a, Mode>>,
}
