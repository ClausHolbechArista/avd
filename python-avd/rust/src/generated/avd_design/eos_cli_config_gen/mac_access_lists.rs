// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub counters_per_entry: ::validated_data::Field<bool>,
    pub entries: ::validated_data::Field<item::Entries<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view(list)]
    pub struct Entries<'a, Mode> (::validated_data::Field<entries::Item<'a, Mode>>);

    pub mod entries {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub sequence: ::validated_data::Field<i64>,
            pub action: ::validated_data::RequiredValue<&'a str, Mode>,
        }
    }
}
