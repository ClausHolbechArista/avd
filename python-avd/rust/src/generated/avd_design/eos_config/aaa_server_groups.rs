// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct AaaServerGroups<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view]
pub struct Item<'a, Mode> {
    pub name: ::data_views::Field<&'a str>,
    #[data_view(rename = "type")]
    pub field_type: ::data_views::RequiredValue<&'a str, Mode>,
    pub servers: ::data_views::Field<item::Servers<'a, Mode>>,
}

pub mod item {

    #[::data_views::data_view(list)]
    pub struct Servers<'a, Mode> (::data_views::Field<servers::Item<'a, Mode>>);

    pub mod servers {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub server: ::data_views::RequiredValue<&'a str, Mode>,
            pub vrf: ::data_views::Field<&'a str>,
            pub tls: ::data_views::Field<item::Tls<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct Tls<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub port: ::data_views::Field<i64>,
            }
        }
    }
}
