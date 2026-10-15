// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.MonitorConnectivity.InterfaceSets", item_name = "AVDDesign.MonitorConnectivity.InterfaceSetsItem"))]
pub struct InterfaceSets<'a, Mode> (::validated_data::Field<interface_sets::Item<'a, Mode>>);

pub mod interface_sets {

    #[::validated_data::data_view(python(name = "AVDDesign.MonitorConnectivity.InterfaceSetsItemBase"))]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub interfaces: ::validated_data::RequiredValue<item::Interfaces<'a, Mode>, Mode>,
    }

    pub mod item {

        #[::validated_data::data_view(list, python(name = "AVDDesign.MonitorConnectivity.InterfaceSetsItemBase.Interfaces"))]
        pub struct Interfaces<'a, Mode> (::validated_data::Field<&'a str>);
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.MonitorConnectivity.Hosts", item_name = "AVDDesign.MonitorConnectivity.HostsItem"))]
pub struct Hosts<'a, Mode> (::validated_data::Field<hosts::Item<'a, Mode>>);

pub mod hosts {

    #[::validated_data::data_view(python(name = "AVDDesign.MonitorConnectivity.HostsItemBase"))]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub description: ::validated_data::Field<&'a str>,
        pub single_line_description: ::validated_data::Field<&'a str>,
        pub ip: ::validated_data::Field<&'a str>,
        pub icmp_echo_size: ::validated_data::Field<i64>,
        pub local_interfaces: ::validated_data::Field<&'a str>,
        pub address_only: ::validated_data::Field<bool>,
        pub url: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.MonitorConnectivity.Vrfs", item_name = "AVDDesign.MonitorConnectivity.VrfsItem"))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view(python(name = "AVDDesign.MonitorConnectivity.VrfsItemBase"))]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub description: ::validated_data::Field<&'a str>,
        pub single_line_description: ::validated_data::Field<&'a str>,
        pub interface_sets: ::validated_data::Field<item::InterfaceSets<'a, Mode>>,
        pub local_interfaces: ::validated_data::Field<&'a str>,
        pub address_only: ::validated_data::Field<bool>,
        pub hosts: ::validated_data::Field<item::Hosts<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.MonitorConnectivity.VrfsItemBase.InterfaceSets", item_name = "AVDDesign.MonitorConnectivity.VrfsItemBase.InterfaceSetsItem"))]
        pub struct InterfaceSets<'a, Mode> (::validated_data::Field<interface_sets::Item<'a, Mode>>);

        pub mod interface_sets {

            #[::validated_data::data_view(python(name = "AVDDesign.MonitorConnectivity.VrfsItemBase.InterfaceSetsItemBase"))]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub interfaces: ::validated_data::RequiredValue<item::Interfaces<'a, Mode>, Mode>,
            }

            pub mod item {

                #[::validated_data::data_view(list, python(name = "AVDDesign.MonitorConnectivity.VrfsItemBase.InterfaceSetsItemBase.Interfaces"))]
                pub struct Interfaces<'a, Mode> (::validated_data::Field<&'a str>);
            }
        }

        #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.MonitorConnectivity.VrfsItemBase.Hosts", item_name = "AVDDesign.MonitorConnectivity.VrfsItemBase.HostsItem"))]
        pub struct Hosts<'a, Mode> (::validated_data::Field<hosts::Item<'a, Mode>>);

        pub mod hosts {

            #[::validated_data::data_view(python(name = "AVDDesign.MonitorConnectivity.VrfsItemBase.HostsItemBase"))]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub description: ::validated_data::Field<&'a str>,
                pub single_line_description: ::validated_data::Field<&'a str>,
                pub ip: ::validated_data::Field<&'a str>,
                pub icmp_echo_size: ::validated_data::Field<i64>,
                pub local_interfaces: ::validated_data::Field<&'a str>,
                pub address_only: ::validated_data::Field<bool>,
                pub url: ::validated_data::Field<&'a str>,
            }
        }
    }
}
