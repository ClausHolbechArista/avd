// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(list, python(name = "AVDDesign.LoggingSettings.Hosts"))]
pub struct Hosts<'a, Mode> (::validated_data::Field<hosts::Item<'a, Mode>>);

pub mod hosts {

    #[::validated_data::data_view(python(name = "AVDDesign.LoggingSettings.HostsItem"))]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::RequiredValue<&'a str, Mode>,
        pub vrf: ::validated_data::Field<&'a str>,
        pub protocol: ::validated_data::Field<&'a str>,
        pub ports: ::validated_data::Field<item::Ports<'a, Mode>>,
        pub ssl_profile: ::validated_data::Field<&'a str>,
    }

    pub mod item {

        #[::validated_data::data_view(list, python(name = "AVDDesign.LoggingSettings.HostsItem.Ports"))]
        pub struct Ports<'a, Mode> (::validated_data::Field<i64>);
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.LoggingSettings.Vrfs", item_name = "AVDDesign.LoggingSettings.VrfsItem"))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view(python(name = "AVDDesign.LoggingSettings.VrfsItemBase"))]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub source_interface: ::validated_data::Field<&'a str>,
    }
}
