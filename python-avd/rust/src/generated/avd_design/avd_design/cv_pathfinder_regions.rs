// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::RequiredValue<&'a str, Mode>,
    pub description: ::validated_data::Field<&'a str>,
    pub id: ::validated_data::RequiredValue<i64, Mode>,
    pub sites: ::validated_data::Field<item::Sites<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct Sites<'a, Mode> (::validated_data::Field<sites::Item<'a, Mode>>);

    pub mod sites {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::RequiredValue<&'a str, Mode>,
            pub description: ::validated_data::Field<&'a str>,
            pub id: ::validated_data::RequiredValue<i64, Mode>,
            pub location: ::validated_data::Field<&'a str>,
            pub site_contact: ::validated_data::Field<&'a str>,
            pub site_after_hours_contact: ::validated_data::Field<&'a str>,
        }
    }
}
