// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub entries: ::validated_data::RequiredValue<item::Entries<'a, Mode>, Mode>,
}

pub mod item {

    #[::validated_data::data_view(list)]
    pub struct Entries<'a, Mode> (::validated_data::Field<entries::Item<'a, Mode>>);

    pub mod entries {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub action: ::validated_data::RequiredValue<&'a str, Mode>,
            pub large_communities: ::validated_data::Field<item::LargeCommunities<'a, Mode>>,
            pub regexp: ::validated_data::Field<&'a str>,
        }

        pub mod item {

            #[::validated_data::data_view(list)]
            pub struct LargeCommunities<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }
}
