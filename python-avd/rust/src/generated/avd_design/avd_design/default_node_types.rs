// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub node_type: ::validated_data::Field<&'a str>,
    pub match_hostnames: ::validated_data::RequiredValue<item::MatchHostnames<'a, Mode>, Mode>,
}

pub mod item {

    #[::validated_data::data_view(list)]
    pub struct MatchHostnames<'a, Mode> (::validated_data::RequiredValue<&'a str, Mode>);
}
