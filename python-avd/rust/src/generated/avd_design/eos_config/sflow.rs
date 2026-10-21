// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Sflow<'a, Mode> {
    pub sample: ::data_views::Field<i64>,
    pub sample_truncate_size: ::data_views::Field<i64>,
    pub sample_input_subinterface: ::data_views::Field<bool>,
    pub sample_output_subinterface: ::data_views::Field<bool>,
    pub dangerous: ::data_views::Field<bool>,
    pub polling_interval: ::data_views::Field<i64>,
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
    pub destinations: ::data_views::Field<Destinations<'a, Mode>>,
    pub source: ::data_views::Field<&'a str>,
    pub source_interface: ::data_views::Field<&'a str>,
    pub extensions: ::data_views::Field<Extensions<'a, Mode>>,
    pub interface: ::data_views::Field<Interface<'a, Mode>>,
    pub run: ::data_views::Field<bool>,
    pub hardware_acceleration: ::data_views::Field<HardwareAcceleration<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub destinations: ::data_views::Field<item::Destinations<'a, Mode>>,
        pub source: ::data_views::Field<&'a str>,
        pub source_interface: ::data_views::Field<&'a str>,
    }

    pub mod item {

        #[::data_views::data_view(indexed_list, primary_key(destination))]
        pub struct Destinations<'a, Mode> (::data_views::Field<destinations::Item<'a, Mode>>);

        pub mod destinations {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub destination: ::data_views::Field<&'a str>,
                pub port: ::data_views::Field<i64>,
            }
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(destination))]
pub struct Destinations<'a, Mode> (::data_views::Field<destinations::Item<'a, Mode>>);

pub mod destinations {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub destination: ::data_views::Field<&'a str>,
        pub port: ::data_views::Field<i64>,
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Extensions<'a, Mode> (::data_views::Field<extensions::Item<'a, Mode>>);

pub mod extensions {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub enabled: ::data_views::RequiredValue<bool, Mode>,
    }
}

#[::data_views::data_view]
pub struct Interface<'a, Mode> {
    pub disable: ::data_views::Field<interface::Disable<'a, Mode>>,
    pub egress: ::data_views::Field<interface::Egress<'a, Mode>>,
}

pub mod interface {

    #[::data_views::data_view]
    pub struct Disable<'a, Mode> {
        pub default: ::data_views::Field<bool>,
    }

    #[::data_views::data_view]
    pub struct Egress<'a, Mode> {
        pub enable_default: ::data_views::RequiredValue<bool, Mode>,
        pub unmodified: ::data_views::Field<bool>,
    }
}

#[::data_views::data_view]
pub struct HardwareAcceleration<'a, Mode> {
    pub enabled: ::data_views::Field<bool>,
    pub sample: ::data_views::Field<i64>,
    pub modules: ::data_views::Field<hardware_acceleration::Modules<'a, Mode>>,
}

pub mod hardware_acceleration {

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct Modules<'a, Mode> (::data_views::Field<modules::Item<'a, Mode>>);

    pub mod modules {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub enabled: ::data_views::Field<bool>,
        }
    }
}
