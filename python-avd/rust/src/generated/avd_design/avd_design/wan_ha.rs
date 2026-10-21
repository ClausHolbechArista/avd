// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "AVDDesign.WanHa", schema = Dict(1778)))]
pub struct WanHa<'a, Mode> {
    #[data_view(schema = Str(1670))]
    pub lan_ha_path_group_name: ::validated_data::Field<&'a str>,
}
