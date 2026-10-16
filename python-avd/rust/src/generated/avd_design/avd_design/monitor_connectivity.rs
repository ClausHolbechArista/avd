// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.MonitorConnectivity.InterfaceSets", schema = List(678), item_name = "AVDDesign.MonitorConnectivity.InterfaceSetsItem"))]
pub struct InterfaceSets<'a, Mode> (::validated_data::Field<interface_sets::Item<'a, Mode>>);

pub mod interface_sets {

    #[::validated_data::data_view(python(name = "AVDDesign.MonitorConnectivity.InterfaceSetsItemBase", schema = Dict(1627)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(3))]
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(schema = List(677))]
        pub interfaces: ::validated_data::RequiredValue<item::Interfaces<'a, Mode>, Mode>,
    }

    pub mod item {

        #[::validated_data::data_view(list, python(name = "AVDDesign.MonitorConnectivity.InterfaceSetsItemBase.Interfaces", schema = List(677)))]
        pub struct Interfaces<'a, Mode> (::validated_data::Field<&'a str>);
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.MonitorConnectivity.Hosts", schema = List(679), item_name = "AVDDesign.MonitorConnectivity.HostsItem"))]
pub struct Hosts<'a, Mode> (::validated_data::Field<hosts::Item<'a, Mode>>);

pub mod hosts {

    #[::validated_data::data_view(python(name = "AVDDesign.MonitorConnectivity.HostsItemBase", schema = Dict(1628)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(581))]
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(582))]
        pub description: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(583))]
        pub single_line_description: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(3))]
        pub ip: ::validated_data::Field<&'a str>,
        #[data_view(schema = Int(258))]
        pub icmp_echo_size: ::validated_data::Field<i64>,
        #[data_view(schema = Str(3))]
        pub local_interfaces: ::validated_data::Field<&'a str>,
        #[data_view(schema = Bool(559))]
        pub address_only: ::validated_data::Field<bool>,
        #[data_view(schema = Str(3))]
        pub url: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.MonitorConnectivity.Vrfs", schema = List(681), item_name = "AVDDesign.MonitorConnectivity.VrfsItem"))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view(python(name = "AVDDesign.MonitorConnectivity.VrfsItemBase", schema = Dict(1630)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(400))]
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(582))]
        pub description: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(583))]
        pub single_line_description: ::validated_data::Field<&'a str>,
        #[data_view(schema = List(678))]
        pub interface_sets: ::validated_data::Field<item::InterfaceSets<'a, Mode>>,
        #[data_view(schema = Str(3))]
        pub local_interfaces: ::validated_data::Field<&'a str>,
        #[data_view(schema = Bool(559))]
        pub address_only: ::validated_data::Field<bool>,
        #[data_view(schema = List(680))]
        pub hosts: ::validated_data::Field<item::Hosts<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.MonitorConnectivity.VrfsItemBase.InterfaceSets", schema = List(678), item_name = "AVDDesign.MonitorConnectivity.VrfsItemBase.InterfaceSetsItem"))]
        pub struct InterfaceSets<'a, Mode> (::validated_data::Field<interface_sets::Item<'a, Mode>>);

        pub mod interface_sets {

            #[::validated_data::data_view(python(name = "AVDDesign.MonitorConnectivity.VrfsItemBase.InterfaceSetsItemBase", schema = Dict(1627)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(3))]
                pub name: ::validated_data::Field<&'a str>,
                #[data_view(schema = List(677))]
                pub interfaces: ::validated_data::RequiredValue<item::Interfaces<'a, Mode>, Mode>,
            }

            pub mod item {

                #[::validated_data::data_view(list, python(name = "AVDDesign.MonitorConnectivity.VrfsItemBase.InterfaceSetsItemBase.Interfaces", schema = List(677)))]
                pub struct Interfaces<'a, Mode> (::validated_data::Field<&'a str>);
            }
        }

        #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.MonitorConnectivity.VrfsItemBase.Hosts", schema = List(680), item_name = "AVDDesign.MonitorConnectivity.VrfsItemBase.HostsItem"))]
        pub struct Hosts<'a, Mode> (::validated_data::Field<hosts::Item<'a, Mode>>);

        pub mod hosts {

            #[::validated_data::data_view(python(name = "AVDDesign.MonitorConnectivity.VrfsItemBase.HostsItemBase", schema = Dict(1629)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(585))]
                pub name: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(582))]
                pub description: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(583))]
                pub single_line_description: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(3))]
                pub ip: ::validated_data::Field<&'a str>,
                #[data_view(schema = Int(258))]
                pub icmp_echo_size: ::validated_data::Field<i64>,
                #[data_view(schema = Str(3))]
                pub local_interfaces: ::validated_data::Field<&'a str>,
                #[data_view(schema = Bool(559))]
                pub address_only: ::validated_data::Field<bool>,
                #[data_view(schema = Str(3))]
                pub url: ::validated_data::Field<&'a str>,
            }
        }
    }
}
