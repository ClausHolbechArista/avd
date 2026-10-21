// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct BgpGroups<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view]
pub struct Item<'a, Mode> {
    pub name: ::data_views::Field<&'a str>,
    pub vrf: ::data_views::Field<&'a str>,
    pub neighbors: ::data_views::Field<item::Neighbors<'a, Mode>>,
    pub bgp_maintenance_profiles: ::data_views::Field<item::BgpMaintenanceProfiles<'a, Mode>>,
}

pub mod item {

    #[::data_views::data_view(list)]
    pub struct Neighbors<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(list)]
    pub struct BgpMaintenanceProfiles<'a, Mode> (::data_views::Field<&'a str>);
}
