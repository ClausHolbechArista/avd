// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub hostname: ::validated_data::Field<&'a str>,
    pub ipv4_addresses: ::validated_data::RequiredValue<item::Ipv4Addresses<'a, Mode>, Mode>,
}

pub mod item {

    #[::validated_data::data_view(list)]
    pub struct Ipv4Addresses<'a, Mode> (::validated_data::Field<&'a str>);
}
