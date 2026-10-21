// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct IpLargeCommunityLists<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view]
pub struct Item<'a, Mode> {
    pub name: ::data_views::Field<&'a str>,
    pub entries: ::data_views::RequiredValue<item::Entries<'a, Mode>, Mode>,
}

pub mod item {

    #[::data_views::data_view(list)]
    pub struct Entries<'a, Mode> (::data_views::Field<entries::Item<'a, Mode>>);

    pub mod entries {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub action: ::data_views::RequiredValue<&'a str, Mode>,
            pub large_communities: ::data_views::Field<item::LargeCommunities<'a, Mode>>,
            pub regexp: ::data_views::Field<&'a str>,
        }

        pub mod item {

            #[::data_views::data_view(list)]
            pub struct LargeCommunities<'a, Mode> (::data_views::Field<&'a str>);
        }
    }
}
