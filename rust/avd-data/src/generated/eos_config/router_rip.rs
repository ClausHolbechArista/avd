// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct RouterRip<'a, Mode> {
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(vrf))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub vrf: ::data_views::Field<&'a str>,
        pub metric_default: ::data_views::Field<i64>,
        pub networks: ::data_views::Field<item::Networks<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(list)]
        pub struct Networks<'a, Mode> (::data_views::Field<&'a str>);
    }
}
