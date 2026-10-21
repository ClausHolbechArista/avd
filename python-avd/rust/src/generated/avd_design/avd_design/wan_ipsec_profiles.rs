// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.WanIpsecProfiles", schema = Dict(1781)))]
pub struct WanIpsecProfiles<'a, Mode> {
    #[data_view(schema = Dict(1779))]
    pub control_plane: ::data_views::RequiredValue<ControlPlane<'a, Mode>, Mode>,
    #[data_view(schema = Dict(1780))]
    pub data_plane: ::data_views::Field<DataPlane<'a, Mode>>,
}

#[::data_views::data_view(python(name = "AVDDesign.WanIpsecProfiles.ControlPlane", schema = Dict(1779)))]
pub struct ControlPlane<'a, Mode> {
    #[data_view(schema = Str(1671))]
    pub ike_policy_name: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1672))]
    pub sa_policy_name: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1673))]
    pub profile_name: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1674))]
    pub shared_key: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1675))]
    pub cleartext_shared_key: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(python(name = "AVDDesign.WanIpsecProfiles.DataPlane", schema = Dict(1780)))]
pub struct DataPlane<'a, Mode> {
    #[data_view(schema = Str(1676))]
    pub ike_policy_name: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1677))]
    pub sa_policy_name: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1678))]
    pub profile_name: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1674))]
    pub shared_key: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1675))]
    pub cleartext_shared_key: ::data_views::Field<&'a str>,
}
