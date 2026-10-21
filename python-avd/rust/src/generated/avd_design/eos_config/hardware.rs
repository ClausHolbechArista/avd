// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Hardware<'a, Mode> {
    pub access_list: ::data_views::Field<AccessList<'a, Mode>>,
    pub speed_groups: ::data_views::Field<SpeedGroups<'a, Mode>>,
    pub port_groups: ::data_views::Field<PortGroups<'a, Mode>>,
}

#[::data_views::data_view]
pub struct AccessList<'a, Mode> {
    pub mechanism: ::data_views::Field<&'a str>,
    pub update_default_result_permit: ::data_views::Field<bool>,
}

#[::data_views::data_view(indexed_list, primary_key(speed_group))]
pub struct SpeedGroups<'a, Mode> (::data_views::Field<speed_groups::Item<'a, Mode>>);

pub mod speed_groups {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub speed_group: ::data_views::Field<&'a str>,
        pub serdes: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view(indexed_list, primary_key(port_group))]
pub struct PortGroups<'a, Mode> (::data_views::Field<port_groups::Item<'a, Mode>>);

pub mod port_groups {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub port_group: ::data_views::Field<&'a str>,
        pub select: ::data_views::Field<&'a str>,
    }
}
