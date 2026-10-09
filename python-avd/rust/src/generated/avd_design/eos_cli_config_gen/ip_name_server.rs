// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub servers: ::validated_data::RequiredValue<item::Servers<'a, Mode>, Mode>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
        pub struct Servers<'a, Mode> (::validated_data::Field<servers::Item<'a, Mode>>);

        pub mod servers {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub ip_address: ::validated_data::RequiredValue<&'a str, Mode>,
                pub priority: ::validated_data::Field<i64>,
            }
        }
    }
}
