// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(list, python(name = "AVDDesign.EosDesignsCustomTemplates", schema = List(650)))]
pub struct EosDesignsCustomTemplates<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.EosDesignsCustomTemplatesItem", schema = Dict(1544)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1391))]
    pub template: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Dict(1543))]
    pub options: ::data_views::Field<item::Options<'a, Mode>>,
}

pub mod item {

    #[::data_views::data_view(python(name = "AVDDesign.EosDesignsCustomTemplatesItem.Options", schema = Dict(1543)))]
    pub struct Options<'a, Mode> {
        #[data_view(schema = Str(1392))]
        pub list_merge: ::data_views::Field<&'a str>,
        #[data_view(schema = Bool(510))]
        pub strip_empty_keys: ::data_views::Field<bool>,
    }
}
