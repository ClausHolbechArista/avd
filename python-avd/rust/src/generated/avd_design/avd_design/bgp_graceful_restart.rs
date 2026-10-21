// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.BgpGracefulRestart", schema = Dict(1392)))]
pub struct BgpGracefulRestart<'a, Mode> {
    #[data_view(schema = Bool(425))]
    pub enabled: ::data_views::RequiredValue<bool, Mode>,
    #[data_view(schema = Int(499))]
    pub restart_time: ::data_views::Field<i64>,
}
