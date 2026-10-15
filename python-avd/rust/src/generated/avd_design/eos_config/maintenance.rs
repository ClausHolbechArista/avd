// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct InterfaceProfiles<'a, Mode> (::validated_data::Field<interface_profiles::Item<'a, Mode>>);

pub mod interface_profiles {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub rate_monitoring: ::validated_data::Field<item::RateMonitoring<'a, Mode>>,
        pub shutdown: ::validated_data::Field<item::Shutdown<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct RateMonitoring<'a, Mode> {
            pub load_interval: ::validated_data::Field<i64>,
            pub threshold: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view]
        pub struct Shutdown<'a, Mode> {
            pub max_delay: ::validated_data::Field<i64>,
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct BgpProfiles<'a, Mode> (::validated_data::Field<bgp_profiles::Item<'a, Mode>>);

pub mod bgp_profiles {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub initiator: ::validated_data::Field<item::Initiator<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct Initiator<'a, Mode> {
            pub route_map_inout: ::validated_data::Field<&'a str>,
            pub route_map_in: ::validated_data::Field<&'a str>,
            pub route_map_out: ::validated_data::Field<&'a str>,
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct UnitProfiles<'a, Mode> (::validated_data::Field<unit_profiles::Item<'a, Mode>>);

pub mod unit_profiles {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub on_boot: ::validated_data::Field<item::OnBoot<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct OnBoot<'a, Mode> {
            pub duration: ::validated_data::Field<i64>,
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Units<'a, Mode> (::validated_data::Field<units::Item<'a, Mode>>);

pub mod units {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub quiesce: ::validated_data::Field<bool>,
        pub profile: ::validated_data::Field<&'a str>,
        pub groups: ::validated_data::Field<item::Groups<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct Groups<'a, Mode> {
            pub bgp_groups: ::validated_data::Field<groups::BgpGroups<'a, Mode>>,
            pub interface_groups: ::validated_data::Field<groups::InterfaceGroups<'a, Mode>>,
        }

        pub mod groups {

            #[::validated_data::data_view(list)]
            pub struct BgpGroups<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list)]
            pub struct InterfaceGroups<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }
}
