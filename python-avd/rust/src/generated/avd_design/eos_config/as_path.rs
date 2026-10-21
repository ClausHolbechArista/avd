// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct AsPath<'a, Mode> {
    pub regex_mode: ::data_views::Field<&'a str>,
    pub access_lists: ::data_views::Field<AccessLists<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct AccessLists<'a, Mode> (::data_views::Field<access_lists::Item<'a, Mode>>);

pub mod access_lists {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub entries: ::data_views::Field<item::Entries<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(list)]
        pub struct Entries<'a, Mode> (::data_views::Field<entries::Item<'a, Mode>>);

        pub mod entries {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                #[data_view(rename = "type")]
                pub field_type: ::data_views::RequiredValue<&'a str, Mode>,
                #[data_view(rename = "match")]
                pub field_match: ::data_views::RequiredValue<&'a str, Mode>,
                pub origin: ::data_views::Field<&'a str>,
            }
        }
    }
}
