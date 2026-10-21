// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "AVDDesign.IsisTiLfa", schema = Dict(1604)))]
pub struct IsisTiLfa<'a, Mode> {
    #[data_view(schema = Bool(340))]
    pub enabled: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1430))]
    pub protection: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(592))]
    pub local_convergence_delay: ::validated_data::Field<i64>,
}
