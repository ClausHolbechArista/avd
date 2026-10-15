// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Tlvs<'a, Mode> (::validated_data::Field<tlvs::Item<'a, Mode>>);

pub mod tlvs {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub transmit: ::validated_data::Field<bool>,
    }
}
