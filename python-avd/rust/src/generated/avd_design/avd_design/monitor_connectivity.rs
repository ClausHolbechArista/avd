// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.MonitorConnectivity", schema = Dict(1631)))]
pub struct MonitorConnectivity<'a, Mode> {
    #[data_view(schema = Bool(7))]
    pub shutdown: ::data_views::Field<bool>,
    #[data_view(schema = Int(77))]
    pub interval: ::data_views::Field<i64>,
    #[data_view(schema = List(678))]
    pub interface_sets: ::data_views::Field<InterfaceSets<'a, Mode>>,
    #[data_view(schema = Str(3))]
    pub local_interfaces: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(559))]
    pub address_only: ::data_views::Field<bool>,
    #[data_view(schema = List(679))]
    pub hosts: ::data_views::Field<Hosts<'a, Mode>>,
    #[data_view(schema = Str(584))]
    pub name_server_group: ::data_views::Field<&'a str>,
    #[data_view(schema = List(681))]
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.MonitorConnectivity.InterfaceSets", schema = List(678), item_name = "AVDDesign.MonitorConnectivity.InterfaceSetsItem"))]
pub struct InterfaceSets<'a, Mode> (::data_views::Field<interface_sets::Item<'a, Mode>>);

pub mod interface_sets {

    #[::data_views::data_view(python(name = "AVDDesign.MonitorConnectivity.InterfaceSetsItemBase", schema = Dict(1627)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(3))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = List(677))]
        pub interfaces: ::data_views::RequiredValue<item::Interfaces<'a, Mode>, Mode>,
    }

    pub mod item {

        #[::data_views::data_view(list, python(name = "AVDDesign.MonitorConnectivity.InterfaceSetsItemBase.Interfaces", schema = List(677)))]
        pub struct Interfaces<'a, Mode> (::data_views::Field<&'a str>);
    }
}

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.MonitorConnectivity.Hosts", schema = List(679), item_name = "AVDDesign.MonitorConnectivity.HostsItem"))]
pub struct Hosts<'a, Mode> (::data_views::Field<hosts::Item<'a, Mode>>);

pub mod hosts {

    #[::data_views::data_view(python(name = "AVDDesign.MonitorConnectivity.HostsItemBase", schema = Dict(1628)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(581))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(582))]
        pub description: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(583))]
        pub single_line_description: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(3))]
        pub ip: ::data_views::Field<&'a str>,
        #[data_view(schema = Int(258))]
        pub icmp_echo_size: ::data_views::Field<i64>,
        #[data_view(schema = Str(3))]
        pub local_interfaces: ::data_views::Field<&'a str>,
        #[data_view(schema = Bool(559))]
        pub address_only: ::data_views::Field<bool>,
        #[data_view(schema = Str(3))]
        pub url: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.MonitorConnectivity.Vrfs", schema = List(681), item_name = "AVDDesign.MonitorConnectivity.VrfsItem"))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view(python(name = "AVDDesign.MonitorConnectivity.VrfsItemBase", schema = Dict(1630)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(400))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(582))]
        pub description: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(583))]
        pub single_line_description: ::data_views::Field<&'a str>,
        #[data_view(schema = List(678))]
        pub interface_sets: ::data_views::Field<item::InterfaceSets<'a, Mode>>,
        #[data_view(schema = Str(3))]
        pub local_interfaces: ::data_views::Field<&'a str>,
        #[data_view(schema = Bool(559))]
        pub address_only: ::data_views::Field<bool>,
        #[data_view(schema = List(680))]
        pub hosts: ::data_views::Field<item::Hosts<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.MonitorConnectivity.VrfsItemBase.InterfaceSets", schema = List(678), item_name = "AVDDesign.MonitorConnectivity.VrfsItemBase.InterfaceSetsItem"))]
        pub struct InterfaceSets<'a, Mode> (::data_views::Field<interface_sets::Item<'a, Mode>>);

        pub mod interface_sets {

            #[::data_views::data_view(python(name = "AVDDesign.MonitorConnectivity.VrfsItemBase.InterfaceSetsItemBase", schema = Dict(1627)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(3))]
                pub name: ::data_views::Field<&'a str>,
                #[data_view(schema = List(677))]
                pub interfaces: ::data_views::RequiredValue<item::Interfaces<'a, Mode>, Mode>,
            }

            pub mod item {

                #[::data_views::data_view(list, python(name = "AVDDesign.MonitorConnectivity.VrfsItemBase.InterfaceSetsItemBase.Interfaces", schema = List(677)))]
                pub struct Interfaces<'a, Mode> (::data_views::Field<&'a str>);
            }
        }

        #[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.MonitorConnectivity.VrfsItemBase.Hosts", schema = List(680), item_name = "AVDDesign.MonitorConnectivity.VrfsItemBase.HostsItem"))]
        pub struct Hosts<'a, Mode> (::data_views::Field<hosts::Item<'a, Mode>>);

        pub mod hosts {

            #[::data_views::data_view(python(name = "AVDDesign.MonitorConnectivity.VrfsItemBase.HostsItemBase", schema = Dict(1629)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(585))]
                pub name: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(582))]
                pub description: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(583))]
                pub single_line_description: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(3))]
                pub ip: ::data_views::Field<&'a str>,
                #[data_view(schema = Int(258))]
                pub icmp_echo_size: ::data_views::Field<i64>,
                #[data_view(schema = Str(3))]
                pub local_interfaces: ::data_views::Field<&'a str>,
                #[data_view(schema = Bool(559))]
                pub address_only: ::data_views::Field<bool>,
                #[data_view(schema = Str(3))]
                pub url: ::data_views::Field<&'a str>,
            }
        }
    }
}
