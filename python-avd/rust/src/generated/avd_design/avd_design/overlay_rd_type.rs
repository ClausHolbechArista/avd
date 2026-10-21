// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.OverlayRdType", schema = Dict(1714)))]
pub struct OverlayRdType<'a, Mode> {
    #[data_view(schema = Str(1601))]
    pub admin_subfield: ::data_views::Field<&'a str>,
    #[data_view(schema = Int(619))]
    pub admin_subfield_offset: ::data_views::Field<i64>,
    #[data_view(schema = Str(1602))]
    pub vrf_admin_subfield: ::data_views::Field<&'a str>,
    #[data_view(schema = Int(620))]
    pub vrf_admin_subfield_offset: ::data_views::Field<i64>,
    #[data_view(schema = Str(1603))]
    pub vlan_assigned_number_subfield: ::data_views::Field<&'a str>,
}
