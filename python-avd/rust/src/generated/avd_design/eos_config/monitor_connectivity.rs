// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct MonitorConnectivity<'a, Mode> {
    pub shutdown: ::data_views::Field<bool>,
    pub interval: ::data_views::Field<i64>,
    pub interface_sets: ::data_views::Field<InterfaceSets<'a, Mode>>,
    pub local_interfaces: ::data_views::Field<&'a str>,
    pub address_only: ::data_views::Field<bool>,
    pub hosts: ::data_views::Field<Hosts<'a, Mode>>,
    pub name_server_group: ::data_views::Field<&'a str>,
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct InterfaceSets<'a, Mode> (::data_views::Field<interface_sets::Item<'a, Mode>>);

pub mod interface_sets {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub interfaces: ::data_views::RequiredValue<&'a str, Mode>,
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Hosts<'a, Mode> (::data_views::Field<hosts::Item<'a, Mode>>);

pub mod hosts {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub description: ::data_views::Field<&'a str>,
        pub single_line_description: ::data_views::Field<&'a str>,
        pub ip: ::data_views::Field<&'a str>,
        pub icmp_echo_size: ::data_views::Field<i64>,
        pub local_interfaces: ::data_views::Field<&'a str>,
        pub address_only: ::data_views::Field<bool>,
        pub url: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub description: ::data_views::Field<&'a str>,
        pub single_line_description: ::data_views::Field<&'a str>,
        pub interface_sets: ::data_views::Field<item::InterfaceSets<'a, Mode>>,
        pub local_interfaces: ::data_views::Field<&'a str>,
        pub address_only: ::data_views::Field<bool>,
        pub hosts: ::data_views::Field<item::Hosts<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct InterfaceSets<'a, Mode> (::data_views::Field<interface_sets::Item<'a, Mode>>);

        pub mod interface_sets {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
                pub interfaces: ::data_views::Field<&'a str>,
            }
        }

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct Hosts<'a, Mode> (::data_views::Field<hosts::Item<'a, Mode>>);

        pub mod hosts {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
                pub description: ::data_views::Field<&'a str>,
                pub single_line_description: ::data_views::Field<&'a str>,
                pub ip: ::data_views::Field<&'a str>,
                pub icmp_echo_size: ::data_views::Field<i64>,
                pub local_interfaces: ::data_views::Field<&'a str>,
                pub address_only: ::data_views::Field<bool>,
                pub url: ::data_views::Field<&'a str>,
            }
        }
    }
}
