// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(list, python(name = "AVDDesign.LoggingSettings.Hosts", schema = List(671)))]
pub struct Hosts<'a, Mode> (::validated_data::Field<hosts::Item<'a, Mode>>);

pub mod hosts {

    #[::validated_data::data_view(python(name = "AVDDesign.LoggingSettings.HostsItem", schema = Dict(1615)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1442))]
        pub name: ::validated_data::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Str(1443))]
        pub vrf: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(467))]
        pub protocol: ::validated_data::Field<&'a str>,
        #[data_view(schema = List(217))]
        pub ports: ::validated_data::Field<item::Ports<'a, Mode>>,
        #[data_view(schema = Str(468))]
        pub ssl_profile: ::validated_data::Field<&'a str>,
    }

    pub mod item {

        #[::validated_data::data_view(list, python(name = "AVDDesign.LoggingSettings.HostsItem.Ports", schema = List(217)))]
        pub struct Ports<'a, Mode> (::validated_data::Field<i64>);
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.LoggingSettings.Vrfs", schema = List(672), item_name = "AVDDesign.LoggingSettings.VrfsItem"))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view(python(name = "AVDDesign.LoggingSettings.VrfsItemBase", schema = Dict(1616)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(25))]
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1444))]
        pub source_interface: ::validated_data::Field<&'a str>,
    }
}
