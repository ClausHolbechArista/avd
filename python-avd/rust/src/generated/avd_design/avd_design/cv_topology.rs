// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub hostname: ::validated_data::Field<&'a str>,
    pub platform: ::validated_data::RequiredValue<&'a str, Mode>,
    pub interfaces: ::validated_data::RequiredValue<item::Interfaces<'a, Mode>, Mode>,
}

pub mod item {

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct Interfaces<'a, Mode> (::validated_data::Field<interfaces::Item<'a, Mode>>);

    pub mod interfaces {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub neighbor: ::validated_data::Field<&'a str>,
            pub neighbor_interface: ::validated_data::Field<&'a str>,
        }
    }
}
