// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub types: ::validated_data::RequiredValue<item::Types<'a, Mode>, Mode>,
    pub platforms: ::validated_data::RequiredValue<item::Platforms<'a, Mode>, Mode>,
    pub uplink_interfaces: ::validated_data::Field<item::UplinkInterfaces<'a, Mode>>,
    pub mlag_interfaces: ::validated_data::Field<item::MlagInterfaces<'a, Mode>>,
    pub mlag_interfaces_speed: ::validated_data::Field<&'a str>,
    pub downlink_interfaces: ::validated_data::Field<item::DownlinkInterfaces<'a, Mode>>,
    pub uplink_interface_speed: ::validated_data::Field<&'a str>,
}

pub mod item {

    #[::validated_data::data_view(list)]
    pub struct Types<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list)]
    pub struct Platforms<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list)]
    pub struct UplinkInterfaces<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list)]
    pub struct MlagInterfaces<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list)]
    pub struct DownlinkInterfaces<'a, Mode> (::validated_data::Field<&'a str>);
}
