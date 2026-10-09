// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub sequence_numbers: ::validated_data::RequiredValue<item::SequenceNumbers<'a, Mode>, Mode>,
}

pub mod item {

    #[::validated_data::data_view(indexed_list, primary_key(sequence))]
    pub struct SequenceNumbers<'a, Mode> (::validated_data::Field<sequence_numbers::Item<'a, Mode>>);

    pub mod sequence_numbers {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub sequence: ::validated_data::Field<i64>,
            #[data_view(rename = "type")]
            pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
            pub description: ::validated_data::Field<&'a str>,
            #[data_view(rename = "match")]
            pub field_match: ::validated_data::Field<item::FieldMatch<'a, Mode>>,
            pub set: ::validated_data::Field<item::Set<'a, Mode>>,
            pub sub_route_map: ::validated_data::Field<&'a str>,
            #[data_view(rename = "continue")]
            pub field_continue: ::validated_data::Field<item::FieldContinue<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(list)]
            pub struct FieldMatch<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list)]
            pub struct Set<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view]
            pub struct FieldContinue<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub sequence_number: ::validated_data::Field<i64>,
            }
        }
    }
}
