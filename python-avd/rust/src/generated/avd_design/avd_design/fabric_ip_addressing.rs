// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.FabricIpAddressing.Loopback", schema = Dict(1565)))]
pub struct Loopback<'a, Mode> {
    #[data_view(schema = Int(577))]
    pub ipv6_prefix_length: ::validated_data::Field<i64>,
}

#[::validated_data::data_view(python(name = "AVDDesign.FabricIpAddressing.Mlag", schema = Dict(1566)))]
pub struct Mlag<'a, Mode> {
    #[data_view(schema = Str(1402))]
    pub algorithm: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(578))]
    pub ipv4_prefix_length: ::validated_data::Field<i64>,
    #[data_view(schema = Int(579))]
    pub ipv6_prefix_length: ::validated_data::Field<i64>,
}

#[::validated_data::data_view(python(name = "AVDDesign.FabricIpAddressing.P2pUplinks", schema = Dict(1567)))]
pub struct P2pUplinks<'a, Mode> {
    #[data_view(schema = Int(580))]
    pub ipv4_prefix_length: ::validated_data::Field<i64>,
    #[data_view(schema = Int(581))]
    pub ipv6_prefix_length: ::validated_data::Field<i64>,
}

#[::validated_data::data_view(python(name = "AVDDesign.FabricIpAddressing.WanHa", schema = Dict(1568)))]
pub struct WanHa<'a, Mode> {
    #[data_view(schema = Int(582))]
    pub ipv4_prefix_length: ::validated_data::Field<i64>,
}
