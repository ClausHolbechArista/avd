// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.WanCarriersItemBase", schema = Dict(1777)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1666))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1667))]
    pub description: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1668))]
    pub path_group: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Bool(676))]
    pub trusted: ::validated_data::Field<bool>,
}
