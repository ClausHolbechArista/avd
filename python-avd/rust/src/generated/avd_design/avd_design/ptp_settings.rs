// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.PtpSettings", schema = Dict(1735)))]
pub struct PtpSettings<'a, Mode> {
    #[data_view(schema = Bool(7))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = Str(1154))]
    pub profile: ::data_views::Field<&'a str>,
    #[data_view(schema = Int(629))]
    pub domain: ::data_views::Field<i64>,
    #[data_view(schema = Bool(37))]
    pub auto_clock_identity: ::data_views::Field<bool>,
    #[data_view(schema = Bool(639))]
    pub forward_v1: ::data_views::Field<bool>,
    #[data_view(schema = Dict(818))]
    pub free_running: ::data_views::Field<super::super::eos_config::ptp::FreeRunning<'a, Mode>>,
}
