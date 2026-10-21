// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct DynamicPrefixLists<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view]
pub struct Item<'a, Mode> {
    pub name: ::data_views::Field<&'a str>,
    pub match_map: ::data_views::Field<&'a str>,
    pub prefix_list: ::data_views::Field<item::PrefixList<'a, Mode>>,
}

pub mod item {

    #[::data_views::data_view]
    pub struct PrefixList<'a, Mode> {
        pub ipv4: ::data_views::Field<&'a str>,
        pub ipv6: ::data_views::Field<&'a str>,
    }
}
