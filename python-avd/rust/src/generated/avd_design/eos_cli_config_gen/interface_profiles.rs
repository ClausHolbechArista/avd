// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub commands: ::validated_data::RequiredValue<item::Commands<'a, Mode>, Mode>,
}

pub mod item {

    #[::validated_data::data_view(list)]
    pub struct Commands<'a, Mode> (::validated_data::Field<&'a str>);
}
