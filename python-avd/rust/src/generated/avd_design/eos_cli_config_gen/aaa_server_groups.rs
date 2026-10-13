// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(rename = "type")]
    pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
    pub servers: ::validated_data::Field<item::Servers<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view(list)]
    pub struct Servers<'a, Mode> (::validated_data::Field<servers::Item<'a, Mode>>);

    pub mod servers {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub server: ::validated_data::RequiredValue<&'a str, Mode>,
            pub vrf: ::validated_data::Field<&'a str>,
            pub tls: ::validated_data::Field<item::Tls<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct Tls<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub port: ::validated_data::Field<i64>,
            }
        }
    }
}
