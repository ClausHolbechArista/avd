// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct IpNameServer<'a, Mode> {
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub servers: ::data_views::RequiredValue<item::Servers<'a, Mode>, Mode>,
    }

    pub mod item {

        #[::data_views::data_view(indexed_list, primary_key(ip_address))]
        pub struct Servers<'a, Mode> (::data_views::Field<servers::Item<'a, Mode>>);

        pub mod servers {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub ip_address: ::data_views::RequiredValue<&'a str, Mode>,
                pub priority: ::data_views::Field<i64>,
            }
        }
    }
}
