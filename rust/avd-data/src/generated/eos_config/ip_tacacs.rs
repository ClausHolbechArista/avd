// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct IpTacacs<'a, Mode> {
    pub source_interface: ::data_views::Field<&'a str>,
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub source_interface: ::data_views::RequiredValue<&'a str, Mode>,
    }
}
