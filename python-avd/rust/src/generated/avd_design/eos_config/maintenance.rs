// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Maintenance<'a, Mode> {
    pub default_interface_profile: ::data_views::Field<&'a str>,
    pub default_bgp_profile: ::data_views::Field<&'a str>,
    pub default_unit_profile: ::data_views::Field<&'a str>,
    pub interface_profiles: ::data_views::Field<InterfaceProfiles<'a, Mode>>,
    pub bgp_profiles: ::data_views::Field<BgpProfiles<'a, Mode>>,
    pub unit_profiles: ::data_views::Field<UnitProfiles<'a, Mode>>,
    pub units: ::data_views::Field<Units<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct InterfaceProfiles<'a, Mode> (::data_views::Field<interface_profiles::Item<'a, Mode>>);

pub mod interface_profiles {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub rate_monitoring: ::data_views::Field<item::RateMonitoring<'a, Mode>>,
        pub shutdown: ::data_views::Field<item::Shutdown<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct RateMonitoring<'a, Mode> {
            pub load_interval: ::data_views::Field<i64>,
            pub threshold: ::data_views::Field<i64>,
        }

        #[::data_views::data_view]
        pub struct Shutdown<'a, Mode> {
            pub max_delay: ::data_views::Field<i64>,
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct BgpProfiles<'a, Mode> (::data_views::Field<bgp_profiles::Item<'a, Mode>>);

pub mod bgp_profiles {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub initiator: ::data_views::Field<item::Initiator<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct Initiator<'a, Mode> {
            pub route_map_inout: ::data_views::Field<&'a str>,
            pub route_map_in: ::data_views::Field<&'a str>,
            pub route_map_out: ::data_views::Field<&'a str>,
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct UnitProfiles<'a, Mode> (::data_views::Field<unit_profiles::Item<'a, Mode>>);

pub mod unit_profiles {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub on_boot: ::data_views::Field<item::OnBoot<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct OnBoot<'a, Mode> {
            pub duration: ::data_views::Field<i64>,
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Units<'a, Mode> (::data_views::Field<units::Item<'a, Mode>>);

pub mod units {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub quiesce: ::data_views::Field<bool>,
        pub profile: ::data_views::Field<&'a str>,
        pub groups: ::data_views::Field<item::Groups<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct Groups<'a, Mode> {
            pub bgp_groups: ::data_views::Field<groups::BgpGroups<'a, Mode>>,
            pub interface_groups: ::data_views::Field<groups::InterfaceGroups<'a, Mode>>,
        }

        pub mod groups {

            #[::data_views::data_view(list)]
            pub struct BgpGroups<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list)]
            pub struct InterfaceGroups<'a, Mode> (::data_views::Field<&'a str>);
        }
    }
}
