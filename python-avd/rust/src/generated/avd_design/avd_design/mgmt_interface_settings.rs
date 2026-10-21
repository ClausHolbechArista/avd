// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.MgmtInterfaceSettings", schema = Dict(1625)))]
pub struct MgmtInterfaceSettings<'a, Mode> {
    #[data_view(schema = Str(1456))]
    pub description: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1457))]
    pub vrf: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(554))]
    pub vrf_routing: ::data_views::Field<bool>,
    #[data_view(schema = Str(1458))]
    pub interface: ::data_views::Field<&'a str>,
    #[data_view(schema = Dict(1624))]
    pub lldp: ::data_views::Field<Lldp<'a, Mode>>,
}

#[::data_views::data_view(python(name = "AVDDesign.MgmtInterfaceSettings.Lldp", schema = Dict(1624)))]
pub struct Lldp<'a, Mode> {
    #[data_view(schema = Bool(555))]
    pub transmit: ::data_views::Field<bool>,
    #[data_view(schema = Bool(556))]
    pub receive: ::data_views::Field<bool>,
    #[data_view(schema = Int(597))]
    pub ztp_vlan: ::data_views::Field<i64>,
}
