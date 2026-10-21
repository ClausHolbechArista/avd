// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct Sflow<'a, Mode> {
    pub sample: ::validated_data::Field<i64>,
    pub sample_truncate_size: ::validated_data::Field<i64>,
    pub sample_input_subinterface: ::validated_data::Field<bool>,
    pub sample_output_subinterface: ::validated_data::Field<bool>,
    pub dangerous: ::validated_data::Field<bool>,
    pub polling_interval: ::validated_data::Field<i64>,
    pub vrfs: ::validated_data::Field<Vrfs<'a, Mode>>,
    pub destinations: ::validated_data::Field<Destinations<'a, Mode>>,
    pub source: ::validated_data::Field<&'a str>,
    pub source_interface: ::validated_data::Field<&'a str>,
    pub extensions: ::validated_data::Field<Extensions<'a, Mode>>,
    pub interface: ::validated_data::Field<Interface<'a, Mode>>,
    pub run: ::validated_data::Field<bool>,
    pub hardware_acceleration: ::validated_data::Field<HardwareAcceleration<'a, Mode>>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub destinations: ::validated_data::Field<item::Destinations<'a, Mode>>,
        pub source: ::validated_data::Field<&'a str>,
        pub source_interface: ::validated_data::Field<&'a str>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(destination))]
        pub struct Destinations<'a, Mode> (::validated_data::Field<destinations::Item<'a, Mode>>);

        pub mod destinations {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub destination: ::validated_data::Field<&'a str>,
                pub port: ::validated_data::Field<i64>,
            }
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(destination))]
pub struct Destinations<'a, Mode> (::validated_data::Field<destinations::Item<'a, Mode>>);

pub mod destinations {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub destination: ::validated_data::Field<&'a str>,
        pub port: ::validated_data::Field<i64>,
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Extensions<'a, Mode> (::validated_data::Field<extensions::Item<'a, Mode>>);

pub mod extensions {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    }
}

#[::validated_data::data_view]
pub struct Interface<'a, Mode> {
    pub disable: ::validated_data::Field<interface::Disable<'a, Mode>>,
    pub egress: ::validated_data::Field<interface::Egress<'a, Mode>>,
}

pub mod interface {

    #[::validated_data::data_view]
    pub struct Disable<'a, Mode> {
        pub default: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view]
    pub struct Egress<'a, Mode> {
        pub enable_default: ::validated_data::RequiredValue<bool, Mode>,
        pub unmodified: ::validated_data::Field<bool>,
    }
}

#[::validated_data::data_view]
pub struct HardwareAcceleration<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub sample: ::validated_data::Field<i64>,
    pub modules: ::validated_data::Field<hardware_acceleration::Modules<'a, Mode>>,
}

pub mod hardware_acceleration {

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct Modules<'a, Mode> (::validated_data::Field<modules::Item<'a, Mode>>);

    pub mod modules {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub enabled: ::validated_data::Field<bool>,
        }
    }
}
