// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct InterfaceGroups<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view]
pub struct Item<'a, Mode> {
    pub name: ::data_views::Field<&'a str>,
    pub interfaces: ::data_views::Field<item::Interfaces<'a, Mode>>,
    pub bgp_maintenance_profiles: ::data_views::Field<item::BgpMaintenanceProfiles<'a, Mode>>,
    pub interface_maintenance_profiles: ::data_views::Field<item::InterfaceMaintenanceProfiles<'a, Mode>>,
}

pub mod item {

    #[::data_views::data_view(list)]
    pub struct Interfaces<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(list)]
    pub struct BgpMaintenanceProfiles<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(list)]
    pub struct InterfaceMaintenanceProfiles<'a, Mode> (::data_views::Field<&'a str>);
}
