// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub match_map: ::validated_data::Field<&'a str>,
    pub prefix_list: ::validated_data::Field<item::PrefixList<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view]
    pub struct PrefixList<'a, Mode> {
        pub ipv4: ::validated_data::Field<&'a str>,
        pub ipv6: ::validated_data::Field<&'a str>,
    }
}
