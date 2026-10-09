// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(list)]
pub struct Features<'a, Mode> (::validated_data::Field<features::Item<'a, Mode>>);

pub mod features {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::RequiredValue<&'a str, Mode>,
        pub direction: ::validated_data::Field<&'a str>,
        pub enabled: ::validated_data::Field<bool>,
        pub address_type: ::validated_data::Field<&'a str>,
        pub layer3: ::validated_data::Field<bool>,
        pub vrf: ::validated_data::Field<&'a str>,
        pub prefix: ::validated_data::Field<&'a str>,
        pub units_packets: ::validated_data::Field<bool>,
    }
}
