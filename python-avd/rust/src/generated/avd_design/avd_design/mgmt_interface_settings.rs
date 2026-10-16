// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.MgmtInterfaceSettings.Lldp", schema = Dict(1624)))]
pub struct Lldp<'a, Mode> {
    #[data_view(schema = Bool(555))]
    pub transmit: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(556))]
    pub receive: ::validated_data::Field<bool>,
    #[data_view(schema = Int(597))]
    pub ztp_vlan: ::validated_data::Field<i64>,
}
