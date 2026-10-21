// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct RouteMaps<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view]
pub struct Item<'a, Mode> {
    pub name: ::data_views::Field<&'a str>,
    pub sequence_numbers: ::data_views::RequiredValue<item::SequenceNumbers<'a, Mode>, Mode>,
}

pub mod item {

    #[::data_views::data_view(indexed_list, primary_key(sequence))]
    pub struct SequenceNumbers<'a, Mode> (::data_views::Field<sequence_numbers::Item<'a, Mode>>);

    pub mod sequence_numbers {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub sequence: ::data_views::Field<i64>,
            #[data_view(rename = "type")]
            pub field_type: ::data_views::RequiredValue<&'a str, Mode>,
            pub description: ::data_views::Field<&'a str>,
            #[data_view(rename = "match")]
            pub field_match: ::data_views::Field<item::FieldMatch<'a, Mode>>,
            pub set: ::data_views::Field<item::Set<'a, Mode>>,
            pub sub_route_map: ::data_views::Field<&'a str>,
            #[data_view(rename = "continue")]
            pub field_continue: ::data_views::Field<item::FieldContinue<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view(list)]
            pub struct FieldMatch<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list)]
            pub struct Set<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view]
            pub struct FieldContinue<'a, Mode> {
                pub enabled: ::data_views::Field<bool>,
                pub sequence_number: ::data_views::Field<i64>,
            }
        }
    }
}
