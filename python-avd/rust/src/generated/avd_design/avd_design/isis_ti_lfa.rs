// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.IsisTiLfa", schema = Dict(1604)))]
pub struct IsisTiLfa<'a, Mode> {
    #[data_view(schema = Bool(340))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = Str(1430))]
    pub protection: ::data_views::Field<&'a str>,
    #[data_view(schema = Int(592))]
    pub local_convergence_delay: ::data_views::Field<i64>,
}
