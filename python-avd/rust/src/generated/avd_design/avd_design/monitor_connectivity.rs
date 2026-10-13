// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesignMonitorConnectivityInterfaceSetsList", item_name = "AVDDesignMonitorConnectivityInterfaceSetsListIndexedItem"))]
pub struct InterfaceSets<'a, Mode> (::validated_data::Field<interface_sets::Item<'a, Mode>>);

pub mod interface_sets {

    #[::validated_data::data_view(python(name = "AVDDesignMonitorConnectivityInterfaceSetsItems"))]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub interfaces: ::validated_data::RequiredValue<item::Interfaces<'a, Mode>, Mode>,
    }

    pub mod item {

        #[::validated_data::data_view(list, python(name = "AVDDesignMonitorConnectivityInterfaceSetsItemsInterfacesList"))]
        pub struct Interfaces<'a, Mode> (::validated_data::Field<&'a str>);
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesignMonitorConnectivityHostsList", item_name = "AVDDesignMonitorConnectivityHostsListIndexedItem"))]
pub struct Hosts<'a, Mode> (::validated_data::Field<hosts::Item<'a, Mode>>);

pub mod hosts {

    #[::validated_data::data_view(python(name = "AVDDesignMonitorConnectivityHostsItems"))]
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

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesignMonitorConnectivityVrfsList", item_name = "AVDDesignMonitorConnectivityVrfsListIndexedItem"))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view(python(name = "AVDDesignMonitorConnectivityVrfsItems"))]
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

        #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesignMonitorConnectivityVrfsItemsInterfaceSetsList", item_name = "AVDDesignMonitorConnectivityVrfsItemsInterfaceSetsListIndexedItem"))]
        pub struct InterfaceSets<'a, Mode> (::validated_data::Field<interface_sets::Item<'a, Mode>>);

        pub mod interface_sets {

            #[::validated_data::data_view(python(name = "AVDDesignMonitorConnectivityVrfsItemsInterfaceSetsItems"))]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub interfaces: ::validated_data::RequiredValue<item::Interfaces<'a, Mode>, Mode>,
            }

            pub mod item {

                #[::validated_data::data_view(list, python(name = "AVDDesignMonitorConnectivityVrfsItemsInterfaceSetsItemsInterfacesList"))]
                pub struct Interfaces<'a, Mode> (::validated_data::Field<&'a str>);
            }
        }

        #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesignMonitorConnectivityVrfsItemsHostsList", item_name = "AVDDesignMonitorConnectivityVrfsItemsHostsListIndexedItem"))]
        pub struct Hosts<'a, Mode> (::validated_data::Field<hosts::Item<'a, Mode>>);

        pub mod hosts {

            #[::validated_data::data_view(python(name = "AVDDesignMonitorConnectivityVrfsItemsHostsItems"))]
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
