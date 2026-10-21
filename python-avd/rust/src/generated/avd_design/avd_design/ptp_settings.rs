// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "AVDDesign.PtpSettings", schema = Dict(1735)))]
pub struct PtpSettings<'a, Mode> {
    #[data_view(schema = Bool(7))]
    pub enabled: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1154))]
    pub profile: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(629))]
    pub domain: ::validated_data::Field<i64>,
    #[data_view(schema = Bool(37))]
    pub auto_clock_identity: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(639))]
    pub forward_v1: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(818))]
    pub free_running: ::validated_data::Field<super::super::eos_config::ptp::FreeRunning<'a, Mode>>,
}
