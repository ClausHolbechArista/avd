// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct AccessLists<'a, Mode> (::validated_data::Field<access_lists::Item<'a, Mode>>);

pub mod access_lists {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub entries: ::validated_data::Field<item::Entries<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(list)]
        pub struct Entries<'a, Mode> (::validated_data::Field<entries::Item<'a, Mode>>);

        pub mod entries {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                #[data_view(rename = "type")]
                pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
                #[data_view(rename = "match")]
                pub field_match: ::validated_data::RequiredValue<&'a str, Mode>,
                pub origin: ::validated_data::Field<&'a str>,
            }
        }
    }
}
