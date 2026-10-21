// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct MacAccessLists<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view]
pub struct Item<'a, Mode> {
    pub name: ::data_views::Field<&'a str>,
    pub counters_per_entry: ::data_views::Field<bool>,
    pub entries: ::data_views::Field<item::Entries<'a, Mode>>,
}

pub mod item {

    #[::data_views::data_view(list)]
    pub struct Entries<'a, Mode> (::data_views::Field<entries::Item<'a, Mode>>);

    pub mod entries {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub sequence: ::data_views::Field<i64>,
            pub action: ::data_views::RequiredValue<&'a str, Mode>,
        }
    }
}
