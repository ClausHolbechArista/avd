// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub counters_per_entry: ::validated_data::Field<bool>,
    pub permit_response_traffic: ::validated_data::Field<&'a str>,
    pub sequence_numbers: ::validated_data::RequiredValue<item::SequenceNumbers<'a, Mode>, Mode>,
}

pub mod item {

    #[::validated_data::data_view(indexed_list, primary_key(sequence))]
    pub struct SequenceNumbers<'a, Mode> (::validated_data::Field<sequence_numbers::Item<'a, Mode>>);

    pub mod sequence_numbers {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub sequence: ::validated_data::Field<i64>,
            pub action: ::validated_data::RequiredValue<&'a str, Mode>,
        }
    }
}
