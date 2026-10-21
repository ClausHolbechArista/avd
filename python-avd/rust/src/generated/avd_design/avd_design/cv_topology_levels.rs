// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(indexed_list, primary_key(field_type), python(name = "AVDDesign.CvTopologyLevels", schema = List(602), item_name = "AVDDesign.CvTopologyLevelsItem"))]
pub struct CvTopologyLevels<'a, Mode> (::validated_data::Field<Item<'a, Mode>>);

#[::validated_data::data_view(python(name = "AVDDesign.CvTopologyLevelsItemBase", schema = Dict(1461)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1217), rename = "type")]
    pub field_type: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(528))]
    pub level: ::validated_data::RequiredValue<i64, Mode>,
}
