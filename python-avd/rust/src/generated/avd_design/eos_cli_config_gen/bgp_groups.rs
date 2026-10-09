// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub vrf: ::validated_data::Field<&'a str>,
    pub neighbors: ::validated_data::Field<item::Neighbors<'a, Mode>>,
    pub bgp_maintenance_profiles: ::validated_data::Field<item::BgpMaintenanceProfiles<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view(list)]
    pub struct Neighbors<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list)]
    pub struct BgpMaintenanceProfiles<'a, Mode> (::validated_data::Field<&'a str>);
}
