// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct InterfaceTags<'a, Mode> (::validated_data::Field<interface_tags::Item<'a, Mode>>);

pub mod interface_tags {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::RequiredValue<&'a str, Mode>,
        pub data_path: ::validated_data::Field<&'a str>,
        pub value: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(list)]
pub struct DeviceTags<'a, Mode> (::validated_data::Field<device_tags::Item<'a, Mode>>);

pub mod device_tags {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::RequiredValue<&'a str, Mode>,
        pub data_path: ::validated_data::Field<&'a str>,
        pub value: ::validated_data::Field<&'a str>,
    }
}
