// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct AccessList<'a, Mode> {
    pub mechanism: ::validated_data::Field<&'a str>,
    pub update_default_result_permit: ::validated_data::Field<bool>,
}

#[::validated_data::data_view(indexed_list, primary_key(speed_group))]
pub struct SpeedGroups<'a, Mode> (::validated_data::Field<speed_groups::Item<'a, Mode>>);

pub mod speed_groups {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub speed_group: ::validated_data::Field<&'a str>,
        pub serdes: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(indexed_list, primary_key(port_group))]
pub struct PortGroups<'a, Mode> (::validated_data::Field<port_groups::Item<'a, Mode>>);

pub mod port_groups {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub port_group: ::validated_data::Field<&'a str>,
        pub select: ::validated_data::Field<&'a str>,
    }
}
