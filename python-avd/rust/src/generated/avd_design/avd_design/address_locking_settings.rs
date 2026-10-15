// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(list, python(name = "AVDDesign.AddressLockingSettings.DhcpServerInterfaces"))]
pub struct DhcpServerInterfaces<'a, Mode> (::validated_data::Field<&'a str>);

#[::validated_data::data_view(list, python(name = "AVDDesign.AddressLockingSettings.DhcpServersIpv4"))]
pub struct DhcpServersIpv4<'a, Mode> (::validated_data::Field<&'a str>);

#[::validated_data::data_view(list, python(name = "AVDDesign.AddressLockingSettings.Leases"))]
pub struct Leases<'a, Mode> (::validated_data::Field<leases::Item<'a, Mode>>);

pub mod leases {

    #[::validated_data::data_view(python(name = "AVDDesign.AddressLockingSettings.LeasesItem"))]
    pub struct Item<'a, Mode> {
        pub ip: ::validated_data::RequiredValue<&'a str, Mode>,
        pub mac: ::validated_data::RequiredValue<&'a str, Mode>,
    }
}

#[::validated_data::data_view(python(name = "AVDDesign.AddressLockingSettings.LockedAddress"))]
pub struct LockedAddress<'a, Mode> {
    pub expiration_mac_disabled: ::validated_data::Field<bool>,
    pub ipv4_enforcement_disabled: ::validated_data::Field<bool>,
    pub ipv6_enforcement_disabled: ::validated_data::Field<bool>,
}
