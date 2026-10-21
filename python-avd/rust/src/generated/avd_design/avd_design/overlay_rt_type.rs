// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "AVDDesign.OverlayRtType", schema = Dict(1715)))]
pub struct OverlayRtType<'a, Mode> {
    #[data_view(schema = Str(1606))]
    pub admin_subfield: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1607))]
    pub vrf_admin_subfield: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1608))]
    pub vlan_assigned_number_subfield: ::validated_data::Field<&'a str>,
}
