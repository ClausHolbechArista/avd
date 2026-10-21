// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(list, python(name = "AVDDesign.DefaultInterfaces", schema = List(609)))]
pub struct DefaultInterfaces<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.DefaultInterfacesItem", schema = Dict(1462)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = List(604))]
    pub types: ::data_views::RequiredValue<item::Types<'a, Mode>, Mode>,
    #[data_view(schema = List(605))]
    pub platforms: ::data_views::RequiredValue<item::Platforms<'a, Mode>, Mode>,
    #[data_view(schema = List(606))]
    pub uplink_interfaces: ::data_views::Field<item::UplinkInterfaces<'a, Mode>>,
    #[data_view(schema = List(607))]
    pub mlag_interfaces: ::data_views::Field<item::MlagInterfaces<'a, Mode>>,
    #[data_view(schema = Str(1225))]
    pub mlag_interfaces_speed: ::data_views::Field<&'a str>,
    #[data_view(schema = List(608))]
    pub downlink_interfaces: ::data_views::Field<item::DownlinkInterfaces<'a, Mode>>,
    #[data_view(schema = Str(1226))]
    pub uplink_interface_speed: ::data_views::Field<&'a str>,
}

pub mod item {

    #[::data_views::data_view(list, python(name = "AVDDesign.DefaultInterfacesItem.Types", schema = List(604)))]
    pub struct Types<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(list, python(name = "AVDDesign.DefaultInterfacesItem.Platforms", schema = List(605)))]
    pub struct Platforms<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(list, python(name = "AVDDesign.DefaultInterfacesItem.UplinkInterfaces", schema = List(606)))]
    pub struct UplinkInterfaces<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(list, python(name = "AVDDesign.DefaultInterfacesItem.MlagInterfaces", schema = List(607)))]
    pub struct MlagInterfaces<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(list, python(name = "AVDDesign.DefaultInterfacesItem.DownlinkInterfaces", schema = List(608)))]
    pub struct DownlinkInterfaces<'a, Mode> (::data_views::Field<&'a str>);
}
