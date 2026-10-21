// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "AVDDesign.MgmtInterfaceSettings", schema = Dict(1625)))]
pub struct MgmtInterfaceSettings<'a, Mode> {
    #[data_view(schema = Str(1456))]
    pub description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1457))]
    pub vrf: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(554))]
    pub vrf_routing: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1458))]
    pub interface: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1624))]
    pub lldp: ::validated_data::Field<Lldp<'a, Mode>>,
}

#[::validated_data::data_view(python(name = "AVDDesign.MgmtInterfaceSettings.Lldp", schema = Dict(1624)))]
pub struct Lldp<'a, Mode> {
    #[data_view(schema = Bool(555))]
    pub transmit: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(556))]
    pub receive: ::validated_data::Field<bool>,
    #[data_view(schema = Int(597))]
    pub ztp_vlan: ::validated_data::Field<i64>,
}
